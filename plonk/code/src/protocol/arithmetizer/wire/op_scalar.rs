use super::Wire;
use crate::curve::Scalar;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add (Wire, Constant) -------------------------------------------------

impl Add<Scalar> for Wire {
    type Output = Wire;

    fn add(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(other),
        }
    }
}

impl Add<&Scalar> for Wire {
    type Output = Wire;

    fn add(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(*other),
        }
    }
}

impl Add<Scalar> for &Wire {
    type Output = Wire;

    fn add(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(other),
        }
    }
}

impl Add<&Scalar> for &Wire {
    type Output = Wire;

    fn add(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(*other),
        }
    }
}

// Add (Constant, Wire) -------------------------------------------------

impl Add<Wire> for Scalar {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

impl Add<&Wire> for Scalar {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        other + self
    }
}

impl Add<Wire> for &Scalar {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

impl Add<&Wire> for &Scalar {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        other + self
    }
}

// Sub (Wire, Constant) -------------------------------------------------

impl Sub<Scalar> for Wire {
    type Output = Wire;

    fn sub(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, -other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(-other),
        }
    }
}

impl Sub<&Scalar> for Wire {
    type Output = Wire;

    fn sub(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, -*other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(-*other),
        }
    }
}

impl Sub<Scalar> for &Wire {
    type Output = Wire;

    fn sub(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, -other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(-other),
        }
    }
}

impl Sub<&Scalar> for &Wire {
    type Output = Wire;

    fn sub(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, -*other),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast_const(-*other),
        }
    }
}

// Sub (Constant, Wire) -------------------------------------------------

impl Sub<Wire> for Scalar {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = &other * -Scalar::ONE;
        let mut circuit = other.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, self),
            circuit: Rc::clone(&other.circuit),
            ast: neg.add_ast_const(self),
        }
    }
}

impl Sub<&Wire> for Scalar {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let neg = other * -Scalar::ONE;
        let mut circuit = other.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, self),
            circuit: Rc::clone(&other.circuit),
            ast: neg.add_ast_const(self),
        }
    }
}

impl Sub<Wire> for &Scalar {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = &other * -Scalar::ONE;
        let mut circuit = other.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, *self),
            circuit: Rc::clone(&other.circuit),
            ast: neg.add_ast_const(*self),
        }
    }
}

impl Sub<&Wire> for &Scalar {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let neg = other * -Scalar::ONE;
        let mut circuit = other.circuit.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, *self),
            circuit: Rc::clone(&other.circuit),
            ast: neg.add_ast_const(*self),
        }
    }
}

// Mul (Wire, Constant) -------------------------------------------------

impl Mul<Scalar> for Wire {
    type Output = Wire;

    fn mul(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(other),
        }
    }
}

impl Mul<&Scalar> for Wire {
    type Output = Wire;

    fn mul(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(*other),
        }
    }
}

impl Mul<Scalar> for &Wire {
    type Output = Wire;

    fn mul(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(other),
        }
    }
}

impl Mul<&Scalar> for &Wire {
    type Output = Wire;

    fn mul(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(*other),
        }
    }
}

// Mul (Constant, Wire) -------------------------------------------------

impl Mul<Wire> for Scalar {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

impl Mul<&Wire> for Scalar {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        other * self
    }
}

impl Mul<Wire> for &Scalar {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

impl Mul<&Wire> for &Scalar {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        other * self
    }
}

// Div (Wire, Constant) -------------------------------------------------

impl Div<Scalar> for Wire {
    type Output = Wire;

    fn div(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, 1 / other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(1 / other),
        }
    }
}

impl Div<&Scalar> for Wire {
    type Output = Wire;

    fn div(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, 1 / *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(1 / *other),
        }
    }
}

impl Div<Scalar> for &Wire {
    type Output = Wire;

    fn div(self, other: Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, 1 / other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(1 / other),
        }
    }
}

impl Div<&Scalar> for &Wire {
    type Output = Wire;

    fn div(self, other: &Scalar) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, 1 / *other),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast_const(1 / *other),
        }
    }
}
