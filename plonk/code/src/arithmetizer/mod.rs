mod arith_wire;
mod cache;
mod errors;
mod plookup;
mod synthesize;
mod trace;
mod wire;

use arith_wire::ArithWire;
pub use errors::ArithmetizerError;
use plookup::opsets::{BinXorOr, EmptyOpSet};
pub use plookup::*;
pub use trace::{Pos, Trace};
pub use wire::Wire;

use crate::{
    circuit::Circuit,
    pcs::PCS,
    utils::{misc::map_to_alphabet, Scalar},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use ark_pallas::PallasConfig;

use educe::Educe;
use log::debug;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{cell::RefCell, rc::Rc};

/// A unique identifier for a wire in the circuit.
type WireID = usize;

pub type PallasArithmetizer<Op = EmptyOpSet> = Arithmetizer<Op, PallasConfig>;
pub type PallasEmptyArith = PallasArithmetizer<EmptyOpSet>;
pub type PallasBitArith = PallasArithmetizer<BinXorOr>;

/// Constructs a circuit and arithmetizes it.
#[derive(Educe)]
#[educe(Debug, Clone, PartialEq)]
pub struct Arithmetizer<Op: PlookupOps = EmptyOpSet, P: SWCurveConfig = PallasConfig> {
    inputs: usize,
    pub(crate) wires: cache::ArithWireCache<Op, P>,
}

impl<Op: PlookupOps, P: SWCurveConfig> Arithmetizer<Op, P> {
    // constructors -------------------------------------------------------

    fn new(inputs: usize) -> Self {
        Self {
            inputs,
            wires: cache::ArithWireCache::<Op, P>::new(),
        }
    }

    /// Returns `N` input wires to build a circuit.
    pub fn build<const M: usize>() -> [Wire<Op, P>; M] {
        let cell = Rc::new(RefCell::new(Self::new(M)));
        let mut circuit = cell.borrow_mut();
        let mut wires = Vec::new();
        for i in 0..M {
            let id = circuit.wires.get_id(ArithWire::<Op, P>::Input(i));
            wires.push(Wire::new_input(id, cell.clone()));
        }
        wires.try_into().unwrap()
    }

    /// Compute the circuit R where R(x,w) = ⊤.
    pub fn to_circuit<R: Rng, T, PCST: PCS<P>>(
        rng: &mut R,
        input_values: Vec<T>,
        output_wires: &[Wire<Op, P>],
        d: Option<usize>,
    ) -> Result<Circuit<P>, ArithmetizerError<Op, P>>
    where
        Scalar<P>: From<T>,
        Standard: Distribution<Scalar<P>>,
    {
        ArithmetizerError::validate(&input_values, output_wires)?;
        let wires = &output_wires[0].arith().borrow().wires;
        let input_scalars = input_values.into_iter().map(Scalar::<P>::from).collect();
        let output_ids = output_wires.iter().map(Wire::id).collect();
        Trace::<P>::new(rng, d, wires, input_scalars, output_ids)
            .map_err(ArithmetizerError::EvaluatorError)
            .map(|t| {
                debug!("\n{}", t);
                t.to_circuit::<PCST>()
            })
    }

    // operators ----------------------------------------------------------

    pub fn wire_publicize(&mut self, id: WireID) {
        self.wires.publicize(id);
    }

    /// a + b : 𝔽
    pub fn wire_add(&mut self, a: WireID, b: WireID) -> WireID {
        self.wires.get_id(ArithWire::AddGate(a, b))
    }

    /// a b : 𝔽
    pub fn wire_mul(&mut self, a: WireID, b: WireID) -> WireID {
        self.wires.get_id(ArithWire::MulGate(a, b))
    }

    /// a - b : 𝔽
    pub fn wire_sub(&mut self, a: WireID, b: WireID) -> WireID {
        let neg_one = self.wires.get_const_id(-Scalar::<P>::ONE);
        let b_ = self.wire_mul(b, neg_one);
        self.wire_add(a, b_)
    }

    /// a + b : 𝔽
    pub fn wire_add_const(&mut self, a: WireID, b: Scalar<P>) -> WireID {
        let right = self.wires.get_const_id(b);
        let gate = ArithWire::AddGate(a, right);
        self.wires.get_id(gate)
    }

    /// a - b : 𝔽
    pub fn wire_sub_const(&mut self, a: WireID, b: Scalar<P>) -> WireID {
        let right = self.wires.get_const_id(-b);
        let gate = ArithWire::AddGate(a, right);
        self.wires.get_id(gate)
    }

    /// a b : 𝔽
    pub fn wire_mul_const(&mut self, a: WireID, b: Scalar<P>) -> WireID {
        let right = self.wires.get_const_id(b);
        let gate = ArithWire::MulGate(a, right);
        self.wires.get_id(gate)
    }

    /// -a : 𝔽
    pub fn wire_neg(&mut self, a: WireID) -> WireID {
        self.wire_mul_const(a, -Scalar::<P>::ONE)
    }

    /// a / b : 𝔽
    pub fn wire_div_const(&mut self, a: WireID, b: Scalar<P>) -> WireID {
        let right = self.wires.get_const_id(Scalar::<P>::ONE / b);
        let gate = ArithWire::MulGate(a, right);
        self.wires.get_id(gate)
    }

    /// Plookup operations
    pub fn wire_lookup(&mut self, op: Op, a: WireID, b: WireID) -> WireID {
        self.wires.get_id(ArithWire::Lookup(op, a, b))
    }

    // boolean operators --------------------------------------------------

    /// a : 𝔹
    pub fn wire_bool(&mut self, a: WireID) -> Result<(), ArithmetizerError<Op, P>> {
        self.wires.set_bit(a).map_err(ArithmetizerError::CacheError)
    }

    /// ¬a
    pub fn wire_not(&mut self, a: WireID) -> WireID {
        let one = self.wires.get_const_id(Scalar::<P>::ONE);
        self.wire_sub(one, a)
    }

    /// a ∧ b : 𝔹
    pub fn wire_and(&mut self, a: WireID, b: WireID) -> WireID {
        self.wire_mul(a, b)
    }

    /// a ∨ b : 𝔹
    /// ¬(¬a ∧ ¬b)
    /// 1 - ((1 - a) * (1 - b))
    /// 1 - (1 - a - b + a b)
    /// 1 - 1 + a + b - a b
    /// a + b - a b
    pub fn wire_or(&mut self, a: WireID, b: WireID) -> WireID {
        let a_plus_b = self.wire_add(a, b);
        let a_b = self.wire_mul(a, b);
        self.wire_sub(a_plus_b, a_b)
    }

    // utils --------------------------------------------------------------

    pub fn cache_len(&self) -> usize {
        self.wires.len()
    }

    pub fn to_string<T: std::fmt::Display>(
        input_values: &[T],
        output_wires: &[Wire<Op, P>],
    ) -> String {
        let mut result = String::new();
        result.push_str("Arithmetizer {\n");
        input_values
            .iter()
            .enumerate()
            .for_each(|(i, v)| result.push_str(&format!("    {} = {},\n", map_to_alphabet(i), v)));
        output_wires
            .iter()
            .for_each(|w| result.push_str(&format!("    {}\n", w)));
        result.push('}');
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetizer::plookup::opsets::EmptyOpSet;

    use halo_accumulation::group::PallasScalar;

    #[test]
    fn new() {
        let arith = Arithmetizer::<EmptyOpSet, PallasConfig>::new(2);
        assert_eq!(arith.inputs, 2);
    }

    #[test]
    fn build() {
        let wires = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].id(), 0);
        assert_eq!(wires[1].id(), 1);
    }

    #[test]
    fn get_wire_commutative() {
        let [a, b] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = &(a.clone() + b.clone());
        assert_eq!(c.id(), 2);
        let d = &(b.clone() + a.clone());
        assert_eq!(d.id(), 2);
        let e = &(a.clone() * b.clone());
        assert_eq!(e.id(), 3);
        let f = &(b * a);
        assert_eq!(f.id(), 3);
    }

    #[test]
    fn commutative_2() {
        let [a, b, c] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<3>();
        let f = a.clone() * b.clone() * c.clone();
        let g = c * a * b;
        assert_eq!(f.id(), g.id())
    }

    #[test]
    fn add() {
        let [a, b] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = a.clone() + b.clone();
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(b.id()), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), b.id()));
    }

    #[test]
    fn mul() {
        let [a, b] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = a.clone() * b.clone();
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(0), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::MulGate(a.id(), b.id()));
    }

    #[test]
    fn sub() {
        let [a, b] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = &(a.clone() - b.clone());
        assert_eq!(c.id(), 4);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(b.id()), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(2), ArithWire::Constant(-PallasScalar::ONE));
        assert_eq!(wires.to_arith_(3), ArithWire::MulGate(b.id(), 2));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), 3));
    }

    #[test]
    fn add_const() {
        let [a] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<1>();
        assert_eq!(a.id(), 0);
        let c = &(a + 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(0), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(PallasScalar::ONE));
    }

    #[test]
    fn sub_const() {
        let [a] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<1>();
        assert_eq!(a.id(), 0);
        let c = &(a.clone() - 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(-PallasScalar::ONE));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), 1));
    }

    #[test]
    fn mul_const() {
        let [a] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<1>();
        assert_eq!(a.id(), 0);
        let c = &(a.clone() * 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(PallasScalar::ONE));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::MulGate(a.id(), 1));
    }
}
