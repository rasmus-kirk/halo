use halo_accumulation::{
    group::{PallasPoint, PallasScalar},
    pcdl::EvalProof,
};

#[derive(Clone)]
pub struct ProofEvaluations {
    pub a: PallasScalar,
    pub b: PallasScalar,
    pub c: PallasScalar,
    pub qc: PallasScalar,
    pub ql: PallasScalar,
    pub qm: PallasScalar,
    pub qo: PallasScalar,
    pub qr: PallasScalar,
    pub pip: PallasScalar,
    pub sa: PallasScalar,
    pub sb: PallasScalar,
    pub sc: PallasScalar,
    pub z: PallasScalar,
    pub t_parts: Vec<PallasScalar>,
    pub pl_j: PallasScalar,
    pub pl_qk: PallasScalar,
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
    pub a: PallasPoint,
    pub b: PallasPoint,
    pub c: PallasPoint,
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
