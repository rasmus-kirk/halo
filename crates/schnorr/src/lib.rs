use ark_ec::AffineRepr;
use ark_ec::CurveGroup;
use ark_ec::PrimeGroup;
use ark_ff::PrimeField;
use ark_pallas::{Fr, Projective};
use ark_serialize::CanonicalSerialize;
use ark_std::UniformRand;
use halo_group::Affine;
use halo_group::Fp;
use halo_group::Fq;
use halo_group::PallasConfig;
use halo_poseidon::Protocols;
use halo_poseidon::Sponge;
use rand::thread_rng;

// Schnorr signature struct: (R, s)
#[derive(Clone, Debug)]
pub struct SchnorrSignature {
    r: Affine<PallasConfig>, // Commitment point R = k * G
    s: Fp,                   // s = k + e * x
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SecretKey(Fp);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PublicKey(Affine<PallasConfig>);

// Hash function for Fiat-Shamir transform: H(P || R || m)
fn hash_message(public_key: Affine<PallasConfig>, r: Affine<PallasConfig>, message: &[Fq]) -> Fp {
    let mut sponge = Sponge::new(Protocols::SIGNATURE);

    // Hash P || R || m
    sponge.absorb_g_affine(&[public_key, r]);
    sponge.absorb_fq(message);
    sponge.challenge()
}

// Generate key pair: (private_key, public_key)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let mut rng = thread_rng();
    let secret_key = SecretKey(Fp::rand(&mut rng));
    let public_key = PublicKey((Affine::generator() * secret_key.0).into_affine());
    (secret_key, public_key)
}

impl SecretKey {
    // Sign a message with the private key
    pub fn sign(&self, message: &[Fq]) -> SchnorrSignature {
        let sk = self.0;
        let mut rng = thread_rng();

        let k = Fp::rand(&mut rng);

        // R = k * G
        let r = (Affine::generator() * k).into_affine();

        // e = H(P || R || m)
        let pk = (Affine::generator() * sk).into_affine();
        let e = hash_message(pk, r, message);

        // s = k + e * x
        let s = k + e * sk;

        SchnorrSignature { r, s }
    }
}

impl PublicKey {
    // Verify a signature
    pub fn verify(&self, message: &[Fq], signature: SchnorrSignature) -> bool {
        let pk = self.0;

        // e = H(P || R || m)
        let e = hash_message(pk, signature.r, message);

        // s * G =? R + e * P
        Affine::generator() * signature.s == signature.r + pk * e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;
    use ark_pallas::Fr;
    use ark_std::{test_rng, UniformRand};
    use hex::FromHex;
    use rand::{thread_rng, Rng, RngCore};

    // Helper function to generate a random message
    fn random_message() -> Vec<u8> {
        let mut rng = thread_rng();
        let len = rng.gen_range(10..100);
        let mut msg = vec![0u8; len];
        rng.fill_bytes(&mut msg);
        msg
    }

    #[test]
    fn test_keypair_generation() {
        let (sk, pk) = generate_keypair();

        // Check that private key is non-zero
        assert_ne!(sk.0, Fr::from(0));

        // Check that public key is on curve
        assert!(pk.0.is_on_curve());

        // Check that public key = private_key * G
        let expected_pk = Affine::generator() * sk.0;
        assert_eq!(pk.0, expected_pk);
    }

    #[test]
    fn test_signature_verification() {
        let rng = &mut test_rng();

        let (sk, pk) = generate_keypair();
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

        let (sk, pk) = generate_keypair();
        let message = [Fq::rand(rng)];
        let wrong_message = [Fq::rand(rng)];

        let signature = sk.sign(&message);
        assert!(!pk.verify(&wrong_message, signature));
    }

    #[test]
    fn test_invalid_signature_fails() {
        let rng = &mut test_rng();

        let (sk, pk) = generate_keypair();
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

        let (sk, _) = generate_keypair();
        let (_, other_pk) = generate_keypair();
        let message = [Fq::rand(rng)];

        let signature = sk.sign(&message);
        assert!(!other_pk.verify(&message, signature));
    }

    #[test]
    fn test_empty_message() {
        let (sk, pk) = generate_keypair();

        let signature = sk.sign(&[]);
        assert!(pk.verify(&[], signature));
    }
}
