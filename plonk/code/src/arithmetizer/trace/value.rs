use super::WireID;
use crate::{
    arithmetizer::{cache::ArithWireCache, plookup::PlookupOps},
    utils::{misc::map_to_alphabet, print_table::print_scalar},
};

use ark_ff::{AdditiveGroup, Field, Fp, FpConfig};
use educe::Educe;
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    Bit,
    Field,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Bit => write!(f, "𝔹 "),
            ValueType::Field => write!(f, "𝔽 "),
        }
    }
}

/// Possible evaluation values
///
#[derive(Educe)]
#[educe(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Value<const N: usize, C: FpConfig<N>> {
    AnonWire(Fp<C, N>),
    Wire(WireID, ValueType, Fp<C, N>),
}

impl<const N: usize, C: FpConfig<N>> Default for Value<N, C> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize, C: FpConfig<N>> Value<N, C> {
    pub const ZERO: Self = Self::AnonWire(Fp::ZERO);
    pub const ONE: Self = Self::AnonWire(Fp::ONE);

    pub fn neg_one() -> Self {
        Self::AnonWire(-Fp::ONE)
    }

    pub fn new_wire(wire: WireID, value: Fp<C, N>) -> Self {
        Self::Wire(wire, ValueType::Field, value)
    }

    /// Check if the value is an anonymous wire.
    pub fn is_anon(&self) -> bool {
        matches!(self, Self::AnonWire(_))
    }

    /// Check if the value scalar is zero.
    pub fn is_zero(&self) -> bool {
        Into::<Fp<C, N>>::into(self) == Fp::ZERO
    }

    /// Check if the value is a bit type.
    pub fn is_bit(&self) -> bool {
        matches!(self, Self::Wire(_, ValueType::Bit, _))
    }

    /// Use the scalar of the value and construct a Value::Wire with the given id.
    pub fn set_id(self, id: WireID) -> Self {
        match self {
            Self::AnonWire(scalar) => Self::Wire(id, ValueType::Field, scalar),
            Self::Wire(_, val_type, scalar) => Self::Wire(id, val_type, scalar),
        }
    }

    /// Set the value type of the value to bit if the wire id is a bit.
    pub fn set_bit_type<Op: PlookupOps>(self, cache: &ArithWireCache<Op, N, C>) -> Self {
        match self {
            Self::Wire(id, _, scalar) if cache.is_bit(id) => Self::Wire(id, ValueType::Bit, scalar),
            x => x,
        }
    }
}

impl<const N: usize, C: FpConfig<N>> From<Value<N, C>> for Fp<C, N> {
    fn from(value: Value<N, C>) -> Self {
        match value {
            Value::AnonWire(scalar) => scalar,
            Value::Wire(_, _, scalar) => scalar,
        }
    }
}

impl<const N: usize, C: FpConfig<N>> From<&Value<N, C>> for Fp<C, N> {
    fn from(value: &Value<N, C>) -> Self {
        Into::<Fp<C, N>>::into(value)
    }
}

impl<const N: usize, C: FpConfig<N>> Neg for Value<N, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::AnonWire(scalar) => Value::AnonWire(-scalar),
            Value::Wire(_, _, scalar) => Value::AnonWire(-scalar),
        }
    }
}

impl<const N: usize, C: FpConfig<N>> Add for Value<N, C> {
    type Output = Value<N, C>;

    fn add(self, other: Self) -> Self::Output {
        Value::AnonWire(Into::<Fp<C, N>>::into(self) + Into::<Fp<C, N>>::into(other))
    }
}

impl<const N: usize, C: FpConfig<N>> Mul for Value<N, C> {
    type Output = Value<N, C>;

    fn mul(self, other: Self) -> Self::Output {
        Value::AnonWire(Into::<Fp<C, N>>::into(self) * Into::<Fp<C, N>>::into(other))
    }
}

impl<const N: usize, C: FpConfig<N>> fmt::Display for Value<N, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::AnonWire(scalar) => write!(f, "{}", print_scalar(scalar)),
            Value::Wire(wire_id, val_type, scalar) => {
                write!(
                    f,
                    "{} {}:{}",
                    map_to_alphabet(wire_id),
                    print_scalar(scalar),
                    val_type
                )
            }
        }
    }
}
