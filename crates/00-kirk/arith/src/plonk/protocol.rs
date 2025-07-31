#![allow(non_snake_case)]
use std::{marker::PhantomData, time::Instant};

use anyhow::{Result, ensure};
use halo_accumulation::pcdl::{self, EvalProof, commit};
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::{FftField, Field},
    ark_poly::{
        self, DenseUVPolynomial, EvaluationDomain, Evaluations, Polynomial, Radix2EvaluationDomain,
        univariate::SparsePolynomial,
    },
    ark_std::{One, Zero, rand::Rng},
};
use halo_poseidon::{Protocols, Sponge};
use log::debug;

use crate::{
    circuit::Trace,
    utils::{F_MAX_DEGREE_MULTIPLIER, WITNESS_POLYS, fmt_scalar},
};

pub struct PlonkProof<P: PastaConfig> {
    _phantom: PhantomData<P>,
}

impl<P: PastaConfig> PlonkProof<P> {
    pub fn prove(trace: Trace<P>) {
        let transcript = &mut Sponge::<P>::new(Protocols::PLONK);

        // -------------------- Round 1 --------------------
        let now = Instant::now();

        let d = trace.rows - 1;
        let C_ws: Vec<_> = trace
            .w_polys
            .iter()
            .map(|w| commit::<P>(w, d, None))
            .collect();
        transcript.absorb_g(&C_ws);

        debug!("Round 1 took {} s", now.elapsed().as_secs_f32());

        // -------------------- Round 2 --------------------
        let now = Instant::now();

        // ζ = H(transcript)
        // let zeta = transcript.challenge();
        // let p = &w.plookup.compute(&x.h, zeta);

        debug!("Round 2 took {} s", now.elapsed().as_secs_f32());

        // -------------------- Round 3 --------------------
        // β = H(transcript)
        let beta = transcript.challenge();
        // γ = H(transcript)
        let gamma = transcript.challenge();

        let mut f_prime = Evals::<P>::one(trace.domain);
        let mut g_prime = Evals::<P>::one(trace.domain);
        // let f = (&trace.w_evals[0] + &trace.id_evals[0]);
        // let g = (&trace.w_evals[0] + &trace.sigma_evals[0]);
        // f_prime *= &f;
        // g_prime *= &g;
        for i in 0..WITNESS_POLYS {
            let f = (&trace.w_evals[i] + trace.id_evals[i].scale_ref(beta)).add_scalar(gamma);
            let g = (&trace.w_evals[i] + trace.sigma_evals[i].scale_ref(beta)).add_scalar(gamma);
            // let f = (&trace.w_evals[i] + &trace.id_evals[i]);
            // let g = (&trace.w_evals[i] + &trace.sigma_evals[i]);
            f_prime *= &f;
            g_prime *= &g;
        }
        let f_prime_poly = f_prime.interpolate_by_ref();
        let g_prime_poly = g_prime.interpolate_by_ref();

        // Z
        let mut z = vec![P::ScalarField::zero(); trace.rows];
        for i in 0..trace.rows {
            let zero_index = i;
            let one_index = (i + 1) % 8;
            if one_index == 1 {
                z[zero_index] = P::ScalarField::one();
            } else {
                // TODO: Fix this disgusting indexing
                z[zero_index] = z[zero_index - 1] * f_prime[zero_index] / g_prime[zero_index]
            }
        }
        let z = Evals::<P>::from_vec_and_domain(z, trace.domain);
        let z_poly = z.interpolate_by_ref();

        let z_omega = z.clone().shift_left();
        let z_omega_poly = z_omega.interpolate_by_ref();

        let zpl_com = pcdl::commit(&z_poly, trace.rows - 1, None);
        transcript.absorb_g(&[zpl_com]);

        // -------------------- Round 3 --------------------
        let large_domain = Domain::<P>::new(F_MAX_DEGREE_MULTIPLIER * trace.rows).unwrap();
        let a = Evals::<P>::from_poly_ref(&trace.w_polys[0], large_domain);
        let b = Evals::<P>::from_poly_ref(&trace.w_polys[1], large_domain);
        let c = Evals::<P>::from_poly_ref(&trace.w_polys[2], large_domain);
        let q_l = Evals::<P>::from_poly_ref(&trace.q_polys[0], large_domain);
        let q_r = Evals::<P>::from_poly_ref(&trace.q_polys[1], large_domain);
        let q_o = Evals::<P>::from_poly_ref(&trace.q_polys[2], large_domain);
        let q_m = Evals::<P>::from_poly_ref(&trace.q_polys[3], large_domain);
        let q_c = Evals::<P>::from_poly_ref(&trace.q_polys[4], large_domain);
        let f_gc_evals: Evals<P> = &a * q_l + q_r * &b + q_o * c + q_m * a * b + q_c;

        let f_gc_fft = f_gc_evals.interpolate_by_ref();

        let a_poly = &trace.w_polys[0];
        let b_poly = &trace.w_polys[1];
        let c_poly = &trace.w_polys[2];
        let q_l = &trace.q_polys[0];
        let q_r = &trace.q_polys[1];
        let q_o = &trace.q_polys[2];
        let q_m = &trace.q_polys[3];
        let q_c = &trace.q_polys[4];

        let f_gc: Poly<P> =
            q_l * a_poly + q_r * b_poly + q_o * c_poly + q_m * a_poly * b_poly + q_c;

        let domain = trace.domain;
        let (t, _rem) = f_gc.divide_by_vanishing_poly(domain);

        //assert_eq!(rem, Poly::<P>::zero());
        assert_eq!(f_gc_fft, f_gc);
        let xi = transcript.challenge();
        assert_eq!(
            f_gc.evaluate(&xi),
            t.evaluate(&xi) * trace.domain.vanishing_polynomial().evaluate(&xi)
        );

        let l1 = lagrange_basis::<P>(1, domain);
        let l1_poly = l1.interpolate_by_ref();

        let f_cc1_poly = &l1_poly * (&z_poly - deg0::<P>(Scalar::<P>::ONE));
        let f_cc2_poly = &z_poly * &f_prime_poly - &z_omega_poly * &g_prime_poly;

        let l1_evals = Evals::<P>::from_poly_ref(&l1_poly, large_domain);
        let z_evals = Evals::<P>::from_poly_ref(&z_poly, large_domain);
        let z_omega_evals = Evals::<P>::from_poly_ref(&z_omega_poly, large_domain);
        let f_prime_evals = Evals::<P>::from_poly_ref(&f_prime_poly, large_domain);
        let g_prime_evals = Evals::<P>::from_poly_ref(&g_prime_poly, large_domain);
        let f_cc1 = l1_evals * (z_evals.sub_scalar_ref(Scalar::<P>::ONE));
        let f_cc2 = z_evals * f_prime_evals - z_omega_evals * g_prime_evals;
        let f_cc1_fft = f_cc1.interpolate();
        let f_cc2_fft = f_cc2.interpolate();

        assert_eq!(f_cc1_poly, f_cc1_fft);
        assert_eq!(f_cc2_poly, f_cc2_fft);
        for i in 1..trace.rows + 1 {
            let omega_i = trace.omega.pow([i as u64]);
            let omega_ii = trace.omega.pow([i as u64 + 1]);

            let v_a = a_poly.evaluate(&omega_i);
            let v_b = b_poly.evaluate(&omega_i);
            let v_c = c_poly.evaluate(&omega_i);
            let v_sa = trace.sigma_polys[0].evaluate(&omega_i);
            let v_sb = trace.sigma_polys[1].evaluate(&omega_i);
            let v_sc = trace.sigma_polys[2].evaluate(&omega_i);
            let v_ia = trace.id_polys[0].evaluate(&omega_i);
            let v_ib = trace.id_polys[1].evaluate(&omega_i);
            let v_ic = trace.id_polys[2].evaluate(&omega_i);
            let v_z = z_poly.evaluate(&omega_i);
            let v_z_omega_check = z_poly.evaluate(&omega_ii);
            let v_z_omega = z_omega_poly.evaluate(&omega_i);
            let v_f_prime = f_prime_poly.evaluate(&omega_i);
            let v_g_prime = g_prime_poly.evaluate(&omega_i);
            let v_fg = f_prime_poly.evaluate(&omega_i) / f_prime_poly.evaluate(&omega_i);

            assert_eq!(v_z_omega, v_z_omega_check);
            assert_eq!(v_z_omega, v_z * (v_f_prime / v_g_prime));
            assert_eq!(v_z * v_f_prime, v_z_omega * v_g_prime);
            assert_eq!(v_z * v_f_prime - v_z_omega * v_g_prime, Scalar::<P>::zero());
            println!("{i:?}");
            // println!("f_prime: {v_f_prime} = {v_a} + {v_ia} * {v_b} + {v_ib} * {v_c} + {v_ic}");
            // println!("g_prime: {v_g_prime} = {v_a} + {v_sa} * {v_b} + {v_sb} * {v_c} + {v_sc}");
            // println!("{v_z} * {v_f_prime} - {v_z_omega} * {v_g_prime}");
            if i == 1 {
                assert_eq!(l1_poly.evaluate(&omega_i), Scalar::<P>::one());
            } else {
                assert_eq!(l1_poly.evaluate(&omega_i), Scalar::<P>::zero());
            }
            assert_eq!(f_gc.evaluate(&omega_i), Scalar::<P>::zero());
            assert_eq!(f_cc1_poly.evaluate(&omega_i), Scalar::<P>::zero());
            assert_eq!(f_cc2_poly.evaluate(&omega_i), Scalar::<P>::zero());
        }
    }

    pub fn verifier() {}
}

fn lagrange_basis<P: PastaConfig>(i: usize, domain: Domain<P>) -> Evals<P> {
    let mut evals = vec![Scalar::<P>::zero(); domain.size()];
    evals[i - 1] = Scalar::<P>::one();
    Evals::from_vec_and_domain(evals, domain)
}

fn deg0<P: PastaConfig>(x: Scalar<P>) -> Poly<P> {
    Poly::<P>::from_coefficients_vec(vec![x])
}

fn sparse_to_dense<P: PastaConfig>(sparse: SparsePolynomial<Scalar<P>>) -> Poly<P> {
    sparse.into()
}
