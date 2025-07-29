use std::{array, collections::HashMap};

use anyhow::{Context, Result, anyhow, bail, ensure};
use halo_group::{
    Evals, PastaConfig,
    ark_std::{One, Zero},
};
use petgraph::algo::toposort;

use crate::{
    circuit::{CircuitSpec, GateType, Trace, Wire},
    utils::{MultiAssign, SELECTOR_POLYS, WITNESS_POLYS},
};

pub struct TraceBuilder<P: PastaConfig> {
    spec: CircuitSpec<P>,
    witnesses: HashMap<Wire, P::ScalarField>,
    public_inputs: HashMap<Wire, P::ScalarField>,
}

impl<P: PastaConfig> TraceBuilder<P> {
    pub fn new(spec: CircuitSpec<P>) -> Self {
        let witnesses = HashMap::with_capacity(spec.witness_wire_count);
        let public_inputs = HashMap::with_capacity(spec.public_input_wire_count);
        Self {
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

    pub fn trace(self) -> Result<Trace<P>> {
        let spec = self.spec;
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

        let zero = P::ScalarField::zero();
        let one = P::ScalarField::one();
        let mut output = None;
        let mut ws: [Evals<P>; WITNESS_POLYS] = array::from_fn(|_| Evals::new(vec![zero; n], n));
        let mut qs: [Evals<P>; SELECTOR_POLYS] = array::from_fn(|_| Evals::new(vec![zero; n], n));

        let topo_order = toposort(&spec.graph, None).map_err(|_| anyhow!("Cycle detected"))?;
        let mut wire_values = vec![P::ScalarField::zero(); spec.wire_count];
        let mut copy_constraints = Vec::with_capacity(2 * spec.graph.node_count());

        // Evaluate and collect inputs/outputs for gates
        for node_idx in topo_order {
            match spec.graph[node_idx] {
                GateType::Witness => {
                    let output_wire = spec.get_gate_outputs(node_idx)[0];
                    let v = self
                        .witnesses
                        .get(&output_wire)
                        .context("Wire unassigned!")?;

                    wire_values[output_wire.id] = *v;
                }
                GateType::PublicInput => {
                    let output_wire = spec.get_gate_outputs(node_idx)[0];
                    let row = output_wire.output_slot_id.unwrap().row_0_indexed();
                    let v = self
                        .public_inputs
                        .get(&output_wire)
                        .context("Wire unassigned!")?;

                    wire_values[output_wire.id] = *v;
                    //                   [a,  b,    c  ]
                    ws.multi_assign(row, [*v, zero, zero]);
                    //            [q_l, q_r,  q_o, q_m,  q_c]
                    qs.multi_assign(row, [one, zero, zero, zero, -(*v)]);
                }
                GateType::Constant(slot_id, v) => {
                    let row = slot_id.row_0_indexed();
                    let output_wire = spec.get_gate_outputs(node_idx)[0];

                    wire_values[output_wire.node_idx.index()] = v;
                    //                   [a,  b,    c  ]
                    ws.multi_assign(row, [v, zero, zero]);
                    //            [q_l, q_r,  q_o, q_m,  q_c]
                    qs.multi_assign(row, [one, zero, zero, zero, -v]);
                }
                GateType::Output => {
                    let input_wire = spec.get_gate_inputs(node_idx)[0];
                    match output {
                        None => output = Some(wire_values[input_wire.id]),
                        Some(_) => bail!("Multiple output gates found in circuit!"),
                    }
                }
                GateType::AssertEq => {
                    let input_wires = spec.get_gate_inputs(node_idx);
                    copy_constraints.push((
                        input_wires[0].output_slot_id.unwrap(),
                        input_wires[1].output_slot_id.unwrap(),
                    ));
                }
                GateType::Add(slot_ids) => {
                    let input_wires = spec.get_gate_inputs(node_idx);
                    let output_wire = spec.get_gate_outputs(node_idx)[0];
                    let row = slot_ids[0].row_0_indexed();

                    // ----- Values ----- //
                    let a = wire_values[input_wires[0].id];
                    let b = wire_values[input_wires[1].id];
                    let c = a + b;
                    wire_values[output_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c]);
                    //            [q_l, q_r, q_o,  q_m,  q_c ]
                    qs.multi_assign(row, [one, one, -one, zero, zero]);

                    // ----- Copy Constraints ----- //
                    match (input_wires[0].output_slot_id, input_wires[1].output_slot_id) {
                        (None, None) if input_wires[0].node_idx == input_wires[1].node_idx => {
                            copy_constraints.push((slot_ids[0], slot_ids[1]));
                        }
                        (None, None) => (),
                        (Some(l_slot_id), None) => copy_constraints.push((l_slot_id, slot_ids[0])),
                        (None, Some(r_slot_id)) => copy_constraints.push((r_slot_id, slot_ids[1])),
                        (Some(l_slot_id), Some(r_slot_id)) => {
                            copy_constraints.push((l_slot_id, slot_ids[0]));
                            copy_constraints.push((r_slot_id, slot_ids[1]));
                        }
                    };
                }
                GateType::Multiply(slot_ids) => {
                    let input_wires = spec.get_gate_inputs(node_idx);
                    let output_wire = spec.get_gate_outputs(node_idx)[0];
                    let row = slot_ids[0].row_0_indexed();

                    let a = wire_values[input_wires[0].id];
                    let b = wire_values[input_wires[1].id];
                    let c = a * b;
                    wire_values[output_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c]);
                    //            [q_l,  q_r,  q_o,  q_m, q_c ]
                    qs.multi_assign(row, [zero, zero, -one, one, zero]);

                    // ----- Copy Constraints ----- //
                    match (input_wires[0].output_slot_id, input_wires[1].output_slot_id) {
                        (None, None) if input_wires[0].node_idx == input_wires[1].node_idx => {
                            copy_constraints.push((slot_ids[0], slot_ids[1]));
                        }
                        (None, None) => (),
                        (Some(l_slot_id), None) => copy_constraints.push((l_slot_id, slot_ids[0])),
                        (None, Some(r_slot_id)) => copy_constraints.push((r_slot_id, slot_ids[0])),
                        (Some(l_slot_id), Some(r_slot_id)) => {
                            copy_constraints.push((l_slot_id, slot_ids[0]));
                            copy_constraints.push((r_slot_id, slot_ids[1]));
                        }
                    };
                }
            }
        }

        let out = output.context("No output gate found in circuit!")?;
        Ok(Trace::new(copy_constraints, ws, qs, out, n))
    }
}
