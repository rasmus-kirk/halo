use std::fmt;

/// Used to determine degree of root of unity along with number of constraints.
pub const MAX_BLIND_TERMS: u64 = 0;

/// Enum of slots in the constraint system; private polynomials.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Slots {
    #[default]
    A,
    B,
    C,
}

impl fmt::Display for Slots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Slots::A => "A",
            Slots::B => "B",
            Slots::C => "C",
        };
        write!(f, "{}", s)
    }
}

impl Slots {
    pub const COUNT: usize = 3;

    pub fn iter() -> impl Iterator<Item = Self> {
        [Slots::A, Slots::B, Slots::C].into_iter()
    }

    pub fn perm_string(&self) -> String {
        match self {
            Slots::A => "S₁".to_string(),
            Slots::B => "S₂".to_string(),
            Slots::C => "S₃".to_string(),
        }
    }
}

impl From<Slots> for usize {
    fn from(slot: Slots) -> Self {
        slot as usize
    }
}

impl From<usize> for Slots {
    fn from(index: usize) -> Self {
        match index {
            0 => Slots::A,
            1 => Slots::B,
            2 => Slots::C,
            _ => panic!("Invalid index for Slots"),
        }
    }
}

/// Enum of selectors in the constraint system; public polynomials.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Selectors {
    #[default]
    Ql,
    Qr,
    Qo,
    Qm,
    Qc,
    Qk,
    J,
}

impl fmt::Display for Selectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Selectors::Ql => "Qₗ",
            Selectors::Qr => "Qᵣ",
            Selectors::Qo => "Qₒ",
            Selectors::Qm => "Qₘ",
            Selectors::Qc => "Q꜀",
            Selectors::Qk => "Qₖ",
            Selectors::J => "J",
        };
        write!(f, "{}", s)
    }
}

impl Selectors {
    pub const COUNT: usize = 7;

    pub fn iter() -> impl Iterator<Item = Self> {
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
}

impl From<Selectors> for usize {
    fn from(selector: Selectors) -> Self {
        selector as usize
    }
}

impl From<usize> for Selectors {
    fn from(index: usize) -> Self {
        match index {
            0 => Selectors::Ql,
            1 => Selectors::Qr,
            2 => Selectors::Qo,
            3 => Selectors::Qm,
            4 => Selectors::Qc,
            5 => Selectors::Qk,
            6 => Selectors::J,
            _ => panic!("Invalid index for Selectors"),
        }
    }
}

/// Enum of slots and selectors in the constraint system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Terms {
    F(Slots),
    Q(Selectors),
    PublicInputs,
}

impl Default for Terms {
    fn default() -> Self {
        Terms::F(Slots::default())
    }
}

impl Terms {
    pub const COUNT: usize = Slots::COUNT + Selectors::COUNT + 1;

    pub fn iter() -> impl Iterator<Item = Self> {
        Slots::iter()
            .map(Terms::F)
            .chain(Selectors::iter().map(Terms::Q))
            .chain(std::iter::once(Terms::PublicInputs))
    }

    pub fn is_slot(&self) -> bool {
        matches!(self, Terms::F(_))
    }

    pub fn is_selector(&self) -> bool {
        matches!(self, Terms::Q(_))
    }

    pub fn index(self) -> usize {
        Into::<usize>::into(self)
    }
}

impl From<Terms> for usize {
    fn from(term: Terms) -> Self {
        match term {
            Terms::F(slot) => slot as usize,
            Terms::Q(selector) => Slots::COUNT + selector as usize,
            Terms::PublicInputs => Slots::COUNT + Selectors::COUNT,
        }
    }
}

impl From<usize> for Terms {
    fn from(index: usize) -> Self {
        if index < Slots::COUNT {
            Terms::F(Slots::from(index))
        } else if index < Terms::COUNT {
            Terms::Q(Selectors::from(index - Slots::COUNT))
        } else {
            panic!("Invalid index for Terms")
        }
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
