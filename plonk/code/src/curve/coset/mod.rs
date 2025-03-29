mod poly;

use crate::{curve::Scalar, util::misc::to_superscript};

use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use halo_accumulation::group::PallasScalar;
use rand::Rng;
use std::fmt;

/// Base coset scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coset {
    /// n:ℕ <=> ωⁿ = 1
    n: u64,
    /// ω:𝔽
    w: Scalar,
    /// k:𝔽
    ks: Vec<Scalar>,
    pub coset_domain: GeneralEvaluationDomain<PallasScalar>,
    pub domain: GeneralEvaluationDomain<PallasScalar>,
}

impl Default for Coset {
    fn default() -> Self {
        Coset {
            n: 0,
            w: Scalar::ZERO,
            ks: Vec::new(),
            coset_domain: GeneralEvaluationDomain::<PallasScalar>::new(0).unwrap(),
            domain: GeneralEvaluationDomain::<PallasScalar>::new(0).unwrap(),
        }
    }
}

impl fmt::Display for Coset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ω{} = 1", to_superscript(self.n()))
    }
}

impl Coset {
    /// m is the number of elements (excluding 1) the cylic group should have.
    /// l is the number cosets in the group. (minimum 1)
    pub fn new<R: Rng>(rng: &mut R, m: u64, l: usize) -> Option<Self> {
        assert!(l > 0);
        let n = (m + 1).next_power_of_two();
        let w = Scalar::get_root_of_unity(n)?;
        let domain = GeneralEvaluationDomain::<PallasScalar>::new(n as usize).unwrap();
        let coset_domain = domain.get_coset(w.into()).unwrap();
        let mut nw = Coset {
            n,
            w,
            ks: Vec::new(),
            coset_domain,
            domain,
        };
        let mut ks = vec![Scalar::ZERO; l];
        ks[0] = Scalar::ONE;
        for i in 1..l {
            ks[i] = loop {
                let k_ = rng.gen();
                if k_ != Scalar::ZERO
                    && !nw.vec().contains(&k_)
                    && !ks[1..i].iter().any(|&k| nw.vec_mul(&k).contains(&k_))
                {
                    break k_;
                }
            };
        }
        nw.ks = ks;
        Some(nw)
    }

    /// number of elements in one coset
    pub fn n(&self) -> u64 {
        self.n
    }

    /// number of cosets
    pub fn l(&self) -> usize {
        self.ks.len()
    }

    /// ωⁱ:𝔽
    pub fn w(&self, i: u64) -> Scalar {
        self.w.pow(i)
    }

    // Hₛ = { kₛ ωⁱ | 1 ≤ i < n }
    pub fn h<T: Into<usize>>(&self, slot: T, i: u64) -> Scalar {
        self.ks[slot.into()] * self.w(i)
    }

    /// [1, n)
    pub fn iter(&self) -> impl Iterator<Item = u64> {
        1..self.n
    }

    /// { ωⁱ | 1 ≤ i < n }
    pub fn vec(&self) -> Vec<Scalar> {
        self.iter().map(|i| self.w(i)).collect()
    }

    /// { k ωⁱ | 1 ≤ i < n }
    pub fn vec_mul(&self, k: &Scalar) -> Vec<Scalar> {
        self.vec().iter().map(|h| k * h).collect()
    }

    pub fn vec_k<T: Into<usize>>(&self, slot: T) -> Vec<Scalar> {
        self.vec_mul(&self.ks[slot.into()])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::protocol::scheme::Slots;

    use super::*;

    #[test]
    fn coset() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        assert_eq!(h.n, 8);
        let h_vec = h.vec();
        for w in h_vec.iter() {
            assert!(w != &Scalar::ONE);
        }
        assert!(h.w(0) == Scalar::ONE);
        assert_eq!(h.w(0), h.w(h.n));
        for i in h.iter() {
            assert_eq!(h.w(i), h.w(h.n + i));
            assert_eq!(h.w(i + 1), h.w(i) * h.w);
        }
    }

    #[test]
    fn coset_with_k() {
        let rng = &mut rand::thread_rng();
        let h = Coset::new(rng, 3, Slots::COUNT).unwrap();
        let mut set = HashSet::new();
        for i in h.iter() {
            set.insert(h.h(Slots::A, i));
            set.insert(h.h(Slots::B, i));
            set.insert(h.h(Slots::C, i));
        }
        assert_eq!(set.len() as u64, 3 * (h.n - 1));
        for w in set {
            assert!(w != Scalar::ZERO);
        }
    }
}
