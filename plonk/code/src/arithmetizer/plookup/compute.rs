use super::{PlookupOps, TableRegistry};
use crate::{
    arithmetizer::trace::Constraints,
    scheme::{Selectors, Slots, Terms},
    utils::{
        misc::batch_op,
        poly::{plookup_compress, shift_wrap_eval},
    },
    Coset,
};

use halo_accumulation::group::{PallasPoly, PallasScalar};

use ark_ff::{AdditiveGroup, Field};
use ark_poly::Evaluations;

type Scalar = PallasScalar;
type Poly = PallasPoly;
type Evals = Evaluations<Scalar>;

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for plookup
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

    fn compute_t_evs(&self, zeta: &Scalar, h: &Coset) -> Evals {
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
    ) -> Evals {
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

    fn split_sort(h: &Coset, s: Vec<Scalar>) -> Vec<Evals> {
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

    pub fn compute(&self, h: &Coset, zeta: &Scalar) -> PlookupPolys {
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
        evals.extend(Self::split_sort(&self.h, s));

        PlookupPolys::new(h, evals)
    }
}

pub struct PlookupPolys {
    pub t: Poly,
    pub _t: Evals,
    pub f: Poly,
    pub _f: Evals,
    pub h1: Poly,
    pub _h1: Evals,
    pub h2: Poly,
    pub _h2: Evals,
    pub h1_bar: Poly,
    pub _h1_bar: Evals,
    pub t_bar: Poly,
    pub _t_bar: Evals,
}

impl PlookupPolys {
    pub fn new(h: &Coset, evals: Vec<Evals>) -> Self {
        let _t = evals[0].clone();
        let _f = evals[1].clone();
        let _h1 = evals[2].clone();
        let _h2 = evals[3].clone();
        let _t_bar = shift_wrap_eval(h, _t.clone());
        let _h1_bar = shift_wrap_eval(h, _h1.clone());
        let mut plp = batch_op(evals, |eval| eval.interpolate());
        let h2 = plp.pop().unwrap();
        let h1 = plp.pop().unwrap();
        let f = plp.pop().unwrap();
        let t = plp.pop().unwrap();
        let t_bar = _t_bar.clone().interpolate();
        let h1_bar = _h1_bar.clone().interpolate();
        PlookupPolys {
            t,
            _t,
            f,
            _f,
            h1,
            _h1,
            h2,
            _h2,
            h1_bar,
            _h1_bar,
            t_bar,
            _t_bar,
        }
    }

    pub fn base_polys(&self) -> Vec<&Poly> {
        vec![&self.t, &self.f, &self.h1, &self.h2]
    }
}
