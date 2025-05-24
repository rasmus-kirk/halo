use std::marker::PhantomData;

use ark_ec::{short_weierstrass::{Affine, SWCurveConfig}, AffineRepr, CurveConfig, CurveGroup};
use ark_ff::{BigInteger, Field, PrimeField, Zero};
use ark_pallas::{Fr, Projective};
use ark_std::{rand::Rng, UniformRand};

use halo_group::wrappers::PastaConfig;

// ECDSA key pair
#[derive(Clone)]
pub struct KeyPair<P: PastaConfig> {
    pub private_key: P::ScalarField,
    pub public_key: Affine<P>,
}

// ECDSA signature
#[derive(Clone, PartialEq)]
pub struct Signature<P: PastaConfig> {
    pub r: P::ScalarField,
    pub s: P::ScalarField,
}

pub struct ECDSA<C: SWCurveConfig>(PhantomData<C>);

impl<P: PastaConfig> ECDSA<P> {
    pub fn key_gen<R: Rng>(rng: &mut R) -> KeyPair<P>{
        let private_key = P::ScalarField::rand(rng);
        let public_key = (Affine::generator() * private_key).into_affine();
        KeyPair {
            private_key,
            public_key,
        }
    }

    // Sign a message hash (assumed to be 32 bytes, e.g., SHA-256 output)
    pub fn sign<R: Rng>(rng: &mut R, private_key: &P::ScalarField, message_hash: &[u8]) -> Option<Signature<P>> {
        // Convert message hash to a field element
        let z = P::ScalarField::from_le_bytes_mod_order(message_hash);

        // Generate random nonce k
        let k = P::ScalarField::rand(rng);
        if k.is_zero() {
            return None; // Prevent zero nonce
        }

        let x = ark_pallas::Fq::zero();
        x.into_bigint();


        // Compute k * G
        let r_point = (Affine::<P>::generator() * k).into_affine();
        let r = P::scalar_from_bigint(P::basefield_into_bigint(r_point.x));
        if r.is_zero() {
            return None; // Prevent zero r
        }

        // Compute s = k^(-1) * (z + r * d) mod q
        let k_inv = k.inverse()?;
        let rd = r * private_key;
        let z_plus_rd = z + rd;
        let s = k_inv * z_plus_rd;
        if s.is_zero() {
            return None; // Prevent zero s
        }

        Some(Signature { r, s })
    }

    // Verify a signature
    pub fn verify(public_key: &Affine<P>, message_hash: &[u8], signature: &Signature<P>) -> bool {
        let r = signature.r;
        let s = signature.s;

        // Check if r and s are non-zero
        if r.is_zero() || s.is_zero() {
            return false;
        }

        // Convert message hash to field element
        let z = P::ScalarField::from_le_bytes_mod_order(message_hash);

        // Compute s^(-1)
        let s_inv = match s.inverse() {
            Some(inv) => inv,
            None => return false,
        };

        // Compute u1 = z * s^(-1) and u2 = r * s^(-1)
        let u1 = z * s_inv;
        let u2 = r * s_inv;

        // Compute u1 * G + u2 * Q
        let point = (Affine::generator() * u1 + *public_key * u2).into_affine();

        // Check if x-coordinate of point equals r
        Fr::from(point.x) == r
    }
}

// Example usage
fn main() {
    let mut rng = ark_std::rand::thread_rng();

    // Generate key pair
    let key_pair = KeyPair::generate(&mut rng);
    println!("Public key: {:?}", key_pair.public_key);

    // Example message
    let message = b"Hello, ECDSA over Pallas!";
    let message_hash = Sha256::digest(message);

    // Sign the message
    let signature = ECDSA::sign(&mut rng, &key_pair.private_key, &message_hash).expect("Failed to sign");
    println!("Signature: r = {:?}, s = {:?}", signature.r, signature.s);

    // Verify the signature
    let is_valid = ECDSA::verify(&key_pair.public_key, &message_hash, &signature);
    println!("Signature valid: {}", is_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa_sign_verify() {
        let mut rng = ark_std::rand::thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Test message";
        let message_hash = Sha256::digest(message);

        let signature = ECDSA::sign(&mut rng, &key_pair.private_key, &message_hash).expect("Failed to sign");
        assert!(ECDSA::verify(&key_pair.public_key, &message_hash, &signature));

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let wrong_hash = Sha256::digest(wrong_message);
        assert!(!ECDSA::verify(&key_pair.public_key, &wrong_hash, &signature));
    }
}
