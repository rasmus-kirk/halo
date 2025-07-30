#![allow(non_snake_case)]
use std::{marker::PhantomData, time::Instant};

use anyhow::{Result, ensure};
use halo_accumulation::pcdl::{self, EvalProof, commit};
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::{FftField, Field},
    ark_poly::{
        self, DenseUVPolynomial, EvaluationDomain, Evaluations, Polynomial, Radix2EvaluationDomain,
    },
    ark_std::{One, Zero, rand::Rng},
};
use halo_poseidon::{Protocols, Sponge};
use log::debug;

use crate::{circuit::Trace, utils::WITNESS_POLYS};

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
        for i in 0..WITNESS_POLYS {
            let f = (&trace.w_evals[i] + trace.id_evals[i].scale(beta)).add_scalar(gamma);
            let g = (&trace.w_evals[i] + trace.sigma_evals[i].scale(beta)).add_scalar(gamma);
            f_prime *= &f;
            g_prime *= &g;
        }

        // Z
        let mut z = vec![P::ScalarField::zero(); trace.rows];
        for i in 0..trace.rows {
            let one_index = (i + 1) % 8;
            if one_index == 1 {
                z[i] = P::ScalarField::one();
            } else {
                z[i] = z[i - 1] * f_prime[one_index]
            }
        }
        let z = Evals::<P>::from_vec_and_domain(z, trace.domain);

        let poly_z = z.interpolate_by_ref();
        let zpl_com = pcdl::commit(&poly_z, trace.rows - 1, None);
        transcript.absorb_g(&[zpl_com]);

        // -------------------- Round 3 --------------------
        let a = &trace.w_polys[0];
        let b = &trace.w_polys[1];
        let c = &trace.w_polys[2];
        let q_l = &trace.q_polys[0];
        let q_r = &trace.q_polys[1];
        let q_m = &trace.q_polys[2];
        let q_o = &trace.q_polys[3];
        let q_c = &trace.q_polys[4];

        let f_gc: Poly<P> = q_l * a + q_r * b + q_o * c + q_m * a * b + q_c;

        let large_domain = Domain::<P>::new(3 * trace.rows).unwrap();
        let a = Evals::<P>::from_poly_ref(&trace.w_polys[0], large_domain);
        let b = Evals::<P>::from_poly_ref(&trace.w_polys[1], large_domain);
        let c = Evals::<P>::from_poly_ref(&trace.w_polys[2], large_domain);
        let q_l = Evals::<P>::from_poly_ref(&trace.q_polys[0], large_domain);
        let q_r = Evals::<P>::from_poly_ref(&trace.q_polys[1], large_domain);
        let q_m = Evals::<P>::from_poly_ref(&trace.q_polys[2], large_domain);
        let q_o = Evals::<P>::from_poly_ref(&trace.q_polys[3], large_domain);
        let q_c = Evals::<P>::from_poly_ref(&trace.q_polys[4], large_domain);
        let f_gc_evals: Evals<P> = &a * &q_l + q_r * &b + q_o * c + q_m * &a * b + q_c;

        let f_gc_fft = f_gc_evals.interpolate_by_ref();
        let z_H = trace.domain.vanishing_polynomial();

        let domain_32 = Domain::<P>::new(32).unwrap(); // or however you construct it
        let offset = Scalar::<P>::GENERATOR;
        let coset_domain = domain_32.get_coset(offset).unwrap();
        let z_H = domain_32.vanishing_polynomial();

        let z_H_evals = Evals::<P>::new(z_H.evaluate_over_domain(coset_domain));
        let coset_evals = Evals::<P>::new(f_gc.evaluate_over_domain_by_ref(coset_domain));

        // let f_evals = f_gc_evals.scale(Scalar::<P>::GENERATOR);
        // let t_evals = coset_evals.divide_by_vanishing(trace.domain);
        let mut t_evals = coset_evals / z_H_evals;
        // t_evals.evals.evals.iter_mut().for_each(|x| *x /= offset);
        let t_via_coset = t_evals.interpolate();
        // let t_via_coset = scale_poly_input::<P>(&t_via_coset, offset);
        // let t_evals = t_evals.scale(Scalar::<P>::GENERATOR.inverse().unwrap());
        let (t, _) = f_gc.divide_by_vanishing_poly(trace.domain);

        let a_offset = Evals::<P>::new(trace.w_polys[0].evaluate_over_domain_by_ref(coset_domain));
        let ql_offset = Evals::<P>::new(trace.q_polys[0].evaluate_over_domain_by_ref(coset_domain));

        assert_eq!(
            (&a_offset * &ql_offset).interpolate(),
            &trace.w_polys[0] * &trace.q_polys[0]
        );
        assert_eq!(
            (&a * &q_l).interpolate(),
            &trace.w_polys[0] * &trace.q_polys[0]
        );
        assert_eq!(f_gc_fft, f_gc);
        assert_eq!(t_via_coset, t);

        let (t, _) = f_gc.divide_by_vanishing_poly(trace.domain);

        println!("domain_size: {}", trace.domain.size());
        println!("f_gc degree: {}", f_gc.degree());
        println!("t degree: {}", t.degree());
    }

    pub fn verifier() {}
}

/// Scales the input of the polynomial f(x) to get f(x / a)
fn scale_poly_input<P: PastaConfig>(poly: &Poly<P>, a: Scalar<P>) -> Poly<P> {
    let a_inv = a.inverse().expect("Offset must be invertible");
    let mut coeffs = poly.coeffs.clone();

    let mut power = Scalar::<P>::one();
    for coeff in coeffs.iter_mut() {
        *coeff *= power;
        power *= a_inv;
    }

    Poly::<P>::from_coefficients_vec(coeffs)
}
