use super::{PlookupOps, TableRegistry};
use crate::{
    curve::Coset,
    protocol::{
        arithmetizer::trace::Constraints,
        scheme::{Selectors, Slots, Terms},
    },
    util::poly::plookup_compress,
};

use halo_accumulation::group::PallasScalar;

use ark_ff::{AdditiveGroup, Field};
use ark_poly::Evaluations;

type Scalar = PallasScalar;

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for Plonkup protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlookupEvsThunk {
    h: Coset,
    constraints: Vec<Constraints>,
    table: TableRegistry,
}

impl PlookupEvsThunk {
    pub fn new(h: Coset, constraints: Vec<Constraints>, table: TableRegistry) -> Self {
        Self {
            h,
            constraints,
            table,
        }
    }

    fn compute_t_evs(&self, zeta: &Scalar, h: &Coset) -> Evaluations<Scalar> {
        let mut t = vec![Scalar::ZERO];
        for op in PlookupOps::iter() {
            let j = &op.into();
            t.extend(self.table.tables[op as usize].compress(zeta, j));
        }
        t.sort();
        let default = t.last().unwrap();
        let extend = h.n() as usize - t.len();
        t.extend(vec![*default; extend]);
        Evaluations::from_vec_and_domain(t, h.domain)
    }

    fn compute_f_evs(
        &self,
        zeta: &Scalar,
        h: &Coset,
        constraints: &[Constraints],
        default: &Scalar,
    ) -> Evaluations<Scalar> {
        let mut f = vec![Scalar::ZERO];
        for constraint in constraints.iter() {
            if Into::<Scalar>::into(constraint[Terms::Q(Selectors::Qk)]) == Scalar::ONE {
                let a: Scalar = constraint[Terms::F(Slots::A)].into();
                let b: Scalar = constraint[Terms::F(Slots::B)].into();
                let c: Scalar = constraint[Terms::F(Slots::C)].into();
                let j: Scalar = constraint[Terms::Q(Selectors::J)].into();
                f.push(plookup_compress(zeta, &a, &b, &c, &j));
            } else {
                f.push(*default);
            }
        }
        let extend = h.n() as usize - f.len();
        f.extend(vec![*default; extend]);
        Evaluations::from_vec_and_domain(f, h.domain)
    }

    fn split_sort(h: &Coset, s: Vec<Scalar>) -> Vec<Evaluations<Scalar>> {
        let mut h1 = Vec::new();
        let mut h2 = Vec::new();
        for (i, x) in s.into_iter().enumerate() {
            if i % 2 == 0 {
                h1.push(x);
            } else {
                h2.push(x);
            }
        }
        [h1, h2]
            .into_iter()
            .map(|points| Evaluations::from_vec_and_domain(points, h.domain))
            .collect()
    }

    pub fn compute(&self, zeta: &Scalar) -> Vec<Evaluations<Scalar>> {
        let mut evals = vec![];
        evals.push(self.compute_t_evs(zeta, &self.h));
        evals.push(self.compute_f_evs(
            zeta,
            &self.h,
            &self.constraints,
            evals[0].evals.last().unwrap(),
        ));
        let mut s: Vec<Scalar> = Vec::new();
        s.extend(evals[0].evals.iter());
        s.extend(evals[1].evals.iter());
        s.sort();
        evals.extend(Self::split_sort(&self.h, s).into_iter());

        evals
    }
}
