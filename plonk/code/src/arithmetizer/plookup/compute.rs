use super::TableRegistry;
use crate::{
    arithmetizer::trace::Constraints,
    scheme::{eqns::plookup_compress_fp, Selectors, Slots, Terms},
    utils::{misc::batch_op, poly::shift_wrap_eval, Evals, Poly},
    Coset,
};

use ark_ff::{AdditiveGroup, Field, Fp, FpConfig};

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for plookup
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlookupEvsThunk<const N: usize, C: FpConfig<N>> {
    constraints: Vec<Constraints<N, C>>,
    table: TableRegistry<N, C>,
}

impl<const N: usize, C: FpConfig<N>> PlookupEvsThunk<N, C> {
    pub fn new(constraints: Vec<Constraints<N, C>>, table: TableRegistry<N, C>) -> Self {
        Self { constraints, table }
    }

    fn compute_t_evs(&self, zeta: Fp<C, N>, h: &Coset<N, C>) -> Evals<N, C> {
        let mut t =
            self.table
                .tables
                .iter()
                .enumerate()
                .fold(vec![Fp::ZERO], |mut acc, (_j, table)| {
                    acc.extend(table.compress(zeta, Fp::from(_j as u64)));
                    acc
                });
        t.sort();
        let default = t.last().unwrap();
        let extend = h.n() as usize - t.len();
        t.extend(vec![*default; extend]);
        Evals::from_vec_and_domain(t, h.domain)
    }

    fn compute_f_evs(
        &self,
        zeta: Fp<C, N>,
        h: &Coset<N, C>,
        constraints: &[Constraints<N, C>],
        default: Fp<C, N>,
    ) -> Evals<N, C> {
        let mut f = vec![Fp::ZERO];
        f.extend(constraints.iter().map(|constraint| {
            if Into::<Fp<C, N>>::into(constraint[Terms::Q(Selectors::Qk)]) == Fp::ONE {
                let a: Fp<C, N> = constraint[Terms::F(Slots::A)].into();
                let b = constraint[Terms::F(Slots::B)].into();
                let c = constraint[Terms::F(Slots::C)].into();
                let j = constraint[Terms::Q(Selectors::J)].into();
                plookup_compress_fp(zeta, a, b, c, j)
            } else {
                default
            }
        }));
        let extend = h.n() as usize - f.len();
        f.extend(vec![default; extend]);
        Evals::from_vec_and_domain(f, h.domain)
    }

    fn split_sort(h: &Coset<N, C>, s: Vec<Fp<C, N>>) -> Vec<Evals<N, C>> {
        s.into_iter()
            .enumerate()
            .fold([vec![], vec![]], |mut hs, (i, x)| {
                hs[i % 2].push(x);
                hs
            })
            .into_iter()
            .map(|evals| Evals::from_vec_and_domain(evals, h.domain))
            .collect()
    }

    pub fn compute(&self, h: &Coset<N, C>, zeta: Fp<C, N>) -> PlookupPolys<N, C> {
        let mut evals = vec![];
        evals.push(self.compute_t_evs(zeta, h));
        let default = *evals[0].evals.last().unwrap();
        evals.push(self.compute_f_evs(zeta, h, &self.constraints, default));
        let mut s: Vec<Fp<C, N>> = Vec::new();
        s.extend(evals[0].evals.iter());
        s.extend(evals[1].evals.iter());
        s.sort();
        evals.extend(Self::split_sort(h, s));

        PlookupPolys::new(h, evals)
    }
}

pub struct PlookupPolys<const N: usize, C: FpConfig<N>> {
    pub t: Poly<N, C>,
    pub _t: Evals<N, C>,
    pub f: Poly<N, C>,
    pub _f: Evals<N, C>,
    pub h1: Poly<N, C>,
    pub _h1: Evals<N, C>,
    pub h2: Poly<N, C>,
    pub _h2: Evals<N, C>,
    pub h1_bar: Poly<N, C>,
    pub _h1_bar: Evals<N, C>,
    pub t_bar: Poly<N, C>,
    pub _t_bar: Evals<N, C>,
}

impl<const N: usize, C: FpConfig<N>> PlookupPolys<N, C> {
    pub fn new(h: &Coset<N, C>, evals: Vec<Evals<N, C>>) -> Self {
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

    pub fn base_polys(&self) -> Vec<&Poly<N, C>> {
        vec![&self.t, &self.f, &self.h1, &self.h2]
    }
}
