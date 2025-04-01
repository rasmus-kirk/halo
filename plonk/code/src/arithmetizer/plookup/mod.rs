mod compute;
mod opsets;
mod plookupops;

pub use compute::PlookupEvsThunk;
pub use opsets::*;
pub use plookupops::PlookupOps;

use crate::scheme::eqns::plookup_compress;

use halo_accumulation::group::PallasScalar;

type Scalar = PallasScalar;

/// A lookup table for a given Plookup operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table(Vec<[Scalar; 3]>);

impl Table {
    pub fn new(table: Vec<[Scalar; 3]>) -> Self {
        Self(table)
    }

    /// Compress table to the table vector
    pub fn compress(&self, zeta: Scalar, j: Scalar) -> Vec<Scalar> {
        let mut res = Vec::new();
        for row in self.0.iter().copied() {
            let [a, b, c] = row;
            let t = plookup_compress(zeta, a, b, c, j);
            res.push(t);
        }
        res
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

/// The collection of all lookup tables for the Plookup protocol.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableRegistry {
    tables: Vec<Table>,
}

impl Default for TableRegistry {
    fn default() -> Self {
        Self::new::<EmptyOpSet>()
    }
}

impl TableRegistry {
    pub fn new<Op: PlookupOps>() -> Self {
        Self {
            tables: Op::all_tables(),
        }
    }

    /// Lookup the result of an operation
    pub fn lookup<Op: PlookupOps>(&self, op: Op, a: Scalar, b: Scalar) -> Option<Scalar> {
        self.tables[op.id()]
            .0
            .iter()
            .find(|&&row| row[0] == a && row[1] == b)
            .map(|&row| row[2])
    }

    /// Lookup the result of an operation and use it to compute the compressed vector value
    pub fn query<Op: PlookupOps>(
        &self,
        op: Op,
        zeta: Scalar,
        a: Scalar,
        b: Scalar,
    ) -> Option<Scalar> {
        let c = self.lookup(op, a, b)?;
        let j = op.to_fp();
        Some(plookup_compress(zeta, a, b, c, j))
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
