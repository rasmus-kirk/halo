#![allow(non_snake_case)]

use anyhow::{ensure, Result};
use ark_ff::Field;
use ark_poly::{DenseUVPolynomial, Polynomial};
use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl::{self, EvalProof, Instance as HaloInstance},
};
use log::trace;
use merlin::Transcript;
use rand::Rng;

use super::{instance::Instance, transcript::TranscriptProtocol, SNARKProof};
use crate::{
    curve::Scalar,
    protocol::circuit::{CircuitPrivate, CircuitPublic},
};

#[derive(Clone)]
pub struct ProofEvaluations {
    a: PallasScalar,
    b: PallasScalar,
    c: PallasScalar,
    qc: PallasScalar,
    ql: PallasScalar,
    qm: PallasScalar,
    qo: PallasScalar,
    qr: PallasScalar,
    sa: PallasScalar,
    sb: PallasScalar,
    sc: PallasScalar,
    z: PallasScalar,
    t: PallasScalar,
    pl_j: PallasScalar,
    pl_qk: PallasScalar,
    pl_f: PallasScalar,
    pl_t: PallasScalar,
    pl_h1: PallasScalar,
    pl_h2: PallasScalar,
    z_bar: PallasScalar,
    pl_t_bar: PallasScalar,
    pl_h1_bar: PallasScalar,
}

#[derive(Clone)]
pub struct ProofCommitments {
    a: PallasPoint,
    b: PallasPoint,
    c: PallasPoint,
    z: PallasPoint,
    t: PallasPoint,
}

#[derive(Clone)]
pub struct EvalProofs {
    W: EvalProof,
    W_bar: EvalProof,
}

#[derive(Clone)]
pub struct Proof {
    ev: ProofEvaluations,
    com: ProofCommitments,
    pis: EvalProofs,
}

