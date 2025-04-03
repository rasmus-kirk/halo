use super::Value;
use crate::{
    arithmetizer::{plookup::PlookupOps, WireID},
    scheme::{
        eqns::{plonk_eqn, plonk_eqn_str, plonkup_eqn_fp},
        Selectors, Slots, Terms,
    },
    utils::misc::{batch_op, EnumIter},
};

use ark_ff::{AdditiveGroup, Fp, FpConfig};
use bimap::BiMap;
use educe::Educe;
use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
};

/// Values for a single equation / constraint.

#[derive(Educe)]
#[educe(Clone, Copy, PartialEq, Eq)]
pub struct Constraints<const N: usize, C: FpConfig<N>> {
    pub vs: [Value<N, C>; Terms::COUNT],
}

impl<const N: usize, C: FpConfig<N>> Default for Constraints<N, C> {
    fn default() -> Self {
        Self {
            vs: [Value::ZERO; Terms::COUNT],
        }
    }
}

impl<const N: usize, C: FpConfig<N>> Index<Terms> for Constraints<N, C> {
    type Output = Value<N, C>;

    fn index(&self, index: Terms) -> &Self::Output {
        &self.vs[index.id()]
    }
}

impl<const N: usize, C: FpConfig<N>> IndexMut<Terms> for Constraints<N, C> {
    fn index_mut(&mut self, index: Terms) -> &mut Self::Output {
        &mut self.vs[index.id()]
    }
}

impl<const N: usize, C: FpConfig<N>> Display for Constraints<N, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", plonk_eqn_str(self.vs.map(|v| v.to_string())))
    }
}

impl<const N: usize, C: FpConfig<N>> Debug for Constraints<N, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constraint: {}", self)
    }
}

impl<const N: usize, C: FpConfig<N>> Constraints<N, C> {
    pub fn new(vs: [Value<N, C>; Terms::COUNT]) -> Self {
        Self { vs }
    }

