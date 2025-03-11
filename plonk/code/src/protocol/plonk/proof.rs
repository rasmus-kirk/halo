#![allow(non_snake_case)]

use anyhow::{ensure, Result};
use ark_ff::AdditiveGroup;
use halo_accumulation::{
    group::{PallasPoint, PallasScalar},
    pcdl::{EvalProof, Instance as HaloInstance},
};
use merlin::Transcript;
use rand::Rng;

use super::{
    instance::{many::Instances, Instance},
    transcript::TranscriptProtocol,
    SNARKProof,
};
use crate::{
    curve::{Point, Poly, Scalar},
    protocol::{
        circuit::{CircuitPrivate, CircuitPublic},
        scheme::Slots,
    },
};

pub fn proof<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> SNARKProof {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // A(X), B(X), C(X)
    let [a, b, c] = &w.ws;
    // S₁(X), S₂(X), S₃(X)
    let [sa, sb, sc] = &x.ss;
    // Sᵢ₁(X), Sᵢ₂(X), Sᵢ₃(X)
    let [sida, sidb, sidc] = &x.sids;

    // -------------------- Round 1 --------------------

    let comms_abc = &Poly::commit_many(&w.ws);
    transcript.append_points(b"abc", comms_abc);
    // Round 2 -----------------------------------------------------
    let zeta = &transcript.challenge_scalar(b"zeta");
    let [ql, qr, qo, qm, qc, qk, jpl] = &x.qs;
    let [tpl, fpl, h1pl, h2pl] = &w.plonkup.compute(zeta);
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

    // copy constraints
    // w(X) + β s(X) + γ
    let zterm_ev = |w, s, i| x.h.evaluate(w, i) + beta * x.h.evaluate(s, i) + gamma;
    let zterm = |w, s| w + beta * s + gamma;
    // f_cc'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    let zf_ev = |i| (zterm_ev(a, sida, i) * zterm_ev(b, sidb, i) * zterm_ev(c, sidc, i));
    let zf = &(zterm(a, sida) * zterm(b, sidb) * zterm(c, sidc));
    // g_cc'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    let zg_ev = |i| (zterm_ev(a, sa, i) * zterm_ev(b, sb, i) * zterm_ev(c, sc, i));
    let zg = &(zterm(a, sa) * zterm(b, sb) * zterm(c, sc));

    // plookup constraints
    // ε(1 + δ) + a + δb
    let zplterm_ev = |a, b, i, j| {
        epsilon * (Scalar::ONE + delta) + x.h.evaluate(a, i) + delta * x.h.evaluate(b, j)
    };
    let zplterm = |a, b| epsilon * (Scalar::ONE + delta) + a + delta * b;
    // f_pl'(X) = (ε(1 + δ) + f(X) + δf(X))(ε(1 + δ) + t(X) + δt(Xω))
    let zfpl_ev = |i| zplterm_ev(fpl, fpl, i, i) * zplterm_ev(tpl, tplbar, i, i);
    let zfpl = &(zplterm(fpl, fpl) * zplterm(tpl, tplbar));
    // g_pl'(X) = (ε(1 + δ) + h₁(X) + δh₂(X))(ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgpl_ev = |i| zplterm_ev(h1pl, h2pl, i, i) * zplterm_ev(h2pl, h1plbar, i, i);
    let zgpl = &(zplterm(h1pl, h2pl) * zplterm(h2pl, h1plbar));

    // grand product argument
    // Z(ω) = 1
    let mut z_points = vec![Scalar::ONE; 2];
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f_cc'(ωᶦ⁻¹) f_pl'(ωᶦ⁻¹) / g_cc'(ωᶦ⁻¹) g_pl'(ωᶦ⁻¹)
    for i in 1..x.h.n() - 1 {
        z_points.push(z_points[i as usize] * zf_ev(i) * zfpl_ev(i) / (zg_ev(i) * zgpl_ev(i)));
    }
    let z = &x.h.interpolate(z_points);
    // Z(ω X)
    let zbar = &x.h.poly_times_arg(z, &x.h.w(1));
    let comm_z = &z.commit();
    transcript.append_point(b"z", comm_z);

    // Round 4 -----------------------------------------------------
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X)
    let f_plgc = &(qk * (a + zeta * b + zeta * zeta * c + zeta * zeta * zeta + jpl - fpl));
    let f_gc = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + &x.pi + f_plgc);
    // F_C1(X) = L₁(X) (Z(X) - 1)
    let f_c1 = &(x.h.lagrange(1) * (z - Poly::a(&Scalar::ONE)));
    // F_C2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    let f_c2 = &((z * zf * zfpl) - (zg * zgpl * zbar));

    // T(X) = (F_GC(X) + α F_C1(X) + α² F_C2(X)) / Zₕ(X)
    // let t = &(f_gc / x.h.zh());
    let t = &((f_gc + alpha * f_c1 + alpha.pow(2) * f_c2) / x.h.zh());
    // let t = &(f_pl2 / x.h.zh());
    let comm_t = &t.commit();
    transcript.append_point(b"t", comm_t);
    // Round 5 -----------------------------------------------------
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");

    let qs_abc = Instances::<{ Slots::COUNT }>::new_from_comm(rng, &w.ws, comms_abc, ch, true);
    let q_fgc = Instance::new(rng, f_gc, ch, false);
    let q_z = Instance::new_from_comm(rng, z, ch, comm_z, true);
    let q_fcc1 = Instance::new(rng, f_c1, ch, false);
    let zbar_ev = zbar.evaluate(ch);
    let q_fcc2 = Instance::new(rng, f_c2, ch, false);
    let q_t = Instance::new_from_comm(rng, t, ch, comm_t, true);
    let fpl_ev = fpl.evaluate(ch);
    let jpl_ev = jpl.evaluate(ch);
    let q_tpl = Instance::new(rng, tpl, ch, true);
    let tplbar_ev = tplbar.evaluate(ch);
    let q_h1 = Instance::new(rng, h1pl, ch, true);
    let q_h2 = Instance::new(rng, h2pl, ch, true);
    let h1plbar_ev = h1plbar.evaluate(ch);

    let hdrs = vec![
        "t".to_string(),
        "f".to_string(),
        "h1".to_string(),
        "h2".to_string(),
        "F_GC(X)".to_string(),
        "Z(X)".to_string(),
        "Z(ωX)".to_string(),
        "F_C1(X)".to_string(),
        "F_C2(X)".to_string(),
    ];
    println!(
        "{}",
        x.h.evals_str(
            &[tpl, fpl, h1pl, h2pl, f_gc, z, zbar, f_c1, f_c2],
            hdrs,
            vec![false; 13]
        )
    );
    let pi = SNARKProof {
        qs_abc,
        q_fgc,
        q_z,
        q_fcc1,
        zbar_ev,
        q_fcc2,
        q_t,
        q_tpl,
        tplbar_ev,
        fpl_ev,
        jpl_ev,
        q_h1,
        q_h2,
        h1plbar_ev,
    };

    pi
}

