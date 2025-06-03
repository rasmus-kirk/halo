use super::Value;
use crate::{
    arithmetizer::{plookup::PlookupOps, WireID},
    scheme::{
        eqns::{plonk_eqn, plonk_eqn_str, EqnsF},
        Selectors, Slots, Terms,
    },
    utils::{
        misc::{batch_op, EnumIter},
        Scalar,
    },
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::AdditiveGroup;

use bimap::BiMap;
use educe::Educe;
use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
};

/// Values for a single equation / constraint.

#[derive(Educe)]
#[educe(Clone, Copy, PartialEq, Eq)]
pub struct Constraints<P: SWCurveConfig> {
    pub vs: [Value<P>; Terms::COUNT],
}

impl<P: SWCurveConfig> Default for Constraints<P> {
    fn default() -> Self {
        Self {
            vs: [Value::ZERO; Terms::COUNT],
        }
    }
}

impl<P: SWCurveConfig> Index<Terms> for Constraints<P> {
    type Output = Value<P>;

    fn index(&self, index: Terms) -> &Self::Output {
        &self.vs[index.id()]
    }
}

impl<P: SWCurveConfig> IndexMut<Terms> for Constraints<P> {
    fn index_mut(&mut self, index: Terms) -> &mut Self::Output {
        &mut self.vs[index.id()]
    }
}

impl<P: SWCurveConfig> Display for Constraints<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", plonk_eqn_str(self.vs.map(|v| v.to_string())))
    }
}

impl<P: SWCurveConfig> Debug for Constraints<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constraint: {}", self)
    }
}

impl<P: SWCurveConfig> Constraints<P> {
    pub fn new(vs: [Value<P>; Terms::COUNT]) -> Self {
        Self { vs }
    }

