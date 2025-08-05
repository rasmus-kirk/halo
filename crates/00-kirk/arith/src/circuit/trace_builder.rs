use std::{array, collections::HashMap, time::Instant};

use anyhow::{Context, Result, anyhow, bail, ensure};
use halo_group::{
    Affine, Domain, Evals, PastaConfig, Point, Scalar,
    ark_ec::CurveConfig,
    ark_ec::CurveGroup,
    ark_ff::{BigInteger, Field},
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
            Some(GateType::Witness(..)) => (),
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
            Some(GateType::PublicInput(..)) => (),
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

        let n = match spec.is_empty() {
            true => 4,
            false => spec.row_count.next_power_of_two(),
        };

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
        let mut out_wires = vec![O; spec.output_wire_count];
        let mut ws: [_; WITNESS_POLYS] = array::from_fn(|_| vec![O; n]);
        let mut rs: [_; WITNESS_POLYS] = array::from_fn(|_| vec![O; n]);
        let mut qs: [_; SELECTOR_POLYS] = array::from_fn(|_| vec![O; n]);

        let topo_order = toposort(&spec.graph, None).map_err(|_| anyhow!("Cycle detected"))?;
        let mut wire_vals = vec![Scalar::<P>::zero(); spec.wire_count];
        let mut wire_slots = vec![Vec::new(); spec.wire_count];
        let mut node_map = HashMap::new();
        let mut public_inputs = Vec::new();

        // Evaluate and collect inputs/outputs for gates
        for node_idx in topo_order {
            match spec.graph[node_idx] {
                GateType::Witness(_, [out_wire]) => {
                    let v = self.witnesses.get(&out_wire).context("Wire unassigned!")?;
                    wire_vals[out_wire.id] = *v;
                }
                GateType::PublicInput(_, [out_wire]) => {
                    let slots = self.get_witness_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let v = *self
                        .public_inputs
                        .get(&out_wire)
                        .context("Wire unassigned!")?;
                    public_inputs.push(-v);
                    wire_vals[out_wire.id] = v;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [v, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r, o, m, c, p, +]
                    qs.multi_assign(row, [I, O, O, O, O, O, O]);

                    // ----- Copy Constraints ----- //
                    wire_slots[out_wire.id].push(slots[0]);
                }
                GateType::Constant((), [out_wire], c) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    wire_vals[out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [c, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p, +]
                    qs.multi_assign(row, [I, O, O, O, -c, O, O]);
                    // ----- Copy Constraints ----- //
                    wire_slots[out_wire.id].push(slots[0]);
                }
                GateType::Output([in_wire], (), n) => {
                    out_wires[n] = wire_vals[in_wire.id];
                }
                GateType::AssertEq([left_wire, right_wire], ()) => {
                    // ----- Copy Constraints ----- //
                    let left_slots = node_map[&left_wire.node_idx];
                    let right_slots = node_map[&right_wire.node_idx];
                    wire_slots[left_wire.id].push(right_slots[right_wire.output_id as usize]);
                    wire_slots[right_wire.id].push(left_slots[left_wire.output_id as usize]);
                }
                GateType::Add([left_wire, right_wire], [out_wire]) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let a = wire_vals[left_wire.id];
                    let b = wire_vals[right_wire.id];
                    let c = a + b;
                    wire_vals[out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p, +]
                    qs.multi_assign(row, [I, I, -I, O, O, O, O]);
                    rs.multi_assign(row, [O; WITNESS_POLYS]);

                    // ----- Copy Constraints ----- //
                    wire_slots[left_wire.id].push(slots[0]);
                    wire_slots[right_wire.id].push(slots[1]);
                    wire_slots[out_wire.id].push(slots[2]);
                }
                GateType::Multiply([left_wire, right_wire], [out_wire]) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let a = wire_vals[left_wire.id];
                    let b = wire_vals[right_wire.id];
                    let c = a * b;
                    wire_vals[out_wire.id] = c;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [a, b, c, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p, +]
                    qs.multi_assign(row, [O, O, -I, I, O, O, O]);

                    // ----- Copy Constraints ----- //
                    wire_slots[left_wire.id].push(slots[0]);
                    wire_slots[right_wire.id].push(slots[1]);
                    wire_slots[out_wire.id].push(slots[2]);
                }
                GateType::Poseidon(in_wires, out_wires, r) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let w00 = wire_vals[in_wires[0].id];
                    let w01 = wire_vals[in_wires[1].id];
                    let w02 = wire_vals[in_wires[2].id];

                    let (w03, w04, w05) = poseidon_round::<P>(r[0], r[1], r[2], w00, w01, w02);
                    let (w06, w07, w08) = poseidon_round::<P>(r[3], r[4], r[5], w03, w04, w05);
                    let (w09, w10, w11) = poseidon_round::<P>(r[6], r[7], r[8], w06, w07, w08);
                    let (w12, w13, w14) = poseidon_round::<P>(r[9], r[10], r[11], w09, w10, w11);
                    let (v0, v1, v2) = poseidon_round::<P>(r[12], r[13], r[14], w12, w13, w14);

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[wire.id] = v0,
                            1 => wire_vals[wire.id] = v1,
                            2 => wire_vals[wire.id] = v2,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(
                        row,
                        [w00, w01, w02, w03, w04, w05, w06, w07, w08, w09, w10, w11, w12, w13, w14],
                    );
                    //                   [l, r, o, m, c, p, +]
                    qs.multi_assign(row, [O, O, O, O, O, I, O]);
                    rs.multi_assign(row, r);

                    // ----- Copy Constraints ----- //
                    wire_slots[in_wires[0].id].push(slots[0]);
                    wire_slots[in_wires[1].id].push(slots[1]);
                    wire_slots[in_wires[2].id].push(slots[2]);
                }
                GateType::PoseidonEnd(in_wires, out_wires) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let w0 = wire_vals[in_wires[0].id];
                    let w1 = wire_vals[in_wires[1].id];
                    let w2 = wire_vals[in_wires[2].id];

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[wire.id] = w0,
                            1 => wire_vals[wire.id] = w1,
                            2 => wire_vals[wire.id] = w2,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [w0, w1, w2, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r, o, m, c, p, +]
                    qs.multi_assign(row, [O, O, O, O, O, O, O]);
                    rs.multi_assign(row, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);

                    // ----- Copy Constraints ----- //
                    wire_slots[in_wires[0].id].push(slots[0]);
                    wire_slots[in_wires[1].id].push(slots[1]);
                    wire_slots[in_wires[2].id].push(slots[2]);
                }
                GateType::CurveAdd([xp_wire, yp_wire, xq_wire, yq_wire], out_wires) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    let xp = wire_vals[xp_wire.id];
                    let yp = wire_vals[yp_wire.id];
                    let xq = wire_vals[xq_wire.id];
                    let yq = wire_vals[yq_wire.id];
                    let (xr, yr) = add_affine::<P>((xp, yp), (xq, yq));
                    let [α, β, γ, δ, λ] = add_affine_params::<P>((xp, yp), (xq, yq));

                    for wire in out_wires {
                        match wire.output_id {
                            0 => wire_vals[wire.id] = xr,
                            1 => wire_vals[wire.id] = yr,
                            _ => unreachable!(),
                        }
                    }

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, O, O, O, O]);
                    //                   [l, r, o, m, c, p, +]
                    qs.multi_assign(row, [O, O, O, O, O, O, I]);
                    rs.multi_assign(row, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);

                    // ----- Copy Constraints ----- //
                    wire_slots[xp_wire.id].push(slots[0]);
                    wire_slots[yp_wire.id].push(slots[1]);
                    wire_slots[xq_wire.id].push(slots[2]);
                    wire_slots[yq_wire.id].push(slots[3]);
                    wire_slots[out_wires[0].id].push(slots[4]);
                    wire_slots[out_wires[1].id].push(slots[5]);
                }
                GateType::Invert([in_wire, one_wire], [out_wire]) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let x = wire_vals[in_wire.id];
                    let one = wire_vals[one_wire.id];
                    let x_inv = x.inverse().context("x has no inverse!")?;
                    wire_vals[out_wire.id] = x_inv;
                    assert_eq!(one, I);

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [x, x_inv, I, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p, +]
                    qs.multi_assign(row, [O, O, -I, I, O, O, O]);

                    // ----- Copy Constraints ----- //
                    wire_slots[in_wire.id].push(slots[0]);
                    wire_slots[out_wire.id].push(slots[1]);
                    wire_slots[one_wire.id].push(slots[2]);
                }
                GateType::Negate([in_wire, zero_wire], [out_wire]) => {
                    let slots = self.get_slot_ids();
                    node_map.insert(node_idx, slots);

                    // ----- Values ----- //
                    let x = wire_vals[in_wire.id];
                    let zero = wire_vals[zero_wire.id];
                    assert_eq!(zero, O);
                    let x_neg = -x;
                    wire_vals[out_wire.id] = x_neg;

                    // ----- Gate Constraints ----- //
                    let row = slots[0].row_0_indexed();
                    ws.multi_assign(row, [x, x_neg, O, O, O, O, O, O, O, O, O, O, O, O, O]);
                    //                   [l, r,  o, m, c, p, +]
                    qs.multi_assign(row, [I, I, -I, O, O, O, O]);
                    rs.multi_assign(row, [O; WITNESS_POLYS]);

                    // ----- Copy Constraints ----- //
                    wire_slots[in_wire.id].push(slots[0]);
                    wire_slots[out_wire.id].push(slots[1]);
                    wire_slots[zero_wire.id].push(slots[2]);
                }
            }
        }

        debug!("trace_builder_time: {:?}", now.elapsed().as_secs_f32());

        let trace = Trace::new(wire_slots, public_inputs, ws, rs, qs, out_wires, n);
        Ok(trace)
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
    r0: Scalar<P>,
    r1: Scalar<P>,
    r2: Scalar<P>,
    w0: Scalar<P>,
    w1: Scalar<P>,
    w2: Scalar<P>,
) -> (Scalar<P>, Scalar<P>, Scalar<P>) {
    let M = P::SCALAR_POSEIDON_MDS;
    (
        r0 + (M[0][0] * sbox(w0) + M[0][1] * sbox(w1) + M[0][2] * sbox(w2)),
        r1 + (M[1][0] * sbox(w0) + M[1][1] * sbox(w1) + M[1][2] * sbox(w2)),
        r2 + (M[2][0] * sbox(w0) + M[2][1] * sbox(w1) + M[2][2] * sbox(w2)),
    )
}

