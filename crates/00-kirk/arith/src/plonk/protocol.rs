#![allow(non_snake_case)]
use std::{
    array,
    ops::{Add, AddAssign, Mul, MulAssign, Sub},
    time::Instant,
};

use anyhow::{Result, ensure};
use halo_accumulation::pcdl::{self, EvalProof, commit, open};
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::{AdditiveGroup, Field},
    ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial, univariate::DensePolynomial},
    ark_std::{One, Zero, rand::Rng},
};
use halo_poseidon::{Protocols, Sponge};
use log::debug;

use crate::{
    circuit::Trace,
    utils::{QUOTIENT_POLYS, SELECTOR_POLYS, WITNESS_POLYS, fmt_scalar},
};

#[derive(Clone)]
pub struct PlonkProofEvalProofs<P: PastaConfig> {
    r: EvalProof<P>,
    r_omega: EvalProof<P>,
}

#[derive(Clone)]
pub struct PlonkProofEvals<P: PastaConfig> {
    ws: [Scalar<P>; WITNESS_POLYS],
    rs: [Scalar<P>; WITNESS_POLYS],
    qs: [Scalar<P>; SELECTOR_POLYS],
    ts: [Scalar<P>; QUOTIENT_POLYS],
    z: Scalar<P>,
    z_omega: Scalar<P>,
    w_omegas: [Scalar<P>; 3],
}

#[derive(Clone)]
pub struct PlonkProofCommitments<P: PastaConfig> {
    ws: [Point<P>; WITNESS_POLYS],
    ts: [Point<P>; QUOTIENT_POLYS],
    z: Point<P>,
    r: Point<P>,
}

#[derive(Clone)]
pub struct PlonkProof<P: PastaConfig> {
    vs: PlonkProofEvals<P>,
    Cs: PlonkProofCommitments<P>,
    pis: PlonkProofEvalProofs<P>,
}

impl<P: PastaConfig> PlonkProof<P> {
    pub fn prove<R: Rng>(rng: &mut R, trace: Trace<P>) -> Self {
        let transcript = &mut Sponge::<P>::new(Protocols::PLONK);

        // -------------------- Round 0 --------------------

        let r0_now = Instant::now();

        let large_domain = Domain::<P>::new(QUOTIENT_POLYS * trace.rows).unwrap();
        let d = trace.rows - 1;

        let public_inputs_evals =
            Evals::<P>::from_poly_ref(&trace.public_inputs_poly, large_domain);
        let w_evals: Vec<Evals<P>> = trace
            .w_polys
            .iter()
            .map(|w| Evals::<P>::from_poly_ref(&w, large_domain))
            .collect();
        let q_evals: Vec<Evals<P>> = trace
            .q_polys
            .iter()
            .map(|w| Evals::<P>::from_poly_ref(&w, large_domain))
            .collect();
        let id_evals: Vec<Evals<P>> = trace
            .id_polys
            .iter()
            .map(|w| Evals::<P>::from_poly_ref(&w, large_domain))
            .collect();
        let sigma_evals: Vec<Evals<P>> = trace
            .sigma_polys
            .iter()
            .map(|w| Evals::<P>::from_poly_ref(&w, large_domain))
            .collect();

        let w_omega_evals =
            array::from_fn(|i| w_evals[i].clone().shift_left_small_domain(trace.domain));
        let w_omegas = w_omega_evals.map(|w| w.interpolate());

        let r0_time = r0_now.elapsed().as_secs_f64();
        debug!("Round 0 took {} s", r0_time);

        // -------------------- Round 1 --------------------
        let r1_now = Instant::now();

        let C_ws: [Point<P>; WITNESS_POLYS] =
            array::from_fn(|i| commit(&trace.w_polys[i], d, None));
        transcript.absorb_g(&C_ws);

        let r1_time = r1_now.elapsed().as_secs_f64();
        debug!("Round 1 took {} s", r1_time);

        // -------------------- Round 2 --------------------
        let r2_now = Instant::now();

        let r2_time = r2_now.elapsed().as_secs_f64();
        debug!("Round 2 took {} s", r2_time);

        // -------------------- Round 3 --------------------
        let r3_now = Instant::now();

        let beta = transcript.challenge();
        let gamma = transcript.challenge();

        let mut f_prime = &w_evals[0] + &id_evals[0].scale_ref(&beta).add_scalar(&gamma);
        let mut g_prime = &w_evals[0] + &sigma_evals[0].scale_ref(&beta).add_scalar(&gamma);
        for i in 1..WITNESS_POLYS {
            f_prime *= &(&w_evals[i] + &id_evals[i].scale_ref(&beta).add_scalar(&gamma));
            g_prime *= &(&w_evals[i] + &sigma_evals[i].scale_ref(&beta).add_scalar(&gamma));
        }

        // Z
        let mut z = vec![P::ScalarField::zero(); trace.rows];
        for i in 0..trace.rows {
            let zero_index = i;
            let one_index = (i + 1) % trace.rows;
            if one_index == 1 {
                z[zero_index] = P::ScalarField::one();
            } else {
                // TODO: Fix this disgusting indexing
                let ratio = f_prime.index_small_domain(zero_index, trace.domain)
                    / g_prime.index_small_domain(zero_index, trace.domain);
                z[zero_index] = z[zero_index - 1] * ratio
            }
        }

        let z = Evals::<P>::from_vec_and_domain(z, trace.domain);
        let z_poly = z.interpolate_by_ref();

        let C_z = pcdl::commit(&z_poly, trace.rows - 1, None);
        transcript.absorb_g(&[C_z]);

        let r3_time = r3_now.elapsed().as_secs_f64();
        debug!("Round 3 took {} s", r3_time);

        // -------------------- Round 4 --------------------
        let r4_now = Instant::now();

        let alpha = transcript.challenge();

        let f_gc: Evals<P> = &w_evals[0] * &q_evals[0]
            + &q_evals[1] * &w_evals[1]
            + &q_evals[2] * &w_evals[2]
            + &q_evals[3] * &w_evals[0] * &w_evals[1]
            + &q_evals[4]
            + public_inputs_evals;

        let l1 = lagrange_basis::<P>(1, trace.domain, large_domain);
        let z = Evals::<P>::from_poly_ref(&z_poly, large_domain);
        let z_omega = z.clone().shift_left_small_domain(trace.domain);

        let f_cc1 = l1 * (z.sub_scalar_ref(Scalar::<P>::ONE));
        let f_cc2 = &z * f_prime - z_omega * g_prime;

        let f = &f_gc + f_cc1.scale_ref(&alpha) + f_cc2.scale_ref(&alpha.pow([2]));
        let f_poly = f.interpolate();
        let (t, _) = f_poly.divide_by_vanishing_poly(trace.domain);

        let ts = t_split::<P>(t.clone(), trace.rows);
        let C_ts: [Point<P>; QUOTIENT_POLYS] = array::from_fn(|i| commit(&ts[i], d, None));

        transcript.absorb_g(&C_ts);

        let r4_time = r4_now.elapsed().as_secs_f64();
        debug!("Round 4 took {} s", r4_time);

        // -------------------- Round 5 --------------------
        let r5_now = Instant::now();

        let zeta = transcript.challenge();

        let mut vec = Vec::new();
        vec.extend_from_slice(&trace.q_polys);
        vec.extend_from_slice(&trace.w_polys);
        vec.extend_from_slice(&ts);
        vec.push(z_poly.clone());
        let r = geometric_polys::<P>(zeta, vec);

        let C_r = commit::<P>(&r, d, None);

        transcript.absorb_g(&[C_r]);

        let xi = transcript.challenge();
        let pi = Self {
            Cs: PlonkProofCommitments {
                ws: C_ws,
                ts: C_ts,
                z: C_z,
                r: C_r,
            },
            vs: PlonkProofEvals {
                ws: trace.w_polys.map(|w| w.evaluate(&xi)),
                rs: trace.r_polys.map(|r| r.evaluate(&xi)),
                qs: trace.q_polys.map(|q| q.evaluate(&xi)),
                ts: ts.map(|t| t.evaluate(&xi)),
                z: z_poly.evaluate(&xi),
                z_omega: z_poly.evaluate(&(xi * trace.omega)),
                w_omegas: w_omegas.map(|w_omega| w_omega.evaluate(&xi)),
            },
            pis: PlonkProofEvalProofs {
                r: open(rng, r.clone(), C_r, d, &xi, None),
                r_omega: open(rng, z_poly, C_z, d, &(xi * trace.omega), None),
            },
        };

        let r5_time = r5_now.elapsed().as_secs_f64();
        debug!("Round 5 took {} s", r5_time);

        let total_time = r1_time + r2_time + r3_time + r4_time + r5_time;
        let r0_frac = r0_time / total_time * 100.0;
        let r1_frac = r1_time / total_time * 100.0;
        let r2_frac = r2_time / total_time * 100.0;
        let r3_frac = r3_time / total_time * 100.0;
        let r4_frac = r4_time / total_time * 100.0;
        let r5_frac = r5_time / total_time * 100.0;

        debug!(
            "Fractions: | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% |",
            r0_frac, r1_frac, r2_frac, r3_frac, r4_frac, r5_frac
        );

        pi
    }

