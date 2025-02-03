mod display;
mod poly;

use super::scheme::Slots;
use crate::curve::Scalar;

use rand::{rngs::ThreadRng, Rng};

/// Base coset scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coset {
    /// n:ℕ <=> ωⁿ = 1
    n: u64,
    /// ω:𝔽
    w: Scalar,
    /// k:𝔽
    ks: [Scalar; Slots::COUNT],
}

impl Default for Coset {
    fn default() -> Self {
        Coset {
            n: 0,
            w: Scalar::ZERO,
            ks: [Scalar::ZERO; Slots::COUNT],
        }
    }
}

impl Coset {
    /// m is the number of elements (excluding 1) the cylic group should have.
    pub fn new(rng: &mut ThreadRng, m: u64) -> Option<Self> {
        let n = (m + 1).next_power_of_two();
        let w = Scalar::get_root_of_unity(n)?;
        let mut nw = Coset {
            n,
            w,
            ks: Default::default(),
        };
        let mut ks = [Scalar::ZERO; Slots::COUNT];
        ks[0] = Scalar::ONE;
        for i in 1..Slots::COUNT {
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

    pub fn n(&self) -> u64 {
        self.n
    }

    /// ωⁱ:𝔽
    pub fn w(&self, i: u64) -> Scalar {
        self.w.pow(i)
    }

    // Hₛ = { kₛ ωⁱ | 1 ≤ i < n }
    pub fn h(&self, slot: Slots, i: u64) -> Scalar {
        self.ks[slot as usize] * self.w(i)
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

    pub fn vec_k(&self, slot: Slots) -> Vec<Scalar> {
        self.vec_mul(&self.ks[slot as usize])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn coset() {
        let rng = &mut rand::thread_rng();
        let h_opt = Coset::new(rng, 5);
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
        let h = Coset::new(rng, 3).unwrap();
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