    /// Create a constraint that enforces a constant value.
    pub fn constant(const_wire: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = const_wire;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::Q(Selectors::Qc)] = Value::AnonWire(-const_wire.to_fp());
        vs
    }

    /// Create a constraint that enforces the sum of two values.
    pub fn add(lhs: Value<P>, rhs: Value<P>, out: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = lhs;
        vs[Terms::F(Slots::B)] = rhs;
        vs[Terms::F(Slots::C)] = out;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::Q(Selectors::Qr)] = Value::ONE;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs
    }

    /// Create a constraint that enforces the product of two values.
    pub fn mul(lhs: Value<P>, rhs: Value<P>, out: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = lhs;
        vs[Terms::F(Slots::B)] = rhs;
        vs[Terms::F(Slots::C)] = out;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    /// Create a constraint that enforces the product of two values is 1.
    pub fn mul_inv(v: Value<P>, v_inv: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = v;
        vs[Terms::F(Slots::B)] = v_inv;
        vs[Terms::F(Slots::C)] = Value::ONE;
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs
    }

    /// Create a booleanity constraint.
    pub fn boolean(val: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = val;
        vs[Terms::F(Slots::B)] = val;
        vs[Terms::Q(Selectors::Ql)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    /// Create a constraint that enforces a public input value.
    pub fn public_input(val: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = val;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::PublicInputs] = -val;
        vs
    }

    /// Create a plookup constraint.
    pub fn lookup<Op: PlookupOps>(op: Op, lhs: Value<P>, rhs: Value<P>, out: Value<P>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = lhs;
        vs[Terms::F(Slots::B)] = rhs;
        vs[Terms::F(Slots::C)] = out;
        vs[Terms::Q(Selectors::Qk)] = Value::ONE;
        vs[Terms::Q(Selectors::J)] = Value::AnonWire(op.to_fp::<P>());
        vs
    }

    /// Get all scalar values of the constraint.
    pub fn scalars(&self) -> [Scalar<P>; Terms::COUNT] {
        batch_op(self.vs, |x| x.to_fp()).try_into().unwrap()
    }

    /// Get the slot scalar values of the constraint.
    pub fn ws(&self) -> [Scalar<P>; Slots::COUNT] {
        batch_op(&self.vs[..Slots::COUNT], |x| x.to_fp())
            .try_into()
            .unwrap()
    }

    /// Get the selector scalar values of the constraint.
    pub fn qs(&self) -> [Scalar<P>; Selectors::COUNT] {
        batch_op(
            &self.vs[Slots::COUNT..Slots::COUNT + Selectors::COUNT],
            |x| x.to_fp(),
        )
        .try_into()
        .unwrap()
    }

    /// Get the public input scalar value of the constraint.
    pub fn pip(&self) -> Scalar<P> {
        self.vs[Terms::PublicInputs.id()].to_fp()
    }

    /// Check if plonk constraints are satisfied.
    pub fn is_satisfied(&self) -> bool {
        plonk_eqn(self.ws(), self.qs(), self.pip()) == Scalar::<P>::ZERO
    }

    /// Check if plonkup constraints are satisfied.
    pub fn is_plonkup_satisfied(&self, zeta: Scalar<P>, f: Scalar<P>) -> bool {
        EqnsF::<P>::plonkup_eqn(zeta, self.ws(), self.qs(), self.pip(), f) == Scalar::<P>::ZERO
    }

    /// Check if the constraints are structurally equal.
    /// `Scalar` must be equal
    /// `WireID` are modulo renaming
    /// Renames that must be respected are in `enforced_map`
    pub fn structural_eq(&self, other: &Self, enforced_map: &mut BiMap<WireID, WireID>) -> bool {
        for term in Terms::iter() {
            let lhs_scalar: Scalar<P> = self[term].to_fp();
            let rhs_scalar: Scalar<P> = other[term].to_fp();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        arithmetizer::plookup::{opsets::BinXorOr, TableRegistry},
        utils::scalar::tests::bitxor,
    };

    use ark_ff::Field;
    use ark_pallas::PallasConfig;
    use halo_group::PallasScalar;

    use rand::Rng;

    const N: usize = 100;

    #[test]
    fn constant() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: PallasScalar = rng.gen();
            let eqn_values = Constraints::<PallasConfig>::constant(Value::new_wire(0, scalar));
            assert!(eqn_values[Terms::F(Slots::A)] == Value::new_wire(0, scalar));
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
            let r1: PallasScalar = rng.gen();
            let a: Value<PallasConfig> = Value::new_wire(0, r1);
            let b = Value::new_wire(1, rng.gen());
            let c = Value::new_wire(2, (a + b).to_fp());
            let eqn_values = Constraints::add(a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], c);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn mul() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let r1: PallasScalar = rng.gen();
            let a: Value<PallasConfig> = Value::new_wire(0, r1);
            let b = Value::new_wire(1, rng.gen());
            let c = Value::new_wire(2, (a * b).to_fp());
            let eqn_values = Constraints::mul(a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], c);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn boolean() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let bit: bool = rng.gen();
            let a: Value<PallasConfig> = Value::new_wire(
                0,
                if bit {
                    PallasScalar::ONE
                } else {
                    PallasScalar::ZERO
                },
            );
            let eqn_values = Constraints::boolean(a);
            assert_eq!(eqn_values[Terms::F(Slots::A)], a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], a);
            assert!(eqn_values.is_satisfied());
        }
        for _ in 0..N {
            let mut val: PallasScalar = rng.gen();
            loop {
                if val == PallasScalar::ZERO || val == PallasScalar::ONE {
                    val = rng.gen();
                } else {
                    break;
                }
            }
            let a: Value<PallasConfig> = Value::new_wire(0, val);
            let eqn_values = Constraints::boolean(a);
            assert_eq!(eqn_values[Terms::F(Slots::A)], a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], a);
            assert!(!eqn_values.is_satisfied());
        }
    }

    #[test]
    fn public_input() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: PallasScalar = rng.gen();
            let eqn_values = Constraints::<PallasConfig>::public_input(Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::F(Slots::A)], Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::PublicInputs], -Value::new_wire(0, scalar));
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn lookup() {
        let table = TableRegistry::<PallasConfig>::new::<BinXorOr>();
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let a_ = PallasScalar::from(rng.gen_range(0..2));
            let b_ = PallasScalar::from(rng.gen_range(0..2));
            let c_ = bitxor::<PallasConfig>(a_, b_);
            let a = Value::<PallasConfig>::new_wire(0, a_);
            let b = Value::new_wire(1, b_);
            let c = Value::new_wire(2, c_);
            let op = BinXorOr::Xor;
            let eqn_values = Constraints::lookup(op, a, b, c);
            assert_eq!(eqn_values[Terms::F(Slots::A)], a);
            assert_eq!(eqn_values[Terms::F(Slots::B)], b);
            assert_eq!(eqn_values[Terms::F(Slots::C)], c);
            assert_eq!(eqn_values[Terms::Q(Selectors::Qk)], Value::ONE);
            assert!(eqn_values.is_satisfied());
            let zeta: PallasScalar = rng.gen();
            let f = table.query(BinXorOr::Xor, zeta, a_, b_);
            assert!(f.is_some());
            assert!(eqn_values.is_plonkup_satisfied(zeta, f.unwrap()))
        }
    }

    #[test]
    fn mul_inv() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let mut a_: PallasScalar = rng.gen();
            while a_ == PallasScalar::ZERO {
                a_ = rng.gen();
            }
            let v = Value::<PallasConfig>::new_wire(0, a_);
            let v_inv = v.inv();
            assert!(v_inv.is_some());
            let v_inv = v_inv.unwrap();
            let eqn_values = Constraints::mul_inv(v, v_inv);
            assert_eq!(eqn_values[Terms::F(Slots::A)], v);
            assert_eq!(eqn_values[Terms::F(Slots::B)], v_inv);
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn structural_eq() {
        let c1 = Constraints::<PallasConfig>::constant(Value::new_wire(0, PallasScalar::ZERO));
        let c2 = Constraints::constant(Value::new_wire(1, PallasScalar::ZERO));
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 1);
        assert_eq!(hmap.get_by_left(&0), Some(&1));

        let c1 = Constraints::add(
            Value::<PallasConfig>::new_wire(0, PallasScalar::ONE),
            Value::new_wire(1, 2.into()),
            Value::new_wire(2, 3.into()),
        );
        let c2 = Constraints::add(
            Value::new_wire(1, PallasScalar::ONE),
            Value::new_wire(2, 2.into()),
            Value::new_wire(0, 3.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            Value::<PallasConfig>::new_wire(0, PallasScalar::from(2)),
            Value::new_wire(1, 3.into()),
            Value::new_wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            Value::new_wire(1, PallasScalar::from(2)),
            Value::new_wire(2, 3.into()),
            Value::new_wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 3);
        assert_eq!(hmap.get_by_left(&0), Some(&1));
        assert_eq!(hmap.get_by_left(&1), Some(&2));
        assert_eq!(hmap.get_by_left(&2), Some(&0));

        let c1 = Constraints::mul(
            Value::<PallasConfig>::new_wire(0, PallasScalar::from(2)),
            Value::new_wire(1, 3.into()),
            Value::new_wire(2, 6.into()),
        );
        let c2 = Constraints::mul(
            Value::new_wire(1, PallasScalar::from(2)),
            Value::new_wire(1, 3.into()),
            Value::new_wire(0, 6.into()),
        );
        let hmap = &mut BiMap::new();
        assert!(!c1.structural_eq(&c2, hmap));
    }
}
