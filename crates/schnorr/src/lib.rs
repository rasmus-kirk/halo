use halo_group::ark_std::rand::thread_rng;
use halo_group::Affine;
use halo_group::{
    ark_ec::{short_weierstrass::Projective, AffineRepr, CurveGroup},
    ark_std::UniformRand,
    PastaConfig, Scalar,
};
use halo_poseidon::{Protocols, Sponge};

// Schnorr signature struct: (R, s)
#[derive(Clone, Copy, Debug)]
pub struct SchnorrSignature<P: PastaConfig> {
    pub r: Affine<P>, // Commitment point R = k * G
    pub s: Scalar<P>, // s = k + e * x
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SecretKey<P: PastaConfig>(Scalar<P>);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PublicKey<P: PastaConfig>(pub Affine<P>);

// Hash function for Fiat-Shamir transform: H(P || R || m)
pub fn hash_message<P: PastaConfig>(
    public_key: Affine<P>,
    r: Affine<P>,
    message: &[P::BaseField],
) -> Scalar<P> {
    let mut sponge = Sponge::new(Protocols::SIGNATURE);

    // Hash P || R || m
    sponge.absorb_g_affine(&[public_key, r]);
    sponge.absorb_fq(message);
    sponge.challenge()
}

// Generate key pair: (private_key, public_key)
pub fn generate_keypair<P: PastaConfig>() -> (SecretKey<P>, PublicKey<P>) {
    let mut rng = thread_rng();
    let secret_key = SecretKey(Scalar::<P>::rand(&mut rng));
    let public_key = PublicKey(Projective::<P>::into_affine(
        Affine::<P>::generator() * secret_key.0,
    ));
    (secret_key, public_key)
}

impl<P: PastaConfig> SecretKey<P> {
    // Sign a message with the private key
    pub fn sign(&self, message: &[P::BaseField]) -> SchnorrSignature<P> {
        let sk = self.0;
        let mut rng = thread_rng();

        let k = Scalar::<P>::rand(&mut rng);

        // R = k * G
        let r = (Affine::<P>::generator() * k).into_affine();

        // e = H(P || R || m)
        let pk = (Affine::<P>::generator() * sk).into_affine();
        let e = hash_message::<P>(pk, r, message);

        // s = k + e * x
        let s = k + e * sk;

        SchnorrSignature { r, s }
    }
}

impl<P: PastaConfig> PublicKey<P> {
    // Verify a signature
    pub fn verify(&self, message: &[P::BaseField], signature: SchnorrSignature<P>) -> bool {
        let pk = self.0;

        // e = H(P || R || m)
        let e = hash_message(pk, signature.r, message);

        // s * G =? R + e * P
        Affine::generator() * signature.s == signature.r + pk * e
    }
}

#[cfg(test)]
mod tests {
    use halo_group::{
        ark_std::{rand::Rng, test_rng},
        Affine, Fp, Fq, PallasConfig,
    };

    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (sk, pk) = generate_keypair::<PallasConfig>();

        // Check that private key is non-zero
        assert_ne!(sk.0, Fp::from(0));

        // Check that public key is on curve
        assert!(pk.0.is_on_curve());

        // Check that public key = private_key * G
        let expected_pk = Affine::generator() * sk.0;
        assert_eq!(pk.0, expected_pk);
    }

    #[test]
    fn test_signature_verification() {
        let rng = &mut test_rng();

        let (sk, pk) = generate_keypair::<PallasConfig>();
        let mut message = Vec::new();
        let range = rng.gen_range(3..15);
        for _ in 0..range {
            message.push(Fq::rand(rng))
        }

        // Generate and verify valid signature
        let signature = sk.sign(&message);
        assert!(pk.verify(&message, signature));
    }

    #[test]
    fn test_wrong_message_fails() {
        let rng = &mut test_rng();

        let (sk, pk) = generate_keypair::<PallasConfig>();
        let message = [Fq::rand(rng)];
        let wrong_message = [Fq::rand(rng)];

        let signature = sk.sign(&message);
        assert!(!pk.verify(&wrong_message, signature));
    }

    #[test]
    fn test_invalid_signature_fails() {
        let rng = &mut test_rng();

        let (sk, pk) = generate_keypair::<PallasConfig>();
        let message = [Fq::rand(rng)];

        let mut signature = sk.sign(&message);

        // Modify signature to make it invalid
        let mut rng = thread_rng();
        signature.s = Fp::rand(&mut rng);

        assert!(!pk.verify(&message, signature));
    }

    #[test]
    fn test_different_keypair_fails() {
        let rng = &mut test_rng();

        let (sk, _) = generate_keypair::<PallasConfig>();
        let (_, other_pk) = generate_keypair::<PallasConfig>();
        let message = [Fq::rand(rng)];

        let signature = sk.sign(&message);
        assert!(!other_pk.verify(&message, signature));
    }

    #[test]
    fn test_empty_message() {
        let (sk, pk) = generate_keypair::<PallasConfig>();

        let signature = sk.sign(&[]);
        assert!(pk.verify(&[], signature));
    }
}