    pub fn naive_prover<R: Rng>(rng: &mut R, trace: Trace<P>) -> Self {
        let transcript = &mut Sponge::<P>::new(Protocols::PLONK);

        // -------------------- Round 0 --------------------

        let r0_now = Instant::now();

        let d = trace.rows - 1;

        let w_omega_evals = array::from_fn(|i| trace.w_evals[i].clone().shift_left());
        let w_omegas = w_omega_evals.map(|w| w.interpolate());

        let r0_time = r0_now.elapsed().as_secs_f64();
        debug!("Round 0 took {} s", r0_time);

        // -------------------- Round 1 --------------------
        let r1_now = Instant::now();

        let C_ws: [Point<P>; WITNESS_POLYS] =
            array::from_fn(|i| commit(&trace.w_polys[i], d, None));
        transcript.absorb_g(&C_ws);

        let r1_time = r1_now.elapsed().as_secs_f64();
        debug!("Round 1 took {} s", r1_time);

        // -------------------- Round 2 --------------------
        let r2_now = Instant::now();

        let r2_time = r2_now.elapsed().as_secs_f64();
        debug!("Round 2 took {} s", r2_time);

        // -------------------- Round 3 --------------------
        let r3_now = Instant::now();

        let beta = transcript.challenge();
        let gamma = transcript.challenge();

        let mut f_prime = &trace.w_polys[0] + &trace.id_polys[0] * beta + deg0::<P>(gamma);
        let mut g_prime = &trace.w_polys[0] + &trace.sigma_polys[0] * beta + deg0::<P>(gamma);
        for i in 1..WITNESS_POLYS {
            f_prime = &f_prime * (&trace.w_polys[i] + &trace.id_polys[i] * beta + deg0::<P>(gamma));
            g_prime =
                &g_prime * (&trace.w_polys[i] + &trace.sigma_polys[i] * beta + deg0::<P>(gamma));
        }
        let f_prime_evals = Evals::<P>::new(f_prime.evaluate_over_domain_by_ref(trace.domain));
        let g_prime_evals = Evals::<P>::new(g_prime.evaluate_over_domain_by_ref(trace.domain));

        // Z
        let mut z = vec![P::ScalarField::zero(); trace.rows];
        for i in 0..trace.rows {
            let zero_index = i;
            let one_index = (i + 1) % trace.rows;
            if one_index == 1 {
                z[zero_index] = P::ScalarField::one();
            } else {
                // TODO: Fix this disgusting indexing
                let ratio = f_prime_evals[zero_index] / g_prime_evals[zero_index];
                z[zero_index] = z[zero_index - 1] * ratio
            }
        }

        let mut prod = Scalar::<P>::one();
        for i in 0..f_prime_evals.evals.evals.len() {
            prod *= f_prime_evals[i] / g_prime_evals[i]
        }

        // assert_eq!(prod, Scalar::<P>::one());

        let z = Evals::<P>::from_vec_and_domain(z, trace.domain);
        let z_omega = z.clone().shift_left().interpolate();
        let z = z.interpolate();

        let C_z = pcdl::commit(&z, trace.rows - 1, None);
        transcript.absorb_g(&[C_z]);

        let r3_time = r3_now.elapsed().as_secs_f64();
        debug!("Round 3 took {} s", r3_time);

        // -------------------- Round 4 --------------------
        let r4_now = Instant::now();

        let alpha = transcript.challenge();

        let w = trace.w_polys.clone();
        let r = trace.r_polys.clone();
        let poseidon = poly_poseidon::<P>(P::SCALAR_POSEIDON_MDS, &r, &w, &w_omegas);
        let affine_add = affine_add_constraints_poly::<P>(&w);
        let f_gc: Poly<P> = &trace.w_polys[0] * &trace.q_polys[0]
            + &trace.q_polys[1] * &trace.w_polys[1]
            + &trace.q_polys[2] * &trace.w_polys[2]
            + &trace.q_polys[3] * &trace.w_polys[0] * &trace.w_polys[1]
            + &trace.q_polys[4]
            + &trace.q_polys[5] * &poseidon
            + &trace.q_polys[6] * &affine_add
            + &trace.public_inputs_poly;

        let l1 = lagrange_basis_poly::<P>(1, trace.domain);
        let f_cc1 = l1 * (&z - deg0::<P>(Scalar::<P>::one()));
        let f_cc2 = &z * &f_prime - z_omega * &g_prime;

        // for i in 0..trace.rows {
        //     let omega = trace.omega;
        //     let omega_i = trace.omega.pow([i as u64]);
        //     let omega_ii = trace.omega * trace.omega.pow([i as u64]);
        //     let a = trace.w_polys[0].evaluate(&omega_i);
        //     let b = trace.w_polys[1].evaluate(&omega_i);
        //     let c = trace.w_polys[2].evaluate(&omega_i);
        //     let ql = trace.q_polys[0].evaluate(&omega_i);
        //     let qr = trace.q_polys[1].evaluate(&omega_i);
        //     let qo = trace.q_polys[2].evaluate(&omega_i);
        //     let qm = trace.q_polys[3].evaluate(&omega_i);
        //     let qc = trace.q_polys[4].evaluate(&omega_i);
        //     let pi = trace.public_inputs_poly.evaluate(&omega_i);
        //     println!(
        //         "{} + {} + {} + {} + {} + {}",
        //         fmt_scalar::<P>(a * ql),
        //         fmt_scalar::<P>(b * qr),
        //         fmt_scalar::<P>(c * qo),
        //         fmt_scalar::<P>(a * b * qm),
        //         fmt_scalar::<P>(qc),
        //         fmt_scalar::<P>(pi)
        //     );

        //     let M = P::SCALAR_POSEIDON_MDS;
        //     let sbox = |w: &Scalar<P>| w.pow([7]);
        //     let w0 = trace.w_polys[0].evaluate(&omega_i);
        //     let w1 = trace.w_polys[1].evaluate(&omega_i);
        //     let w2 = trace.w_polys[2].evaluate(&omega_i);
        //     let w3 = trace.w_polys[3].evaluate(&omega_i);
        //     let w4 = trace.w_polys[4].evaluate(&omega_i);
        //     let w5 = trace.w_polys[5].evaluate(&omega_i);
        //     let w6 = trace.w_polys[6].evaluate(&omega_i);
        //     let w7 = trace.w_polys[7].evaluate(&omega_i);
        //     let w8 = trace.w_polys[8].evaluate(&omega_i);
        //     let w9 = trace.w_polys[9].evaluate(&omega_i);
        //     let w10 = trace.w_polys[10].evaluate(&omega_i);
        //     let w11 = trace.w_polys[11].evaluate(&omega_i);
        //     let w12 = trace.w_polys[12].evaluate(&omega_i);
        //     let w13 = trace.w_polys[13].evaluate(&omega_i);
        //     let w14 = trace.w_polys[14].evaluate(&omega_i);
        //     let wn0 = trace.w_polys[0].evaluate(&omega_ii);
        //     let wn1 = trace.w_polys[1].evaluate(&omega_ii);
        //     let wn2 = trace.w_polys[2].evaluate(&omega_ii);
        //     let r0 = r[0].evaluate(&omega_i);
        //     let r1 = r[1].evaluate(&omega_i);
        //     let r2 = r[2].evaluate(&omega_i);
        //     let r3 = r[3].evaluate(&omega_i);
        //     let r4 = r[4].evaluate(&omega_i);
        //     let r5 = r[5].evaluate(&omega_i);
        //     let r6 = r[6].evaluate(&omega_i);
        //     let r7 = r[7].evaluate(&omega_i);
        //     let r8 = r[8].evaluate(&omega_i);
        //     let r9 = r[9].evaluate(&omega_i);
        //     let r10 = r[10].evaluate(&omega_i);
        //     let r11 = r[11].evaluate(&omega_i);
        //     let r12 = r[12].evaluate(&omega_i);
        //     let r13 = r[13].evaluate(&omega_i);
        //     let r14 = r[14].evaluate(&omega_i);

        //     let w3_prime = r0 + sbox(&w0) * M[0][0] + sbox(&w1) * M[0][1] + sbox(&w2) * M[0][2];
        //     let w4_prime = r1 + sbox(&w0) * M[1][0] + sbox(&w1) * M[1][1] + sbox(&w2) * M[1][2];
        //     let w5_prime = r2 + sbox(&w0) * M[2][0] + sbox(&w1) * M[2][1] + sbox(&w2) * M[2][2];

        //     let w6_prime = r3 + sbox(&w3) * M[0][0] + sbox(&w4) * M[0][1] + sbox(&w5) * M[0][2];
        //     let w7_prime = r4 + sbox(&w3) * M[1][0] + sbox(&w4) * M[1][1] + sbox(&w5) * M[1][2];
        //     let w8_prime = r5 + sbox(&w3) * M[2][0] + sbox(&w4) * M[2][1] + sbox(&w5) * M[2][2];

        //     let w9_prime = r6 + sbox(&w6) * M[0][0] + sbox(&w7) * M[0][1] + sbox(&w8) * M[0][2];
        //     let w10_prime = r7 + sbox(&w6) * M[1][0] + sbox(&w7) * M[1][1] + sbox(&w8) * M[1][2];
        //     let w11_prime = r8 + sbox(&w6) * M[2][0] + sbox(&w7) * M[2][1] + sbox(&w8) * M[2][2];

        //     let w12_prime = r9 + sbox(&w9) * M[0][0] + sbox(&w10) * M[0][1] + sbox(&w11) * M[0][2];
        //     let w13_prime = r10 + sbox(&w9) * M[1][0] + sbox(&w10) * M[1][1] + sbox(&w11) * M[1][2];
        //     let w14_prime = r11 + sbox(&w9) * M[2][0] + sbox(&w10) * M[2][1] + sbox(&w11) * M[2][2];

        //     let wn0_prime =
        //         r12 + sbox(&w12) * M[0][0] + sbox(&w13) * M[0][1] + sbox(&w14) * M[0][2];
        //     let wn1_prime =
        //         r13 + sbox(&w12) * M[1][0] + sbox(&w13) * M[1][1] + sbox(&w14) * M[1][2];
        //     let wn2_prime =
        //         r14 + sbox(&w12) * M[2][0] + sbox(&w13) * M[2][1] + sbox(&w14) * M[2][2];

        //     let ch = P::scalar_from_u64(rng.next_u64());
        //     let w = [
        //         trace.w_polys[0].evaluate(&omega_i),
        //         trace.w_polys[1].evaluate(&omega_i),
        //         trace.w_polys[2].evaluate(&omega_i),
        //         trace.w_polys[3].evaluate(&omega_i),
        //         trace.w_polys[4].evaluate(&omega_i),
        //         trace.w_polys[5].evaluate(&omega_i),
        //         trace.w_polys[6].evaluate(&omega_i),
        //         trace.w_polys[7].evaluate(&omega_i),
        //         trace.w_polys[8].evaluate(&omega_i),
        //         trace.w_polys[9].evaluate(&omega_i),
        //         trace.w_polys[10].evaluate(&omega_i),
        //         trace.w_polys[11].evaluate(&omega_i),
        //         trace.w_polys[12].evaluate(&omega_i),
        //         trace.w_polys[13].evaluate(&omega_i),
        //         trace.w_polys[14].evaluate(&omega_i),
        //     ];
        //     let nw = [
        //         trace.w_polys[0].evaluate(&(omega * ch)),
        //         trace.w_polys[1].evaluate(&(omega * ch)),
        //         trace.w_polys[2].evaluate(&(omega * ch)),
        //     ];
        //     let r = [
        //         r[0].evaluate(&ch),
        //         r[1].evaluate(&ch),
        //         r[2].evaluate(&ch),
        //         r[3].evaluate(&ch),
        //         r[4].evaluate(&ch),
        //         r[5].evaluate(&ch),
        //         r[6].evaluate(&ch),
        //         r[7].evaluate(&ch),
        //         r[8].evaluate(&ch),
        //         r[9].evaluate(&ch),
        //         r[10].evaluate(&ch),
        //         r[11].evaluate(&ch),
        //         r[12].evaluate(&ch),
        //         r[13].evaluate(&ch),
        //         r[14].evaluate(&ch),
        //     ];

        //     println!("w0: {}", w0);
        //     println!("w1: {}", w1);
        //     println!("w2: {}", w2);
        //     println!("w3: {} = {}", w3, w3_prime);
        //     println!("w4: {} = {}", w4, w4_prime);
        //     println!("w5: {} = {}", w5, w5_prime);
        //     println!("w6: {} = {}", w6, w6_prime);
        //     println!("w7: {} = {}", w7, w7_prime);
        //     println!("w8: {} = {}", w8, w8_prime);
        //     println!("w9: {} = {}", w9, w9_prime);
        //     println!("w10: {} = {}", w10, w10_prime);
        //     println!("w11: {} = {}", w11, w11_prime);
        //     println!("w12: {} = {}", w12, w12_prime);
        //     println!("w13: {} = {}", w13, w13_prime);
        //     println!("w14: {} = {}", w14, w14_prime);
        //     println!("wn0: {} = {}", wn0, wn0_prime);
        //     println!("wn1: {} = {}", wn1, wn1_prime);
        //     println!("wn2: {} = {}", wn2, wn2_prime);
        //     println!(
        //         "affine: {} = {}",
        //         affine_add.evaluate(&omega_i),
        //         affine_add_constraints_scalar::<P>(w)
        //     );

        //     assert_eq!(Scalar::<P>::zero(), f_gc.evaluate(&omega_i));
        //     assert_eq!(Scalar::<P>::zero(), f_cc1.evaluate(&omega_i));
        //     assert_eq!(Scalar::<P>::zero(), f_cc2.evaluate(&omega_i));
        // }

        let f: Poly<P> = &f_gc + &f_cc1 * alpha + &f_cc2 * alpha.pow([2]);
        let (t, _) = f.divide_by_vanishing_poly(trace.domain);

        let ch = P::scalar_from_u64(rng.next_u64());
        let zH = trace.domain.vanishing_polynomial();
        assert_eq!(f.evaluate(&ch), t.evaluate(&ch) * zH.evaluate(&ch));

        let ts = t_split::<P>(t.clone(), trace.rows);
        let C_ts: [Point<P>; QUOTIENT_POLYS] = array::from_fn(|i| commit(&ts[i], d, None));

        transcript.absorb_g(&C_ts);

        let r4_time = r4_now.elapsed().as_secs_f64();
        debug!("Round 4 took {} s", r4_time);

        // -------------------- Round 5 --------------------
        let r5_now = Instant::now();

        let zeta = transcript.challenge();

        let mut vec = Vec::new();
        vec.extend_from_slice(&trace.q_polys);
        vec.extend_from_slice(&trace.w_polys);
        vec.extend_from_slice(&ts);
        vec.push(z.clone());
        let r = geometric_polys::<P>(zeta, vec);

        let C_r = commit::<P>(&r, d, None);

        transcript.absorb_g(&[C_r]);

        let xi = transcript.challenge();
        let pi = Self {
            Cs: PlonkProofCommitments {
                ws: C_ws,
                ts: C_ts,
                z: C_z,
                r: C_r,
            },
            vs: PlonkProofEvals {
                ws: trace.w_polys.map(|w| w.evaluate(&xi)),
                rs: trace.r_polys.map(|r| r.evaluate(&xi)),
                qs: trace.q_polys.map(|q| q.evaluate(&xi)),
                ts: ts.map(|t| t.evaluate(&xi)),
                z: z.evaluate(&xi),
                z_omega: z.evaluate(&(xi * trace.omega)),
                w_omegas: w_omegas.map(|w_omega| w_omega.evaluate(&xi)),
            },
            pis: PlonkProofEvalProofs {
                r: open(rng, r.clone(), C_r, d, &xi, None),
                r_omega: open(rng, z, C_z, d, &(xi * trace.omega), None),
            },
        };

        let r5_time = r5_now.elapsed().as_secs_f64();
        debug!("Round 5 took {} s", r5_time);

        let total_time = r1_time + r2_time + r3_time + r4_time + r5_time;
        let r0_frac = r0_time / total_time * 100.0;
        let r1_frac = r1_time / total_time * 100.0;
        let r2_frac = r2_time / total_time * 100.0;
        let r3_frac = r3_time / total_time * 100.0;
        let r4_frac = r4_time / total_time * 100.0;
        let r5_frac = r5_time / total_time * 100.0;

        debug!(
            "Fractions: | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% |",
            r0_frac, r1_frac, r2_frac, r3_frac, r4_frac, r5_frac
        );

        let one = Scalar::<P>::one();
        let n = trace.rows;

        let ids: [Scalar<P>; WITNESS_POLYS] = array::from_fn(|i| trace.id_polys[i].evaluate(&xi));
        let sigmas: [Scalar<P>; WITNESS_POLYS] =
            array::from_fn(|i| trace.sigma_polys[i].evaluate(&xi));

        let mut f_prime_eval = pi.vs.ws[0] + beta * ids[0] + gamma;
        let mut g_prime_eval = pi.vs.ws[0] + beta * sigmas[0] + gamma;
        for i in 1..WITNESS_POLYS {
            f_prime_eval *= pi.vs.ws[i] + beta * ids[i] + gamma;
            g_prime_eval *= pi.vs.ws[i] + beta * sigmas[i] + gamma;
        }

        let pi_eval = public_input_eval::<P>(&trace.public_inputs, trace.domain, &xi);
        let poseidon_eval = scalar_poseidon::<P>(&pi.vs.rs, &pi.vs.ws, &pi.vs.w_omegas);
        let affine_add_eval = affine_add_constraints_scalar::<P>(pi.vs.ws);
        let f_gc_eval = pi.vs.ws[0] * pi.vs.qs[0]
            + pi.vs.ws[1] * pi.vs.qs[1]
            + pi.vs.ws[2] * pi.vs.qs[2]
            + pi.vs.ws[0] * pi.vs.ws[1] * pi.vs.qs[3]
            + pi.vs.qs[4]
            + pi.vs.qs[5] * poseidon_eval
            + pi.vs.qs[6] * affine_add_eval
            + pi_eval;

        let omega = trace.omega;
        let l1 =
            (omega * (xi.pow([n as u64]) - one)) / (P::scalar_from_u64(n as u64) * (xi - omega));
        let z_H = xi.pow([n as u64]) - one;
        let f_cc1_eval = l1 * (pi.vs.z - one);
        let f_cc2_eval = pi.vs.z * f_prime_eval - pi.vs.z_omega * g_prime_eval;

        assert_eq!(poseidon.evaluate(&xi), poseidon_eval);
        assert_eq!(f_prime_eval, f_prime.evaluate(&xi));
        assert_eq!(g_prime_eval, g_prime.evaluate(&xi));
        assert_eq!(f_gc_eval, f_gc.evaluate(&xi));
        assert_eq!(f_cc1_eval, f_cc1.evaluate(&xi));
        assert_eq!(f_cc2_eval, f_cc2.evaluate(&xi));
        assert_eq!(
            t.evaluate(&xi),
            t_reconstruct::<P>(pi.vs.ts, xi, trace.rows)
        );
        assert_eq!(
            xi.pow([trace.rows as u64]) - Scalar::<P>::one(),
            trace.domain.vanishing_polynomial().evaluate(&xi),
        );

        pi
    }

