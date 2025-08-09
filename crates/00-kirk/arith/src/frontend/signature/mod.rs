use halo_group::PastaConfig;
use halo_poseidon::{Protocols, SPONGE_RATE, STATE_SIZE};

use crate::frontend::{
    FRONTEND, curve::WireAffine, field::WireScalar, poseidon::outer_sponge::OuterSponge,
};

// Schnorr signature struct: (R, s)
#[derive(Clone)]
pub struct SchnorrSignature<P: PastaConfig> {
    r: WireAffine<P>, // Commitment point R = k * G
    s: WireScalar<P>, // s = k + e * x
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

    pub fn verify(&self, pk: WireAffine<P>, message: &[WireScalar<P::OtherCurve>]) {
        // e = H(P || R || m)
        let e = Self::hash_message(pk, self.r, message);

        // s * G =? R + e * P
        (WireAffine::generator() * self.s).assert_eq(self.r + pk * e);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn absorb_squeeze() -> Result<()> {
        Ok(())
    }
}
