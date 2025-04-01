use super::{ConstraintID, Coset};
use crate::{
    scheme::Slots,
    utils::misc::{to_superscript, EnumIter},
};

use halo_accumulation::group::PallasScalar;

use std::fmt;

type Scalar = PallasScalar;

/// Position in the permutation polynomial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub slot: Slots,
    pub id: ConstraintID,
}

impl Pos {
    pub fn new(slot: Slots, id: ConstraintID) -> Self {
        Self { slot, id }
    }

    pub fn id(&self) -> usize {
        self.id as usize
    }

    /// Convert the position to a scalar used in the permutation polynomial.
    pub fn to_scalar(self, scheme: &Coset) -> Scalar {
        scheme.h(self.slot, self.id)
    }

    pub fn from_scalar(scalar: Scalar, scheme: &Coset) -> Option<Self> {
        for slot in Slots::iter() {
            for (i_, &x) in scheme.vec_k(slot).iter().enumerate() {
                let i = (i_ + 1) as u64;
                if x == scalar {
                    return Some(Self::new(slot, i));
                }
            }
        }
        None
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id_str = to_superscript(self.id);
        match self.slot {
            Slots::A => write!(f, "ω{}", id_str),
            Slots::B => write!(f, "k₁ ω{}", id_str),
            Slots::C => write!(f, "k₂ ω{}", id_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos() {
        let pos = Pos {
            slot: Slots::A,
            id: 1,
        };
        assert_eq!(format!("{}", pos), "ω¹");
        let pos = Pos {
            slot: Slots::B,
            id: 1,
        };
        assert_eq!(format!("{}", pos), "k₁ ω¹");
        let pos = Pos {
            slot: Slots::C,
            id: 1,
        };
        assert_eq!(format!("{}", pos), "k₂ ω¹");
    }
}
