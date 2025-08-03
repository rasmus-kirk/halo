use std::{array, collections::HashMap, time::Instant};

use anyhow::{Context, Result, anyhow, bail, ensure};
use halo_group::{
    Domain, Evals, PastaConfig, Scalar,
    ark_ff::Field,
    ark_poly::{EvaluationDomain, Evaluations, Radix2EvaluationDomain},
    ark_std::{One, Zero},
};
use log::debug;
use petgraph::algo::toposort;

use crate::{
    circuit::{CircuitSpec, GateType, Trace, Wire},
    utils::{MultiAssign, SELECTOR_POLYS, WITNESS_POLYS},
};

use super::SlotId;

pub struct TraceBuilder<P: PastaConfig> {
    spec: CircuitSpec<P>,
    witnesses: HashMap<Wire, P::ScalarField>,
    public_inputs: HashMap<Wire, P::ScalarField>,
    public_row_count: usize,
    row_count: usize,
}

impl<P: PastaConfig> TraceBuilder<P> {
    fn get_slot_ids(&mut self) -> [SlotId; WITNESS_POLYS] {
        let row = self.row_count + self.spec.public_input_wire_count;
        self.row_count += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        slot_ids
    }

    fn get_witness_slot_ids(&mut self) -> [SlotId; WITNESS_POLYS] {
        let row = self.public_row_count;
        self.public_row_count += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        slot_ids
    }

    pub fn new(spec: CircuitSpec<P>) -> Self {
        let witnesses = HashMap::with_capacity(spec.witness_wire_count);
        let public_inputs = HashMap::with_capacity(spec.public_input_wire_count);
        Self {
            row_count: 0,
            public_row_count: 0,
            spec,
            witnesses,
            public_inputs,
        }
    }

    pub fn witness(&mut self, wire: Wire, w: P::ScalarField) -> Result<()> {
        match self.spec.graph.node_weight(wire.node_idx) {
            Some(GateType::Witness) => (),
            Some(_) => bail!("The provided wire was not a witness wire!"),
            None => bail!("Wire does not exist, this should be impossible!"),
        }
        let already_assigned = self.witnesses.insert(wire, w).is_some();
        if already_assigned {
            bail!("Wire already assigned! ({wire:?}, {w:?})")
        };
        Ok(())
    }

    pub fn public_input(&mut self, wire: Wire, x: P::ScalarField) -> Result<()> {
        match self.spec.graph.node_weight(wire.node_idx) {
            Some(GateType::PublicInput) => (),
            Some(_) => bail!("The provided wire was not a public input wire!"),
            None => bail!("Wire does not exist, this should be impossible!"),
        }
        let already_assigned = self.public_inputs.insert(wire, x).is_some();
        if already_assigned {
            bail!("Wire already assigned! ({wire:?}, {x:?})")
        };
        Ok(())
    }

