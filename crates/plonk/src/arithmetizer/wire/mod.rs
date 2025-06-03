mod ast;
mod op_scalar;
mod op_wire;
mod witness;

use ast::WireAST;
pub use witness::Witness;

use super::{plookup::PlookupOps, Arithmetizer, WireID};
use crate::utils::misc::{if_debug, is_debug};

use ark_ec::short_weierstrass::SWCurveConfig;

use educe::Educe;

use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
};

#[derive(Educe)]
#[educe(Clone, PartialEq)]
pub struct Wire<Op: PlookupOps, P: SWCurveConfig> {
    id: WireID,
    arith: Rc<RefCell<Arithmetizer<Op, P>>>,
    ast: Option<Rc<WireAST<Op, P>>>,
}

impl<Op: PlookupOps, P: SWCurveConfig> Wire<Op, P> {
    // constructors -------------------------------------------------------

    pub fn new(
        id: WireID,
        arith: Rc<RefCell<Arithmetizer<Op, P>>>,
        ast: Option<Rc<WireAST<Op, P>>>,
    ) -> Self {
        Self { id, arith, ast }
    }

    /// Create a new input wire.
    pub fn new_input(id: WireID, arith: Rc<RefCell<Arithmetizer<Op, P>>>) -> Self {
        Self::new(id, arith, if_debug(Rc::new(WireAST::Input(id))))
    }

    // getters ------------------------------------------------------------

    /// Returns the unique identifier of the wire.
    pub fn id(&self) -> WireID {
        self.id
    }

    /// Returns the circuit that the wire belongs to.
    pub fn arith(&self) -> &Rc<RefCell<Arithmetizer<Op, P>>> {
        &self.arith
    }

    // operations ----------------------------------------------------------

    /// Requires that the wire is a bit
    pub fn is_bit(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        if let Err(e) = arith.wire_bool(self.id) {
            panic!("Failed to enforce bit: {}", e);
        }
        self.clone()
    }

    /// Publicize the wire's value to Q꜀.
    pub fn is_public(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        arith.wire_publicize(self.id);
        self.clone()
    }

    /// Multiplicative inverse of the wire
    pub fn inv(self) -> Self {
        let mut arith = self.arith.borrow_mut();
        Wire {
            id: arith.wire_inv(self.id),
            arith: self.arith.clone(),
            ast: self.ast.map(WireAST::inv),
        }
    }

    /// Perform a lookup operation between itself and other
    pub fn lookup(self, op: Op, other: Self) -> Self {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .wire_lookup(op, self.id, other.id),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::lookup(op, ast, other.ast.unwrap())),
        }
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> Display for Wire<Op, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if is_debug() {
            write!(f, "{}", self.ast.clone().unwrap())
        } else {
            write!(f, "AST only computed in `RUST_LOG=debug`")
        }
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> Debug for Wire<Op, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wire: {}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        arithmetizer::PallasEmptyArith,
        utils::misc::{map_to_alphabet, tests::on_debug},
    };

    use ark_ff::Field;
    use halo_group::PallasScalar;

    #[test]
    fn new() {
        on_debug();
        let [wire_, _] = &PallasEmptyArith::build::<2>();
        let arithmetizer = wire_.arith().clone();
        let wire = Wire::new_input(0, arithmetizer);
        assert_eq!(wire.id, 0);
        assert_eq!(format!("{}", wire), map_to_alphabet(0));
    }

    #[test]
    fn add() {
        on_debug();
        let [a, b] = PallasEmptyArith::build::<2>();
        let c = a.clone() + b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(+ {} {})", map_to_alphabet(0), map_to_alphabet(1))
        );
    }

    #[test]
    fn sub() {
        on_debug();
        let [a, b] = PallasEmptyArith::build::<2>();
        let c = a.clone() - b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 4);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(+ {} (* {} -1))", map_to_alphabet(0), map_to_alphabet(1),)
        );
    }

    #[test]
    fn mul() {
        on_debug();
        let [a, b] = PallasEmptyArith::build::<2>();
        let c = a.clone() * b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(* {} {})", map_to_alphabet(0), map_to_alphabet(1))
        );
    }

    #[test]
    fn add_const() {
        on_debug();
        let [a] = PallasEmptyArith::build::<1>();
        let b = a.clone() + PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} 1)", map_to_alphabet(0)));
    }

    #[test]
    fn sub_const() {
        on_debug();
        let [a] = PallasEmptyArith::build::<1>();
        let b = a.clone() - PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} -1)", map_to_alphabet(0)));
    }

    #[test]
    fn mul_const() {
        on_debug();
        let [a] = PallasEmptyArith::build::<1>();
        let b = a.clone() * PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(* {} 1)", map_to_alphabet(0)));
    }
}
