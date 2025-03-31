#![allow(non_snake_case)]

use super::{
    pi::{EvalProofs, Proof, ProofCommitments, ProofEvaluations},
    transcript::TranscriptProtocol,
};
use crate::{
    circuit::{CircuitPrivate, CircuitPublic},
    scheme::eqns::plonkup_eqn,
    utils::{
        self,
        poly::{self, deg0},
    },
};

use halo_accumulation::{
    group::{PallasPoly, PallasScalar},
    pcdl::{self, Instance},
};

use ark_ff::Field;
use ark_poly::{Evaluations, Polynomial};
use log::{debug, info};
use merlin::Transcript;
use std::time::Instant;

type Scalar = PallasScalar;
type Poly = PallasPoly;
type Evals = Evaluations<Scalar>;

pub fn prove<R: rand::Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> Proof {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    // -------------------- Round 1 --------------------

    let now = Instant::now();
    let ws_coms = poly::batch_commit(&w.ws, x.d, None);
    transcript.append_points(b"abc", &ws_coms);
    info!("Round 1 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 2 --------------------

    let now = Instant::now();
    // ζ = H(transcript)
    let zeta = transcript.challenge_scalar(b"zeta");
    let p = &w.plookup.compute(&x.h, zeta);
    info!("Round 2 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 3 --------------------

    let now = Instant::now();
    // β = H(transcript)
    let beta = transcript.challenge_scalar(b"beta");
    // γ = H(transcript)
    let gamma = transcript.challenge_scalar(b"gamma");
    // δ = H(transcript)
    let delta = transcript.challenge_scalar(b"delta");
    // ε = H(transcript)
    let epsilon = transcript.challenge_scalar(b"epsilon");

    // ----- Lambdas ----- //

    // plookup constraint term: ε(1 + δ) + a(X) + δb(X)
    let zpl_sc = epsilon * (Scalar::ONE + delta);
    let pl = |a: &Evals, b: &Evals, i: usize| zpl_sc + a.evals[i] + (delta * b.evals[i]);
    let zpl = |a: &Poly, b: &Poly| deg0(&zpl_sc) + a + (deg0(&delta) * b);

    // copy constraint term: w(X) + β s(X) + γ
    let cc = |w: &Evals, s: &Evals, i: usize| w.evals[i] + (beta * s.evals[i]) + gamma;
    let zcc = |w: &Poly, s: &Poly| w + (s * beta) + deg0(&gamma);

    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    //         (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let zf_cc = zcc(w.a(), x.ia()) * zcc(w.b(), x.ib()) * zcc(w.c(), x.ic());
    let zf_pl = zpl(&p.f, &p.f) * zpl(&p.t, &p.t_bar);
    let zf = zf_cc * zf_pl;
    let zf_cc_ev = |i| cc(w._a(), x._ia(), i) * cc(w._b(), x._ib(), i) * cc(w._c(), x._ic(), i);
    let zf_pl_ev = |i| pl(&p._f, &p._f, i) * pl(&p._t, &p._t_bar, i);
    let zf_ev = |i| zf_cc_ev(i) * zf_pl_ev(i);
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    //         (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zg_cc = zcc(w.a(), x.pa()) * zcc(w.b(), x.pb()) * zcc(w.c(), x.pc());
    let zg_pl = zpl(&p.h1, &p.h2) * zpl(&p.h2, &p.h1_bar);
    let zg = zg_cc * zg_pl;
    let zg_cc_ev = |i| cc(w._a(), x._pa(), i) * cc(w._b(), x._pb(), i) * cc(w._c(), x._pc(), i);
    let zg_pl_ev = |i| pl(&p._h1, &p._h2, i) * pl(&p._h2, &p._h1_bar, i);
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
    let z_bar = &poly::shift_wrap_eval(&x.h, z_cache).interpolate();
    info!("Round 3 - D - {} s", now.elapsed().as_secs_f64());
    let z_com = pcdl::commit(z, x.d, None);
    transcript.append_point(b"z", &z_com);
    info!("Round 3 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 4 --------------------

    let now = Instant::now();
    // α = H(transcript)
    let alpha = transcript.challenge_scalar(b"alpha");

    // info!("Round 4A - {} s", now.elapsed().as_secs_f64());
    let f_gc = &plonkup_eqn(zeta, &w.ws, &x.qs, &x.pip, &p.f);
    // F_Z1(X) = L₁(X) (Z(X) - 1)
    // info!("Round 4C - {} s", now.elapsed().as_secs_f64());
    let f_z1 = &(poly::lagrange_basis(&x.h, 1) * (z - deg0(&PallasScalar::ONE)));
    // F_Z2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    // info!("Round 4D - {} s", now.elapsed().as_secs_f64());
    let f_z2 = &((z * zf) - (zg * z_bar));
    // T(X) = (F_GC(X) + α F_C1(X) + α² F_C2(X)) / Zₕ(X)
    // info!("Round 4E1 - {} s", now.elapsed().as_secs_f64());
    let tzh = utils::geometric(alpha, [f_gc, f_z1, f_z2]);
    // info!("Round 4E2 - {} s", now.elapsed().as_secs_f64());
    let (t, _) = tzh.divide_by_vanishing_poly(x.h.coset_domain);
    // info!("Round 4E3 - {} s", now.elapsed().as_secs_f64());
    let ts = &poly::split(x.h.n() as usize, &t);
    // info!("Round 4F - {} s", now.elapsed().as_secs_f64());
    let ts_coms = poly::batch_commit(ts, x.d, None);
    // info!("Round 4G - {} s", now.elapsed().as_secs_f64());

    transcript.append_points(b"t", &ts_coms);
    // transcript.append_point_new(b"t", &t_com);
    info!("Round 4 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 5 --------------------

    let now = Instant::now();
    // 𝔷 = H(transcript)
    let ch = transcript.challenge_scalar(b"xi");
    let ch_bar = &(ch * x.h.w(1));
    let z_bar_ev = z_bar.evaluate(&ch);

    let ws_ev = poly::batch_evaluate(&w.ws, ch);
    let qs_ev = poly::batch_evaluate(&x.qs, ch);
    let pip_ev = x.pip.evaluate(&ch);
    let ps_ev = poly::batch_evaluate(&x.ps, ch);
    let ts_ev = poly::batch_evaluate(ts, ch);
    let z_ev = z.evaluate(&ch);
    let pl_evs = poly::batch_evaluate(p.base_polys(), ch);
    let pl_h1_bar_ev = p.h1_bar.evaluate(&ch);
    let pl_t_bar_ev = p.t_bar.evaluate(&ch);

    transcript.append_scalars(b"ws_ev", &ws_ev);
    transcript.append_scalars(b"qs_ev", &qs_ev);
    transcript.append_scalars(b"ss_ev", &ps_ev);
    transcript.append_scalars(b"plonkup_ev", &pl_evs);
    transcript.append_scalar(b"z_bar_ev", &z_bar_ev);
    transcript.append_scalars(b"t_ev", ts_ev.as_slice());
    transcript.append_scalar(b"z_ev", &z_ev);
    // WARNING: soundness t1_bar_ev and h1_bar_ev? pip?

    let v = transcript.challenge_scalar(b"v");

    // W(X) = Qₗ(X) + vQᵣ(X) + v²Qₒ(X) + v³Qₘ(X) + v⁴Q꜀(X) + v⁵Qₖ(X) + v⁶J(X)
    //      + v⁷A(X) + v⁸B(X) + v⁹C(X) + v¹⁰Z(X)
    let W_polys = x.qs.iter().chain(w.ws.iter()).chain(std::iter::once(z));
    let W = utils::geometric(v, W_polys);
    // WARNING: Possible soundness issue; include plookup polynomials

    let (_, _, _, _, W_pi) = Instance::open(rng, W, x.d, &ch, None).into_tuple();

    // W'(X) = Z(ωX)
    let W_bar = z.clone();
    let (_, _, _, _, W_bar_pi) = Instance::open(rng, W_bar, x.d, ch_bar, None).into_tuple();

    debug!(
        "\n{}",
        utils::print_table::evals_str(
            &x.h,
            vec![&p.t, &p.t_bar, &p.f, &p.h1, &p.h1_bar, &p.h2, z, z_bar, f_gc, f_z1, f_z2,],
            utils::misc::batch_op(
                vec![
                    "t(X)", "t(ωX)", "f(X)", "h1(X)", "h1(ωX)", "h2(X)", "Z(X)", "Z(ωX)", "FGC(X)",
                    "FZ1(X)", "FZ2(X)"
                ],
                |s| s.to_string()
            ),
            vec![false; 11]
        )
    );

    let pi = Proof {
        ev: ProofEvaluations {
            ws: ws_ev,
            qs: qs_ev,
            ps: ps_ev,
            pip: pip_ev,
            z: z_ev,
            ts: ts_ev,
            pls: pl_evs,
            z_bar: z_bar_ev,
            h1_bar: pl_h1_bar_ev,
            t_bar: pl_t_bar_ev,
        },
        com: ProofCommitments {
            ws: ws_coms,
            z: z_com,
            ts: ts_coms,
        },
        pis: EvalProofs {
            W: W_pi,
            W_bar: W_bar_pi,
        },
    };

    info!("Round 5 took {} s", now.elapsed().as_secs_f64());

    pi
}

// TODO extract Z lambdas into its own function
// TODO generalize Coset for any field
// TODO generalize Arithmetizer for any field
// TODO generalize Circuit for any field; prover, verifier
// TODO compare Circuit output with rust function extensionality tester
// TODO optimization by parallel evaluate at ch?
