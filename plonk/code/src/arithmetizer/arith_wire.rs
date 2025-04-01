use super::{plookup::PlookupOps, ArithmetizerError, WireID};
use crate::utils::misc::map_to_alphabet;

use halo_accumulation::group::PallasScalar;

use std::fmt;

type Scalar = PallasScalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArithWire {
    Input(WireID),
    Constant(Scalar),
    AddGate(WireID, WireID),
    MulGate(WireID, WireID),
    Lookup(PlookupOps, WireID, WireID),
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
    pub fn inputs(&self) -> impl Iterator<Item = WireID> {
        match *self {
            Self::AddGate(lhs, rhs) => vec![lhs, rhs].into_iter(),
            Self::MulGate(lhs, rhs) => vec![lhs, rhs].into_iter(),
            Self::Lookup(_, lhs, rhs) => vec![lhs, rhs].into_iter(),
            _ => vec![].into_iter(),
        }
    }
}

/// The types of gates that are commutative.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommutativeOps {
    Add,
    Mul,
    Lookup(PlookupOps),
}

impl TryFrom<ArithWire> for CommutativeOps {
    type Error = ArithmetizerError;

    fn try_from(val: ArithWire) -> Result<Self, Self::Error> {
        match val {
            ArithWire::AddGate(_, _) => Ok(CommutativeOps::Add),
            ArithWire::MulGate(_, _) => Ok(CommutativeOps::Mul),
            ArithWire::Lookup(op, _, _) => match op {
                PlookupOps::Xor => Ok(CommutativeOps::Lookup(PlookupOps::Xor)),
                PlookupOps::Or => Ok(CommutativeOps::Lookup(PlookupOps::Or)),
            },
            _ => Err(ArithmetizerError::CommutativeSetTypeConversionError(val)),
        }
    }
}