pub fn proof<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> SNARKProof {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // -------------------- Round 1 --------------------

    let com_a = &w.a.commit();
    let com_b = &w.b.commit();
    let com_c = &w.c.commit();
    transcript.append_points(b"abc", &[*com_a, *com_b, *com_c]);
    // Round 2 -----------------------------------------------------
    // ζ = H(transcript)
    let zeta = &transcript.challenge_scalar(b"zeta");
    let plp = &w.plonkup.compute(zeta);
    let tpl = &plp[0];
    let fpl = &plp[1];
    let h1pl = &plp[2];
    let h2pl = &plp[3];
    let tplbar = &x.h.poly_times_arg(tpl, &x.h.w(1));
    let h1plbar = &x.h.poly_times_arg(h1pl, &x.h.w(1));
    // Round 3 -----------------------------------------------------
    // β = H(transcript, 1)
    let beta = &transcript.challenge_scalar_augment(1, b"beta");
    // γ = H(transcript, 2)
    let gamma = &transcript.challenge_scalar_augment(2, b"gamma");
    // δ = H(transcript, 3)
    let delta = &transcript.challenge_scalar_augment(3, b"delta");
    // ε = H(transcript, 4)
    let epsilon = &transcript.challenge_scalar_augment(4, b"epsilon");
    // copy constraints: w(X) + β s(X) + γ
    let zcc_ev = |w, s, i| x.h.evaluate(w, i) + beta * x.h.evaluate(s, i) + gamma;
    let zcc = |w, s| w + beta * s + gamma;
    // plookup constraints: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &(epsilon * (Scalar::ONE + delta));
    let zpl_ev = |a, b, i| zpl_sc + x.h.evaluate(a, i) + delta * x.h.evaluate(b, i);
    let zpl = |a, b| zpl_sc + a + delta * b;
    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    //         (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let zfcc_ev =
        |i| zcc_ev(&w.a, &x.sida, i) * zcc_ev(&w.b, &x.sidb, i) * zcc_ev(&w.c, &x.sidc, i);
    let zfpl_ev = |i| zpl_ev(fpl, fpl, i) * zpl_ev(tpl, tplbar, i);
    let zf = &(zcc(&w.a, &x.sida)
        * zcc(&w.b, &x.sidb)
        * zcc(&w.c, &x.sidc)
        * zpl(fpl, fpl)
        * zpl(tpl, tplbar));
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgcc_ev = |i| zcc_ev(&w.a, &x.sa, i) * zcc_ev(&w.b, &x.sb, i) * zcc_ev(&w.c, &x.sc, i);
    let zgpl_ev = |i| zpl_ev(h1pl, h2pl, i) * zpl_ev(h2pl, h1plbar, i);
    let zg = &(zcc(&w.a, &x.sa)
        * zcc(&w.b, &x.sb)
        * zcc(&w.c, &x.sc)
        * zpl(h1pl, h2pl)
        * zpl(h2pl, h1plbar));
    // Z(ω) = 1
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    let z_points = (1..x.h.n() - 1).fold(vec![Scalar::ONE; 2], |mut acc, i| {
        acc.push(acc[i as usize] * zfcc_ev(i) * zfpl_ev(i) / (zgcc_ev(i) * zgpl_ev(i)));
        acc
    });
    let z = &x.h.interpolate(z_points);
    // Z(ω X)
    let zbar = &x.h.poly_times_arg(z, &x.h.w(1));
    let comm_z = &z.commit();
    transcript.append_point(b"z", comm_z);
    // Round 4 -----------------------------------------------------
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X) + PI(X)
    //         + Qₖ(X)(A(X) + ζB(X) + ζ²C(X) + ζ³J(X) - f(X))
    let f_plgc =
        &(&x.pl_qk * (&w.a + (zeta * &w.b) + (zeta.pow(2) * &w.c) + (zeta.pow(3) * &x.pl_j) - fpl));
    let f_gc = &((&w.a * &x.ql)
        + (&w.b * &x.qr)
        + (&w.c * &x.qo)
        + (&w.a * &w.b * &x.qm)
        + &x.qc
        + &x.pip
        + f_plgc);
    // F_Z1(X) = L₁(X) (Z(X) - 1)
    let f_z1 = &(x.h.lagrange(1) * (z - Scalar::ONE));
    // F_Z2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    let f_z2 = &((z * zf) - (zg * zbar));
    // T(X) = (F_GC(X) + α F_C1(X) + α² F_C2(X)) / Zₕ(X)
    let t = &((f_gc + alpha * f_z1 + alpha.pow(2) * f_z2) / x.h.zh());
    let comm_t = &t.commit();
    transcript.append_point(b"t", comm_t);
    // Round 5 -----------------------------------------------------
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");

    let q_a = Instance::new(rng, &w.a, ch, true);
    let q_b = Instance::new(rng, &w.b, ch, true);
    let q_c = Instance::new(rng, &w.c, ch, true);
    let q_fgc = Instance::new(rng, f_gc, ch, false);
    let q_z = Instance::new_from_comm(rng, z, ch, comm_z, true);
    let zbar_ev = zbar.evaluate(ch);
    let q_fz1 = Instance::new(rng, f_z1, ch, false);
    let q_fz2 = Instance::new(rng, f_z2, ch, false);
    let fpl_ev = fpl.evaluate(ch);
    let q_tpl = Instance::new(rng, tpl, ch, true);
    let tplbar_ev = tplbar.evaluate(ch);
    let q_h1 = Instance::new(rng, h1pl, ch, true);
    let q_h2 = Instance::new(rng, h2pl, ch, true);
    let h1plbar_ev = h1plbar.evaluate(ch);
    let q_t = Instance::new_from_comm(rng, t, ch, comm_t, true);

    // let hdrs = vec![
    //     "t".to_string(),
    //     "f".to_string(),
    //     "h1".to_string(),
    //     "h2".to_string(),
    //     "Z(X)".to_string(),
    //     "Z(ωX)".to_string(),
    //     "F_GC(X)".to_string(),
    //     "F_Z1(X)".to_string(),
    //     "F_Z2(X)".to_string(),
    // ];
    // println!(
    //     "{}",
    //     x.h.evals_str(
    //         vec![tpl, fpl, h1pl, h2pl, z, zbar, f_gc, f_z1, f_z2],
    //         hdrs,
    //         vec![false; 9]
    //     )
    // );
    SNARKProof {
        q_a,
        q_b,
        q_c,
        q_fgc,
        q_z,
        zbar_ev,
        q_fz1,
        q_fz2,
        q_t,
        q_tpl,
        tplbar_ev,
        fpl_ev,
        q_h1,
        q_h2,
        h1plbar_ev,
    }
}

