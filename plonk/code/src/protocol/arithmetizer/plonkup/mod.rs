use crate::curve::{Coset, Scalar};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum PlonkupOps {
    Xor,
    Or,
}

impl PlonkupOps {
    pub const COUNT: usize = 2;

    pub fn iter() -> impl Iterator<Item = PlonkupOps> {
        [PlonkupOps::Xor, PlonkupOps::Or].iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table(Vec<[Scalar; 3]>);

impl Table {
    pub fn new(table: Vec<[Scalar; 3]>) -> Self {
        Self(table)
    }

    pub fn compress(&self, ch: &Scalar, n: u64) -> Vec<Scalar> {
        let mut res = Vec::new();
        for row in self.0.iter() {
            let [a, b, c] = row;
            let t = a + ch * b + ch * ch * c;
            res.push(t);
        }
        res.extend(vec![res[0]; n as usize - res.len()]);
        res
    }

    pub fn unwrap(&self) -> Vec<[Scalar; 3]> {
        self.0.clone()
    }
}

impl From<PlonkupOps> for Table {
    fn from(op: PlonkupOps) -> Self {
        match op {
            PlonkupOps::Xor => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ZERO],
            ]),
            PlonkupOps::Or => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ONE],
            ]),
        }
    }
}

pub struct TableRegistry {
    tables: [Table; PlonkupOps::COUNT],
}

impl TableRegistry {
    fn new() -> Self {
        let mut tables = Vec::new();
        for op in PlonkupOps::iter() {
            let table = Table::from(op);
            tables.push(table);
        }
        Self {
            tables: tables.try_into().unwrap(),
        }
    }

    pub fn lookup(&self, op: PlonkupOps, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let table = &self.tables[op as usize];
        for row in table.unwrap() {
            if row[0] == *a && row[1] == *b {
                return Some(row[2]);
            }
        }
        None
    }

    pub fn compress(&self, ch: &Scalar, coset: Coset) -> Vec<Scalar> {
        self.tables[PlonkupOps::Xor as usize].compress(ch, coset.n())
        // TODO understand section 4.1 of paper on encoding multiple lookup tables
    }
}

lazy_static! {
    pub static ref TABLE_REGISTRY: TableRegistry = TableRegistry::new();
}

// TODO wire interface for plonkup ops
// TODO arithmetizer interface for plonkup ops
// TODO trace interface for plonkup ops - construct f_i, query wire / column
// TODO trace compute F_PL for plonkup gate constraints
// TODO trace compute F_PCC1 and F_PCC2 for permutation polynomial for plonkup
// TODO prover and verifier calls for plonkup
