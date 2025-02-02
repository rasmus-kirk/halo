use super::WireID;
use crate::{curve::Scalar, util::map_to_alphabet};

use std::{
    fmt,
    ops::{Add, Mul},
};

/// Possible evaluation values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    AnonWire(Scalar),
    Wire(WireID, Scalar),
}

impl Value {
    pub const ZERO: Self = Self::AnonWire(Scalar::ZERO);
    pub const ONE: Self = Self::AnonWire(Scalar::ONE);

    pub fn neg_one() -> Self {
        Self::AnonWire(-Scalar::ONE)
    }

    /// Check if the value scalar is zero.
    pub fn is_zero(&self) -> bool {
        Into::<Scalar>::into(*self) == Scalar::ZERO
    }

    /// Use the scalar of the value and construct a Value::Wire with the given id.
    pub fn set_id(self, id: WireID) -> Self {
        Self::Wire(id, self.into())
    }
}

impl From<Value> for Scalar {
    fn from(value: Value) -> Self {
        match value {
            Value::AnonWire(scalar) => scalar,
            Value::Wire(_, scalar) => scalar,
        }
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
        match self {
            Value::AnonWire(scalar) => write!(f, "{}", scalar),
            Value::Wire(wire_id, scalar) => write!(f, "{}:{}", map_to_alphabet(*wire_id), scalar),
        }
    }
}
