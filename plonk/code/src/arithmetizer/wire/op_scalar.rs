use halo_accumulation::group::PallasScalar;

use super::{ast::WireAST, Wire};

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

type Scalar = PallasScalar;

// Add ------------------------------------------------------------------------

impl<T> Add<T> for Wire
where
    T: Into<Scalar> + Copy,
{
    type Output = Wire;

    fn add(self, other: T) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.ast.map(|ast| WireAST::add_const(ast, other.into())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<T> Sub<T> for Wire
where
    T: Into<Scalar> + Copy,
{
    type Output = Wire;

    fn sub(self, other: T) -> Self::Output {
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

// Mul ------------------------------------------------------------------------

impl<T> Mul<T> for Wire
where
    T: Into<Scalar> + Copy,
{
    type Output = Wire;

    fn mul(self, other: T) -> Self::Output {
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

// Div ------------------------------------------------------------------------

impl<T> Div<T> for Wire
where
    T: Into<Scalar> + Copy,
{
    type Output = Wire;

    fn div(self, other: T) -> Self::Output {
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
