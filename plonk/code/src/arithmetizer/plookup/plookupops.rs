use super::Table;
use crate::utils::misc::EnumIter;

use halo_accumulation::group::PallasScalar;

use std::hash::Hash;

type Scalar = PallasScalar;

pub trait PlookupOps: EnumIter + Hash {
    fn to_table(self) -> Table;
    fn all_tables() -> Vec<Table> {
        Self::iter().map(|op| op.to_table()).collect()
    }
    fn to_fp(self) -> Scalar {
        Scalar::from(self.id() as u64)
    }
    fn is_commutative(&self) -> bool;
}
