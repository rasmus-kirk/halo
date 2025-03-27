use crate::protocol::arithmetizer::plonkup::PlonkupOps;

use super::{ast::WireAST, Wire};

use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};

// Add ------------------------------------------------------------------------

impl Add for Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.clone().borrow_mut().add(self.id, other.id),
            arith: self.arith,
            ast: WireAST::add(self.ast, other.ast),
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
            ast: WireAST::sub(self.ast, other.ast),
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
            ast: WireAST::mul(self.ast, other.ast),
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
            ast: WireAST::not(self.ast),
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
            ast: WireAST::and(self.ast, other.ast),
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
                .lookup(PlonkupOps::Or, self.id, other.id),
            arith: self.arith,
            ast: WireAST::lookup(PlonkupOps::Or, self.ast, other.ast),
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
                .lookup(PlonkupOps::Xor, self.id, other.id),
            arith: self.arith,
            ast: WireAST::lookup(PlonkupOps::Xor, self.ast, other.ast),
        }
    }
}
