use super::{Wire, WireID};
use crate::{curve::Scalar, protocol::arithmetizer::plonkup::PlonkupOps, util::map_to_alphabet};

use std::fmt;

/// An abstract syntax tree representing a wire.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WireAST {
    Input(WireID),
    Constant(Scalar),
    Add(Box<WireAST>, Box<WireAST>),
    Mul(Box<WireAST>, Box<WireAST>),
    Lookup(PlonkupOps, Box<WireAST>, Box<WireAST>),
}

impl fmt::Display for WireAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WireAST::Input(id) => write!(f, "{}", map_to_alphabet(*id)),
            WireAST::Constant(c) => write!(f, "{}", c),
            WireAST::Add(lhs, rhs) => write!(f, "(+ {} {})", lhs, rhs),
            WireAST::Mul(lhs, rhs) => write!(f, "(* {} {})", lhs, rhs),
            WireAST::Lookup(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

impl Wire {
    pub fn add_ast(&self, other: &Wire) -> WireAST {
        WireAST::Add(Box::new(self.ast()), Box::new(other.ast()))
    }

    pub fn neg_ast(&self) -> WireAST {
        self.mul_ast_const(-Scalar::ONE)
    }

    pub fn sub_ast(&self, other: &Wire) -> WireAST {
        WireAST::Add(Box::new(self.ast()), Box::new(other.neg_ast()))
    }

    pub fn add_ast_const(&self, other: Scalar) -> WireAST {
        WireAST::Add(Box::new(self.ast()), Box::new(WireAST::Constant(other)))
    }

    pub fn sub_ast_const(&self, other: Scalar) -> WireAST {
        WireAST::Add(Box::new(self.ast()), Box::new(WireAST::Constant(-other)))
    }

    pub fn mul_ast(&self, other: &Wire) -> WireAST {
        WireAST::Mul(Box::new(self.ast()), Box::new(other.ast()))
    }

    pub fn mul_ast_const(&self, other: Scalar) -> WireAST {
        WireAST::Mul(Box::new(self.ast()), Box::new(WireAST::Constant(other)))
    }

    pub fn lookup_ast(&self, op: PlonkupOps, other: &Wire) -> WireAST {
        WireAST::Lookup(op, Box::new(self.ast()), Box::new(other.ast()))
    }

    pub fn not_ast_(ast: WireAST) -> WireAST {
        WireAST::Add(
            Box::new(WireAST::Constant(Scalar::ONE)),
            Box::new(WireAST::Mul(
                Box::new(ast),
                Box::new(WireAST::Constant(-Scalar::ONE)),
            )),
        )
    }

    pub fn not_ast(&self) -> WireAST {
        Wire::not_ast_(self.ast())
    }

    pub fn and_ast(&self, other: &Wire) -> WireAST {
        self.mul_ast(other)
    }

    pub fn or_ast(&self, other: &Wire) -> WireAST {
        let a_plus_b = self.add_ast(other);
        let a_b = self.mul_ast(other);
        let neg_a_b = WireAST::Mul(Box::new(a_b), Box::new(WireAST::Constant(-Scalar::ONE)));
        WireAST::Add(Box::new(a_plus_b), Box::new(neg_a_b))
    }
}