    pub fn verify(self, trace: Trace<P>) -> Result<()> {
        let pi = self;
        let n = trace.rows;
        let one = Scalar::<P>::one();
        let mut transcript = Sponge::new(Protocols::PLONK);

        // -------------------- Round 1 --------------------

        transcript.absorb_g(&pi.Cs.ws);

        // -------------------- Round 2 --------------------

        // let zeta = transcript.challenge();

        // -------------------- Round 3 --------------------

        // β = H(transcript)
        let beta = transcript.challenge();
        // γ = H(transcript)
        let gamma = transcript.challenge();
        // δ = H(transcript)
        transcript.absorb_g(&[pi.Cs.z]);

        // -------------------- Round 4 --------------------

        let alpha = transcript.challenge();
        transcript.absorb_g(&pi.Cs.ts);

        // -------------------- Round 5 --------------------

        let zeta = transcript.challenge();
        transcript.absorb_g(&[pi.Cs.r]);
        let xi = transcript.challenge();
        let xi_omega = xi * trace.omega;
        let ids: [Scalar<P>; WITNESS_POLYS] = array::from_fn(|i| trace.id_polys[i].evaluate(&xi));
        let sigmas: [Scalar<P>; WITNESS_POLYS] =
            array::from_fn(|i| trace.sigma_polys[i].evaluate(&xi));

        // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
        // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
        let mut f_prime = pi.vs.ws[0] + beta * ids[0] + gamma;
        let mut g_prime = pi.vs.ws[0] + beta * sigmas[0] + gamma;
        for i in 1..WITNESS_POLYS {
            f_prime *= pi.vs.ws[i] + beta * ids[i] + gamma;
            g_prime *= pi.vs.ws[i] + beta * sigmas[i] + gamma;
        }

        // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷) + PI(𝔷)
        let poseidon_terms = scalar_poseidon::<P>(&pi.vs.rs, &pi.vs.ws, &pi.vs.w_omegas);
        let affine_add_terms = affine_add_constraints_scalar::<P>(pi.vs.ws.clone());
        let f_gc = pi.vs.ws[0] * pi.vs.qs[0]
            + pi.vs.ws[1] * pi.vs.qs[1]
            + pi.vs.ws[2] * pi.vs.qs[2]
            + pi.vs.ws[0] * pi.vs.ws[1] * pi.vs.qs[3]
            + pi.vs.qs[4]
            + pi.vs.qs[5] * poseidon_terms
            + pi.vs.qs[6] * affine_add_terms
            + public_input_eval::<P>(&trace.public_inputs, trace.domain, &xi);

        let omega = trace.omega;
        let l1 =
            (omega * (xi.pow([n as u64]) - one)) / (P::scalar_from_u64(n as u64) * (xi - omega));
        let z_H = xi.pow([n as u64]) - one;
        let f_cc1 = l1 * (pi.vs.z - one);
        let f_cc2 = pi.vs.z * f_prime - pi.vs.z_omega * g_prime;

        let f = f_gc + alpha * f_cc1 + alpha.pow([2]) * f_cc2;
        let t = t_reconstruct::<P>(pi.vs.ts, xi, n);

        ensure!(
            f == t * z_H,
            "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷) + α³ F_PL1(𝔷) + α⁴ F_PL2(𝔷)) / Zₕ(𝔷)"
        );

        let mut vec = Vec::new();
        vec.extend_from_slice(&pi.vs.qs);
        vec.extend_from_slice(&pi.vs.ws);
        vec.extend_from_slice(&pi.vs.ts);
        vec.push(pi.vs.z);
        let v_r = geometric_scalar::<P>(zeta, vec);
        pcdl::check(&pi.Cs.r, n - 1, &xi, &v_r, pi.pis.r)?;
        pcdl::check(&pi.Cs.z, n - 1, &xi_omega, &pi.vs.z_omega, pi.pis.r_omega)?;

        Ok(())
    }
}

