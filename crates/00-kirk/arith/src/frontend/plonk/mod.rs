use std::array;

use anyhow::Result;
use halo_group::{PastaConfig, Scalar, ark_ec::CurveGroup, ark_std::Zero};
use halo_poseidon::Protocols;

use crate::{
    circuit::PlonkPublicInputs,
    frontend::{
        Call,
        pcdl::{CallInstance, WireEvalProof, WireInstance, WirePublicParams},
        poseidon::outer_sponge::OuterSponge,
        primitives::{WireAffine, WireScalar},
    },
    plonk::{
        PlonkProof, affine_add_constraints_generic, affine_mul_constraints_generic,
        geometric_generic, poseidon_constraints_generic, pow_n, public_input_eval_generic,
        t_reconstruct_generic,
    },
    utils::{Q_POLYS, R_POLYS, S_POLYS, T_POLYS, W_POLYS},
};

const MAX_PUBLIC_INPUTS: usize = 512;

pub trait CallPlonk {
    fn witness_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: WirePlonkProof<P>,
        proof: PlonkProof<P>,
    ) -> Result<()>;
    fn witness_plonk_public_input<P: PastaConfig>(
        &mut self,
        wire_public_input: &[Scalar<P>],
        plonk_public_input: &WirePlonkPublicInputs<P>,
    ) -> Result<()>;
}
impl CallPlonk for Call {
    fn witness_plonk_proof<P: PastaConfig>(
        &mut self,
        wire_proof: WirePlonkProof<P>,
        proof: PlonkProof<P>,
    ) -> Result<()> {
        self.witness(wire_proof.vs.z, proof.vs.z)?;
        self.witness(wire_proof.vs.z_omega, proof.vs.z_omega)?;
        for (wire_w, w) in wire_proof.vs.ws.iter().zip(proof.vs.ws) {
            self.witness(*wire_w, w)?;
        }
        for (wire_r, r) in wire_proof.vs.rs.iter().zip(proof.vs.rs) {
            self.witness(*wire_r, r)?;
        }
        for (wire_q, q) in wire_proof.vs.qs.iter().zip(proof.vs.qs) {
            self.witness(*wire_q, q)?;
        }
        for (wire_t, t) in wire_proof.vs.ts.iter().zip(proof.vs.ts) {
            self.witness(*wire_t, t)?;
        }
        for (wire_id, id) in wire_proof.vs.ids.iter().zip(proof.vs.ids) {
            self.witness(*wire_id, id)?;
        }
        for (wire_sigma, sigma) in wire_proof.vs.sigmas.iter().zip(proof.vs.sigmas) {
            self.witness(*wire_sigma, sigma)?;
        }
        for (wire_w_omega, w_omega) in wire_proof.vs.w_omegas.iter().zip(proof.vs.w_omegas) {
            self.witness(*wire_w_omega, w_omega)?;
        }

        self.witness_affine(wire_proof.Cs.z, proof.Cs.z.into_affine())?;
        self.witness_affine(wire_proof.Cs.r, proof.Cs.r.into_affine())?;
        for (wire_w, w) in wire_proof.Cs.ws.iter().zip(proof.Cs.ws) {
            self.witness_affine(*wire_w, w.into_affine())?;
        }
        for (wire_t, t) in wire_proof.Cs.ts.iter().zip(proof.Cs.ts) {
            self.witness_affine(*wire_t, t.into_affine())?;
        }

        self.witness_eval_proof(&wire_proof.pis.r, &proof.pis.r)?;
        self.witness_eval_proof(&wire_proof.pis.r_omega, &proof.pis.r_omega)?;

        Ok(())
    }
    fn witness_plonk_public_input<P: PastaConfig>(
        &mut self,
        public_inputs: &[Scalar<P>],
        wire_plonk_public_inputs: &WirePlonkPublicInputs<P>,
    ) -> Result<()> {
        assert!(public_inputs.len() < MAX_PUBLIC_INPUTS);
        for i in 0..MAX_PUBLIC_INPUTS {
            if i < public_inputs.len() {
                self.public_input(wire_plonk_public_inputs.public_inputs[i], public_inputs[i])?
            } else {
                self.public_input(
                    wire_plonk_public_inputs.public_inputs[i],
                    Scalar::<P>::zero(),
                )?
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct WirePlonkPublicInputCommitments<P: PastaConfig> {
    pub qs: [WireAffine<P>; Q_POLYS],
    pub rs: [WireAffine<P>; R_POLYS],
    pub ids: [WireAffine<P>; S_POLYS],
    pub sigmas: [WireAffine<P>; S_POLYS],
}

#[derive(Clone)]
pub struct WirePlonkPublicInputs<P: PastaConfig> {
    pub mds: [[WireScalar<P>; 3]; 3],
    pub rows: usize,
    pub n: WireScalar<P>,
    pub omega: WireScalar<P>,
    pub public_inputs: [WireScalar<P>; MAX_PUBLIC_INPUTS],
    pub Cs: WirePlonkPublicInputCommitments<P>,
}
impl<P: PastaConfig> WirePlonkPublicInputs<P> {
    pub fn witness(plonk_public_inputs: &PlonkPublicInputs<P>) -> Self {
        let rows = plonk_public_inputs.rows;
        let Cs = WirePlonkPublicInputCommitments {
            qs: array::from_fn(|i| {
                WireAffine::constant(plonk_public_inputs.Cs.qs[i].into_affine())
            }),
            rs: array::from_fn(|i| {
                WireAffine::constant(plonk_public_inputs.Cs.rs[i].into_affine())
            }),
            ids: array::from_fn(|i| {
                WireAffine::constant(plonk_public_inputs.Cs.ids[i].into_affine())
            }),
            sigmas: array::from_fn(|i| {
                WireAffine::constant(plonk_public_inputs.Cs.sigmas[i].into_affine())
            }),
        };
        Self {
            n: WireScalar::constant(Scalar::<P>::from(rows as u64)),
            rows,
            mds: P::SCALAR_POSEIDON_MDS.map(|x| x.map(|y| WireScalar::constant(y))),
            omega: WireScalar::constant(plonk_public_inputs.omega),
            public_inputs: array::from_fn(|_| WireScalar::public_input()),
            Cs,
        }
    }
}

#[derive(Clone)]
pub struct WirePlonkProofEvals<P: PastaConfig> {
    ws: [WireScalar<P>; W_POLYS],
    rs: [WireScalar<P>; R_POLYS],
    qs: [WireScalar<P>; Q_POLYS],
    ts: [WireScalar<P>; T_POLYS],
    ids: [WireScalar<P>; S_POLYS],
    sigmas: [WireScalar<P>; S_POLYS],
    z: WireScalar<P>,
    z_omega: WireScalar<P>,
    w_omegas: [WireScalar<P>; 3],
}

#[derive(Clone)]
pub struct WirePlonkProofCommitments<P: PastaConfig> {
    ws: [WireAffine<P>; W_POLYS],
    ts: [WireAffine<P>; T_POLYS],
    z: WireAffine<P>,
    r: WireAffine<P>,
}

#[derive(Clone)]
pub struct WirePlonkProofEvalProofs<P: PastaConfig> {
    r: WireEvalProof<P>,
    r_omega: WireEvalProof<P>,
}

#[derive(Clone)]
pub struct WirePlonkProof<P: PastaConfig> {
    vs: WirePlonkProofEvals<P>,
    Cs: WirePlonkProofCommitments<P>,
    pis: WirePlonkProofEvalProofs<P>,
}
impl<P: PastaConfig> WirePlonkProof<P> {
    pub fn witness(n: usize) -> Self {
        let ws: [WireScalar<P>; W_POLYS] = array::from_fn(|_| WireScalar::witness());
        let rs: [WireScalar<P>; R_POLYS] = array::from_fn(|_| WireScalar::witness());
        let qs: [WireScalar<P>; Q_POLYS] = array::from_fn(|_| WireScalar::witness());
        let ts: [WireScalar<P>; T_POLYS] = array::from_fn(|_| WireScalar::witness());
        let ids: [WireScalar<P>; S_POLYS] = array::from_fn(|_| WireScalar::witness());
        let sigmas: [WireScalar<P>; S_POLYS] = array::from_fn(|_| WireScalar::witness());
        let z: WireScalar<P> = WireScalar::witness();
        let z_omega: WireScalar<P> = WireScalar::witness();
        let w_omegas: [WireScalar<P>; 3] = array::from_fn(|_| WireScalar::witness());

        let vs = WirePlonkProofEvals {
            ws,
            rs,
            qs,
            ts,
            ids,
            sigmas,
            z,
            z_omega,
            w_omegas,
        };

        let ws: [WireAffine<P>; W_POLYS] = array::from_fn(|_| WireAffine::witness());
        let ts: [WireAffine<P>; T_POLYS] = array::from_fn(|_| WireAffine::witness());
        let z = WireAffine::witness();
        let r = WireAffine::witness();

        let Cs = WirePlonkProofCommitments { ws, ts, z, r };

        let r = WireEvalProof::witness(n);
        let r_omega = WireEvalProof::witness(n);
        let pis = WirePlonkProofEvalProofs { r, r_omega };

        WirePlonkProof { vs, Cs, pis }
    }

    pub fn verify(self, public_inputs: WirePlonkPublicInputs<P>) {
        let pi = self;
        let d = public_inputs.rows - 1;
        let n = public_inputs.n;
        let one = WireScalar::one();
        let mut transcript = OuterSponge::new(Protocols::PLONK);

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
        let xi_n = pow_n(xi, public_inputs.rows);
        let xi_omega = xi * public_inputs.omega;
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
        let poseidon_terms =
            poseidon_constraints_generic(public_inputs.mds, &pi.vs.rs, &pi.vs.ws, &pi.vs.w_omegas);
        let affine_add_terms = affine_add_constraints_generic(pi.vs.ws.clone());
        let affine_mul_terms =
            affine_mul_constraints_generic(pi.vs.ws, pi.vs.w_omegas, pi.vs.rs[0]);

        let f_gc = pi.vs.ws[0] * pi.vs.qs[0]
            + pi.vs.ws[1] * pi.vs.qs[1]
            + pi.vs.ws[2] * pi.vs.qs[2]
            + pi.vs.ws[0] * pi.vs.ws[1] * pi.vs.qs[3]
            + pi.vs.qs[4]
            + pi.vs.qs[5] * poseidon_terms
            + pi.vs.qs[6] * affine_add_terms
            + pi.vs.qs[7] * affine_mul_terms
            + public_input_eval_generic(
                &public_inputs.public_inputs,
                public_inputs.n,
                public_inputs.omega,
                xi,
                xi_n,
            );

        let omega = public_inputs.omega;
        let l1 = (omega * (xi_n - one)) / (n * (xi - omega));
        let z_H = xi_n - one;
        let f_cc1 = l1 * (pi.vs.z - one);
        let f_cc2 = pi.vs.z * f_prime - pi.vs.z_omega * g_prime;

        let f = f_gc + alpha * f_cc1 + (alpha * alpha) * f_cc2;
        let t = t_reconstruct_generic(pi.vs.ts, xi_n);

        f.assert_eq(t * z_H);

        let mut vec = Vec::new();
        vec.extend_from_slice(&pi.vs.qs);
        vec.extend_from_slice(&pi.vs.ws);
        vec.extend_from_slice(&pi.vs.ts);
        vec.push(pi.vs.z);
        let v_r = geometric_generic(zeta, vec);
        let pp = WirePublicParams::new(d + 1);
        WireInstance::new(pi.Cs.r, xi, v_r, pi.pis.r).succinct_check(pp);
        WireInstance::new(pi.Cs.z, xi_omega, pi.vs.z_omega, pi.pis.r_omega).succinct_check(pp);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{Fq, PallasConfig, VestaConfig, ark_ff::UniformRand, ark_std::test_rng};
    use halo_schnorr::generate_keypair;

    use crate::{
        frontend::{
            Call, Frontend,
            plonk::{CallPlonk, WirePlonkProof, WirePlonkPublicInputs},
            primitives::{WireAffine, WireScalar},
            signature::SchnorrSignature,
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

        let signature = SchnorrSignature::new(r, s);
        signature.verify(pk, &message);

        let mut call = Call::new();

        call.witness_affine(r, signature_v.r)?;
        call.witness(s, signature_v.s)?;

        let (fp_trace, fq_trace) = call.trace(None)?;
        let n_fp = fp_trace.rows;
        let n_fq = fq_trace.rows;

        let (plonk_public_input_fp, plonk_witness) = fp_trace.consume();
        let pi_fp = PlonkProof::naive_prover(rng, plonk_witness);
        pi_fp.clone().verify(plonk_public_input_fp.clone())?;
        let (plonk_public_input_fq, plonk_witness) = fq_trace.consume();
        let pi_fq = PlonkProof::naive_prover(rng, plonk_witness);
        pi_fq.clone().verify(plonk_public_input_fq.clone())?;

        Frontend::reset();

        let fp_public_inputs = WirePlonkPublicInputs::witness(&plonk_public_input_fp);
        let fp_wire_pi = WirePlonkProof::<PallasConfig>::witness(n_fp);
        fp_wire_pi.clone().verify(fp_public_inputs.clone());

        let fq_public_inputs = WirePlonkPublicInputs::witness(&plonk_public_input_fq);
        let fq_wire_pi = WirePlonkProof::<VestaConfig>::witness(n_fq);
        fq_wire_pi.clone().verify(fq_public_inputs.clone());

        let mut call = Call::new();

        call.witness_plonk_proof(fp_wire_pi, pi_fp)?;
        call.witness_plonk_public_input(&plonk_public_input_fp.public_inputs, &fp_public_inputs)?;

        call.witness_plonk_proof(fq_wire_pi, pi_fq)?;
        call.witness_plonk_public_input(&plonk_public_input_fq.public_inputs, &fq_public_inputs)?;

        let (fp_trace, fq_trace) = call.trace(None)?;

        let (plonk_public_input_fp, plonk_witness) = fp_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input_fp)?;
        let (plonk_public_input_fq, plonk_witness) = fq_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input_fq)?;

        Ok(())
    }
}
