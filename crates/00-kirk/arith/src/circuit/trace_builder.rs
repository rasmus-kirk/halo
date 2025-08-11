use std::{array, collections::HashMap, time::Instant};

use anyhow::{Context, Result, anyhow, bail, ensure};
use halo_accumulation::acc::Accumulator;
use halo_group::{
    Fp, Fq, PallasConfig, PastaAffine, PastaFE, PastaFieldId, VestaConfig,
    ark_ff::{BigInt, BigInteger, PrimeField},
    ark_std::{One, Zero},
};
use log::debug;
use petgraph::algo::toposort;

use crate::{
    circuit::{CircuitSpec, GateType, Trace, Wire},
    utils::{MultiAssign, Q_POLYS, R_POLYS, W_POLYS},
};

use super::SlotId;

pub struct TraceBuilder {
    spec: CircuitSpec,
    witnesses: [HashMap<Wire, PastaFE>; 2],
    public_inputs: [HashMap<Wire, PastaFE>; 2],
    public_row_count: [usize; 2],
    message_pass_row_count: [usize; 2],
    row_count: [usize; 2],
}

impl TraceBuilder {
    fn get_slot_ids(&mut self, fid: PastaFieldId) -> [SlotId; W_POLYS] {
        let row = self.row_count[fid as usize]
            + self.spec.public_input_wire_count[fid as usize]
            + self.spec.message_pass_wire_count[fid as usize];
        self.row_count[fid as usize] += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        slot_ids
    }

    fn get_public_inputs_slot_ids(&mut self, fid: PastaFieldId) -> [SlotId; W_POLYS] {
        let row = self.public_row_count[fid as usize];
        self.public_row_count[fid as usize] += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        slot_ids
    }

    fn get_message_pass_slot_ids(&mut self, fid: PastaFieldId) -> [SlotId; W_POLYS] {
        let row = self.message_pass_row_count[fid as usize]
            + self.spec.public_input_wire_count[fid as usize];
        self.message_pass_row_count[fid as usize] += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        slot_ids
    }

    pub fn new(spec: CircuitSpec) -> Self {
        let witnesses = array::from_fn(|i| HashMap::with_capacity(spec.witness_wire_count[i]));
        let public_inputs =
            array::from_fn(|i| HashMap::with_capacity(spec.public_input_wire_count[i]));
        Self {
            row_count: [0, 0],
            public_row_count: [0, 0],
            message_pass_row_count: [0, 0],
            spec,
            witnesses,
            public_inputs,
        }
    }

    pub fn witness(&mut self, wire: Wire, w: PastaFE) -> Result<()> {
        let fid = wire.fid;
        assert_eq!(fid, w.fid.unwrap());
        match self.spec.graph.node_weight(wire.node_idx) {
            Some(GateType::Witness(..)) => (),
            Some(GateType::WitnessBool(..)) => (),
            Some(_) => bail!("The provided wire was not a witness wire!"),
            None => bail!("Wire does not exist, this should be impossible!"),
        }
        let already_assigned = self.witnesses[fid as usize].insert(wire, w).is_some();
        if already_assigned {
            bail!("Wire already assigned! ({wire:?}, {w:?})")
        };
        Ok(())
    }

    pub fn public_input(&mut self, wire: Wire, x: PastaFE) -> Result<()> {
        let fid = wire.fid;
        println!("{fid}: adding {wire:?}");
        assert_eq!(fid, x.fid.unwrap());
        match self.spec.graph.node_weight(wire.node_idx) {
            Some(GateType::PublicInput(..)) => (),
            Some(_) => bail!("The provided wire was not a public input wire!"),
            None => bail!("Wire does not exist, this should be impossible!"),
        }
        let already_assigned = self.public_inputs[fid as usize].insert(wire, x).is_some();
        if already_assigned {
            bail!("Wire already assigned! ({wire:?}, {x:?})")
        };

        // let v = self.public_inputs[fid as usize].get(&wire).unwrap();
        // println!("{fid}: got wire value {v:?} from {wire:?}");
        println!("{:?}", self.public_inputs[fid as usize]);

        Ok(())
    }

    pub fn trace(
        mut self,
        accs_prev: Option<(Accumulator<PallasConfig>, Accumulator<VestaConfig>)>,
    ) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
        let now = Instant::now();
        let spec = self.spec.clone();