fn t_split<P: PastaConfig>(mut t: Poly<P>, n: usize) -> [Poly<P>; QUOTIENT_POLYS] {
    // TODO: Make sure this is necessary
    assert!(
        t.degree() < QUOTIENT_POLYS * n,
        "{} < {}",
        t.degree(),
        QUOTIENT_POLYS * n
    );
    t.coeffs.resize(QUOTIENT_POLYS * n, Scalar::<P>::zero());
    let mut iter = t
        .coeffs
        .chunks(n as usize)
        .map(DensePolynomial::from_coefficients_slice);
    array::from_fn(|_| iter.next().unwrap())
}

fn t_reconstruct<P: PastaConfig>(
    ts: [Scalar<P>; QUOTIENT_POLYS],
    challenge: Scalar<P>,
    n: usize,
) -> Scalar<P> {
    let mut result = Scalar::<P>::zero();
    for (i, t) in ts.into_iter().enumerate() {
        result += challenge.pow([(i * n) as u64]) * t
    }
    result
}

fn geometric_polys<P: PastaConfig>(zeta: Scalar<P>, vec: Vec<Poly<P>>) -> Poly<P> {
    let mut result = Poly::<P>::zero();
    for (i, p) in vec.into_iter().enumerate() {
        result += &(p * zeta.pow([i as u64]))
    }
    result
}

