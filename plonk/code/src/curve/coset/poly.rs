use std::time::Instant;

use ark_poly::{EvaluationDomain, Evaluations, GeneralEvaluationDomain};
use halo_accumulation::group::PallasScalar;
use log::info;
use rayon::prelude::*;

use crate::curve::{Poly, Scalar};

use super::Coset;

impl Coset {
    /// Interpolate polynomial from evaluations.
    /// p(xᶦ) = yᵢ
    pub fn interpolate(&self, y: Vec<Scalar>) -> Poly {
        let domain = GeneralEvaluationDomain::<PallasScalar>::new(self.n as usize).unwrap();
        let evals = y
            .clone()
            .into_iter()
            .map(Into::<PallasScalar>::into)
            .collect::<Vec<_>>();
        let poly = Evaluations::from_vec_and_domain(evals, domain).interpolate();
        Poly::new_cache(poly, y)
    }

    /// Interpolate to a zero free polynomial where the first coefficient is zero.
    /// p(ω⁰) = 0
    pub fn interpolate_zf(&self, y: Vec<Scalar>) -> Poly {
        let mut y_zf = vec![Scalar::ZERO];
        let y_len = y.len() + 1;
        assert!(y_len <= self.n as usize);
        y_zf.extend(y);
        y_zf.extend(vec![Scalar::ZERO; self.n as usize - y_len]);
        self.interpolate(y_zf)
    }

    /// Given a polynomial f(X) and a scalar a, return a polynomial g(X) such that:
    /// ∀X ∈ H₀: g(X) = f(aX)
    pub fn poly_times_arg(&self, f: &Poly, a: &Scalar) -> Poly {
        let mut points = Vec::with_capacity(self.n() as usize);

        const PARALLEL: bool = true;
        if PARALLEL {
            points = (0..self.n()).into_par_iter().map(|i| f.evaluate(&(self.w(i) * a))).collect();
        } else {
            for i in 0..self.n() {
                let x = self.w(i) * a;
                points.push(f.evaluate(&x));
            }
        }
        self.interpolate(points)
    }

    /// Zₕ(X) = Xⁿ - 1 s.t.
    /// ∀X ∈ H₀: Zₕ(X) = 0
    pub fn zh(&self) -> Poly {
        Poly::x(self.n) - Poly::a(&Scalar::ONE)
    }

    /// Lᵢ(X) = (ωⁱ (Xⁿ - 1)) / (n (X - ωⁱ))
    pub fn lagrange(&self, i: u64) -> Poly {
        let wi = &Poly::a(&self.w(i));
        let numerator = wi * (Poly::x(self.n) - Poly::a(&-Scalar::ONE));
        let denominator = Poly::a(&self.n.into()) * (Poly::x(1) - wi);
        numerator / denominator
    }

    /// L₁(X) = (Xⁿ - 1) / (n (X - 1))
    pub fn l1_ev(&self, x: &Scalar) -> Scalar {
        self.w * (x.pow(self.n) - Scalar::ONE) / (self.n * (x - self.w))
    }

    pub fn evaluate(&self, p: &Poly, i: u64) -> Scalar {
        if let Some(y) = p.cache(i) {
            y
        } else {
            p.evaluate(&self.w(i))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::scheme::Slots;
    use rand::Rng;

    use super::*;

    #[test]
    fn interpolate() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let evals = &vec![1.into(), 2.into(), 3.into(), 4.into()];
        let p = h.interpolate(evals.clone());
        for i_ in 0..evals.len() {
            let i = i_ as u64;
            assert_eq!(p.evaluate(&h.w(i)), evals[i_]);
        }
        let len_opt = p.cache_len();
        assert!(len_opt.is_some());
        let len = len_opt.unwrap();
        for i in 0..len {
            let cache_opt = p.cache(i as u64);
            assert!(cache_opt.is_some());
            let cache_val = cache_opt.unwrap();
            assert_eq!(p.evaluate(&h.w(i as u64)), cache_val);
        }
    }

    #[test]
    fn interpolate_zf() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let evals = &vec![1.into(), 2.into(), 3.into(), 4.into()];
        let p = h.interpolate_zf(evals.clone());
        assert_eq!(p.evaluate(&Scalar::ONE), Scalar::ZERO);
        for i_ in 0..evals.len() {
            let i = i_ as u64;
            assert_eq!(p.evaluate(&h.w(i + 1)), evals[i_]);
        }
    }
    #[test]
    fn zh() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        let zh = h.zh();
        for i in h.iter() {
            assert_eq!(zh.evaluate(&h.w(i)), Scalar::ZERO);
        }
    }

    #[test]
    fn lagrange() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        for i in h.iter() {
            let l = h.lagrange(i);
            for j in h.iter() {
                if i == j {
                    assert_eq!(l.evaluate(&h.w(j)), Scalar::ONE);
                } else {
                    assert_eq!(l.evaluate(&h.w(j)), Scalar::ZERO);
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
        let l1 = h.lagrange(1);
        for _ in 0..100 {
            let x = rng.gen();
            assert_eq!(h.l1_ev(&x), l1.evaluate(&x));
        }
    }
}
