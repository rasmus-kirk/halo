use super::constraints::Constraints;
use crate::{
    arithmetizer::{plookup::PlookupOps, WireID},
    utils::misc::map_to_alphabet,
};

use halo_accumulation::group::PallasScalar;

type Scalar = PallasScalar;

#[derive(Debug)]
pub enum TraceError {
    InputNotSet(WireID),
    WireNotInCache(WireID),
    ConstNotInCache(Scalar),
    FailedToEval(WireID),
    FailedToMakeCoset(u64),
    ConstraintNotSatisfied(String),
    LookupFailed(PlookupOps, Scalar, Scalar),
}

impl TraceError {
    pub fn constraint_not_satisfied(constraint: &Constraints) -> Self {
        TraceError::ConstraintNotSatisfied(constraint.to_string())
    }
}

impl std::fmt::Display for TraceError {
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

impl std::error::Error for TraceError {}
