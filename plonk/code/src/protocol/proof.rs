#![allow(non_snake_case)]

use super::{
    pi::{EvalProofs, Proof, ProofCommitments, ProofEvaluations},
    transcript::TranscriptProtocol,
};
use crate::{
    circuit::{CircuitPrivate, CircuitPublic},
    scheme::{Selectors, Slots},
    utils::{
        misc::batch_op,
        poly::{
            batch_commit, batch_evaluate, coset_scale_omega_evals, deg0, lagrange_basis_poly,
            linear_comb_poly, split_poly,
        },
        print_table::evals_str,
    },
};

use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl::{self, Instance},
};

use ark_ff::Field;
use ark_poly::{Evaluations, Polynomial};
use log::{debug, info};
use merlin::Transcript;
use rand::Rng;
use std::time::Instant;

type Scalar = PallasScalar;
type Poly = PallasPoly;

pub fn prove<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> Proof {
    let d = x.d;

    let w_a = &w.ws[Slots::A as usize];
    let w_b = &w.ws[Slots::B as usize];
    let w_c = &w.ws[Slots::C as usize];
    let _a = &w.ws_cache[Slots::A as usize];
    let _b = &w.ws_cache[Slots::B as usize];
    let _c = &w.ws_cache[Slots::C as usize];
    let x_ql = &x.qs[Selectors::Ql as usize];
    let x_qr = &x.qs[Selectors::Qr as usize];
    let x_qo = &x.qs[Selectors::Qo as usize];
    let x_qm = &x.qs[Selectors::Qm as usize];
    let x_qc = &x.qs[Selectors::Qc as usize];
    let x_qk = &x.qs[Selectors::Qk as usize];
    let x_j = &x.qs[Selectors::J as usize];
    let x_sida = &x.sids[Slots::A as usize];
    let x_sidb = &x.sids[Slots::B as usize];
    let x_sidc = &x.sids[Slots::C as usize];
    let _sida = &x.sids_cache[Slots::A as usize];
    let _sidb = &x.sids_cache[Slots::B as usize];
    let _sidc = &x.sids_cache[Slots::C as usize];
    let x_sa = &x.ss[Slots::A as usize];
    let x_sb = &x.ss[Slots::B as usize];
    let x_sc = &x.ss[Slots::C as usize];
    let _sa = &x.ss_cache[Slots::A as usize];
    let _sb = &x.ss_cache[Slots::B as usize];
    let _sc = &x.ss_cache[Slots::C as usize];
    let x_pip = &x.pip;

    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    // -------------------- Round 1 --------------------

    let now = Instant::now();
    let abc_coms = batch_commit(&w.ws, d, None);
    transcript.append_points(b"abc", &abc_coms);
    info!("Round 1 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 2 --------------------

    let now = Instant::now();
    // ζ = H(transcript)
    let zeta = &transcript.challenge_scalar(b"zeta");
    let pl_cache = &w.plonkup.compute(zeta);
    let _pl_t = &pl_cache[0];
    let _pl_f = &pl_cache[1];
    let _pl_h1 = &pl_cache[2];
    let _pl_h2 = &pl_cache[3];
    let _pl_t_bar = &coset_scale_omega_evals(&x.h, _pl_t.clone());
    let _pl_h1_bar = &coset_scale_omega_evals(&x.h, _pl_h1.clone());
    let plp = batch_op(pl_cache, |evals| evals.clone().interpolate());
    let pl_t = &plp[0];
    let pl_f = &plp[1];
    let pl_h1 = &plp[2];
    let pl_h2 = &plp[3];
    let pl_t_bar = &_pl_t_bar.clone().interpolate();
    let pl_h1_bar = &_pl_h1_bar.clone().interpolate();
    info!("Round 2 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 3 --------------------

    let now = Instant::now();
    // β = H(transcript, 1)
    let beta = transcript.challenge_scalar(b"beta");
    // γ = H(transcript, 2)
    let gamma = transcript.challenge_scalar(b"gamma");
    // δ = H(transcript, 3)
    let delta = transcript.challenge_scalar(b"delta");
    // ε = H(transcript, 4)
    let epsilon = transcript.challenge_scalar(b"epsilon");

    // ----- Lambdas ----- //

    // plookup constraint term: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = &(epsilon * (Scalar::ONE + delta));
    let zpl_ev = |a: &Evaluations<Scalar>, b: &Evaluations<Scalar>, i: usize| {
        *zpl_sc + a.evals[i] + (delta * b.evals[i])
    };
    let zpl = |a: &Poly, b: &Poly| deg0(zpl_sc) + a + (deg0(&delta) * b);

    // copy constraint term: w(X) + β s(X) + γ
    let zcc_ev = |w: &Evaluations<Scalar>, s: &Evaluations<Scalar>, i: usize| {
        w.evals[i] + (beta * s.evals[i]) + gamma
    };
    let zcc = |w: &Poly, s: &Poly| w + (s * beta) + deg0(&gamma);

    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    //         (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let zf_cc = &(zcc(w_a, x_sida) * zcc(w_b, x_sidb) * zcc(w_c, x_sidc));
    let zf_pl = &(zpl(pl_f, pl_f) * zpl(pl_t, pl_t_bar));
    let zf = &(zf_cc * zf_pl);
    let zf_cc_ev = |i| zcc_ev(_a, _sida, i) * zcc_ev(_b, _sidb, i) * zcc_ev(_c, _sidc, i);
    let zf_pl_ev = |i| zpl_ev(_pl_f, _pl_f, i) * zpl_ev(_pl_t, _pl_t_bar, i);
    let zf_ev = |i| zf_cc_ev(i) * zf_pl_ev(i);
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zg_cc = &(zcc(w_a, x_sa) * zcc(w_b, x_sb) * zcc(w_c, x_sc));
    let zg_pl = &(zpl(pl_h1, pl_h2) * zpl(pl_h2, pl_h1_bar));
    let zg = &(zg_cc * zg_pl);
    let zg_cc_ev = |i| zcc_ev(_a, _sa, i) * zcc_ev(_b, _sb, i) * zcc_ev(_c, _sc, i);
    let zg_pl_ev = |i| zpl_ev(_pl_h1, _pl_h2, i) * zpl_ev(_pl_h2, _pl_h1_bar, i);
    let zg_ev = |i| zg_cc_ev(i) * zg_pl_ev(i);

    // ----- Calculate z ----- //
    // Z(ω) = 1
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    info!("Round 3 - A - {} s", now.elapsed().as_secs_f64());
    let z_points = (1..x.h.n() as usize - 1).fold(vec![Scalar::ONE; 2], |mut acc, i| {
        acc.push(acc[i] * zf_ev(i) / zg_ev(i));
        acc
    });
    info!("Round 3 - B - {} s", now.elapsed().as_secs_f64());
    let z_cache = Evaluations::from_vec_and_domain(z_points, x.h.domain);
    let z = &z_cache.clone().interpolate();
    info!("Round 3 - C - {} s", now.elapsed().as_secs_f64());
    // Z(ω X)
    let z_bar = &coset_scale_omega_evals(&x.h, z_cache).interpolate();
    info!("Round 3 - D - {} s", now.elapsed().as_secs_f64());
    let z_com = &pcdl::commit(z, d, None);
    transcript.append_point(b"z", z_com);
    info!("Round 3 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 4 --------------------

    let now = Instant::now();
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X) + PI(X)
    //         + Qₖ(X)(A(X) + ζB(X) + ζ²C(X) + ζ³J(X) - f(X))
    // info!("Round 4A - {} s", now.elapsed().as_secs_f64());
    let pl_query = linear_comb_poly(zeta, [w_a, w_b, w_c, x_j]);
    let pl_f_gc = &(x_qk * (pl_query - pl_f));
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
    let (t, _) = &t_a.divide_by_vanishing_poly(x.h.coset_domain);
    // info!("Round 4E3 - {} s", now.elapsed().as_secs_f64());
    let ts = &split_poly(x.h.n() as usize, t);
    // info!("Round 4F - {} s", now.elapsed().as_secs_f64());
    let t_coms: Vec<PallasPoint> = batch_commit(ts, d, None);
    // info!("Round 4G - {} s", now.elapsed().as_secs_f64());

    transcript.append_points(b"t", &t_coms);
    // transcript.append_point_new(b"t", &t_com);
    info!("Round 4 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 5 --------------------

    let now = Instant::now();
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");
    let ws_ev = batch_evaluate(&w.ws, ch);
    let qs_ev = batch_evaluate(&x.qs, ch);
    let ss_ev = batch_evaluate(&x.ss, ch);
    let pip_ev = &x_pip.evaluate(ch);
    let ts_ev = batch_evaluate(ts, ch);
    let z_ev = z.evaluate(ch);
    let pl_evs = batch_evaluate(&plp, ch);

    let ch_bar = &(*ch * x.h.w(1));
    let z_bar_ev = z_bar.evaluate(ch);
    let pl_h1_bar_ev = pl_h1_bar.evaluate(ch);
    let pl_t_bar_ev = pl_t_bar.evaluate(ch);

    transcript.append_scalars(b"ws_ev", &ws_ev);
    transcript.append_scalars(b"qs_ev", &qs_ev);
    transcript.append_scalars(b"ss_ev", &ss_ev);
    transcript.append_scalars(b"plonkup_ev", &pl_evs);
    transcript.append_scalar(b"z_bar_ev", &z_bar_ev);
    transcript.append_scalars(b"t_ev", ts_ev.as_slice());
    transcript.append_scalar(b"z_ev", &z_ev);
    // WARNING: soundness t1_bar_ev and h1_bar_ev?

    let v = &transcript.challenge_scalar(b"v");

    // W(X) = Qₗ(X) + vQᵣ(X) + v²Qₒ(X) + v³Qₖ(X) + v⁴Qₘ(X) + v⁵Q꜀(X) + v⁶Qₖ(X) + v⁷J(X)
    //      + v⁸A(X) + v⁹B(X) + v¹⁰C(X) + v¹¹Z(X)
    let W = linear_comb_poly(v, x.qs.iter().chain(w.ws.iter()).chain(std::iter::once(z)));
    // WARNING: Possible soundness issue; include plookup polynomials

    let (_, _, _, _, W_pi) = Instance::open(rng, W, d, ch, None).into_tuple();

    // W'(X) = Z(ωX)
    let W_bar = z.clone();
    let (_, _, _, _, W_bar_pi) = Instance::open(rng, W_bar, d, ch_bar, None).into_tuple();

    debug!(
        "\n{}",
        evals_str(
            &x.h,
            vec![pl_t, pl_t_bar, pl_f, pl_h1, pl_h1_bar, pl_h2, z, z_bar, f_gc, f_z1, f_z2,],
            vec![
                "t(X)".to_string(),
                "t(ωX)".to_string(),
                "f(X)".to_string(),
                "h1(X)".to_string(),
                "h1(ωX)".to_string(),
                "h2(X)".to_string(),
                "Z(X)".to_string(),
                "Z(ωX)".to_string(),
                "F_GC(X)".to_string(),
                "F_Z1(X)".to_string(),
                "F_Z2(X)".to_string(),
            ],
            vec![false; 11]
        )
    );

    let pi = Proof {
        ev: ProofEvaluations {
            ws: ws_ev,
            qs: qs_ev,
            ss: ss_ev,
            pip: *pip_ev,
            z: z_ev,
            ts: ts_ev,
            pls: pl_evs,
            z_bar: z_bar_ev,
            pl_h1_bar: pl_h1_bar_ev,
            pl_t_bar: pl_t_bar_ev,
        },
        com: ProofCommitments {
            abc_coms,
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

// TODO think how to make destructure of pi more cleaner, maybe a struct for Terms indexed objects
// TODO consider extracting equational constraints into its own function like linear_comb, thus single point of truth, no room for deviation
