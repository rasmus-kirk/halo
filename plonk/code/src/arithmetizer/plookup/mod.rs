mod compute;
mod opsets;
mod plookupops;

use ark_ff::{Fp, FpConfig};
pub use compute::PlookupEvsThunk;
pub use opsets::*;
pub use plookupops::PlookupOps;

use crate::scheme::eqns::plookup_compress_fp;

/// A lookup table for a given Plookup operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table<const N: usize, C: FpConfig<N>>(Vec<[Fp<C, N>; 3]>);

impl<const N: usize, C: FpConfig<N>> Table<N, C> {
    pub fn new(table: Vec<[Fp<C, N>; 3]>) -> Self {
        Self(table)
    }

    /// Compress table to the table vector
    pub fn compress(&self, zeta: Fp<C, N>, j: Fp<C, N>) -> Vec<Fp<C, N>> {
        let mut res = Vec::new();
        for row in self.0.iter().copied() {
            let [a, b, c] = row;
            let t = plookup_compress_fp(zeta, a, b, c, j);
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
pub struct TableRegistry<const N: usize, C: FpConfig<N>> {
    tables: Vec<Table<N, C>>,
}

impl<const N: usize, C: FpConfig<N>> Default for TableRegistry<N, C> {
    fn default() -> Self {
        Self::new::<EmptyOpSet>()
    }
}

impl<const N: usize, C: FpConfig<N>> TableRegistry<N, C> {
    pub fn new<Op: PlookupOps>() -> Self {
        Self {
            tables: Op::all_tables(),
        }
    }

    /// Lookup the result of an operation
    pub fn lookup<Op: PlookupOps>(&self, op: Op, a: Fp<C, N>, b: Fp<C, N>) -> Option<Fp<C, N>> {
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
        zeta: Fp<C, N>,
        a: Fp<C, N>,
        b: Fp<C, N>,
    ) -> Option<Fp<C, N>> {
        let c = self.lookup(op, a, b)?;
        let j = op.to_fp();
        Some(plookup_compress_fp(zeta, a, b, c, j))
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
