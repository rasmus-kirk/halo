mod commutative_set;
mod errors;

pub use commutative_set::CommutativeSet;
pub use errors::{BitError, CacheError};

use super::{
    arith_wire::{ArithWire, CommutativeOps},
    plookup::PlookupOps,
    WireID,
};

use ark_ff::{AdditiveGroup, Field, Fp, FpConfig};
use bimap::BiMap;
use educe::Educe;
use std::collections::{HashMap, HashSet};

/// Cache of arithmetized wires.
/// Wire reuse leads to smaller circuits.
///
#[derive(Educe)]
#[educe(Default, Clone, PartialEq)]
pub struct ArithWireCache<Op: PlookupOps, const N: usize, C: FpConfig<N>> {
    uuid: WireID,
    wires: BiMap<WireID, ArithWire<Op, N, C>>,
    commutative_lookup: HashMap<CommutativeSet<Op>, WireID>,
    bit_wires: HashMap<WireID, bool>,
    public_wires: HashSet<WireID>,
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> ArithWireCache<Op, N, C> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.uuid
    }

    /// Get the next WireID
    fn next_id(&mut self) -> WireID {
        let id = self.uuid;
        self.uuid += 1;
        id
    }

    /// Register a wire in the cache
    fn insert_wire(&mut self, wire: ArithWire<Op, N, C>) -> WireID {
        let id = self.next_id();
        self.wires.insert(id, wire);
        id
    }

    /// Register the wire as a public input
    pub fn publicize(&mut self, id: WireID) {
        self.public_wires.insert(id);
    }

    /// Check if the wire is a public input
    pub fn is_public(&self, id: WireID) -> bool {
        self.public_wires.contains(&id)
    }

    // Get the WireID of a wire
    pub fn get_id(&mut self, wire: ArithWire<Op, N, C>) -> WireID {
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
    pub fn get_const_id(&mut self, val: Fp<C, N>) -> WireID {
        let wire = ArithWire::Constant(val);
        if let Some(&id) = self.wires.get_by_right(&wire) {
            return id;
        }
        self.insert_wire(wire)
    }

    /// Get WireID of a constant value
    pub fn lookup_const_id(&self, val: Fp<C, N>) -> Option<WireID> {
        let wire = ArithWire::Constant(val);
        self.wires.get_by_right(&wire).copied()
    }

    /// Get ArithWire from WireID
    pub fn to_arith(&self, id: WireID) -> Option<ArithWire<Op, N, C>> {
        self.wires.get_by_left(&id).copied()
    }

    #[cfg(test)]
    pub fn to_arith_(&self, id: WireID) -> ArithWire<Op, N, C> {
        self.to_arith(id).unwrap()
    }

    // commutative set lookup ------------------------------------------------

    /// Get the commutative set (leafs of a chain of a commutative operation (add / mul)).
    pub fn get_commutative_set(
        &self,
        wire: ArithWire<Op, N, C>,
    ) -> Result<CommutativeSet<Op>, CacheError<Op, N, C>> {
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
        comm_type: &CommutativeOps<Op>,
        wire: ArithWire<Op, N, C>,
    ) -> Result<Vec<WireID>, CacheError<Op, N, C>> {
        if let Ok(set_type) = <ArithWire<Op, N, C> as TryInto<CommutativeOps<Op>>>::try_into(wire) {
            if &set_type == comm_type {
                return wire.inputs().try_fold(vec![], |mut set, operand| {
                    self.wires
                        .get_by_left(&operand)
                        .ok_or(CacheError::OperandNotInCache)
                        .and_then(|&wire_| self.get_commutative_vec(comm_type, wire_))
                        .map(|xs| {
                            set.extend(if xs.is_empty() { vec![operand] } else { xs });
                            set
                        })
                });
            }
        }
        Ok(vec![])
    }

    // bit typechecking -------------------------------------------------------

    /// Set a wire as a bit, marking it for boolean constraint generation.
    pub fn set_bit(&mut self, id: WireID) -> Result<(), CacheError<Op, N, C>> {
        self.set_bit_(id, true)
    }

    fn set_bit_(&mut self, id: WireID, gen_constraint: bool) -> Result<(), CacheError<Op, N, C>> {
        match self.to_arith(id) {
            Some(w) => match w {
                ArithWire::Input(_) => {
                    self.bit_wires.insert(id, gen_constraint);
                    Ok(())
                }
                ArithWire::Constant(b) => {
                    if b != Fp::ZERO && b != Fp::ONE {
                        return Err(BitError::ScalarIsNotBit(b).into());
                    }
                    Ok(())
                }
                ArithWire::AddGate(_, _)
                | ArithWire::MulGate(_, _)
                | ArithWire::Lookup(_, _, _) => {
                    w.inputs().for_each(|operand| {
                        let _ = self.set_bit_(operand, false);
                    });
                    self.bit_wires.insert(id, gen_constraint);
                    Ok(())
                }
            },
            None => Err(CacheError::WireIDNotInCache),
        }
    }

    pub fn is_bit(&self, id: WireID) -> bool {
        match self.to_arith(id) {
            Some(w) => match w {
                ArithWire::Input(_) => self.bit_wires.contains_key(&id),
                ArithWire::Constant(b) => b == Fp::ZERO || b == Fp::ONE,
                ArithWire::AddGate(_, _)
                | ArithWire::MulGate(_, _)
                | ArithWire::Lookup(_, _, _) => !w
                    .inputs()
                    .any(|operand| !self.bit_wires.contains_key(&operand) && !self.is_bit(operand)),
            },
            None => false,
        }
    }

    pub fn is_bool_constraint(&self, id: WireID) -> bool {
        self.bit_wires.get(&id).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::MontBackend;
    use ark_pallas::FrConfig;
    use halo_accumulation::group::PallasScalar;

    use crate::arithmetizer::plookup::EmptyOpSet;

    use super::*;

    #[test]
    fn insert_wire() {
        let mut cache = ArithWireCache::<EmptyOpSet, 4, MontBackend<FrConfig, 4>>::new();
        let wire = ArithWire::Constant(PallasScalar::ZERO);
        let id = cache.insert_wire(wire);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
        let id = cache.insert_wire(wire);
        assert_eq!(id, 1);
        assert_eq!(
            cache.wires.get_by_left(&1),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
    }

    #[test]
    fn get_const_id() {
        let mut cache = ArithWireCache::<EmptyOpSet, 4, MontBackend<FrConfig, 4>>::new();
        let id = cache.get_const_id(PallasScalar::ZERO);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
        let id = cache.get_const_id(PallasScalar::ZERO);
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
    }

    #[test]
    fn get_id() {
        let mut cache = ArithWireCache::<EmptyOpSet, 4, MontBackend<FrConfig, 4>>::new();
        let id = cache.get_id(ArithWire::Constant(PallasScalar::ZERO));
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
        let id = cache.get_id(ArithWire::Constant(PallasScalar::ZERO));
        assert_eq!(id, 0);
        assert_eq!(
            cache.wires.get_by_left(&0),
            Some(&ArithWire::Constant(PallasScalar::ZERO))
        );
    }
}
