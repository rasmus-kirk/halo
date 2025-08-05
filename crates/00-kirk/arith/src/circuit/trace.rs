#![allow(non_snake_case)]

use std::{array, collections::HashMap, time::Instant};

use anyhow::Result;
use halo_accumulation::pcdl;
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::Field,
    ark_poly::{EvaluationDomain, Polynomial},
    ark_std::Zero,
};
use log::debug;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::{
    circuit::SlotId,
    utils::{SELECTOR_POLYS, WITNESS_POLYS},
};

pub(crate) fn build_sigma<P: PastaConfig>(
    eqs: Vec<Vec<SlotId>>,
    domain: Domain<P>,
) -> (
    Vec<SlotId>,
    [Evals<P>; WITNESS_POLYS],
    [Evals<P>; WITNESS_POLYS],
) {
    let rows = domain.size();
    assert!(rows.is_power_of_two());

    // 2. Initialize identity permutation
    let id = (0..(rows * WITNESS_POLYS))
        .map(|i| SlotId::from_usize(i, rows))
        .collect::<Vec<_>>();

    // 3. For each non-trivial equivalence class, form a cycle
    let mut sigma = id.clone();
    for wires in eqs {
        if wires.len() <= 1 {
            continue;
        }

        for i in 0..wires.len() {
            let from = wires[i];
            let to = wires[(i + 1) % wires.len()];
            sigma[from.to_usize(rows)] = to;
        }
    }

    // let sigma_evals: [Evals<P>; WITNESS_POLYS] = array::from_fn(|i| {
    //     let evaluations: Vec<_> = sigma
    //         .iter()
    //         .skip(rows * i)
    //         .take(rows)
    //         .map(|slot_id| slot_id.to_scalar::<P>(rows))
    //         .collect();
    //     Evals::<P>::from_vec_and_domain(evaluations, domain)
    // });

    let mut id_vecs: [Vec<_>; WITNESS_POLYS] = array::from_fn(|_| Vec::with_capacity(rows));
    let mut sigma_vecs: [Vec<_>; WITNESS_POLYS] = array::from_fn(|_| Vec::with_capacity(rows));
    for (i, (id_chunk, sigma_chunk)) in id.chunks(rows).zip(sigma.chunks(rows)).enumerate() {
        for (id, sigma) in id_chunk.iter().zip(sigma_chunk) {
            id_vecs[i].push(id.to_scalar::<P>(rows));
            sigma_vecs[i].push(sigma.to_scalar::<P>(rows));
        }
    }

    (
        sigma,
        id_vecs.map(|vec| Evals::from_vec_and_domain(vec, domain)),
        sigma_vecs.map(|vec| Evals::from_vec_and_domain(vec, domain)),
    )
}

/// Extracts the permutation vector π(w) from a union-find structure:
/// each cycle represents a set of wires that are equal under copy constraints,
/// and π maps each wire to the next one in its cycle.
pub(crate) fn build_pi(mut uf: QuickUnionUf<UnionBySize>) -> Vec<SlotId> {
    let n = uf.size() / WITNESS_POLYS;

    // 1. Group wires by their equivalence class representative
    let mut classes: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..uf.size() {
        let repr = uf.find(i);
        classes.entry(repr).or_default().push(i);
    }

    // 2. Initialize identity permutation
    let mut pi = (0..uf.size())
        .map(|i| SlotId::from_usize(i, n))
        .collect::<Vec<_>>();

    // 3. For each non-trivial equivalence class, form a cycle
    for wires in classes.values() {
        if wires.len() <= 1 {
            continue;
        }

        for i in 0..wires.len() {
            let from = wires[i];
            let to = wires[(i + 1) % wires.len()];
            let slot = SlotId::from_usize(to, n);
            pi[from] = slot;
        }
    }

    pi
}

#[derive(Clone)]
pub struct Trace<P: PastaConfig> {
    pub(crate) rows: usize,
    pub(crate) omega: Scalar<P>,
    pub(crate) domain: Domain<P>,
    pub(crate) outputs: Vec<Scalar<P>>,
    pub(crate) sigma: Vec<SlotId>,
    pub(crate) public_inputs: Vec<Scalar<P>>,
    pub(crate) public_inputs_poly: Poly<P>,
    pub(crate) public_inputs_evals: Evals<P>,
    pub(crate) C_qs: [Point<P>; SELECTOR_POLYS],
    pub(crate) id_evals: [Evals<P>; WITNESS_POLYS],
    pub(crate) id_polys: [Poly<P>; WITNESS_POLYS],
    pub(crate) q_evals: [Evals<P>; SELECTOR_POLYS],
    pub(crate) q_polys: [Poly<P>; SELECTOR_POLYS],
    pub(crate) sigma_evals: [Evals<P>; WITNESS_POLYS],
    pub(crate) sigma_polys: [Poly<P>; WITNESS_POLYS],
    pub(crate) w_evals: [Evals<P>; WITNESS_POLYS],
    pub(crate) w_polys: [Poly<P>; WITNESS_POLYS],
    pub(crate) r_evals: [Evals<P>; WITNESS_POLYS],
    pub(crate) r_polys: [Poly<P>; WITNESS_POLYS],
}

