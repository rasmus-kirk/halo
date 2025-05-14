#![allow(non_snake_case)]
use super::{
    pi::{EvalProofs, Proof, ProofCommitments, ProofEvaluations},
    transcript::TranscriptProtocol,
};
use crate::{
    circuit::{CircuitPrivate, CircuitPublic},
    pcs::PCS,
    protocol::grandproduct::GrandProduct,
    scheme::eqns::{self, EqnsF},
    utils::{self, batch_p, poly, Poly, Scalar},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use ark_poly::Polynomial;

use log::{debug, info};
use merlin::Transcript;
use std::time::Instant;

pub fn prove<R: rand::Rng, P: SWCurveConfig, PCST: PCS<P>>(
    rng: &mut R,
    x: &CircuitPublic<P>,
    w: &CircuitPrivate<P>,
) -> Proof<P, PCST>
where
    Transcript: TranscriptProtocol<P>,
{
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();
    // -------------------- Round 1 --------------------

    let now = Instant::now();
    transcript.append_points(b"abc", &[w.com.a, w.com.b, w.com.c]);
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

    // a + βb + γ
    let _cc = eqns::copy_constraint_term(Into::<Scalar<P>>::into, beta, gamma);
    let cc = eqns::copy_constraint_term(poly::deg0::<P>, beta, gamma);
    // fcc'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    let zfcc = &(cc(&w.a.p, &x.ia.p) * cc(&w.b.p, &x.ib.p) * cc(&w.c.p, &x.ic.p));
    let _zfcc = |i| _cc(w.a[i], x.ia[i]) * _cc(w.b[i], x.ib[i]) * _cc(w.c[i], x.ic[i]);
    // gcc'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    let zgcc = &(cc(&w.a.p, &x.pa.p) * cc(&w.b.p, &x.pb.p) * cc(&w.c.p, &x.pc.p));
    let _zgcc = |i| _cc(w.a[i], x.pa[i]) * _cc(w.b[i], x.pb[i]) * _cc(w.c[i], x.pc[i]);
    // Z
    let zcc = &GrandProduct::<P>::poly(&x.h, _zfcc, _zgcc);
    let zcc_bar = &zcc.e.clone().shift_left().fft_sp();
    let zcc_com = PCST::commit(&zcc.p, x.d, None);
    transcript.append_point(b"zcc", &zcc_com);
    info!("Round 3 - A - {} s", now.elapsed().as_secs_f64());
    // copy constraints

    // ε(1 + δ) + a + δb
    let e1d = epsilon * (Scalar::<P>::ONE + delta);
    let _pl = eqns::plookup_term(Into::<Scalar<P>>::into, e1d, delta);
    let pl = eqns::plookup_term(poly::deg0::<P>, e1d, delta);
    // fpl'(X) = (ε(1 + δ) + f(X) + δf(X)) (ε(1 + δ) + t(X) + δt(Xω))
    let zfpl = &(pl(&p.f.p, &p.f.p) * pl(&p.t.p, &p.t_bar.p));
    let _zfpl = |i| _pl(p.f[i], p.f[i]) * _pl(p.t[i], p.t_bar[i]);
    // gpl'(X) = (ε(1 + δ) + h₁(X) + δh₂(X)) (ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgpl = &(pl(&p.h1.p, &p.h2.p) * pl(&p.h2.p, &p.h1_bar.p));
    let _zgpl = |i| _pl(p.h1[i], p.h2[i]) * _pl(p.h2[i], p.h1_bar[i]);
    // Z
    let zpl = &GrandProduct::<P>::poly(&x.h, _zfpl, _zgpl);
    let zpl_bar = &zpl.e.clone().shift_left().fft_sp();
    let zpl_com = PCST::commit(&zpl.p, x.d, None);
    transcript.append_point(b"zpl", &zpl_com);
    info!("Round 3 took {} s", now.elapsed().as_secs_f64());
    // plookup

    // -------------------- Round 4 --------------------

    let now = Instant::now();
    // α = H(transcript)
    let alpha = transcript.challenge_scalar(b"alpha");

    info!("Round 4A - {} s", now.elapsed().as_secs_f64());
    let f_gc = &EqnsF::<P>::plonkup_eqn(zeta, w.wsp(), x.qsp(), &x.pip.p, &p.f.p);
    info!("Round 4C - {} s", now.elapsed().as_secs_f64());
    let onepoly = &Poly::<P>::new_v(&Scalar::<P>::ONE).p;
    let l1poly = &Poly::<P>::new_li(&x.h, 1);
    let fcc_z1 = &eqns::grand_product1(onepoly, &zcc.p, &l1poly.p);
    let fpl_z1 = &eqns::grand_product1(onepoly, &zpl.p, &l1poly.p);
    info!("Round 4D - {} s", now.elapsed().as_secs_f64());
    let fcc_z2 = &eqns::grand_product2(&zcc.p, zfcc, zgcc, zcc_bar);
    let fpl_z2 = &eqns::grand_product2(&zpl.p, zfpl, zgpl, zpl_bar);
    info!("Round 4E1 - {} s", now.elapsed().as_secs_f64());
    // T(X) = (F_GC(X) + α F_CC1(X) + α² F_CC2(X) + α³ F_PL1(X) + α⁴ F_PL2(X) ) / Zₕ(X)
    let tzh = &EqnsF::<P>::geometric(alpha, [f_gc, fcc_z1, fcc_z2, fpl_z1, fpl_z2]);
    info!("Round 4E2 - {} s", now.elapsed().as_secs_f64());
    let (t, _) = tzh.divide_by_vanishing_poly(x.h.coset_domain);
    info!("Round 4E3 - {} s", now.elapsed().as_secs_f64());
    let ts = &Poly::new(t).split(x.h.n());
    info!("Round 4F - {} s", now.elapsed().as_secs_f64());
    let ts_coms = PCST::batch_commit(batch_p(ts), x.d, None);
    info!("Round 4G - {} s", now.elapsed().as_secs_f64());

    transcript.append_points(b"t", &ts_coms);
    info!("Round 4 took {} s", now.elapsed().as_secs_f64());

    // -------------------- Round 5 --------------------

    let now = Instant::now();
    // 𝔷 = H(transcript)
    let ch = transcript.challenge_scalar(b"xi");
    let ch_bar = &(ch * x.h.w(1));
    let zcc_bar_ev = zcc_bar.evaluate(&ch);
    let zpl_bar_ev = zpl_bar.evaluate(&ch);

    let ws_ev = poly::batch_evaluate::<P, _>(w.ws(), ch);
    let qs_ev = poly::batch_evaluate::<P, _>(x.qs(), ch);
    let pip_ev = x.pip.evaluate(&ch);
    let ps_ev = poly::batch_evaluate::<P, _>(x.ps(), ch);
    let ts_ev = poly::batch_evaluate::<P, _>(ts, ch);
    let zcc_ev = zcc.evaluate(&ch);
    let zpl_ev = zpl.evaluate(&ch);
    let pl_evs = poly::batch_evaluate::<P, _>(p.base_polys(), ch);
    let pl_h1_bar_ev = p.h1_bar.evaluate(&ch);
    let pl_t_bar_ev = p.t_bar.evaluate(&ch);

    transcript.append_scalars(b"ws_ev", &ws_ev);
    transcript.append_scalars(b"qs_ev", &qs_ev);
    transcript.append_scalars(b"ss_ev", &ps_ev);
    transcript.append_scalars(b"plonkup_ev", &pl_evs);
    transcript.append_scalar(b"zcc_bar_ev", &zcc_bar_ev);
    transcript.append_scalar(b"zpl_bar_ev", &zpl_bar_ev);
    transcript.append_scalars(b"t_ev", ts_ev.as_slice());
    transcript.append_scalar(b"zcc_ev", &zcc_ev);
    transcript.append_scalar(b"zpl_ev", &zpl_ev);
    // WARNING: soundness t1_bar_ev and h1_bar_ev? pip?

    let v = transcript.challenge_scalar(b"v");

    // W(X) = Qₗ(X) + vQᵣ(X) + v²Qₒ(X) + v³Qₘ(X) + v⁴Q꜀(X) + v⁵Qₖ(X) + v⁶J(X)
    //      + v⁷A(X) + v⁸B(X) + v⁹C(X) + v¹⁰ZCC(X) + v¹¹ZPL(X)
    let W = EqnsF::<P>::flat_geometric(v, [x.qsp(), w.wsp(), vec![&zcc.p, &zpl.p]]);
    // WARNING: Possible soundness issue; include plookup polynomials

    let (_, _, _, _, W_pi) = PCST::open(rng, W, x.d, &ch, None);

    debug!(
        "\n{}",
        utils::print_table::evals_str(
            &x.h,
            batch_p([&p.t, &p.t_bar, &p.f, &p.h1, &p.h1_bar, &p.h2, zcc, zpl])
                .into_iter()
                .chain([f_gc, fcc_z1, fcc_z2, fpl_z1, fpl_z2].into_iter())
                .collect(),
            utils::misc::batch_op(
                vec![
                    "t(X)", "t(ωX)", "f(X)", "h1(X)", "h1(ωX)", "h2(X)", "ZCC(X)", "ZPL(X)",
                    "FGC(X)", "FCCZ1(X)", "FCCZ2(X)", "FPLZ1(X)", "FPLZ2(X)"
                ],
                |s| s.to_string()
            ),
            vec![false; 15]
        )
    );

    // W'(X) = ZCC(ωX) + vZPL(ωX)
    let W_bar = EqnsF::<P>::geometric(v, [&zcc.p, &zpl.p]);
    let (_, _, _, _, W_bar_pi) = PCST::open(rng, W_bar, x.d, ch_bar, None);

    let pi = Proof {
        ev: ProofEvaluations {
            ws: ws_ev,
            qs: qs_ev,
            ps: ps_ev,
            pip: pip_ev,
            zcc: zcc_ev,
            zpl: zpl_ev,
            ts: ts_ev,
            pls: pl_evs,
            zcc_bar: zcc_bar_ev,
            zpl_bar: zpl_bar_ev,
            h1_bar: pl_h1_bar_ev,
            t_bar: pl_t_bar_ev,
        },
        com: ProofCommitments {
            ws: vec![w.com.a, w.com.b, w.com.c],
            zcc: zcc_com,
            zpl: zpl_com,
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

// TODO optimization by parallel evaluate at ch?
// TODO implement inverse starting from wire then recurse till working
// TODO implement predicate / exists from wire then recurse till working
// TODO commutative plookup test?
