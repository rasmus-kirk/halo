use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::AdditiveGroup;
use std::fmt::Display;

use crate::{
    arithmetizer::{plookup::PlookupOps, Table},
    utils::{misc::EnumIter, Scalar},
};
use educe::Educe;

#[derive(Educe)]
#[educe(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum EmptyOpSet {
    #[educe(Default)]
    UnitTable,
}

impl PlookupOps for EmptyOpSet {
    fn to_table<P: SWCurveConfig>(self) -> Table<P> {
        Table::new(vec![[
            Scalar::<P>::ZERO,
            Scalar::<P>::ZERO,
            Scalar::<P>::ZERO,
        ]])
    }
    fn is_commutative(&self) -> bool {
        true
    }
}

impl EnumIter for EmptyOpSet {
    const COUNT: usize = 1;

    fn iter() -> impl Iterator<Item = EmptyOpSet> {
        [EmptyOpSet::UnitTable].into_iter()
    }

    fn id(self) -> usize {
        self as usize
    }
}

impl Display for EmptyOpSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EmptyOpSet::UnitTable => "UnitTable",
        };
        write!(f, "{}", s)
    }
}
