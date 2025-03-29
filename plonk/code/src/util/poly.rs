use ark_ff::{AdditiveGroup, Field, Zero};
use ark_poly::{DenseUVPolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain};
use halo_accumulation::group::{PallasPoly, PallasScalar};

use crate::curve::Coset;

type Poly = PallasPoly;
type Scalar = PallasScalar;

/// f(X) = v
pub fn deg0(v: &Scalar) -> Poly {
    PallasPoly::from_coefficients_slice(&[*v])
}

/// f(X) = vXⁿ
pub fn vxn_poly(v: &Scalar, n: u64) -> Poly {
    let mut coeffs = vec![Scalar::ZERO; n as usize];
    coeffs.push(*v);
    PallasPoly::from_coefficients_slice(&coeffs)
}

/// f(X) = Xⁿ
pub fn xn_poly(n: u64) -> Poly {
    vxn_poly(&PallasScalar::ONE, n)
}

/// f(X) = X
pub fn x_poly() -> Poly {
    vxn_poly(&PallasScalar::ONE, 1)
}

/// ∀X ∈ H₀: g(X) = f(aX)
pub fn coset_scale(
    h: &Coset,
    f: &Poly,
    a: Scalar,
    domain: &GeneralEvaluationDomain<Scalar>,
) -> PallasPoly {
    // Step 1: Get the coset domain scaled by `a`
    let coset_domain = domain.get_coset(domain.coset_offset() * a).unwrap();

    // Step 2: Perform FFT on `f` over the coset domain {a * ζ^i}
    let mut evals_new = coset_domain.fft(&f.coeffs);
    let evals_new_last = evals_new.pop().unwrap();
    evals_new.insert(0, evals_new_last);

    // Step 3: Perform inverse FFT to interpolate the new polynomial g(X)
    let domain2 = GeneralEvaluationDomain::<Scalar>::new(h.n() as usize).unwrap();
    let poly = Evaluations::from_vec_and_domain(evals_new, domain2).interpolate();
    poly
}

/// ∀X ∈ H₀: g(X) = f(ωX)
pub fn coset_scale_omega(h: &Coset, f: &Poly, domain: &GeneralEvaluationDomain<Scalar>) -> Poly {
    coset_scale(h, f, h.w(1).scalar, domain)
}

/// f(X) = p₀(X) + Xⁿp₁(X) + X²ⁿp₂(X) + ...
/// where n = |H₀|
pub fn split_poly(h: &Coset, f: &Poly) -> Vec<Poly> {
    f.coeffs
        .chunks(h.n() as usize)
        .map(PallasPoly::from_coefficients_slice)
        .collect()
}

/// f(X) = p₀(X) + ap₁(X) + a²p₂(X) + ...
pub fn linear_comb_poly<'a, I>(a: &Scalar, ps: I) -> Poly
where
    I: IntoIterator<Item = &'a Poly>,
{
    let mut p = Poly::zero();
    for (i, p_i) in ps.into_iter().enumerate() {
        p = p + (deg0(&a.pow([i as u64])) * p_i.clone());
    }
    p
}

/// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
pub fn lagrange_basis_poly(h: &Coset, i: u64) -> Poly {
    let wi = &h.w(i).scalar;
    let numerator = (xn_poly(h.n()) + deg0(&PallasScalar::ONE)) * *wi;
    let denominator = (x_poly() - deg0(wi)) * PallasScalar::from(h.n());
    numerator / denominator
}

/// L₁(X) = (Xⁿ - 1) / (n (X - 1))
pub fn lagrange_basis1_ev(h: &Coset, x: &Scalar) -> Scalar {
    let n = h.n();
    let w = h.w(1).scalar;
    w * (x.pow([n]) - Scalar::ONE) / (Scalar::from(n) * (*x - w))
}

/// Zₕ(X) = Xⁿ - 1
/// such that ∀X ∈ H₀: Zₕ(X) = 0
pub fn zh_poly(h: &Coset) -> Poly {
    xn_poly(h.n()) - deg0(&Scalar::ONE)
}

#[cfg(test)]
mod tests {
    use crate::protocol::scheme::Slots;
    use ark_poly::Polynomial;
    use rand::Rng;

    use super::*;

    #[test]
    fn zh() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let zh = zh_poly(&h);
        for i in h.iter() {
            assert_eq!(zh.evaluate(&h.w(i).scalar), Scalar::ZERO);
        }
    }

    #[test]
    fn lagrange() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        for i in h.iter() {
            let l = lagrange_basis_poly(&h, i);
            for j in h.iter() {
                if i == j {
                    assert_eq!(l.evaluate(&h.w(j).scalar), Scalar::ONE);
                } else {
                    assert_eq!(l.evaluate(&h.w(j).scalar), Scalar::ZERO);
                }
            }
        }
    }

    #[test]
    fn l1_ev() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let l1 = lagrange_basis_poly(&h, 1);
        for _ in 0..100 {
            let x = rng.gen();
            assert_eq!(lagrange_basis1_ev(&h, &x), l1.evaluate(&x));
        }
    }
}