    pub fn trace(mut self) -> Result<Trace<P>> {
        let now = Instant::now();

        let spec = self.spec.clone();
        let n = spec.row_count.next_power_of_two();

        ensure!(
            self.witnesses.len() == spec.witness_wire_count,
            "Expected {} witnesses, got {}",
            spec.witness_wire_count,
            self.witnesses.len()
        );
        ensure!(
            self.public_inputs.len() == spec.public_input_wire_count,
            "Expected {} witnesses, got {}",
            spec.public_input_wire_count,
            self.public_inputs.len()
        );

        let O = P::ScalarField::zero();
        let I = P::ScalarField::one();
        let mut output = None;
        let mut ws: [_; WITNESS_POLYS] = array::from_fn(|_| vec![O; n]);
        let mut rs: [_; WITNESS_POLYS] = array::from_fn(|_| vec![O; n]);
        let mut qs: [_; SELECTOR_POLYS] = array::from_fn(|_| vec![O; n]);

        let topo_order = toposort(&spec.graph, None).map_err(|_| anyhow!("Cycle detected"))?;
        let mut wire_value_map = vec![Scalar::<P>::zero(); spec.wire_count];
        let mut wire_slot_map = vec![Vec::new(); spec.wire_count];
        let mut node_map = HashMap::new();
        let mut public_inputs = Vec::new();

        // Evaluate and collect inputs/outputs for gates
        for node_idx in topo_order {
            match spec.graph[node_idx] {
                GateType::Witness => {
                    let (out_wire, _) = spec.get_gate_outputs(node_idx)[0];
                    let v = self.witnesses.get(&out_wire).context("Wire unassigned!")?;

                    wire_value_map[out_wire.id] = *v;
                }
                GateType::PublicInput => {
                    let slots = self.get_witness_slot_ids();
                    let (out_wire, _) = spec.get_gate_outputs(node_idx)[0];
                    let v = self
                        .public_inputs
                        .get(&out_wire)
                        .context("Wire unassigned!")?;

                    let row = slots[0].row_0_indexed();
                    node_map.insert(node_idx, slots);

                    public_inputs.push(-(*v));
                    wire_value_map[out_wire.id] = *v;
                    wire_slot_map[out_wire.id].push(slots[0]);
                    ws.multi_assign(row, [*v, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r, o, m, c, p]
                    qs.multi_assign(row, [I, O, O, O, O, O]);
                }
                GateType::Constant(v) => {
                    let (out_wire, _) = spec.get_gate_outputs(node_idx)[0];

                    let slots = self.get_slot_ids();
                    let row = slots[0].row_0_indexed();
                    node_map.insert(node_idx, slots);

                    wire_value_map[out_wire.id] = v;
                    wire_slot_map[out_wire.id].push(slots[0]);
                    ws.multi_assign(row, [v, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r, o, m,  c, p]
                    qs.multi_assign(row, [I, O, O, O, -v, O]);
                }
                GateType::Output => {
                    let [(in_wire, _)] = spec.get_gate_inputs(node_idx);
                    match output {
                        None => output = Some(wire_value_map[in_wire.id]),
                        Some(_) => bail!("Multiple output gates found in circuit!"),
                    }
                }
                GateType::AssertEq => {
                    let [(l_wire, l_edge_idx), (r_wire, r_edge_idx)] =
                        spec.get_gate_inputs(node_idx);
                    let l_slots = node_map[&spec.get_parent_node(l_edge_idx)];
                    let r_slots = node_map[&spec.get_parent_node(r_edge_idx)];

                    wire_slot_map[l_wire.id].push(r_slots[r_wire.output_id as usize]);
                    wire_slot_map[r_wire.id].push(l_slots[l_wire.output_id as usize]);
                }
                GateType::Add => {
                    let [(l_wire, _), (r_wire, _)] = spec.get_gate_inputs(node_idx);
                    let (out_wire, _) = spec.get_gate_outputs(node_idx)[0];

                    let slots = self.get_slot_ids();
                    let row = slots[0].row_0_indexed();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let a = wire_value_map[l_wire.id];
                    let b = wire_value_map[r_wire.id];
                    let c = a + b;
                    wire_value_map[out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p ]
                    qs.multi_assign(row, [I, I, -I, O, O, O]);
                    rs.multi_assign(row, [O; WITNESS_POLYS]);

                    // ----- Copy Constraints ----- //
                    wire_slot_map[l_wire.id].push(slots[0]);
                    wire_slot_map[r_wire.id].push(slots[1]);
                    wire_slot_map[out_wire.id].push(slots[2]);
                }
                GateType::Multiply => {
                    let [(l_wire, _), (r_wire, _)] = spec.get_gate_inputs(node_idx);
                    let (out_wire, _) = spec.get_gate_outputs(node_idx)[0];

                    let slots = self.get_slot_ids();
                    let row = slots[0].row_0_indexed();
                    node_map.insert(node_idx, slots);

                    let a = wire_value_map[l_wire.id];
                    let b = wire_value_map[r_wire.id];
                    let c = a * b;
                    wire_value_map[out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p]
                    qs.multi_assign(row, [O, O, -I, I, O, O]);
                    rs.multi_assign(row, [O; WITNESS_POLYS]);

                    // ----- Copy Constraints ----- //
                    wire_slot_map[l_wire.id].push(slots[0]);
                    wire_slot_map[r_wire.id].push(slots[1]);
                    wire_slot_map[out_wire.id].push(slots[2]);
                }
                GateType::Poseidon(r) => {
                    let [(wire_s0, _), (wire_s1, _), (wire_s2, _)] = spec.get_gate_inputs(node_idx);

                    let slots = self.get_slot_ids();
                    let row = slots[0].row_0_indexed();
                    node_map.insert(node_idx, slots);

                    let m = P::SCALAR_POSEIDON_MDS;
                    let w00 = wire_value_map[wire_s0.id];
                    let w01 = wire_value_map[wire_s1.id];
                    let w02 = wire_value_map[wire_s2.id];

                    let (w03, w04, w05) = poseidon_round::<P>(m, r[0], r[1], r[2], w00, w01, w02);
                    let (w06, w07, w08) = poseidon_round::<P>(m, r[3], r[4], r[5], w03, w04, w05);
                    let (w09, w10, w11) = poseidon_round::<P>(m, r[6], r[7], r[8], w06, w07, w08);
                    let (w12, w13, w14) = poseidon_round::<P>(m, r[9], r[10], r[11], w09, w10, w11);

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [
                        w00, w01, w02, w03, w04, w05, w06, w07, w08, w09, w10, w11, w12, w13, w14,
                    ]);
                    //                   [l, r, o, m, c, p]
                    qs.multi_assign(row, [O, O, O, O, O, I]);
                    rs.multi_assign(row, r);

                    // ----- Copy Constraints ----- //
                    wire_slot_map[wire_s0.id].push(slots[0]);
                    wire_slot_map[wire_s1.id].push(slots[1]);
                    wire_slot_map[wire_s2.id].push(slots[2]);
                }
            }
        }

        debug!("trace_builder_time: {:?}", now.elapsed().as_secs_f32());

        let out = output.context("No output gate found in circuit!")?;
        Ok(Trace::new(wire_slot_map, public_inputs, ws, qs, out, n))
    }
}

fn sbox<F: Field>(mut x: F) -> F {
    let mut square = x;
    square.square_in_place();
    x *= square;
    square.square_in_place();
    x *= square;
    x
}

fn poseidon_round<P: PastaConfig>(
    M: [[Scalar<P>; 3]; 3],
    r0: Scalar<P>,
    r1: Scalar<P>,
    r2: Scalar<P>,
    w0: Scalar<P>,
    w1: Scalar<P>,
    w2: Scalar<P>,
) -> (Scalar<P>, Scalar<P>, Scalar<P>) {
    (
        r0 + (M[0][0] * sbox(w0) + M[0][1] * sbox(w1) + M[0][2] * sbox(w2)),
        r1 + (M[1][0] * sbox(w0) + M[1][1] * sbox(w1) + M[1][2] * sbox(w2)),
        r2 + (M[2][0] * sbox(w0) + M[2][1] * sbox(w1) + M[2][2] * sbox(w2)),
    )
}
