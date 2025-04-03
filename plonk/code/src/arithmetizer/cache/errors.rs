use std::{error::Error, fmt::Display};

use crate::arithmetizer::{arith_wire::ArithWire, plookup::PlookupOps};

use ark_ff::{Fp, FpConfig};

use educe::Educe;

#[derive(Educe)]
#[educe(Debug)]
pub enum CacheError<Op: PlookupOps, const N: usize, C: FpConfig<N>> {
    WireIDNotInCache,
    OperandNotInCache,
    InvalidCommutativeOperator(ArithWire<Op, N, C>),
    TypeError(TypeError<N, C>),
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Error for CacheError<Op, N, C> {}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Display for CacheError<Op, N, C> {
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
pub enum TypeError<const N: usize, C: FpConfig<N>> {
    BitErrors(BitError<N, C>),
}

impl<const N: usize, C: FpConfig<N>> Error for TypeError<N, C> {}

impl<const N: usize, C: FpConfig<N>> std::fmt::Display for TypeError<N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::BitErrors(e) => write!(f, "Bit Type error: {}", e),
        }
    }
}

#[derive(Educe)]
#[educe(Debug)]
pub enum BitError<const N: usize, C: FpConfig<N>> {
    ScalarIsNotBit(Fp<C, N>),
}

impl<const N: usize, C: FpConfig<N>> std::fmt::Display for BitError<N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BitError::ScalarIsNotBit(scalar) => {
                write!(f, "Scalar `{}` is not a bit", scalar)
            }
        }
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> From<BitError<N, C>> for CacheError<Op, N, C> {
    fn from(e: BitError<N, C>) -> Self {
        Self::TypeError(TypeError::BitErrors(e))
    }
}

impl<const N: usize, C: FpConfig<N>> Error for BitError<N, C> {}