pub fn prove<R: Rng>(rng: &mut R, x: &CircuitPublic, w: &CircuitPrivate) -> Proof {
    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // A(X), B(X), C(X)
    let [a, b, c] = &w.ws;
    // S₁(X), S₂(X), S₃(X)
    let [sa, sb, sc] = &x.ss;
    // Sᵢ₁(X), Sᵢ₂(X), Sᵢ₃(X)
    let [sida, sidb, sidc] = &x.sids;

    // -------------------- Round 1 --------------------

    let comms_abc = &Poly::commit_many(&w.ws);
    transcript.append_points(b"abc", comms_abc);

    // -------------------- Round 2 --------------------

    // β = H(transcript, 0)
    let beta = &transcript.challenge_scalar_augment(0, b"beta");
    // γ = H(transcript, 1)
    let gamma = &transcript.challenge_scalar_augment(1, b"gamma");
    // w(X) + β s(X) + γ
    let zterm_ev = |w, s, i| x.h.evaluate(w, i) + beta * x.h.evaluate(s, i) + gamma;
    let zterm = |w, s| w + beta * s + gamma;
    // f'(X) = (A(X) + β Sᵢ₁(X) + γ) (B(X) + β Sᵢ₂(X) + γ) (C(X) + β Sᵢ₃(X) + γ)
    let zf_ev = |i| (zterm_ev(a, sida, i) * zterm_ev(b, sidb, i) * zterm_ev(c, sidc, i));
    let zf = &(zterm(a, sida) * zterm(b, sidb) * zterm(c, sidc));
    // g'(X) = (A(X) + β S₁(X) + γ) (B(X) + β S₂(X) + γ) (C(X) + β S₃(X) + γ)
    let zg_ev = |i| (zterm_ev(a, sa, i) * zterm_ev(b, sb, i) * zterm_ev(c, sc, i));
    let zg = &(zterm(a, sa) * zterm(b, sb) * zterm(c, sc));
    // Z(ω) = 1
    let mut z_points = vec![Scalar::ONE; 2];
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    for i in 1..x.h.n() - 1 {
        z_points.push(z_points[i as usize] * zf_ev(i) / zg_ev(i));
    }
    let z = &x.h.interpolate(z_points);
    // Z(ω X)
    let zbar = &x.h.poly_times_arg(z, &x.h.w(1));
    let comm_z = &z.commit();
    transcript.append_point(b"z", comm_z);

    // -------------------- Round 3 --------------------

    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    let [ql, qr, qo, qm, qc, _] = &x.qs;
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X)
    let f_gc = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc);
    // F_CC1(X) = L₁(X) (Z(X) - 1)
    let f_cc1 = &(x.h.lagrange(1) * (z - Poly::a(&Scalar::ONE)));
    // F_CC2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    let f_cc2 = &((z * zf) - (zg * zbar));
    // T(X) = (F_GC(X) + α F_CC1(X) + α² F_CC2(X)) / Zₕ(X)
    // let t = &(f_gc / x.h.zh());
    let mut t_ = Poly::a(&Scalar::ZERO);
    for (i, &f) in [f_gc, f_cc1, f_cc2].iter().enumerate() {
        t_ = t_ + (Poly::a_exp(alpha, i as u64) * f);
    }
    let t = &(t_ / x.h.zh());
    let comm_t = &t.commit();
    transcript.append_point(b"t", comm_t);

    // -------------------- Round 4 --------------------

    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");

    // -------------------- Round 5 --------------------

    let ch_w = ch * x.h.w(1);

    let [a_com, b_com, c_com] = comms_abc;
    let t_com = comm_t;
    let z_com = comm_z;

    let a_ev = a.evaluate(ch);
    let b_ev = b.evaluate(ch);
    let c_ev = c.evaluate(ch);
    let qc_ev = qc.evaluate(ch);
    let ql_ev = ql.evaluate(ch);
    let qm_ev = qm.evaluate(ch);
    let qo_ev = qo.evaluate(ch);
    let qr_ev = qr.evaluate(ch);
    let sa_ev = sa.evaluate(ch);
    let sb_ev = sb.evaluate(ch);
    let sc_ev = sc.evaluate(ch);
    let zbar_ev = zbar.evaluate(ch);
    let t_ev = t.evaluate(ch);
    let z_ev = z.evaluate(ch);

    transcript.append_scalar(b"a_ev", &a_ev);
    transcript.append_scalar(b"b_ev", &b_ev);
    transcript.append_scalar(b"c_ev", &c_ev);
    transcript.append_scalar(b"qc_ev", &qc_ev);
    transcript.append_scalar(b"ql_ev", &ql_ev);
    transcript.append_scalar(b"qm_ev", &qm_ev);
    transcript.append_scalar(b"qo_ev", &qo_ev);
    transcript.append_scalar(b"qr_ev", &qr_ev);
    transcript.append_scalar(b"sa_ev", &sa_ev);
    transcript.append_scalar(b"sb_ev", &sb_ev);
    transcript.append_scalar(b"sc_ev", &sc_ev);
    transcript.append_scalar(b"zbar_ev", &zbar_ev);
    transcript.append_scalar(b"t_ev", &t_ev);
    transcript.append_scalar(b"z_ev", &z_ev);

    let d = a_com.d;
    let v = &transcript.challenge_scalar(b"v");

    let W: Poly = ql
        + v.pow(1) * qr
        + v.pow(2) * qo
        + v.pow(3) * qc
        + v.pow(4) * qm
        + v.pow(5) * a
        + v.pow(6) * b
        + v.pow(7) * c
        + v.pow(8) * sa
        + v.pow(9) * sb
        + v.pow(10) * sc
        + v.pow(11) * z;

    let (_, _, _, _, W_pi) =
        HaloInstance::open(rng, W.poly.clone(), d as usize, &ch.scalar, None).into_tuple();
    let z_bar_q = HaloInstance::open(rng, z.poly.clone(), d as usize, &ch_w.scalar, None);

    let pi = Proof {
        ev: ProofEvaluations {
            a: a_ev.into(),
            b: b_ev.into(),
            c: c_ev.into(),
            qc: qc_ev.into(),
            ql: ql_ev.into(),
            qm: qm_ev.into(),
            qo: qo_ev.into(),
            qr: qr_ev.into(),
            sa: sa_ev.into(),
            sb: sb_ev.into(),
            sc: sc_ev.into(),
            z: z_ev.into(),
            t: t_ev.into(),
        },
        com: ProofCommitments {
            a: a_com.into(),
            b: b_com.into(),
            c: c_com.into(),
            z: z_com.into(),
            t: t_com.into(),
        },
        W_pi,
        z_bar_q,
    };

    pi
}

