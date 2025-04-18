#![allow(non_snake_case)]

use super::{transcript::TranscriptProtocol, Proof};
use crate::{
    circuit::CircuitPublic,
    scheme::{Selectors, Slots},
    utils::{poly, scalar},
};

use halo_accumulation::{group::PallasScalar, pcdl};

use anyhow::{ensure, Result};
use ark_ff::Field;
use merlin::Transcript;

type Scalar = PallasScalar;

pub fn verify(x: &CircuitPublic, pi: Proof) -> Result<()> {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // -------------------- Round 1 --------------------

    transcript.append_points(b"abc", &pi.com.ws);

    // -------------------- Round 2 --------------------

    let zeta = &transcript.challenge_scalar(b"zeta");

    // -------------------- Round 3 --------------------

    // β = H(transcript)
    let beta = &transcript.challenge_scalar(b"beta");
    // γ = H(transcript)
    let gamma = &transcript.challenge_scalar(b"gamma");
    // δ = H(transcript)
    let delta = &transcript.challenge_scalar(b"delta");
    // ε = H(transcript)
    let epsilon = &transcript.challenge_scalar(b"epsilon");
    transcript.append_point(b"z", &pi.com.z);

    // -------------------- Round 4 --------------------

    let alpha = &transcript.challenge_scalar(b"alpha");
    transcript.append_points(b"t", &pi.com.ts);

    // -------------------- Round 5 --------------------

    let ch = &transcript.challenge_scalar(b"xi");
    let ch_w = ch * &x.h.w(1);
    let zh_ev = scalar::zh_ev(&x.h, ch);
    let l1_ev_ch = scalar::lagrange_basis1(&x.h, ch);
    let sids_ev = poly::batch_evaluate(&x.is, ch);
    let a = &pi.ev.ws[Slots::A as usize];
    let b = &pi.ev.ws[Slots::B as usize];
    let c = &pi.ev.ws[Slots::C as usize];
    let ql = &pi.ev.qs[Selectors::Ql as usize];
    let qr = &pi.ev.qs[Selectors::Qr as usize];
    let qo = &pi.ev.qs[Selectors::Qo as usize];
    let qm = &pi.ev.qs[Selectors::Qm as usize];
    let qc = &pi.ev.qs[Selectors::Qc as usize];
    let qk = &pi.ev.qs[Selectors::Qk as usize];
    let j = &pi.ev.qs[Selectors::J as usize];
    let sida_ev = &sids_ev[Slots::A as usize];
    let sidb_ev = &sids_ev[Slots::B as usize];
    let sidc_ev = &sids_ev[Slots::C as usize];
    let sa_ev = &pi.ev.ss[Slots::A as usize];
    let sb_ev = &pi.ev.ss[Slots::B as usize];
    let sc_ev = &pi.ev.ss[Slots::C as usize];
    let pip = &pi.ev.pip;
    let pl_t = &pi.ev.pls[0];
    let pl_f = &pi.ev.pls[1];
    let pl_h1 = &pi.ev.pls[2];
    let pl_h2 = &pi.ev.pls[3];
    let t_evs = pi.ev.ts;

    transcript.append_scalars(b"ws_ev", &pi.ev.ws);
    transcript.append_scalars(b"qs_ev", &pi.ev.qs);
    transcript.append_scalars(b"ss_ev", &pi.ev.ss);
    transcript.append_scalars(b"plonkup_ev", &pi.ev.pls);
    transcript.append_scalar(b"z_bar_ev", &pi.ev.z_bar);
    transcript.append_scalars(b"t_ev", &t_evs);
    transcript.append_scalar(b"z_ev", &pi.ev.z);

    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    //         + Qₖ(𝔷)(A(𝔷) + ζB(𝔷) + ζ²C(𝔷) + ζ³J(𝔷) - f(𝔷))
    let f_gcpl_ev = &(*qk * (scalar::linear_comb_right(zeta, [*a, *b, *c, *j]) - pl_f));
    let f_gc_ev = (a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + pip + f_gcpl_ev;

    // plookup constraint term: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &((Scalar::ONE + delta) * epsilon);
    let zpl = |a: &Scalar, b: &Scalar| zpl_sc + a + (delta * b);
    // copy constraint term: w(X) + β s(X) + γ
    let zcc = |w: &Scalar, s: &Scalar| *w + (beta * s) + gamma;
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    //         (ε(1 + δ) + f(𝔷) + δf(𝔷))(ε(1 + δ) + t(𝔷) + δt(Xω))
    let zfcc_ev = &(zcc(a, sida_ev) * zcc(b, sidb_ev) * zcc(c, sidc_ev));
    let zfpl_ev = &(zpl(pl_f, pl_f) * zpl(pl_t, &pi.ev.pl_t_bar));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    //         (ε(1 + δ) + h₁(𝔷) + δh₂(𝔷))(ε(1 + δ) + h₂(𝔷) + δh₁(Xω))
    let zgcc_ev = &(zcc(a, sa_ev) * zcc(b, sb_ev) * zcc(c, sc_ev));
    let zgpl_ev = &(zpl(pl_h1, pl_h2) * zpl(pl_h2, &pi.ev.pl_h1_bar));

    // F_Z1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_z1_ev = l1_ev_ch * (pi.ev.z - Scalar::ONE);
    // F_Z2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_z2_ev = (pi.ev.z * zfcc_ev * zfpl_ev) - (zgcc_ev * zgpl_ev * pi.ev.z_bar);

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    let t_ev = scalar::linear_comb(&ch.pow([x.h.n()]), t_evs);
    ensure!(
        scalar::linear_comb(alpha, [f_gc_ev, f_z1_ev, f_z2_ev]) == t_ev * zh_ev,
        "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)"
    );

    let v = &transcript.challenge_scalar(b"v");

    // W(𝔷) = Qₗ(𝔷) + vQᵣ(𝔷) + v²Qₒ(𝔷) + v³Qₘ(𝔷) + v⁴Q꜀(𝔷) + v⁵Qₖ(𝔷) + v⁶J(𝔷)
    //      + v⁷A(𝔷) + v⁸B(𝔷) + v⁹C(𝔷) + v¹⁰Z(𝔷)
    let W_com = scalar::linear_comb(
        v,
        x.qs_coms
            .iter()
            .chain(pi.com.ws.iter())
            .chain(std::iter::once(&pi.com.z))
            .cloned(),
    );
    let W_ev = scalar::linear_comb(
        v,
        pi.ev
            .qs
            .iter()
            .chain(pi.ev.ws.iter())
            .chain(std::iter::once(&pi.ev.z))
            .cloned(),
    );
    pcdl::check(&W_com, x.d, ch, &W_ev, pi.pis.W)?;
    // W'(𝔷) = Z(ω𝔷)
    pcdl::check(&pi.com.z, x.d, &ch_w, &pi.ev.z_bar, pi.pis.W_bar)?;

    Ok(())
}
