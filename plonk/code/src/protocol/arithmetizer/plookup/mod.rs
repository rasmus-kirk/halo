mod compute;

pub use compute::PlookupEvsThunk;

use std::fmt::Display;

use crate::{curve::Scalar, util::poly::plookup_compress};

/// Operations defined for the Plookup protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum PlookupOps {
    Xor,
    Or,
}

impl From<PlookupOps> for Scalar {
    fn from(op: PlookupOps) -> Self {
        Scalar::from(op as usize)
    }
}

impl Display for PlookupOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PlookupOps::Xor => "XOR",
            PlookupOps::Or => "OR",
        };
        write!(f, "{}", s)
    }
}

impl PlookupOps {
    pub const COUNT: usize = 2;

    /// Get an iterator over all Plookup operations
    pub fn iter() -> impl Iterator<Item = PlookupOps> {
        [PlookupOps::Xor, PlookupOps::Or].iter().copied()
    }
}

/// A lookup table for a given Plookup operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table(Vec<[Scalar; 3]>);

impl Table {
    pub fn new(table: Vec<[Scalar; 3]>) -> Self {
        Self(table)
    }

    /// Compress table to the table vector
    pub fn compress(&self, zeta: &Scalar, j: &Scalar) -> Vec<Scalar> {
        let mut res = Vec::new();
        for row in self.0.iter() {
            let [a, b, c] = row;
            let t = Self::eval_compress(zeta, a, b, c, j);
            res.push(t);
        }
        res
    }

    pub fn eval_compress(zeta: &Scalar, a: &Scalar, b: &Scalar, c: &Scalar, j: &Scalar) -> Scalar {
        plookup_compress(&zeta.scalar, &a.scalar, &b.scalar, &c.scalar, &j.scalar).into()
    }

    /// Number of entries in the table
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the table is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<PlookupOps> for Table {
    fn from(op: PlookupOps) -> Self {
        match op {
            // PlonkupOps::Xor => {
            //     let mut rows = Vec::new();
            //     for a in 0..2 ^ 8 {
            //         for b in 0..2 ^ 8 {
            //             let c = a ^ b;
            //             rows.push([Scalar::from(a), Scalar::from(b), Scalar::from(c)]);
            //         }
            //     }
            //     Table::new(rows)
            // }
            PlookupOps::Xor => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ZERO],
            ]),
            PlookupOps::Or => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ONE],
            ]),
        }
    }
}

/// The collection of all lookup tables for the Plookup protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRegistry {
    tables: [Table; PlookupOps::COUNT],
}

impl Default for TableRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRegistry {
    pub fn new() -> Self {
        let mut tables = Vec::new();
        for op in PlookupOps::iter() {
            let table = Table::from(op);
            tables.push(table);
        }
        Self {
            tables: tables.try_into().unwrap(),
        }
    }

    /// Lookup the result of an operation
    pub fn lookup(&self, op: PlookupOps, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let table = &self.tables[op as usize];
        for &row in table.0.iter() {
            if row[0] == *a && row[1] == *b {
                return Some(row[2]);
            }
        }
        None
    }

    /// Lookup the result of an operation and use it to compute the compressed vector value
    pub fn query(&self, op: PlookupOps, zeta: &Scalar, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let c = &self.lookup(op, a, b)?;
        let j = &op.into();
        Some(Table::eval_compress(zeta, a, b, c, j))
    }

    /// Total number of entries in all tables
    pub fn len(&self) -> usize {
        self.tables.iter().map(|table| table.len()).sum()
    }

    /// Check if the table registry is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
