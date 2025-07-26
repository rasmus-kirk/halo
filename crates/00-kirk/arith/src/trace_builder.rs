use std::array;

use anyhow::{Result, anyhow, bail, ensure};
use halo_group::{
    Evals, PastaConfig,
    ark_std::{One, Zero},
};
use petgraph::algo::toposort;

use crate::{
    circuit_spec::{CircuitSpec, GateType, Wire},
    trace::Trace,
    utils::{MultiAssign, SELECTOR_POLYS, WITNESS_POLYS},
};

pub struct TraceBuilder<P: PastaConfig> {
    spec: CircuitSpec<P>,
    witnesses: Vec<(Wire, P::ScalarField)>,
    public_inputs: Vec<(Wire, P::ScalarField)>,
}

impl<P: PastaConfig> TraceBuilder<P> {
    pub fn new(spec: CircuitSpec<P>) -> Self {
        let witnesses = Vec::with_capacity(spec.witness_wires.len());
        let public_inputs = Vec::with_capacity(spec.public_input_wires.len());
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
        ensure!(
            self.witnesses.len() < self.spec.witness_wires.len(),
            "Tried to add witness, but all witness inputs has been provided!"
        );
        Ok(self.witnesses.push((wire, w)))
    }

    pub fn public_input(&mut self, wire: Wire, x: P::ScalarField) -> Result<()> {
        match self.spec.graph.node_weight(wire.node_idx) {
            Some(GateType::PublicInput) => (),
            Some(_) => bail!("The provided wire was not a public input wire!"),
            None => bail!("Wire does not exist, this should be impossible!"),
        }
        ensure!(
            self.public_inputs.len() < self.spec.public_input_wires.len(),
            "Tried to add witness, but all witness inputs has been provided!"
        );
        Ok(self.public_inputs.push((wire, x)))
    }

