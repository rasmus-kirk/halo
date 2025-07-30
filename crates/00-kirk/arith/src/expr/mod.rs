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
        let circuit = expr.traverse();

        // Verify the circuit structure
        assert_eq!(circuit.witness_wire_count, 3, "Should have 3 witness gates");
        assert_eq!(
            circuit.row_count, 2,
            "Should have 2 rows (1 for Add, 1 for Mul)"
        );
        assert_eq!(
            circuit.wire_count, 5,
            "Should have 5 wires (3 witness, 1 add, 1 mul)"
        );

        // Verify gate types
        let mut witness_count = 0;
        let mut add_count = 0;
        let mut mul_count = 0;
        for node_idx in circuit.graph.node_indices() {
            match circuit.graph[node_idx] {
                GateType::Witness => witness_count += 1,
                GateType::Add(_) => add_count += 1,
                GateType::Multiply(_) => mul_count += 1,
                _ => panic!("Unexpected gate type"),
            }
        }
        assert_eq!(witness_count, 3, "Should have 3 Witness gates");
        assert_eq!(add_count, 1, "Should have 1 Add gate");
        assert_eq!(mul_count, 1, "Should have 1 Multiply gate");
    }
}
