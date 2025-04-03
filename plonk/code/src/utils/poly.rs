use crate::Coset;

use halo_accumulation::pcdl;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field, Fp, FpConfig};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Evaluations, Polynomial};

use super::{misc::batch_op, Evals, Point, Poly};

pub fn batch_interpolate<const N: usize, C: FpConfig<N>>(
    es: Vec<Evaluations<Fp<C, N>>>,
) -> Vec<DensePolynomial<Fp<C, N>>> {
    batch_op(es, |e| e.interpolate())
}

/// f(X) = v
pub fn deg0<const N: usize, C: FpConfig<N>>(v: Fp<C, N>) -> DensePolynomial<Fp<C, N>> {
    DensePolynomial::from_coefficients_slice(&[v])
}

/// f(X) = vXⁿ
pub fn vxn<const N: usize, C: FpConfig<N>>(v: &Fp<C, N>, n: u64) -> DensePolynomial<Fp<C, N>> {
    let mut coeffs = vec![Fp::ZERO; n as usize];
    coeffs.push(*v);
    DensePolynomial::from_coefficients_slice(&coeffs)
}

/// f(X) = Xⁿ
pub fn xn<const N: usize, C: FpConfig<N>>(n: u64) -> DensePolynomial<Fp<C, N>> {
    vxn(&Fp::ONE, n)
}

/// f(X) = X
pub fn x<const N: usize, C: FpConfig<N>>() -> DensePolynomial<Fp<C, N>> {
    vxn(&Fp::ONE, 1)
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

// TODO coset needs to generalize
/// ∀X ∈ H₀: g(X) = f(ωX)
pub fn shift_wrap_eval<const N: usize, C: FpConfig<N>>(
    h: &Coset<N, C>,
    evals: Evals<N, C>,
) -> Evals<N, C> {
    let mut evals_new = evals.evals;
    let evals_new_first = evals_new.remove(0);
    evals_new.push(evals_new_first);
    Evaluations::from_vec_and_domain(evals_new, h.domain)
}

/// f(X) = p₀(X) + Xⁿp₁(X) + X²ⁿp₂(X) + ...
pub fn split<const N: usize, C: FpConfig<N>>(
    n: u64,
    f: &DensePolynomial<Fp<C, N>>,
) -> Vec<DensePolynomial<Fp<C, N>>> {
    f.coeffs
        .chunks(n as usize)
        .map(DensePolynomial::from_coefficients_slice)
        .collect()
}

// TODO Coset needs to generalize
/// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
pub fn lagrange_basis<const N: usize, C: FpConfig<N>>(h: &Coset<N, C>, i: u64) -> Poly<N, C> {
    let wi = h.w(i);
    let numerator = (xn(h.n()) + deg0(Fp::ONE)) * wi;
    let denominator = (x() - deg0(wi)) * Fp::from(h.n());
    numerator / denominator
}

// /// Zₕ(X) = Xⁿ - 1
// /// such that ∀X ∈ H₀: Zₕ(X) = 0
// pub fn zh_poly(h: &Coset) -> Poly {
//     xn_poly(h.n()) - deg0(&Scalar::ONE)
// }

pub fn batch_evaluate<'a, const N: usize, C, I>(ps: I, x: Fp<C, N>) -> Vec<Fp<C, N>>
where
    C: FpConfig<N>,
    I: IntoIterator<Item = &'a DensePolynomial<Fp<C, N>>>,
{
    batch_op(ps, |f| f.evaluate(&x))
}

// TODO pcdl needs to generalize
pub fn batch_commit<'a, const N: usize, C: FpConfig<N>, P: SWCurveConfig, I>(
    ps: I,
    d: usize,
    w: Option<&Fp<C, N>>,
) -> Vec<Point<P>>
where
    I: IntoIterator<Item = &'a Fp<C, N>>,
{
    batch_op(ps, |f| pcdl::commit(f, d, w))
}

#[cfg(test)]
mod tests {
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
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
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
