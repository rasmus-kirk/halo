use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{
    curve::WireAffine, field::WireScalar, poseidon::outer_sponge::OuterSponge,
    primitives::bool::WireBool,
};

// Schnorr signature struct: (R, s)
#[derive(Clone)]
pub struct SchnorrSignature<P: PastaConfig> {
    r: WireAffine<P>, // Commitment point R = k * G
    s: WireScalar<P>, // s = k + e * x
}
impl<P: PastaConfig> SchnorrSignature<P> {
    pub fn new(r: WireAffine<P>, s: WireScalar<P>) -> Self {
        Self { r, s }
    }
}

impl<P: PastaConfig> SchnorrSignature<P> {
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
        ark_ff::{Field, UniformRand},
        ark_std::{rand::Rng, test_rng},
    };
    use halo_schnorr::generate_keypair;

    use crate::{
        frontend::{Call, curve::WireAffine, field::WireScalar, signature::SchnorrSignature},
        plonk::PlonkProof,
    };

    #[test]
    fn signature() -> Result<()> {
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

        let signature = SchnorrSignature::new(r, s);
        signature.verify(pk, &message).output();

        let mut call = Call::<PallasConfig>::new();

        call.witness_affine(r, signature_v.r)?;
        call.witness(s, signature_v.s)?;

        let (fp_trace, fq_trace) = call.trace()?;

        assert_eq!(fq_trace.outputs[0], Fq::ONE);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}
