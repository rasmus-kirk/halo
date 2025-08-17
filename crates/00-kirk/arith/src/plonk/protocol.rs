#![allow(non_snake_case)]
use std::{
    array,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
    time::Instant,
};

use anyhow::{Result, ensure};
use halo_accumulation::{
    acc::{self, Accumulator},
    pcdl::{self, EvalProof, Instance, commit},
};
use halo_group::{
    Domain, Evals, PastaConfig, Point, Poly, Scalar,
    ark_ff::Field,
    ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial, univariate::DensePolynomial},
    ark_std::{One, Zero, rand::Rng},
};
use halo_poseidon::{Protocols, Sponge};
use log::debug;

use crate::{
    circuit::{PlonkCircuit, PlonkPublicInputs, PlonkWitness},
    utils::{
        CONSTRAINT_DEGREE_MULTIPLIER, Q_POLYS, R_POLYS, S_POLYS, T_POLYS, W_POLYS, fmt_scalar,
    },
};

#[derive(Clone)]
pub struct PlonkProofEvalProofs<P: PastaConfig> {
    pub r: EvalProof<P>,
    pub r_omega: EvalProof<P>,
}

#[derive(Clone)]
pub struct PlonkProofEvals<P: PastaConfig> {
    pub ws: [Scalar<P>; W_POLYS],
    pub rs: [Scalar<P>; R_POLYS],
    pub qs: [Scalar<P>; Q_POLYS],
    pub ts: [Scalar<P>; T_POLYS],
    pub ids: [Scalar<P>; S_POLYS],
    pub sigmas: [Scalar<P>; S_POLYS],
    pub z: Scalar<P>,
    pub z_omega: Scalar<P>,
    pub w_omegas: [Scalar<P>; 3],
}

#[derive(Clone)]
pub struct PlonkProofCommitments<P: PastaConfig> {
    pub ws: [Point<P>; W_POLYS],
    pub ts: [Point<P>; T_POLYS],
    pub z: Point<P>,
}

#[derive(Clone)]
pub struct PlonkProof<P: PastaConfig> {
    pub vs: PlonkProofEvals<P>,
    pub Cs: PlonkProofCommitments<P>,
    pub pis: PlonkProofEvalProofs<P>,
    pub acc_next: Accumulator<P>,
}

