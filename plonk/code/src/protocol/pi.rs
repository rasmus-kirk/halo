#![allow(non_snake_case)]

use halo_accumulation::{
    group::{PallasPoint, PallasScalar},
    pcdl::EvalProof,
};

#[derive(Clone)]
pub struct ProofEvaluations {
    pub ws: Vec<PallasScalar>,
    pub qs: Vec<PallasScalar>,
    pub pip: PallasScalar,
    pub ss: Vec<PallasScalar>,
    pub z: PallasScalar,
    pub ts: Vec<PallasScalar>,
    pub pl_f: PallasScalar,
    pub pl_t: PallasScalar,
    pub pl_h1: PallasScalar,
    pub pl_h2: PallasScalar,
    pub z_bar: PallasScalar,
    pub pl_t_bar: PallasScalar,
    pub pl_h1_bar: PallasScalar,
}

#[derive(Clone)]
pub struct ProofCommitments {
    pub abc_coms: Vec<PallasPoint>,
    pub z: PallasPoint,
    pub t_coms: Vec<PallasPoint>,
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
