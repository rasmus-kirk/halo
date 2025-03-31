use std::ops::{Add, AddAssign, Mul};

use ark_ff::AdditiveGroup;
use ark_poly::Evaluations;
use halo_accumulation::group::PallasScalar;

use ark_ff::Field;
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

/// Y = p₀ + a₁p₁ + a₂p₂ + ...
pub fn linear_comb<I, T: Mul<Scalar, Output = T> + AdditiveGroup + AddAssign>(
    a: &Scalar,
    ps: I,
) -> T
where
    I: IntoIterator<Item = T>,
{
    ps.into_iter()
        .enumerate()
        .fold(T::ZERO, |acc, (i, p_i)| acc + p_i * a.pow([i as u64]))
}

// /// Y = p₀ + a₁p₁ + a₂p₂ + ...
// pub fn linear_comb_right<I, T: AdditiveGroup + AddAssign>(a: &Scalar, ps: I) -> T
// where
//     I: IntoIterator<Item = T>,
//     Scalar: Mul<T, Output = T>,
// {
//     ps.into_iter()
//         .enumerate()
//         .fold(T::ZERO, |acc, (i, p_i)| acc + a.pow([i as u64]) * p_i)
// }

/// f = a + ζb + ζ²c + ζ³j
/// TODO move to scheme?
pub fn plookup_compress(zeta: &Scalar, a: &Scalar, b: &Scalar, c: &Scalar, j: &Scalar) -> Scalar {
    linear_comb(zeta, [*a, *b, *c, *j])
}

/// Y = L₁(X) = (Xⁿ - 1) / (n (X - 1))
pub fn lagrange_basis1(h: &Coset, x: &Scalar) -> Scalar {
    let n = h.n();
    let w = h.w(1);
    w * (x.pow([n]) - Scalar::ONE) / (Scalar::from(n) * (*x - w))
}

/// Y = Zₕ(X) = Xⁿ - 1
pub fn zh_ev(h: &Coset, x: &Scalar) -> Scalar {
    x.pow([h.n()]) - Scalar::ONE
}

/// Y = x₀y₀ + x₁y₁ + x₂y₂ + ...
pub fn hadamard<T: Mul<T, Output = T> + Add<T, Output = T> + Copy>(xs: &[T], ys: &[T]) -> T {
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| *x * *y)
        .reduce(|acc, x| acc + x)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scheme::Slots, utils::poly::lagrange_basis};

    use ark_poly::Polynomial;
    use rand::Rng;

    #[test]
    fn l1_ev() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let l1 = lagrange_basis(&h, 1);
        for _ in 0..100 {
            let x: Scalar = rng.gen();
            assert_eq!(lagrange_basis1(&h, &x), l1.evaluate(&x));
        }
    }
}