impl<P: PastaConfig> PlonkProof<P> {
    pub fn naive_prover<R: Rng>(
        rng: &mut R,
        circuit: PlonkCircuit<P>,
        public_inputs: &PlonkPublicInputs<P>,
        witness: PlonkWitness<P>,
    ) -> Self {
        let transcript = &mut Sponge::<P>::new(Protocols::PLONK);

        // -------------------- Round 0 --------------------

        let r0_now = Instant::now();

        let d = circuit.rows - 1;
        let domain = Domain::<P>::new(circuit.rows).unwrap();
        let mut public_inputs_clone = public_inputs.public_inputs.clone();
        public_inputs_clone.resize(circuit.rows, Scalar::<P>::zero());
        public_inputs_clone = public_inputs_clone.into_iter().map(|x| -x).collect();
        let public_inputs_evals = Evals::<P>::from_vec_and_domain(public_inputs_clone, domain);
        let public_inputs_poly = public_inputs_evals.interpolate();

        let w_omega_evals = array::from_fn(|i| witness.w_evals[i].clone().shift_left());
        let w_omegas = w_omega_evals.map(|w| w.interpolate());

        let large_domain = Domain::<P>::new(domain.size() * CONSTRAINT_DEGREE_MULTIPLIER).unwrap();
        let q_evals: [_; Q_POLYS] = array::from_fn(|i| {
            Evals::<P>::new(witness.polys.qs[i].evaluate_over_domain_by_ref(large_domain))
        });
        let w_evals: [_; W_POLYS] = array::from_fn(|i| {
            Evals::<P>::new(witness.polys.ws[i].evaluate_over_domain_by_ref(large_domain))
        });
        let r_evals: [_; R_POLYS] = array::from_fn(|i| {
            Evals::<P>::new(witness.polys.rs[i].evaluate_over_domain_by_ref(large_domain))
        });
        // let id_evals: [_; S_POLYS] = array::from_fn(|i| {
        //     Evals::<P>::new(witness.polys.ids[i].evaluate_over_domain_by_ref(large_domain))
        // });
        // let sigma_evals: [_; S_POLYS] = array::from_fn(|i| {
        //     Evals::<P>::new(witness.polys.sigmas[i].evaluate_over_domain_by_ref(large_domain))
        // });
        let w_omega_evals: [_; 3] =
            array::from_fn(|i| w_evals[i].clone().shift_left_small_domain(domain));
        let public_inputs_evals =
            Evals::<P>::new(public_inputs_poly.evaluate_over_domain_by_ref(large_domain));

        let r0_time = r0_now.elapsed().as_secs_f64();
        debug!("({}) Round 0 took {} s", P::SFID, r0_time);

        // -------------------- Round 1 --------------------
        let r1_now = Instant::now();

        let C_ws: [Point<P>; W_POLYS] = array::from_fn(|i| commit(&witness.polys.ws[i], d, None));
        transcript.absorb_g(&C_ws);

        let r1_time = r1_now.elapsed().as_secs_f64();
        debug!("({}) Round 1 took {} s", P::SFID, r1_time);

        // -------------------- Round 2 --------------------
        let r2_now = Instant::now();

        let r2_time = r2_now.elapsed().as_secs_f64();
        debug!("({}) Round 2 took {} s", P::SFID, r2_time);

        // -------------------- Round 3 --------------------
        let r3_now = Instant::now();

        let beta = transcript.challenge();
        let gamma = transcript.challenge();

        let mut f_prime = &witness.polys.ws[0] + &witness.polys.ids[0] * beta + deg0::<P>(gamma);
        let mut g_prime = &witness.polys.ws[0] + &witness.polys.sigmas[0] * beta + deg0::<P>(gamma);
        for i in 1..S_POLYS {
            f_prime =
                &f_prime * (&witness.polys.ws[i] + &witness.polys.ids[i] * beta + deg0::<P>(gamma));
            g_prime = &g_prime
                * (&witness.polys.ws[i] + &witness.polys.sigmas[i] * beta + deg0::<P>(gamma));
        }
        let f_prime_evals = Evals::<P>::new(f_prime.evaluate_over_domain_by_ref(domain));
        let g_prime_evals = Evals::<P>::new(g_prime.evaluate_over_domain_by_ref(domain));

        // Z
        let mut z = vec![P::ScalarField::zero(); circuit.rows];
        for i in 0..circuit.rows {
            let zero_index = i;
            let one_index = (i + 1) % circuit.rows;
            if one_index == 1 {
                z[zero_index] = P::ScalarField::one();
            } else {
                // TODO: Fix this disgusting indexing
                let ratio = f_prime_evals[zero_index] / g_prime_evals[zero_index];
                z[zero_index] = z[zero_index - 1] * ratio
            }
        }

        let z = Evals::<P>::from_vec_and_domain(z, domain);
        let z_omega = z.clone().shift_left().interpolate();
        let z = z.interpolate();

        let C_z = pcdl::commit(&z, circuit.rows - 1, None);
        transcript.absorb_g(&[C_z]);

        let r3_time = r3_now.elapsed().as_secs_f64();
        debug!("({}) Round 3 took {} s", P::SFID, r3_time);

        // -------------------- Round 4 --------------------
        let r4_now = Instant::now();

        let alpha = transcript.challenge();

        let poseidon_evals = poseidon_constraints_evals::<P>(
            P::SCALAR_POSEIDON_MDS,
            &r_evals,
            &w_evals,
            &w_omega_evals,
        );
        let affine_add_evals = affine_add_constraints_evals(&w_evals);
        let affine_mul_evals = affine_mul_constraints_evals(&w_evals, &w_omega_evals, &r_evals[0]);
        let eq_evals = eq_constraints_evals(&w_evals);
        let range_check_evals = range_check_constraints_evals(&w_evals, &w_omega_evals, &r_evals);

        let f_gc_evals: Evals<P> = &w_evals[0] * &q_evals[0]
            + &q_evals[1] * &w_evals[1]
            + &q_evals[2] * &w_evals[2]
            + &q_evals[3] * &w_evals[0] * &w_evals[1]
            + &q_evals[4]
            + &q_evals[5] * &poseidon_evals
            + &q_evals[6] * &affine_add_evals
            + &q_evals[7] * &affine_mul_evals
            + &q_evals[8] * &eq_evals
            + &q_evals[9] * &range_check_evals
            + &public_inputs_evals;

        let f_gc = f_gc_evals.interpolate();

        let l1 = lagrange_basis_poly::<P>(1, domain);
        let f_cc1 = l1 * (&z - deg0::<P>(Scalar::<P>::one()));
        let f_cc2 = &z * &f_prime - &z_omega * &g_prime;

        // let a = witness.polys.ws[0].evaluate_over_domain_by_ref(domain);
        // let b = witness.polys.ws[1].evaluate_over_domain_by_ref(domain);
        // let c = witness.polys.ws[2].evaluate_over_domain_by_ref(domain);
        // let ql = witness.polys.qs[0].evaluate_over_domain_by_ref(domain);
        // let qr = witness.polys.qs[1].evaluate_over_domain_by_ref(domain);
        // let qo = witness.polys.qs[2].evaluate_over_domain_by_ref(domain);
        // let qm = witness.polys.qs[3].evaluate_over_domain_by_ref(domain);
        // let qc = witness.polys.qs[4].evaluate_over_domain_by_ref(domain);
        // let q5 = witness.polys.qs[5].evaluate_over_domain_by_ref(domain);
        // let q6 = witness.polys.qs[6].evaluate_over_domain_by_ref(domain);
        // let q7 = witness.polys.qs[7].evaluate_over_domain_by_ref(domain);
        // let q8 = witness.polys.qs[8].evaluate_over_domain_by_ref(domain);
        // let q9 = witness.polys.qs[9].evaluate_over_domain_by_ref(domain);
        // let poseidon_eval = poseidon.evaluate_over_domain_by_ref(domain);
        // let affine_add_eval = affine_add.evaluate_over_domain_by_ref(domain);
        // let affine_mul_eval = affine_mul.evaluate_over_domain_by_ref(domain);
        // let eq_eval = eq.evaluate_over_domain_by_ref(domain);
        // let rangecheck_eval = rangecheck.evaluate_over_domain_by_ref(domain);
        // let pi_eval = public_inputs_poly.evaluate_over_domain_by_ref(domain);
        // let f_gc_eval = f_gc.evaluate_over_domain_by_ref(domain);
        // let f_cc1_eval = f_cc1.evaluate_over_domain_by_ref(domain);
        // let f_cc2_eval = f_cc2.evaluate_over_domain_by_ref(domain);
        // for i in 0..circuit.rows {
        //     println!(
        //         "{}: {}*{} + {}*{} + {}*{} + {}*{} + {} + {}*{} + {}*{} + {}*{} + {}*{} + {}*{} + {} = {}",
        //         P::SFID,
        //         fmt_scalar::<P>(ql[i]),
        //         fmt_scalar::<P>(ql[i] * a[i]),
        //         fmt_scalar::<P>(qr[i]),
        //         fmt_scalar::<P>(qr[i] * b[i]),
        //         fmt_scalar::<P>(qo[i]),
        //         fmt_scalar::<P>(qo[i] * c[i]),
        //         fmt_scalar::<P>(qm[i]),
        //         fmt_scalar::<P>(qm[i] * a[i] * b[i]),
        //         fmt_scalar::<P>(qc[i]),
        //         fmt_scalar::<P>(q5[i]),
        //         fmt_scalar::<P>(q5[i] * poseidon_eval[i]),
        //         fmt_scalar::<P>(q6[i]),
        //         fmt_scalar::<P>(q6[i] * affine_add_eval[i]),
        //         fmt_scalar::<P>(q7[i]),
        //         fmt_scalar::<P>(q7[i] * affine_mul_eval[i]),
        //         fmt_scalar::<P>(q8[i]),
        //         fmt_scalar::<P>(q8[i] * eq_eval[i]),
        //         fmt_scalar::<P>(q9[i]),
        //         fmt_scalar::<P>(q9[i] * rangecheck_eval[i]),
        //         fmt_scalar::<P>(pi_eval[i]),
        //         fmt_scalar::<P>(f_gc_eval[i])
        //     );

        //     assert_eq!(Scalar::<P>::zero(), f_gc_eval[i]);
        //     assert_eq!(Scalar::<P>::zero(), f_cc1_eval[i]);
        //     assert_eq!(Scalar::<P>::zero(), f_cc2_eval[i]);
        // }

        let f: Poly<P> = &f_gc + &f_cc1 * alpha + &f_cc2 * alpha.pow([2]);
        let (t, _) = f.divide_by_vanishing_poly(domain);

        // let ch = P::scalar_from_u64(rng.next_u64());
        // let zH = witness.domain.vanishing_polynomial();
        // assert_eq!(f.evaluate(&ch), t.evaluate(&ch) * zH.evaluate(&ch));

        let ts = t_split::<P>(t.clone(), circuit.rows);
        let C_ts: [Point<P>; T_POLYS] = array::from_fn(|i| commit(&ts[i], d, None));

        transcript.absorb_g(&C_ts);

        let r4_time = r4_now.elapsed().as_secs_f64();
        debug!("({}) Round 4 took {} s", P::SFID, r4_time);

        // -------------------- Round 5 --------------------
        let r5_now = Instant::now();

        let zeta = transcript.challenge();

        let mut vec = Vec::new();
        vec.extend_from_slice(&witness.polys.qs);
        vec.extend_from_slice(&witness.polys.ws);
        vec.extend_from_slice(&ts);
        vec.push(z.clone());
        let r = geometric_polys::<P>(zeta, vec);

        let mut vec = Vec::new();
        vec.extend_from_slice(&witness.polys.ws[0..3]);
        vec.push(z.clone());
        let r_omega = geometric_polys::<P>(zeta, vec);

        debug!(
            "({}) end of unoptimizable round 5 {} s",
            P::SFID,
            r5_now.elapsed().as_secs_f32()
        );

        let xi = transcript.challenge();
        let acc_prev = public_inputs.acc_prev.clone();
        let q_r = Instance::open(rng, r.clone(), d, &xi, None);
        let q_r_omega = Instance::open(rng, r_omega.clone(), d, &(xi * witness.omega), None);

        // acc_prev.q.check().unwrap();
        // q_r.check().unwrap();
        // q_r_omega.check().unwrap();

        let acc_next = acc::prover(
            rng,
            &[acc_prev.clone().into(), q_r.clone(), q_r_omega.clone()],
        )
        .unwrap();

        let pi = Self {
            Cs: PlonkProofCommitments {
                ws: C_ws,
                ts: C_ts,
                z: C_z,
            },
            vs: PlonkProofEvals {
                ws: witness.polys.ws.map(|w| w.evaluate(&xi)),
                rs: witness.polys.rs.map(|r| r.evaluate(&xi)),
                qs: witness.polys.qs.map(|q| q.evaluate(&xi)),
                ts: ts.map(|t| t.evaluate(&xi)),
                ids: witness.polys.ids.map(|id| id.evaluate(&xi)),
                sigmas: witness.polys.sigmas.map(|sigma| sigma.evaluate(&xi)),
                z: z.evaluate(&xi),
                z_omega: z.evaluate(&(xi * witness.omega)),
                w_omegas: w_omegas.map(|w_omega| w_omega.evaluate(&xi)),
            },
            pis: PlonkProofEvalProofs {
                r: q_r.pi,
                r_omega: q_r_omega.pi,
            },
            acc_next,
        };

        let r5_time = r5_now.elapsed().as_secs_f64();
        debug!("({}) Round 5 took {} s", P::SFID, r5_time);

        let total_time = r1_time + r2_time + r3_time + r4_time + r5_time;
        let r0_frac = r0_time / total_time * 100.0;
        let r1_frac = r1_time / total_time * 100.0;
        let r2_frac = r2_time / total_time * 100.0;
        let r3_frac = r3_time / total_time * 100.0;
        let r4_frac = r4_time / total_time * 100.0;
        let r5_frac = r5_time / total_time * 100.0;

        debug!(
            "({}) Fractions: | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% | {:>6.3}% |",
            P::SFID,
            r0_frac,
            r1_frac,
            r2_frac,
            r3_frac,
            r4_frac,
            r5_frac
        );

        pi
    }