pub fn prove_w_lu<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> Proof {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    let d = x.d;

    let w_a = &w.a.poly;
    let w_b = &w.b.poly;
    let w_c = &w.c.poly;
    let x_sida = &x.sida.poly;
    let x_sidb = &x.sidb.poly;
    let x_sidc = &x.sidc.poly;
    let x_sa = &x.sa.poly;
    let x_sb = &x.sb.poly;
    let x_sc = &x.sc.poly;
    let x_qc = &x.qc.poly;
    let x_ql = &x.ql.poly;
    let x_qm = &x.qm.poly;
    let x_qo = &x.qo.poly;
    let x_qr = &x.qr.poly;
    let x_pip = &x.pip.poly;
    let x_zh = &x.h.zh();
    let x_pl_qk = &x.pl_qk.poly;
    let x_pl_j = &x.pl_j.poly;

    // -------------------- Round 1 --------------------

    let a_com = &pcdl::commit(&w.a.poly, d, None);
    let b_com = &pcdl::commit(&w.b.poly, d, None);
    let c_com = &pcdl::commit(&w.c.poly, d, None);
    transcript.append_points_new(b"abc", &[*a_com, *b_com, *c_com]);

    // -------------------- Round 2 --------------------

    // ζ = H(transcript)
    let zeta = &transcript.challenge_scalar_new(b"zeta");
    let plp = &w.plonkup.compute(&Scalar::new(*zeta));
    let pl_t = &plp[0];
    let pl_f = &plp[1];
    let pl_h1 = &plp[2];
    let pl_h2 = &plp[3];
    let pl_t_bar = &x.h.poly_times_arg(pl_t, &x.h.w(1)).poly;
    let pl_h1_bar = &x.h.poly_times_arg(pl_h1, &x.h.w(1)).poly;
    let pl_t = &pl_t.poly;
    let pl_f = &pl_f.poly;
    let pl_h1 = &pl_h1.poly;
    let pl_h2 = &pl_h2.poly;

    // -------------------- Round 3 --------------------

    // NOTICE: The `1..4` is omitted since the label handles it.
    // β = H(transcript, 1)
    let beta = transcript.challenge_scalar_new(b"beta");
    // γ = H(transcript, 2)
    let gamma = transcript.challenge_scalar_new(b"gamma");
    // δ = H(transcript, 3)
    let delta = transcript.challenge_scalar_new(b"delta");
    // ε = H(transcript, 4)
    let epsilon = transcript.challenge_scalar_new(b"epsilon");

    // ----- Lambdas ----- //

    let deg0 = |scalar: &PallasScalar| PallasPoly::from_coefficients_slice(&[*scalar]);

    // plookup constraints: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &(epsilon * (PallasScalar::ONE + delta));
    let zpl_ev = |a: &PallasPoly, b: &PallasPoly, i| {
        *zpl_sc + a.evaluate(&x.h.w(i).into()) + delta * b.evaluate(&x.h.w(i).into())
    };
    let zpl = |a: &PallasPoly, b: &PallasPoly| deg0(zpl_sc) + a + deg0(&delta) * b;

    // copy constraints: w(X) + β s(X) + γ
    let zcc_ev = |w: &PallasPoly, s: &PallasPoly, i| {
        w.evaluate(&x.h.w(i).into()) + beta * s.evaluate(&x.h.w(i).into()) + gamma
    };
    // w + s * beta + gamma
    let zcc = |w: &PallasPoly, s: &PallasPoly| w + s * beta + deg0(&gamma);

    let cc_zf_ev = |i| zcc_ev(w_a, x_sida, i) * zcc_ev(w_b, x_sidb, i) * zcc_ev(w_c, x_sidc, i);

    // plookup constraints: ε(1 + δ) + a(X) + δb(X)
    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    //         (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let pl_zf_ev = |i| zpl_ev(pl_f, pl_f, i) * zpl_ev(pl_t, pl_t_bar, i);
    let zf = &(zcc(w_a, x_sida)
        * zcc(w_b, x_sidb)
        * zcc(w_c, x_sidc)
        * zpl(pl_f, pl_f)
        * zpl(pl_t, pl_t_bar));
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let cc_zg_ev = |i| zcc_ev(w_a, x_sa, i) * zcc_ev(w_b, x_sb, i) * zcc_ev(w_c, x_sc, i);
    let pl_zg_ev = |i| zpl_ev(pl_h1, pl_h2, i) * zpl_ev(pl_h2, pl_h1_bar, i);

    // ----- Calculate z ----- //

    let zg = &(zcc(w_a, x_sa)
        * zcc(w_b, x_sb)
        * zcc(w_c, x_sc)
        * zpl(pl_h1, pl_h2)
        * zpl(pl_h2, pl_h1_bar));
    // Z(ω) = 1
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    let z_points = (1..x.h.n() - 1).fold(vec![Scalar::ONE; 2], |mut acc, i| {
        acc.push(acc[i as usize] * cc_zf_ev(i) * pl_zf_ev(i) / (cc_zg_ev(i) * pl_zg_ev(i)));
        acc
    });
    let z = &x.h.interpolate(z_points);
    // Z(ω X)
    let z_bar = &x.h.poly_times_arg(z, &x.h.w(1));
    let z = &z.into();
    let z_com = &pcdl::commit(&z, d, None);
    transcript.append_point_new(b"z", &z_com);

    // -------------------- Round 4 --------------------

    // α = H(transcript)
    let alpha = &transcript.challenge_scalar_new(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X) + PI(X)
    //         + Qₖ(X)(A(X) + ζB(X) + ζ²C(X) + ζ³J(X) - f(X))
    let pl_f_gc = &(x_pl_qk
        * (w_a
            + (deg0(zeta) * w_b)
            + (deg0(&zeta.pow([2])) * w_c)
            + (deg0(&zeta.pow([3])) * x_pl_j)
            - pl_f));
    let f_gc =
        &((w_a * x_ql) + (w_b * x_qr) + (w_c * x_qo) + (w_a * w_b * x_qm) + x_qc + x_pip + pl_f_gc);
    // F_Z1(X) = L₁(X) (Z(X) - 1)
    let f_z1: &PallasPoly = &(x.h.lagrange(1).poly * (z - deg0(&PallasScalar::ONE)));
    // F_Z2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    let f_z2 = &((z * zf) - (zg * z_bar));
    // T(X) = (F_GC(X) + α F_C1(X) + α² F_C2(X)) / Zₕ(X)
    let t = &((f_gc + deg0(alpha) * f_z1 + deg0(&alpha.pow([2])) * f_z2) / x_zh);
    trace!("t1");
    let t_com = pcdl::commit(&t, d, None);
    transcript.append_point_new(b"t", &t_com);
    trace!("t1");

    // -------------------- Round 5 --------------------

    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar_new(b"xi");
    let a_ev = &w.a.poly.evaluate(ch);
    let b_ev = &w.b.poly.evaluate(ch);
    let c_ev = &w.c.poly.evaluate(ch);
    let qc_ev = &x.qc.poly.evaluate(ch);
    let ql_ev = &x.ql.poly.evaluate(ch);
    let qm_ev = &x.qm.poly.evaluate(ch);
    let qo_ev = &x.qo.poly.evaluate(ch);
    let qr_ev = &x.qr.poly.evaluate(ch);
    let sa_ev = &x.sa.poly.evaluate(ch);
    let sb_ev = &x.sb.poly.evaluate(ch);
    let sc_ev = &x.sc.poly.evaluate(ch);
    let t_ev = t.evaluate(ch);
    let z_ev = z.evaluate(ch);
    let pl_j_ev = x.pl_j.poly.evaluate(ch);
    let pl_f_ev = pl_f.evaluate(ch);
    let pl_qk_ev = x.pl_qk.poly.evaluate(ch);
    let pl_h1_ev = pl_h1.evaluate(ch);
    let pl_h2_ev = pl_h2.evaluate(ch);
    let pl_t_ev = pl_t.evaluate(ch);

    let ch_bar = &(*ch * x.h.w(1).scalar);
    let z_bar_ev = z_bar.poly.evaluate(ch);
    let pl_h1_bar_ev = pl_h1_bar.evaluate(ch);
    //let pl_h1_bar_ev = pl_h1.evaluate(ch_bar);
    let pl_t_bar_ev = pl_t_bar.evaluate(ch);
    //let pl_t_bar_ev = pl_h1.evaluate(ch_bar);

    transcript.append_scalar_new(b"a_ev", &a_ev);
    transcript.append_scalar_new(b"b_ev", &b_ev);
    transcript.append_scalar_new(b"c_ev", &c_ev);
    transcript.append_scalar_new(b"qc_ev", &qc_ev);
    transcript.append_scalar_new(b"ql_ev", &ql_ev);
    transcript.append_scalar_new(b"qm_ev", &qm_ev);
    transcript.append_scalar_new(b"qo_ev", &qo_ev);
    transcript.append_scalar_new(b"qr_ev", &qr_ev);
    transcript.append_scalar_new(b"sa_ev", &sa_ev);
    transcript.append_scalar_new(b"sb_ev", &sb_ev);
    transcript.append_scalar_new(b"sc_ev", &sc_ev);
    transcript.append_scalar_new(b"z_bar_ev", &z_bar_ev.into());
    transcript.append_scalar_new(b"t_ev", &t_ev.into());
    transcript.append_scalar_new(b"z_ev", &z_ev.into());

    let v = &transcript.challenge_scalar_new(b"v");

    let W = x_ql
        + x_qr * deg0(&v.pow([1]))
        + x_qo * deg0(&v.pow([2]))
        + x_qc * deg0(&v.pow([3]))
        + x_qm * deg0(&v.pow([4]))
        + x_sa * deg0(&v.pow([5]))
        + x_sb * deg0(&v.pow([6]))
        + x_sc * deg0(&v.pow([7]))
        + w_a * deg0(&v.pow([8]))
        + w_b * deg0(&v.pow([9]))
        + w_c * deg0(&v.pow([10]))
        + z * deg0(&v.pow([11]))
        + x_pl_j * deg0(&v.pow([12]))
        + x_pl_qk * deg0(&v.pow([13]));
    // WARNING: Possible soundness issue
    // + v.pow(13) * pl_f
    // + v.pow(15) * pl_h1
    // + v.pow(16) * pl_h2
    // + v.pow(17) * pl_t;
    let (_, _, _, _, W_pi) = HaloInstance::open(rng, W, d as usize, &ch, None).into_tuple();

    //let W_bar: Poly = z_bar + v.pow(1) * pl_h1_bar + v.pow(2) * pl_t_bar;
    let W_bar = z.clone();
    let (_, _, _, _, W_bar_pi) =
        HaloInstance::open(rng, W_bar, d as usize, &ch_bar, None).into_tuple();

    let pi = Proof {
        ev: ProofEvaluations {
            a: *a_ev,
            b: *b_ev,
            c: *c_ev,
            qc: *qc_ev,
            ql: *ql_ev,
            qm: *qm_ev,
            qo: *qo_ev,
            qr: *qr_ev,
            sa: *sa_ev,
            sb: *sb_ev,
            sc: *sc_ev,
            z: z_ev,
            t: t_ev,
            pl_j: pl_j_ev,
            pl_f: pl_f_ev,
            pl_qk: pl_qk_ev,
            pl_h1: pl_h1_ev,
            pl_h2: pl_h2_ev,
            pl_t: pl_t_ev,
            z_bar: z_bar_ev,
            pl_h1_bar: pl_h1_bar_ev,
            pl_t_bar: pl_t_bar_ev,
        },
        com: ProofCommitments {
            a: *a_com,
            b: *b_com,
            c: *c_com,
            z: *z_com,
            t: t_com,
        },
        pis: EvalProofs {
            W: W_pi,
            W_bar: W_bar_pi,
        },
    };

    pi
}

