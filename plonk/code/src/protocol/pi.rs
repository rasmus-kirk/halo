#![allow(non_snake_case)]

use ark_ec::short_weierstrass::SWCurveConfig;
use halo_accumulation::pcdl::EvalProof;

use crate::{
    scheme::{Selectors, Slots},
    utils::{misc::EnumIter, Point, Scalar},
};

#[derive(Clone)]
pub struct ProofEvaluations<P: SWCurveConfig> {
    pub ws: Vec<Scalar<P>>,
    pub qs: Vec<Scalar<P>>,
    pub pip: Scalar<P>,
    pub ps: Vec<Scalar<P>>,
    pub z: Scalar<P>,
    pub ts: Vec<Scalar<P>>,
    pub pls: Vec<Scalar<P>>,
    pub z_bar: Scalar<P>,
    pub t_bar: Scalar<P>,
    pub h1_bar: Scalar<P>,
}

impl<P: SWCurveConfig> ProofEvaluations<P> {
    // Slots ------------------------------------------------------------------

    pub fn a(&self) -> Scalar<P> {
        self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> Scalar<P> {
        self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> Scalar<P> {
        self.ws[Slots::C.id()]
    }

    // Selectors --------------------------------------------------------------

    pub fn ql(&self) -> Scalar<P> {
        self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> Scalar<P> {
        self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> Scalar<P> {
        self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> Scalar<P> {
        self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> Scalar<P> {
        self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> Scalar<P> {
        self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> Scalar<P> {
        self.qs[Selectors::J.id()]
    }

    // Permutation ------------------------------------------------------------

    pub fn pa(&self) -> Scalar<P> {
        self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> Scalar<P> {
        self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> Scalar<P> {
        self.ps[Slots::C.id()]
    }

    // Plookup ------------------------------------------------------

    pub fn t(&self) -> Scalar<P> {
        self.pls[0]
    }

    pub fn f(&self) -> Scalar<P> {
        self.pls[1]
    }

    pub fn h1(&self) -> Scalar<P> {
        self.pls[2]
    }

    pub fn h2(&self) -> Scalar<P> {
        self.pls[3]
    }
}

#[derive(Clone)]
pub struct ProofCommitments<P: SWCurveConfig> {
    pub ws: Vec<Point<P>>,
    pub z: Point<P>,
    pub ts: Vec<Point<P>>,
}

#[derive(Clone)]
pub struct EvalProofs {
    pub W: EvalProof,
    pub W_bar: EvalProof,
}

#[derive(Clone)]
pub struct Proof<P: SWCurveConfig> {
    pub ev: ProofEvaluations<P>,
    pub com: ProofCommitments<P>,
    pub pis: EvalProofs,
}