        println!("FQS = {:?}", self.public_inputs[1 as usize]);

        let row_counts: [usize; 2] =
            array::from_fn(|i| spec.row_count[i].next_power_of_two().max(4));

        for i in 0..1 {
            let fid = if i == 0 {
                PastaFieldId::Fp
            } else {
                PastaFieldId::Fq
            };
            ensure!(
                self.witnesses[i].len() == spec.witness_wire_count[i],
                "{fid} Expected {} witnesses, got {}",
                spec.witness_wire_count[i],
                self.witnesses[i].len()
            );
            ensure!(
                self.public_inputs[i].len() == spec.public_input_wire_count[i],
                "{fid} Expected {} public inputs, got {}",
                spec.public_input_wire_count[i],
                self.public_inputs[i].len()
            );
        }

        let O = PastaFE::zero(None);
        let I = PastaFE::one(None);
        let NI = PastaFE::neg_one();
        let mut out_wires =
            [vec![O; spec.output_wire_count[0]], vec![O; spec.output_wire_count[1]]];
        let mut ws: [[Vec<PastaFE>; W_POLYS]; 2] = [
            array::from_fn(|_| vec![O; row_counts[0]]),
            array::from_fn(|_| vec![O; row_counts[1]]),
        ];
        let mut rs: [[Vec<PastaFE>; R_POLYS]; 2] = [
            array::from_fn(|_| vec![O; row_counts[0]]),
            array::from_fn(|_| vec![O; row_counts[1]]),
        ];
        let mut qs: [[Vec<PastaFE>; Q_POLYS]; 2] = [
            array::from_fn(|_| vec![O; row_counts[0]]),
            array::from_fn(|_| vec![O; row_counts[1]]),
        ];

        let topo_order =
            toposort(&spec.graph, None).map_err(|_| anyhow!("Cycle detected {spec:?}"))?;
        let mut wire_vals = [
            vec![PastaFE::zero(None); spec.wire_count[0]],
            vec![PastaFE::zero(None); spec.wire_count[1]],
        ];
        let mut copy_constraints =
            [vec![Vec::new(); spec.wire_count[0]], vec![Vec::new(); spec.wire_count[1]]];
        let mut wire_output_slots: [Vec<_>; 2] =
            [vec![None; spec.wire_count[0]], vec![None; spec.wire_count[1]]];

        let mut node_map = HashMap::new();
        let mut public_inputs = [
            Vec::with_capacity(spec.public_input_wire_count[0]),
            Vec::with_capacity(spec.public_input_wire_count[1]),
        ];
        let mut message_pass_inputs = [
            Vec::with_capacity(spec.message_pass_wire_count[0]),
            Vec::with_capacity(spec.message_pass_wire_count[1]),
        ];