pub fn verify_lu_with_w(x: &CircuitPublic, pi: Proof) -> Result<()> {
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
    transcript.append_point_new(b"z", &pi.com.z.into());

    // -------------------- Round 4 --------------------

    let alpha = &transcript.challenge_scalar_new(b"alpha");
    transcript.append_point_new(b"t", &pi.com.t);

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
    let l1_ev_ch = x.h.l1_ev(&ch.into());
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
    let t = &pi.ev.t;
    let z = &pi.ev.z;
    let j = &pi.ev.pl_j;
    let pip = PallasPoly::from(x.pip.clone()).evaluate(ch);

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
    transcript.append_scalar(b"t_ev", &t.into());
    transcript.append_scalar(b"z_ev", &z.into());

    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    //         + Qₖ(𝔷)(A(𝔷) + ζB(𝔷) + ζ²C(𝔷) + ζ³J(𝔷) - f(𝔷))
    let f_gcpl_ev = &(*qk * (*a + zeta * b + zeta.pow([2]) * c + zeta.pow([2]) * j - pi.ev.pl_f));
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
    let zcc = |w: &PallasScalar, s: &PallasScalar| *w + beta * s + gamma;
    let zpl = |a: &PallasScalar, b: &PallasScalar| zpl_sc + a + delta * b;
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
    ensure!(
        (f_gc_ev + (alpha * f_z1_ev) + (alpha.pow([2]) * f_z2_ev)) - (pi.ev.t * zh_ev)
            == Scalar::ZERO,
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
