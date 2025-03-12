use super::{arithmetizer::PlonkupVecCompute, scheme::Slots};
use crate::{
    curve::{Coset, Poly},
    protocol::scheme::Terms,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic {
    pub d: usize,
    // coset scheme
    pub h: Coset,
    // selector polynomials
    pub ql: Poly,
    pub qr: Poly,
    pub qo: Poly,
    pub qm: Poly,
    pub qc: Poly,
    pub pl_qk: Poly,
    pub pl_j: Poly,
    // public input polynomial
    pub pip: Poly,
    // identity permutation polynomial
    pub sida: Poly,
    pub sidb: Poly,
    pub sidc: Poly,
    // permutation polynomial
    pub sa: Poly,
    pub sb: Poly,
    pub sc: Poly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate {
    // slot polynomials
    pub a: Poly,
    pub b: Poly,
    pub c: Poly,
    // thunk to compute Plonkup polys
    pub plonkup: PlonkupVecCompute,
}

pub type Circuit = (CircuitPublic, CircuitPrivate);

pub fn print_poly_evaluations(x: &CircuitPublic, w: &CircuitPrivate) {
    println!("Circuit {{");
    for line in
        x.h.evals_str(
            vec![
                &w.a, &w.b, &w.c, &x.ql, &x.qr, &x.qo, &x.qm, &x.qc, &x.pl_qk, &x.pl_j, &x.pip,
                &x.sa, &x.sb, &x.sc,
            ],
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
        println!("    {}", line);
    }
    println!("}}");
}