impl<P: PastaConfig> Trace<P> {
    pub fn new(
        copy_constraints: Vec<Vec<SlotId>>,
        public_inputs: Vec<Scalar<P>>,
        ws: [Vec<Scalar<P>>; WITNESS_POLYS],
        rs: [Vec<Scalar<P>>; WITNESS_POLYS],
        qs: [Vec<Scalar<P>>; SELECTOR_POLYS],
        outputs: Vec<P::ScalarField>,
        n: usize,
    ) -> Self {
        let now = Instant::now();

        let d = n - 1;
        let domain = Domain::<P>::new(n).unwrap();
        let omega = domain.element(1);

        let (sigma, id_evals, sigma_evals) = build_sigma(copy_constraints, domain);

        let mut public_inputs_clone = public_inputs.clone();
        public_inputs_clone.resize(n, Scalar::<P>::zero());
        let public_inputs_evals = Evals::<P>::from_vec_and_domain(public_inputs_clone, domain);

        let w_evals = ws.map(|vec| Evals::<P>::from_vec_and_domain(vec, domain));
        let r_evals = rs.map(|vec| Evals::<P>::from_vec_and_domain(vec, domain));
        let q_evals = qs.map(|vec| Evals::<P>::from_vec_and_domain(vec, domain));

        let id_polys: [Poly<P>; WITNESS_POLYS] =
            array::from_fn(|i| id_evals[i].interpolate_by_ref());
        let sigma_polys: [Poly<P>; WITNESS_POLYS] =
            array::from_fn(|i| sigma_evals[i].interpolate_by_ref());
        let w_polys: [Poly<P>; WITNESS_POLYS] = array::from_fn(|i| w_evals[i].interpolate_by_ref());
        let r_polys: [Poly<P>; WITNESS_POLYS] = array::from_fn(|i| r_evals[i].interpolate_by_ref());
        let q_polys: [Poly<P>; SELECTOR_POLYS] =
            array::from_fn(|i| q_evals[i].interpolate_by_ref());
        let public_inputs_poly = public_inputs_evals.interpolate_by_ref();
        let C_qs = array::from_fn(|i| pcdl::commit(&q_polys[i], d, None));

        debug!("trace_time: {:?}", now.elapsed().as_secs_f32());

        Self {
            rows: n,
            domain,
            omega,
            sigma,
            public_inputs,
            public_inputs_poly,
            public_inputs_evals,
            C_qs,
            sigma_polys,
            id_polys,
            w_polys,
            r_polys,
            q_polys,
            sigma_evals,
            id_evals,
            w_evals,
            r_evals,
            q_evals,
            outputs,
        }
    }

    pub fn consume(self) {}

    pub fn s(self) -> Result<()> {
        Ok(())
    }