fn geometric_scalar<P: PastaConfig>(zeta: Scalar<P>, vec: Vec<Scalar<P>>) -> Scalar<P> {
    let mut result = Scalar::<P>::zero();
    for (i, scalar) in vec.into_iter().enumerate() {
        result += scalar * &zeta.pow([i as u64]);
    }
    result
}

fn lagrange_basis<P: PastaConfig>(
    i: usize,
    small_domain: Domain<P>,
    large_domain: Domain<P>,
) -> Evals<P> {
    let mut evals = vec![Scalar::<P>::zero(); small_domain.size()];
    evals[i - 1] = Scalar::<P>::one();
    let li_poly = Evals::<P>::from_vec_and_domain(evals, small_domain).interpolate();
    Evals::new(li_poly.evaluate_over_domain(large_domain))
}

fn lagrange_basis_poly<P: PastaConfig>(i: usize, small_domain: Domain<P>) -> Poly<P> {
    let mut evals = vec![Scalar::<P>::zero(); small_domain.size()];
    evals[i - 1] = Scalar::<P>::one();
    Evals::<P>::from_vec_and_domain(evals, small_domain).interpolate()
}

fn public_input_eval<P: PastaConfig>(
    public_inputs: &[Scalar<P>],
    domain: Domain<P>,
    xi: &Scalar<P>,
) -> Scalar<P> {
    let omega = domain.element(1);
    let n = domain.size() as u64;
    let one = Scalar::<P>::one();
    let xi_n = xi.pow([n]);
    let n = P::scalar_from_u64(n);

    // (xi_n - one) * omega_j / n * (xi - omega_j)
    let mut omega_j = omega;
    let mut public_input_xi = Scalar::<P>::zero();
    for x in public_inputs {
        let l_j = ((xi_n - one) * omega_j) / (n * (*xi - omega_j));
        public_input_xi += l_j * x;
        omega_j *= omega;
    }

    public_input_xi
}

