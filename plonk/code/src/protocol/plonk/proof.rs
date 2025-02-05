use merlin::Transcript;
use rand::rngs::ThreadRng;

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

pub fn proof(rng: &mut ThreadRng, x: &CircuitPublic, w: &CircuitPrivate) -> SNARKProof {
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
    // Round 3 -----------------------------------------------------
    // α = H(transcript)
    let alpha = &transcript.challenge_scalar(b"alpha");
    let [ql, qr, qo, qm, qc] = &x.qs;
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
    // Round 4 -----------------------------------------------------
    // 𝔷 = H(transcript)
    let ch = &transcript.challenge_scalar(b"xi");

    let qs_abc = Instances::<{ Slots::COUNT }, true>::new_from_comm(rng, &w.ws, comms_abc, ch);
    let q_fgc = Instance::<false>::new(rng, f_gc, ch);
    let q_z = Instance::<true>::new_from_comm(rng, z, ch, comm_z);
    let q_fcc1 = Instance::<false>::new(rng, f_cc1, ch);
    let zbar_ev = zbar.evaluate(ch);
    let q_fcc2 = Instance::<false>::new(rng, f_cc2, ch);
    let q_t = Instance::<true>::new_from_comm(rng, t, ch, comm_t);

    let hdrs = vec![
        "F_GC(X)".to_string(),
        "Z(X)".to_string(),
        "Z(ωX)".to_string(),
        "F_CC1(X)".to_string(),
        "F_CC2(X)".to_string(),
    ];
    println!(
        "{}",
        x.h.evals_str(&[f_gc, z, zbar, f_cc1, f_cc2], hdrs, vec![false; 5])
    );
    SNARKProof {
        qs_abc,
        q_fgc,
        q_z,
        q_fcc1,
        zbar_ev,
        q_fcc2,
        q_t,
    }
}
