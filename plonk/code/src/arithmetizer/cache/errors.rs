use std::fmt::Display;

use crate::arithmetizer::{arith_wire::ArithWire, plookup::PlookupOps};

use halo_accumulation::group::PallasScalar;

type Scalar = PallasScalar;

#[derive(Debug)]
pub enum CacheError<Op: PlookupOps> {
    WireIDNotInCache,
    OperandNotInCache,
    InvalidCommutativeOperator(ArithWire<Op>),
    TypeError(TypeError),
}

impl<Op: PlookupOps> Display for CacheError<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::WireIDNotInCache => write!(f, "Cache: WireID not in cache"),
            CacheError::OperandNotInCache => write!(f, "Cache: Operand not in cache"),
            CacheError::InvalidCommutativeOperator(wire) => {
                write!(f, "Cache: Invalid commutative operator: {:?}", wire)
            }
            CacheError::TypeError(e) => write!(f, "Cache: {}", e),
        }
    }
}

impl<Op: PlookupOps> std::error::Error for CacheError<Op> {}

#[derive(Debug)]
pub enum TypeError {
    BitErrors(BitError),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::BitErrors(e) => write!(f, "Bit Type error: {}", e),
        }
    }
}

impl std::error::Error for TypeError {}

#[derive(Debug)]
pub enum BitError {
    ScalarIsNotBit(Scalar),
}

impl std::fmt::Display for BitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BitError::ScalarIsNotBit(scalar) => {
                write!(f, "Scalar `{}` is not a bit", scalar)
            }
        }
    }
}

impl<Op: PlookupOps> From<BitError> for CacheError<Op> {
    fn from(e: BitError) -> Self {
        Self::TypeError(TypeError::BitErrors(e))
    }
}

impl std::error::Error for BitError {}
