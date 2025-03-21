use super::{instance::Instance, transcript::TranscriptProtocol};
use crate::{curve::Scalar, protocol::circuit::CircuitPublic};

use merlin::Transcript;

#[derive(Clone)]
pub struct SNARKProof {
    pub q_a: Instance,
    pub q_b: Instance,
    pub q_c: Instance,
    pub q_fgc: Instance,
    pub q_z: Instance,
    pub q_fz1: Instance,
    pub zbar_ev: Scalar,
    pub q_fz2: Instance,
    pub q_t: Instance,
    pub q_tpl: Instance,
    pub tplbar_ev: Scalar,
    pub fpl_ev: Scalar,
    pub q_h1: Instance,
    pub q_h2: Instance,
    pub h1plbar_ev: Scalar,
}

pub fn verify(x: &CircuitPublic, pi: SNARKProof) -> bool {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // Round 1 -----------------------------------------------------

    transcript.append_points(b"abc", &[pi.q_a.comm, pi.q_b.comm, pi.q_c.comm]);
    // Round 2 -----------------------------------------------------
    let zeta = &transcript.challenge_scalar(b"zeta");
    // Round 3 -----------------------------------------------------
    let beta = &transcript.challenge_scalar_augment(1, b"beta");
    let gamma = &transcript.challenge_scalar_augment(2, b"gamma");
    let delta = &transcript.challenge_scalar_augment(3, b"delta");
    let epsilon = &transcript.challenge_scalar_augment(4, b"epsilon");
    transcript.append_point(b"z", &pi.q_z.comm);
    // Round 4 -----------------------------------------------------
    let alpha = &transcript.challenge_scalar(b"alpha");
    transcript.append_point(b"t", &pi.q_t.comm);
    // Round 5 -----------------------------------------------------
    let ch = &transcript.challenge_scalar(b"xi");
    let sida_ev = &x.sida.evaluate(ch);
    let sidb_ev = &x.sidb.evaluate(ch);
    let sidc_ev = &x.sidc.evaluate(ch);
    let sa_ev = &x.sa.evaluate(ch);
    let sb_ev = &x.sb.evaluate(ch);
    let sc_ev = &x.sc.evaluate(ch);
    let zh_ev = ch.pow(x.h.n()) - Scalar::ONE;
    let l1_ev_ch = x.h.l1_ev(ch);
    // get / compute evaluations on challenge
    let a = &pi.q_a.ev.unwrap();
    let b = &pi.q_b.ev.unwrap();
    let c = &pi.q_c.ev.unwrap();
    let ql = &x.ql.evaluate(ch);
    let qr = &x.qr.evaluate(ch);
    let qo = &x.qo.evaluate(ch);
    let qm = &x.qm.evaluate(ch);
    let qc = &x.qc.evaluate(ch);
    let qk = &x.pl_qk.evaluate(ch);
    let j_ev = &x.pl_j.evaluate(ch);
    let pi_ev = x.pip.evaluate(ch);
    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    //         + Qₖ(𝔷)(A(𝔷) + ζB(𝔷) + ζ²C(𝔷) + ζ³J(𝔷) - f(𝔷))
    let f_gcpl_ev = &(qk * (a + zeta * b + zeta.pow(2) * c + zeta.pow(3) * j_ev - pi.fpl_ev));
    let f_gc_ev = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + pi_ev + f_gcpl_ev);
    if *f_gc_ev == Scalar::ZERO || !pi.q_fgc.check(ch, Some(f_gc_ev)) {
        println!("FAILED GC");
        return false;
    }
    // F_Z1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_z1_ev = &(l1_ev_ch * (pi.q_z.ev.unwrap() - Scalar::ONE));
    if !pi.q_fz1.check(ch, Some(f_z1_ev)) {
        println!("FAILED CC1");
        return false;
    }
    let zpl_sc = &(epsilon * (Scalar::ONE + delta));
    let zcc = |w, s| w + beta * s + gamma;
    let zpl = |a, b| zpl_sc + a + delta * b;
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    //         (ε(1 + δ) + f(X) + δf(X))(ε(1 + δ) + t(X) + δt(Xω))
    let zfcc_ev = &(zcc(a, sida_ev) * zcc(b, sidb_ev) * zcc(c, sidc_ev));
    let zfpl_ev = &(zpl(pi.fpl_ev, pi.fpl_ev) * zpl(pi.q_tpl.ev.unwrap(), pi.tplbar_ev));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X))(ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgcc_ev = &(zcc(a, sa_ev) * zcc(b, sb_ev) * zcc(c, sc_ev));
    let h2_ev = pi.q_h2.ev.unwrap();
    let zgpl_ev = &(zpl(pi.q_h1.ev.unwrap(), h2_ev) * zpl(h2_ev, pi.h1plbar_ev));
    // F_Z2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_z2_ev = &((pi.q_z.ev.unwrap() * zfcc_ev * zfpl_ev) - (zgcc_ev * zgpl_ev * pi.zbar_ev));
    if !pi.q_fz2.check(ch, Some(f_z2_ev)) {
        println!("FAILED CC2");
        return false;
    }

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    (f_gc_ev + (alpha * f_z1_ev) + (alpha.pow(2) * f_z2_ev)) - (pi.q_t.ev.unwrap() * zh_ev)
        == Scalar::ZERO
}

// TODO use commits instead of evals
// TODO avoid using evaluate in verifier
