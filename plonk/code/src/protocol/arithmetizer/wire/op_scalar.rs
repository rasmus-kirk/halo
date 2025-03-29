use super::{ast::WireAST, Wire};
use crate::curve::Scalar;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add ------------------------------------------------------------------------

impl Add<Scalar> for Wire {
    type Output = Wire;

    fn add(self, other: Scalar) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add_const(self.id, other),
            arith: Rc::clone(&self.arith),
            ast: self.ast.map(|ast| WireAST::add_const(ast, other)),
        }
    }
}

impl Add<Wire> for Scalar {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

// Sub ------------------------------------------------------------------------

impl Sub<Scalar> for Wire {
    type Output = Wire;

    fn sub(self, other: Scalar) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().sub_const(self.id, other),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub_const(ast, other)),
        }
    }
}

impl Sub<Wire> for Scalar {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = other.clone() * -Scalar::ONE;
        Wire {
            id: other.arith.clone().borrow_mut().add_const(neg.id, self),
            arith: other.arith,
            ast: neg.ast.map(|ast| WireAST::add_const(ast, self)),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl Mul<Scalar> for Wire {
    type Output = Wire;

    fn mul(self, other: Scalar) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().mul_const(self.id, other),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul_const(ast, other)),
        }
    }
}

impl Mul<Wire> for Scalar {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

// Div ------------------------------------------------------------------------

impl Div<Scalar> for Wire {
    type Output = Wire;

    fn div(self, other: Scalar) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().div_const(self.id, other),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::div_const(ast, other)),
        }
    }
}
