#![allow(non_snake_case)]

use super::{transcript::TranscriptProtocol, Proof};
use crate::{
    circuit::CircuitPublic,
    pcs::PCS,
    scheme::eqns::{self, EqnsF},
    utils::{poly, scalar, Scalar},
};

use anyhow::{ensure, Result};
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use merlin::Transcript;

pub fn verify<P: SWCurveConfig, PCST: PCS<P>>(
    x: &CircuitPublic<P>,
    pi: Proof<P, PCST>,
) -> Result<()>
where
    Transcript: TranscriptProtocol<P>,
{
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
    let beta = transcript.challenge_scalar(b"beta");
    // γ = H(transcript)
    let gamma = transcript.challenge_scalar(b"gamma");
    // δ = H(transcript)
    let delta = transcript.challenge_scalar(b"delta");
    // ε = H(transcript)
    let epsilon = transcript.challenge_scalar(b"epsilon");
    transcript.append_point(b"z", &com.z);

    // -------------------- Round 4 --------------------

    let alpha = transcript.challenge_scalar(b"alpha");
    transcript.append_points(b"t", &com.ts);

    // -------------------- Round 5 --------------------

    let ch = transcript.challenge_scalar(b"xi");
    let ch_w = ch * x.h.w(1);
    let zh_ev = scalar::zh_ev::<P>(x.h.n(), ch);
    let [ia, ib, ic] = poly::batch_evaluate::<P, _>(&x.is, ch).try_into().unwrap();

    transcript.append_scalars(b"ws_ev", &ev.ws);
    transcript.append_scalars(b"qs_ev", &ev.qs);
    transcript.append_scalars(b"ss_ev", &ev.ps);
    transcript.append_scalars(b"plonkup_ev", &ev.pls);
    transcript.append_scalar(b"z_bar_ev", &ev.z_bar);
    transcript.append_scalars(b"t_ev", &ev.ts);
    transcript.append_scalar(b"z_ev", &ev.z);

    // a + βb + γ
    let cc = eqns::copy_constraint_term(Into::into, beta, gamma);
    // ε(1 + δ) + a + δb
    let pl = eqns::plookup_term(Into::into, epsilon * (Scalar::<P>::ONE + delta), delta);
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    //         (ε(1 + δ) + f(𝔷) + δf(𝔷))(ε(1 + δ) + t(𝔷) + δt(Xω))
    let zf_ev = cc(ev.a(), ia)
        * cc(ev.b(), ib)
        * cc(ev.c(), ic)
        * pl(ev.f(), ev.f())
        * pl(ev.t(), ev.t_bar);
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    //         (ε(1 + δ) + h₁(𝔷) + δh₂(𝔷))(ε(1 + δ) + h₂(𝔷) + δh₁(Xω))
    let zg_ev = cc(ev.a(), ev.pa())
        * cc(ev.b(), ev.pb())
        * cc(ev.c(), ev.pc())
        * pl(ev.h1(), ev.h2())
        * pl(ev.h2(), ev.h1_bar);

    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷) + PI(𝔷)
    //         + Qₖ(𝔷)(A(𝔷) + ζB(𝔷) + ζ²C(𝔷) + ζ³J(𝔷) - f(𝔷))
    let f_gc_ev = EqnsF::<P>::plonkup_eqn(zeta, ev.ws.clone(), ev.qs.clone(), ev.pip, ev.f());
    // F_Z1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_z1_ev =
        EqnsF::<P>::grand_product1(ev.z, scalar::lagrange_basis1::<P>(x.h.n(), x.h.w(1), ch));
    // F_Z2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_z2_ev = eqns::grand_product2(ev.z, zf_ev, zg_ev, ev.z_bar);

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    let t_ev = EqnsF::<P>::geometric_fp(ch.pow([x.h.n()]), ev.ts.clone());
    ensure!(
        EqnsF::<P>::geometric_fp(alpha, [f_gc_ev, f_z1_ev, f_z2_ev]) == t_ev * zh_ev,
        "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)"
    );

    let v = transcript.challenge_scalar(b"v");

    // W(𝔷) = Qₗ(𝔷) + vQᵣ(𝔷) + v²Qₒ(𝔷) + v³Qₘ(𝔷) + v⁴Q꜀(𝔷) + v⁵Qₖ(𝔷) + v⁶J(𝔷)
    //      + v⁷A(𝔷) + v⁸B(𝔷) + v⁹C(𝔷) + v¹⁰Z(𝔷)
    let W_com = EqnsF::<P>::flat_geometric_fp(v, [x.qs_com.clone(), com.ws.clone(), vec![com.z]]);
    let W_ev = EqnsF::<P>::flat_geometric_fp(v, [ev.qs.clone(), ev.ws.clone(), vec![ev.z]]);
    PCST::check(&W_com, x.d, &ch, &W_ev, pi.pis.W)?;
    // W'(𝔷) = Z(ω𝔷)
    PCST::check(&com.z, x.d, &ch_w, &ev.z_bar, pi.pis.W_bar)?;

    Ok(())
}