fn deg0<P: PastaConfig>(x: Scalar<P>) -> Poly<P> {
    Poly::<P>::from_coefficients_vec(vec![x])
}
// Broken
fn poly_pow<P: PastaConfig>(poly: &Poly<P>, exponent: usize) -> Poly<P> {
    if poly.is_zero() {
        Poly::<P>::zero()
    } else {
        let domain = Domain::<P>::new(exponent * poly.coeffs.len() - 1)
            .expect("field is not smooth enough to construct domain");
        let mut evals = poly.evaluate_over_domain_by_ref(domain);
        let evals_clone = evals.clone();
        for _ in 0..exponent {
            evals *= &evals_clone;
        }
        evals.interpolate()
    }
}

fn poly_poseidon<P: PastaConfig>(
    M: [[Scalar<P>; 3]; 3],
    r: &[Poly<P>; WITNESS_POLYS],
    w: &[Poly<P>; WITNESS_POLYS],
    nw: &[Poly<P>; 3],
) -> Poly<P> {
    // let sbox = |w| poly_pow::<P>(w, 7);
    let sbox = |w| w * w * w * w * w * w * w;
    #[rustfmt::skip]
    let round = |w0,w1,w2,w3,w4,w5,r0,r1,r2| {
          w3 - (r0 + sbox(w0) * M[0][0] + sbox(w1) * M[0][1] + sbox(w2) * M[0][2])
        + w4 - (r1 + sbox(w0) * M[1][0] + sbox(w1) * M[1][1] + sbox(w2) * M[1][2])
        + w5 - (r2 + sbox(w0) * M[2][0] + sbox(w1) * M[2][1] + sbox(w2) * M[2][2])
    };
    let round_1 = round(
        &w[0], &w[1], &w[2], &w[3], &w[4], &w[5], &r[0], &r[1], &r[2],
    );
    let round_2 = round(
        &w[3], &w[4], &w[5], &w[6], &w[7], &w[8], &r[3], &r[4], &r[5],
    );
    let round_3 = round(
        &w[6], &w[7], &w[8], &w[9], &w[10], &w[11], &r[6], &r[7], &r[8],
    );
    let round_4 = round(
        &w[9], &w[10], &w[11], &w[12], &w[13], &w[14], &r[9], &r[10], &r[11],
    );
    let round_5 = round(
        &w[12], &w[13], &w[14], &nw[0], &nw[1], &nw[2], &r[12], &r[13], &r[14],
    );
    round_1 + round_2 + round_3 + round_4 + round_5
}

