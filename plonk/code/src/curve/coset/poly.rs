use ark_poly::{EvaluationDomain, Evaluations, GeneralEvaluationDomain};
use halo_accumulation::group::PallasScalar;

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
}
