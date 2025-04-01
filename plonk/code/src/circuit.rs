use crate::{
    scheme::{Selectors, Terms},
    utils::{misc::EnumIter, print_table::evals_str},
    Coset,
};

use super::{arithmetizer::PlookupEvsThunk, scheme::Slots};

use halo_accumulation::group::{PallasPoint, PallasPoly, PallasScalar};

use ark_poly::Evaluations;

type Scalar = PallasScalar;
type Poly = PallasPoly;
type Point = PallasPoint;
type Evals = Evaluations<Scalar>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic {
    pub d: usize,
    // coset scheme
    pub h: Coset,
    // selector polynomials
    pub qs: Vec<Poly>,
    // public input polynomial
    pub pip: Poly,
    // identity permutation polynomial
    pub is: Vec<Poly>,
    pub _is: Vec<Evals>,
    // permutation polynomial
    pub ps: Vec<Poly>,
    pub _ps: Vec<Evals>,

    pub pip_com: Point,
    pub qs_com: Vec<Point>,
    pub ps_com: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate {
    // slot polynomials
    pub ws: Vec<Poly>,
    pub _ws: Vec<Evals>,
    // thunk to compute Plonkup polys
    pub plookup: PlookupEvsThunk,
}

pub type Circuit = (CircuitPublic, CircuitPrivate);

pub fn poly_evaluations_to_string(x: &CircuitPublic, w: &CircuitPrivate) -> String {
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

impl CircuitPrivate {
    // Slot Getters ---------------------------------------------

    pub fn a(&self) -> &Poly {
        &self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> &Poly {
        &self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> &Poly {
        &self.ws[Slots::C.id()]
    }

    pub fn _a(&self, i: usize) -> Scalar {
        self._ws[Slots::A.id()].evals[i]
    }

    pub fn _b(&self, i: usize) -> Scalar {
        self._ws[Slots::B.id()].evals[i]
    }

    pub fn _c(&self, i: usize) -> Scalar {
        self._ws[Slots::C.id()].evals[i]
    }
}

impl CircuitPublic {
    // Selector Getters ---------------------------------------------

    pub fn ql(&self) -> &Poly {
        &self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> &Poly {
        &self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> &Poly {
        &self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> &Poly {
        &self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> &Poly {
        &self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> &Poly {
        &self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> &Poly {
        &self.qs[Selectors::J.id()]
    }

    // Identity Permutation Getters ---------------------------------------------

    pub fn ia(&self) -> &Poly {
        &self.is[Slots::A.id()]
    }

    pub fn ib(&self) -> &Poly {
        &self.is[Slots::B.id()]
    }

    pub fn ic(&self) -> &Poly {
        &self.is[Slots::C.id()]
    }

    pub fn _ia(&self, i: usize) -> Scalar {
        self._is[Slots::A.id()].evals[i]
    }

    pub fn _ib(&self, i: usize) -> Scalar {
        self._is[Slots::B.id()].evals[i]
    }

    pub fn _ic(&self, i: usize) -> Scalar {
        self._is[Slots::C.id()].evals[i]
    }

    // Permutation Getters ---------------------------------------------

    pub fn pa(&self) -> &Poly {
        &self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> &Poly {
        &self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> &Poly {
        &self.ps[Slots::C.id()]
    }

    pub fn _pa(&self, i: usize) -> Scalar {
        self._ps[Slots::A.id()].evals[i]
    }

    pub fn _pb(&self, i: usize) -> Scalar {
        self._ps[Slots::B.id()].evals[i]
    }

    pub fn _pc(&self, i: usize) -> Scalar {
        self._ps[Slots::C.id()].evals[i]
    }
}
