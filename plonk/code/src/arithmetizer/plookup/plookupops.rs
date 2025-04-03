use super::Table;
use crate::utils::misc::EnumIter;

use ark_ff::{Fp, FpConfig};

use std::hash::Hash;

pub trait PlookupOps: EnumIter + Hash {
    fn to_table<const N: usize, C: FpConfig<N>>(self) -> Table<N, C>;
    fn all_tables<const N: usize, C: FpConfig<N>>() -> Vec<Table<N, C>> {
        Self::iter().map(|op| op.to_table()).collect()
    }
    fn to_fp<const N: usize, C: FpConfig<N>>(self) -> Fp<C, N> {
        Fp::from(self.id() as u64)
    }
    fn is_commutative(&self) -> bool;
}
