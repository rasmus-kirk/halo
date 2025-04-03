use crate::Coset;

use halo_accumulation::pcdl;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Evaluations, Polynomial};

use super::{misc::batch_op, Evals, Point, Poly, Scalar};

pub fn batch_interpolate<P: SWCurveConfig>(
    es: Vec<Evaluations<Scalar<P>>>,
) -> Vec<DensePolynomial<Scalar<P>>> {
    batch_op(es, |e| e.interpolate())
}

/// f(X) = v
pub fn deg0<P: SWCurveConfig>(v: Scalar<P>) -> DensePolynomial<Scalar<P>> {
    DensePolynomial::from_coefficients_slice(&[v])
}

/// f(X) = vXⁿ
pub fn vxn<P: SWCurveConfig>(v: &Scalar<P>, n: u64) -> DensePolynomial<Scalar<P>> {
    let mut coeffs = vec![Scalar::<P>::ZERO; n as usize];
    coeffs.push(*v);
    DensePolynomial::from_coefficients_slice(&coeffs)
}

/// f(X) = Xⁿ
pub fn xn<P: SWCurveConfig>(n: u64) -> DensePolynomial<Scalar<P>> {
    vxn::<P>(&Scalar::<P>::ONE, n)
}

/// f(X) = X
pub fn x<P: SWCurveConfig>() -> DensePolynomial<Scalar<P>> {
    vxn::<P>(&Scalar::<P>::ONE, 1)
}

// /// ∀X ∈ H₀: g(X) = f(aX)
// pub fn coset_scale(h: &Coset, f: &Poly, a: Scalar) -> Poly {
//     // Step 1: Get the coset domain scaled by `a`
//     let coset_domain = h
//         .coset_domain
//         .get_coset(h.coset_domain.coset_offset() * a)
//         .unwrap();

//     // Step 2: Perform FFT on `f` over the coset domain {a * ωᶦ}
//     let mut evals_new = coset_domain.fft(&f.coeffs);
//     let evals_new_last = evals_new.pop().unwrap();
//     evals_new.insert(0, evals_new_last);

//     // Step 3: Perform inverse FFT to interpolate the new polynomial g(X)
//     Evaluations::from_vec_and_domain(evals_new, h.domain).interpolate()
// }

// /// ∀X ∈ H₀: g(X) = f(ωX)
// pub fn coset_scale_omega(h: &Coset, f: &Poly) -> Poly {
//     coset_scale(h, f, h.w(1))
// }

/// ∀X ∈ H₀: g(X) = f(ωX)
pub fn shift_wrap_eval<P: SWCurveConfig>(h: &Coset<P>, evals: Evals<P>) -> Evals<P> {
    let mut evals_new = evals.evals;
    let evals_new_first = evals_new.remove(0);
    evals_new.push(evals_new_first);
    Evaluations::from_vec_and_domain(evals_new, h.domain)
}

/// f(X) = p₀(X) + Xⁿp₁(X) + X²ⁿp₂(X) + ...
pub fn split<P: SWCurveConfig>(
    n: u64,
    f: &DensePolynomial<Scalar<P>>,
) -> Vec<DensePolynomial<Scalar<P>>> {
    f.coeffs
        .chunks(n as usize)
        .map(DensePolynomial::from_coefficients_slice)
        .collect()
}

/// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
pub fn lagrange_basis<P: SWCurveConfig>(h: &Coset<P>, i: u64) -> Poly<P> {
    let wi = h.w(i);
    let numerator = (xn::<P>(h.n()) + deg0::<P>(Scalar::<P>::ONE)) * wi;
    let denominator = (x::<P>() - deg0::<P>(wi)) * Scalar::<P>::from(h.n());
    numerator / denominator
}

// /// Zₕ(X) = Xⁿ - 1
// /// such that ∀X ∈ H₀: Zₕ(X) = 0
// pub fn zh_poly(h: &Coset) -> Poly {
//     xn_poly(h.n()) - deg0(&Scalar::ONE)
// }

pub fn batch_evaluate<'a, P: SWCurveConfig, I>(ps: I, x: Scalar<P>) -> Vec<Scalar<P>>
where
    I: IntoIterator<Item = &'a DensePolynomial<Scalar<P>>>,
{
    batch_op(ps, |f| f.evaluate(&x))
}

pub fn batch_commit<'a, P: SWCurveConfig, I>(
    ps: I,
    d: usize,
    w: Option<&Scalar<P>>,
) -> Vec<Point<P>>
where
    I: IntoIterator<Item = &'a Scalar<P>>,
{
    batch_op(ps, |f| pcdl::commit(f, d, w))
}

#[cfg(test)]
mod tests {
    use ark_pallas::PallasConfig;
    use halo_accumulation::group::PallasScalar;

    use crate::{scheme::Slots, utils::misc::EnumIter};

    use super::*;

    // #[test]
    // fn zh() {
    //     let rng = &mut rand::thread_rng();
    //     let h_opt = Coset::new(rng, 5, Slots::COUNT);
    //     assert!(h_opt.is_some());
    //     let h = h_opt.unwrap();
    //     let zh = zh_poly(&h);
    //     for i in h.iter() {
    //         assert_eq!(zh.evaluate(&h.w(i)), Scalar::ZERO);
    //     }
    // }

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
