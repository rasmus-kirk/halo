#![allow(non_snake_case)]

use halo_accumulation::{
    group::{PallasPoint, PallasScalar},
    pcdl::EvalProof,
};

use crate::{
    scheme::{Selectors, Slots},
    utils::misc::EnumIter,
};

#[derive(Clone)]
pub struct ProofEvaluations {
    pub ws: Vec<PallasScalar>,
    pub qs: Vec<PallasScalar>,
    pub pip: PallasScalar,
    pub ps: Vec<PallasScalar>,
    pub z: PallasScalar,
    pub ts: Vec<PallasScalar>,
    pub pls: Vec<PallasScalar>,
    pub z_bar: PallasScalar,
    pub t_bar: PallasScalar,
    pub h1_bar: PallasScalar,
}

impl ProofEvaluations {
    // Slots ------------------------------------------------------------------

    pub fn a(&self) -> PallasScalar {
        self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> PallasScalar {
        self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> PallasScalar {
        self.ws[Slots::C.id()]
    }

    // Selectors --------------------------------------------------------------

    pub fn ql(&self) -> PallasScalar {
        self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> PallasScalar {
        self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> PallasScalar {
        self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> PallasScalar {
        self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> PallasScalar {
        self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> PallasScalar {
        self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> PallasScalar {
        self.qs[Selectors::J.id()]
    }

    // Permutation ------------------------------------------------------------

    pub fn pa(&self) -> PallasScalar {
        self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> PallasScalar {
        self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> PallasScalar {
        self.ps[Slots::C.id()]
    }

    // Plookup ------------------------------------------------------

    pub fn t(&self) -> PallasScalar {
        self.pls[0]
    }

    pub fn f(&self) -> PallasScalar {
        self.pls[1]
    }

    pub fn h1(&self) -> PallasScalar {
        self.pls[2]
    }

    pub fn h2(&self) -> PallasScalar {
        self.pls[3]
    }
}

#[derive(Clone)]
pub struct ProofCommitments {
    pub ws: Vec<PallasPoint>,
    pub z: PallasPoint,
    pub ts: Vec<PallasPoint>,
}

#[derive(Clone)]
pub struct EvalProofs {
    pub W: EvalProof,
    pub W_bar: EvalProof,
}

#[derive(Clone)]
pub struct Proof {
    pub ev: ProofEvaluations,
    pub com: ProofCommitments,
    pub pis: EvalProofs,
}
