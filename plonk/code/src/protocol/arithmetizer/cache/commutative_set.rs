use crate::protocol::arithmetizer::{arith_wire::CommutativeOps, WireID};

/// A set of `WireID`s that exist as a commutative chain in a circuit.
/// Used as an index to lookup for cached wires.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommutativeSet {
    set: Vec<WireID>,
    set_type: CommutativeOps,
}

impl CommutativeSet {
    pub fn new(set: Vec<WireID>, set_type: CommutativeOps) -> Self {
        let mut set = set;
        set.sort();
        CommutativeSet { set, set_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let set = CommutativeSet::new(vec![1, 0], CommutativeOps::Add);
        assert_eq!(set.set, vec![0, 1]);
        assert_eq!(set.set_type, CommutativeOps::Add);
    }
}