    pub fn verify_succinct(
        &self,
        circuit: PlonkCircuit<P>,
        public_inputs: &PlonkPublicInputs<P>,
    ) -> Result<()> {
        let pi = self;
        let d = circuit.rows - 1;
        let n = P::scalar_from_u64(circuit.rows as u64);
        let one = Scalar::<P>::one();
        let mut transcript = Sponge::new(Protocols::PLONK);

        ensure!(public_inputs.public_inputs.len() == circuit.public_input_count);

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
        let xi = transcript.challenge();
        let xi_n = pow_n(xi, circuit.rows);
        let xi_omega = xi * circuit.omega;
        let ids = pi.vs.ids;
        let sigmas = pi.vs.sigmas;

        // f'(𝔷) = (A(𝔷) + β Sᵢ₁(𝔷) + γ) (B(𝔷) + β Sᵢ₂(𝔷) + γ) (C(𝔷) + β Sᵢ₃(𝔷) + γ)
        // g'(𝔷) = (A(𝔷)) + β S₁(𝔷)) + γ) (B(𝔷)) + β S₂(𝔷)) + γ) (C(𝔷)) + β S₃(𝔷)) + γ)
        let mut f_prime = pi.vs.ws[0] + beta * ids[0] + gamma;
        let mut g_prime = pi.vs.ws[0] + beta * sigmas[0] + gamma;
        for i in 1..S_POLYS {
            f_prime *= pi.vs.ws[i] + beta * ids[i] + gamma;
            g_prime *= pi.vs.ws[i] + beta * sigmas[i] + gamma;
        }

        // F_GC(𝔷) = A(𝔷)Qₗ(𝔷) + B(𝔷)Qᵣ(𝔷) + C(𝔷)Qₒ(𝔷) + A(𝔷)B(𝔷)Qₘ(𝔷) + Q꜀(𝔷) + PI(𝔷)
        let MDS = P::SCALAR_POSEIDON_MDS;
        let poseidon_terms =
            poseidon_constraints_generic(MDS, &pi.vs.rs, &pi.vs.ws, &pi.vs.w_omegas);
        let affine_add_terms = affine_add_constraints_generic(pi.vs.ws.clone());
        let affine_mul_terms =
            affine_mul_constraints_generic(pi.vs.ws, pi.vs.w_omegas, pi.vs.rs[0]);
        let eq = eq_generic(pi.vs.ws);
        let rangecheck = range_check_generic(pi.vs.ws, pi.vs.w_omegas, pi.vs.rs);

        let f_gc = pi.vs.ws[0] * pi.vs.qs[0]
            + pi.vs.ws[1] * pi.vs.qs[1]
            + pi.vs.ws[2] * pi.vs.qs[2]
            + pi.vs.ws[0] * pi.vs.ws[1] * pi.vs.qs[3]
            + pi.vs.qs[4]
            + pi.vs.qs[5] * poseidon_terms
            + pi.vs.qs[6] * affine_add_terms
            + pi.vs.qs[7] * affine_mul_terms
            + pi.vs.qs[8] * eq
            + pi.vs.qs[9] * rangecheck
            + public_input_eval_generic(&public_inputs.public_inputs, n, circuit.omega, xi, xi_n);

        let omega = circuit.omega;
        let l1 = (omega * (xi_n - one)) / (n * (xi - omega));
        let z_H = xi_n - one;
        let f_cc1 = l1 * (pi.vs.z - one);
        let f_cc2 = pi.vs.z * f_prime - pi.vs.z_omega * g_prime;

        let f = f_gc + alpha * f_cc1 + alpha.pow([2]) * f_cc2;
        let t = t_reconstruct_generic(pi.vs.ts, xi_n);

        ensure!(
            f == t * z_H,
            "T(𝔷) ≠ (F_GC(𝔷) + α F_CC1(𝔷) + α² F_CC2(𝔷) + α³ F_PL1(𝔷) + α⁴ F_PL2(𝔷)) / Zₕ(𝔷)"
        );

        // let pp = PublicParams::get_pp();
        // let mut acc: Point<P> = (Affine::identity()).into();
        // for i in 0..public_inputs.public_inputs.len() {
        //     acc += pp.Gs[i] * public_inputs.public_inputs[i];
        // }
        // ensure!(circuit.Cs.public_input == acc);

        let mut vec = Vec::new();
        vec.extend_from_slice(&pi.vs.qs);
        vec.extend_from_slice(&pi.vs.ws);
        vec.extend_from_slice(&pi.vs.ts);
        vec.push(pi.vs.z);
        let v_r = geometric_generic(zeta, vec);

        let mut vec = Vec::new();
        vec.extend_from_slice(&pi.vs.w_omegas);
        vec.push(pi.vs.z_omega);
        let v_r_omega = geometric_generic(zeta, vec);

        let mut vec = Vec::new();
        vec.extend_from_slice(&circuit.Cs.qs);
        vec.extend_from_slice(&pi.Cs.ws);
        vec.extend_from_slice(&pi.Cs.ts);
        vec.push(pi.Cs.z);
        let C_r = geometric_generic(zeta, vec);

        let mut vec = Vec::new();
        vec.extend_from_slice(&pi.Cs.ws[0..3]);
        vec.push(pi.Cs.z);
        let C_r_omega = geometric_generic(zeta, vec);

        let instance_1 = Instance::new(C_r, d, xi, v_r, pi.pis.r.clone());
        let instance_2 = Instance::new(C_r_omega, d, xi_omega, v_r_omega, pi.pis.r_omega.clone());

        let acc_prev = public_inputs.acc_prev.clone();

        // let _ = instance_1.check()?;
        // let _ = instance_2.check()?;
        // let _ = Instance::from(acc_prev.clone()).check()?;

        let acc_next = pi.acc_next.clone();
        let qs = [acc_prev.into(), instance_1, instance_2];
        acc::verifier(&qs, acc_next)?;

        Ok(())
    }

