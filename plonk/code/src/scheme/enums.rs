use crate::utils::misc::EnumIter;

use educe::Educe;
use std::fmt;

/// Used to determine degree of root of unity along with number of constraints.
pub const MAX_BLIND_TERMS: u64 = 0;

/// Enum of slots in the constraint system; private polynomials.
#[derive(Educe)]
#[educe(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Slots {
    #[educe(Default)]
    A,
    B,
    C,
}

impl EnumIter for Slots {
    const COUNT: usize = 3;

    fn iter() -> impl Iterator<Item = Self> {
        [Slots::A, Slots::B, Slots::C].into_iter()
    }

    fn id(self) -> usize {
        self as usize
    }
}

impl fmt::Display for Slots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ["A", "B", "C"][self.id()])
    }
}

impl Slots {
    pub fn perm_string(&self) -> String {
        ["S₁", "S₂", "S₃"][self.id()].to_string()
    }
}

/// Enum of selectors in the constraint system; public polynomials.'
#[derive(Educe)]
#[educe(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Selectors {
    #[educe(Default)]
    Ql,
    Qr,
    Qo,
    Qm,
    Qc,
    Qk,
    J,
}

impl EnumIter for Selectors {
    const COUNT: usize = 7;

    fn iter() -> impl Iterator<Item = Self> {
        [
            Selectors::Ql,
            Selectors::Qr,
            Selectors::Qo,
            Selectors::Qm,
            Selectors::Qc,
            Selectors::Qk,
            Selectors::J,
        ]
        .into_iter()
    }

    fn id(self) -> usize {
        self as usize
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            ["Qₗ", "Qᵣ", "Qₒ", "Qₘ", "Q꜀", "Qₖ", "J",][self.id()]
        )
    }
}

/// Enum of slots and selectors in the constraint system.
#[derive(Educe)]
#[educe(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Terms {
    F(Slots),
    Q(Selectors),
    PublicInputs,
}

impl Default for Terms {
    fn default() -> Self {
        Terms::F(Default::default())
    }
}

impl EnumIter for Terms {
    const COUNT: usize = Slots::COUNT + Selectors::COUNT + 1;

    fn iter() -> impl Iterator<Item = Self> {
        Slots::iter()
            .map(Terms::F)
            .chain(Selectors::iter().map(Terms::Q))
            .chain(std::iter::once(Terms::PublicInputs))
    }

    fn id(self) -> usize {
        Self::iter().position(|term| term == self).unwrap()
    }
}

impl Terms {
    pub fn is_slot(&self) -> bool {
        matches!(self, Terms::F(_))
    }

    pub fn is_selector(&self) -> bool {
        matches!(self, Terms::Q(_))
    }
}

impl fmt::Display for Terms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terms::F(slot) => write!(f, "{}", slot),
            Terms::Q(selector) => write!(f, "{}", selector),
            Terms::PublicInputs => write!(f, "PI"),
        }
    }
}