fn scalar_poseidon<P: PastaConfig>(
    r: &[Scalar<P>; WITNESS_POLYS],
    w: &[Scalar<P>; WITNESS_POLYS],
    nw: &[Scalar<P>; 3],
) -> Scalar<P> {
    let M = P::SCALAR_POSEIDON_MDS;
    let sbox = |w: &Scalar<P>| w.pow([7]);
    #[rustfmt::skip]
    let round = |w0: &Scalar<P>, w1: &Scalar<P>, w2: &Scalar<P>, w3: &Scalar<P>,w4: &Scalar<P>, w5: &Scalar<P>, r0: &Scalar<P>, r1: &Scalar<P>, r2: &Scalar<P>| {
          *w3 - (*r0 + sbox(w0) * M[0][0] + sbox(w1) * M[0][1] + sbox(w2) * M[0][2])
        + *w4 - (*r1 + sbox(w0) * M[1][0] + sbox(w1) * M[1][1] + sbox(w2) * M[1][2])
        + *w5 - (*r2 + sbox(w0) * M[2][0] + sbox(w1) * M[2][1] + sbox(w2) * M[2][2])
    };
    let round_1 = round(
        &w[0], &w[1], &w[2], &w[3], &w[4], &w[5], &r[0], &r[1], &r[2],
    );
    let round_2 = round(
        &w[3], &w[4], &w[5], &w[6], &w[7], &w[8], &r[3], &r[4], &r[5],
    );
    let round_3 = round(
        &w[6], &w[7], &w[8], &w[9], &w[10], &w[11], &r[6], &r[7], &r[8],
    );
    let round_4 = round(
        &w[9], &w[10], &w[11], &w[12], &w[13], &w[14], &r[9], &r[10], &r[11],
    );
    let round_5 = round(
        &w[12], &w[13], &w[14], &nw[0], &nw[1], &nw[2], &r[12], &r[13], &r[14],
    );
    round_1 + round_2 + round_3 + round_4 + round_5
}

