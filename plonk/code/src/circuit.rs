use ark_ec::short_weierstrass::SWCurveConfig;

use crate::{
    scheme::{Selectors, Terms},
    utils::{misc::EnumIter, print_table::evals_str, Evals, Point, Poly, Scalar},
    Coset,
};

use super::{arithmetizer::PlookupEvsThunk, scheme::Slots};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic<P: SWCurveConfig> {
    pub d: usize,
    // coset scheme
    pub h: Coset<P>,
    // selector polynomials
    pub qs: Vec<Poly<P>>,
    // public input polynomial
    pub pip: Poly<P>,
    // identity permutation polynomial
    pub is: Vec<Poly<P>>,
    pub _is: Vec<Evals<P>>,
    // permutation polynomial
    pub ps: Vec<Poly<P>>,
    pub _ps: Vec<Evals<P>>,

    pub pip_com: Point<P>,
    pub qs_com: Vec<Point<P>>,
    pub ps_com: Vec<Point<P>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate<P: SWCurveConfig> {
    // slot polynomials
    pub ws: Vec<Poly<P>>,
    pub _ws: Vec<Evals<P>>,
    // thunk to compute Plonkup polys
    pub plookup: PlookupEvsThunk<P>,
}

pub type Circuit<P: SWCurveConfig> = (CircuitPublic<P>, CircuitPrivate<P>);

pub fn poly_evaluations_to_string<P: SWCurveConfig>(
    x: &CircuitPublic<P>,
    w: &CircuitPrivate<P>,
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

impl<P: SWCurveConfig> CircuitPrivate<P> {
    // Slot Getters ---------------------------------------------

    pub fn a(&self) -> &Poly<P> {
        &self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> &Poly<P> {
        &self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> &Poly<P> {
        &self.ws[Slots::C.id()]
    }

    pub fn _a(&self, i: usize) -> Scalar<P> {
        self._ws[Slots::A.id()].evals[i]
    }

    pub fn _b(&self, i: usize) -> Scalar<P> {
        self._ws[Slots::B.id()].evals[i]
    }

    pub fn _c(&self, i: usize) -> Scalar<P> {
        self._ws[Slots::C.id()].evals[i]
    }
}

impl<P: SWCurveConfig> CircuitPublic<P> {
    // Selector Getters ---------------------------------------------

    pub fn ql(&self) -> &Poly<P> {
        &self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> &Poly<P> {
        &self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> &Poly<P> {
        &self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> &Poly<P> {
        &self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> &Poly<P> {
        &self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> &Poly<P> {
        &self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> &Poly<P> {
        &self.qs[Selectors::J.id()]
    }

    // Identity Permutation Getters ---------------------------------------------

    pub fn ia(&self) -> &Poly<P> {
        &self.is[Slots::A.id()]
    }

    pub fn ib(&self) -> &Poly<P> {
        &self.is[Slots::B.id()]
    }

    pub fn ic(&self) -> &Poly<P> {
        &self.is[Slots::C.id()]
    }

    pub fn _ia(&self, i: usize) -> Scalar<P> {
        self._is[Slots::A.id()].evals[i]
    }

    pub fn _ib(&self, i: usize) -> Scalar<P> {
        self._is[Slots::B.id()].evals[i]
    }

    pub fn _ic(&self, i: usize) -> Scalar<P> {
        self._is[Slots::C.id()].evals[i]
    }

    // Permutation Getters ---------------------------------------------

    pub fn pa(&self) -> &Poly<P> {
        &self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> &Poly<P> {
        &self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> &Poly<P> {
        &self.ps[Slots::C.id()]
    }

    pub fn _pa(&self, i: usize) -> Scalar<P> {
        self._ps[Slots::A.id()].evals[i]
    }

    pub fn _pb(&self, i: usize) -> Scalar<P> {
        self._ps[Slots::B.id()].evals[i]
    }

    pub fn _pc(&self, i: usize) -> Scalar<P> {
        self._ps[Slots::C.id()].evals[i]
    }
}
