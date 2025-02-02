mod commutative_set;
mod errors;

use super::{
    arith_wire::{ArithWire, CommutativeOps},
    WireID,
};
use crate::{curve::Scalar, util::if_empty};
pub use commutative_set::CommutativeSet;
use errors::CacheError;

use bimap::BiMap;
use std::collections::HashMap;

/// Cache of arithmetized wires.
/// Wire reuse leads to smaller circuits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArithWireCache {
    uuid: WireID,
    wires: BiMap<WireID, ArithWire>,
    commutative_lookup: HashMap<CommutativeSet, WireID>,
}

impl Default for ArithWireCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithWireCache {
    pub fn new() -> Self {
        ArithWireCache {
            uuid: 0,
            wires: BiMap::new(),
            commutative_lookup: HashMap::new(),
        }
    }

    /// Get the next WireID
    fn next_id(&mut self) -> WireID {
        let id = self.uuid;
        self.uuid += 1;
        id
    }

    /// Register a wire in the cache
    fn insert_wire(&mut self, wire: ArithWire) -> WireID {
        let id = self.next_id();
        self.wires.insert(id, wire);
        id
    }

    // Get the WireID of a wire
    pub fn get_id(&mut self, wire: ArithWire) -> WireID {
        if let Some(&id) = self.wires.get_by_right(&wire) {
            return id;
        }
        // exists in cache
        let mut comm_set_ = None;
        if let Ok(comm_set) = self.get_commutative_set(wire) {
            if let Some(&id) = self.commutative_lookup.get(&comm_set) {
                return id;
                // exists in cache modulo commutativity
            } else {
                comm_set_ = Some(comm_set);
                // capture commutative set
            }
        }
        let id = self.insert_wire(wire);
        // register wire
        if let Some(comm_set) = comm_set_ {
            self.commutative_lookup.insert(comm_set, id);
        }
        // register commutative cache
        id
    }

    /// Get WireID of a constant value
    pub fn get_const_id(&mut self, val: Scalar) -> WireID {
        let wire = ArithWire::Constant(val);
        if let Some(&id) = self.wires.get_by_right(&wire) {
            return id;
        }
        self.insert_wire(wire)
    }

    pub fn lookup_const_id(&self, val: Scalar) -> Option<WireID> {
        let wire = ArithWire::Constant(val);
        self.wires.get_by_right(&wire).copied()
    }

    /// Get ArithWire from WireID
    pub fn to_arith(&self, id: WireID) -> Option<ArithWire> {
        self.wires.get_by_left(&id).copied()
    }

    #[cfg(test)]
    pub fn to_arith_(&self, id: WireID) -> ArithWire {
        self.to_arith(id).unwrap()
    }

    // commutative set lookup ------------------------------------------------

    /// Get the commutative set (leafs of a chain of a commutative operation (add / mul)).
    pub fn get_commutative_set(&self, wire: ArithWire) -> Result<CommutativeSet, CacheError> {
        if let Ok(set_type) = wire.try_into() {
            let set = self.get_commutative_vec(&set_type, wire)?;
            Ok(CommutativeSet::new(set, set_type))
        } else {
            Err(CacheError::InvalidCommutativeOperator(wire))
        }
    }

    /// Recursive helper for `get_commutative_set`.
    fn get_commutative_vec(
        &self,
        comm_type: &CommutativeOps,
        wire: ArithWire,
    ) -> Result<Vec<WireID>, CacheError> {
        let mut set = vec![];
        if let Ok(set_type) = <ArithWire as TryInto<CommutativeOps>>::try_into(wire) {
            if &set_type == comm_type {
                for operand in wire.inputs() {
                    let wire_ = self
                        .wires
                        .get_by_left(&operand)
                        .ok_or(CacheError::OperandNotInCache)?;
                    let xs = self.get_commutative_vec(comm_type, *wire_)?;
                    set.push(if_empty(xs, operand));
                }
            }
        }
        Ok(set.iter().flatten().copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_wire() {
        let mut cache = ArithWireCache::new();
        let wire = ArithWire::Constant(Scalar::ZERO);
        let id = cache.insert_wire(wire);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
        let id = cache.insert_wire(wire);
        assert_eq!(id, 1);
        assert_eq!(
            cache.wires.get_by_left(&1),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
    }

    #[test]
    fn get_const_id() {
        let mut cache = ArithWireCache::new();
        let id = cache.get_const_id(Scalar::ZERO);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
        let id = cache.get_const_id(Scalar::ZERO);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
    }

    #[test]
    fn get_id() {
        let mut cache = ArithWireCache::new();
        let id = cache.get_id(ArithWire::Constant(Scalar::ZERO));
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
        let id = cache.get_id(ArithWire::Constant(Scalar::ZERO));
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(Scalar::ZERO))
        );
    }
}
