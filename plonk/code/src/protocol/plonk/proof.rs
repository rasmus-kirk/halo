use merlin::Transcript;
use rand::Rng;

use super::{
    instance::{many::Instances, Instance},
    transcript::TranscriptProtocol,
    SNARKProof,
};
use crate::{
    curve::{Poly, Scalar},
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
    // Round 1 -----------------------------------------------------
    let comms_abc = &Poly::commit_many(&w.ws);
    transcript.append_points(b"abc", comms_abc);
    // Round 2 -----------------------------------------------------
    let zeta = &transcript.challenge_scalar(b"zeta");
    let [ql, qr, qo, qm, qc, qk, jpl] = &x.qs;
    let [tpl, fpl, h1pl, h2pl] = &x.plonkup.compute(zeta);
    let tplbar = &x.h.poly_times_arg(tpl, &x.h.w(1));
    let h1plbar = &x.h.poly_times_arg(h1pl, &x.h.w(1));
    // Round 3 -----------------------------------------------------
    // β = H(transcript, 1)
    let beta = &transcript.challenge_scalar_augment(1, b"beta");
    // γ = H(transcript, 2)
    let gamma = &transcript.challenge_scalar_augment(2, b"gamma");
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

    // δ = H(transcript, 3)
    let delta = &transcript.challenge_scalar_augment(3, b"delta");
    // ε = H(transcript, 4)
    let epsilon = &transcript.challenge_scalar_augment(4, b"epsilon");

    // ε(1 + δ) + a + δb
    let zplterm_ev = |a, b, i, j| {
        epsilon * (Scalar::ONE + delta) + x.h.evaluate(a, i) + delta * x.h.evaluate(b, j)
    };
    let zplterm = |a, b| epsilon * (Scalar::ONE + delta) + a + delta * b;
    // f'(X) = (ε(1 + δ) + f(X) + δf(X))(ε(1 + δ) + t(X) + δt(Xω))
    let zfpl_ev = |i| zplterm_ev(fpl, fpl, i, i) * zplterm_ev(tpl, tplbar, i, i);
    let zfpl = &(zplterm(fpl, fpl) * zplterm(tpl, tplbar));
    // g'(X) = (ε(1 + δ) + h₁(X) + δh₂(X))(ε(1 + δ) + h₂(X) + δh₁(Xω))
    let zgpl_ev = |i| zplterm_ev(h1pl, h2pl, i, i) * zplterm_ev(h2pl, h1plbar, i, i);
    let zgpl = &(zplterm(h1pl, h2pl) * zplterm(h2pl, h1plbar));
    // Z_PL(ω) = 1
    let mut zpl_points = vec![Scalar::ONE; 2];
    // Z_PL(ωⁱ) = Z_PL(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    for i in 1..x.h.n() - 1 {
        zpl_points.push(zpl_points[i as usize] * zfpl_ev(i) / zgpl_ev(i));
    }
    let zpl = &x.h.interpolate(zpl_points);
    // Z_PL(ω X)
    let zplbar = &x.h.poly_times_arg(zpl, &x.h.w(1));
    let comm_zpl = &zpl.commit();
    transcript.append_point(b"zpl", comm_zpl);

    // Round 4 -----------------------------------------------------
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    // F_GC(X) = A(X)Qₗ(X) + B(X)Qᵣ(X) + C(X)Qₒ(X) + A(X)B(X)Qₘ(X) + Q꜀(X)
    let f_plgc = &(qk * (a + zeta * b + zeta * zeta * c + zeta * zeta * zeta + jpl - fpl));
    let f_gc = &((a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + &x.pi + f_plgc);
    // F_CC1(X) = L₁(X) (Z(X) - 1)
    let f_cc1 = &(x.h.lagrange(1) * (z - Poly::a(&Scalar::ONE)));
    // F_CC2(X) = Z(X)f'(X) - g'(X)Z(ω X)
    let f_cc2 = &((z * zf) - (zg * zbar));
    // F_PL1(X) = L₁(X) (Z_PL(X) - 1)
    let f_pl1 = &(x.h.lagrange(1) * (zpl - Poly::a(&Scalar::ONE)));
    // F_PL2(X) = Z_PL(X)f'(X) - g'(X)Z_PL(ω X)
    let f_pl2 = &((zpl * zfpl) - (zgpl * zplbar));

    // T(X) = (F_GC(X) + α F_CC1(X) + α² F_CC2(X)) / Zₕ(X)
    // let t = &(f_gc / x.h.zh());
    let t = &((f_gc
        + alpha * f_cc1
        + alpha.pow(2) * f_cc2
        + alpha.pow(3) * f_pl1
        + alpha.pow(4) * f_pl2)
        / x.h.zh());
    // let t = &(f_pl2 / x.h.zh());
    let comm_t = &t.commit();
    transcript.append_point(b"t", comm_t);
    // Round 5 -----------------------------------------------------
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");

    let qs_abc = Instances::<{ Slots::COUNT }>::new_from_comm(rng, &w.ws, comms_abc, ch, true);
    let q_fgc = Instance::new(rng, f_gc, ch, false);
    let q_z = Instance::new_from_comm(rng, z, ch, comm_z, true);
    let q_fcc1 = Instance::new(rng, f_cc1, ch, false);
    let zbar_ev = zbar.evaluate(ch);
    let q_fcc2 = Instance::new(rng, f_cc2, ch, false);
    let q_t = Instance::new_from_comm(rng, t, ch, comm_t, true);
    let fpl_ev = fpl.evaluate(ch);
    let jpl_ev = jpl.evaluate(ch);
    let q_zpl = Instance::new_from_comm(rng, zpl, ch, comm_zpl, true);
    let q_tpl = Instance::new(rng, tpl, ch, true);
    let tplbar_ev = tplbar.evaluate(ch);
    let zplbar_ev = zplbar.evaluate(ch);
    let q_h1 = Instance::new(rng, h1pl, ch, true);
    let q_h2 = Instance::new(rng, h2pl, ch, true);
    let h1plbar_ev = h1plbar.evaluate(ch);

    let hdrs = vec![
        "F_GC(X)".to_string(),
        "Z1(X)".to_string(),
        "Z1(ωX)".to_string(),
        "F_CC1(X)".to_string(),
        "F_CC2(X)".to_string(),
        "t".to_string(),
        "f".to_string(),
        "h1".to_string(),
        "h2".to_string(),
        "Z2(X)".to_string(),
        "Z2(ωX)".to_string(),
        "F_PL1(X)".to_string(),
        "F_PL2(X)".to_string(),
    ];
    println!(
        "{}",
        x.h.evals_str(
            &[f_gc, z, zbar, f_cc1, f_cc2, tpl, fpl, h1pl, h2pl, zpl, zplbar, f_pl1, f_pl2],
            hdrs,
            vec![false; 13]
        )
    );
    SNARKProof {
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
        q_zpl,
        zplbar_ev,
        q_h1,
        q_h2,
        h1plbar_ev,
    }
}