pub fn verify(x: &CircuitPublic, pi: &Proof) -> Result<()> {
    let [ql, qr, qo, qm, qc, _] = &x.qs;
    let [sa, sb, sc] = &x.ss;
    let [sida, sidb, sidc] = &x.sids;
    let d = *pi.z_bar_q.d();

    let qc_com = qc.commit();
    let ql_com = ql.commit();
    let qm_com = qm.commit();
    let qo_com = qo.commit();
    let qr_com = qr.commit();
    let sa_com = sa.commit();
    let sb_com = sb.commit();
    let sc_com = sc.commit();

    let a_com: Point = pi.com.a.into();
    let b_com: Point = pi.com.b.into();
    let c_com: Point = pi.com.c.into();
    let z_com = pi.com.z.into();
    let t_com = pi.com.t.into();

    let a_ev = pi.ev.a.into();
    let b_ev = pi.ev.b.into();
    let c_ev = pi.ev.c.into();
    //let ch_w = ch * x.h.w(1);
    let qc_ev = pi.ev.qc.into();
    let ql_ev = pi.ev.ql.into();
    let qm_ev = pi.ev.qm.into();
    let qo_ev = pi.ev.qo.into();
    let qr_ev = pi.ev.qr.into();
    let sa_ev = pi.ev.sa.into();
    let sb_ev = pi.ev.sb.into();
    let sc_ev = pi.ev.sc.into();
    let zbar_ev = pi.z_bar_q.v().into();
    let t_ev = pi.ev.t.into();
    let z_ev = pi.ev.z.into();

    let mut transcript = Transcript::new(b"protocol");
    transcript.domain_sep();

    // Round 1 -----------------------------------------------------
    transcript.append_points(b"abc", &[a_com.clone(), b_com.clone(), c_com.clone()]);
    // Round 2 -----------------------------------------------------
    let beta = &transcript.challenge_scalar_augment(0, b"beta");
    let gamma = &transcript.challenge_scalar_augment(1, b"gamma");
    transcript.append_point(b"z", &z_com);
    // Round 3 -----------------------------------------------------
    let alpha = &transcript.challenge_scalar(b"alpha");
    transcript.append_point(b"t", &t_com);
    // Round 4 -----------------------------------------------------
    let ch = &transcript.challenge_scalar(b"xi");

    // round 5

    transcript.append_scalar(b"a_ev", &a_ev);
    transcript.append_scalar(b"b_ev", &b_ev);
    transcript.append_scalar(b"c_ev", &c_ev);
    transcript.append_scalar(b"qc_ev", &qc_ev);
    transcript.append_scalar(b"ql_ev", &ql_ev);
    transcript.append_scalar(b"qm_ev", &qm_ev);
    transcript.append_scalar(b"qo_ev", &qo_ev);
    transcript.append_scalar(b"qr_ev", &qr_ev);
    transcript.append_scalar(b"sa_ev", &sa_ev);
    transcript.append_scalar(b"sb_ev", &sb_ev);
    transcript.append_scalar(b"sc_ev", &sc_ev);
    transcript.append_scalar(b"zbar_ev", &zbar_ev);
    transcript.append_scalar(b"t_ev", &t_ev);
    transcript.append_scalar(b"z_ev", &z_ev);

    let v = transcript.challenge_scalar(b"v");

    // Verification

    let zh_ev = &x.h.zh().evaluate(ch);

    // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷)
    let f_gc_ev =
        &((a_ev * ql_ev) + (b_ev * qr_ev) + (c_ev * qo_ev) + (a_ev * b_ev * qm_ev) + qc_ev);
    ensure!(
        *f_gc_ev != Scalar::ZERO,
        "F_GC(𝔷) ≠ A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷), F_GC = {}", *f_gc_ev
    );
    // F_CC1(𝔷) = L₁(𝔷) (Z(𝔷) - 1)
    let f_cc1_ev = &(x.h.lagrange(1).evaluate(ch) * (z_ev - Scalar::ONE));

    // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
    let zf_ev = &((a_ev + beta * sida.evaluate(ch) + gamma)
        * (b_ev + beta * sidb.evaluate(ch) + gamma)
        * (c_ev + beta * sidc.evaluate(ch) + gamma));
    // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
    let zg_ev = &((a_ev + beta * sa_ev + gamma)
        * (b_ev + beta * sb_ev + gamma)
        * (c_ev + beta * sc_ev + gamma));
    // F_CC2(𝔷) = Z(𝔷)f'(𝔷) - g'(𝔷)Z(ω 𝔷)
    let f_cc2_ev = &((z_ev * zf_ev) - (zg_ev * zbar_ev));
    // T(𝔷) = (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)
    ensure!(
        f_gc_ev + alpha * f_cc1_ev + alpha.pow(2) * f_cc2_ev - (t_ev * zh_ev) == Scalar::ZERO,
        "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷)) / Zₕ(𝔷)"
    );

    let W_com = ql_com
        + v.pow(1) * qr_com
        + v.pow(2) * qo_com
        + v.pow(3) * qc_com
        + v.pow(4) * qm_com
        + v.pow(5) * a_com
        + v.pow(6) * b_com
        + v.pow(7) * c_com
        + v.pow(8) * sa_com
        + v.pow(9) * sb_com
        + v.pow(10) * sc_com
        + v.pow(11) * z_com;

    let W_ev = ql_ev
        + v.pow(1) * qr_ev
        + v.pow(2) * qo_ev
        + v.pow(3) * qc_ev
        + v.pow(4) * qm_ev
        + v.pow(5) * a_ev
        + v.pow(6) * b_ev
        + v.pow(7) * c_ev
        + v.pow(8) * sa_ev
        + v.pow(9) * sb_ev
        + v.pow(10) * sc_ev
        + v.pow(11) * z_ev;

    HaloInstance::new(
        W_com.point,
        d as usize,
        ch.scalar,
        W_ev.scalar.clone(),
        pi.W_pi.clone(),
    )
    .check()?;
    pi.z_bar_q.check()?;

    Ok(())
}

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
pub struct Proof {
    ev: ProofEvaluations,
    com: ProofCommitments,
    W_pi: EvalProof,
    z_bar_q: HaloInstance,
}
