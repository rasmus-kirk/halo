use super::Value;
use crate::{
    arithmetizer::{plookup::PlookupOps, WireID},
    scheme::{Selectors, Slots, Terms},
};

use halo_accumulation::group::PallasScalar;

use ark_ff::AdditiveGroup;
use bimap::BiMap;
use std::{fmt, ops::Index};

type Scalar = PallasScalar;

/// Values for a single equation / constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constraints {
    pub vs: [Value; Terms::COUNT],
}

impl Index<Terms> for Constraints {
    type Output = Value;

    fn index(&self, index: Terms) -> &Self::Output {
        let index_usize: usize = index.into();
        &self.vs[index_usize]
    }
}

impl std::ops::IndexMut<Terms> for Constraints {
    fn index_mut(&mut self, index: Terms) -> &mut Self::Output {
        let index_usize: usize = index.into();
        &mut self.vs[index_usize]
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints::new([Value::ZERO; Terms::COUNT])
    }
}

impl Constraints {
    pub fn new(vs: [Value; Terms::COUNT]) -> Self {
        Constraints { vs }
    }

    /// Create a constraint that enforces a constant value.
    pub fn constant(const_wire: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *const_wire;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::Q(Selectors::Qc)] = Value::AnonWire(-Into::<Scalar>::into(*const_wire));
        vs
    }

    /// Create a constraint that enforces the sum of two values.
    pub fn add(lhs: &Value, rhs: &Value, out: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *lhs;
        vs[Terms::F(Slots::B)] = *rhs;
        vs[Terms::F(Slots::C)] = *out;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::Q(Selectors::Qr)] = Value::ONE;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs
    }

    /// Create a constraint that enforces the product of two values.
    pub fn mul(lhs: &Value, rhs: &Value, out: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *lhs;
        vs[Terms::F(Slots::B)] = *rhs;
        vs[Terms::F(Slots::C)] = *out;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    pub fn boolean(val: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *val;
        vs[Terms::F(Slots::B)] = *val;
        vs[Terms::Q(Selectors::Ql)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    pub fn public_input(val: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *val;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::PublicInputs] = -val;
        vs
    }

    pub fn lookup(op: PlookupOps, lhs: &Value, rhs: &Value, out: &Value) -> Self {
        let mut vs = Constraints::default();
        vs[Terms::F(Slots::A)] = *lhs;
        vs[Terms::F(Slots::B)] = *rhs;
        vs[Terms::F(Slots::C)] = *out;
        vs[Terms::Q(Selectors::Qk)] = Value::ONE;
        vs[Terms::Q(Selectors::J)] = Value::AnonWire(Into::<Scalar>::into(op));
        vs
    }

    pub fn scalars(&self) -> [Scalar; Terms::COUNT] {
        self.vs
            .iter()
            .map(|&v| v.into())
            .collect::<Vec<Scalar>>()
            .try_into()
            .unwrap()
    }

    pub fn is_satisfied(&self) -> bool {
        let scalars = self.scalars();
        Terms::eqn(scalars) == Scalar::ZERO
    }

    pub fn is_plonkup_satisfied(&self, zeta: &Scalar, f: &Scalar) -> bool {
        let scalars = self.scalars();
        Terms::plonkup_eqn(scalars, zeta, f) == Scalar::ZERO
    }

    /// Check if the constraints are structurally equal.
    /// `Scalar` must be equal
    /// `WireID` are modulo renaming
    /// Renames that must be respected are in `enforced_map`
    pub fn structural_eq(&self, other: &Self, enforced_map: &mut BiMap<WireID, WireID>) -> bool {
        for term in Terms::iter() {
            let lhs_scalar: Scalar = self[term].into();
            let rhs_scalar: Scalar = other[term].into();
            if lhs_scalar != rhs_scalar {
                return false;
            }
            if let (Value::Wire(lhs_id, _, _), Value::Wire(rhs_id, _, _)) =
                (self[term], other[term])
            {
                if let Some(rhs_enforced) = enforced_map.get_by_left(&lhs_id) {
                    if *rhs_enforced != rhs_id {
                        return false;
                    }
                }
                if let Some(lhs_enforced) = enforced_map.get_by_right(&rhs_id) {
                    if *lhs_enforced != lhs_id {
                        return false;
                    }
                }
                enforced_map.insert(lhs_id, rhs_id);
            }
        }
        true
    }
}

impl fmt::Display for Constraints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Terms::eqn_str(self.vs.map(|v| v.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{arithmetizer::plookup::TableRegistry, util::scalar::bitxor};

    use ark_ff::Field;
    use rand::Rng;

    const N: usize = 100;

    #[test]
    fn constant() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: Scalar = rng.gen();
            let eqn_values = Constraints::constant(&Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::F(Slots::A)], Value::new_wire(0, scalar));
            assert_eq!(
                eqn_values[Terms::Q(Selectors::Qc)],
                Value::AnonWire(-scalar)
            );
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn add() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let a = &Value::new_wire(0, rng.gen());
            let b = &Value::new_wire(1, rng.gen());
            let c = &Value::new_wire(2, (a + b).into());
            let eqn_values = Constraints::add(a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], *c);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn mul() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let a = &Value::new_wire(0, rng.gen());
            let b = &Value::new_wire(1, rng.gen());
            let c = &Value::new_wire(2, (a * b).into());
            let eqn_values = Constraints::mul(a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], *c);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn boolean() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let bit: bool = rng.gen();
            let a = &Value::new_wire(0, if bit { Scalar::ONE } else { Scalar::ZERO });
            let eqn_values = Constraints::boolean(a);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *a);
            assert!(eqn_values.is_satisfied());
        }
        for _ in 0..N {
            let mut val: Scalar = rng.gen();
            loop {
                if val == Scalar::ZERO || val == Scalar::ONE {
                    val = rng.gen();
                } else {
                    break;
                }
            }
            let a = &Value::new_wire(0, val);
            let eqn_values = Constraints::boolean(a);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *a);
            assert!(!eqn_values.is_satisfied());
        }
    }

    #[test]
    fn public_input() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: Scalar = rng.gen();
            let eqn_values = Constraints::public_input(&Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::F(Slots::A)], Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::PublicInputs], -Value::new_wire(0, scalar));
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn lookup() {
        let table = TableRegistry::new();
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let a_ = &Scalar::from(rng.gen_range(0..2));
            let b_ = &Scalar::from(rng.gen_range(0..2));
            let c_ = bitxor(*a_, *b_);
            let a = &Value::new_wire(0, *a_);
            let b = &Value::new_wire(1, *b_);
            let c = &Value::new_wire(2, c_);
            let op = PlookupOps::Xor;
            let eqn_values = Constraints::lookup(op, a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], *c);
            assert_eq!(eqn_values[Terms::Q(Selectors::Qk)], Value::ONE);
            assert!(eqn_values.is_satisfied());
            let zeta: Scalar = rng.gen();
            let f = table.query(PlookupOps::Xor, &zeta, a_, b_);
            assert!(f.is_some());
            assert!(eqn_values.is_plonkup_satisfied(&zeta, &f.unwrap()))
        }
    }

    #[test]
    fn structural_eq() {
        let c1 = Constraints::constant(&Value::new_wire(0, Scalar::ZERO));
        let c2 = Constraints::constant(&Value::new_wire(1, Scalar::ZERO));
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 1);
        assert_eq!(hmap.get_by_left(&0), Some(&1));

        let c1 = Constraints::add(
            &Value::new_wire(0, Scalar::ONE),
            &Value::new_wire(1, 2.into()),
            &Value::new_wire(2, 3.into()),
        );
        let c2 = Constraints::add(
            &Value::new_wire(1, Scalar::ONE),
            &Value::new_wire(2, 2.into()),
            &Value::new_wire(0, 3.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            &Value::new_wire(0, 2.into()),
            &Value::new_wire(1, 3.into()),
            &Value::new_wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            &Value::new_wire(1, 2.into()),
            &Value::new_wire(2, 3.into()),
            &Value::new_wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            &Value::new_wire(0, 2.into()),
            &Value::new_wire(1, 3.into()),
            &Value::new_wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            &Value::new_wire(1, 2.into()),
            &Value::new_wire(1, 3.into()),
            &Value::new_wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(!c1.structural_eq(&c2, hmap));
    }
}
