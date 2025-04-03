use super::Table;
use crate::utils::{misc::EnumIter, Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;

use std::hash::Hash;

pub trait PlookupOps: EnumIter + Hash {
    fn to_table<P: SWCurveConfig>(self) -> Table<P>;
    fn all_tables<P: SWCurveConfig>() -> Vec<Table<P>> {
        Self::iter().map(|op| op.to_table()).collect()
    }
    fn to_fp<P: SWCurveConfig>(self) -> Scalar<P> {
        Scalar::<P>::from(self.id() as u64)
    }
    fn is_commutative(&self) -> bool;
}
