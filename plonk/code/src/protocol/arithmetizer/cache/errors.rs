use crate::protocol::arithmetizer::arith_wire::ArithWire;

#[derive(Debug)]
pub enum CacheError {
    OperandNotInCache,
    InvalidCommutativeOperator(ArithWire),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::OperandNotInCache => write!(f, "Cache: Operand not in cache"),
            CacheError::InvalidCommutativeOperator(wire) => {
                write!(f, "Cache: Invalid commutative operator: {:?}", wire)
            }
        }
    }
}

impl std::error::Error for CacheError {}
