use super::{ast::WireAST, Wire};

use halo_accumulation::group::PallasScalar;

use ark_ff::Field;
use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

type Scalar = PallasScalar;

// Add ------------------------------------------------------------------------

impl Add<usize> for Wire {
    type Output = Wire;

    fn add(self, other: usize) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .add_const(self.id, Scalar::from(other as u64)),
            arith: Rc::clone(&self.arith),
            ast: self
                .ast
                .map(|ast| WireAST::add_const(ast, Scalar::from(other as u64))),
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
                .sub_const(self.id, Scalar::from(other as u64)),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::sub_const(ast, Scalar::from(other as u64))),
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
                .add_const(neg.id, Scalar::from(self as u64)),
            arith: other.arith,
            ast: neg
                .ast
                .map(|ast| WireAST::add_const(ast, Scalar::from(self as u64))),
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
                .mul_const(self.id, Scalar::from(other as u64)),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::mul_const(ast, Scalar::from(other as u64))),
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
                .div_const(self.id, Scalar::from(other as u64)),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::div_const(ast, Scalar::from(other as u64))),
        }
    }
}
