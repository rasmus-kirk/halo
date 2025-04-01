use super::WireID;
use crate::{
    arithmetizer::plookup::PlookupOps,
    utils::{misc::map_to_alphabet, print_table::print_scalar},
};

use halo_accumulation::group::PallasScalar;

use ark_ff::Field;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

type Scalar = PallasScalar;

/// An abstract syntax tree representing a wire.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireAST<Op: PlookupOps> {
    Input(WireID),
    Constant(Scalar),
    Add(Rc<WireAST<Op>>, Rc<WireAST<Op>>),
    Mul(Rc<WireAST<Op>>, Rc<WireAST<Op>>),
    Lookup(Op, Rc<WireAST<Op>>, Rc<WireAST<Op>>),
}

impl<Op: PlookupOps> Display for WireAST<Op> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &WireAST::Input(id) => write!(f, "{}", map_to_alphabet(id)),
            &WireAST::Constant(c) => write!(f, "{}", print_scalar(c)),
            WireAST::Add(lhs, rhs) => write!(f, "(+ {} {})", lhs, rhs),
            WireAST::Mul(lhs, rhs) => write!(f, "(* {} {})", lhs, rhs),
            WireAST::Lookup(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

impl<Op: PlookupOps> WireAST<Op> {
    pub fn constant(value: Scalar) -> Rc<Self> {
        Rc::new(Self::Constant(value))
    }

    pub fn add(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, rhs))
    }

    pub fn neg(ast: Rc<Self>) -> Rc<Self> {
        Self::mul_const(ast, -Scalar::ONE)
    }

    pub fn sub(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, Self::neg(rhs)))
    }

    pub fn add_const(ast: Rc<Self>, other: Scalar) -> Rc<Self> {
        Self::add(ast, Self::constant(other))
    }

    pub fn sub_const(ast: Rc<Self>, other: Scalar) -> Rc<Self> {
        Self::add(ast, Self::constant(-other))
    }

    pub fn mul(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Mul(lhs, rhs))
    }

    pub fn mul_const(ast: Rc<Self>, other: Scalar) -> Rc<Self> {
        Rc::new(Self::Mul(ast, Self::constant(other)))
    }

    pub fn div_const(ast: Rc<Self>, other: Scalar) -> Rc<Self> {
        Self::mul(ast, Self::constant(Scalar::ONE / other))
    }

    pub fn lookup(op: Op, lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Lookup(op, lhs, rhs))
    }

    pub fn not(ast: Rc<Self>) -> Rc<Self> {
        Self::add(
            Self::constant(Scalar::ONE),
            Self::mul(ast, Self::constant(-Scalar::ONE)),
        )
    }

    pub fn and(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Self::mul(lhs, rhs)
    }

    pub fn or(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        let a_plus_b = Self::add(lhs.clone(), rhs.clone());
        let a_b = Self::mul(lhs, rhs);
        let neg_a_b = Self::mul(a_b, Self::constant(-Scalar::ONE));
        Self::add(a_plus_b, neg_a_b)
    }
}
