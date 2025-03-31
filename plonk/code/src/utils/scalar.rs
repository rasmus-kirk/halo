use ark_poly::Evaluations;
use halo_accumulation::group::PallasScalar;

#[cfg(test)]
use ark_ff::{BigInteger, PrimeField};

use crate::Coset;

use super::misc::batch_op;

type Scalar = PallasScalar;
type Evals = Evaluations<Scalar>;

#[cfg(test)]
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
pub fn batch_compute_evals(h: &Coset, ys: Vec<Vec<Scalar>>) -> Vec<Evals> {
    batch_op(ys, |evals| {
        Evaluations::from_vec_and_domain(evals, h.domain)
    })
}
