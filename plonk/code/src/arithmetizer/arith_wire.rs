use super::{plookup::PlookupOps, ArithmetizerError, WireID};
use crate::utils::{misc::map_to_alphabet, Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::AdditiveGroup;
use educe::Educe;
use std::fmt::{self, Debug, Display};

#[derive(Educe)]
#[educe(Hash, Clone, Copy, PartialEq, Eq)]
pub enum ArithWire<Op: PlookupOps, P: SWCurveConfig> {
    Input(WireID),
    Constant(Scalar<P>),
    AddGate(WireID, WireID),
    MulGate(WireID, WireID),
    Lookup(Op, WireID, WireID),
}

impl<Op: PlookupOps, P: SWCurveConfig> ArithWire<Op, P> {
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

impl<Op: PlookupOps, P: SWCurveConfig> Default for ArithWire<Op, P> {
    fn default() -> Self {
        ArithWire::Constant(Scalar::<P>::ZERO)
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> Display for ArithWire<Op, P> {
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

impl<Op: PlookupOps, P: SWCurveConfig> Debug for ArithWire<Op, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArithWire: {}", self)
    }
}

/// The types of gates that are commutative.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommutativeOps<Op: PlookupOps> {
    Add,
    Mul,
    Lookup(Op),
}

impl<Op: PlookupOps, P: SWCurveConfig> TryFrom<ArithWire<Op, P>> for CommutativeOps<Op> {
    type Error = ArithmetizerError<Op, P>;

    fn try_from(val: ArithWire<Op, P>) -> Result<Self, Self::Error> {
        match val {
            ArithWire::AddGate(_, _) => Ok(CommutativeOps::Add),
            ArithWire::MulGate(_, _) => Ok(CommutativeOps::Mul),
            ArithWire::Lookup(op, _, _) if op.is_commutative() => Ok(CommutativeOps::Lookup(op)),
            _ => Err(ArithmetizerError::CommutativeSetTypeConversionError(val)),
        }
    }
}
