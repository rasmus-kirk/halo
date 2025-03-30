use crate::protocol::arithmetizer::plookup::PlookupOps;

use super::{ast::WireAST, Wire};

use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};

// Add ------------------------------------------------------------------------

impl Add for Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().add(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::add(ast, other.ast.unwrap())),
        }
    }
}

// Sub ------------------------------------------------------------------------

impl Sub for Wire {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().sub(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::sub(ast, other.ast.unwrap())),
        }
    }
}

// Mul ------------------------------------------------------------------------

impl Mul for Wire {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().mul(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::mul(ast, other.ast.unwrap())),
        }
    }
}

// Not -----------------------------------------------------

impl Not for Wire {
    type Output = Wire;

    fn not(self) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().not(self.id),
            arith: self.arith,
            ast: self.ast.map(WireAST::not),
        }
    }
}

// BitAnd -----------------------------------------------------

impl BitAnd for Wire {
    type Output = Wire;

    fn bitand(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().and(self.id, other.id),
            arith: self.arith,
            ast: self.ast.map(|ast| WireAST::and(ast, other.ast.unwrap())),
        }
    }
}

// BitOr -----------------------------------------------------

impl BitOr for Wire {
    type Output = Wire;

    fn bitor(self, other: Wire) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .lookup(PlookupOps::Or, self.id, other.id),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::lookup(PlookupOps::Or, ast, other.ast.unwrap())),
        }
    }
}

// BitXor ----------------------------------------------------

impl BitXor for Wire {
    type Output = Wire;

    fn bitxor(self, other: Self) -> Self::Output {
        Wire {
            id: self
                .arith
                .clone()
                .borrow_mut()
                .lookup(PlookupOps::Xor, self.id, other.id),
            arith: self.arith,
            ast: self
                .ast
                .map(|ast| WireAST::lookup(PlookupOps::Xor, ast, other.ast.unwrap())),
        }
    }
}
