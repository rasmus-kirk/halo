use super::TableRegistry;
use crate::{
    arithmetizer::trace::Constraints,
    scheme::{eqns::EqnsF, Selectors, Slots, Terms},
    utils::{misc::batch_op, poly::shift_wrap_eval, Evals, Poly, Scalar},
    Coset,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use educe::Educe;

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for plookup
#[derive(Educe)]
#[educe(Debug, PartialEq, Eq, Clone)]
pub struct PlookupEvsThunk<P: SWCurveConfig> {
    constraints: Vec<Constraints<P>>,
    table: TableRegistry<P>,
}

impl<P: SWCurveConfig> PlookupEvsThunk<P> {
    pub fn new(constraints: Vec<Constraints<P>>, table: TableRegistry<P>) -> Self {
        Self { constraints, table }
    }

    fn compute_t_evs(&self, zeta: Scalar<P>, h: &Coset<P>) -> Evals<P> {
        let mut t = self.table.tables.iter().enumerate().fold(
            vec![Scalar::<P>::ZERO],
            |mut acc, (_j, table)| {
                acc.extend(table.compress(zeta, Scalar::<P>::from(_j as u64)));
                acc
            },
        );
        t.sort();
        let default = t.last().unwrap();
        let extend = h.n() as usize - t.len();
        t.extend(vec![*default; extend]);
        Evals::<P>::new(t, h.domain)
    }

    fn compute_f_evs(
        &self,
        zeta: Scalar<P>,
        h: &Coset<P>,
        constraints: &[Constraints<P>],
        default: Scalar<P>,
    ) -> Evals<P> {
        let mut f = vec![Scalar::<P>::ZERO];
        f.extend(constraints.iter().map(|constraint| {
            if constraint[Terms::Q(Selectors::Qk)].to_fp() == Scalar::<P>::ONE {
                let a = constraint[Terms::F(Slots::A)].to_fp();
                let b = constraint[Terms::F(Slots::B)].to_fp();
                let c = constraint[Terms::F(Slots::C)].to_fp();
                let j = constraint[Terms::Q(Selectors::J)].to_fp();
                EqnsF::<P>::plookup_compress(zeta, a, b, c, j)
            } else {
                default
            }
        }));
        let extend = h.n() as usize - f.len();
        f.extend(vec![default; extend]);
        Evals::<P>::new(f, h.domain)
    }

    fn split_sort(h: &Coset<P>, s: Vec<Scalar<P>>) -> Vec<Evals<P>> {
        s.into_iter()
            .enumerate()
            .fold([vec![], vec![]], |mut hs, (i, x)| {
                hs[i % 2].push(x);
                hs
            })
            .into_iter()
            .map(|evals| Evals::<P>::new(evals, h.domain))
            .collect()
    }

    pub fn compute(&self, h: &Coset<P>, zeta: Scalar<P>) -> PlookupPolys<P> {
        let mut evals = vec![];
        evals.push(self.compute_t_evs(zeta, h));
        let default = *evals[0].last();
        evals.push(self.compute_f_evs(zeta, h, &self.constraints, default));
        let mut s: Vec<Scalar<P>> = Vec::new();
        s.extend(evals[0].clone().vec().iter());
        s.extend(evals[1].clone().vec().iter());
        s.sort();
        evals.extend(Self::split_sort(h, s));

        PlookupPolys::new(h, evals)
    }
}

pub struct PlookupPolys<P: SWCurveConfig> {
    pub t: Poly<P>,
    pub _t: Evals<P>,
    pub f: Poly<P>,
    pub _f: Evals<P>,
    pub h1: Poly<P>,
    pub _h1: Evals<P>,
    pub h2: Poly<P>,
    pub _h2: Evals<P>,
    pub h1_bar: Poly<P>,
    pub _h1_bar: Evals<P>,
    pub t_bar: Poly<P>,
    pub _t_bar: Evals<P>,
}

impl<P: SWCurveConfig> PlookupPolys<P> {
    pub fn new(h: &Coset<P>, evals: Vec<Evals<P>>) -> Self {
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

    pub fn base_polys(&self) -> Vec<&Poly<P>> {
        vec![&self.t, &self.f, &self.h1, &self.h2]
    }
}
