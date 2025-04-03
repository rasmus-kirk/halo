#![allow(non_snake_case)]

use ark_ff::{Fp, FpConfig};
use halo_accumulation::{group::PallasPoint, pcdl::EvalProof};

use crate::{
    scheme::{Selectors, Slots},
    utils::misc::EnumIter,
};

#[derive(Clone)]
pub struct ProofEvaluations<const N: usize, C: FpConfig<N>> {
    pub ws: Vec<Fp<C, N>>,
    pub qs: Vec<Fp<C, N>>,
    pub pip: Fp<C, N>,
    pub ps: Vec<Fp<C, N>>,
    pub z: Fp<C, N>,
    pub ts: Vec<Fp<C, N>>,
    pub pls: Vec<Fp<C, N>>,
    pub z_bar: Fp<C, N>,
    pub t_bar: Fp<C, N>,
    pub h1_bar: Fp<C, N>,
}

impl<const N: usize, C: FpConfig<N>> ProofEvaluations<N, C> {
    // Slots ------------------------------------------------------------------

    pub fn a(&self) -> Fp<C, N> {
        self.ws[Slots::A.id()]
    }

    pub fn b(&self) -> Fp<C, N> {
        self.ws[Slots::B.id()]
    }

    pub fn c(&self) -> Fp<C, N> {
        self.ws[Slots::C.id()]
    }

    // Selectors --------------------------------------------------------------

    pub fn ql(&self) -> Fp<C, N> {
        self.qs[Selectors::Ql.id()]
    }

    pub fn qr(&self) -> Fp<C, N> {
        self.qs[Selectors::Qr.id()]
    }

    pub fn qo(&self) -> Fp<C, N> {
        self.qs[Selectors::Qo.id()]
    }

    pub fn qm(&self) -> Fp<C, N> {
        self.qs[Selectors::Qm.id()]
    }

    pub fn qc(&self) -> Fp<C, N> {
        self.qs[Selectors::Qc.id()]
    }

    pub fn qk(&self) -> Fp<C, N> {
        self.qs[Selectors::Qk.id()]
    }

    pub fn j(&self) -> Fp<C, N> {
        self.qs[Selectors::J.id()]
    }

    // Permutation ------------------------------------------------------------

    pub fn pa(&self) -> Fp<C, N> {
        self.ps[Slots::A.id()]
    }

    pub fn pb(&self) -> Fp<C, N> {
        self.ps[Slots::B.id()]
    }

    pub fn pc(&self) -> Fp<C, N> {
        self.ps[Slots::C.id()]
    }

    // Plookup ------------------------------------------------------

    pub fn t(&self) -> Fp<C, N> {
        self.pls[0]
    }

    pub fn f(&self) -> Fp<C, N> {
        self.pls[1]
    }

    pub fn h1(&self) -> Fp<C, N> {
        self.pls[2]
    }

    pub fn h2(&self) -> Fp<C, N> {
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
pub struct Proof<const N: usize, C: FpConfig<N>> {
    pub ev: ProofEvaluations<N, C>,
    pub com: ProofCommitments,
    pub pis: EvalProofs,
}
