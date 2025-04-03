use std::{error::Error, fmt::Display};

use super::constraints::Constraints;
use crate::{
    arithmetizer::{plookup::PlookupOps, WireID},
    utils::misc::map_to_alphabet,
};

use ark_ff::{Fp, FpConfig};
use educe::Educe;

#[derive(Educe)]
#[educe(Debug)]
pub enum TraceError<Op: PlookupOps, const N: usize, C: FpConfig<N>> {
    InputNotSet(WireID),
    WireNotInCache(WireID),
    ConstNotInCache(Fp<C, N>),
    FailedToEval(WireID),
    FailedToMakeCoset(u64),
    ConstraintNotSatisfied(String),
    LookupFailed(Op, Fp<C, N>, Fp<C, N>),
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Error for TraceError<Op, N, C> {}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> TraceError<Op, N, C> {
    pub fn constraint_not_satisfied(constraint: &Constraints<N, C>) -> Self {
        TraceError::ConstraintNotSatisfied(constraint.to_string())
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Display for TraceError<Op, N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TraceError::InputNotSet(id) => {
                write!(f, "Evaluator: Input `{}` not set", map_to_alphabet(*id))
            }
            TraceError::WireNotInCache(id) => {
                write!(f, "Evaluator: Wire `{}` not in cache", map_to_alphabet(*id))
            }
            TraceError::ConstNotInCache(c) => {
                write!(f, "Evaluator: Constant `{}` not in cache", c)
            }
            TraceError::FailedToEval(id) => {
                write!(
                    f,
                    "Evaluator: Failed to evaluate wire `{}`",
                    map_to_alphabet(*id)
                )
            }
            TraceError::FailedToMakeCoset(m) => {
                write!(f, "Evaluator: Failed to make coset for `m={}`", m)
            }
            TraceError::ConstraintNotSatisfied(constraint_str) => {
                write!(
                    f,
                    "Evaluator: Constraints not satisfied: {}",
                    constraint_str
                )
            }
            TraceError::LookupFailed(op, a, b) => {
                write!(
                    f,
                    "Evaluator: Failed to lookup for op {:?} with a={} and b={}",
                    op, a, b
                )
            }
        }
    }
}