        // Evaluate and collect inputs/outputs for gates
        for node_idx in topo_order {
            match spec.graph[node_idx] {
                GateType::Witness(_, [out_wire]) => {
                    let fid = out_wire.fid;
                    let v = self.witnesses[fid as usize]
                        .get(&out_wire)
                        .context("Wire unassigned!")?;
                    wire_vals[fid as usize][out_wire.id] = *v;
                }
                GateType::PublicInput(_, [out_wire]) => {
                    let fid = out_wire.fid;
                    let slots = self.get_public_inputs_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    println!("{:?}", self.public_inputs);
                    let v = *self.public_inputs[fid as usize]
                        .get(&out_wire)
                        .context(format!("{fid}: Wire unassigned ({out_wire:?})!"))?;
                    public_inputs[fid as usize].push(-v);
                    wire_vals[fid as usize][out_wire.id] = v;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r, o, m, c, p, +, *, +]
                    let q: [_; Q_POLYS] = [I, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [v, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    qs.multi_assign(row, q);
                    ws.multi_assign(row, w);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[0]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[0]);
                }
                GateType::Constant((), [out_wire], c) => {
                    let fid = out_wire.fid;

                    // ----- Values ----- //
                    wire_vals[fid as usize][out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let slots = self.get_slot_ids(fid);
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r,  o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, O, O, O, -c, O, O, O, O];
                    let w: [_; W_POLYS] = [c, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    qs.multi_assign(row, q);
                    ws.multi_assign(row, w);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[0]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[0]);
                }
                GateType::Output([in_wire], (), n) => {
                    let fid_idx = in_wire.fid as usize;
                    out_wires[fid_idx][n] = wire_vals[fid_idx][in_wire.id];
                }
                GateType::Print([in_wire], (), (label_1, label_2)) => {
                    let fid_idx = in_wire.fid as usize;
                    println!("{label_1}{label_2}: {:?}", wire_vals[fid_idx][in_wire.id]);
                }
                GateType::AssertEq([left_wire, right_wire], ()) => {
                    let fid = left_wire.fid;
                    let fid_idx = left_wire.fid as usize;

                    // ----- Copy Constraints ----- //
                    let slots = self.get_slot_ids(fid);
                    let l = wire_vals[fid as usize][left_wire.id];
                    let r = wire_vals[fid as usize][right_wire.id];
                    let q: [_; Q_POLYS] = [I, NI, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [l, r, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    qs[fid_idx].multi_assign(slots[0].row_0_indexed(), q);
                    ws[fid_idx].multi_assign(slots[0].row_0_indexed(), w);

                    copy_constraints[fid_idx][left_wire.id].push(slots[0]);
                    copy_constraints[fid_idx][right_wire.id].push(slots[1]);
                }
                GateType::Add([left_wire, right_wire], [out_wire]) => {
                    let fid = left_wire.fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let a = wire_vals[fid as usize][left_wire.id];
                    let b = wire_vals[fid as usize][right_wire.id];
                    let c = a + b;
                    wire_vals[fid as usize][out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r,  o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, I, NI, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[2]);
                    copy_constraints[fid as usize][left_wire.id].push(slots[0]);
                    copy_constraints[fid as usize][right_wire.id].push(slots[1]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[2]);
                }
                GateType::Multiply([left_wire, right_wire], [out_wire]) => {
                    let fid = left_wire.fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let a = wire_vals[fid as usize][left_wire.id];
                    let b = wire_vals[fid as usize][right_wire.id];
                    let c = a * b;
                    wire_vals[fid as usize][out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r,  o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, NI, I, O, O, O, O, O];
                    let w: [_; W_POLYS] = [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[2]);
                    copy_constraints[fid as usize][left_wire.id].push(slots[0]);
                    copy_constraints[fid as usize][right_wire.id].push(slots[1]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[2]);
                }
                GateType::Poseidon(in_wires, out_wires, r) => {
                    let fid = in_wires[0].fid;
                    let fid_idx = fid as usize;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let w0 = wire_vals[fid_idx][in_wires[0].id];
                    let w1 = wire_vals[fid_idx][in_wires[1].id];
                    let w2 = wire_vals[fid_idx][in_wires[2].id];

                    let (w3, w4, w5) = poseidon_round(fid, r[0], r[1], r[2], w0, w1, w2);
                    let (w6, w7, w8) = poseidon_round(fid, r[3], r[4], r[5], w3, w4, w5);
                    let (w9, w10, w11) = poseidon_round(fid, r[6], r[7], r[8], w6, w7, w8);
                    let (w12, w13, w14) = poseidon_round(fid, r[9], r[10], r[11], w9, w10, w11);
                    let (v0, v1, v2) = poseidon_round(fid, r[12], r[13], r[14], w12, w13, w14);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid_idx][wire.id] = v0,
                            1 => wire_vals[fid_idx][wire.id] = v1,
                            2 => wire_vals[fid_idx][wire.id] = v2,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs, rs) = (&mut ws[fid_idx], &mut qs[fid_idx], &mut rs[fid_idx]);
                    let w: [_; W_POLYS] =
                        [w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, O];
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, I, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);
                    rs.multi_assign(row, r);

                    // ----- Copy Constraints ----- //
                    copy_constraints[fid_idx][in_wires[0].id].push(slots[0]);
                    copy_constraints[fid_idx][in_wires[1].id].push(slots[1]);
                    copy_constraints[fid_idx][in_wires[2].id].push(slots[2]);
                }
                GateType::PoseidonEnd(in_wires, out_wires) => {
                    let fid = in_wires[0].fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let w0 = wire_vals[fid as usize][in_wires[0].id];
                    let w1 = wire_vals[fid as usize][in_wires[1].id];
                    let w2 = wire_vals[fid as usize][in_wires[2].id];

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid as usize][wire.id] = w0,
                            1 => wire_vals[fid as usize][wire.id] = w1,
                            2 => wire_vals[fid as usize][wire.id] = w2,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [w0, w1, w2, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wires[0].id] = Some(slots[0]);
                    wire_output_slots[fid as usize][out_wires[1].id] = Some(slots[1]);
                    wire_output_slots[fid as usize][out_wires[2].id] = Some(slots[2]);
                    copy_constraints[fid as usize][in_wires[0].id].push(slots[0]);
                    copy_constraints[fid as usize][in_wires[1].id].push(slots[1]);
                    copy_constraints[fid as usize][in_wires[2].id].push(slots[2]);
                }
                GateType::AffineAdd([xp_wire, yp_wire, xq_wire, yq_wire], out_wires) => {
                    let fid = xp_wire.fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    let xp = wire_vals[fid as usize][xp_wire.id];
                    let yp = wire_vals[fid as usize][yp_wire.id];
                    let xq = wire_vals[fid as usize][xq_wire.id];
                    let yq = wire_vals[fid as usize][yq_wire.id];
                    let p = PastaAffine::new(xp, yp);
                    let q = PastaAffine::new(xq, yq);
                    let r = p + q;
                    let (xr, yr) = (r.x, r.y);
                    let [α, β, γ, δ, λ] = affine_add_params(p, q);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid as usize][wire.id] = xr,
                            1 => wire_vals[fid as usize][wire.id] = yr,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    let w: [_; W_POLYS] = [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, O, O, O, O, O];
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, I, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wires[0].id] = Some(slots[4]);
                    wire_output_slots[fid as usize][out_wires[1].id] = Some(slots[5]);
                    copy_constraints[fid as usize][xp_wire.id].push(slots[0]);
                    copy_constraints[fid as usize][yp_wire.id].push(slots[1]);
                    copy_constraints[fid as usize][xq_wire.id].push(slots[2]);
                    copy_constraints[fid as usize][yq_wire.id].push(slots[3]);
                    copy_constraints[fid as usize][out_wires[0].id].push(slots[4]);
                    copy_constraints[fid as usize][out_wires[1].id].push(slots[5]);
                }
                GateType::Invert([in_wire, one_wire], [out_wire]) => {
                    let fid = in_wire.fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let x = wire_vals[fid as usize][in_wire.id];
                    let one = wire_vals[fid as usize][one_wire.id];
                    let x_inv = x.inverse().context("x has no inverse!")?;
                    wire_vals[fid as usize][out_wire.id] = x_inv;
                    assert_eq!(one, I);

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    let w: [_; W_POLYS] = [x, x_inv, I, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    //                    [l, r,  o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, NI, I, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[1]);
                    copy_constraints[fid as usize][in_wire.id].push(slots[0]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[1]);
                    copy_constraints[fid as usize][one_wire.id].push(slots[2]);
                }
                GateType::Negate([in_wire, zero_wire], [out_wire]) => {
                    let fid = in_wire.fid;

                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let x = wire_vals[fid as usize][in_wire.id];
                    let zero = wire_vals[fid as usize][zero_wire.id];
                    assert_eq!(zero, O);
                    let x_neg = -x;
                    wire_vals[fid as usize][out_wire.id] = x_neg;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r,  o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, I, NI, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [x, x_neg, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[1]);
                    copy_constraints[fid as usize][in_wire.id].push(slots[0]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[1]);
                    copy_constraints[fid as usize][zero_wire.id].push(slots[2]);
                }
                GateType::FpMessagePass([in_wire], out_wires) => {
                    let fid = in_wire.fid;
                    let fid_idx = fid as usize;
                    let fid_idx_inv = fid.inv() as usize;

                    let x = wire_vals[fid_idx][in_wire.id];

                    let bits = x.into_bigint().to_bits_le();
                    let low_bit = match bits[0] {
                        true => Fq::one(),
                        false => Fq::zero(),
                    };
                    let high_bits =
                        Fq::from_bigint(BigInt::<4>::from_bits_le(&bits[1..bits.len()])).unwrap();

                    let h = high_bits.into();
                    let l = low_bit.into();

                    let (ws, qs) = (&mut ws[fid_idx_inv], &mut qs[fid_idx_inv]);

                    let slots = self.get_message_pass_slot_ids(fid.inv());
                    let row = slots[0].row_0_indexed();
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [h, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);
                    message_pass_inputs[fid_idx_inv].push(-h);
                    wire_output_slots[fid_idx_inv][out_wires[0].id] = Some(slots[0]);
                    copy_constraints[fid_idx_inv][out_wires[0].id].push(slots[0]);

                    let slots = self.get_message_pass_slot_ids(fid.inv());
                    let row = slots[0].row_0_indexed();
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [l, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);
                    message_pass_inputs[fid_idx_inv].push(-l);
                    wire_output_slots[fid_idx_inv][out_wires[1].id] = Some(slots[0]);
                    copy_constraints[fid_idx_inv][out_wires[1].id].push(slots[0]);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid_idx_inv][wire.id] = h,
                            1 => wire_vals[fid_idx_inv][wire.id] = l,
                            _ => unreachable!(),
                        }
                    }
                }
                GateType::FqMessagePass([in_wire], out_wires) => {
                    let fid = in_wire.fid;
                    let fid_idx = fid as usize;
                    let fid_idx_inv = fid.inv() as usize;

                    let v_fp = wire_vals[fid_idx][in_wire.id];
                    let v_fq = PastaFE::new(v_fp.into_bigint(), Some(fid.inv()));

                    let (ws, qs) = (&mut ws[fid_idx_inv], &mut qs[fid_idx_inv]);
                    let slots = self.get_message_pass_slot_ids(fid.inv());
                    let row = slots[0].row_0_indexed();
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [I, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] = [v_fq, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    //                   [l, r, o, m, c, p, +, *]
                    qs.multi_assign(row, q);
                    message_pass_inputs[fid_idx_inv].push(-v_fq);
                    wire_output_slots[fid_idx_inv][out_wires[0].id] = Some(slots[0]);
                    copy_constraints[fid_idx_inv][out_wires[0].id].push(slots[0]);

                    wire_vals[fid_idx_inv][out_wires[0].id] = v_fq;
                }
                GateType::ScalarMulPallas(in_wires, out_wires) => {
                    let fid = in_wires[0].fid;
                    let fid_idx = fid as usize;

                    // ----- Values ----- //
                    let h = wire_vals[fid_idx][in_wires[0].id];
                    let l = wire_vals[fid_idx][in_wires[1].id];
                    let xg = wire_vals[fid_idx][in_wires[2].id];
                    let yg = wire_vals[fid_idx][in_wires[3].id];
                    let g = PastaAffine::new(xg, yg);

                    // 2 most significant bits are always zero, so remove them
                    let mut bits_lsb_to_msb = h.into_bigint().to_bits_le();
                    assert!(!bits_lsb_to_msb.pop().unwrap());
                    assert!(!bits_lsb_to_msb.pop().unwrap());
                    let bits: Vec<bool> = bits_lsb_to_msb.into_iter().rev().collect();

                    let mut point_acc = PastaAffine::identity(Some(fid));
                    let mut bit_acc = PastaFE::zero(Some(fid));

                    let is = (0..bits.len()).rev();
                    for (bit, i) in bits.into_iter().zip(is) {
                        let slots = self.get_slot_ids(fid);

                        let pow_2i = PastaFE::from_u64(2u64, Some(fid)).pow(i);
                        let b = PastaFE::from_bool(bit, Some(fid));
                        let p = point_acc;

                        let [βq, λq] = affine_double_params(p);
                        let q = p + p;

                        let [αr, _, γr, δr, λr] = affine_add_params(q, g);
                        let r = q + g;
                        let a = bit_acc;

                        point_acc = if bit { r } else { q };
                        bit_acc = bit_acc + b * pow_2i;

                        // ----- Gate Constraints ----- //
                        let row = slots[0].row_0_indexed();
                        let (ws, qs, rs) = (&mut ws[fid_idx], &mut qs[fid_idx], &mut rs[fid_idx]);
                        let w: [_; W_POLYS] =
                            [p.x, p.y, a, g.x, g.y, b, q.x, q.y, r.x, r.y, βq, λq, αr, γr, δr, λr];
                        //                    [l, r, o, m, c, p, +, *, =]
                        let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, I, O];
                        let r: [_; R_POLYS] = [pow_2i, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                        ws.multi_assign(row, w);
                        qs.multi_assign(row, q);
                        rs.multi_assign(row, r);
                    }

                    let slots = self.get_slot_ids(fid);

                    let pow_2i = PastaFE::from_u64(2u64, Some(fid)).pow(0);
                    let b = l;
                    let p = point_acc;

                    let [βq, λq] = affine_double_params(p);
                    let q = p + p;

                    let [αr, _, γr, δr, λr] = affine_add_params(q, g);
                    let r = q + g;
                    let a = bit_acc;

                    point_acc = if l == PastaFE::one(None) { r } else { q };
                    bit_acc = bit_acc + b * pow_2i;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs, rs) = (&mut ws[fid_idx], &mut qs[fid_idx], &mut rs[fid_idx]);

                    let w: [_; W_POLYS] =
                        [p.x, p.y, a, g.x, g.y, b, q.x, q.y, r.x, r.y, βq, λq, αr, γr, δr, λr];
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, I, O];
                    let r: [_; R_POLYS] = [pow_2i, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);
                    rs.multi_assign(row, r);

                    // wire_slots[fid as usize][out_wires[1].id].push(slots[5]);

                    // ----- Zero Row Gate Constraints ----- //
                    let slots = self.get_slot_ids(fid);
                    let row = slots[0].row_0_indexed();
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] =
                        [point_acc.x, point_acc.y, bit_acc, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws.multi_assign(row, w);
                    qs.multi_assign(row, q);

                    node_map.insert(node_idx, slots);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid_idx][wire.id] = point_acc.x,
                            1 => wire_vals[fid_idx][wire.id] = point_acc.y,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wires[0].id] = Some(slots[0]);
                    wire_output_slots[fid as usize][out_wires[1].id] = Some(slots[1]);
                    // wire_slots[fid as usize][out_wires[0].id].push(slots[0]);
                    // wire_slots[fid as usize][out_wires[1].id].push(slots[1]);
                }
                GateType::ScalarMulVesta(in_wires, out_wires) => {
                    let fid = in_wires[0].fid;
                    let fid_idx = fid as usize;

                    // ----- Values ----- //
                    let v = wire_vals[fid_idx][in_wires[0].id];
                    let xg = wire_vals[fid_idx][in_wires[1].id];
                    let yg = wire_vals[fid_idx][in_wires[2].id];
                    let g = PastaAffine::new(xg, yg);

                    // most significant bits is always zero, so remove it
                    let mut bits_lsb_to_msb = v.into_bigint().to_bits_le();
                    assert!(!bits_lsb_to_msb.pop().unwrap());
                    let bits: Vec<bool> = bits_lsb_to_msb.into_iter().rev().collect();

                    let mut point_acc = PastaAffine::identity(Some(fid));
                    let mut bit_acc = PastaFE::zero(Some(fid));

                    let is = (0..bits.len()).rev();
                    for (bit, i) in bits.into_iter().zip(is) {
                        let slots = self.get_slot_ids(fid);

                        let pow_2i = PastaFE::from_u64(2u64, Some(fid)).pow(i);
                        let b = PastaFE::from_bool(bit, Some(fid));
                        let p = point_acc;

                        let [βq, λq] = affine_double_params(p);
                        let q = p + p;

                        let [αr, _, γr, δr, λr] = affine_add_params(q, g);
                        let r = q + g;
                        let a = bit_acc;

                        point_acc = if bit { r } else { q };
                        bit_acc = bit_acc + b * pow_2i;

                        // ----- Gate Constraints ----- //
                        let row = slots[0].row_0_indexed();
                        let w: [_; W_POLYS] =
                            [p.x, p.y, a, g.x, g.y, b, q.x, q.y, r.x, r.y, βq, λq, αr, γr, δr, λr];
                        //                    [l, r, o, m, c, p, +, *, =]
                        let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, I, O];
                        let r: [_; R_POLYS] = [pow_2i, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                        ws[fid_idx].multi_assign(row, w);
                        qs[fid_idx].multi_assign(row, q);
                        rs[fid_idx].multi_assign(row, r);
                    }

                    // ----- Zero Row Gate Constraints ----- //
                    let slots = self.get_slot_ids(fid);
                    let row = slots[0].row_0_indexed();
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, O, O];
                    let w: [_; W_POLYS] =
                        [point_acc.x, point_acc.y, bit_acc, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    ws[fid_idx].multi_assign(row, w);
                    qs[fid_idx].multi_assign(row, q);

                    node_map.insert(node_idx, slots);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[fid_idx][wire.id] = point_acc.x,
                            1 => wire_vals[fid_idx][wire.id] = point_acc.y,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wires[0].id] = Some(slots[0]);
                    wire_output_slots[fid as usize][out_wires[1].id] = Some(slots[1]);
                    // wire_slots[fid as usize][out_wires[0].id].push(slots[0]);
                    // wire_slots[fid as usize][out_wires[1].id].push(slots[1]);
                }
                GateType::WitnessBool(_, [out_wire]) => {
                    let fid = out_wire.fid;
                    let slots = self.get_slot_ids(fid);
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let v = *self.witnesses[fid as usize]
                        .get(&out_wire)
                        .context("Wire unassigned!")?;
                    wire_vals[fid as usize][out_wire.id] = v;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l,  r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [NI, O, O, I, O, O, O, O, O];
                    let w: [_; W_POLYS] = [v, v, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
                    qs.multi_assign(row, q);
                    ws.multi_assign(row, w);

                    // ----- Copy Constraints ----- //
                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[0]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[0]);
                }
                GateType::Eq([a, b], [out_wire]) => {
                    let fid = out_wire.fid;

                    let a_v = wire_vals[fid as usize][a.id];
                    let b_v = wire_vals[fid as usize][b.id];
                    let zero = PastaFE::zero(Some(fid));
                    let one = PastaFE::one(Some(fid));
                    let diff = a_v - b_v;
                    let inv = diff.inv0();
                    let eq = if a_v == b_v { one } else { zero };
                    wire_vals[fid as usize][out_wire.id] = eq;

                    // ----- Gate Constraints ----- //
                    let slots = self.get_slot_ids(fid);
                    let row = slots[0].row_0_indexed();
                    let (ws, qs) = (&mut ws[fid as usize], &mut qs[fid as usize]);
                    //                    [l, r, o, m, c, p, +, *, =]
                    let q: [_; Q_POLYS] = [O, O, O, O, O, O, O, O, I];
                    let w: [_; W_POLYS] = [a_v, b_v, one, eq, inv, O, O, O, O, O, O, O, O, O, O, O];
                    qs.multi_assign(row, q);
                    ws.multi_assign(row, w);

                    wire_output_slots[fid as usize][out_wire.id] = Some(slots[3]);
                    copy_constraints[fid as usize][a.id].push(slots[0]);
                    copy_constraints[fid as usize][b.id].push(slots[1]);
                    copy_constraints[fid as usize][spec.one[fid as usize].id].push(slots[2]);
                    copy_constraints[fid as usize][out_wire.id].push(slots[3]);
                }
            }
        }

        debug!("trace_builder_time: {:?}", now.elapsed().as_secs_f32());

        public_inputs[0].extend_from_slice(&message_pass_inputs[0]);
        public_inputs[1].extend_from_slice(&message_pass_inputs[1]);

        let [fp_ws, fq_ws] = ws;
        let [fp_rs, fq_rs] = rs;
        let [fp_qs, fq_qs] = qs;
        let [fp_public_inputs, fq_public_inputs] = public_inputs;
        let [fp_copy_constraints, fq_copy_constraints] = copy_constraints;
        let [fp_out_wires, fq_out_wires] = out_wires;
        let [fp_rows, fq_rows] = row_counts;
        let (fp_acc_prev, fq_acc_prev) = match accs_prev {
            None => (Accumulator::zero(fp_rows, 2), Accumulator::zero(fq_rows, 2)),
            Some((fp, fq)) => (fp, fq),
        };

        let fp_public_inputs: Vec<Fp> = fp_public_inputs.into_iter().map(Fp::from).collect();
        let fp_ws = fp_ws.map(|x| x.into_iter().map(Fp::from).collect());
        let fp_rs = fp_rs.map(|x| x.into_iter().map(Fp::from).collect());
        let fp_qs = fp_qs.map(|x| x.into_iter().map(Fp::from).collect());
        let fp_out_wires = fp_out_wires.into_iter().map(Fp::from).collect();
        let fp_trace = Trace::<PallasConfig>::new(
            fp_copy_constraints,
            fp_public_inputs,
            fp_ws,
            fp_rs,
            fp_qs,
            fp_out_wires,
            fp_rows,
            fp_acc_prev,
        );

        let fq_public_inputs: Vec<Fq> = fq_public_inputs.into_iter().map(Fq::from).collect();
        let fq_ws = fq_ws.map(|x| x.into_iter().map(Fq::from).collect());
        let fq_rs = fq_rs.map(|x| x.into_iter().map(Fq::from).collect());
        let fq_qs = fq_qs.map(|x| x.into_iter().map(Fq::from).collect());
        let fq_out_wires = fq_out_wires.into_iter().map(Fq::from).collect();
        let fq_trace = Trace::<VestaConfig>::new(
            fq_copy_constraints,
            fq_public_inputs,
            fq_ws,
            fq_rs,
            fq_qs,
            fq_out_wires,
            fq_rows,
            fq_acc_prev,
        );

        Ok((fp_trace, fq_trace))
    }
}

