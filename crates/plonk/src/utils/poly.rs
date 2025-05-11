use super::{misc::batch_op, Evals, Scalar};
use crate::Coset;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};

use educe::Educe;
use std::ops::Index;

/// ∀X ∈ [n]: f(ωᶦ) = v
pub fn deg0<P: SWCurveConfig>(v: Scalar<P>) -> DensePolynomial<Scalar<P>> {
    Poly::<P>::new_v(&v).p
}

#[derive(Educe)]
#[educe(Default, Debug, Clone, PartialEq, Eq)]
pub struct Poly<P: SWCurveConfig> {
    pub p: DensePolynomial<Scalar<P>>,
    pub e: Evals<P>,
}

impl<P: SWCurveConfig> Poly<P> {
    pub fn new(p: DensePolynomial<Scalar<P>>) -> Self {
        Self {
            p,
            e: Default::default(),
        }
    }

    pub fn new_e(p: DensePolynomial<Scalar<P>>, e: Evals<P>) -> Self {
        Self { p, e }
    }

    /// f(X) = vXⁿ
    pub fn new_vxn(v: &Scalar<P>, n: u64) -> Self {
        let mut coeffs = vec![Scalar::<P>::ZERO; n as usize];
        coeffs.push(*v);
        Self::new(DenseUVPolynomial::from_coefficients_vec(coeffs))
    }

    /// f(X) = v
    pub fn new_v(v: &Scalar<P>) -> Self {
        Self::new_vxn(v, 0)
    }

    /// f(X) = Xⁿ
    pub fn new_xn(n: u64) -> Self {
        Self::new_vxn(&Scalar::<P>::ONE, n)
    }

    /// f(X) = X
    pub fn new_x() -> Self {
        Self::new_xn(1)
    }

    /// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
    pub fn new_li(h: &Coset<P>, i: u64) -> Self {
        let wi = h.w(i);
        let numerator = (Self::new_xn(h.n()).p + Self::new_v(&Scalar::<P>::ONE).p) * wi;
        let denominator = (Self::new_x().p - Self::new_v(&wi).p) * Scalar::<P>::from(h.n());
        Self::new(numerator / denominator)
    }

    /// f(X) = p₀(X) + Xp₁(X) + X²p₂(X) + ...
    pub fn split(self, n: u64) -> Vec<Self> {
        self.p
            .coeffs
            .chunks(n as usize)
            .map(DensePolynomial::from_coefficients_slice)
            .map(Self::new)
            .collect()
    }

    /// f(X)
    pub fn evaluate(&self, x: &Scalar<P>) -> Scalar<P> {
        self.p.evaluate(x)
    }
}

pub fn batch_p<'a, P: SWCurveConfig, I>(ps: I) -> Vec<&'a DensePolynomial<Scalar<P>>>
where
    I: IntoIterator<Item = &'a Poly<P>>,
{
    batch_op(ps, |f| &f.p)
}

pub fn batch_evaluate<'a, P: SWCurveConfig, I>(ps: I, x: Scalar<P>) -> Vec<Scalar<P>>
where
    I: IntoIterator<Item = &'a Poly<P>>,
{
    batch_op(ps, |f| f.evaluate(&x))
}

impl<P: SWCurveConfig> Index<usize> for Poly<P> {
    type Output = Scalar<P>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scheme::Slots, utils::misc::EnumIter};

    use ark_pallas::PallasConfig;
    use halo_accumulation::group::PallasScalar;

    #[test]
    fn lagrange() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::<PallasConfig>::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        for i in h.iter() {
            let l = Poly::new_li(&h, i);
            for j in h.iter() {
                if i == j {
                    assert_eq!(l.evaluate(&h.w(j)), PallasScalar::ONE);
                } else {
                    assert_eq!(l.evaluate(&h.w(j)), PallasScalar::ZERO);
                }
            }
        }
    }
}
