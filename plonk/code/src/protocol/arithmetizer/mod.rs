mod arith_wire;
mod cache;
mod errors;
mod trace;
mod wire;

use crate::{curve::Scalar, protocol::circuit::Circuit, util::map_to_alphabet};
use arith_wire::ArithWire;
pub use errors::ArithmetizerError;
pub use trace::{Pos, Trace};
pub use wire::Wire;

use rand::{rngs::ThreadRng, Rng};
use std::{cell::RefCell, rc::Rc};

/// A unique identifier for a wire in the circuit.
type WireID = usize;

/// Constructs a circuit and arithmetizes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arithmetizer {
    inputs: usize,
    wires: cache::ArithWireCache,
}
// TODO standard library package
// TODO primitive ops for std::public(x) and std::bit(x)
// TODO primitive ops includes plonkup operations std::xor(x,y)

impl Arithmetizer {
    // constructors -------------------------------------------------------

    fn new(inputs: usize) -> Self {
        Self {
            inputs,
            wires: cache::ArithWireCache::new(),
        }
    }

    /// Returns `N` input wires to build a circuit.
    pub fn build<const N: usize>() -> [Wire; N] {
        let cell = Rc::new(RefCell::new(Self::new(N)));
        let mut circuit = cell.borrow_mut();
        let mut wires = Vec::new();
        for i in 0..N {
            let id = circuit.wires.get_id(ArithWire::Input(i));
            wires.push(Wire::new_input(id, cell.clone()));
        }
        wires.try_into().unwrap()
    }

    /// Compute the circuit R where R(x,w) = ⊤.
    pub fn to_circuit<T, R: Rng>(
        rng: &mut R,
        input_values: Vec<T>,
        output_wires: &[Wire],
    ) -> Result<(Circuit, Trace), ArithmetizerError>
    where
        T: Into<Scalar> + Copy + std::fmt::Display,
    {
        ArithmetizerError::validate(&input_values, output_wires)?;
        let wires = &output_wires[0].arith().borrow().wires;
        let input_scalars = input_values.iter().map(|&v| v.into()).collect();
        let output_ids = output_wires.iter().map(Wire::id).collect();
        Trace::new(rng, wires, input_scalars, output_ids)
            .map_err(ArithmetizerError::EvaluatorError)
            .map(Into::<(Circuit, Trace)>::into)
    }

    // operators ----------------------------------------------------------

    pub fn publicize(&mut self, id: WireID) {
        self.wires.publicize(id);
    }

    /// a + b : 𝔽
    pub fn add(&mut self, a: WireID, b: WireID) -> WireID {
        self.wires.get_id(ArithWire::AddGate(a, b))
    }

    /// a b : 𝔽
    pub fn mul(&mut self, a: WireID, b: WireID) -> WireID {
        self.wires.get_id(ArithWire::MulGate(a, b))
    }

    /// a - b : 𝔽
    pub fn sub(&mut self, a: WireID, b: WireID) -> WireID {
        let neg_one = self.wires.get_const_id(-Scalar::ONE);
        let b_ = self.mul(b, neg_one);
        self.add(a, b_)
    }

    /// a + b : 𝔽
    pub fn add_const(&mut self, a: WireID, b: Scalar) -> WireID {
        let right = self.wires.get_const_id(b);
        let gate = ArithWire::AddGate(a, right);
        self.wires.get_id(gate)
    }

    /// a - b : 𝔽
    pub fn sub_const(&mut self, a: WireID, b: Scalar) -> WireID {
        let right = self.wires.get_const_id(-b);
        let gate = ArithWire::AddGate(a, right);
        self.wires.get_id(gate)
    }

    /// a b : 𝔽
    pub fn mul_const(&mut self, a: WireID, b: Scalar) -> WireID {
        let right = self.wires.get_const_id(b);
        let gate = ArithWire::MulGate(a, right);
        self.wires.get_id(gate)
    }

    // boolean operators --------------------------------------------------

