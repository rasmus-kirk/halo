use super::constraints::Constraints;
use crate::{curve::Scalar, protocol::arithmetizer::WireID, util::map_to_alphabet};

#[derive(Debug)]
pub enum TraceError {
    InputNotSet(WireID),
    WireNotInCache(WireID),
    ConstNotInCache(Scalar),
    FailedToMakeCoset(u64),
    ConstraintNotSatisfied(String),
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
        }
    }
}

impl std::error::Error for TraceError {}
