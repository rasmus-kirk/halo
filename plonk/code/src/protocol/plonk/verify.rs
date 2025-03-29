use crate::{curve::Scalar, protocol::circuit::CircuitPublic, util::poly::lagrange_basis1_ev};

use super::{transcript::TranscriptProtocol, Proof};

use anyhow::{ensure, Result};
use ark_ff::{Field, Zero};
use ark_poly::Polynomial;
use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl,
};
use merlin::Transcript;

pub fn verify(x: &CircuitPublic, pi: Proof) -> Result<()> {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    let d = x.d;

    // -------------------- Round 1 --------------------

    transcript.append_points_new(b"abc", &[pi.com.a, pi.com.b, pi.com.c]);

    // -------------------- Round 2 --------------------

    let zeta = &transcript.challenge_scalar_new(b"zeta");

    // -------------------- Round 3 --------------------

    // β = H(transcript, 1)
    let beta = &transcript.challenge_scalar_new(b"beta");
    // γ = H(transcript, 2)
    let gamma = &transcript.challenge_scalar_new(b"gamma");
    // δ = H(transcript, 3)
    let delta = &transcript.challenge_scalar_new(b"delta");
    // ε = H(transcript, 4)
    let epsilon = &transcript.challenge_scalar_new(b"epsilon");
    transcript.append_point_new(b"z", &pi.com.z);

    // -------------------- Round 4 --------------------

    let alpha = &transcript.challenge_scalar_new(b"alpha");
    transcript.append_points_new(b"t", &pi.com.t_coms);

    // -------------------- Round 5 --------------------

    let ch = &transcript.challenge_scalar_new(b"xi");
    let ch_w = ch * &x.h.w(1).scalar;
    let sida_ev = &PallasPoly::from(x.sida.clone()).evaluate(ch);
    let sidb_ev = &PallasPoly::from(x.sidb.clone()).evaluate(ch);
    let sidc_ev = &PallasPoly::from(x.sidc.clone()).evaluate(ch);
    let sa_ev = &pi.ev.sa;
    let sb_ev = &pi.ev.sb;
    let sc_ev = &pi.ev.sc;
    let zh_ev = ch.pow([x.h.n()]) - PallasScalar::ONE;
    let l1_ev_ch = lagrange_basis1_ev(&x.h, ch);
    // get / compute evaluations on challenge
    let a = &pi.ev.a;
    let b = &pi.ev.b;
    let c = &pi.ev.c;
    let ql = &pi.ev.ql;
    let qr = &pi.ev.qr;
    let qo = &pi.ev.qo;
    let qm = &pi.ev.qm;
    let qc = &pi.ev.qc;
    let qk = &pi.ev.pl_qk;
    let z_bar = &pi.ev.z_bar;
    let t_evs = &pi.ev.t_parts;
    let z = &pi.ev.z;
    let j = &pi.ev.pl_j;
    let pip = &pi.ev.pip;

    transcript.append_scalar(b"a_ev", &a.into());
    transcript.append_scalar(b"b_ev", &b.into());
    transcript.append_scalar(b"c_ev", &c.into());
    transcript.append_scalar(b"qc_ev", &qc.into());
    transcript.append_scalar(b"ql_ev", &ql.into());
    transcript.append_scalar(b"qm_ev", &qm.into());
    transcript.append_scalar(b"qo_ev", &qo.into());
    transcript.append_scalar(b"qr_ev", &qr.into());
    transcript.append_scalar(b"sa_ev", &sa_ev.into());
    transcript.append_scalar(b"sb_ev", &sb_ev.into());
    transcript.append_scalar(b"sc_ev", &sc_ev.into());
    transcript.append_scalar(b"z_bar_ev", &z_bar.into());
    transcript.append_scalars_new(b"t_ev", &t_evs);
    transcript.append_scalar(b"z_ev", &z.into());

    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    //         + Qₖ(𝔷)(A(𝔷) + ζB(𝔷) + ζ²C(𝔷) + ζ³J(𝔷) - f(𝔷))
    let f_gcpl_ev =
        &(*qk * (*a + (zeta * b) + (zeta.pow([2]) * c) + (zeta.pow([3]) * j) - pi.ev.pl_f));
    let f_gc_ev = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + pip + f_gcpl_ev);
    // if *f_gc_ev == Scalar::ZERO || !pi.q_fgc.check(ch, Some(f_gc_ev)) {
    //     println!("FAILED GC");
    //     panic!();
    // }
    // F_Z1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_z1_ev = &(l1_ev_ch * (pi.ev.z - Scalar::ONE));
    // if !pi.q_fz1.check(ch, Some(f_z1_ev)) {
    //     println!("FAILED CC1");
    //     panic!();
    // }
    let zpl_sc = &(epsilon * (Scalar::ONE + delta));
    let zcc = |w: &PallasScalar, s: &PallasScalar| *w + (beta * s) + gamma;
    let zpl = |a: &PallasScalar, b: &PallasScalar| zpl_sc + a + (delta * b);
    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    //         (ε(1 + δ) + f(X) + δf(X))(ε(1 + δ) + t(X) + δt(Xω))
    let zfcc_ev = &(zcc(a, sida_ev) * zcc(b, sidb_ev) * zcc(c, sidc_ev));
    let zfpl_ev = &(zpl(&pi.ev.pl_f, &pi.ev.pl_f) * zpl(&pi.ev.pl_t, &pi.ev.pl_t_bar));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X))(ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgcc_ev = &(zcc(a, sa_ev) * zcc(b, sb_ev) * zcc(c, sc_ev));
    let zgpl_ev = &(zpl(&pi.ev.pl_h1, &pi.ev.pl_h2) * zpl(&pi.ev.pl_h2, &pi.ev.pl_h1_bar));
    // F_Z2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_z2_ev = &((pi.ev.z * zfcc_ev * zfpl_ev) - (zgcc_ev * zgpl_ev * pi.ev.z_bar));
    // if !pi.q_fz2.check(ch, Some(f_z2_ev)) {
    //     println!("FAILED CC2");
    //     panic!();
    // }

    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    let n = x.h.n();
    let t_ev = t_evs
        .iter()
        .enumerate()
        .fold(PallasScalar::zero(), |acc, (i, t)| {
            acc + (*t * ch.pow([n * i as u64]))
        });
    ensure!(
        (f_gc_ev + (alpha * f_z1_ev) + (alpha.pow([2]) * f_z2_ev)) - (t_ev * zh_ev) == Scalar::ZERO,
        "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)"
    );

    let qc_com = x.qc_com;
    let ql_com = x.ql_com;
    let qm_com = x.qm_com;
    let qo_com = x.qo_com;
    let qr_com = x.qr_com;
    let sa_com = x.sa_com;
    let sb_com = x.sb_com;
    let sc_com = x.sc_com;
    let pl_j_com = x.pl_j_com;
    let pl_qk_com = x.pl_qk_com;

    let v = &transcript.challenge_scalar_new(b"v");

    let W_com: PallasPoint = ql_com
        + qr_com * v.pow([1])
        + qo_com * v.pow([2])
        + qc_com * v.pow([3])
        + qm_com * v.pow([4])
        + sa_com * v.pow([5])
        + sb_com * v.pow([6])
        + sc_com * v.pow([7])
        + pi.com.a * v.pow([8])
        + pi.com.b * v.pow([9])
        + pi.com.c * v.pow([10])
        + pi.com.z * v.pow([11])
        + pl_j_com * v.pow([12])
        + pl_qk_com * v.pow([13]);

    let W_ev: PallasScalar = pi.ev.ql
        + pi.ev.qr * v.pow([1])
        + pi.ev.qo * v.pow([2])
        + pi.ev.qc * v.pow([3])
        + pi.ev.qm * v.pow([4])
        + pi.ev.sa * v.pow([5])
        + pi.ev.sb * v.pow([6])
        + pi.ev.sc * v.pow([7])
        + pi.ev.a * v.pow([8])
        + pi.ev.b * v.pow([9])
        + pi.ev.c * v.pow([10])
        + pi.ev.z * v.pow([11])
        + pi.ev.pl_j * v.pow([12])
        + pi.ev.pl_qk * v.pow([13]);

    pcdl::check(&W_com, d, ch, &W_ev, pi.pis.W)?;
    pcdl::check(&pi.com.z, d, &ch_w, &pi.ev.z_bar, pi.pis.W_bar)?;

    Ok(())
}
