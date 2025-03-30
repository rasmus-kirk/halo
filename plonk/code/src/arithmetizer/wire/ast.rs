use super::WireID;
use crate::{arithmetizer::plookup::PlookupOps, util::misc::map_to_alphabet};

use halo_accumulation::group::PallasScalar;

use ark_ff::Field;
use std::{fmt, rc::Rc};

type Scalar = PallasScalar;

/// An abstract syntax tree representing a wire.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireAST {
    Input(WireID),
    Constant(Scalar),
    Add(Rc<WireAST>, Rc<WireAST>),
    Mul(Rc<WireAST>, Rc<WireAST>),
    Lookup(PlookupOps, Rc<WireAST>, Rc<WireAST>),
}

impl fmt::Display for WireAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WireAST::Input(id) => write!(f, "{}", map_to_alphabet(*id)),
            WireAST::Constant(c) => write!(f, "{}", c),
            WireAST::Add(lhs, rhs) => write!(f, "(+ {} {})", lhs, rhs),
            WireAST::Mul(lhs, rhs) => write!(f, "(* {} {})", lhs, rhs),
            WireAST::Lookup(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

impl WireAST {
    pub fn constant(value: Scalar) -> Rc<WireAST> {
        Rc::new(WireAST::Constant(value))
    }

    pub fn add(lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        Rc::new(WireAST::Add(lhs, rhs))
    }

    pub fn neg(ast: Rc<WireAST>) -> Rc<WireAST> {
        WireAST::mul_const(ast, -Scalar::ONE)
    }

    pub fn sub(lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        Rc::new(WireAST::Add(lhs, WireAST::neg(rhs)))
    }

    pub fn add_const(ast: Rc<WireAST>, other: Scalar) -> Rc<WireAST> {
        WireAST::add(ast, WireAST::constant(other))
    }

    pub fn sub_const(ast: Rc<WireAST>, other: Scalar) -> Rc<WireAST> {
        WireAST::add(ast, WireAST::constant(-other))
    }

    pub fn mul(lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        Rc::new(WireAST::Mul(lhs, rhs))
    }

    pub fn mul_const(ast: Rc<WireAST>, other: Scalar) -> Rc<WireAST> {
        Rc::new(WireAST::Mul(ast, WireAST::constant(other)))
    }

    pub fn div_const(ast: Rc<WireAST>, other: Scalar) -> Rc<WireAST> {
        WireAST::mul(ast, WireAST::constant(Scalar::ONE / other))
    }

    pub fn lookup(op: PlookupOps, lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        Rc::new(WireAST::Lookup(op, lhs, rhs))
    }

    pub fn not(ast: Rc<WireAST>) -> Rc<WireAST> {
        WireAST::add(
            WireAST::constant(Scalar::ONE),
            WireAST::mul(ast, WireAST::constant(-Scalar::ONE)),
        )
    }

    pub fn and(lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        WireAST::mul(lhs, rhs)
    }

    pub fn or(lhs: Rc<WireAST>, rhs: Rc<WireAST>) -> Rc<WireAST> {
        let a_plus_b = WireAST::add(lhs.clone(), rhs.clone());
        let a_b = WireAST::mul(lhs, rhs);
        let neg_a_b = WireAST::mul(a_b, WireAST::constant(-Scalar::ONE));
        WireAST::add(a_plus_b, neg_a_b)
    }
}
