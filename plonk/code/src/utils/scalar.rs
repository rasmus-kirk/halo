use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
#[cfg(test)]
use ark_ff::{BigInteger, PrimeField};

use super::Scalar;

#[cfg(test)]
/// Compute the bitwise XOR of two scalars
pub fn bitxor<P: SWCurveConfig>(lhs: Scalar<P>, rhs: Scalar<P>) -> Scalar<P> {
    let xs = lhs.into_bigint().to_bits_be();
    let ys = rhs.into_bigint().to_bits_be();
    let mut zs = vec![false; xs.len().max(ys.len())];
    for (i, z) in zs.iter_mut().enumerate() {
        *z = xs.get(i).unwrap_or(&false) ^ ys.get(i).unwrap_or(&false);
    }
    Scalar::<P>::from_bigint(BigInteger::from_bits_be(&zs)).unwrap()
}

// /// Compute the Evaluation struct for a Vec of Vec of Scalars
// pub fn batch_compute_evals(h: &Coset, ys: Vec<Vec<Scalar>>) -> Vec<Evals> {
//     batch_op(ys, |evals| {
//         Evals::<P>::from_vec_and_domain(evals, h.domain)
//     })
// }

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

/// Y = L₁(X) = (Xⁿ - 1) / (n (X - 1))
pub fn lagrange_basis1<P: SWCurveConfig>(n: u64, w: Scalar<P>, x: Scalar<P>) -> Scalar<P> {
    w * (x.pow([n]) - Scalar::<P>::ONE) / (Scalar::<P>::from(n) * (x - w))
}

/// Y = Zₕ(X) = Xⁿ - 1
pub fn zh_ev<P: SWCurveConfig>(n: u64, x: Scalar<P>) -> Scalar<P> {
    x.pow([n]) - Scalar::<P>::ONE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        scheme::Slots,
        utils::{misc::EnumIter, poly::lagrange_basis},
        Coset,
    };

    use ark_pallas::PallasConfig;
    use ark_poly::Polynomial;
    use halo_accumulation::group::PallasScalar;
    use rand::Rng;

    #[test]
    fn l1_ev() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::<PallasConfig>::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let l1 = lagrange_basis(&h, 1);
        for _ in 0..100 {
            let x: PallasScalar = rng.gen();
            assert_eq!(
                lagrange_basis1::<PallasConfig>(h.n(), h.w(1), x),
                l1.evaluate(&x)
            );
        }
    }
}
