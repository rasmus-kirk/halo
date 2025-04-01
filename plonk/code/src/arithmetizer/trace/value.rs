use super::WireID;
use crate::{
    arithmetizer::{cache::ArithWireCache, plookup::PlookupOps},
    utils::{misc::map_to_alphabet, print_table::print_scalar},
};

use halo_accumulation::group::PallasScalar;

use ark_ff::{AdditiveGroup, Field};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

type Scalar = PallasScalar;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    AnonWire(Scalar),
    Wire(WireID, ValueType, Scalar),
}

impl Default for Value {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Value {
    pub const ZERO: Self = Self::AnonWire(Scalar::ZERO);
    pub const ONE: Self = Self::AnonWire(Scalar::ONE);

    pub fn neg_one() -> Self {
        Self::AnonWire(-Scalar::ONE)
    }

    pub fn new_wire(wire: WireID, value: Scalar) -> Self {
        Self::Wire(wire, ValueType::Field, value)
    }

    /// Check if the value is an anonymous wire.
    pub fn is_anon(&self) -> bool {
        matches!(self, Self::AnonWire(_))
    }

    /// Check if the value scalar is zero.
    pub fn is_zero(&self) -> bool {
        Into::<Scalar>::into(*self) == Scalar::ZERO
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
    pub fn set_bit_type<Op: PlookupOps>(self, cache: &ArithWireCache<Op>) -> Self {
        match self {
            Self::Wire(id, _, scalar) if cache.is_bit(id) => Self::Wire(id, ValueType::Bit, scalar),
            x => x,
        }
    }
}

impl From<Value> for Scalar {
    fn from(value: Value) -> Self {
        match value {
            Value::AnonWire(scalar) => scalar,
            Value::Wire(_, _, scalar) => scalar,
        }
    }
}

impl From<&Value> for Scalar {
    fn from(value: &Value) -> Self {
        Into::<Scalar>::into(*value)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::AnonWire(scalar) => Value::AnonWire(-scalar),
            Value::Wire(_, _, scalar) => Value::AnonWire(-scalar),
        }
    }
}

impl Neg for &Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Self::Output {
        Value::AnonWire(Into::<Scalar>::into(self) + Into::<Scalar>::into(other))
    }
}

impl Add for &Value {
    type Output = Value;

    fn add(self, other: Self) -> Self::Output {
        *self + *other
    }
}

impl Add<&Value> for Value {
    type Output = Value;

    fn add(self, other: &Value) -> Self::Output {
        self + *other
    }
}

impl Add<Value> for &Value {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        *self + other
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self::Output {
        Value::AnonWire(Into::<Scalar>::into(self) * Into::<Scalar>::into(other))
    }
}

impl Mul for &Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self::Output {
        *self * *other
    }
}

impl Mul<&Value> for Value {
    type Output = Value;

    fn mul(self, other: &Value) -> Self::Output {
        self * *other
    }
}

impl Mul<Value> for &Value {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        *self * other
    }
}

impl fmt::Display for Value {
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
