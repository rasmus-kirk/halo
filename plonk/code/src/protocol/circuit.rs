use super::{
    scheme::{Selectors, Slots},
    Coset,
};
use crate::{curve::Poly, protocol::scheme::Terms};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic {
    // coset scheme
    pub h: Coset,
    // selector polynomials
    pub qs: [Poly; Selectors::COUNT],
    // identity permutation polynomial
    pub sids: [Poly; Slots::COUNT],
    // permutation polynomial
    pub ss: [Poly; Slots::COUNT],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivate {
    // slot polynomials
    pub ws: [Poly; Slots::COUNT],
}

pub type Circuit = (CircuitPublic, CircuitPrivate);

pub fn print_poly_evaluations(x: &CircuitPublic, w: &CircuitPrivate) {
    println!("Circuit {{");
    for line in
        x.h.evals_str(
            w.ws.iter()
                .chain(x.qs.iter())
                .chain(x.ss.iter())
                .collect::<Vec<&Poly>>()
                .as_slice(),
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
