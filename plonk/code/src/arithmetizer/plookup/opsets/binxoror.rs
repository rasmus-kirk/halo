use crate::{
    arithmetizer::{plookup::plookupops::PlookupOps, Table},
    utils::{misc::EnumIter, Scalar},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{AdditiveGroup, Field};
use educe::Educe;
use std::fmt::Display;

#[derive(Educe)]
#[educe(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum BinXorOr {
    #[educe(Default)]
    Xor,
    Or,
}

impl PlookupOps for BinXorOr {
    fn to_table<P: SWCurveConfig>(self) -> Table<P> {
        match self {
            BinXorOr::Xor => Table::new(vec![
                [Scalar::<P>::ZERO, Scalar::<P>::ZERO, Scalar::<P>::ZERO],
                [Scalar::<P>::ZERO, Scalar::<P>::ONE, Scalar::<P>::ONE],
                [Scalar::<P>::ONE, Scalar::<P>::ZERO, Scalar::<P>::ONE],
                [Scalar::<P>::ONE, Scalar::<P>::ONE, Scalar::<P>::ZERO],
            ]),
            BinXorOr::Or => Table::new(vec![
                [Scalar::<P>::ZERO, Scalar::<P>::ZERO, Scalar::<P>::ZERO],
                [Scalar::<P>::ZERO, Scalar::<P>::ONE, Scalar::<P>::ONE],
                [Scalar::<P>::ONE, Scalar::<P>::ZERO, Scalar::<P>::ONE],
                [Scalar::<P>::ONE, Scalar::<P>::ONE, Scalar::<P>::ONE],
            ]),
        }
    }
    fn is_commutative(&self) -> bool {
        true
    }
}

impl EnumIter for BinXorOr {
    const COUNT: usize = 2;

    fn iter() -> impl Iterator<Item = BinXorOr> {
        [BinXorOr::Xor, BinXorOr::Or].into_iter()
    }

    fn id(self) -> usize {
        self as usize
    }
}

impl Display for BinXorOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinXorOr::Xor => "XOR",
            BinXorOr::Or => "OR",
        };
        write!(f, "{}", s)
    }
}