    pub fn verify(
        &self,
        circuit: PlonkCircuit<P>,
        public_inputs: &PlonkPublicInputs<P>,
    ) -> Result<()> {
        let acc_next = self.acc_next.clone();
        self.verify_succinct(circuit, public_inputs)?;
        acc::decider(acc_next)?;
        Ok(())
    }
}

fn deg0<P: PastaConfig>(x: Scalar<P>) -> Poly<P> {
    Poly::<P>::from_coefficients_vec(vec![x])
}

fn t_split<P: PastaConfig>(mut t: Poly<P>, n: usize) -> [Poly<P>; T_POLYS] {
    assert!(t.degree() < T_POLYS * n, "{} < {}", t.degree(), T_POLYS * n);
    t.coeffs.resize(T_POLYS * n, Scalar::<P>::zero());
    let mut iter = t
        .coeffs
        .chunks(n as usize)
        .map(DensePolynomial::from_coefficients_slice);
    array::from_fn(|_| iter.next().unwrap())
}

pub fn t_reconstruct_generic<T>(ts: [T; T_POLYS], challenge_pow_n: T) -> T
where
    T: Copy + AddAssign + Mul<Output = T> + MulAssign,
{
    let mut result = ts[0];
    let mut acc = challenge_pow_n;
    for i in 1..ts.len() {
        result += acc * ts[i];
        acc *= challenge_pow_n
    }
    result
}

pub fn pow_n<T>(mut x: T, n: usize) -> T
where
    T: Copy + MulAssign,
{
    for _ in 0..n.ilog2() {
        x *= x
    }
    x
}

fn geometric_polys<P: PastaConfig>(zeta: Scalar<P>, vec: Vec<Poly<P>>) -> Poly<P> {
    let mut result = Poly::<P>::zero();
    for (i, p) in vec.into_iter().enumerate() {
        result += &(p * zeta.pow([i as u64]))
    }
    result
}

pub fn geometric_generic<T, U>(x: T, vec: Vec<U>) -> U
where
    T: Copy + Add<Output = T> + AddAssign + Mul<Output = T> + MulAssign,
    U: Copy + Add<Output = U> + AddAssign + Mul<T, Output = U>,
{
    let mut result = vec[0];
    let mut acc = x;
    for scalar in vec.into_iter().skip(1) {
        result += scalar * acc;
        acc *= x;
    }
    result
}

pub fn public_input_eval_generic<T>(public_inputs: &[T], n: T, omega: T, xi: T, xi_n: T) -> T
where
    T: Copy
        + Add<Output = T>
        + AddAssign
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAssign
        + Zero
        + Neg<Output = T>
        + One,
{
    let one = T::one();

    // (xi_n - one) * omega_j / n * (xi - omega_j)
    let mut omega_j = omega;
    let mut public_input_xi = T::zero();
    for x in public_inputs {
        let l_j = ((xi_n - one) * omega_j) / (n * (xi - omega_j));
        public_input_xi += l_j * (-(*x));
        omega_j *= omega;
    }

    public_input_xi
}

