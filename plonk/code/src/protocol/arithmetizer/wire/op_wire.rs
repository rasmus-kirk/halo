use crate::protocol::arithmetizer::plonkup::PlonkupOps;

use super::Wire;

use std::{
    ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub},
    rc::Rc,
};

// Add -----------------------------------------------------

impl Add for Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().add(self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.ast + other.ast,
        }
    }
}

impl Add<&Wire> for Wire {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        self + other.clone()
    }
}

impl Add<Wire> for &Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        self.clone() + other
    }
}

impl Add for &Wire {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        self.clone() + other.clone()
    }
}

// Sub -----------------------------------------------------

impl Sub for Wire {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().sub(self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.sub_ast(&other),
        }
    }
}

impl Sub<&Wire> for Wire {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        self - other.clone()
    }
}

impl Sub<Wire> for &Wire {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        self.clone() - other
    }
}

impl Sub for &Wire {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        self.clone() - other.clone()
    }
}

// Mul -----------------------------------------------------

impl Mul for Wire {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().mul(self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.ast * other.ast,
        }
    }
}

impl Mul<&Wire> for Wire {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        self * other.clone()
    }
}

impl Mul<Wire> for &Wire {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        self.clone() * other
    }
}

impl Mul for &Wire {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        self.clone() * other.clone()
    }
}

// Not -----------------------------------------------------

impl Not for Wire {
    type Output = Wire;

    fn not(self) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().not(self.id),
            arith: Rc::clone(&self.arith),
            ast: self.not_ast(),
        }
    }
}

impl Not for &Wire {
    type Output = Wire;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

// BitAnd -----------------------------------------------------

impl BitAnd for Wire {
    type Output = Wire;

    fn bitand(self, other: Wire) -> Self::Output {
        Wire {
            id: self.arith.borrow_mut().and(self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.and_ast(&other),
        }
    }
}

impl BitAnd<&Wire> for Wire {
    type Output = Wire;

    fn bitand(self, other: &Wire) -> Self::Output {
        self & other.clone()
    }
}

impl BitAnd<Wire> for &Wire {
    type Output = Wire;

    fn bitand(self, other: Wire) -> Self::Output {
        self.clone() & other
    }
}

impl BitAnd for &Wire {
    type Output = Wire;

    fn bitand(self, other: &Wire) -> Self::Output {
        self.clone() & other.clone()
    }
}

// BitOr -----------------------------------------------------

impl BitOr for Wire {
    type Output = Wire;

    fn bitor(self, other: Wire) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .lookup(PlonkupOps::Or, self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.lookup_ast(PlonkupOps::Or, &other),
        }
    }
}

impl BitOr<&Wire> for Wire {
    type Output = Wire;

    fn bitor(self, other: &Wire) -> Self::Output {
        self | other.clone()
    }
}

impl BitOr<Wire> for &Wire {
    type Output = Wire;

    fn bitor(self, other: Wire) -> Self::Output {
        self.clone() | other
    }
}

impl BitOr for &Wire {
    type Output = Wire;

    fn bitor(self, other: &Wire) -> Self::Output {
        self.clone() | other.clone()
    }
}

// BitXor ----------------------------------------------------

impl BitXor for Wire {
    type Output = Wire;

    fn bitxor(self, other: Self) -> Self::Output {
        Wire {
            id: self
                .arith
                .borrow_mut()
                .lookup(PlonkupOps::Xor, self.id, other.id),
            arith: Rc::clone(&self.arith),
            ast: self.lookup_ast(PlonkupOps::Xor, &other),
        }
    }
}

impl BitXor<&Wire> for Wire {
    type Output = Wire;

    fn bitxor(self, other: &Wire) -> Self::Output {
        self ^ other.clone()
    }
}

impl BitXor<Wire> for &Wire {
    type Output = Wire;

    fn bitxor(self, other: Wire) -> Self::Output {
        self.clone() ^ other
    }
}

impl BitXor for &Wire {
    type Output = Wire;

    fn bitxor(self, other: &Wire) -> Self::Output {
        self.clone() ^ other.clone()
    }
}
