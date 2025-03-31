#![allow(non_snake_case)]

use halo_accumulation::{
    group::{PallasPoint, PallasScalar},
    pcdl::EvalProof,
};

use crate::scheme::{Selectors, Slots};

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
        self.ws[Slots::A as usize]
    }

    pub fn b(&self) -> PallasScalar {
        self.ws[Slots::B as usize]
    }

    pub fn c(&self) -> PallasScalar {
        self.ws[Slots::C as usize]
    }

    // Selectors --------------------------------------------------------------

    pub fn ql(&self) -> PallasScalar {
        self.qs[Selectors::Ql as usize]
    }

    pub fn qr(&self) -> PallasScalar {
        self.qs[Selectors::Qr as usize]
    }

    pub fn qo(&self) -> PallasScalar {
        self.qs[Selectors::Qo as usize]
    }

    pub fn qm(&self) -> PallasScalar {
        self.qs[Selectors::Qm as usize]
    }

    pub fn qc(&self) -> PallasScalar {
        self.qs[Selectors::Qc as usize]
    }

    pub fn qk(&self) -> PallasScalar {
        self.qs[Selectors::Qk as usize]
    }

    pub fn j(&self) -> PallasScalar {
        self.qs[Selectors::J as usize]
    }

    // Permutation ------------------------------------------------------------

    pub fn pa(&self) -> PallasScalar {
        self.ps[Slots::A as usize]
    }

    pub fn pb(&self) -> PallasScalar {
        self.ps[Slots::B as usize]
    }

    pub fn pc(&self) -> PallasScalar {
        self.ps[Slots::C as usize]
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