fn add_affine<P: PastaConfig>(
    p: (Scalar<P>, Scalar<P>),
    q: (Scalar<P>, Scalar<P>),
) -> (Scalar<P>, Scalar<P>) {
    let (px, py) = p;
    let (qx, qy) = q;
    let px = P::OtherCurve::basefield_from_bigint(P::scalar_into_bigint(px)).unwrap();
    let py = P::OtherCurve::basefield_from_bigint(P::scalar_into_bigint(py)).unwrap();
    let qx = P::OtherCurve::basefield_from_bigint(P::scalar_into_bigint(qx)).unwrap();
    let qy = P::OtherCurve::basefield_from_bigint(P::scalar_into_bigint(qy)).unwrap();

    let p_affine = if px.is_zero() && py.is_zero() {
        Affine::<P::OtherCurve>::identity()
    } else {
        Affine::<P::OtherCurve>::new(px, py)
    };
    let q_affine = if qx.is_zero() && qy.is_zero() {
        Affine::<P::OtherCurve>::identity()
    } else {
        Affine::<P::OtherCurve>::new(qx, qy)
    };

    let r_affine = (p_affine + q_affine).into_affine();
    let rx = P::scalar_from_bigint(P::OtherCurve::basefield_into_bigint(r_affine.x)).unwrap();
    let ry = P::scalar_from_bigint(P::OtherCurve::basefield_into_bigint(r_affine.y)).unwrap();

    (rx, ry)
}

fn add_affine_params<P: PastaConfig>(
    p: (Scalar<P>, Scalar<P>),
    q: (Scalar<P>, Scalar<P>),
) -> [Scalar<P>; 5] {
    let (xp, yp) = p;
    let (xq, yq) = q;
    let one = Scalar::<P>::one();
    let zero = Scalar::<P>::zero();
    let two = P::scalar_from_u64(2);
    let three = P::scalar_from_u64(3);
    let inv0 = |x: Scalar<P>| {
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
