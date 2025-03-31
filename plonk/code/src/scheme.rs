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
        [Slots::A, Slots::B, Slots::C].iter().copied()
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
        .iter()
        .copied()
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

/// Canonical equations used in the protocol are defined here
pub mod eqns {
    use std::{
        fmt::Debug,
        ops::{Add, Mul, Sub},
    };

    use ark_ff::{Fp, FpConfig};

    use crate::utils::general::{dot, geometric};

    use super::*;

    /// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI
    pub fn plonk_eqn<const N: usize, C: FpConfig<N>, P, T, I1, I2>(ws: I1, qs: I2, pip: T) -> P
    where
        I1: IntoIterator<Item = T>,
        I2: IntoIterator<Item = T>,
        P: Default + Add<P, Output = P> + Add<T, Output = P> + Mul<T, Output = P>,
        T: Mul<Fp<C, N>, Output = P> + Mul<T, Output = P> + Mul<P, Output = P> + Debug + Copy,
    {
        let [a, b, c] = ws.into_iter().collect::<Vec<T>>().try_into().unwrap();
        let [ql, qr, qo, qm, qc, _, _] = qs.into_iter().collect::<Vec<T>>().try_into().unwrap();

        dot([a, b, c], [ql, qr, qo]) + (a * b * qm) + qc + pip
    }

    pub fn plonk_eqn_str(terms: [String; Terms::COUNT]) -> String {
        let [a, b, c, ql, qr, qo, qm, qc, _, _, pi] = terms;
        format!(
            "{} × {} + {} × {} + {} × {} + {} × {} × {} + {} + {}",
            a, ql, b, qr, c, qo, a, b, qm, qc, pi,
        )
    }

    /// a + ζb + ζ²c + ζ³j
    pub fn plookup_compress<const N: usize, C, P, T>(zeta: Fp<C, N>, a: T, b: T, c: T, j: T) -> P
    where
        C: FpConfig<N>,
        P: Default + Add<P, Output = P>,
        T: Mul<Fp<C, N>, Output = P>,
    {
        geometric(zeta, [a, b, c, j])
    }

    // Qₖ(a + ζb + ζ²c + ζ³j - f)
    pub fn plookup_eqn<const N: usize, C, P, T, I1, I2>(zeta: Fp<C, N>, ws: I1, qs: I2, f: T) -> P
    where
        I1: IntoIterator<Item = T>,
        I2: IntoIterator<Item = T>,
        C: FpConfig<N>,
        P: Default + Add<P, Output = P> + Sub<T, Output = P>,
        T: Mul<Fp<C, N>, Output = P> + Mul<P, Output = P> + Debug,
    {
        let [a, b, c] = ws.into_iter().collect::<Vec<_>>().try_into().unwrap();
        let [_, _, _, _, _, qk, j] = qs.into_iter().collect::<Vec<_>>().try_into().unwrap();
        qk * (plookup_compress(zeta, a, b, c, j) - f)
    }

    /// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI(X) + Qₖ(a + ζb + ζ²c + ζ³j - f)
    pub fn plonkup_eqn<const N: usize, C: FpConfig<N>, T, P, I1, I2>(
        zeta: Fp<C, N>,
        ws: I1,
        qs: I2,
        pip: T,
        f: T,
    ) -> P
    where
        I1: IntoIterator<Item = T> + Clone,
        I2: IntoIterator<Item = T> + Clone,
        P: Default
            + Add<P, Output = P>
            + Add<T, Output = P>
            + Mul<T, Output = P>
            + Sub<T, Output = P>,
        T: Mul<Fp<C, N>, Output = P>
            + Mul<Fp<C, N>, Output = P>
            + Mul<T, Output = P>
            + Mul<P, Output = P>
            + Debug
            + Copy,
    {
        plonk_eqn(ws.clone(), qs.clone(), pip) + plookup_eqn(zeta, ws, qs, f)
    }

    // TODO copy constraint term, plookup constraint term, grand product f, grand product g
}
