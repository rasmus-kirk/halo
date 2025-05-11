use crate::{
    arithmetizer::{arith_wire::ArithWire, plookup::PlookupOps},
    utils::Scalar,
};

use ark_ec::short_weierstrass::SWCurveConfig;

use educe::Educe;
use std::{error::Error, fmt::Display};

#[derive(Educe)]
#[educe(Debug)]
pub enum CacheError<Op: PlookupOps, P: SWCurveConfig> {
    WireIDNotInCache,
    OperandNotInCache,
    InvalidCommutativeOperator(ArithWire<Op, P>),
    TypeError(TypeError<P>),
}

impl<Op: PlookupOps, P: SWCurveConfig> Error for CacheError<Op, P> {}

impl<Op: PlookupOps, P: SWCurveConfig> Display for CacheError<Op, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::WireIDNotInCache => write!(f, "Cache: WireID not in cache"),
            CacheError::OperandNotInCache => write!(f, "Cache: Operand not in cache"),
            CacheError::InvalidCommutativeOperator(wire) => {
                write!(f, "Cache: Invalid commutative operator: {}", wire)
            }
            CacheError::TypeError(e) => write!(f, "Cache: {}", e),
        }
    }
}

#[derive(Educe)]
#[educe(Debug)]
pub enum TypeError<P: SWCurveConfig> {
    BitErrors(BitError<P>),
}

impl<P: SWCurveConfig> Error for TypeError<P> {}

impl<P: SWCurveConfig> std::fmt::Display for TypeError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::BitErrors(e) => write!(f, "Bit Type error: {}", e),
        }
    }
}

#[derive(Educe)]
#[educe(Debug)]
pub enum BitError<P: SWCurveConfig> {
    ScalarIsNotBit(Scalar<P>),
}

impl<P: SWCurveConfig> std::fmt::Display for BitError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BitError::ScalarIsNotBit(scalar) => {
                write!(f, "Scalar `{}` is not a bit", scalar)
            }
        }
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> From<BitError<P>> for CacheError<Op, P> {
    fn from(e: BitError<P>) -> Self {
        Self::TypeError(TypeError::BitErrors(e))
    }
}

impl<P: SWCurveConfig> Error for BitError<P> {}
