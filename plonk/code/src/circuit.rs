use crate::{
    scheme::{Selectors, Terms},
    utils::print_table::evals_str,
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
    pub is_cache: Vec<Evals>,
    // permutation polynomial
    pub ps: Vec<Poly>,
    pub ps_cache: Vec<Evals>,

    pub pip_com: Point,
    pub qs_coms: Vec<Point>,
    pub ps_coms: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate {
    // slot polynomials
    pub ws: Vec<Poly>,
    pub ws_cache: Vec<Evals>,
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
        &self.ws[Slots::A as usize]
    }

    pub fn b(&self) -> &Poly {
        &self.ws[Slots::B as usize]
    }

    pub fn c(&self) -> &Poly {
        &self.ws[Slots::C as usize]
    }

    pub fn _a(&self) -> &Evals {
        &self.ws_cache[Slots::A as usize]
    }

    pub fn _b(&self) -> &Evals {
        &self.ws_cache[Slots::B as usize]
    }

    pub fn _c(&self) -> &Evals {
        &self.ws_cache[Slots::C as usize]
    }
}

impl CircuitPublic {
    // Selector Getters ---------------------------------------------

    pub fn ql(&self) -> &Poly {
        &self.qs[Selectors::Ql as usize]
    }

    pub fn qr(&self) -> &Poly {
        &self.qs[Selectors::Qr as usize]
    }

    pub fn qo(&self) -> &Poly {
        &self.qs[Selectors::Qo as usize]
    }

    pub fn qm(&self) -> &Poly {
        &self.qs[Selectors::Qm as usize]
    }

    pub fn qc(&self) -> &Poly {
        &self.qs[Selectors::Qc as usize]
    }

    pub fn qk(&self) -> &Poly {
        &self.qs[Selectors::Qk as usize]
    }

    pub fn j(&self) -> &Poly {
        &self.qs[Selectors::J as usize]
    }

    // Identity Permutation Getters ---------------------------------------------

    pub fn ia(&self) -> &Poly {
        &self.is[Slots::A as usize]
    }

    pub fn ib(&self) -> &Poly {
        &self.is[Slots::B as usize]
    }

    pub fn ic(&self) -> &Poly {
        &self.is[Slots::C as usize]
    }

    pub fn _ia(&self) -> &Evals {
        &self.is_cache[Slots::A as usize]
    }

    pub fn _ib(&self) -> &Evals {
        &self.is_cache[Slots::B as usize]
    }

    pub fn _ic(&self) -> &Evals {
        &self.is_cache[Slots::C as usize]
    }

    // Permutation Getters ---------------------------------------------

    pub fn pa(&self) -> &Poly {
        &self.ps[Slots::A as usize]
    }

    pub fn pb(&self) -> &Poly {
        &self.ps[Slots::B as usize]
    }

    pub fn pc(&self) -> &Poly {
        &self.ps[Slots::C as usize]
    }

    pub fn _pa(&self) -> &Evals {
        &self.ps_cache[Slots::A as usize]
    }

    pub fn _pb(&self) -> &Evals {
        &self.ps_cache[Slots::B as usize]
    }

    pub fn _pc(&self) -> &Evals {
        &self.ps_cache[Slots::C as usize]
    }
}
