use super::{
    pcdl::PCDLProof,
    transcript::TranscriptProtocol,
};
use crate::{
    curve::{Point, Poly, Scalar},
    protocol::{circuit::CircuitPublic, scheme::Slots},
};

use halo_accumulation::pcdl;
use merlin::Transcript;

pub struct SNARKProof {
    pub comms_abc: [Point; Slots::COUNT],
    pub abc_ev: [Scalar; Slots::COUNT],
    pub comm_fgc: Point,
    pub comm_z: Point,
    pub comm_fcc1: Point,
    pub zbar_ev: Scalar,
    pub comm_fcc2: Point,
    pub comm_t: Point,
    pub q_tw: PCDLProof<false>,
}

pub fn verify(x: &CircuitPublic, pi: SNARKProof) -> bool {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    let [sa, sb, sc] = &x.ss;
    let [sida, sidb, sidc] = &x.sids;
    // Round 1 -----------------------------------------------------
    transcript.append_points(b"abc", &pi.comms_abc);
    // Round 2 -----------------------------------------------------
    let beta = &transcript.challenge_scalar_augment(0, b"beta");
    let gamma = &transcript.challenge_scalar_augment(1, b"gamma");
    transcript.append_point(b"z", &pi.comm_z);
    // Round 3 -----------------------------------------------------
    let alpha = &transcript.challenge_scalar(b"alpha");
    transcript.append_point(b"t", &pi.comm_t);
    // Round 4 -----------------------------------------------------
    let ch = &transcript.challenge_scalar(b"xi");
    // get / compute evaluations on challenge
    let [a, b, c] = &pi.abc_ev;
    let [ql, qr, qo, qm, qc] = &Poly::commit_many(&x.qs);
    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    let pt_fgc: &Point = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc).into();
    // F_CC1(𝔷) = L₁(𝔷) (Z(𝔷) - 1) = (L₁(𝔷) Z(𝔷)) - (L₁(𝔷))
    let l1_ev = &x.h.lagrange(1).evaluate(ch);
    let pt_fcc1: &Point = &(l1_ev * pi.comm_z);
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    let zf_ev = &((a + beta * sida.evaluate(ch) + gamma)
        * (b + beta * sidb.evaluate(ch) + gamma)
        * (c + beta * sidc.evaluate(ch) + gamma));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    let zg_ev = &((a + beta * sa.evaluate(ch) + gamma) * (b + beta * sb.evaluate(ch) + gamma) * (c + beta * sc.evaluate(ch) + gamma));
    // F_CC2(𝔷) = (Z(𝔷)f'(𝔷)) - (g'(𝔷)Z(ω 𝔷))
    let pt_fcc2: &Point = &(pi.comm_z * zf_ev).into();
    let val_fcc2 = &(zg_ev * pi.zbar_ev);
    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    // F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷) - T(𝔷) * Zₕ(𝔷) = 0
    let zh_ev = &x.h.zh().evaluate(ch);
    let alpha2 = &alpha.pow(2);
    let pt_t: Point = pi.comm_t.into();
    let pt_tv: Point = pt_fgc + (alpha * pt_fcc1) + (alpha2 * pt_fcc2) - (pt_t * zh_ev);
    let t_ev = (alpha * l1_ev) + (alpha2 * val_fcc2);
    pcdl::check(&pt_tv.into(), pi.q_tw.comm.into(), &ch.into(), &t_ev.into(), pi.q_tw.pi).is_ok()
}

// TODO use commits instead of evals
// TODO avoid using evaluate in verifier
