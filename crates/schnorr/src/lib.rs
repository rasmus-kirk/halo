use ark_pallas::{Projective, Fr};
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use ark_std::UniformRand;
use rand::thread_rng;
use ark_ec::PrimeGroup;

use sha2::{Digest, Sha256};

// Define types for convenience
type Scalar = Fr; // Scalar field of Pallas curve
type Point = Projective; // Curve point in projective coordinates

// Schnorr signature struct: (R, s)
#[derive(Clone, Debug)]
pub struct SchnorrSignature {
    r: Point, // Commitment point R = k * G
    s: Scalar, // s = k + e * x
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SecretKey(Scalar);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PublicKey(Point);

// Hash function for Fiat-Shamir transform: H(P || R || m)
fn hash_message(public_key: &Point, r: &Point, message: &[u8]) -> Scalar {
    let mut hasher = Sha256::new();
    
    // Convert points to affine and get x-coordinates for hashing
    let public_key_affine = public_key.into_affine();
    let r_affine = r.into_affine();
    
    // Serialize x-coordinates as 32-byte arrays
    let mut bytes = vec![];
    public_key_affine.x.serialize_compressed(&mut bytes).unwrap();
    r_affine.x.serialize_compressed(&mut bytes).unwrap();
    
    // Hash P || R || m
    hasher.update(&bytes);
    hasher.update(message);
    
    // Convert hash to scalar (mod q)
    let hash_bytes = hasher.finalize();
    Scalar::from_be_bytes_mod_order(&hash_bytes)
}

// Generate key pair: (private_key, public_key)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let mut rng = thread_rng();
    let secret_key = SecretKey(Scalar::rand(&mut rng));
    let public_key = PublicKey(Point::generator() * secret_key.0);
    (secret_key, public_key)
}

impl SecretKey {
    // Sign a message with the private key
    pub fn sign(&self, message: &[u8]) -> SchnorrSignature {
        let sk = self.0;
        let mut rng = thread_rng();

        let k = Scalar::rand(&mut rng);

        // R = k * G
        let r = Point::generator() * k;

        // e = H(P || R || m)
        let pk = Point::generator() * sk;
        let e = hash_message(&pk, &r, message);

        // s = k + e * x
        let s = k + e * sk;

        SchnorrSignature { r, s }
    }
}

impl PublicKey {
    // Verify a signature
    pub fn verify(&self, message: &[u8], signature: &SchnorrSignature) -> bool {
        let pk = self.0;

        // e = H(P || R || m)
        let e = hash_message(&pk, &signature.r, message);

        // s * G =? R + e * P
        Point::generator() * signature.s == signature.r + pk * e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_pallas::Fr;
    use ark_ec::CurveGroup;
    use ark_std::UniformRand;
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
        assert!(pk.0.into_affine().is_on_curve());
        
        // Check that public key = private_key * G
        let expected_pk = Point::generator() * sk.0;
        assert_eq!(pk.0, expected_pk);
    }

    #[test]
    fn test_signature_verification() {
        let (sk, pk) = generate_keypair();
        let message = b"Test message for Schnorr signature";
        
        // Generate and verify valid signature
        let signature = sk.sign(message);
        assert!(pk.verify(message, &signature));
    }

    #[test]
    fn test_wrong_message_fails() {
        let (sk, pk) = generate_keypair();
        let message = b"Correct message";
        let wrong_message = b"Wrong message";
        
        let signature = sk.sign(message);
        assert!(!pk.verify(wrong_message, &signature));
    }

    #[test]
    fn test_invalid_signature_fails() {
        let (sk, pk) = generate_keypair();
        let message = b"Test message";
        
        let mut signature = sk.sign(message);
        
        // Modify signature to make it invalid
        let mut rng = thread_rng();
        signature.s = Fr::rand(&mut rng);
        
        assert!(!pk.verify(message, &signature));
    }

    #[test]
    fn test_different_keypair_fails() {
        let (sk, _) = generate_keypair();
        let (_, other_pk) = generate_keypair();
        let message = b"Test message";
        
        let signature = sk.sign(message);
        assert!(!other_pk.verify(message, &signature));
    }

    #[test]
    fn test_multiple_messages() {
        let (sk, pk) = generate_keypair();
        
        // Test with 5 random messages
        for _ in 0..5 {
            let message = random_message();
            let signature = sk.sign(&message);
            assert!(pk.verify(&message, &signature));
        }
    }

    #[test]
    fn test_empty_message() {
        let (sk, pk) = generate_keypair();
        let empty_message = b"";
        
        let signature = sk.sign(empty_message);
        assert!(pk.verify(empty_message, &signature));
    }
}
