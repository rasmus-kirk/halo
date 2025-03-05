use super::{
    instance::{many::Instances, Instance},
    transcript::TranscriptProtocol,
};
use crate::{
    curve::{Poly, Scalar},
    protocol::{circuit::CircuitPublic, scheme::Slots},
};

use merlin::Transcript;

#[derive(Clone)]
pub struct SNARKProof {
    pub qs_abc: Instances<{ Slots::COUNT }>,
    pub q_fgc: Instance,
    pub q_z: Instance,
    pub q_fcc1: Instance,
    pub zbar_ev: Scalar,
    pub q_fcc2: Instance,
    pub q_t: Instance,
    pub q_tpl: Instance,
    pub tplbar_ev: Scalar,
    pub fpl_ev: Scalar,
    pub jpl_ev: Scalar,
    pub q_zpl: Instance,
    pub zplbar_ev: Scalar,
    pub q_h1: Instance,
    pub q_h2: Instance,
    pub h1plbar_ev: Scalar,
}

pub fn verify(x: &CircuitPublic, pi: SNARKProof) -> bool {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    let [sa, sb, sc] = &x.ss;
    let [sida, sidb, sidc] = &x.sids;
    // Round 1 -----------------------------------------------------
    transcript.append_points(b"abc", &Instances::get_comms(&pi.qs_abc));
    // Round 2 -----------------------------------------------------
    let zeta = &transcript.challenge_scalar(b"zeta");
    // Round 3 -----------------------------------------------------
    let beta = &transcript.challenge_scalar_augment(1, b"beta");
    let gamma = &transcript.challenge_scalar_augment(2, b"gamma");
    transcript.append_point(b"z", &pi.q_z.comm);

    let delta = &transcript.challenge_scalar_augment(3, b"delta");
    let epsilon = &transcript.challenge_scalar_augment(4, b"epsilon");
    transcript.append_point(b"zpl", &pi.q_zpl.comm);
    // Round 4 -----------------------------------------------------
    let alpha = &transcript.challenge_scalar(b"alpha");
    transcript.append_point(b"t", &pi.q_t.comm);
    // Round 5 -----------------------------------------------------
    let ch = &transcript.challenge_scalar(b"xi");
    let zh_ev = &x.h.zh().evaluate(ch);
    // check commits
    if !Instances::check(&pi.qs_abc, ch) || !pi.q_z.check(ch, None) || !pi.q_t.check(ch, None) {
        println!("FAILED COMMITS");
        return false;
    }
    // get / compute evaluations on challenge
    let [a, b, c] = &Instances::get_evs(&pi.qs_abc).unwrap();
    let [ql, qr, qo, qm, qc, qk, _] = &Poly::evaluate_many(&x.qs, ch);
    let pi_ev = x.pi.evaluate(ch);
    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    let fplgc_ev =
        &(qk * (a + zeta * b + zeta * zeta * c + zeta * zeta * zeta * pi.jpl_ev - pi.fpl_ev));
    let f_gc_ev = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + pi_ev + fplgc_ev);
    if *f_gc_ev == Scalar::ZERO || !pi.q_fgc.check(ch, Some(f_gc_ev)) {
        println!("FAILED GC");
        return false;
    }
    // F_CC1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_cc1_ev = &(x.h.lagrange(1).evaluate(ch) * (pi.q_z.ev.unwrap() - Scalar::ONE));
    if !pi.q_fcc1.check(ch, Some(f_cc1_ev)) {
        println!("FAILED CC1");
        return false;
    }
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    let zf_ev = &((a + beta * sida.evaluate(ch) + gamma)
        * (b + beta * sidb.evaluate(ch) + gamma)
        * (c + beta * sidc.evaluate(ch) + gamma));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    let zg_ev = &((a + beta * sa.evaluate(ch) + gamma)
        * (b + beta * sb.evaluate(ch) + gamma)
        * (c + beta * sc.evaluate(ch) + gamma));
    // F_CC2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_cc2_ev = &((pi.q_z.ev.unwrap() * zf_ev) - (zg_ev * pi.zbar_ev));
    if !pi.q_fcc2.check(ch, Some(f_cc2_ev)) {
        println!("FAILED CC2");
        return false;
    }

    // F_PL1(𝔷) = L₁(𝔷) (Z_PL(𝔷) - 1)
    let f_pl1_ev = &(x.h.lagrange(1).evaluate(ch) * (pi.q_zpl.ev.unwrap() - Scalar::ONE));
    // f'(𝔷) = (ε(1 + δ) + f(𝔷) + δf(𝔷))(ε(1 + δ) + t(𝔷) + δt(𝔷ω))
    let zplf_ev = &((epsilon * (Scalar::ONE + delta) + pi.fpl_ev + (delta * pi.fpl_ev))
        * (epsilon * (Scalar::ONE + delta) + pi.q_tpl.ev.unwrap() + (delta * pi.tplbar_ev)));
    // g'(𝔷) = (ε(1 + δ) + h₁(𝔷) + δh₂(𝔷))(ε(1 + δ) + h₂(𝔷) + δh₁(𝔷ω))
    let zplg_ev =
        &((epsilon * (Scalar::ONE + delta) + pi.q_h1.ev.unwrap() + (delta * pi.q_h2.ev.unwrap()))
            * (epsilon * (Scalar::ONE + delta) + pi.q_h2.ev.unwrap() + delta * pi.h1plbar_ev));
    // F_PL2(𝔷) = Z_PL(𝔷)f'(𝔷) - g'(𝔷)Z_PL(ω 𝔷)
    let f_pl2_ev = &((pi.q_zpl.ev.unwrap() * zplf_ev) - (zplg_ev * pi.zplbar_ev));

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    (f_gc_ev
        + (alpha * f_cc1_ev)
        + (alpha.pow(2) * f_cc2_ev)
        + (alpha.pow(3) * f_pl1_ev)
        + (alpha.pow(4) * f_pl2_ev))
        - (pi.q_t.ev.unwrap() * zh_ev)
        == Scalar::ZERO
}

// TODO use commits instead of evals
// TODO avoid using evaluate in verifier
