#![allow(non_snake_case)]

use crate::{
    curve::Scalar,
    protocol::circuit::{CircuitPrivate, CircuitPublic},
    util::poly::{coset_scale, deg0, lagrange_basis_poly, linear_comb_poly, split_poly},
};

use super::{
    pi::{EvalProofs, Proof, ProofCommitments, ProofEvaluations},
    transcript::TranscriptProtocol,
};

use std::time::Instant;

use ark_ff::Field;
use ark_poly::Polynomial;
use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl::{self, Instance as HaloInstance},
};
use log::info;
use merlin::Transcript;
use rand::Rng;

pub fn prove<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> Proof {
    let d = x.d;
    let domain = x.h.get_domain();
    let poly_arg_times_omega =
        |poly: &PallasPoly| coset_scale(&x.h, poly, x.h.w(1).into(), &domain);

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
    let x_pl_qk = &x.pl_qk.poly;
    let x_pl_j = &x.pl_j.poly;

    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    // -------------------- Round 1 --------------------

    let now = Instant::now();
    let a_com = &pcdl::commit(&w.a.poly, d, None);
    let b_com = &pcdl::commit(&w.b.poly, d, None);
    let c_com = &pcdl::commit(&w.c.poly, d, None);
    transcript.append_points_new(b"abc", &[*a_com, *b_com, *c_com]);
    info!("Round 1 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 2 --------------------

    let now = Instant::now();
    // ζ = H(transcript)
    let zeta = &transcript.challenge_scalar_new(b"zeta");
    let plp = &w.plonkup.compute(&Scalar::new(*zeta));
    let pl_t = &plp[0];
    let pl_f = &plp[1];
    let pl_h1 = &plp[2];
    let pl_h2 = &plp[3];
    let pl_t = &pl_t.poly;
    let pl_f = &pl_f.poly;
    let pl_h1 = &pl_h1.poly;
    let pl_h2 = &pl_h2.poly;
    info!("Round 2 took {} s", now.elapsed().as_secs_f64());

    let pl_t_bar = &poly_arg_times_omega(pl_t);
    let pl_h1_bar = &poly_arg_times_omega(pl_h1);

    // -------------------- Round 3 --------------------

    let now = Instant::now();
    // β = H(transcript, 1)
    let beta = transcript.challenge_scalar_new(b"beta");
    // γ = H(transcript, 2)
    let gamma = transcript.challenge_scalar_new(b"gamma");
    // δ = H(transcript, 3)
    let delta = transcript.challenge_scalar_new(b"delta");
    // ε = H(transcript, 4)
    let epsilon = transcript.challenge_scalar_new(b"epsilon");

    // ----- Lambdas ----- //

    // plookup constraint term: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &(epsilon * (PallasScalar::ONE + delta));
    let zpl_ev = |a: &PallasPoly, b: &PallasPoly, i| {
        *zpl_sc + a.evaluate(&x.h.w(i).into()) + (delta * b.evaluate(&x.h.w(i).into()))
    };
    let zpl = |a: &PallasPoly, b: &PallasPoly| deg0(zpl_sc) + a + (deg0(&delta) * b);

    // copy constraint term: w(X) + β s(X) + γ
    let zcc_ev = |w: &PallasPoly, s: &PallasPoly, i| {
        w.evaluate(&x.h.w(i).into()) + (beta * s.evaluate(&x.h.w(i).into())) + gamma
    };
    let zcc = |w: &PallasPoly, s: &PallasPoly| w + (s * beta) + deg0(&gamma);

    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    //         (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let zf_cc = &(zcc(w_a, x_sida) * zcc(w_b, x_sidb) * zcc(w_c, x_sidc));
    let zf_pl = &(zpl(pl_f, pl_f) * zpl(pl_t, pl_t_bar));
    let zf = &(zf_cc * zf_pl);
    let zf_cc_ev = |i| zcc_ev(w_a, x_sida, i) * zcc_ev(w_b, x_sidb, i) * zcc_ev(w_c, x_sidc, i);
    let zf_pl_ev = |i| zpl_ev(pl_f, pl_f, i) * zpl_ev(pl_t, pl_t_bar, i);
    let zf_ev = |i| zf_cc_ev(i) * zf_pl_ev(i);
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zg_cc = &(zcc(w_a, x_sa) * zcc(w_b, x_sb) * zcc(w_c, x_sc));
    let zg_pl = &(zpl(pl_h1, pl_h2) * zpl(pl_h2, pl_h1_bar));
    let zg = &(zg_cc * zg_pl);
    let zg_cc_ev = |i| zcc_ev(w_a, x_sa, i) * zcc_ev(w_b, x_sb, i) * zcc_ev(w_c, x_sc, i);
    let zg_pl_ev = |i| zpl_ev(pl_h1, pl_h2, i) * zpl_ev(pl_h2, pl_h1_bar, i);
    let zg_ev = |i| zg_cc_ev(i) * zg_pl_ev(i);

    // ----- Calculate z ----- //

    // Z(ω) = 1
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    info!("Round 3 - A - {} s", now.elapsed().as_secs_f64());
    let z_points = (1..x.h.n() - 1).fold(vec![Scalar::ONE; 2], |mut acc, i| {
        acc.push(acc[i as usize] * zf_ev(i) / zg_ev(i));
        acc
    });
    info!("Round 3 - B - {} s", now.elapsed().as_secs_f64());
    let z = &x.h.interpolate(z_points);
    info!("Round 3 - C - {} s", now.elapsed().as_secs_f64());
    // Z(ω X)
    let z_bar = &poly_arg_times_omega(&z.poly);
    info!("Round 3 - D - {} s", now.elapsed().as_secs_f64());
    let z = &z.into();
    let z_com = &pcdl::commit(z, d, None);
    transcript.append_point_new(b"z", z_com);
    info!("Round 3 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 4 --------------------

    let now = Instant::now();
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar_new(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X) + PI(X)
    //         + Qₖ(X)(A(X) + ζB(X) + ζ²C(X) + ζ³J(X) - f(X))
    // info!("Round 4A - {} s", now.elapsed().as_secs_f64());
    let pl_query = linear_comb_poly(zeta, [w_a, w_b, w_c, x_pl_j]);
    let pl_f_gc = &(x_pl_qk * (pl_query - pl_f));
    // info!("Round 4B - {} s", now.elapsed().as_secs_f64());
    let f_gc =
        &((w_a * x_ql) + (w_b * x_qr) + (w_c * x_qo) + (w_a * w_b * x_qm) + x_qc + x_pip + pl_f_gc);
    // F_Z1(X) = L₁(X) (Z(X) - 1)
    // info!("Round 4C - {} s", now.elapsed().as_secs_f64());
    let f_z1: &PallasPoly = &(lagrange_basis_poly(&x.h, 1) * (z - deg0(&PallasScalar::ONE)));
    // F_Z2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    // info!("Round 4D - {} s", now.elapsed().as_secs_f64());
    let f_z2 = &((z * zf) - (zg * z_bar));
    // T(X) = (F_GC(X) + α F_C1(X) + α² F_C2(X)) / Zₕ(X)
    // info!("Round 4E1 - {} s", now.elapsed().as_secs_f64());
    let t_a = linear_comb_poly(alpha, [f_gc, f_z1, f_z2]);
    // info!("Round 4E2 - {} s", now.elapsed().as_secs_f64());
    let (t, _) = &t_a.divide_by_vanishing_poly(domain);
    // info!("Round 4E3 - {} s", now.elapsed().as_secs_f64());
    let ts = split_poly(&x.h, t);
    // info!("Round 4F - {} s", now.elapsed().as_secs_f64());
    let t_coms: Vec<PallasPoint> = ts.iter().map(|p| pcdl::commit(p, d, None)).collect();
    // info!("Round 4G - {} s", now.elapsed().as_secs_f64());

    transcript.append_points_new(b"t", &t_coms);
    // transcript.append_point_new(b"t", &t_com);
    info!("Round 4 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 5 --------------------

    let now = Instant::now();
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar_new(b"xi");
    let a_ev = &w_a.evaluate(ch);
    let b_ev = &w_b.evaluate(ch);
    let c_ev = &w_c.evaluate(ch);
    let qc_ev = &x_qc.evaluate(ch);
    let ql_ev = &x_ql.evaluate(ch);
    let qm_ev = &x_qm.evaluate(ch);
    let qo_ev = &x_qo.evaluate(ch);
    let qr_ev = &x_qr.evaluate(ch);
    let sa_ev = &x_sa.evaluate(ch);
    let sb_ev = &x_sb.evaluate(ch);
    let sc_ev = &x_sc.evaluate(ch);
    let pip_ev = &x_pip.evaluate(ch);
    let t_parts_ev = ts.iter().map(|p| p.evaluate(ch)).collect::<Vec<_>>();
    let z_ev = z.evaluate(ch);
    let pl_j_ev = x_pl_j.evaluate(ch);
    let pl_f_ev = pl_f.evaluate(ch);
    let pl_qk_ev = x_pl_qk.evaluate(ch);
    let pl_h1_ev = pl_h1.evaluate(ch);
    let pl_h2_ev = pl_h2.evaluate(ch);
    let pl_t_ev = pl_t.evaluate(ch);

    let ch_bar = &(*ch * x.h.w(1).scalar);
    let z_bar_ev = z_bar.evaluate(ch);
    let pl_h1_bar_ev = pl_h1_bar.evaluate(ch);
    let pl_t_bar_ev = pl_t_bar.evaluate(ch);

    transcript.append_scalar_new(b"a_ev", a_ev);
    transcript.append_scalar_new(b"b_ev", b_ev);
    transcript.append_scalar_new(b"c_ev", c_ev);
    transcript.append_scalar_new(b"qc_ev", qc_ev);
    transcript.append_scalar_new(b"ql_ev", ql_ev);
    transcript.append_scalar_new(b"qm_ev", qm_ev);
    transcript.append_scalar_new(b"qo_ev", qo_ev);
    transcript.append_scalar_new(b"qr_ev", qr_ev);
    transcript.append_scalar_new(b"sa_ev", sa_ev);
    transcript.append_scalar_new(b"sb_ev", sb_ev);
    transcript.append_scalar_new(b"sc_ev", sc_ev);
    transcript.append_scalar_new(b"z_bar_ev", &z_bar_ev);
    transcript.append_scalars_new(b"t_ev", &t_parts_ev.as_slice());
    transcript.append_scalar_new(b"z_ev", &z_ev);

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

    let (_, _, _, _, W_pi) = HaloInstance::open(rng, W, d, ch, None).into_tuple();

    //let W_bar: Poly = z_bar + v.pow(1) * pl_h1_bar + v.pow(2) * pl_t_bar;
    let W_bar = z.clone();
    let (_, _, _, _, W_bar_pi) = HaloInstance::open(rng, W_bar, d, ch_bar, None).into_tuple();

    // let hdrs = vec![
    //     "t".to_string(),
    //     "t_bar".to_string(),
    //     "f".to_string(),
    //     "h1".to_string(),
    //     "h1_bar".to_string(),
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
    //         vec![
    //             &plp[0],
    //             &pl_t_bar.into(),
    //             &plp[1],
    //             &plp[2],
    //             &pl_h1_bar.into(),
    //             &plp[3],
    //             &z.into(),
    //             &z_bar.into(),
    //             &f_gc.into(),
    //             &f_z1.into(),
    //             &f_z2.into()
    //         ],
    //         hdrs,
    //         vec![false; 11]
    //     )
    // );

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
            pip: *pip_ev,
            z: z_ev,
            t_parts: t_parts_ev,
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
            t_coms,
        },
        pis: EvalProofs {
            W: W_pi,
            W_bar: W_bar_pi,
        },
    };

    info!("Round 5 took {} s", now.elapsed().as_secs_f64());

    pi
}
