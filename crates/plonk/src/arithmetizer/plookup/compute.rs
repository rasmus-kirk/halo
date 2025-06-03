use super::TableRegistry;
use crate::{
    arithmetizer::trace::Constraints,
    scheme::{eqns::EqnsF, Selectors, Slots, Terms},
    utils::{Evals, Poly, Scalar},
    Coset,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use educe::Educe;

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for plookup
#[derive(Educe)]
#[educe(Default, Debug, PartialEq, Eq, Clone)]
pub struct PlookupEvsThunk<P: SWCurveConfig> {
    constraints: Vec<Constraints<P>>,
    table: TableRegistry<P>,
}

impl<P: SWCurveConfig> PlookupEvsThunk<P> {
    pub fn new(constraints: Vec<Constraints<P>>, table: TableRegistry<P>) -> Self {
        Self { constraints, table }
    }

    fn compute_t_evs(&self, zeta: Scalar<P>, h: &Coset<P>) -> Evals<P> {
        let mut t = self
            .table
            .tables
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (_j, table)| {
                acc.extend(table.compress(zeta, Scalar::<P>::from(_j as u64)));
                acc
            });
        t.sort();
        t.extend(vec![*t.last().unwrap(); h.n() as usize - t.len()]);
        Evals::<P>::new_sr(t)
    }

    fn compute_f_evs(
        &self,
        zeta: Scalar<P>,
        h: &Coset<P>,
        constraints: &[Constraints<P>],
        default: Scalar<P>,
    ) -> Evals<P> {
        Evals::<P>::new_sr(
            constraints
                .iter()
                .map(|constraint| {
                    if constraint[Terms::Q(Selectors::Qk)].to_fp() == Scalar::<P>::ONE {
                        let a = constraint[Terms::F(Slots::A)].to_fp();
                        let b = constraint[Terms::F(Slots::B)].to_fp();
                        let c = constraint[Terms::F(Slots::C)].to_fp();
                        let j = constraint[Terms::Q(Selectors::J)].to_fp();
                        EqnsF::<P>::plookup_compress(zeta, a, b, c, j)
                    } else {
                        default
                    }
                })
                .chain(vec![default; h.n() as usize - constraints.len()])
                .collect(),
        )
    }

    fn split_sort(s: Vec<Scalar<P>>) -> [Evals<P>; 2] {
        s.into_iter()
            .enumerate()
            .fold([vec![], vec![]], |mut hs, (i, x)| {
                hs[i % 2].push(x);
                hs
            })
            .into_iter()
            .map(|evals| Evals::<P>::new_sr(evals))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub fn compute(&self, h: &Coset<P>, zeta: Scalar<P>) -> PlookupEvals<P> {
        let t = self.compute_t_evs(zeta, h);
        let default = *t.last();
        let f = self.compute_f_evs(zeta, h, &self.constraints, default);
        let mut s: Vec<Scalar<P>> = Vec::new();
        s.extend(t.clone().0.iter());
        s.extend(f.clone().0.iter());
        s.sort();
        let [h1, h2] = Self::split_sort(s);

        PlookupEvals::new(t.fft_s(), f.fft_s(), h1.fft_s(), h2.fft_s())
    }
}

pub struct PlookupEvals<P: SWCurveConfig> {
    pub t: Poly<P>,
    pub f: Poly<P>,
    pub h1: Poly<P>,
    pub h2: Poly<P>,
    pub h1_bar: Poly<P>,
    pub t_bar: Poly<P>,
}

impl<P: SWCurveConfig> PlookupEvals<P> {
    pub fn new(t: Poly<P>, f: Poly<P>, h1: Poly<P>, h2: Poly<P>) -> Self {
        let t_bar = t.e.clone().shift_left().fft();
        let h1_bar = h1.e.clone().shift_left().fft();
        PlookupEvals {
            t,
            f,
            h1,
            h2,
            h1_bar,
            t_bar,
        }
    }

    pub fn base_polys(&self) -> impl Iterator<Item = &Poly<P>> {
        [&self.t, &self.f, &self.h1, &self.h2].into_iter()
    }
}
