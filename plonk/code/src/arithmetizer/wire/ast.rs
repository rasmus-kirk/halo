use super::WireID;
use crate::{
    arithmetizer::plookup::PlookupOps,
    utils::{misc::map_to_alphabet, print_table::print_scalar, Scalar},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

/// An abstract syntax tree representing a wire.
#[derive(Clone, PartialEq)]
pub enum WireAST<Op: PlookupOps, P: SWCurveConfig> {
    Input(WireID),
    Constant(Scalar<P>),
    Add(Rc<WireAST<Op, P>>, Rc<WireAST<Op, P>>),
    Mul(Rc<WireAST<Op, P>>, Rc<WireAST<Op, P>>),
    Lookup(Op, Rc<WireAST<Op, P>>, Rc<WireAST<Op, P>>),
}

impl<Op: PlookupOps, P: SWCurveConfig> Display for WireAST<Op, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &WireAST::Input(id) => write!(f, "{}", map_to_alphabet(id)),
            &WireAST::Constant(c) => write!(f, "{}", print_scalar::<P>(c)),
            WireAST::Add(lhs, rhs) => write!(f, "(+ {} {})", lhs, rhs),
            WireAST::Mul(lhs, rhs) => write!(f, "(* {} {})", lhs, rhs),
            WireAST::Lookup(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> Debug for WireAST<Op, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WireAST: {}", self)
    }
}

impl<Op: PlookupOps, P: SWCurveConfig> WireAST<Op, P> {
    pub fn constant(value: Scalar<P>) -> Rc<Self> {
        Rc::new(Self::Constant(value))
    }

    pub fn add(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, rhs))
    }

    pub fn neg(ast: Rc<Self>) -> Rc<Self> {
        Self::mul_const(ast, -Scalar::<P>::ONE)
    }

    pub fn sub(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, Self::neg(rhs)))
    }

    pub fn add_const(ast: Rc<Self>, other: Scalar<P>) -> Rc<Self> {
        Self::add(ast, Self::constant(other))
    }

    pub fn sub_const(ast: Rc<Self>, other: Scalar<P>) -> Rc<Self> {
        Self::add(ast, Self::constant(-other))
    }

    pub fn mul(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Mul(lhs, rhs))
    }

    pub fn mul_const(ast: Rc<Self>, other: Scalar<P>) -> Rc<Self> {
        Rc::new(Self::Mul(ast, Self::constant(other)))
    }

    pub fn div_const(ast: Rc<Self>, other: Scalar<P>) -> Rc<Self> {
        Self::mul(ast, Self::constant(Scalar::<P>::ONE / other))
    }

    pub fn lookup(op: Op, lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Lookup(op, lhs, rhs))
    }

    pub fn not(ast: Rc<Self>) -> Rc<Self> {
        Self::add(
            Self::constant(Scalar::<P>::ONE),
            Self::mul(ast, Self::constant(-Scalar::<P>::ONE)),
        )
    }

    pub fn and(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Self::mul(lhs, rhs)
    }

    pub fn or(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        let a_plus_b = Self::add(lhs.clone(), rhs.clone());
        let a_b = Self::mul(lhs, rhs);
        let neg_a_b = Self::mul(a_b, Self::constant(-Scalar::<P>::ONE));
        Self::add(a_plus_b, neg_a_b)
    }
}
