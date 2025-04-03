use crate::utils::misc::{to_superscript, EnumIter};

use ark_ff::{AdditiveGroup, FftField, Field, Fp, FpConfig};
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use educe::Educe;
use rand::Rng;
use std::fmt::{self, Display};

/// Base coset scheme.
#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq)]
pub struct Coset<const N: usize, C: FpConfig<N>> {
    /// n:ℕ <=> ωⁿ = 1
    n: u64,
    /// ω:𝔽
    w: Fp<C, N>,
    /// k:𝔽
    ks: Vec<Fp<C, N>>,
    pub coset_domain: GeneralEvaluationDomain<Fp<C, N>>,
    pub domain: GeneralEvaluationDomain<Fp<C, N>>,
}

impl<const N: usize, C: FpConfig<N>> Default for Coset<N, C> {
    fn default() -> Self {
        Coset {
            n: Default::default(),
            w: Default::default(),
            ks: Default::default(),
            coset_domain: GeneralEvaluationDomain::<Fp<C, N>>::new(0).unwrap(),
            domain: GeneralEvaluationDomain::<Fp<C, N>>::new(0).unwrap(),
        }
    }
}

impl<const N: usize, C: FpConfig<N>> Display for Coset<N, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ω{} = 1", to_superscript(self.n()))
    }
}

impl<const N: usize, C: FpConfig<N>> Coset<N, C> {
    /// m is the number of elements (excluding 1) the cylic group should have.
    /// l is the number cosets in the group. (minimum 1)
    pub fn new<R: Rng>(rng: &mut R, m: u64, l: usize) -> Option<Self> {
        assert!(l > 0);
        let n = (m + 1).next_power_of_two();
        let w = Fp::get_root_of_unity(n)?;
        let domain = GeneralEvaluationDomain::<Fp<C, N>>::new(n as usize).unwrap();
        let coset_domain = domain.get_coset(w).unwrap();
        let mut nw = Coset {
            n,
            w,
            ks: Vec::new(),
            domain,
            coset_domain,
        };
        nw.ks = (1..l).fold(vec![Fp::ONE], |mut acc, _| {
            acc.push(nw.gen_k(rng, acc.as_slice()));
            acc
        });
        Some(nw)
    }

    /// Generate a random k that is not in any previous cosets of `ks`
    fn gen_k<R: Rng>(&self, rng: &mut R, ks: &[Fp<C, N>]) -> Fp<C, N> {
        loop {
            let k_ = rng.gen();
            if k_ != Fp::ZERO
                && !self.vec().contains(&k_)
                && !ks.iter().any(|&k| self.vec_mul(k).contains(&k_))
            {
                return k_;
            }
        }
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
    pub fn w(&self, i: u64) -> Fp<C, N> {
        self.w.pow([i])
    }

    // Hₛ = { kₛ ωⁱ | 1 ≤ i < n }
    pub fn h<T: EnumIter>(&self, slot: T, i: u64) -> Fp<C, N> {
        self.ks[slot.id()] * self.w(i)
    }

    /// [1, n)
    pub fn iter(&self) -> impl Iterator<Item = u64> {
        1..self.n
    }

    /// { ωⁱ | 1 ≤ i < n }
    pub fn vec(&self) -> Vec<Fp<C, N>> {
        self.iter().map(|i| self.w(i)).collect()
    }

    /// { k ωⁱ | 1 ≤ i < n }
    pub fn vec_mul(&self, k: Fp<C, N>) -> Vec<Fp<C, N>> {
        self.vec().iter().map(|h| k * h).collect()
    }

    pub fn vec_k<T: EnumIter>(&self, slot: T) -> Vec<Fp<C, N>> {
        self.vec_mul(self.ks[slot.id()])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{scheme::Slots, utils::misc::EnumIter};

    use halo_accumulation::group::PallasScalar;

    type Scalar = PallasScalar;

    #[test]
    fn coset() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5, Slots::COUNT);
        assert!(h_opt.is_some());
        let h = h_opt.unwrap();
        assert_eq!(h.n, 8);
        for w in h.vec() {
            assert!(w != Scalar::ONE);
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
