use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::FpConfig;

use crate::arithmetizer::plookup::{BinXorOr, PlookupOps};

use super::{ast::WireAST, Wire};

use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Neg, Not, Sub};

// Add ------------------------------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Add for Wire<Op, N, C, P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().add(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::add(ast, other.ast.unwrap())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Sub for Wire<Op, N, C, P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().sub(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub(ast, other.ast.unwrap())),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Mul for Wire<Op, N, C, P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().mul(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul(ast, other.ast.unwrap())),
        }
    }
}

// Neg ---------------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Neg for Wire<Op, N, C, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().neg(self.id),
            arith: self.arith,
            ast: self.ast.map(WireAST::neg),
        }
    }
}

// Not -----------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Not for Wire<Op, N, C, P> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().not(self.id),
            arith: self.arith,
            ast: self.ast.map(WireAST::not),
        }
    }
}

// BitAnd -----------------------------------------------------

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> BitAnd
    for Wire<Op, N, C, P>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().and(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::and(ast, other.ast.unwrap())),
        }
    }
}

// Lookup -----------------------------------------------------

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> Wire<BinXorOr, N, C, P> {
    /// Perform a lookup operation between itself and other
    pub fn lookup(self, op: BinXorOr, other: Self) -> Self {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .lookup(op, self.id, other.id),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::lookup(op, ast, other.ast.unwrap())),
        }
    }
}

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> BitOr for Wire<BinXorOr, N, C, P> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Or, other)
    }
}

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> BitXor for Wire<BinXorOr, N, C, P> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Xor, other)
    }
}
