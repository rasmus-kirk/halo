use crate::{arithmetizer::plookup::PlookupOps, utils::Scalar};

use super::{ast::WireAST, Wire};

use ark_ec::short_weierstrass::SWCurveConfig;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T> Add<T> for Wire<Op, P>
where
    T: Into<Scalar<P>> + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .wire_add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.ast.map(|ast| WireAST::add_const(ast, other.into())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T> Sub<T> for Wire<Op, P>
where
    T: Into<Scalar<P>> + Copy,
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .wire_sub_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub_const(ast, other.into())),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T> Mul<T> for Wire<Op, P>
where
    T: Into<Scalar<P>> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .wire_mul_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul_const(ast, other.into())),
        }
    }
}

// Div ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig, T> Div<T> for Wire<Op, P>
where
    T: Into<Scalar<P>> + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .wire_div_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::div_const(ast, other.into())),
        }
    }
}
