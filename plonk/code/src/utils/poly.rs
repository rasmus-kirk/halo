use crate::Coset;

use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl,
};

use ark_ff::{AdditiveGroup, Field, Zero};
use ark_poly::{DenseUVPolynomial, Evaluations, Polynomial};

use super::misc::batch_op;

type Poly = PallasPoly;
type Scalar = PallasScalar;
type Point = PallasPoint;
type Evals = Evaluations<Scalar>;

pub fn batch_interpolate(es: Vec<Evals>) -> Vec<Poly> {
    batch_op(es, |e| e.interpolate())
}

/// f(X) = v
pub fn deg0(v: &Scalar) -> Poly {
    Poly::from_coefficients_slice(&[*v])
}

/// f(X) = vXⁿ
pub fn vxn(v: &Scalar, n: u64) -> Poly {
    let mut coeffs = vec![Scalar::ZERO; n as usize];
    coeffs.push(*v);
    Poly::from_coefficients_slice(&coeffs)
}

/// f(X) = Xⁿ
pub fn xn(n: u64) -> Poly {
    vxn(&Scalar::ONE, n)
}

/// f(X) = X
pub fn x() -> Poly {
    vxn(&Scalar::ONE, 1)
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
pub fn shift_wrap_eval(h: &Coset, evals: Evals) -> Evals {
    let mut evals_new = evals.evals;
    let evals_new_first = evals_new.remove(0);
    evals_new.push(evals_new_first);
    Evaluations::from_vec_and_domain(evals_new, h.domain)
}

/// f(X) = p₀(X) + Xⁿp₁(X) + X²ⁿp₂(X) + ...
pub fn split(n: usize, f: &Poly) -> Vec<Poly> {
    f.coeffs
        .chunks(n)
        .map(Poly::from_coefficients_slice)
        .collect()
}

/// f(X) = p₀(X) + ap₁(X) + a²p₂(X) + ...
pub fn linear_comb<'a, I>(a: &Scalar, ps: I) -> Poly
where
    I: IntoIterator<Item = &'a Poly>,
{
    ps.into_iter()
        .enumerate()
        .fold(Poly::zero(), |acc, (i, p_i)| {
            acc + deg0(&a.pow([i as u64])) * p_i
        })
}

/// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
pub fn lagrange_basis(h: &Coset, i: u64) -> Poly {
    let wi = &h.w(i);
    let numerator = (xn(h.n()) + deg0(&PallasScalar::ONE)) * *wi;
    let denominator = (x() - deg0(wi)) * PallasScalar::from(h.n());
    numerator / denominator
}

// /// Zₕ(X) = Xⁿ - 1
// /// such that ∀X ∈ H₀: Zₕ(X) = 0
// pub fn zh_poly(h: &Coset) -> Poly {
//     xn_poly(h.n()) - deg0(&Scalar::ONE)
// }

/// Y = x₀y₀ + x₁y₁ + x₂y₂ + ...
pub fn hadamard(xs: &[Poly], ys: &[Poly]) -> Poly {
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| x * y)
        .reduce(|acc, x| acc + x)
        .unwrap()
}

pub fn batch_evaluate<'a, I>(ps: I, x: &Scalar) -> Vec<Scalar>
where
    I: IntoIterator<Item = &'a Poly>,
{
    batch_op(ps, |f| f.evaluate(x))
}

pub fn batch_commit<'a, I>(ps: I, d: usize, w: Option<&Scalar>) -> Vec<Point>
where
    I: IntoIterator<Item = &'a Poly>,
{
    batch_op(ps, |f| pcdl::commit(f, d, w))
}

#[cfg(test)]
mod tests {
    use crate::scheme::Slots;
    use ark_poly::Polynomial;

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
                    assert_eq!(l.evaluate(&h.w(j)), Scalar::ONE);
                } else {
                    assert_eq!(l.evaluate(&h.w(j)), Scalar::ZERO);
                }
            }
        }
    }
}
