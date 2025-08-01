#![allow(non_snake_case)]
use std::{array, time::Instant};

use anyhow::{Result, ensure};
use halo_accumulation::pcdl::{self, EvalProof, commit, open};
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::Field,
    ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial, univariate::DensePolynomial},
    ark_std::{One, Zero, rand::Rng},
};
use halo_poseidon::{Protocols, Sponge};
use log::debug;

use crate::{
    circuit::Trace,
    utils::{QUOTIENT_POLYS, SELECTOR_POLYS, WITNESS_POLYS},
};

pub struct PlonkProofEvalProofs<P: PastaConfig> {
    r: EvalProof<P>,
    r_omega: EvalProof<P>,
}

pub struct PlonkProofEvals<P: PastaConfig> {
    ws: [Scalar<P>; WITNESS_POLYS],
    qs: [Scalar<P>; SELECTOR_POLYS],
    ts: [Scalar<P>; QUOTIENT_POLYS],
    z: Scalar<P>,
    z_omega: Scalar<P>,
}

pub struct PlonkProofCommitments<P: PastaConfig> {
    ws: [Point<P>; WITNESS_POLYS],
    ts: [Point<P>; QUOTIENT_POLYS],
    z: Point<P>,
    r: Point<P>,
}

pub struct PlonkProof<P: PastaConfig> {
    vs: PlonkProofEvals<P>,
    Cs: PlonkProofCommitments<P>,
    pis: PlonkProofEvalProofs<P>,
}

impl<P: PastaConfig> PlonkProof<P> {
    pub fn prove<R: Rng>(rng: &mut R, trace: Trace<P>) -> Self {
        let transcript = &mut Sponge::<P>::new(Protocols::PLONK);
        println!("{:?}", trace.public_inputs);
        println!("{:?}", trace.public_inputs_evals.evals.evals);

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

        debug!("Round 0 took {} s", r0_now.elapsed().as_secs_f32());

        // -------------------- Round 1 --------------------
        let r1_now = Instant::now();

        let C_ws: [Point<P>; WITNESS_POLYS] =
            array::from_fn(|i| commit(&trace.w_polys[i], d, None));
        transcript.absorb_g(&C_ws);

        debug!("Round 1 took {} s", r1_now.elapsed().as_secs_f32());

        // -------------------- Round 2 --------------------
        let now = Instant::now();

        // ζ = H(transcript)
        // let zeta = transcript.challenge();
        // let p = &w.plookup.compute(&x.h, zeta);

        debug!("Round 2 took {} s", now.elapsed().as_secs_f32());

        // -------------------- Round 3 --------------------
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
            let one_index = (i + 1) % 8;
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

        // -------------------- Round 4 --------------------

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

        let f = f_gc + f_cc1.scale(&alpha) + f_cc2.scale(&alpha.pow([2]));
        let f_poly = f.interpolate();
        let (t, _) = f_poly.divide_by_vanishing_poly(trace.domain);

        let xi = transcript.challenge();
        assert_eq!(
            f_poly.evaluate(&xi),
            t.evaluate(&xi) * trace.domain.vanishing_polynomial().evaluate(&xi)
        );

        let ts = poly_split::<QUOTIENT_POLYS, P>(t.clone(), trace.rows);
        let C_ts: [Point<P>; QUOTIENT_POLYS] = array::from_fn(|i| commit(&ts[i], d, None));

        transcript.absorb_g(&C_ts);

        // -------------------- Round 5 --------------------

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
        Self {
            Cs: PlonkProofCommitments {
                ws: C_ws,
                ts: C_ts,
                z: C_z,
                r: C_r,
            },
            vs: PlonkProofEvals {
                ws: trace.w_polys.map(|w| w.evaluate(&xi)),
                qs: trace.q_polys.map(|q| q.evaluate(&xi)),
                ts: ts.map(|t| t.evaluate(&xi)),
                z: z_poly.evaluate(&xi),
                z_omega: z_poly.evaluate(&(xi * trace.omega)),
            },
            pis: PlonkProofEvalProofs {
                r: open(rng, r.clone(), C_r, d, &xi, None),
                r_omega: open(rng, z_poly, C_z, d, &(xi * trace.omega), None),
            },
        }
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
        let f_gc = pi.vs.ws[0] * pi.vs.qs[0]
            + pi.vs.ws[1] * pi.vs.qs[1]
            + pi.vs.ws[2] * pi.vs.qs[2]
            + pi.vs.ws[0] * pi.vs.ws[1] * pi.vs.qs[3]
            + pi.vs.qs[4]
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

fn poly_split<const N: usize, P: PastaConfig>(p: Poly<P>, n: usize) -> [Poly<P>; N] {
    let mut iter = p
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
