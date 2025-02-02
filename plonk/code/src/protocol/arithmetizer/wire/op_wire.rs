use super::Wire;

use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};

// Add -----------------------------------------------------

impl Add for Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast(&other),
        }
    }
}

impl Add<&Wire> for Wire {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast(other),
        }
    }
}

impl Add<Wire> for &Wire {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast(&other),
        }
    }
}

impl Add for &Wire {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.add(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.add_ast(other),
        }
    }
}

// Sub -----------------------------------------------------

impl Sub for Wire {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.sub(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.sub_ast(&other),
        }
    }
}

impl Sub<&Wire> for Wire {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.sub(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.sub_ast(other),
        }
    }
}

impl Sub<Wire> for &Wire {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.sub(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.sub_ast(&other),
        }
    }
}

impl Sub for &Wire {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.sub(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.sub_ast(other),
        }
    }
}

// Mul -----------------------------------------------------

impl Mul for Wire {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast(&other),
        }
    }
}

impl Mul<&Wire> for Wire {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast(other),
        }
    }
}

impl Mul<Wire> for &Wire {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast(&other),
        }
    }
}

impl Mul for &Wire {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        let mut circuit = self.circuit.borrow_mut();
        Wire {
            id: circuit.mul(self.id, other.id),
            circuit: Rc::clone(&self.circuit),
            ast: self.mul_ast(other),
        }
    }
}
