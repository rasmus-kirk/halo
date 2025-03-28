use super::{ast::WireAST, Wire};
use crate::curve::Scalar;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add ------------------------------------------------------------------------

impl Add<i32> for Wire {
    type Output = Wire;

    fn add(self, other: i32) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.ast.map(|ast| WireAST::add_const(ast, other.into())),
        }
    }
}

impl Add<Wire> for i32 {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

// Sub ------------------------------------------------------------------------

impl Sub<i32> for Wire {
    type Output = Wire;

    fn sub(self, other: i32) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .sub_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub_const(ast, other.into())),
        }
    }
}

impl Sub<Wire> for i32 {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = other.clone() * -Scalar::ONE;
        Wire {
            id: other
                .arith
                .clone()
                .borrow_mut()
                .add_const(neg.id, self.into()),
            arith: other.arith,
            ast: neg.ast.map(|ast| WireAST::add_const(ast, self.into())),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl Mul<i32> for Wire {
    type Output = Wire;

    fn mul(self, other: i32) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .mul_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul_const(ast, other.into())),
        }
    }
}

impl Mul<Wire> for i32 {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

// Div ------------------------------------------------------------------------

impl Div<i32> for Wire {
    type Output = Wire;

    fn div(self, other: i32) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .div_const(self.id, other.into()),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::div_const(ast, other.into())),
        }
    }
}
