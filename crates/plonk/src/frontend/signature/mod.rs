use anyhow::Result;
use halo_group::PastaConfig;
use halo_poseidon::Protocols;
use halo_schnorr::SchnorrSignature;

use crate::frontend::{
    Call,
    poseidon::outer_sponge::OuterSponge,
    primitives::{WireAffine, WireBool, WireScalar},
};

pub type WirePublicKey<P> = WireAffine<P>;

pub trait CallSignature {
    fn witness_signature<P: PastaConfig>(
        &mut self,
        wire_proof: WireSchnorrSignature<P>,
        proof: SchnorrSignature<P>,
    ) -> Result<()>;
    fn public_input_signature<P: PastaConfig>(
        &mut self,
        wire_proof: WireSchnorrSignature<P>,
        proof: SchnorrSignature<P>,
    ) -> Result<()>;
}
impl CallSignature for Call {
    fn witness_signature<P: PastaConfig>(
        &mut self,
        wire_signature: WireSchnorrSignature<P>,
        signature: SchnorrSignature<P>,
    ) -> Result<()> {
        let WireSchnorrSignature { r, s } = wire_signature;
        self.witness_affine(r, signature.r)?;
        self.witness(s, signature.s)
    }
    fn public_input_signature<P: PastaConfig>(
        &mut self,
        wire_signature: WireSchnorrSignature<P>,
        signature: SchnorrSignature<P>,
    ) -> Result<()> {
        let WireSchnorrSignature { r, s } = wire_signature;
        self.public_input_affine(r, signature.r)?;
        self.public_input(s, signature.s)
    }
}

// Schnorr signature struct: (R, s)
#[derive(Clone, Copy)]
pub struct WireSchnorrSignature<P: PastaConfig> {
    pub r: WireAffine<P>, // Commitment point R = k * G
    pub s: WireScalar<P>, // s = k + e * x
}
impl<P: PastaConfig> WireSchnorrSignature<P> {
    pub fn witness() -> Self {
        Self {
            r: WireAffine::witness(),
            s: WireScalar::witness(),
        }
    }
    pub fn public_input() -> Self {
        Self {
            r: WireAffine::public_input(),
            s: WireScalar::public_input(),
        }
    }
    pub fn new(r: WireAffine<P>, s: WireScalar<P>) -> Self {
        Self { r, s }
    }
}

impl<P: PastaConfig> WireSchnorrSignature<P> {
    fn hash_message(
        public_key: WireAffine<P>,
        r: WireAffine<P>,
        message: &[WireScalar<P::OtherCurve>],
    ) -> WireScalar<P> {
        let mut sponge = OuterSponge::new(Protocols::SIGNATURE);

        // Hash P || R || m
        sponge.absorb_g(&[public_key, r]);
        sponge.absorb_fq(message);
        sponge.challenge()
    }

    pub fn verify(
        &self,
        pk: WireAffine<P>,
        message: &[WireScalar<P::OtherCurve>],
    ) -> WireBool<P::OtherCurve> {
        // e = H(P || R || m)
        let e = Self::hash_message(pk, self.r, message);

        // s * G =? R + e * P
        let lhs = WireAffine::generator() * self.s;
        let rhs = self.r + pk * e;
        lhs.equals(rhs)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        Fq, PallasConfig, VestaConfig,
        ark_ff::UniformRand,
        ark_std::{rand::Rng, test_rng},
    };
    use halo_schnorr::generate_keypair;

    use crate::{
        frontend::{
            Call,
            primitives::{WireAffine, WireScalar},
            signature::WireSchnorrSignature,
        },
        plonk::PlonkProof,
    };

    #[test]
    fn test_signature() -> Result<()> {
        let rng = &mut test_rng();

        let (sk_v, pk_v) = generate_keypair();
        let mut message_v = Vec::new();
        let range = rng.gen_range(3..15);
        for _ in 0..range {
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

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        Ok(())
    }
}