    pub fn test_copy_constraints(&self) {
        for i in 0..self.sigma.len() {
            let id = SlotId::from_usize(i, self.rows);
            let sigma = self.sigma[i];
            let v1 =
                self.w_polys[id.column_0_indexed()].evaluate(&self.omega.pow([id.row() as u64]));
            let v2 = self.w_polys[sigma.column_0_indexed()]
                .evaluate(&self.omega.pow([sigma.row() as u64]));
            assert_eq!(v1, v2, "{:?} != {:?}", id, sigma)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        circuit::{CircuitSpec, GateType, SlotId, TraceBuilder, build_pi},
        plonk::PlonkProof,
        utils::WITNESS_POLYS,
    };
    use anyhow::Result;
    use halo_group::{
        PallasConfig, PallasScalar, PastaConfig,
        ark_ff::Field,
        ark_poly::Polynomial,
        ark_std::{
            rand::{Rng, thread_rng},
            test_rng,
        },
    };
    use halo_poseidon::Sponge;
    use petgraph::algo::toposort;

    use super::Trace;

    fn scalar(n: u64) -> PallasScalar {
        PallasConfig::scalar_from_u64(n)
    }

    fn test_copy_constraints<P: PastaConfig>(trace: &Trace<P>) {
        for i in 0..trace.sigma.len() {
            let id = SlotId::from_usize(i, trace.rows);
            let sigma = trace.sigma[i];
            let v1 =
                trace.w_polys[id.column_0_indexed()].evaluate(&trace.omega.pow([id.row() as u64]));
            let v2 = trace.w_polys[sigma.column_0_indexed()]
                .evaluate(&trace.omega.pow([sigma.row() as u64]));
            assert_eq!(v1, v2, "{:?} != {:?}", id, sigma)
        }
    }

    #[test]
    fn test_circuit_eval1() -> Result<()> {
        // Create circuit: (x1 + x2) * x3
        let mut circuit = CircuitSpec::<PallasConfig>::new();
        let x7 = circuit.public_input_gate();
        let x2 = circuit.witness_gate();
        let x3 = circuit.witness_gate();
        let x5 = circuit.witness_gate();
        let c11 = circuit.constant_gate(scalar(11));
        let a5 = circuit.add_gate(x2, x3);
        let mul25 = circuit.mul_gate(a5, x5);
        let mul175 = circuit.mul_gate(x7, mul25);
        let add186 = circuit.add_gate(c11, mul175);
        circuit.output_gate(add186);

        // Evaluate with inputs x1=2.0, x2=3.0, x3=4.0
        let mut trace_builder = TraceBuilder::new(circuit);
        trace_builder.witness(x2, scalar(2))?;
        trace_builder.witness(x3, scalar(3))?;
        trace_builder.witness(x5, scalar(5))?;
        trace_builder.public_input(x7, scalar(7))?;
        let trace = trace_builder.trace()?;

        println!("{:?}", trace);

        trace.test_copy_constraints();

        assert_eq!(vec![scalar(186)], trace.outputs);

        let rng = &mut thread_rng();
        PlonkProof::prove(rng, trace.clone()).verify(trace)?;

        Ok(())
    }

    #[test]
    fn test_circuit_eval2() -> Result<()> {
        // Create circuit: (x1 + x2) * x3
        let mut circuit = CircuitSpec::<PallasConfig>::new();
        let x2 = circuit.witness_gate();
        let x7 = circuit.witness_gate();
        let c3 = circuit.constant_gate(scalar(3));
        let c5 = circuit.constant_gate(scalar(5));
        let c47 = circuit.constant_gate(scalar(47));
        let mul4 = circuit.mul_gate(x2, x2);
        let mul35 = circuit.mul_gate(x7, c5);
        let mul12 = circuit.mul_gate(c3, mul4);
        let add47 = circuit.add_gate(mul12, mul35);
        circuit.assert_eq_gate(add47, c47);
        circuit.output_gate(add47);

        println!("{:?}", circuit);

        // Evaluate with inputs x1=2.0, x2=3.0, x3=4.0
        let mut trace_builder = TraceBuilder::new(circuit);
        trace_builder.witness(x2, scalar(2))?;
        trace_builder.witness(x7, scalar(7))?;
        let trace = trace_builder.trace()?;

        println!("{:?}", trace);

        trace.test_copy_constraints();

        assert_eq!(vec![scalar(47)], trace.outputs);

        let rng = &mut thread_rng();
        PlonkProof::prove(rng, trace.clone()).verify(trace)?;
        Ok(())
    }

    #[test]
    fn test_poseidon() -> Result<()> {
        let rng = &mut test_rng();
        // Create circuit: (x1 + x2) * x3
        let mut circuit = CircuitSpec::<PallasConfig>::new();
        let x1 = circuit.witness_gate();
        let x2 = circuit.witness_gate();
        let x3 = circuit.witness_gate();
        let [p0, p1, p2] = circuit.poseidon_gate(0, [x1, x2, x3]);
        let [p3, p4, p5] = circuit.poseidon_gate(1, [p0, p1, p2]);
        let [p6, p7, p8] = circuit.poseidon_gate_finish([p3, p4, p5]);

        let xa11 = circuit.witness_gate();
        let xa12 = circuit.witness_gate();
        let a1 = circuit.add_gate(xa11, xa12);
        let xa21 = circuit.witness_gate();
        let a2 = circuit.add_gate(a1, xa21);
        let xa31 = circuit.witness_gate();
        let a3 = circuit.add_gate(a2, xa31);

        let m1 = circuit.mul_gate(p0, p6);
        let m2 = circuit.mul_gate(m1, p7);
        let m3 = circuit.mul_gate(m2, p8);
        let m4 = circuit.mul_gate(m3, a3);
        circuit.output_gate(m4);

        let topo_order = toposort(&circuit.graph, None).unwrap();

        let mut seen = false;
        for node_idx in topo_order {
            match circuit.graph[node_idx] {
                GateType::Poseidon(..) => {
                    if !seen {
                        seen = true;
                    } else {
                        seen = false;
                    }
                }
                _ => assert!(!seen),
            }
        }

        println!("{:?}", circuit);

        let mut trace_builder = TraceBuilder::new(circuit);
        trace_builder.witness(x1, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(x2, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(x3, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(xa11, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(xa12, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(xa21, scalar(rng.gen_range(1..10)))?;
        trace_builder.witness(xa31, scalar(rng.gen_range(1..10)))?;

        let trace = trace_builder.trace()?;
        println!("{trace:?}");
        PlonkProof::naive_prover(rng, trace.clone()).verify(trace)?;

        Ok(())
    }
}
