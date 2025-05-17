use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::{arithmetizer::PlookupOps, utils::Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;

use super::{ast::WireAST, Wire};

pub struct Witness<P: SWCurveConfig, T: Into<Scalar<P>> + Copy> {
    pub val: T,
    pub _marker: std::marker::PhantomData<P>,
}

impl<P: SWCurveConfig, T: Into<Scalar<P>> + Copy> Witness<P, T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            _marker: std::marker::PhantomData,
        }
    }
}

// Add ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T: Into<Scalar<P>> + Copy> Add<Witness<P, T>>
    for Wire<Op, P>
{
    type Output = Self;

    fn add(self, other: Witness<P, T>) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .wire_add_const(self.id, other.val.into(), true),
            arith: Rc::clone(&self.arith),
            ast: self
                .ast
                .map(|ast| WireAST::add_const(ast, other.val.into(), true)),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T: Into<Scalar<P>> + Copy> Sub<Witness<P, T>>
    for Wire<Op, P>
{
    type Output = Self;

    fn sub(self, other: Witness<P, T>) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .wire_sub_const(self.id, other.val.into(), true),
            arith: Rc::clone(&self.arith),
            ast: self
                .ast
                .map(|ast| WireAST::sub_const(ast, other.val.into(), true)),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T: Into<Scalar<P>> + Copy> Mul<Witness<P, T>>
    for Wire<Op, P>
{
    type Output = Self;

    fn mul(self, other: Witness<P, T>) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .wire_mul_const(self.id, other.val.into(), true),
            arith: Rc::clone(&self.arith),
            ast: self
                .ast
                .map(|ast| WireAST::mul_const(ast, other.val.into(), true)),
        }
    }
}

// Div ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T: Into<Scalar<P>> + Copy> Div<Witness<P, T>>
    for Wire<Op, P>
{
    type Output = Self;

    fn div(self, other: Witness<P, T>) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .wire_div_const(self.id, other.val.into(), true),
            arith: Rc::clone(&self.arith),
            ast: self
                .ast
                .map(|ast| WireAST::div_const(ast, other.val.into(), true)),
        }
    }
}
