use std::array;

use anyhow::Result;
use halo_group::{
    Domain, PastaConfig, Scalar, ark_ec::CurveGroup, ark_poly::EvaluationDomain, ark_std::Zero,
};
use halo_poseidon::Protocols;

use crate::{
    circuit::{PlonkCircuit, PlonkCircuitCommitments, PlonkPublicInputs},
    frontend::{
        Call,
        asdl::{CallAccumulator, WireAccumulator},
        pcdl::{CallInstance, WireEvalProof, WireInstance, WirePublicParams},
        poseidon::outer_sponge::OuterSponge,
        primitives::{WireAffine, WireBool, WireScalar},
    },
    plonk::{
        PlonkProof, affine_add_constraints_generic, affine_mul_constraints_generic, eq_generic,
        geometric_generic, poseidon_constraints_generic, pow_n, public_input_eval_generic,
        range_check_generic, t_reconstruct_generic,
    },
    utils::{Q_POLYS, R_POLYS, S_POLYS, T_POLYS, W_POLYS},
};

pub trait CallPlonk {
    fn witness_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: &WirePlonkProof<P>,
        proof: &PlonkProof<P>,
    ) -> Result<()>;
    fn public_input_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: WirePlonkProof<P>,
        proof: PlonkProof<P>,
    ) -> Result<()>;
    fn witness_plonk_public_input<P: PastaConfig>(
        &mut self,
        wire_public_input: &WirePlonkPublicInputs<P>,
        plonk_public_input: &PlonkPublicInputs<P>,
    ) -> Result<()>;
    fn public_input_plonk_public_input<P: PastaConfig>(
        &mut self,
        wire_public_input: &WirePlonkPublicInputs<P>,
        plonk_public_input: &PlonkPublicInputs<P>,
    ) -> Result<()>;
    fn public_input_plonk_circuit<P: PastaConfig>(
        &mut self,
        wire_plonk_circuit: &WirePlonkCircuit<P>,
        plonk_circuit: &PlonkCircuit<P>,
    ) -> Result<()>;
}
impl CallPlonk for Call {
    fn witness_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: &WirePlonkProof<P>,
        proof: &PlonkProof<P>,
    ) -> Result<()> {
        let WirePlonkProof {
            vs,
            Cs,
            pis,
            acc_next,
        } = wire_proof;

        let WirePlonkProofEvals {
            ws,
            rs,
            qs,
            ts,
            ids,
            sigmas,
            z,
            z_omega,
            w_omegas,
        } = vs;
        self.witness(*z, proof.vs.z)?;
        self.witness(*z_omega, proof.vs.z_omega)?;
        for (wire_w, w) in ws.iter().zip(proof.vs.ws) {
            self.witness(*wire_w, w)?;
        }
        for (wire_r, r) in rs.iter().zip(proof.vs.rs) {
            self.witness(*wire_r, r)?;
        }
        for (wire_q, q) in qs.iter().zip(proof.vs.qs) {
            self.witness(*wire_q, q)?;
        }
        for (wire_t, t) in ts.iter().zip(proof.vs.ts) {
            self.witness(*wire_t, t)?;
        }
        for (wire_id, id) in ids.iter().zip(proof.vs.ids) {
            self.witness(*wire_id, id)?;
        }
        for (wire_sigma, sigma) in sigmas.iter().zip(proof.vs.sigmas) {
            self.witness(*wire_sigma, sigma)?;
        }
        for (wire_w_omega, w_omega) in w_omegas.iter().zip(proof.vs.w_omegas) {
            self.witness(*wire_w_omega, w_omega)?;
        }

        let WirePlonkProofCommitments { ws, ts, z } = Cs;
        self.witness_affine(*z, proof.Cs.z.into_affine())?;
        for (wire_w, w) in ws.iter().zip(proof.Cs.ws) {
            self.witness_affine(*wire_w, w.into_affine())?;
        }
        for (wire_t, t) in ts.iter().zip(proof.Cs.ts) {
            self.witness_affine(*wire_t, t.into_affine())?;
        }

        let WirePlonkProofEvalProofs { r, r_omega } = pis;
        self.witness_eval_proof(&r, &proof.pis.r)?;
        self.witness_eval_proof(&r_omega, &proof.pis.r_omega)?;

        self.witness_accumulator(&acc_next, &proof.acc_next)?;

        Ok(())
    }
    fn public_input_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: WirePlonkProof<P>,
        proof: PlonkProof<P>,
    ) -> Result<()> {
        let WirePlonkProof {
            vs,
            Cs,
            pis,
            acc_next,
        } = wire_proof;

        let WirePlonkProofEvals {
            ws,
            rs,
            qs,
            ts,
            ids,
            sigmas,
            z,
            z_omega,
            w_omegas,
        } = vs;
        self.public_input(z, proof.vs.z)?;
        self.public_input(z_omega, proof.vs.z_omega)?;
        for (wire_w, w) in ws.iter().zip(proof.vs.ws) {
            self.public_input(*wire_w, w)?;
        }
        for (wire_r, r) in rs.iter().zip(proof.vs.rs) {
            self.public_input(*wire_r, r)?;
        }
        for (wire_q, q) in qs.iter().zip(proof.vs.qs) {
            self.public_input(*wire_q, q)?;
        }
        for (wire_t, t) in ts.iter().zip(proof.vs.ts) {
            self.public_input(*wire_t, t)?;
        }
        for (wire_id, id) in ids.iter().zip(proof.vs.ids) {
            self.public_input(*wire_id, id)?;
        }
        for (wire_sigma, sigma) in sigmas.iter().zip(proof.vs.sigmas) {
            self.public_input(*wire_sigma, sigma)?;
        }
        for (wire_w_omega, w_omega) in w_omegas.iter().zip(proof.vs.w_omegas) {
            self.public_input(*wire_w_omega, w_omega)?;
        }

        let WirePlonkProofCommitments { ws, ts, z } = Cs;
        self.public_input_affine(z, proof.Cs.z.into_affine())?;
        for (wire_w, w) in ws.iter().zip(proof.Cs.ws) {
            self.public_input_affine(*wire_w, w.into_affine())?;
        }
        for (wire_t, t) in ts.iter().zip(proof.Cs.ts) {
            self.public_input_affine(*wire_t, t.into_affine())?;
        }

        let WirePlonkProofEvalProofs { r, r_omega } = pis;
        self.public_input_eval_proof(&r, &proof.pis.r)?;
        self.public_input_eval_proof(&r_omega, &proof.pis.r_omega)?;

        self.public_input_accumulator(&acc_next, &proof.acc_next)?;

        Ok(())
    }
    fn witness_plonk_public_input<P: PastaConfig>(
        &mut self,
        wire_plonk_public_inputs: &WirePlonkPublicInputs<P>,
        plonk_public_inputs: &PlonkPublicInputs<P>,
    ) -> Result<()> {
        assert!(
            plonk_public_inputs.public_inputs.len() <= wire_plonk_public_inputs.public_inputs.len()
        );
        for i in 0..plonk_public_inputs.public_inputs.len() {
            if i < plonk_public_inputs.public_inputs.len() {
                self.witness(
                    wire_plonk_public_inputs.public_inputs[i],
                    plonk_public_inputs.public_inputs[i],
                )?
            } else {
                self.witness(
                    wire_plonk_public_inputs.public_inputs[i],
                    Scalar::<P>::zero(),
                )?
            }
        }
        self.witness_accumulator(
            &wire_plonk_public_inputs.acc_prev,
            &plonk_public_inputs.acc_prev,
        )?;

        Ok(())
    }
    fn public_input_plonk_public_input<P: PastaConfig>(
        &mut self,
        wire_plonk_public_inputs: &WirePlonkPublicInputs<P>,
        plonk_public_inputs: &PlonkPublicInputs<P>,
    ) -> Result<()> {
        assert!(
            plonk_public_inputs.public_inputs.len() <= wire_plonk_public_inputs.public_inputs.len()
        );
        for i in 0..plonk_public_inputs.public_inputs.len() {
            if i < plonk_public_inputs.public_inputs.len() {
                self.public_input(
                    wire_plonk_public_inputs.public_inputs[i],
                    plonk_public_inputs.public_inputs[i],
                )?
            } else {
                self.public_input(
                    wire_plonk_public_inputs.public_inputs[i],
                    Scalar::<P>::zero(),
                )?
            }
        }
        self.public_input_accumulator(
            &wire_plonk_public_inputs.acc_prev,
            &plonk_public_inputs.acc_prev,
        )?;

        Ok(())
    }
    fn public_input_plonk_circuit<P: PastaConfig>(
        &mut self,
        wire_plonk_circuit: &WirePlonkCircuit<P>,
        plonk_circuit: &PlonkCircuit<P>,
    ) -> Result<()> {
        assert!(wire_plonk_circuit.rows == plonk_circuit.rows);
        assert!(wire_plonk_circuit.public_input_count == plonk_circuit.public_input_count);
        let PlonkCircuit {
            rows: _,
            public_input_count: _,
            omega: _,
            Cs,
        } = plonk_circuit;
        let PlonkCircuitCommitments {
            qs,
            rs,
            ids,
            sigmas,
        } = Cs;

        for i in 0..qs.len() {
            self.public_input_affine(wire_plonk_circuit.Cs.qs[i], qs[i].into_affine())?
        }
        for i in 0..rs.len() {
            self.public_input_affine(wire_plonk_circuit.Cs.rs[i], rs[i].into_affine())?
        }
        for i in 0..ids.len() {
            self.public_input_affine(wire_plonk_circuit.Cs.ids[i], ids[i].into_affine())?;
            self.public_input_affine(wire_plonk_circuit.Cs.sigmas[i], sigmas[i].into_affine())?
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct WirePlonkCircuitCommitments<P: PastaConfig> {
    pub qs: [WireAffine<P>; Q_POLYS],
    pub rs: [WireAffine<P>; R_POLYS],
    pub ids: [WireAffine<P>; S_POLYS],
    pub sigmas: [WireAffine<P>; S_POLYS],
}

#[derive(Clone, Copy)]
pub struct WirePlonkCircuit<P: PastaConfig> {
    pub n: WireScalar<P>,
    pub rows: usize,
    pub mds: [[WireScalar<P>; 3]; 3],
    pub public_input_count: usize,
    pub omega: WireScalar<P>,
    pub Cs: WirePlonkCircuitCommitments<P>,
}
impl<P: PastaConfig> WirePlonkCircuit<P> {
    pub fn zero(rows: usize, public_input_count: usize) -> Self {
        Self {
            Cs: WirePlonkCircuitCommitments {
                qs: array::from_fn(|_| WireAffine::generator()),
                rs: array::from_fn(|_| WireAffine::generator()),
                ids: array::from_fn(|_| WireAffine::generator()),
                sigmas: array::from_fn(|_| WireAffine::generator()),
            },
            n: WireScalar::zero(),
            rows,
            public_input_count,
            mds: P::SCALAR_POSEIDON_MDS.map(|x| x.map(|_| WireScalar::zero())),
            omega: WireScalar::zero(),
        }
    }
    pub fn constant(plonk_circuit: &PlonkCircuit<P>) -> Self {
        Self {
            Cs: WirePlonkCircuitCommitments {
                qs: array::from_fn(|i| WireAffine::constant(plonk_circuit.Cs.qs[i].into_affine())),
                rs: array::from_fn(|i| WireAffine::constant(plonk_circuit.Cs.rs[i].into_affine())),
                ids: array::from_fn(|i| {
                    WireAffine::constant(plonk_circuit.Cs.ids[i].into_affine())
                }),
                sigmas: array::from_fn(|i| {
                    WireAffine::constant(plonk_circuit.Cs.sigmas[i].into_affine())
                }),
            },
            n: WireScalar::constant(Scalar::<P>::from(plonk_circuit.rows as u64)),
            rows: plonk_circuit.rows,
            public_input_count: plonk_circuit.public_input_count,
            mds: P::SCALAR_POSEIDON_MDS.map(|x| x.map(|y| WireScalar::constant(y))),
            omega: WireScalar::constant(plonk_circuit.omega),
        }
    }
    pub fn public_input(rows: usize, public_input_count: usize) -> Self {
        Self {
            Cs: WirePlonkCircuitCommitments {
                qs: array::from_fn(|i| WireAffine::public_input()),
                rs: array::from_fn(|i| WireAffine::public_input()),
                ids: array::from_fn(|i| WireAffine::public_input()),
                sigmas: array::from_fn(|i| WireAffine::public_input()),
            },
            n: WireScalar::constant(Scalar::<P>::from(rows as u64)),
            rows,
            public_input_count,
            mds: P::SCALAR_POSEIDON_MDS.map(|x| x.map(|y| WireScalar::constant(y))),
            omega: WireScalar::constant(Domain::<P>::new(rows).unwrap().element(1)),
        }
    }
}

#[derive(Clone)]
pub struct WirePlonkPublicInputs<P: PastaConfig> {
    pub public_inputs: Vec<WireScalar<P>>,
    pub acc_prev: WireAccumulator<P>,
}

impl<P: PastaConfig> WirePlonkPublicInputs<P> {
    pub fn witness(rows: usize, public_input_count: usize) -> Self {
        Self {
            public_inputs: (0..public_input_count)
                .map(|_| WireScalar::witness())
                .collect(),
            acc_prev: WireAccumulator::witness(rows),
        }
    }
    pub fn public_input(rows: usize, public_input_count: usize) -> Self {
        Self {
            public_inputs: (0..public_input_count)
                .map(|_| WireScalar::public_input())
                .collect(),
            acc_prev: WireAccumulator::public_input(rows),
        }
    }
}

#[derive(Clone)]
pub struct WirePlonkProofEvalProofs<P: PastaConfig> {
    pub r: WireEvalProof<P>,
    pub r_omega: WireEvalProof<P>,
}

#[derive(Clone)]
pub struct WirePlonkProofEvals<P: PastaConfig> {
    pub ws: [WireScalar<P>; W_POLYS],
    pub rs: [WireScalar<P>; R_POLYS],
    pub qs: [WireScalar<P>; Q_POLYS],
    pub ts: [WireScalar<P>; T_POLYS],
    pub ids: [WireScalar<P>; S_POLYS],
    pub sigmas: [WireScalar<P>; S_POLYS],
    pub z: WireScalar<P>,
    pub z_omega: WireScalar<P>,
    pub w_omegas: [WireScalar<P>; 3],
}

#[derive(Clone)]
pub struct WirePlonkProofCommitments<P: PastaConfig> {
    pub ws: [WireAffine<P>; W_POLYS],
    pub ts: [WireAffine<P>; T_POLYS],
    pub z: WireAffine<P>,
}

#[derive(Clone)]
pub struct WirePlonkProof<P: PastaConfig> {
    pub vs: WirePlonkProofEvals<P>,
    pub Cs: WirePlonkProofCommitments<P>,
    pub pis: WirePlonkProofEvalProofs<P>,
    pub acc_next: WireAccumulator<P>,
}
impl<P: PastaConfig> WirePlonkProof<P> {
    pub fn witness(n: usize) -> Self {
        WirePlonkProof {
            vs: WirePlonkProofEvals {
                ws: array::from_fn(|_| WireScalar::witness()),
                rs: array::from_fn(|_| WireScalar::witness()),
                qs: array::from_fn(|_| WireScalar::witness()),
                ts: array::from_fn(|_| WireScalar::witness()),
                ids: array::from_fn(|_| WireScalar::witness()),
                sigmas: array::from_fn(|_| WireScalar::witness()),
                z: WireScalar::witness(),
                z_omega: WireScalar::witness(),
                w_omegas: array::from_fn(|_| WireScalar::witness()),
            },
            Cs: WirePlonkProofCommitments {
                ws: array::from_fn(|_| WireAffine::witness()),
                ts: array::from_fn(|_| WireAffine::witness()),
                z: WireAffine::witness(),
            },
            pis: WirePlonkProofEvalProofs {
                r: WireEvalProof::witness(n),
                r_omega: WireEvalProof::witness(n),
            },
            acc_next: WireAccumulator::witness(n),
        }
    }

    pub fn public_input(n: usize) -> Self {
        WirePlonkProof {
            vs: WirePlonkProofEvals {
                ws: array::from_fn(|_| WireScalar::public_input()),
                rs: array::from_fn(|_| WireScalar::public_input()),
                qs: array::from_fn(|_| WireScalar::public_input()),
                ts: array::from_fn(|_| WireScalar::public_input()),
                ids: array::from_fn(|_| WireScalar::public_input()),
                sigmas: array::from_fn(|_| WireScalar::public_input()),
                z: WireScalar::public_input(),
                z_omega: WireScalar::public_input(),
                w_omegas: array::from_fn(|_| WireScalar::public_input()),
            },
            Cs: WirePlonkProofCommitments {
                ws: array::from_fn(|_| WireAffine::public_input()),
                ts: array::from_fn(|_| WireAffine::public_input()),
                z: WireAffine::public_input(),
            },
            pis: WirePlonkProofEvalProofs {
                r: WireEvalProof::public_input(n),
                r_omega: WireEvalProof::public_input(n),
            },
            acc_next: WireAccumulator::public_input(n),
        }
    }

    pub fn verify_succinct(
        &self,
        circuit: WirePlonkCircuit<P>,
        public_inputs: WirePlonkPublicInputs<P>,
    ) -> WireBool<P> {
        let pi = self;
        let n = circuit.n;
        let one = WireScalar::<P>::one();
        let mut transcript = OuterSponge::new(Protocols::PLONK);

        assert!(
            public_inputs.public_inputs.len() <= circuit.public_input_count,
            "PI.len() = {:?} > circuit.PI.len() = {:?}",
            public_inputs.public_inputs.len(),
            circuit.public_input_count
        );

        // -------------------- Round 1 --------------------

        transcript.absorb_g(&pi.Cs.ws);

        // -------------------- Round 2 --------------------

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
        let MDS = circuit.mds;
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

        let f = f_gc + alpha * f_cc1 + (alpha * alpha) * f_cc2;
        let t = t_reconstruct_generic(pi.vs.ts, xi_n);

        let f_eq_t_zh = f.equals(t * z_H);

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

        let instance_1 = WireInstance::new(C_r, xi, v_r, pi.pis.r.clone());
        let instance_2 = WireInstance::new(C_r_omega, xi_omega, v_r_omega, pi.pis.r_omega.clone());

        let acc_prev = public_inputs.acc_prev.clone();

        let pp = WirePublicParams::new(circuit.rows);
        let acc_next = pi.acc_next.clone();
        let qs = vec![acc_prev.instance, instance_1, instance_2];
        let acc_ok = acc_next.verify(pp, qs);

        f_eq_t_zh & acc_ok
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        Fp, Fq, PallasConfig, VestaConfig,
        ark_ff::UniformRand,
        ark_std::{One, test_rng},
    };
    use halo_schnorr::generate_keypair;

    use crate::{
        frontend::{
            Call, Frontend,
            plonk::{CallPlonk, WirePlonkCircuit, WirePlonkProof, WirePlonkPublicInputs},
            primitives::{WireAffine, WireScalar},
            signature::WireSchnorrSignature,
        },
        plonk::PlonkProof,
    };

    #[test]
    fn plonk_verify_signature() -> Result<()> {
        let rng = &mut test_rng();

        let (sk_v, pk_v) = generate_keypair();
        let mut message_v = Vec::new();
        for _ in 0..5 {
            message_v.push(Fq::rand(rng))
        }

        // Generate and verify valid signature
        let signature_v = sk_v.sign(&message_v);
        assert!(pk_v.verify(&message_v, signature_v.clone()));

        let pk = WireAffine::constant(pk_v.0);
        let message: Vec<WireScalar<VestaConfig>> =
            message_v.iter().map(|m| WireScalar::constant(*m)).collect();
        let r = WireAffine::<PallasConfig>::witness();
        let s = WireScalar::<PallasConfig>::witness();

        let signature = WireSchnorrSignature::new(r, s);
        signature.verify(pk, &message);

        let mut call = Call::new();

        call.witness_affine(r, signature_v.r)?;
        call.witness(s, signature_v.s)?;

        let (fp_trace, fq_trace) = call.trace()?;
        let n_fp = fp_trace.rows;
        let n_fq = fq_trace.rows;

        let (fp_circuit, fp_x, fp_w) = fp_trace.consume();
        let pi_fp = PlonkProof::naive_prover(rng, fp_circuit, &fp_x, fp_w);
        let (fq_circuit, fq_x, fq_w) = fq_trace.consume();
        let pi_fq = PlonkProof::naive_prover(rng, fq_circuit, &fq_x, fq_w);

        pi_fp.clone().verify_succinct(fp_circuit, &fp_x)?;
        pi_fq.clone().verify_succinct(fq_circuit, &fq_x)?;

        Frontend::reset();

        let fp_wire_circuit = WirePlonkCircuit::constant(&fp_circuit);
        let fp_wire_x = WirePlonkPublicInputs::witness(fp_circuit.rows, fp_x.public_inputs.len());
        let fp_wire_pi = WirePlonkProof::<PallasConfig>::witness(n_fp);
        fp_wire_pi
            .verify_succinct(fp_wire_circuit, fp_wire_x.clone())
            .output();

        let fq_wire_circuit = WirePlonkCircuit::constant(&fq_circuit);
        let fq_wire_x = WirePlonkPublicInputs::witness(fq_circuit.rows, fq_x.public_inputs.len());
        let fq_wire_pi = WirePlonkProof::<VestaConfig>::witness(n_fq);
        fq_wire_pi
            .verify_succinct(fq_wire_circuit, fq_wire_x.clone())
            .output();

        let mut call = Call::new();

        call.witness_plonk_proof(&fp_wire_pi, &pi_fp)?;
        call.witness_plonk_public_input(&fp_wire_x, &fp_x)?;

        call.witness_plonk_proof(&fq_wire_pi, &pi_fq)?;
        call.witness_plonk_public_input(&fq_wire_x, &fq_x)?;

        let (fp_trace, fq_trace) = call.trace()?;

        assert_eq!(fp_trace.outputs[0], Fp::one());
        assert_eq!(fq_trace.outputs[0], Fq::one());

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }
}
