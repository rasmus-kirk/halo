use super::Wire;
use crate::curve::Scalar;

use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// Add (Wire, Constant) -------------------------------------------------

impl Add<i32> for Wire {
    type Output = Wire;

    fn add(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(other.into()),
        }
    }
}

impl Add<&i32> for Wire {
    type Output = Wire;

    fn add(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, (*other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const((*other).into()),
        }
    }
}

impl Add<i32> for &Wire {
    type Output = Wire;

    fn add(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(other.into()),
        }
    }
}

impl Add<&i32> for &Wire {
    type Output = Wire;

    fn add(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.add_const(self.id, (*other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const((*other).into()),
        }
    }
}

// Add (Constant, Wire) -------------------------------------------------

impl Add<Wire> for i32 {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + self
    }
}

impl Add<&Wire> for i32 {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        other + self
    }
}

impl Add<Wire> for &i32 {
    type Output = Wire;

    fn add(self, other: Wire) -> Self::Output {
        other + *self
    }
}

impl Add<&Wire> for &i32 {
    type Output = Wire;

    fn add(self, other: &Wire) -> Self::Output {
        other + *self
    }
}

// Sub (Wire, Constant) -------------------------------------------------

impl Sub<i32> for Wire {
    type Output = Wire;

    fn sub(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        let scalar: Scalar = other.into();
        Wire {
            id: circuit.add_const(self.id, -scalar),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(-scalar),
        }
    }
}

impl Sub<&i32> for Wire {
    type Output = Wire;

    fn sub(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        let scalar: Scalar = other.into();
        Wire {
            id: circuit.add_const(self.id, -scalar),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(-scalar),
        }
    }
}

impl Sub<i32> for &Wire {
    type Output = Wire;

    fn sub(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        let scalar: Scalar = other.into();
        Wire {
            id: circuit.add_const(self.id, -scalar),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(-scalar),
        }
    }
}

impl Sub<&i32> for &Wire {
    type Output = Wire;

    fn sub(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        let scalar: Scalar = other.into();
        Wire {
            id: circuit.add_const(self.id, -scalar),
            arith: Rc::clone(&self.arith),
            ast: self.add_ast_const(-scalar),
        }
    }
}

// Sub (Constant, Wire) -------------------------------------------------

impl Sub<Wire> for i32 {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = &other * -Scalar::ONE;
        let mut circuit = other.arith.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, self.into()),
            arith: Rc::clone(&other.arith),
            ast: neg.add_ast_const(self.into()),
        }
    }
}

impl Sub<&Wire> for i32 {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let neg = other * -Scalar::ONE;
        let mut circuit = other.arith.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, self.into()),
            arith: Rc::clone(&other.arith),
            ast: neg.add_ast_const(self.into()),
        }
    }
}

impl Sub<Wire> for &i32 {
    type Output = Wire;

    fn sub(self, other: Wire) -> Self::Output {
        let neg = &other * -Scalar::ONE;
        let mut circuit = other.arith.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, (*self).into()),
            arith: Rc::clone(&other.arith),
            ast: neg.add_ast_const((*self).into()),
        }
    }
}

impl Sub<&Wire> for &i32 {
    type Output = Wire;

    fn sub(self, other: &Wire) -> Self::Output {
        let neg = other * -Scalar::ONE;
        let mut circuit = other.arith.borrow_mut();
        Wire {
            id: circuit.add_const(neg.id, (*self).into()),
            arith: Rc::clone(&other.arith),
            ast: neg.add_ast_const((*self).into()),
        }
    }
}

// Mul (Wire, Constant) -------------------------------------------------

impl Mul<i32> for Wire {
    type Output = Wire;

    fn mul(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const(other.into()),
        }
    }
}

impl Mul<&i32> for Wire {
    type Output = Wire;

    fn mul(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (*other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((*other).into()),
        }
    }
}

impl Mul<i32> for &Wire {
    type Output = Wire;

    fn mul(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, other.into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const(other.into()),
        }
    }
}

impl Mul<&i32> for &Wire {
    type Output = Wire;

    fn mul(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (*other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((*other).into()),
        }
    }
}

// Mul (Constant, Wire) -------------------------------------------------

impl Mul<Wire> for i32 {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * self
    }
}

impl Mul<&Wire> for i32 {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        other * self
    }
}

impl Mul<Wire> for &i32 {
    type Output = Wire;

    fn mul(self, other: Wire) -> Self::Output {
        other * *self
    }
}

impl Mul<&Wire> for &i32 {
    type Output = Wire;

    fn mul(self, other: &Wire) -> Self::Output {
        other * *self
    }
}

// Div (Wire, Constant) -------------------------------------------------

impl Div<i32> for Wire {
    type Output = Wire;

    fn div(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (1 / other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((1 / other).into()),
        }
    }
}

impl Div<&i32> for Wire {
    type Output = Wire;

    fn div(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (1 / *other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((1 / *other).into()),
        }
    }
}

impl Div<i32> for &Wire {
    type Output = Wire;

    fn div(self, other: i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (1 / other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((1 / other).into()),
        }
    }
}

impl Div<&i32> for &Wire {
    type Output = Wire;

    fn div(self, other: &i32) -> Self::Output {
        let mut circuit = self.arith.borrow_mut();
        Wire {
            id: circuit.mul_const(self.id, (1 / *other).into()),
            arith: Rc::clone(&self.arith),
            ast: self.mul_ast_const((1 / *other).into()),
        }
    }
}