    /// a : 𝔹
    pub fn enforce_bit(&mut self, a: WireID) -> Result<(), ArithmetizerError> {
        self.wires.set_bit(a).map_err(ArithmetizerError::CacheError)
    }

    /// ¬a
    pub fn not(&mut self, a: WireID) -> WireID {
        let one = self.wires.get_const_id(Scalar::ONE);
        self.sub(one, a)
    }

    /// a ∧ b : 𝔹
    pub fn and(&mut self, a: WireID, b: WireID) -> WireID {
        self.mul(a, b)
    }

    /// a ∨ b : 𝔹
    /// ¬(¬a ∧ ¬b)
    /// 1 - ((1 - a) * (1 - b))
    /// 1 - (1 - a - b + a b)
    /// 1 - 1 + a + b - a b
    /// a + b - a b
    pub fn or(&mut self, a: WireID, b: WireID) -> WireID {
        let a_plus_b = self.add(a, b);
        let a_b = self.mul(a, b);
        self.sub(a_plus_b, a_b)
    }

    // utils --------------------------------------------------------------

    pub fn to_string<T: std::fmt::Display>(input_values: &[T], output_wires: &[Wire]) -> String {
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

    #[test]
    fn new() {
        let arith = Arithmetizer::new(2);
        assert_eq!(arith.inputs, 2);
    }

    #[test]
    fn build() {
        let wires = Arithmetizer::build::<2>();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].id(), 0);
        assert_eq!(wires[1].id(), 1);
    }

    #[test]
    fn get_wire_commutative() {
        let [a, b] = &Arithmetizer::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = &(a + b);
        assert_eq!(c.id(), 2);
        let d = &(b + a);
        assert_eq!(d.id(), 2);
        let e = &(a * b);
        assert_eq!(e.id(), 3);
        let f = &(b * a);
        assert_eq!(f.id(), 3);
    }

    #[test]
    fn commutative_2() {
        let [a, b, c] = &Arithmetizer::build::<3>();
        let f = a * b * c;
        let g = c * a * b;
        assert_eq!(f.id(), g.id())
    }

    #[test]
    fn add() {
        let [a, b] = &Arithmetizer::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = a + b;
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(b.id()), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), b.id()));
    }

    #[test]
    fn mul() {
        let [a, b] = &Arithmetizer::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = a * b;
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(0), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::MulGate(a.id(), b.id()));
    }

    #[test]
    fn sub() {
        let [a, b] = &Arithmetizer::build::<2>();
        assert_eq!(a.id(), 0);
        assert_eq!(b.id(), 1);
        let c = &(a - b);
        assert_eq!(c.id(), 4);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(b.id()), ArithWire::Input(1));
        assert_eq!(wires.to_arith_(2), ArithWire::Constant(-Scalar::ONE));
        assert_eq!(wires.to_arith_(3), ArithWire::MulGate(b.id(), 2));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), 3));
    }

    #[test]
    fn add_const() {
        let [a] = &Arithmetizer::build::<1>();
        assert_eq!(a.id(), 0);
        let c: &Wire = &(a + 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(0), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(Scalar::ONE));
    }

    #[test]
    fn sub_const() {
        let [a] = &Arithmetizer::build::<1>();
        assert_eq!(a.id(), 0);
        let c: &Wire = &(a - 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(-Scalar::ONE));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::AddGate(a.id(), 1));
    }

    #[test]
    fn mul_const() {
        let [a] = &Arithmetizer::build::<1>();
        assert_eq!(a.id(), 0);
        let c: &Wire = &(a * 1);
        assert_eq!(c.id(), 2);
        let wires = &c.arith().borrow().wires;
        assert_eq!(wires.to_arith_(a.id()), ArithWire::Input(0));
        assert_eq!(wires.to_arith_(1), ArithWire::Constant(Scalar::ONE));
        assert_eq!(wires.to_arith_(c.id()), ArithWire::MulGate(a.id(), 1));
    }
}
