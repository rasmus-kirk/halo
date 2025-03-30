use crate::{scheme::Terms, util::print_table::evals_str, Coset};

use super::{arithmetizer::PlookupEvsThunk, scheme::Slots};

use halo_accumulation::group::{PallasPoint, PallasPoly, PallasScalar};

use ark_poly::Evaluations;

type Scalar = PallasScalar;
type Poly = PallasPoly;
type Point = PallasPoint;

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
    pub sids: Vec<Poly>,
    pub sids_cache: Vec<Evaluations<Scalar>>,
    // permutation polynomial
    pub ss: Vec<Poly>,
    pub ss_cache: Vec<Evaluations<Scalar>>,

    pub pip_com: Point,
    pub qs_coms: Vec<Point>,
    pub ss_coms: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate {
    // slot polynomials
    pub ws: Vec<Poly>,
    pub ws_cache: Vec<Evaluations<Scalar>>,
    // thunk to compute Plonkup polys
    pub plonkup: PlookupEvsThunk,
}

pub type Circuit = (CircuitPublic, CircuitPrivate);

pub fn poly_evaluations_to_string(x: &CircuitPublic, w: &CircuitPrivate) -> String {
    let mut result = String::from("Circuit {\n");
    let polys =
        w.ws.iter()
            .chain(x.qs.iter())
            .chain(std::iter::once(&x.pip))
            .chain(x.ss.iter())
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
