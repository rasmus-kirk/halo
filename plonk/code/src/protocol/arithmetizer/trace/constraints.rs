use bimap::BiMap;

use super::Value;
use crate::{
    curve::Scalar,
    protocol::{
        arithmetizer::WireID,
        scheme::{Selectors, Slots, Terms},
    },
};

use std::{fmt, ops::Index};

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
            if let (Value::Wire(lhs_id, _), Value::Wire(rhs_id, _)) = (self[term], other[term]) {
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
    use rand::Rng;

    const N: usize = 100;

    #[test]
    fn constant() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: Scalar = rng.gen();
            let eqn_values = Constraints::constant(&Value::Wire(0, scalar));
            assert_eq!(eqn_values[Terms::F(Slots::A)], Value::Wire(0, scalar));
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
            let a = &Value::Wire(0, rng.gen());
            let b = &Value::Wire(1, rng.gen());
            let c = &Value::Wire(2, (a + b).into());
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
            let a = &Value::Wire(0, rng.gen());
            let b = &Value::Wire(1, rng.gen());
            let c = &Value::Wire(2, (a * b).into());
            let eqn_values = Constraints::mul(a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], *a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], *b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], *c);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn structural_eq() {
        let c1 = Constraints::constant(&Value::Wire(0, Scalar::ZERO));
        let c2 = Constraints::constant(&Value::Wire(1, Scalar::ZERO));
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 1);
        assert_eq!(hmap.get_by_left(&0), Some(&1));

        let c1 = Constraints::add(
            &Value::Wire(0, Scalar::ONE),
            &Value::Wire(1, 2.into()),
            &Value::Wire(2, 3.into()),
        );
        let c2 = Constraints::add(
            &Value::Wire(1, Scalar::ONE),
            &Value::Wire(2, 2.into()),
            &Value::Wire(0, 3.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            &Value::Wire(0, 2.into()),
            &Value::Wire(1, 3.into()),
            &Value::Wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            &Value::Wire(1, 2.into()),
            &Value::Wire(2, 3.into()),
            &Value::Wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            &Value::Wire(0, 2.into()),
            &Value::Wire(1, 3.into()),
            &Value::Wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            &Value::Wire(1, 2.into()),
            &Value::Wire(1, 3.into()),
            &Value::Wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(!c1.structural_eq(&c2, hmap));
    }
}