fn poseidon_constraints_evals<P: PastaConfig>(
    M: [[Scalar<P>; 3]; 3],
    r: &[Evals<P>; R_POLYS],
    w: &[Evals<P>; W_POLYS],
    nw: &[Evals<P>; 3],
) -> Evals<P> {
    // TODO
    let sbox = |w: &Evals<P>| w * w * w * w * w * w * w;
    #[rustfmt::skip]
    let round = |w0,w1,w2,w3,w4,w5,r0,r1,r2| {
          w3 - (r0 + sbox(w0).scale(&M[0][0]) + sbox(w1).scale(&M[0][1]) + sbox(w2).scale(&M[0][2]))
        + w4 - (r1 + sbox(w0).scale(&M[1][0]) + sbox(w1).scale(&M[1][1]) + sbox(w2).scale(&M[1][2]))
        + w5 - (r2 + sbox(w0).scale(&M[2][0]) + sbox(w1).scale(&M[2][1]) + sbox(w2).scale(&M[2][2]))
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

pub(crate) fn poseidon_constraints_generic<T>(
    M: [[T; 3]; 3],
    r: &[T; R_POLYS],
    w: &[T; W_POLYS],
    nw: &[T; 3],
) -> T
where
    T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + Mul<Output = T>,
{
    let pow7 = |w| w * w * w * w * w * w * w;
    let sbox = |w: T| pow7(w);
    #[rustfmt::skip]
    let round = |w0: T, w1: T, w2: T, w3: T,w4: T, w5: T, r0: T, r1: T, r2: T| {
          w3 - (r0 + sbox(w0) * M[0][0] + sbox(w1) * M[0][1] + sbox(w2) * M[0][2])
        + w4 - (r1 + sbox(w0) * M[1][0] + sbox(w1) * M[1][1] + sbox(w2) * M[1][2])
        + w5 - (r2 + sbox(w0) * M[2][0] + sbox(w1) * M[2][1] + sbox(w2) * M[2][2])
    };
    let round_1 = round(w[0], w[1], w[2], w[3], w[4], w[5], r[0], r[1], r[2]);
    let round_2 = round(w[3], w[4], w[5], w[6], w[7], w[8], r[3], r[4], r[5]);
    let round_3 = round(w[6], w[7], w[8], w[9], w[10], w[11], r[6], r[7], r[8]);
    let round_4 = round(w[9], w[10], w[11], w[12], w[13], w[14], r[9], r[10], r[11]);
    let round_5 = round(
        w[12], w[13], w[14], nw[0], nw[1], nw[2], r[12], r[13], r[14],
    );
    round_1 + round_2 + round_3 + round_4 + round_5
}

fn affine_add_constraints_evals<P: PastaConfig>(w: &[Evals<P>; W_POLYS]) -> Evals<P> {
    let one = Evals::<P>::one(w[0].domain());
    let [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, _, _, _, _, _] = w;

    // (xq - xp) · ((xq - xp) · λ - (yq - yp))
    let xq一xp = xq - xp;
    let yq一yp = yq - yp;
    let mut result = (&xq一xp) * (&xq一xp * λ - yq一yp);

    // (1 - (xq - xp) · α) · (2yp · λ - 3xp²)
    let yp·2 = yp + yp;
    let xp·xp = xp * xp;
    let 〡xp·xp〡·3 = &xp·xp + &xp·xp + xp·xp;
    result += (&one - (xq一xp) * α) * (yp·2 * λ - 〡xp·xp〡·3);

    // xp · xq · (xq - xp) · (λ² - xp - xq - xr)
    let xp·xq = xp * xq;
    let xp·xq·〡xq一xp〡 = &xp·xq * (xq - xp);
    let λ·λ = λ * λ;
    let λ·λ一xp一xq一xr = λ·λ - xp - xq - xr;
    result += &xp·xq·〡xq一xp〡 * &λ·λ一xp一xq一xr;

    // xp · xq · (xq - xp) · (λ · (xp - xr) - yp - yr)
    let λ·〡xp一xr〡一yp一yr = λ * (xp - xr) - yp - yr;
    result += xp·xq·〡xq一xp〡 * &λ·〡xp一xr〡一yp一yr;

    // xp · xq · (yq + yp) · (λ² - xp - xq - xr)
    let xp·xq·〡yq〸yp〡 = xp·xq * (yq + yp);
    result += &xp·xq·〡yq〸yp〡 * λ·λ一xp一xq一xr;

    // xq · (yq + yp) · (λ · (xp - xr) - yp - yr)
    result += xp·xq·〡yq〸yp〡 * λ·〡xp一xr〡一yp一yr;

    // (1 - xp · β) · (xr - xq)
    let l一xp·β = &one - xp * β;
    result += &l一xp·β * (xr - xq);

    // (1 - xp · β) · (yr - yq)
    result += l一xp·β * (yr - yq);

    // (1 - xq · γ) · (xr - xp)
    let l一xq·γ = &one - xq * γ;
    result += &l一xq·γ * (xr - xp);

    // (1 - xq · γ) · (yr - yp)
    result += l一xq·γ * (yr - yp);

    // (1 - (xq - xp) · α - (yq + yp) · δ) · xr
    let l一〡xq一xp〡·α一〡yq〸yp〡·δ = one - (xq - xp) * α - (yq + yp) * δ;
    result += (&l一〡xq一xp〡·α一〡yq〸yp〡·δ) * xr;

    // (1 - (xq · xp) · α - (yq + yp) · δ) · yr
    result + (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * yr
}

pub(crate) fn affine_add_constraints_generic<T>(w: [T; W_POLYS]) -> T
where
    T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + Mul<Output = T> + One,
{
    let one = T::one();
    let [xp, yp, xq, yq, xr, yr, α, β, γ, δ, λ, _, _, _, _, _] = w;

    // (xq - xp) · ((xq - xp) · λ - (yq - yp))
    let xq一xp = xq - xp;
    let yq一yp = yq - yp;
    let mut result = (xq一xp) * (xq一xp * λ - yq一yp);

    // (1 - (xq - xp) · α) · (2yp · λ - 3xp²)
    let yp·2 = yp + yp;
    let xp·xp = xp * xp;
    let 〡xp·xp〡·3 = xp·xp + xp·xp + xp·xp;
    result += (one - (xq一xp) * α) * (yp·2 * λ - 〡xp·xp〡·3);

    // xp · xq · (xq - xp) · (λ² - xp - xq - xr)
    let xp·xq = xp * xq;
    let xp·xq·〡xq一xp〡 = xp·xq * (xq - xp);
    let λ·λ = λ * λ;
    let λ·λ一xp一xq一xr = λ·λ - xp - xq - xr;
    result += xp·xq·〡xq一xp〡 * λ·λ一xp一xq一xr;

    // xp · xq · (xq - xp) · (λ · (xp - xr) - yp - yr)
    let λ·〡xp一xr〡一yp一yr = λ * (xp - xr) - yp - yr;
    result += xp·xq·〡xq一xp〡 * λ·〡xp一xr〡一yp一yr;

    // xp · xq · (yq + yp) · (λ² - xp - xq - xr)
    let xp·xq·〡yq〸yp〡 = xp·xq * (yq + yp);
    result += xp·xq·〡yq〸yp〡 * λ·λ一xp一xq一xr;

    // xq · (yq + yp) · (λ · (xp - xr) - yp - yr)
    result += xp·xq·〡yq〸yp〡 * λ·〡xp一xr〡一yp一yr;

    // (1 - xp · β) · (xr - xq)
    let l一xp·β = one - xp * β;
    result += l一xp·β * (xr - xq);

    // (1 - xp · β) · (yr - yq)
    result += l一xp·β * (yr - yq);

    // (1 - xq · γ) · (xr - xp)
    let l一xq·γ = one - xq * γ;
    result += l一xq·γ * (xr - xp);

    // (1 - xq · γ) · (yr - yp)
    result += l一xq·γ * (yr - yp);

    // (1 - (xq - xp) · α - (yq + yp) · δ) · xr
    let l一〡xq一xp〡·α一〡yq〸yp〡·δ = one - (xq - xp) * α - (yq + yp) * δ;
    result += (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * xr;

    // (1 - (xq · xp) · α - (yq + yp) · δ) · yr
    result + (l一〡xq一xp〡·α一〡yq〸yp〡·δ) * yr
}

fn affine_mul_constraints_evals<P: PastaConfig>(
    w: &[Evals<P>; W_POLYS],
    nw: &[Evals<P>; 3],
    two_pow_i: &Evals<P>,
) -> Evals<P> {
    let one = Evals::one(w[0].domain());
    let [xp, yp, a, xg, yg, b, xq, yq, xr, yr, βq, λq, αr, γr, δr, λr] = w;

    let xp·xp = xp * xp;
    let xp·2 = xp + xp;
    let λ·λ = λq * λq;
    let 〡xp·xp〡·3 = &xp·xp + &xp·xp + xp·xp;
    let yp·2 = yp + yp;

    let mut result = (&one - xp * βq) * xq;
    result += (&one - xp * βq) * yq;

    result += yp·2 * λq - 〡xp·xp〡·3;
    result += λ·λ - xp·2 - xq;
    result += λq * (xp - xq) - yp - yq;

    // // ----- R = Q + G ----- //

    // // (xg - xq) · ((xg - xq) · λ - (yg - yq))
    let xg一xq = xg - xq;
    let yg一yq = yg - yq;
    result += (&xg一xq) * (&xg一xq * λr - yg一yq);

    // (1 - (xg - xq) · α) · (2yq · λ - 3xq²)
    let yq·2 = yq + yq;
    let xq·xq = xq * xq;
    let 〡xq·xq〡·3 = &xq·xq + &xq·xq + xq·xq;
    result += (&one - (xg一xq) * αr) * (yq·2 * λr - 〡xq·xq〡·3);

    // xq · xg · (xg - xq) · (λ² - xq - xg - xr)
    let xq·xg = xq * xg;
    let xq·xg·〡xg一xq〡 = &xq·xg * (xg - xq);
    let λ·λ = λr * λr;
    let λ·λ一xq一xg一xr = λ·λ - xq - xg - xr;
    result += &xq·xg·〡xg一xq〡 * &λ·λ一xq一xg一xr;

    // xq · xg · (xg - xq) · (λ · (xq - xr) - yq - yr)
    let λ·〡xq一xr〡一yq一yr = λr * (xq - xr) - yq - yr;
    result += xq·xg·〡xg一xq〡 * &λ·〡xq一xr〡一yq一yr;

    // xq · xg · (yg + yq) · (λ² - xq - xg - xr)
    let xq·xg·〡yg〸yq〡 = xq·xg * (yg + yq);
    result += &xq·xg·〡yg〸yq〡 * λ·λ一xq一xg一xr;

    // xg · (yg + yq) · (λ · (xq - xr) - yq - yr)
    result += xq·xg·〡yg〸yq〡 * λ·〡xq一xr〡一yq一yr;

    // (1 - xq · β) · (xr - xg)
    let l一xp·β = &one - xp * βq;
    result += &l一xp·β * (xr - xg);

    // (1 - xq · β) · (yr - yg)
    result += l一xp·β * (yr - yg);

    // (1 - xg · γ) · (xr - xq)
    let l一xg·γ = &one - xg * γr;
    result += &l一xg·γ * (xr - xq);

    // (1 - xg · γ) · (yr - yq)
    result += l一xg·γ * (yr - yq);

    // (1 - (xg - xq) · α - (yg + yq) · δ) · xr
    let l一〡xg一xq〡·α一〡yg〸yq〡·δ = &one - (xg - xq) * αr - (yg + yq) * δr;
    result += (&l一〡xg一xq〡·α一〡yg〸yq〡·δ) * xr;

    // (1 - (xg · xq) · α - (yg + yq) · δ) · yr
    result += (l一〡xg一xq〡·α一〡yg〸yq〡·δ) * yr;

    // // ----- b is a bit ----- //
    result += b * (b - &one);

    // // ----- S = b ? R : Q ---- //
    let xs = &nw[0];
    let ys = &nw[1];
    result += xs - (b * xr + (&one - b) * xq);
    result += ys - (b * yr + (&one - b) * yq);

    // // ----- bit_acc_next = bit_acc + b * 2^i ----- //
    let bit_acc = a;
    let bit_acc_next = &nw[2];
    result + bit_acc_next - (bit_acc + b * two_pow_i)
}

pub(crate) fn affine_mul_constraints_generic<T>(w: [T; W_POLYS], nw: [T; 3], two_pow_i: T) -> T
where
    T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + Mul<Output = T> + One,
{
    let one = T::one();
    let [xp, yp, a, xg, yg, b, xq, yq, xr, yr, βq, λq, αr, γr, δr, λr] = w;

    let xp·xp = xp * xp;
    let xp·2 = xp + xp;
    let λ·λ = λq * λq;
    let 〡xp·xp〡·3 = xp·xp + xp·xp + xp·xp;
    let yp·2 = yp + yp;

    let mut result = (one - xp * βq) * xq;
    result += (one - xp * βq) * yq;

    result += yp·2 * λq - 〡xp·xp〡·3;
    result += λ·λ - xp·2 - xq;
    result += λq * (xp - xq) - yp - yq;

    // ----- R = Q + G ----- //

    // (xg - xq) · ((xg - xq) · λ - (yg - yq))
    let xg一xq = xg - xq;
    let yg一yq = yg - yq;
    result += (xg一xq) * (xg一xq * λr - yg一yq);

    // (1 - (xg - xq) · α) · (2yq · λ - 3xq²)
    let yq·2 = yq + yq;
    let xq·xq = xq * xq;
    let 〡xq·xq〡·3 = xq·xq + xq·xq + xq·xq;
    result += (one - (xg一xq) * αr) * (yq·2 * λr - 〡xq·xq〡·3);

    // xq · xg · (xg - xq) · (λ² - xq - xg - xr)
    let xq·xg = xq * xg;
    let xq·xg·〡xg一xq〡 = xq·xg * (xg - xq);
    let λ·λ = λr * λr;
    let λ·λ一xq一xg一xr = λ·λ - xq - xg - xr;
    result += xq·xg·〡xg一xq〡 * λ·λ一xq一xg一xr;

    // xq · xg · (xg - xq) · (λ · (xq - xr) - yq - yr)
    let λ·〡xq一xr〡一yq一yr = λr * (xq - xr) - yq - yr;
    result += xq·xg·〡xg一xq〡 * λ·〡xq一xr〡一yq一yr;

    // xq · xg · (yg + yq) · (λ² - xq - xg - xr)
    let xq·xg·〡yg〸yq〡 = xq·xg * (yg + yq);
    result += xq·xg·〡yg〸yq〡 * λ·λ一xq一xg一xr;

    // xg · (yg + yq) · (λ · (xq - xr) - yq - yr)
    result += xq·xg·〡yg〸yq〡 * λ·〡xq一xr〡一yq一yr;

    // (1 - xq · β) · (xr - xg)
    let l一xp·β = one - xp * βq;
    result += l一xp·β * (xr - xg);

    // (1 - xq · β) · (yr - yg)
    result += l一xp·β * (yr - yg);

    // (1 - xg · γ) · (xr - xq)
    let l一xg·γ = one - xg * γr;
    result += l一xg·γ * (xr - xq);

    // (1 - xg · γ) · (yr - yq)
    result += l一xg·γ * (yr - yq);

    // (1 - (xg - xq) · α - (yg + yq) · δ) · xr
    let l一〡xg一xq〡·α一〡yg〸yq〡·δ = one - (xg - xq) * αr - (yg + yq) * δr;
    result += (l一〡xg一xq〡·α一〡yg〸yq〡·δ) * xr;

    // (1 - (xg · xq) · α - (yg + yq) · δ) · yr
    result += (l一〡xg一xq〡·α一〡yg〸yq〡·δ) * yr;

    // ----- b is a bit ----- //
    result += b * (b - one);

    // ----- S = b ? R : Q ---- //
    let xs = nw[0];
    let ys = nw[1];

    result += xs - (b * xr + (one - b) * xq);
    result += ys - (b * yr + (one - b) * yq);

    // ----- bit_acc_next = bit_acc + b * 2^i ----- //
    let bit_acc = a;
    let bit_acc_next = nw[2];
    result + bit_acc_next - (bit_acc + b * two_pow_i)
}

pub(crate) fn range_check_constraints_evals<P: PastaConfig>(
    w: &[Evals<P>; W_POLYS],
    nw: &[Evals<P>; 3],
    r: &[Evals<P>; R_POLYS],
) -> Evals<P> {
    let [acc_prev, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14] = w;
    let acc_next = nw[0].clone();

    let mut result = acc_next;
    result -= acc_prev;
    result -= &(b0 * &r[0]);
    result -= &(b1 * &r[1]);
    result -= &(b2 * &r[2]);
    result -= &(b3 * &r[3]);
    result -= &(b4 * &r[4]);
    result -= &(b5 * &r[5]);
    result -= &(b6 * &r[6]);
    result -= &(b7 * &r[7]);
    result -= &(b8 * &r[8]);
    result -= &(b9 * &r[9]);
    result -= &(b10 * &r[10]);
    result -= &(b11 * &r[11]);
    result -= &(b12 * &r[12]);
    result -= &(b13 * &r[13]);
    result - &(b14 * &r[14])
}

pub(crate) fn range_check_generic<T>(w: [T; W_POLYS], nw: [T; 3], r: [T; R_POLYS]) -> T
where
    T: Copy + SubAssign + Sub<Output = T> + Mul<Output = T>,
{
    let [acc_prev, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14] = w;
    let acc_next = nw[0];

    let mut result = acc_next;
    result -= acc_prev;
    result -= b0 * r[0];
    result -= b1 * r[1];
    result -= b2 * r[2];
    result -= b3 * r[3];
    result -= b4 * r[4];
    result -= b5 * r[5];
    result -= b6 * r[6];
    result -= b7 * r[7];
    result -= b8 * r[8];
    result -= b9 * r[9];
    result -= b10 * r[10];
    result -= b11 * r[11];
    result -= b12 * r[12];
    result -= b13 * r[13];
    result - b14 * r[14]
}

pub(crate) fn eq_constraints_evals<P: PastaConfig>(w: &[Evals<P>; W_POLYS]) -> Evals<P> {
    let [a, b, one, eq, inv, _, _, _, _, _, _, _, _, _, _, _] = w;

    let mut result = (a - b) * eq;
    result += (a - b) * inv + eq - one;

    result
}

pub(crate) fn eq_generic<T>(w: [T; W_POLYS]) -> T
where
    T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + Mul<Output = T> + MulAssign,
{
    let [a, b, one, eq, inv, _, _, _, _, _, _, _, _, _, _, _] = w;

    let mut result = (a - b) * eq;
    result += (a - b) * inv + eq - one;

    result
}

fn lagrange_basis_poly<P: PastaConfig>(i: usize, small_domain: Domain<P>) -> Poly<P> {
    let mut evals = vec![Scalar::<P>::zero(); small_domain.size()];
    evals[i - 1] = Scalar::<P>::one();
    Evals::<P>::from_vec_and_domain(evals, small_domain).interpolate()
}

#[cfg(test)]
mod tests {
    use std::array;

    use anyhow::Result;
    use halo_group::{
        Domain, Evals, PallasConfig, PastaConfig, Poly, Scalar,
        ark_ff::UniformRand,
        ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial, univariate::DensePolynomial},
        ark_std::rand::{Rng, thread_rng},
    };

    use crate::{
        plonk::eq_constraints_evals,
        utils::{CONSTRAINT_DEGREE_MULTIPLIER, R_POLYS, W_POLYS},
    };

    #[test]
    fn evals_add() -> Result<()> {
        let rng = &mut thread_rng();

        let domain_size = 2usize.pow(rng.gen_range(5..=20));
        let domain = Domain::<PallasConfig>::new(domain_size).unwrap();

        let a_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let b_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let a_eval = Evals::<PallasConfig>::new(a_poly.evaluate_over_domain_by_ref(domain));
        let b_eval = Evals::<PallasConfig>::new(b_poly.evaluate_over_domain_by_ref(domain));

        let sum_eval = a_eval + b_eval;
        let sum_fft = sum_eval.interpolate();
        let sum_poly = &a_poly + &b_poly;

        assert_eq!(sum_poly, sum_fft);

        Ok(())
    }

    #[test]
    fn evals_sub() -> Result<()> {
        let rng = &mut thread_rng();

        let domain_size = 2usize.pow(rng.gen_range(5..=20));
        let domain = Domain::<PallasConfig>::new(domain_size).unwrap();

        let a_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let b_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let a_eval = Evals::<PallasConfig>::new(a_poly.evaluate_over_domain_by_ref(domain));
        let b_eval = Evals::<PallasConfig>::new(b_poly.evaluate_over_domain_by_ref(domain));

        let sum_eval = a_eval - b_eval;
        let sum_fft = sum_eval.interpolate();
        let sum_poly = &a_poly - &b_poly;

        assert_eq!(sum_poly, sum_fft);

        Ok(())
    }

    #[test]
    fn evals_mul() -> Result<()> {
        let rng = &mut thread_rng();

        let domain_size = 2usize.pow(rng.gen_range(5..=10));
        let large_domain = Domain::<PallasConfig>::new(domain_size * 2).unwrap();

        let a_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let b_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let a_eval = Evals::<PallasConfig>::new(a_poly.evaluate_over_domain_by_ref(large_domain));
        let b_eval = Evals::<PallasConfig>::new(b_poly.evaluate_over_domain_by_ref(large_domain));

        let sum_eval = &a_eval * &b_eval;
        let sum_fft = sum_eval.interpolate();
        let sum_poly = &a_poly * &b_poly;

        assert_eq!(sum_poly, sum_fft);

        Ok(())
    }

    #[test]
    fn evals_scale() -> Result<()> {
        let rng = &mut thread_rng();

        let domain_size = 2usize.pow(rng.gen_range(5..=10));
        let large_domain = Domain::<PallasConfig>::new(domain_size * 2).unwrap();

        let a_poly = DensePolynomial::<Scalar<PallasConfig>>::rand(domain_size - 1, rng);
        let b = Scalar::<PallasConfig>::rand(rng);

        let a_eval = Evals::<PallasConfig>::new(a_poly.evaluate_over_domain_by_ref(large_domain));

        let scale_eval = a_eval.scale(&b);
        let scale_fft = scale_eval.interpolate();
        let scale_poly = &a_poly * b;

        assert_eq!(scale_poly, scale_fft);

        Ok(())
    }

    // #[test]
    // fn evals_poseidon() -> Result<()> {
    //     type P = PallasConfig;
    //     let rng = &mut thread_rng();
    //     let domain_size = 2usize.pow(rng.gen_range(3..=5));
    //     let large_domain = Domain::<P>::new(domain_size * CONSTRAINT_DEGREE_MULTIPLIER).unwrap();
    //     let M = P::SCALAR_POSEIDON_MDS;

    //     let w: [Poly<P>; W_POLYS] =
    //         array::from_fn(|_| DenseUVPolynomial::rand(domain_size - 1, rng));
    //     let q: [Poly<P>; W_POLYS] =
    //         array::from_fn(|_| DenseUVPolynomial::rand(domain_size - 1, rng));
    //     let w_omegas: [Poly<P>; 3] =
    //         array::from_fn(|_| DenseUVPolynomial::rand(domain_size - 1, rng));
    //     let r: [Poly<P>; R_POLYS] =
    //         array::from_fn(|_| DenseUVPolynomial::rand(domain_size - 1, rng));
    //     let public_inputs: Poly<P> = DenseUVPolynomial::rand(domain_size - 1, rng);
    //     let q_evals: [_; W_POLYS] =
    //         array::from_fn(|i| Evals::<P>::new(q[i].evaluate_over_domain_by_ref(large_domain)));
    //     let w_evals: [_; W_POLYS] =
    //         array::from_fn(|i| Evals::<P>::new(w[i].evaluate_over_domain_by_ref(large_domain)));
    //     let r_evals: [_; R_POLYS] =
    //         array::from_fn(|i| Evals::<P>::new(r[i].evaluate_over_domain_by_ref(large_domain)));
    //     let w_omega_evals: [_; 3] = array::from_fn(|i| {
    //         Evals::<P>::new(w_omegas[i].evaluate_over_domain_by_ref(large_domain))
    //     });
    //     let public_inputs_eval =
    //         Evals::<P>::new(public_inputs.evaluate_over_domain_by_ref(large_domain));

    //     let poseidon_evals = poseidon_constraints_evals::<P>(M, &r_evals, &w_evals, &w_omega_evals);
    //     let affine_add_evals = affine_add_constraints_evals(&w_evals);
    //     let affine_mul_evals = affine_mul_constraints_evals(&w_evals, &w_omega_evals, &r_evals[0]);
    //     let eq_evals = eq_constraints_evals(&w_evals);
    //     let range_check_evals = range_check_constraints_evals(&w_evals, &w_omega_evals, &r_evals);

    //     let poseidon_poly = poseidon_evals.interpolate_by_ref();
    //     let affine_add_poly = affine_add_evals.interpolate_by_ref();
    //     let affine_mul_poly = affine_mul_evals.interpolate_by_ref();
    //     let eq_poly = eq_evals.interpolate_by_ref();
    //     let range_check_poly = range_check_evals.interpolate_by_ref();

    //     let poseidon = poseidon_constraints_poly::<P>(P::SCALAR_POSEIDON_MDS, &r, &w, &w_omegas);
    //     let affine_add = affine_add_constraints_poly::<P>(&w);
    //     let affine_mul = affine_mul_constraints_poly::<P>(&w, &w_omegas, &r[0]);
    //     let rangecheck = range_check_constraints_poly::<P>(&w, &w_omegas, &r);

    //     assert!(poseidon.degree() < large_domain.size());
    //     assert!(affine_add.degree() < large_domain.size());
    //     assert!(affine_mul.degree() < large_domain.size());
    //     assert!(rangecheck.degree() < large_domain.size());
    //     assert_eq!(poseidon, poseidon_poly);
    //     assert_eq!(affine_add, affine_add_poly);
    //     assert_eq!(affine_mul, affine_mul_poly);
    //     assert_eq!(rangecheck, range_check_poly);

    //     let f_gc: Poly<P> = &w[0] * &q[0]
    //         + &q[1] * &w[1]
    //         + &q[2] * &w[2]
    //         + &q[3] * &w[0] * &w[1]
    //         + &q[4]
    //         + &q[5] * &poseidon
    //         + &q[6] * &affine_add
    //         + &q[7] * &affine_mul
    //         + &q[8] * &eq
    //         + &q[9] * &rangecheck
    //         + &public_inputs;

    //     let f_gc_evals: Evals<P> = &w_evals[0] * &q_evals[0]
    //         + &q_evals[1] * &w_evals[1]
    //         + &q_evals[2] * &w_evals[2]
    //         + &q_evals[3] * &w_evals[0] * &w_evals[1]
    //         + &q_evals[4]
    //         + &q_evals[5] * &poseidon_evals
    //         + &q_evals[6] * &affine_add_evals
    //         + &q_evals[7] * &affine_mul_evals
    //         + &q_evals[8] * &eq_evals
    //         + &q_evals[9] * &range_check_evals
    //         + &public_inputs_eval;

    //     let f_gc_poly = f_gc_evals.interpolate();
    //     assert_eq!(f_gc_poly, f_gc);

    //     Ok(())
    // }
}
