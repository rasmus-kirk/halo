use crate::{
    arithmetizer::{plookup::plookupops::PlookupOps, Table},
    utils::misc::EnumIter,
};

use halo_accumulation::group::PallasScalar;

use ark_ff::{AdditiveGroup, Field};
use std::fmt::Display;

type Scalar = PallasScalar;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum BinXorOr {
    #[default]
    Xor,
    Or,
}

impl PlookupOps for BinXorOr {
    fn to_table(self) -> Table {
        match self {
            BinXorOr::Xor => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ZERO],
            ]),
            BinXorOr::Or => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ONE],
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
