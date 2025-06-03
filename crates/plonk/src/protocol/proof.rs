#![allow(non_snake_case)]
use super::pi::{EvalProofs, Proof, ProofCommitments, ProofEvaluations};
use crate::{
    circuit::{CircuitPrivate, CircuitPublic},
    pcs::PCS,
    protocol::grandproduct::GrandProduct,
    scheme::eqns::{self, EqnsF},
    utils::{self, batch_p, poly, Poly, Scalar},
};

use ark_ff::Field;
use ark_poly::Polynomial;

use halo_group::PastaConfig;
use halo_poseidon::Sponge;
use log::{debug, info};
use std::time::Instant;

pub fn prove<R: rand::Rng, P: PastaConfig, PCST: PCS<P>>(
    rng: &mut R,
    x: &CircuitPublic<P>,
    w: &CircuitPrivate<P>,
) -> Proof<P, PCST> {
    let mut transcript = Sponge::new(halo_poseidon::Protocols::PLONK);

    // -------------------- Round 1 --------------------

    let r1_now = Instant::now();
    transcript.absorb_g(&[w.com.a, w.com.b, w.com.c]);

    let r1_time = r1_now.elapsed().as_secs_f64();
    debug!("Round 1 took {} s", r1_time);

    // -------------------- Round 2 --------------------

    let r2_now = Instant::now();
    // ζ = H(transcript)
    let zeta = transcript.challenge();
    let p = &w.plookup.compute(&x.h, zeta);

    let r2_time = r2_now.elapsed().as_secs_f64();
    debug!("Round 2 took {} s", r2_time);

    // -------------------- Round 3 --------------------

    let r3_now = Instant::now();
    // β = H(transcript)
    let beta = transcript.challenge();
    // γ = H(transcript)
    let gamma = transcript.challenge();
    // δ = H(transcript)
    let delta = transcript.challenge();
    // ε = H(transcript)
    let epsilon = transcript.challenge();

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
    transcript.absorb_g(&[zcc_com]);
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
    transcript.absorb_g(&[zpl_com]);

    let r3_time = r3_now.elapsed().as_secs_f64();
    debug!("Round 3 took {} s", r3_time);
    // plookup

    // -------------------- Round 4 --------------------

    let r4_now = Instant::now();
    // α = H(transcript)
    let alpha = transcript.challenge();

    info!("Round 4A - {} s", r4_now.elapsed().as_secs_f64());
    let f_gc = &EqnsF::<P>::plonkup_eqn(zeta, w.wsp(), x.qsp(), &x.pip.p, &p.f.p);
    info!("Round 4C - {} s", r4_now.elapsed().as_secs_f64());
    let onepoly = &Poly::<P>::new_v(&Scalar::<P>::ONE).p;
    let l1poly = &Poly::<P>::new_li(&x.h, 1);
    let fcc_z1 = &eqns::grand_product1(onepoly, &zcc.p, &l1poly.p);
    let fpl_z1 = &eqns::grand_product1(onepoly, &zpl.p, &l1poly.p);
    info!("Round 4D - {} s", r4_now.elapsed().as_secs_f64());
    let fcc_z2 = &eqns::grand_product2(&zcc.p, zfcc, zgcc, zcc_bar);
    let fpl_z2 = &eqns::grand_product2(&zpl.p, zfpl, zgpl, zpl_bar);
    info!("Round 4E1 - {} s", r4_now.elapsed().as_secs_f64());
    // T(X) = (F_GC(X) + α F_CC1(X) + α² F_CC2(X) + α³ F_PL1(X) + α⁴ F_PL2(X) ) / Zₕ(X)
    let tzh = &EqnsF::<P>::geometric(alpha, [f_gc, fcc_z1, fcc_z2, fpl_z1, fpl_z2]);
    info!("Round 4E2 - {} s", r4_now.elapsed().as_secs_f64());
    let (t, _) = tzh.divide_by_vanishing_poly(x.h.coset_domain);
    info!("Round 4E3 - {} s", r4_now.elapsed().as_secs_f64());
    let ts = &Poly::new(t).split(x.h.n());
    info!("Round 4F - {} s", r4_now.elapsed().as_secs_f64());
    let ts_coms = PCST::batch_commit(batch_p(ts), x.d, None);
    info!("Round 4G - {} s", r4_now.elapsed().as_secs_f64());

    transcript.absorb_g(ts_coms.as_slice());

    let r4_time = r4_now.elapsed().as_secs_f64();
    debug!("Round 4 took {} s", r4_time);

    // -------------------- Round 5 --------------------

    let r5_now = Instant::now();
    // 𝔷 = H(transcript)
    let ch = transcript.challenge();
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
    let pl_ev = poly::batch_evaluate::<P, _>(p.base_polys(), ch);
    let pl_h1_bar_ev = p.h1_bar.evaluate(&ch);
    let pl_t_bar_ev = p.t_bar.evaluate(&ch);

    transcript.absorb_fr(ws_ev.as_slice());
    transcript.absorb_fr(qs_ev.as_slice());
    transcript.absorb_fr(ps_ev.as_slice());
    transcript.absorb_fr(pl_ev.as_slice());
    transcript.absorb_fr(&[zcc_bar_ev]);
    transcript.absorb_fr(&[zpl_bar_ev]);
    transcript.absorb_fr(ts_ev.as_slice());
    transcript.absorb_fr(&[zcc_ev]);
    transcript.absorb_fr(&[zpl_ev]);
    // WARNING: soundness t1_bar_ev and h1_bar_ev? pip?

    let v = transcript.challenge();

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
            pls: pl_ev,
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

    let r5_time = r5_now.elapsed().as_secs_f64();
    debug!("Round 5 took {} s", r5_time);

    let total_time = r1_time + r2_time + r3_time + r4_time + r5_time;
    let r1_frac = r1_time / total_time * 100.0;
    let r2_frac = r2_time / total_time * 100.0;
    let r3_frac = r3_time / total_time * 100.0;
    let r4_frac = r4_time / total_time * 100.0;
    let r5_frac = r5_time / total_time * 100.0;

    info!(
        "Fractions: | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% |",
        r1_frac, r2_frac, r3_frac, r4_frac, r5_frac
    );

    pi
}

// TODO optimization by parallel evaluate at ch?
// TODO implement inverse starting from wire then recurse till working
// TODO implement predicate / exists from wire then recurse till working
// TODO commutative plookup test?
