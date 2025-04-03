use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::FpConfig;

use super::{
    arith_wire::ArithWire, cache::CacheError, plookup::PlookupOps, trace::TraceError, Wire,
};

use std::{error::Error, fmt::Debug, rc::Rc};

pub enum ArithmetizerError<Op: PlookupOps, const N: usize, C: FpConfig<N>> {
    EmptyOutputWires,
    MismatchedCircuits,
    InvalidInputLength { expected: usize, got: usize },
    EvaluatorError(TraceError<Op, N, C>),
    CacheError(CacheError<Op, N, C>),
    CommutativeSetTypeConversionError(ArithWire<Op, N, C>),
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Error for ArithmetizerError<Op, N, C> {}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> ArithmetizerError<Op, N, C> {
    pub fn validate<T, P: SWCurveConfig>(
        input_values: &[T],
        output_wires: &[Wire<Op, N, C, P>],
    ) -> Result<(), Self> {
        if output_wires.is_empty() {
            return Err(ArithmetizerError::EmptyOutputWires);
        }
        // verify at least one output wire

        let ptr = output_wires[0].arith();
        let circuit = ptr.borrow();
        for w in output_wires.iter() {
            if !Rc::ptr_eq(ptr, w.arith()) {
                return Err(ArithmetizerError::MismatchedCircuits);
            }
        }
        // verify circuit references in output wires

        if input_values.len() != circuit.inputs {
            return Err(ArithmetizerError::InvalidInputLength {
                expected: circuit.inputs,
                got: input_values.len(),
            });
        }
        // verify expected number of input values

        Ok(())
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> std::fmt::Display
    for ArithmetizerError<Op, N, C>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmetizerError::EmptyOutputWires => {
                write!(f, "Arithmetizer: need at least one output wire")
            }
            ArithmetizerError::InvalidInputLength { expected, got } => write!(
                f,
                "Arithmetizer: need exactly {} input values, got {}",
                expected, got
            ),
            ArithmetizerError::MismatchedCircuits => {
                write!(f, "Arithmetizer: output wires belong to different circuits")
            }
            ArithmetizerError::EvaluatorError(e) => write!(f, "Arithmetizer: {}", e),
            ArithmetizerError::CacheError(e) => write!(f, "Arithmetizer: {}", e),
            ArithmetizerError::CommutativeSetTypeConversionError(gate) => {
                write!(
                    f,
                    "Arithmetizer: failed to convert wire `{}` to commutative set type",
                    gate
                )
            }
        }
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Debug for ArithmetizerError<Op, N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArithmetizerError: {}", self)
    }
}