fn poseidon_round(
    fid: PastaFieldId,
    r0: PastaFE,
    r1: PastaFE,
    r2: PastaFE,
    w0: PastaFE,
    w1: PastaFE,
    w2: PastaFE,
) -> (PastaFE, PastaFE, PastaFE) {
    let sbox = |w: PastaFE| w.pow(7);
    let M = fid.poseidon_mde_matrix();
    (
        r0 + (M[0][0] * sbox(w0) + M[0][1] * sbox(w1) + M[0][2] * sbox(w2)),
        r1 + (M[1][0] * sbox(w0) + M[1][1] * sbox(w1) + M[1][2] * sbox(w2)),
        r2 + (M[2][0] * sbox(w0) + M[2][1] * sbox(w1) + M[2][2] * sbox(w2)),
    )
}

fn affine_add_params(p: PastaAffine, q: PastaAffine) -> [PastaFE; 5] {
    let xp = p.x;
    let yp = p.y;
    let xq = q.x;
    let yq = q.y;
    let one = PastaFE::one(None);
    let zero = PastaFE::zero(None);
    let two = PastaFE::from(2);
    let three = PastaFE::from(3);
    let inv0 = |x: PastaFE| {
        if x.is_zero() { zero } else { one / x }
    };
    let alpha = inv0(xq - xp);
    let beta = inv0(xp);
    let gamma = inv0(xq);
    let delta = if xq == xp { inv0(yq + yp) } else { zero };
    let lambda = if xq != xp {
        (yq - yp) / (xq - xp)
    } else if xq == xp && !yp.is_zero() {
        (three * xp.square()) / (two * yp)
    } else {
        zero
    };

    [alpha, beta, gamma, delta, lambda]
}

fn affine_double_params(acc: PastaAffine) -> [PastaFE; 2] {
    let fid = acc.x.fid;
    let zero = PastaFE::zero(fid);
    let one = PastaFE::one(fid);
    let two = PastaFE::from_u64(2, fid);
    let three = PastaFE::from_u64(3, fid);

    let xp = acc.x;
    let yp = acc.y;

    let inv0 = |x: PastaFE| {
        if x.is_zero() { zero } else { one / x }
    };
    let beta = inv0(xp);
    let lambda = if !yp.is_zero() {
        (three * xp.square()) / (two * yp)
    } else {
        zero
    };

    let p = acc;
    let q = p + p;

    assert_eq!((one - p.x * beta) * q.x, zero);
    assert_eq!((one - p.x * beta) * q.y, zero);
    assert_eq!(two * p.y * lambda - three * p.x.square(), zero);
    assert_eq!(lambda.square() - (two * p.x) - q.x, zero);
    assert_eq!(q.y, lambda * (p.x - q.x) - p.y);

    [beta, lambda]
}
