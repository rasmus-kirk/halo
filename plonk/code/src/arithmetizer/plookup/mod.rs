mod compute;
pub mod opsets;
mod plookupops;

pub use compute::PlookupEvsThunk;
pub use plookupops::PlookupOps;

use crate::scheme::eqns::plookup_compress_fp;
use opsets::EmptyOpSet;

use ark_ec::short_weierstrass::SWCurveConfig;
use educe::Educe;

/// A lookup table for a given Plookup operation.
#[derive(Educe)]
#[educe(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table<P: SWCurveConfig>(Vec<[P::ScalarField; 3]>);

impl<P: SWCurveConfig> Table<P> {
    pub fn new(table: Vec<[P::ScalarField; 3]>) -> Self {
        Self(table)
    }

    /// Compress table to the table vector
    pub fn compress(&self, zeta: P::ScalarField, j: P::ScalarField) -> Vec<P::ScalarField> {
        let mut res = Vec::new();
        for row in self.0.iter().copied() {
            let [a, b, c] = row;
            let t = plookup_compress_fp::<_, _, P>(zeta, a, b, c, j);
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
#[derive(Educe)]
#[educe(Debug, PartialEq, Eq, Clone)]
pub struct TableRegistry<P: SWCurveConfig> {
    tables: Vec<Table<P>>,
}

impl<P: SWCurveConfig> Default for TableRegistry<P> {
    fn default() -> Self {
        Self::new::<EmptyOpSet>()
    }
}

impl<P: SWCurveConfig> TableRegistry<P> {
    pub fn new<Op: PlookupOps>() -> Self {
        Self {
            tables: Op::all_tables(),
        }
    }

    /// Lookup the result of an operation
    pub fn lookup<Op: PlookupOps>(
        &self,
        op: Op,
        a: P::ScalarField,
        b: P::ScalarField,
    ) -> Option<P::ScalarField> {
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
        zeta: P::ScalarField,
        a: P::ScalarField,
        b: P::ScalarField,
    ) -> Option<P::ScalarField> {
        let c = self.lookup(op, a, b)?;
        let j = op.to_fp::<P>();
        Some(plookup_compress_fp::<_, _, P>(zeta, a, b, c, j))
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