    /// Create a constraint that enforces a constant value.
    pub fn constant(const_wire: Value<N, C>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = const_wire;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::Q(Selectors::Qc)] = Value::AnonWire(-Into::<Fp<C, N>>::into(const_wire));
        vs
    }

    /// Create a constraint that enforces the sum of two values.
    pub fn add(lhs: Value<N, C>, rhs: Value<N, C>, out: Value<N, C>) -> Self {
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
    pub fn mul(lhs: Value<N, C>, rhs: Value<N, C>, out: Value<N, C>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = lhs;
        vs[Terms::F(Slots::B)] = rhs;
        vs[Terms::F(Slots::C)] = out;
        vs[Terms::Q(Selectors::Qo)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    /// Create a booleanity constraint.
    pub fn boolean(val: Value<N, C>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = val;
        vs[Terms::F(Slots::B)] = val;
        vs[Terms::Q(Selectors::Ql)] = Value::neg_one();
        vs[Terms::Q(Selectors::Qm)] = Value::ONE;
        vs
    }

    /// Create a constraint that enforces a public input value.
    pub fn public_input(val: Value<N, C>) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = val;
        vs[Terms::Q(Selectors::Ql)] = Value::ONE;
        vs[Terms::PublicInputs] = -val;
        vs
    }

    /// Create a plookup constraint.
    pub fn lookup<Op: PlookupOps>(
        op: Op,
        lhs: Value<N, C>,
        rhs: Value<N, C>,
        out: Value<N, C>,
    ) -> Self {
        let mut vs: Self = Default::default();
        vs[Terms::F(Slots::A)] = lhs;
        vs[Terms::F(Slots::B)] = rhs;
        vs[Terms::F(Slots::C)] = out;
        vs[Terms::Q(Selectors::Qk)] = Value::ONE;
        vs[Terms::Q(Selectors::J)] = Value::AnonWire(op.to_fp());
        vs
    }

    /// Get all scalar values of the constraint.
    pub fn scalars(&self) -> [Fp<C, N>; Terms::COUNT] {
        batch_op(self.vs, Into::<Fp<C, N>>::into)
            .try_into()
            .unwrap()
    }

    /// Get the slot scalar values of the constraint.
    pub fn ws(&self) -> [Fp<C, N>; Slots::COUNT] {
        batch_op(&self.vs[..Slots::COUNT], Into::<Fp<C, N>>::into)
            .try_into()
            .unwrap()
    }

    /// Get the selector scalar values of the constraint.
    pub fn qs(&self) -> [Fp<C, N>; Selectors::COUNT] {
        batch_op(
            &self.vs[Slots::COUNT..Slots::COUNT + Selectors::COUNT],
            Into::<Fp<C, N>>::into,
        )
        .try_into()
        .unwrap()
    }

    /// Get the public input scalar value of the constraint.
    pub fn pip(&self) -> Fp<C, N> {
        self.vs[Terms::PublicInputs.id()].into()
    }

    /// Check if plonk constraints are satisfied.
    pub fn is_satisfied(&self) -> bool {
        plonk_eqn(self.ws(), self.qs(), self.pip()) == Fp::ZERO
    }

    /// Check if plonkup constraints are satisfied.
    pub fn is_plonkup_satisfied(&self, zeta: Fp<C, N>, f: Fp<C, N>) -> bool {
        plonkup_eqn_fp(zeta, self.ws(), self.qs(), self.pip(), f) == Fp::ZERO
    }

    /// Check if the constraints are structurally equal.
    /// `Scalar` must be equal
    /// `WireID` are modulo renaming
    /// Renames that must be respected are in `enforced_map`
    pub fn structural_eq(&self, other: &Self, enforced_map: &mut BiMap<WireID, WireID>) -> bool {
        for term in Terms::iter() {
            let lhs_scalar: Fp<C, N> = self[term].into();
            let rhs_scalar: Fp<C, N> = other[term].into();
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
        arithmetizer::plookup::{BinXorOr, TableRegistry},
        utils::scalar::bitxor,
    };

    use ark_ff::Field;
    use halo_accumulation::group::PallasScalar;
    use rand::Rng;

    const N: usize = 100;

    #[test]
    fn constant() {
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let scalar: PallasScalar = rng.gen();
            let eqn_values = Constraints::constant(Value::new_wire(0, scalar));
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
            let a = Value::new_wire(0, r1);
            let b = Value::new_wire(1, rng.gen());
            let c = Value::new_wire(2, (a + b).into());
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
            let a = Value::new_wire(0, r1);
            let b = Value::new_wire(1, rng.gen());
            let c = Value::new_wire(2, (a * b).into());
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
            let a = Value::new_wire(
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
            let a = Value::new_wire(0, val);
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
            let eqn_values = Constraints::public_input(Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::F(Slots::A)], Value::new_wire(0, scalar));
            assert_eq!(eqn_values[Terms::PublicInputs], -Value::new_wire(0, scalar));
            assert!(eqn_values.is_satisfied());
        }
    }

    #[test]
    fn lookup() {
        let table = TableRegistry::new::<BinXorOr>();
        let rng = &mut rand::thread_rng();
        for _ in 0..N {
            let a_ = PallasScalar::from(rng.gen_range(0..2));
            let b_ = PallasScalar::from(rng.gen_range(0..2));
            let c_ = bitxor(a_, b_);
            let a = Value::new_wire(0, a_);
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
    fn structural_eq() {
        let c1 = Constraints::constant(Value::new_wire(0, PallasScalar::ZERO));
        let c2 = Constraints::constant(Value::new_wire(1, PallasScalar::ZERO));
        let hmap = &mut BiMap::new();
        assert!(c1.structural_eq(&c2, hmap));
        assert_eq!(hmap.len(), 1);
        assert_eq!(hmap.get_by_left(&0), Some(&1));

        let c1 = Constraints::add(
            Value::new_wire(0, PallasScalar::ONE),
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
            Value::new_wire(0, PallasScalar::from(2)),
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
            Value::new_wire(0, PallasScalar::from(2)),
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
