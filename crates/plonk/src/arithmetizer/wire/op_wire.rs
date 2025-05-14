use super::{ast::WireAST, Wire};
use crate::arithmetizer::plookup::{opsets::BinXorOr, PlookupOps};

use ark_ec::short_weierstrass::SWCurveConfig;

use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Neg, Not, Sub};

// Add ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> Add for Wire<Op, P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_add(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::add(ast, other.ast.unwrap())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> Sub for Wire<Op, P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_sub(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub(ast, other.ast.unwrap())),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> Mul for Wire<Op, P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_mul(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul(ast, other.ast.unwrap())),
        }
    }
}

// Neg ---------------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> Neg for Wire<Op, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_neg(self.id),
            arith: self.arith,
            ast: self.ast.map(WireAST::neg),
        }
    }
}

// Not -----------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> Not for Wire<Op, P> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_not(self.id),
            arith: self.arith,
            ast: self.ast.map(WireAST::not),
        }
    }
}

// BitAnd -----------------------------------------------------

impl<Op: PlookupOps, P: SWCurveConfig> BitAnd for Wire<Op, P> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().wire_and(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::and(ast, other.ast.unwrap())),
        }
    }
}

// Lookup -----------------------------------------------------

impl<P: SWCurveConfig> BitOr for Wire<BinXorOr, P> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Or, other)
    }
}

impl<P: SWCurveConfig> BitXor for Wire<BinXorOr, P> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Xor, other)
    }
}
