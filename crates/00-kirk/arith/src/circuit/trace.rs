#![allow(non_snake_case)]

use std::{array, collections::HashMap};

use anyhow::Result;
use halo_accumulation::pcdl;
use halo_group::{
    Evals, PastaConfig, Point, Poly,
    ark_poly::{EvaluationDomain, Radix2EvaluationDomain},
};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::{
    circuit::SlotId,
    utils::{IteratorSplitExt, SELECTOR_POLYS, WITNESS_POLYS},
};

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

pub struct Trace<P: PastaConfig> {
    pub(crate) rows: usize,
    pub(crate) sigma: Vec<SlotId>,
    pub(crate) omega: P::ScalarField,
    pub(crate) domain: Radix2EvaluationDomain<P::ScalarField>,
    pub(crate) id_polys: [Poly<P>; WITNESS_POLYS],
    pub(crate) sigma_polys: [Poly<P>; WITNESS_POLYS],
    pub(crate) ws: [Poly<P>; WITNESS_POLYS],
    pub(crate) qs: [Poly<P>; SELECTOR_POLYS],
    pub(crate) C_qs: [Point<P>; SELECTOR_POLYS],
    pub(crate) output: P::ScalarField,
}

impl<P: PastaConfig> Trace<P> {
    pub fn new(
        copy_constraints: Vec<(SlotId, SlotId)>,
        mut ws: [Evals<P>; WITNESS_POLYS],
        mut qs: [Evals<P>; SELECTOR_POLYS],
        output: P::ScalarField,
        n: usize,
    ) -> Self {
        let d = n - 1;

        let mut uf = QuickUnionUf::<UnionBySize>::new(WITNESS_POLYS * n);

        for (x, y) in copy_constraints.iter() {
            uf.union(x.to_usize(n), y.to_usize(n));
        }
        let sigma = build_pi(uf);
        let mut sigmas: [_; WITNESS_POLYS] = sigma.iter().map(|x| x.to_sigma::<P>(n)).split_array();
        let mut ids: [_; WITNESS_POLYS] = (0..n * WITNESS_POLYS)
            .map(|i| SlotId::from_usize(i, n).to_sigma::<P>(n))
            .split_array();

        let id_polys: [Poly<P>; WITNESS_POLYS] = array::from_fn(|i| {
            let eval = Evals::<P>::new(std::mem::take(&mut ids[i]), n);
            eval.fft()
        });
        let sigma_polys: [Poly<P>; WITNESS_POLYS] = array::from_fn(|i| {
            let eval = Evals::<P>::new(std::mem::take(&mut sigmas[i]), n);
            eval.fft()
        });
        let w_polys: [Poly<P>; WITNESS_POLYS] = array::from_fn(|i| {
            let eval = std::mem::replace(&mut ws[i], Evals::new(vec![], n));
            eval.fft()
        });
        let q_polys: [Poly<P>; SELECTOR_POLYS] = array::from_fn(|i| {
            let eval = std::mem::replace(&mut qs[i], Evals::new(vec![], n));
            eval.fft()
        });
        let C_qs = array::from_fn(|i| pcdl::commit(&q_polys[i], d, None));
        let domain = Radix2EvaluationDomain::<P::ScalarField>::new(n).unwrap();
        let omega = domain.element(1);

        Self {
            rows: n,
            domain,
            omega,
            sigma,
            sigma_polys,
            id_polys,
            C_qs,
            ws: w_polys,
            qs: q_polys,
            output,
        }
    }

    pub fn consume(self) {}

    pub fn output(&self) -> P::ScalarField {
        self.output
    }

    pub fn s(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::{CircuitSpec, SlotId, TraceBuilder, build_pi};
    use anyhow::Result;
    use halo_group::{
        PallasConfig, PallasScalar, PastaConfig, ark_ff::Field, ark_poly::Polynomial,
    };
    use union_find::{QuickUnionUf, UnionBySize, UnionFind};

    use super::Trace;

    fn scalar(n: u64) -> PallasScalar {
        PallasConfig::scalar_from_u64(n)
    }

    fn test_copy_constraints<P: PastaConfig>(trace: &Trace<P>) {
        for i in 0..trace.sigma.len() {
            let id = SlotId::from_usize(i, trace.rows);
            let sigma = trace.sigma[i];
            let v1 = trace.ws[id.column() - 1].evaluate(&trace.omega.pow([id.row() as u64]));
            let v2 = trace.ws[sigma.column() - 1].evaluate(&trace.omega.pow([sigma.row() as u64]));
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

        test_copy_constraints(&trace);

        assert_eq!(scalar(186), trace.output());
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

        // Evaluate with inputs x1=2.0, x2=3.0, x3=4.0
        let mut trace_builder = TraceBuilder::new(circuit);
        trace_builder.witness(x2, scalar(2))?;
        trace_builder.witness(x7, scalar(7))?;
        let trace = trace_builder.trace()?;

        println!("{:?}", trace);

        test_copy_constraints(&trace);

        assert_eq!(scalar(47), trace.output());
        Ok(())
    }

    #[test]
    fn test_pi() {
        // We have 6 wires: a1,b1,c1,a2,b2,c2 mapped to indices 0..5
        // Copy constraints:
        let mut uf = QuickUnionUf::<UnionBySize>::new(6);
        uf.union(3, 4); // a2 = b2
        uf.union(2, 3); // c1 = a2
        uf.union(2, 4); // c1 = b2

        let pi = build_pi(uf);

        // For singleton sets, pi[i] = i
        // For the equivalence class {2,3,4}, we expect a cycle: 2->3, 3->4, 4->2
        let expected: Vec<_> = vec![0, 1, 3, 4, 2, 5]
            .iter()
            .map(|i| SlotId::from_usize(*i, 2))
            .collect();
        assert_eq!(expected, pi);
    }
}
