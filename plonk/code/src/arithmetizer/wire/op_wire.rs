use crate::arithmetizer::plookup::{BinXorOr, PlookupOps};

use super::{ast::WireAST, Wire};

use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Neg, Not, Sub};

// Add ------------------------------------------------------------------------

impl<Op: PlookupOps> Add for Wire<Op> {
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

impl<Op: PlookupOps> Sub for Wire<Op> {
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

impl<Op: PlookupOps> Mul for Wire<Op> {
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

impl<Op: PlookupOps> Neg for Wire<Op> {
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

impl<Op: PlookupOps> Not for Wire<Op> {
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

impl<Op: PlookupOps> BitAnd for Wire<Op> {
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

impl Wire<BinXorOr> {
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

impl BitOr for Wire<BinXorOr> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Or, other)
    }
}

impl BitXor for Wire<BinXorOr> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.lookup(BinXorOr::Xor, other)
    }
}
