use ark_poly::Evaluations;
use halo_accumulation::group::PallasScalar;

use ark_ff::{BigInteger, PrimeField};

use crate::curve::Coset;

type Scalar = PallasScalar;

/// Compute the bitwise XOR of two scalars
pub fn bitxor(lhs: Scalar, rhs: Scalar) -> Scalar {
    let xs = lhs.into_bigint().to_bits_be();
    let ys = rhs.into_bigint().to_bits_be();
    let mut zs = vec![false; xs.len().max(ys.len())];
    for (i, z) in zs.iter_mut().enumerate() {
        *z = xs.get(i).unwrap_or(&false) ^ ys.get(i).unwrap_or(&false);
    }
    Scalar::from_bigint(BigInteger::from_bits_be(&zs)).unwrap()
}

/// Compute the Evaluation struct for a Vec of Vec of Scalars
pub fn batch_compute_evals(h: &Coset, ys: Vec<Vec<Scalar>>) -> Vec<Evaluations<Scalar>> {
    ys.into_iter()
        .map(|evals| Evaluations::from_vec_and_domain(evals, h.domain))
        .collect()
}
