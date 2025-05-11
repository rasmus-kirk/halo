use ark_ec::short_weierstrass::SWCurveConfig;
use ark_poly::univariate::DensePolynomial;

use super::{arithmetizer::PlookupEvsThunk, scheme::Slots};
use crate::{
    scheme::{Selectors, Terms},
    utils::{batch_p, misc::EnumIter, print_table::evals_str, Point, Poly, Scalar},
    Coset,
};

use educe::Educe;

#[derive(Educe)]
#[educe(Default, Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublicComs<P: SWCurveConfig> {
    // public input commitment
    pub pip: Point<P>,
    // selector commitments
    pub ql: Point<P>,
    pub qr: Point<P>,
    pub qo: Point<P>,
    pub qm: Point<P>,
    pub qc: Point<P>,
    pub qk: Point<P>,
    pub j: Point<P>,
    // permutation commitments
    pub pa: Point<P>,
    pub pb: Point<P>,
    pub pc: Point<P>,
}

impl<P: SWCurveConfig> CircuitPublicComs<P> {
    pub fn qs(&self) -> Vec<Point<P>> {
        vec![self.ql, self.qr, self.qo, self.qm, self.qc, self.qk, self.j]
    }
}

#[derive(Educe)]
#[educe(Default, Clone, Debug, PartialEq, Eq)]
pub struct CircuitPublic<P: SWCurveConfig> {
    pub d: usize,
    // coset scheme
    pub h: Coset<P>,
    // selector polynomials
    pub ql: Poly<P>,
    pub qr: Poly<P>,
    pub qo: Poly<P>,
    pub qm: Poly<P>,
    pub qc: Poly<P>,
    pub qk: Poly<P>,
    pub j: Poly<P>,
    // public input polynomial
    pub pip: Poly<P>,
    // identity permutation polynomial
    pub ia: Poly<P>,
    pub ib: Poly<P>,
    pub ic: Poly<P>,
    // permutation polynomial
    pub pa: Poly<P>,
    pub pb: Poly<P>,
    pub pc: Poly<P>,
    // commitments
    pub com: CircuitPublicComs<P>,
}

impl<P: SWCurveConfig> CircuitPublic<P> {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        d: usize,
        h: Coset<P>,
        qs: Vec<Poly<P>>,
        qs_coms: Vec<Point<P>>,
        pip: Poly<P>,
        pip_com: Point<P>,
        is: Vec<Poly<P>>,
        ps: Vec<Poly<P>>,
        ps_coms: Vec<Point<P>>,
    ) -> Self {
        let mut x = Self {
            d,
            h,
            pip,
            com: CircuitPublicComs {
                pip: pip_com,
                ..Default::default()
            },
            ..Default::default()
        };
        qs.into_iter().zip(qs_coms).zip(Selectors::iter()).for_each(
            |((poly, com), slot)| match slot {
                Selectors::Ql => {
                    x.ql = poly;
                    x.com.ql = com;
                }
                Selectors::Qr => {
                    x.qr = poly;
                    x.com.qr = com;
                }
                Selectors::Qo => {
                    x.qo = poly;
                    x.com.qo = com;
                }
                Selectors::Qm => {
                    x.qm = poly;
                    x.com.qm = com;
                }
                Selectors::Qc => {
                    x.qc = poly;
                    x.com.qc = com;
                }
                Selectors::Qk => {
                    x.qk = poly;
                    x.com.qk = com;
                }
                Selectors::J => {
                    x.j = poly;
                    x.com.j = com;
                }
            },
        );
        is.into_iter()
            .zip(ps)
            .zip(ps_coms)
            .zip(Slots::iter())
            .for_each(|(((i_poly, p_poly), com), slot)| match slot {
                Slots::A => {
                    x.ia = i_poly;
                    x.pa = p_poly;
                    x.com.pa = com;
                }
                Slots::B => {
                    x.ib = i_poly;
                    x.pb = p_poly;
                    x.com.pb = com;
                }
                Slots::C => {
                    x.ic = i_poly;
                    x.pc = p_poly;
                    x.com.pc = com;
                }
            });
        x
    }

    pub fn qs(&self) -> impl Iterator<Item = &Poly<P>> {
        [
            &self.ql, &self.qr, &self.qo, &self.qm, &self.qc, &self.qk, &self.j,
        ]
        .into_iter()
    }

    pub fn qsp(&self) -> Vec<&DensePolynomial<Scalar<P>>> {
        batch_p(self.qs())
    }

    pub fn ps(&self) -> impl Iterator<Item = &Poly<P>> {
        [&self.pa, &self.pb, &self.pc].into_iter()
    }

    pub fn is(&self) -> impl Iterator<Item = &Poly<P>> {
        [&self.ia, &self.ib, &self.ic].into_iter()
    }
}

#[derive(Educe)]
#[educe(Default, Clone, Debug, PartialEq, Eq)]
pub struct CircuitPrivateComs<P: SWCurveConfig> {
    pub a: Point<P>,
    pub b: Point<P>,
    pub c: Point<P>,
}

#[derive(Educe)]
#[educe(Clone, Default, Debug, PartialEq, Eq)]
pub struct CircuitPrivate<P: SWCurveConfig> {
    // slot polynomials
    pub a: Poly<P>,
    pub b: Poly<P>,
    pub c: Poly<P>,
    // thunk to compute Plonkup polys
    pub plookup: PlookupEvsThunk<P>,

    pub com: CircuitPrivateComs<P>,
}

impl<P: SWCurveConfig> CircuitPrivate<P> {
    pub fn new(ws: Vec<Poly<P>>, ws_coms: Vec<Point<P>>, plookup: PlookupEvsThunk<P>) -> Self {
        let mut w = Self {
            plookup,
            ..Default::default()
        };
        ws.into_iter()
            .zip(ws_coms)
            .zip(Slots::iter())
            .for_each(|((poly, com), slot)| match slot {
                Slots::A => {
                    w.a = poly;
                    w.com.a = com;
                }
                Slots::B => {
                    w.b = poly;
                    w.com.b = com;
                }
                Slots::C => {
                    w.c = poly;
                    w.com.c = com;
                }
            });
        w
    }

    pub fn ws(&self) -> impl Iterator<Item = &Poly<P>> {
        [&self.a, &self.b, &self.c].into_iter()
    }

    pub fn wsp(&self) -> Vec<&DensePolynomial<Scalar<P>>> {
        batch_p(self.ws())
    }
}

pub type Circuit<P> = (CircuitPublic<P>, CircuitPrivate<P>);

pub fn poly_evaluations_to_string<P: SWCurveConfig>(
    x: &CircuitPublic<P>,
    w: &CircuitPrivate<P>,
) -> String {
    let mut result = String::from("Circuit {\n");
    let polys = w
        .ws()
        .chain(x.qs())
        .chain(std::iter::once(&x.pip))
        .chain(x.ps())
        .collect::<Vec<&Poly<P>>>();
    for line in evals_str(
        &x.h,
        batch_p(polys),
        Terms::iter()
            .map(|t| t.to_string())
            .chain(Slots::iter().map(|slot| slot.perm_string().to_string()))
            // .chain(Slots::iter().map(|slot| slot.perm_string().to_string() + "id"))
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
