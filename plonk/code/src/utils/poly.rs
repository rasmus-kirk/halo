use crate::Coset;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use ark_poly::{DenseUVPolynomial, Polynomial};

use super::{misc::batch_op, Evals, Poly, Scalar};

pub fn batch_interpolate<P: SWCurveConfig>(es: Vec<Evals<P>>) -> Vec<Poly<P>> {
    batch_op(es, |e| e.interpolate())
}

/// f(X) = v
pub fn deg0<P: SWCurveConfig>(v: Scalar<P>) -> Poly<P> {
    Poly::<P>::from_coefficients_slice(&[v])
}

/// f(X) = vXⁿ
pub fn vxn<P: SWCurveConfig>(v: &Scalar<P>, n: u64) -> Poly<P> {
    let mut coeffs = vec![Scalar::<P>::ZERO; n as usize];
    coeffs.push(*v);
    Poly::<P>::from_coefficients_slice(&coeffs)
}

/// f(X) = Xⁿ
pub fn xn<P: SWCurveConfig>(n: u64) -> Poly<P> {
    vxn::<P>(&Scalar::<P>::ONE, n)
}

/// f(X) = X
pub fn x<P: SWCurveConfig>() -> Poly<P> {
    vxn::<P>(&Scalar::<P>::ONE, 1)
}

/// ∀X ∈ H₀: g(X) = f(ωX)
pub fn shift_wrap_eval<P: SWCurveConfig>(h: &Coset<P>, evals: Evals<P>) -> Evals<P> {
    let mut evals_new = evals.evals;
    let evals_new_first = evals_new.remove(0);
    evals_new.push(evals_new_first);
    Evals::<P>::from_vec_and_domain(evals_new, h.domain)
}

/// f(X) = p₀(X) + Xⁿp₁(X) + X²ⁿp₂(X) + ...
pub fn split<P: SWCurveConfig>(n: u64, f: &Poly<P>) -> Vec<Poly<P>> {
    f.coeffs
        .chunks(n as usize)
        .map(Poly::<P>::from_coefficients_slice)
        .collect()
}

/// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
pub fn lagrange_basis<P: SWCurveConfig>(h: &Coset<P>, i: u64) -> Poly<P> {
    let wi = h.w(i);
    let numerator = (xn::<P>(h.n()) + deg0::<P>(Scalar::<P>::ONE)) * wi;
    let denominator = (x::<P>() - deg0::<P>(wi)) * Scalar::<P>::from(h.n());
    numerator / denominator
}

pub fn batch_evaluate<'a, P: SWCurveConfig, I>(ps: I, x: Scalar<P>) -> Vec<Scalar<P>>
where
    I: IntoIterator<Item = &'a Poly<P>>,
{
    batch_op(ps, |f| f.evaluate(&x))
}

#[cfg(test)]
mod tests {
    use ark_pallas::PallasConfig;
    use halo_accumulation::group::PallasScalar;

    use crate::{scheme::Slots, utils::misc::EnumIter};

    use super::*;

    #[test]
    fn lagrange() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::<PallasConfig>::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        for i in h.iter() {
            let l = lagrange_basis(&h, i);
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
