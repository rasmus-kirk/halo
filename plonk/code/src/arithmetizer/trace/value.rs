use super::WireID;
use crate::{
    arithmetizer::{cache::ArithWireCache, plookup::PlookupOps},
    utils::{misc::map_to_alphabet, print_table::print_scalar, Scalar},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use educe::Educe;
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Educe)]
#[educe(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub enum Value<P: SWCurveConfig> {
    AnonWire(Scalar<P>),
    Wire(WireID, ValueType, Scalar<P>),
}

impl<P: SWCurveConfig> Default for Value<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: SWCurveConfig> Value<P> {
    pub const ZERO: Self = Self::AnonWire(Scalar::<P>::ZERO);
    pub const ONE: Self = Self::AnonWire(Scalar::<P>::ONE);

    pub fn to_fp(self) -> Scalar<P> {
        match self {
            Self::AnonWire(scalar) => scalar,
            Self::Wire(_, _, scalar) => scalar,
        }
    }

    pub fn ref_to_fp(&self) -> Scalar<P> {
        match self {
            Self::AnonWire(scalar) => *scalar,
            Self::Wire(_, _, scalar) => *scalar,
        }
    }

    pub fn neg_one() -> Self {
        Self::AnonWire(-Scalar::<P>::ONE)
    }

    pub fn new_wire(wire: WireID, value: Scalar<P>) -> Self {
        Self::Wire(wire, ValueType::Field, value)
    }

    /// Check if the value is an anonymous wire.
    pub fn is_anon(&self) -> bool {
        matches!(self, Self::AnonWire(_))
    }

    /// Check if the value scalar is zero.
    pub fn is_zero(&self) -> bool {
        self.to_fp() == Scalar::<P>::ZERO
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
    pub fn set_bit_type<Op: PlookupOps>(self, cache: &ArithWireCache<Op, P>) -> Self {
        match self {
            Self::Wire(id, _, scalar) if cache.is_bit(id) => Self::Wire(id, ValueType::Bit, scalar),
            x => x,
        }
    }
}

impl<P: SWCurveConfig> Neg for Value<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::AnonWire(scalar) => Value::AnonWire(-scalar),
            Value::Wire(_, _, scalar) => Value::AnonWire(-scalar),
        }
    }
}

impl<P: SWCurveConfig> Add for Value<P> {
    type Output = Value<P>;

    fn add(self, other: Self) -> Self::Output {
        Value::AnonWire(self.to_fp() + other.to_fp())
    }
}

impl<P: SWCurveConfig> Mul for Value<P> {
    type Output = Value<P>;

    fn mul(self, other: Self) -> Self::Output {
        Value::AnonWire(self.to_fp() * other.to_fp())
    }
}

impl<P: SWCurveConfig> fmt::Display for Value<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::AnonWire(scalar) => write!(f, "{}", print_scalar::<P>(scalar)),
            Value::Wire(wire_id, val_type, scalar) => {
                write!(
                    f,
                    "{} {}:{}",
                    map_to_alphabet(wire_id),
                    print_scalar::<P>(scalar),
                    val_type
                )
            }
        }
    }
}
