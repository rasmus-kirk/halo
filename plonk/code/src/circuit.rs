use crate::{
    scheme::{Selectors, Terms},
    utils::{misc::EnumIter, print_table::evals_str, Evals, Point, Poly},
    Coset,
};

use super::{arithmetizer::PlookupEvsThunk, scheme::Slots};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Fp, FpConfig};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic<const N: usize, C: FpConfig<N>, P: SWCurveConfig> {
    pub d: usize,
    // coset scheme
    pub h: Coset<N, C>,
    // selector polynomials
    pub qs: Vec<Poly<N, C>>,
    // public input polynomial
    pub pip: Poly<N, C>,
    // identity permutation polynomial
    pub is: Vec<Poly<N, C>>,
    pub _is: Vec<Evals<N, C>>,
    // permutation polynomial
    pub ps: Vec<Poly<N, C>>,
    pub _ps: Vec<Evals<N, C>>,

    pub pip_com: Point<P>,
    pub qs_com: Vec<Point<P>>,
    pub ps_com: Vec<Point<P>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate<const N: usize, C: FpConfig<N>> {
    // slot polynomials
    pub ws: Vec<Poly<N, C>>,
    pub _ws: Vec<Evals<N, C>>,
    // thunk to compute Plonkup polys
    pub plookup: PlookupEvsThunk<N, C>,
}

pub type Circuit<const N: usize, C: FpConfig<N>, P: SWCurveConfig> =
    (CircuitPublic<N, C, P>, CircuitPrivate<N, C>);

pub fn poly_evaluations_to_string<const N: usize, C: FpConfig<N>, P: SWCurveConfig>(
    x: &CircuitPublic<N, C, P>,
    w: &CircuitPrivate<N, C>,
) -> String {
    let mut result = String::from("Circuit {\n");
    let polys =
        w.ws.iter()
            .chain(x.qs.iter())
            .chain(std::iter::once(&x.pip))
            .chain(x.ps.iter())
            .collect();
    for line in evals_str(
        &x.h,
        polys,
        Terms::iter()
            .map(|t| t.to_string())
            .chain(Slots::iter().map(|slot| slot.perm_string().to_string()))
            .collect::<Vec<String>>(),
        [false; Terms::COUNT]
            .iter()
            .chain([true; Slots::COUNT].iter())
            .cloned()
            .collect(),
    )
    .lines()
    {
        result.push_str(&format!("    {}\n", line));
    }
    result.push('}');
    result
}

impl<const N: usize, C: FpConfig<N>> CircuitPrivate<N, C> {
    // Slot Getters ---------------------------------------------

    pub fn a(&self) -> &Poly<N, C> {
        &self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> &Poly<N, C> {
        &self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> &Poly<N, C> {
        &self.ws[Slots::C.id()]
    }

    pub fn _a(&self, i: usize) -> Fp<C, N> {
        self._ws[Slots::A.id()].evals[i]
    }

    pub fn _b(&self, i: usize) -> Fp<C, N> {
        self._ws[Slots::B.id()].evals[i]
    }

    pub fn _c(&self, i: usize) -> Fp<C, N> {
        self._ws[Slots::C.id()].evals[i]
    }
}

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> CircuitPublic<N, C, P> {
    // Selector Getters ---------------------------------------------

    pub fn ql(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> &Poly<N, C> {
        &self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> &Poly<N, C> {
        &self.qs[Selectors::J.id()]
    }

    // Identity Permutation Getters ---------------------------------------------

    pub fn ia(&self) -> &Poly<N, C> {
        &self.is[Slots::A.id()]
    }

    pub fn ib(&self) -> &Poly<N, C> {
        &self.is[Slots::B.id()]
    }

    pub fn ic(&self) -> &Poly<N, C> {
        &self.is[Slots::C.id()]
    }

    pub fn _ia(&self, i: usize) -> Fp<C, N> {
        self._is[Slots::A.id()].evals[i]
    }

    pub fn _ib(&self, i: usize) -> Fp<C, N> {
        self._is[Slots::B.id()].evals[i]
    }

    pub fn _ic(&self, i: usize) -> Fp<C, N> {
        self._is[Slots::C.id()].evals[i]
    }

    // Permutation Getters ---------------------------------------------

    pub fn pa(&self) -> &Poly<N, C> {
        &self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> &Poly<N, C> {
        &self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> &Poly<N, C> {
        &self.ps[Slots::C.id()]
    }

    pub fn _pa(&self, i: usize) -> Fp<C, N> {
        self._ps[Slots::A.id()].evals[i]
    }

    pub fn _pb(&self, i: usize) -> Fp<C, N> {
        self._ps[Slots::B.id()].evals[i]
    }

    pub fn _pc(&self, i: usize) -> Fp<C, N> {
        self._ps[Slots::C.id()].evals[i]
    }
}
