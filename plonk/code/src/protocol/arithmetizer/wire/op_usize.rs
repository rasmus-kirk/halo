use super::{ast::WireAST, Wire};
use crate::curve::Scalar;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add ------------------------------------------------------------------------

impl Add<usize> for Wire {
    type Output = Wire;

    fn add(self, other: usize) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: WireAST::add_const(self.ast, other.into()),
        }
    }
}

impl Add<Wire> for usize {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

// Sub ------------------------------------------------------------------------

impl Sub<usize> for Wire {
    type Output = Wire;

    fn sub(self, other: usize) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .sub_const(self.id, other.into()),
            arith: self.arith,
            ast: WireAST::sub_const(self.ast, other.into()),
        }
    }
}

impl Sub<Wire> for usize {
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
            ast: WireAST::add_const(neg.ast, self.into()),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl Mul<usize> for Wire {
    type Output = Wire;

    fn mul(self, other: usize) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .mul_const(self.id, other.into()),
            arith: self.arith,
            ast: WireAST::mul_const(self.ast, other.into()),
        }
    }
}

impl Mul<Wire> for usize {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

// Div ------------------------------------------------------------------------

impl Div<usize> for Wire {
    type Output = Wire;

    fn div(self, other: usize) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .div_const(self.id, other.into()),
            arith: self.arith,
            ast: WireAST::div_const(self.ast, other.into()),
        }
    }
}
