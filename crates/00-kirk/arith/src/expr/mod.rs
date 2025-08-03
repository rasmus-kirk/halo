use std::marker::PhantomData;
use std::ops::{Add, Mul};

use halo_group::PastaConfig;

use crate::circuit::{CircuitSpec, Wire};

#[derive(Debug, Clone)]
pub enum Expr<P: PastaConfig> {
    Add(Box<Expr<P>>, Box<Expr<P>>), // Addition of two expressions
    Mul(Box<Expr<P>>, Box<Expr<P>>), // Multiplication of two expressions
    Scalar(PhantomData<P>),          // Variable identifier with PhantomData to constrain P
    Point(PhantomData<P>),           // Variable identifier with PhantomData to constrain P
}

// Implement methods for creating expressions and traversal
impl<P: PastaConfig> Expr<P> {
    // Constructor for addition
    pub fn add(left: Expr<P>, right: Expr<P>) -> Expr<P> {
        Self::Add(Box::new(left), Box::new(right))
    }

    // Constructor for multiplication
    pub fn mul(left: Expr<P>, right: Expr<P>) -> Expr<P> {
        Self::Mul(Box::new(left), Box::new(right))
    }

    // Constructor for variables
    pub fn var() -> Expr<P> {
        Expr::Scalar(PhantomData)
    }

    // Recursive traversal method that builds and returns a CircuitSpec
    pub fn traverse(self) -> CircuitSpec<P> {
        let mut circuit = CircuitSpec::new();
        self.traverse_into(&mut circuit);
        circuit
    }

    // Helper method to build the circuit recursively and return the output Wire
    fn traverse_into(self, circuit: &mut CircuitSpec<P>) -> Wire {
        match self {
            Expr::Add(left, right) => {
                let left_wire = left.traverse_into(circuit);
                let right_wire = right.traverse_into(circuit);
                circuit.add_gate(left_wire, right_wire)
            }
            Expr::Mul(left, right) => {
                let left_wire = left.traverse_into(circuit);
                let right_wire = right.traverse_into(circuit);
                circuit.mul_gate(left_wire, right_wire)
            }
            Expr::Scalar(_) => circuit.witness_gate(),
            Expr::Point(_) => circuit.witness_gate(),
        }
    }
}

// Overload the + operator for Expr
impl<P: PastaConfig> Add for Expr<P> {
    type Output = Expr<P>;

    fn add(self, other: Expr<P>) -> Expr<P> {
        Expr::add(self, other)
    }
}

// Overload the * operator for Expr
impl<P: PastaConfig> Mul for Expr<P> {
    type Output = Expr<P>;

    fn mul(self, other: Expr<P>) -> Expr<P> {
        Expr::mul(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::GateType;

    use super::*;
    use halo_group::PallasConfig;

    #[test]
    fn test_expr_traverse() {
        // Create the expression: (x + y) * z
        let a: Expr<PallasConfig> = Expr::var(); // x = Witness
        let b: Expr<PallasConfig> = Expr::var(); // y = Witness
        let c: Expr<PallasConfig> = Expr::var(); // z = Witness
        let expr: Expr<PallasConfig> = (a + b) * c; // (x + y) * z

        // Create a CircuitSpec and traverse the expression
        let _ = expr.traverse();
    }
}
