use std::fmt::Display;

use crate::{
    curve::{Coset, Poly, Scalar},
    protocol::scheme::{Selectors, Slots, Terms},
};

use super::trace::Constraints;

/// Operations defined for the Plonkup protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum PlonkupOps {
    Xor,
    Or,
}

impl From<PlonkupOps> for Scalar {
    fn from(op: PlonkupOps) -> Self {
        Scalar::from(op as usize)
    }
}

impl Display for PlonkupOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PlonkupOps::Xor => "XOR",
            PlonkupOps::Or => "OR",
        };
        write!(f, "{}", s)
    }
}

impl PlonkupOps {
    pub const COUNT: usize = 2;

    /// Get an iterator over all Plonkup operations
    pub fn iter() -> impl Iterator<Item = PlonkupOps> {
        [PlonkupOps::Xor, PlonkupOps::Or].iter().copied()
    }
}

/// A lookup table for a given plonkup operation.
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

    /// Evaluate a compressed value; Plonkup schema
    pub fn eval_compress(zeta: &Scalar, a: &Scalar, b: &Scalar, c: &Scalar, j: &Scalar) -> Scalar {
        a + (zeta * b) + (zeta.pow(2) * c) + (zeta.pow(3) * j)
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

impl From<PlonkupOps> for Table {
    fn from(op: PlonkupOps) -> Self {
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

/// The collection of all lookup tables for the Plonkup protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRegistry {
    tables: [Table; PlonkupOps::COUNT],
}

impl Default for TableRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRegistry {
    pub fn new() -> Self {
        let mut tables = Vec::new();
        for op in PlonkupOps::iter() {
            let table = Table::from(op);
            tables.push(table);
        }
        Self {
            tables: tables.try_into().unwrap(),
        }
    }

    /// Lookup the result of an operation
    pub fn lookup(&self, op: PlonkupOps, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let table = &self.tables[op as usize];
        for &row in table.0.iter() {
            if row[0] == *a && row[1] == *b {
                return Some(row[2]);
            }
        }
        None
    }

    /// Lookup the result of an operation and use it to compute the compressed vector value
    pub fn query(&self, op: PlonkupOps, zeta: &Scalar, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let c = &self.lookup(op, a, b)?;
        let j = &op.into();
        Some(Table::eval_compress(zeta, a, b, c, j))
    }

    /// Compute the vectors used in the protocol
    pub fn compute_vecs(
        &self,
        zeta: &Scalar,
        coset: &Coset,
        constraints: &[Constraints],
    ) -> [Vec<Scalar>; 4] {
        let mut t = Vec::new();
        for op in PlonkupOps::iter() {
            let j = &op.into();
            t.extend(self.tables[op as usize].compress(zeta, j));
        }
        t.sort();
        let extend = coset.n() as usize - t.len() - 1;
        t.extend(vec![*t.last().unwrap(); extend]);
        // table vector

        let mut f = Vec::new();
        for constraint in constraints.iter() {
            if Into::<Scalar>::into(constraint[Terms::Q(Selectors::Qk)]) == Scalar::ONE {
                let a: Scalar = constraint[Terms::F(Slots::A)].into();
                let b: Scalar = constraint[Terms::F(Slots::B)].into();
                let c: Scalar = constraint[Terms::F(Slots::C)].into();
                let j: Scalar = constraint[Terms::Q(Selectors::J)].into();
                f.push(Table::eval_compress(zeta, &a, &b, &c, &j));
            } else {
                f.push(*t.last().unwrap());
            }
        }
        let extend = coset.n() as usize - f.len() - 1;
        f.extend(vec![*t.last().unwrap(); extend]);
        // query vector

        let mut s: Vec<Scalar> = Vec::new();
        s.extend(t.iter());
        s.extend(f.iter());
        s.sort();
        // sort vector

        let mut h1 = Vec::new();
        let mut h2 = Vec::new();
        for (i, x) in s.into_iter().enumerate() {
            if i % 2 == 0 {
                h1.push(x);
            } else {
                h2.push(x);
            }
        }
        [t, f, h1, h2]
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

/// A struct that acts as a thunk where `compute` takes in zeta
/// from transcript to compute the polynomials for Plonkup protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlonkupVecCompute {
    coset: Coset,
    constraints: Vec<Constraints>,
    table: TableRegistry,
}

impl PlonkupVecCompute {
    pub fn new(coset: Coset, constraints: Vec<Constraints>, table: TableRegistry) -> Self {
        Self {
            coset,
            constraints,
            table,
        }
    }

    pub fn compute(&self, zeta: &Scalar) -> [Poly; 4] {
        self.table
            .compute_vecs(zeta, &self.coset, &self.constraints)
            .into_iter()
            .map(|evals| self.coset.interpolate_zf(evals))
            .collect::<Vec<Poly>>()
            .try_into()
            .unwrap()
    }
}
