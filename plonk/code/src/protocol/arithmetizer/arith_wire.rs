use super::{plonkup::PlonkupOps, ArithmetizerError, WireID};
use crate::{curve::Scalar, util::misc::map_to_alphabet};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArithWire {
    Input(WireID),
    Constant(Scalar),
    AddGate(WireID, WireID),
    MulGate(WireID, WireID),
    Lookup(PlonkupOps, WireID, WireID),
}

impl fmt::Display for ArithWire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArithWire::Input(wire_id) => write!(f, "Input({})", map_to_alphabet(wire_id)),
            ArithWire::Constant(scalar) => write!(f, "Constant({})", scalar),
            ArithWire::AddGate(lhs, rhs) => {
                write!(f, "Add({}, {})", map_to_alphabet(lhs), map_to_alphabet(rhs))
            }
            ArithWire::MulGate(lhs, rhs) => {
                write!(f, "Mul({}, {})", map_to_alphabet(lhs), map_to_alphabet(rhs))
            }
            ArithWire::Lookup(op, lhs, rhs) => {
                write!(
                    f,
                    "Lookup({}, {}, {})",
                    op,
                    map_to_alphabet(lhs),
                    map_to_alphabet(rhs)
                )
            }
        }
    }
}

impl ArithWire {
    /// Get the inputs of the gate, if the wire is a gate.
    pub fn inputs(&self) -> Vec<WireID> {
        match self {
            Self::AddGate(lhs, rhs) => vec![*lhs, *rhs],
            Self::MulGate(lhs, rhs) => vec![*lhs, *rhs],
            Self::Lookup(_, lhs, rhs) => vec![*lhs, *rhs],
            _ => vec![],
        }
    }
}

/// The types of gates that are commutative.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommutativeOps {
    Add,
    Mul,
    Lookup(PlonkupOps),
}

impl TryFrom<ArithWire> for CommutativeOps {
    type Error = ArithmetizerError;

    fn try_from(val: ArithWire) -> Result<Self, Self::Error> {
        match val {
            ArithWire::AddGate(_, _) => Ok(CommutativeOps::Add),
            ArithWire::MulGate(_, _) => Ok(CommutativeOps::Mul),
            ArithWire::Lookup(op, _, _) => match op {
                PlonkupOps::Xor => Ok(CommutativeOps::Lookup(PlonkupOps::Xor)),
                PlonkupOps::Or => Ok(CommutativeOps::Lookup(PlonkupOps::Or)),
            },
            _ => Err(ArithmetizerError::CommutativeSetTypeConversionError(val)),
        }
    }
}
