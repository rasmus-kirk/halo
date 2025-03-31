#![allow(non_snake_case)]

use super::{transcript::TranscriptProtocol, Proof};
use crate::{
    circuit::CircuitPublic,
    scheme::{eqns::plonkup_eqn, Slots},
    utils::{
        general::geometric,
        poly::{self},
        scalar,
    },
};

use halo_accumulation::{group::PallasScalar, pcdl};

use anyhow::{ensure, Result};
use ark_ff::Field;
use merlin::Transcript;

type Scalar = PallasScalar;

pub fn verify(x: &CircuitPublic, pi: Proof) -> Result<()> {
    let ev = &pi.ev;
    let com = &pi.com;
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // -------------------- Round 1 --------------------

    transcript.append_points(b"abc", &com.ws);

    // -------------------- Round 2 --------------------

    let zeta = transcript.challenge_scalar(b"zeta");

    // -------------------- Round 3 --------------------

    // β = H(transcript)
    let beta = &transcript.challenge_scalar(b"beta");
    // γ = H(transcript)
    let gamma = &transcript.challenge_scalar(b"gamma");
    // δ = H(transcript)
    let delta = &transcript.challenge_scalar(b"delta");
    // ε = H(transcript)
    let epsilon = &transcript.challenge_scalar(b"epsilon");
    transcript.append_point(b"z", &com.z);

    // -------------------- Round 4 --------------------

    let alpha = transcript.challenge_scalar(b"alpha");
    transcript.append_points(b"t", &com.ts);

    // -------------------- Round 5 --------------------

    let ch = transcript.challenge_scalar(b"xi");
    let ch_w = ch * x.h.w(1);
    let zh_ev = scalar::zh_ev(&x.h, ch);
    let l1_ev_ch = scalar::lagrange_basis1(&x.h, ch);
    let is_ev = poly::batch_evaluate(&x.is, ch);
    let ia_ev = &is_ev[Slots::A as usize];
    let ib_ev = &is_ev[Slots::B as usize];
    let ic_ev = &is_ev[Slots::C as usize];

    transcript.append_scalars(b"ws_ev", &ev.ws);
    transcript.append_scalars(b"qs_ev", &ev.qs);
    transcript.append_scalars(b"ss_ev", &ev.ps);
    transcript.append_scalars(b"plonkup_ev", &ev.pls);
    transcript.append_scalar(b"z_bar_ev", &ev.z_bar);
    transcript.append_scalars(b"t_ev", &ev.ts);
    transcript.append_scalar(b"z_ev", &ev.z);

    let f_gc_ev = plonkup_eqn(zeta, ev.ws.clone(), ev.qs.clone(), ev.pip, *ev.f());

    // plookup constraint term: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &((Scalar::ONE + delta) * epsilon);
    let zpl = |a: &Scalar, b: &Scalar| zpl_sc + a + (delta * b);
    // copy constraint term: w(X) + β s(X) + γ
    let zcc = |w: &Scalar, s: &Scalar| *w + (beta * s) + gamma;
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    //         (ε(1 + δ) + f(𝔷) + δf(𝔷))(ε(1 + δ) + t(𝔷) + δt(Xω))
    let zfcc_ev = &(zcc(ev.a(), ia_ev) * zcc(ev.b(), ib_ev) * zcc(ev.c(), ic_ev));
    let zfpl_ev = &(zpl(ev.f(), ev.f()) * zpl(ev.t(), &ev.t_bar));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    //         (ε(1 + δ) + h₁(𝔷) + δh₂(𝔷))(ε(1 + δ) + h₂(𝔷) + δh₁(Xω))
    let zgcc_ev = &(zcc(ev.a(), ev.pa()) * zcc(ev.b(), ev.pb()) * zcc(ev.c(), ev.pc()));
    let zgpl_ev = &(zpl(ev.h1(), ev.h2()) * zpl(ev.h2(), &ev.h1_bar));

    // F_Z1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_z1_ev = l1_ev_ch * (ev.z - Scalar::ONE);
    // F_Z2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_z2_ev = (ev.z * zfcc_ev * zfpl_ev) - (zgcc_ev * zgpl_ev * ev.z_bar);

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    let t_ev = geometric(ch.pow([x.h.n()]), ev.ts.clone());
    ensure!(
        geometric(alpha, [f_gc_ev, f_z1_ev, f_z2_ev]) == t_ev * zh_ev,
        "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)"
    );

    let v = transcript.challenge_scalar(b"v");

    // W(𝔷) = Qₗ(𝔷) + vQᵣ(𝔷) + v²Qₒ(𝔷) + v³Qₘ(𝔷) + v⁴Q꜀(𝔷) + v⁵Qₖ(𝔷) + v⁶J(𝔷)
    //      + v⁷A(𝔷) + v⁸B(𝔷) + v⁹C(𝔷) + v¹⁰Z(𝔷)
    let W_com = geometric(
        v,
        x.qs_coms
            .iter()
            .chain(com.ws.iter())
            .chain(std::iter::once(&com.z))
            .cloned(),
    );
    let W_ev = geometric(
        v,
        ev.qs
            .iter()
            .chain(ev.ws.iter())
            .chain(std::iter::once(&ev.z))
            .cloned(),
    );
    pcdl::check(&W_com, x.d, &ch, &W_ev, pi.pis.W)?;
    // W'(𝔷) = Z(ω𝔷)
    pcdl::check(&com.z, x.d, &ch_w, &ev.z_bar, pi.pis.W_bar)?;

    Ok(())
}
