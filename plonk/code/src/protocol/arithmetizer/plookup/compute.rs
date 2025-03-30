use crate::{
    curve::{Coset, Poly, Scalar},
    protocol::{
        arithmetizer::trace::Constraints,
        scheme::{Selectors, Slots, Terms},
    },
};

use super::{PlookupOps, Table, TableRegistry};

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

    fn compute_t_evs(&self, zeta: &Scalar, h: &Coset) -> Vec<Scalar> {
        let mut t = Vec::new();
        for op in PlookupOps::iter() {
            let j = &op.into();
            t.extend(self.table.tables[op as usize].compress(zeta, j));
        }
        t.sort();
        let extend = h.n() as usize - t.len() - 1;
        t.extend(vec![*t.last().unwrap(); extend]);
        t
    }

    fn compute_f_evs(
        &self,
        zeta: &Scalar,
        h: &Coset,
        constraints: &[Constraints],
        default: &Scalar,
    ) -> Vec<Scalar> {
        let mut f = Vec::new();
        for constraint in constraints.iter() {
            if Into::<Scalar>::into(constraint[Terms::Q(Selectors::Qk)]) == Scalar::ONE {
                let a: Scalar = constraint[Terms::F(Slots::A)].into();
                let b: Scalar = constraint[Terms::F(Slots::B)].into();
                let c: Scalar = constraint[Terms::F(Slots::C)].into();
                let j: Scalar = constraint[Terms::Q(Selectors::J)].into();
                f.push(Table::eval_compress(zeta, &a, &b, &c, &j));
            } else {
                f.push(*default);
            }
        }
        let extend = h.n() as usize - f.len() - 1;
        f.extend(vec![*default; extend]);
        f
    }

    fn split_sort(s: Vec<Scalar>) -> (Vec<Scalar>, Vec<Scalar>) {
        let mut h1 = Vec::new();
        let mut h2 = Vec::new();
        for (i, x) in s.into_iter().enumerate() {
            if i % 2 == 0 {
                h1.push(x);
            } else {
                h2.push(x);
            }
        }
        (h1, h2)
    }

    pub fn compute(&self, zeta: &Scalar) -> Vec<Poly> {
        let t = self.compute_t_evs(zeta, &self.h);
        let f = self.compute_f_evs(zeta, &self.h, &self.constraints, t.last().unwrap());

        let mut s: Vec<Scalar> = Vec::new();
        s.extend(t.iter());
        s.extend(f.iter());
        s.sort();
        let (h1, h2) = Self::split_sort(s);

        [t, f, h1, h2]
            .into_iter()
            .map(|evals| self.h.interpolate_zf(evals))
            .collect()
    }
}
