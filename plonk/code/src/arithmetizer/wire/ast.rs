use super::WireID;
use crate::{
    arithmetizer::plookup::PlookupOps,
    utils::{misc::map_to_alphabet, print_table::print_scalar},
};

use ark_ff::{Field, Fp, FpConfig};
use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

/// An abstract syntax tree representing a wire.
#[derive(Clone, PartialEq)]
pub enum WireAST<Op: PlookupOps, const N: usize, C: FpConfig<N>> {
    Input(WireID),
    Constant(Fp<C, N>),
    Add(Rc<WireAST<Op, N, C>>, Rc<WireAST<Op, N, C>>),
    Mul(Rc<WireAST<Op, N, C>>, Rc<WireAST<Op, N, C>>),
    Lookup(Op, Rc<WireAST<Op, N, C>>, Rc<WireAST<Op, N, C>>),
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Display for WireAST<Op, N, C> {
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

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> Debug for WireAST<Op, N, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WireAST: {}", self)
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>> WireAST<Op, N, C> {
    pub fn constant(value: Fp<C, N>) -> Rc<Self> {
        Rc::new(Self::Constant(value))
    }

    pub fn add(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, rhs))
    }

    pub fn neg(ast: Rc<Self>) -> Rc<Self> {
        Self::mul_const(ast, -Fp::ONE)
    }

    pub fn sub(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Add(lhs, Self::neg(rhs)))
    }

    pub fn add_const(ast: Rc<Self>, other: Fp<C, N>) -> Rc<Self> {
        Self::add(ast, Self::constant(other))
    }

    pub fn sub_const(ast: Rc<Self>, other: Fp<C, N>) -> Rc<Self> {
        Self::add(ast, Self::constant(-other))
    }

    pub fn mul(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Mul(lhs, rhs))
    }

    pub fn mul_const(ast: Rc<Self>, other: Fp<C, N>) -> Rc<Self> {
        Rc::new(Self::Mul(ast, Self::constant(other)))
    }

    pub fn div_const(ast: Rc<Self>, other: Fp<C, N>) -> Rc<Self> {
        Self::mul(ast, Self::constant(Fp::ONE / other))
    }

    pub fn lookup(op: Op, lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Lookup(op, lhs, rhs))
    }

    pub fn not(ast: Rc<Self>) -> Rc<Self> {
        Self::add(
            Self::constant(Fp::ONE),
            Self::mul(ast, Self::constant(-Fp::ONE)),
        )
    }

    pub fn and(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Self::mul(lhs, rhs)
    }

    pub fn or(lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        let a_plus_b = Self::add(lhs.clone(), rhs.clone());
        let a_b = Self::mul(lhs, rhs);
        let neg_a_b = Self::mul(a_b, Self::constant(-Fp::ONE));
        Self::add(a_plus_b, neg_a_b)
    }
}
