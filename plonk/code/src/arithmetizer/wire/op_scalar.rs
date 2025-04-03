use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Fp, FpConfig};

use crate::arithmetizer::plookup::PlookupOps;

use super::{ast::WireAST, Wire};

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add ------------------------------------------------------------------------

impl<T, Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Add<T>
    for Wire<Op, N, C, P>
where
    T: Into<Fp<C, N>> + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.ast.map(|ast| WireAST::add_const(ast, other.into())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<T, Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Sub<T>
    for Wire<Op, N, C, P>
where
    T: Into<Fp<C, N>> + Copy,
{
    type Output = Self;

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

impl<T, Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Mul<T>
    for Wire<Op, N, C, P>
where
    T: Into<Fp<C, N>> + Copy,
{
    type Output = Self;

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

impl<T, Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Div<T>
    for Wire<Op, N, C, P>
where
    T: Into<Fp<C, N>> + Copy,
{
    type Output = Self;

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