#[allow(uncommon_codepoints)]
fn affine_add_constraints_poly<P: PastaConfig>(w: &[Poly<P>; WITNESS_POLYS]) -> Poly<P> {
    let one = deg0::<P>(Scalar::<P>::one());
    let [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, _, _, _, _] = w;

    let mut terms: [Poly<P>; 12] = array::from_fn(|_| Poly::<P>::zero());

    // (xq - xp) · ((xq - xp) · λ - (yq - yp))
    let xq一xp = xq - xp;
    let yq一yp = yq - yp;
    terms[0] = (&xq一xp) * (&xq一xp * λ - yq一yp);

    // (1 - (xq - xp) · α) · (2yp · λ - 3xp²)
    let yp·2 = yp + yp;
    let xp·xp = xp * xp;
    let 〡xp·xp〡·3 = &xp·xp + &xp·xp + xp·xp;
    terms[1] = (&one - (xq一xp) * α) * (yp·2 * λ - 〡xp·xp〡·3);

    // xp · xq · (xq - xp) · (λ² - xp - xq - xr)
    let xp·xq = xp * xq;
    let xp·xq·〡xq一xp〡 = &xp·xq * (xq - xp);
    let λ·λ = λ * λ;
    let λ·λ一xp一xq一xr = λ·λ - xp - xq - xr;
    terms[2] = &xp·xq·〡xq一xp〡 * &λ·λ一xp一xq一xr;

    // xp · xq · (xq - xp) · (λ · (xp - xr) - yp - yr)
    let λ·〡xp一xr〡一yp一yr = λ * (xp - xr) - yp - yr;
    terms[3] = xp·xq·〡xq一xp〡 * &λ·〡xp一xr〡一yp一yr;

    // xp · xq · (yq + yp) · (λ² - xp - xq - xr)
    let xp·xq·〡yq〸yp〡 = xp·xq * (yq + yp);
    terms[4] = &xp·xq·〡yq〸yp〡 * λ·λ一xp一xq一xr;

    // xq · (yq + yp) · (λ · (xp - xr) - yp - yr)
    terms[5] = xp·xq·〡yq〸yp〡 * λ·〡xp一xr〡一yp一yr;

    // (1 - xp · β) · (xr - xq)
    let l一xp·β = &one - xp * β;
    terms[6] = &l一xp·β * (xr - xq);

    // (1 - xp · β) · (yr - yq)
    terms[7] = l一xp·β * (yr - yq);

    // (1 - xq · γ) · (xr - xp)
    let l一xq·γ = &one - xq * γ;
    terms[8] = &l一xq·γ * (xr - xp);

    // (1 - xq · γ) · (yr - yp)
    terms[9] = l一xq·γ * (yr - yp);

    // (1 - (xq - xp) · α - (yq + yp) · δ) · xr
    let l一〡xq一xp〡·α一〡yq〸yp〡·δ = one - (xq - xp) * α - (yq + yp) * δ;
    terms[10] = (&l一〡xq一xp〡·α一〡yq〸yp〡·δ) * xr;

    // (1 - (xq · xp) · α - (yq + yp) · δ) · yr
    terms[11] = (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * yr;

    let mut result = Poly::<P>::zero();
    for term in terms {
        result += &term
    }
    result
}

#[allow(uncommon_codepoints)]
fn affine_add_constraints_scalar<P: PastaConfig>(w: [Scalar<P>; WITNESS_POLYS]) -> Scalar<P> {
    let one = Scalar::<P>::one();
    let [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, _, _, _, _] = w;

    let mut terms: [Scalar<P>; 12] = array::from_fn(|_| Scalar::<P>::zero());
    // (xq - xp) · ((xq - xp) · λ - (yq - yp))
    let xq一xp = xq - xp;
    let yq一yp = yq - yp;
    terms[0] = (xq一xp) * (xq一xp * λ - yq一yp);

    // (1 - (xq - xp) · α) · (2yp · λ - 3xp²)
    let yp·2 = yp + yp;
    let xp·xp = xp * xp;
    let 〡xp·xp〡·3 = xp·xp + xp·xp + xp·xp;
    terms[1] = (one - (xq一xp) * α) * (yp·2 * λ - 〡xp·xp〡·3);

    // xp · xq · (xq - xp) · (λ² - xp - xq - xr)
    let xp·xq = xp * xq;
    let xp·xq·〡xq一xp〡 = xp·xq * (xq - xp);
    let λ·λ = λ * λ;
    let λ·λ一xp一xq一xr = λ·λ - xp - xq - xr;
    terms[2] = xp·xq·〡xq一xp〡 * λ·λ一xp一xq一xr;

    // xp · xq · (xq - xp) · (λ · (xp - xr) - yp - yr)
    let λ·〡xp一xr〡一yp一yr = λ * (xp - xr) - yp - yr;
    terms[3] = xp·xq·〡xq一xp〡 * λ·〡xp一xr〡一yp一yr;

    // xp · xq · (yq + yp) · (λ² - xp - xq - xr)
    let xp·xq·〡yq〸yp〡 = xp·xq * (yq + yp);
    terms[4] = xp·xq·〡yq〸yp〡 * λ·λ一xp一xq一xr;

    // xq · (yq + yp) · (λ · (xp - xr) - yp - yr)
    terms[5] = xp·xq·〡yq〸yp〡 * λ·〡xp一xr〡一yp一yr;

    // (1 - xp · β) · (xr - xq)
    let l一xp·β = one - xp * β;
    terms[6] = l一xp·β * (xr - xq);

    // (1 - xp · β) · (yr - yq)
    terms[7] = l一xp·β * (yr - yq);

    // (1 - xq · γ) · (xr - xp)
    let l一xq·γ = one - xq * γ;
    terms[8] = l一xq·γ * (xr - xp);

    // (1 - xq · γ) · (yr - yp)
    terms[9] = l一xq·γ * (yr - yp);

    // (1 - (xq - xp) · α - (yq + yp) · δ) · xr
    let l一〡xq一xp〡·α一〡yq〸yp〡·δ = one - (xq - xp) * α - (yq + yp) * δ;
    terms[10] = (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * xr;

    // (1 - (xq · xp) · α - (yq + yp) · δ) · yr
    terms[11] = (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * yr;

    let mut result = Scalar::<P>::zero();
    for term in terms {
        result += term
    }
    result
}