    pub fn trace(self) -> Result<Trace<P>> {
        let spec = self.spec;
        let n = spec.row_count.next_power_of_two();

        ensure!(
            self.witnesses.len() == spec.witness_wires.len(),
            "Expected {} witnesses, got {}",
            spec.witness_wires.len(),
            self.witnesses.len()
        );

        let zero = P::ScalarField::zero();
        let one = P::ScalarField::one();
        let mut ws: [Evals<P>; WITNESS_POLYS] = array::from_fn(|_| Evals::new(vec![zero; n], n));
        let mut qs: [Evals<P>; SELECTOR_POLYS] = array::from_fn(|_| Evals::new(vec![zero; n], n));

        let topo_order = toposort(&spec.graph, None).map_err(|_| anyhow!("Cycle detected"))?;
        let mut wire_values = vec![P::ScalarField::zero(); spec.wire_count];
        let mut copy_constraints = Vec::with_capacity(2 * spec.graph.node_count());

        // Assign input values
        for (wire, v) in self.public_inputs {
            let row = wire.output_slot_id.unwrap().row() - 1;
            wire_values[wire.id] = v;
            //                   [a,  b,    c  ]
            ws.multi_assign(row, [v, zero, zero]);
            //            [q_l, q_r,  q_o,  q_m,  q_c ]
            qs.multi_assign(row, [one, zero, zero, zero, zero]);
        }
        for (wire, v) in self.witnesses {
            wire_values[wire.id] = v;
        }
        for (wire, v) in spec.constant_wires.iter() {
            let row = wire.output_slot_id.unwrap().row() - 1;
            let i = spec.graph.node_weight(wire.node_idx).unwrap();
            println!("pi: {:?}, {:?}", i, wire.output_slot_id);
            wire_values[wire.node_idx.index()] = *v;
            //                   [a,  b,    c  ]
            ws.multi_assign(row, [*v, zero, zero]);
            //            [q_l, q_r,  q_o, q_m,  q_c]
            qs.multi_assign(row, [one, zero, zero, zero, -(*v)]);
        }

        // Evaluate and collect inputs/outputs for gates
        for node_idx in topo_order {
            match spec.graph[node_idx] {
                GateType::Witness | GateType::Constant(_) => (),
                GateType::PublicInput => (),
                GateType::AssertEq => {
                    let input_wires = spec.get_gate_inputs(node_idx);
                    copy_constraints.push((
                        input_wires[0].output_slot_id.unwrap(),
                        input_wires[1].output_slot_id.unwrap(),
                    ));
                }
                GateType::Add(slot_ids) => {
                    let input_wires = spec.get_gate_inputs(node_idx);
                    let row = slot_ids[0].row() - 1;

                    // ----- Values ----- //
                    let a = wire_values[input_wires[0].id];
                    let b = wire_values[input_wires[1].id];
                    let c = a + b;
                    wire_values[node_idx.index()] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c]);
                    //            [q_l, q_r, q_o,  q_m,  q_c ]
                    qs.multi_assign(row, [one, one, -one, zero, zero]);

                    // ----- Copy Constraints ----- //
                    match (input_wires[0].output_slot_id, input_wires[1].output_slot_id) {
                        (None, None) => {
                            if input_wires[0].node_idx == input_wires[1].node_idx {
                                copy_constraints.push((slot_ids[0], slot_ids[1]));
                            }
                        }
                        (Some(l_slot_id), None) => copy_constraints.push((l_slot_id, slot_ids[0])),
                        (None, Some(r_slot_id)) => copy_constraints.push((r_slot_id, slot_ids[1])),
                        (Some(l_slot_id), Some(r_slot_id)) => {
                            copy_constraints.push((l_slot_id, slot_ids[0]));
                            copy_constraints.push((r_slot_id, slot_ids[1]));
                        }
                    };
                }
                GateType::Multiply(slot_ids) => {
                    let row = slot_ids[0].row() - 1;
                    let input_wires = spec.get_gate_inputs(node_idx);
                    let left_wire = input_wires[0];
                    let right_wire = input_wires[1];

                    let a = wire_values[left_wire.id];
                    let b = wire_values[right_wire.id];
                    let c = a * b;
                    wire_values[node_idx.index()] = c;

                    // ----- Gate Constraints ----- //
                    ws.multi_assign(row, [a, b, c]);
                    //            [q_l,  q_r,  q_o,  q_m, q_c ]
                    qs.multi_assign(row, [zero, zero, -one, one, zero]);

                    // ----- Copy Constraints ----- //
                    let c = spec.graph.node_weight(node_idx).unwrap();
                    let l = spec.graph.node_weight(left_wire.node_idx).unwrap();
                    let r = spec.graph.node_weight(right_wire.node_idx).unwrap();
                    println!("{c:?}: l({l:?}) - r({r:?})");
                    match (left_wire.output_slot_id, right_wire.output_slot_id) {
                        (None, None) => {
                            if left_wire.node_idx == right_wire.node_idx {
                                println!("adding: {:?}, {:?}", slot_ids[0], slot_ids[1]);
                                copy_constraints.push((slot_ids[0], slot_ids[1]));
                            }
                        }
                        (Some(l_slot_id), None) => {
                            println!(
                                "adding: {:?}({}), {:?}",
                                l_slot_id,
                                wire_values[left_wire.node_idx.index()],
                                slot_ids[0],
                            );
                            copy_constraints.push((l_slot_id, slot_ids[0]));
                        }
                        (None, Some(r_slot_id)) => {
                            println!("adding: {:?}, {:?}", r_slot_id, slot_ids[0]);
                            copy_constraints.push((r_slot_id, slot_ids[0]));
                        }
                        (Some(l_slot_id), Some(r_slot_id)) => {
                            println!(
                                "adding: {:?}({}), {:?}({})",
                                l_slot_id,
                                wire_values[left_wire.node_idx.index()],
                                slot_ids[0],
                                wire_values[node_idx.index()]
                            );
                            println!(
                                "adding: {:?}({}), {:?}({})",
                                r_slot_id,
                                wire_values[right_wire.node_idx.index()],
                                slot_ids[1],
                                wire_values[node_idx.index()]
                            );
                            copy_constraints.push((l_slot_id, slot_ids[0]));
                            copy_constraints.push((r_slot_id, slot_ids[1]));
                        }
                    };
                    println!("");
                }
            }
        }

        let output = wire_values
            .last()
            .copied()
            .ok_or_else(|| anyhow!("Empty circuit"))?;

        Ok(Trace::new(copy_constraints, ws, qs, output, n))
    }
}
