use std::collections::HashMap;

use crate::curve::{Coset, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum PlonkupOps {
    Xor,
    Or,
}

impl PlonkupOps {
    pub fn iter() -> impl Iterator<Item = PlonkupOps> {
        [PlonkupOps::Xor, PlonkupOps::Or].iter().copied()
    }
}

pub type Table = Vec<[Scalar; 3]>;

impl From<PlonkupOps> for Table {
    fn from(op: PlonkupOps) -> Self {
        match op {
            PlonkupOps::Xor => vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ZERO],
            ],
            PlonkupOps::Or => vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ONE],
            ],
        }
    }
}

pub struct TableRegistry {
    tables: HashMap<PlonkupOps, Table>,
}

impl TableRegistry {
    pub fn new() -> Self {
        let mut tables = HashMap::new();
        for op in PlonkupOps::iter() {
            let table = Table::from(op);
            tables.insert(op, table);
        }
        Self { tables }
    }

    pub fn lookup(&self, op: PlonkupOps, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let table = self.tables.get(&op)?;
        for row in table {
            if row[0] == *a && row[1] == *b {
                return Some(row[2]);
            }
        }
        None
    }

    pub fn compress(&self, ch: &Scalar, coset: Coset) -> Vec<Scalar> {
        todo!()
    }
}

// TODO wire interface for plonkup ops
// TODO arithmetizer interface for plonkup ops
// TODO trace interface for plonkup ops
// TODO trace compute F_PL for plonkup gate constraints
// TODO trace compute F_PCC1 and F_PCC2 for permutation polynomial for plonkup
// TODO prover and verifier calls for plonkup
