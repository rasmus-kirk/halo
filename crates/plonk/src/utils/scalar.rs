use super::Scalar;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;

/// Y = L₁(X) = (Xⁿ - 1) / (n (X - 1))
pub fn lagrange_basis1<P: SWCurveConfig>(n: u64, w: Scalar<P>, x: Scalar<P>) -> Scalar<P> {
    w * (x.pow([n]) - Scalar::<P>::ONE) / (Scalar::<P>::from(n) * (x - w))
}

/// Y = Zₕ(X) = Xⁿ - 1
pub fn zh_ev<P: SWCurveConfig>(n: u64, x: Scalar<P>) -> Scalar<P> {
    x.pow([n]) - Scalar::<P>::ONE
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{
        scheme::Slots,
        utils::{misc::EnumIter, Poly},
        Coset,
    };

    use ark_ff::{BigInteger, PrimeField};
    use ark_pallas::PallasConfig;

    use rand::Rng;

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

    #[test]
    fn l1_ev() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::<PallasConfig>::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let l1 = Poly::new_li(&h, 1);
        for _ in 0..100 {
            let x = rng.gen();
            assert_eq!(
                lagrange_basis1::<PallasConfig>(h.n(), h.w(1), x),
                l1.evaluate(&x)
            );
        }
    }
}
