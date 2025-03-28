mod ast;
mod op_i32;
mod op_i64;
mod op_scalar;
mod op_u32;
mod op_u64;
mod op_usize;
mod op_wire;

use crate::util::{if_debug, is_debug};

use super::{Arithmetizer, WireID};
use ast::WireAST;

use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wire {
    id: WireID,
    arith: Rc<RefCell<Arithmetizer>>,
    ast: Option<Rc<WireAST>>,
}

impl Wire {
    // constructors -------------------------------------------------------

    pub fn new(id: WireID, arith: Rc<RefCell<Arithmetizer>>, ast: Option<Rc<WireAST>>) -> Self {
        Self { id, arith, ast }
    }

    /// Create a new input wire.
    pub fn new_input(id: WireID, arith: Rc<RefCell<Arithmetizer>>) -> Self {
        Self::new(id, arith, if_debug(Rc::new(WireAST::Input(id))))
    }

    // getters ------------------------------------------------------------

    /// Returns the unique identifier of the wire.
    pub fn id(&self) -> WireID {
        self.id
    }

    /// Returns the circuit that the wire belongs to.
    pub fn arith(&self) -> &Rc<RefCell<Arithmetizer>> {
        &self.arith
    }

    // operations ----------------------------------------------------------

    /// Requires that the wire is a bit
    pub fn is_bit(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        if let Err(e) = arith.enforce_bit(self.id) {
            panic!("Failed to enforce bit: {}", e);
        }
        self.clone()
    }

    /// Publicize the wire's value to Q꜀.
    pub fn is_public(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        arith.publicize(self.id);
        self.clone()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if is_debug() {
            write!(f, "{}", self.ast.clone().unwrap())
        } else {
            write!(f, "AST only computed in `RUST_LOG=debug`")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::map_to_alphabet;

    use super::Arithmetizer;
    use super::*;

    #[test]
    fn new() {
        std::env::set_var("RUST_LOG", "debug");
        let [wire_, _] = &Arithmetizer::build::<2>();
        let arithmetizer = wire_.arith().clone();
        let wire = Wire::new_input(0, arithmetizer);
        assert_eq!(wire.id, 0);
        assert_eq!(format!("{}", wire), map_to_alphabet(0));
    }

    #[test]
    fn add() {
        std::env::set_var("RUST_LOG", "debug");
        let [a, b] = Arithmetizer::build::<2>();
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
        std::env::set_var("RUST_LOG", "debug");
        let [a, b] = Arithmetizer::build::<2>();
        let c = a.clone() - b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 4);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(+ {} (* {} -1))", map_to_alphabet(0), map_to_alphabet(1))
        );
    }

    #[test]
    fn mul() {
        std::env::set_var("RUST_LOG", "debug");
        let [a, b] = Arithmetizer::build::<2>();
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
        std::env::set_var("RUST_LOG", "debug");
        let [a] = Arithmetizer::build::<1>();
        let b: Wire = a.clone() + 1;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} 1)", map_to_alphabet(0)));
    }

    #[test]
    fn sub_const() {
        std::env::set_var("RUST_LOG", "debug");
        let [a] = Arithmetizer::build::<1>();
        let b: Wire = a.clone() - 1;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} -1)", map_to_alphabet(0)));
    }

    #[test]
    fn mul_const() {
        std::env::set_var("RUST_LOG", "debug");
        let [a] = Arithmetizer::build::<1>();
        let b: Wire = a.clone() * 1;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(* {} 1)", map_to_alphabet(0)));
    }
}
