// TODO: Move this to pp

use ark_ff::BigInt;
use ark_pallas::{Fq, Projective};

macro_rules! mk_proj {
    ($x:tt, $y:tt, $z:tt) => {
        Projective::new_unchecked(
            Fq::new_unchecked(BigInt::new($x)),
            Fq::new_unchecked(BigInt::new($y)),
            Fq::new_unchecked(BigInt::new($z)),
        )
    };
}

// Single Source of Truth for N
macro_rules! get_no_of_blocks {
    ($expand:path) => {
        $expand!(64);
    };
}
macro_rules! define_no_of_blocks {
    ($n:literal) => {
        pub(crate) const G_BLOCKS_NO: usize = $n;
    };
}

get_no_of_blocks!(define_no_of_blocks); // G_BLOCKS_NO

pub(crate) use get_no_of_blocks;

// Make clippy shut up!
#[allow(dead_code)]
pub const G_BLOCKS_SIZE: usize = N / G_BLOCKS_NO;
pub const N: usize = 2usize.pow(20);

#[allow(dead_code)]
pub const S: Projective = mk_proj!(
    [
        10511358259169183486,
        2074067763166240952,
        17611644572363664036,
        341020441001484065
    ],
    [
        12835947837332599666,
        6255076945129827893,
        5160699941501430743,
        674756274627950377
    ],
    [
        3780891978758094845,
        11037255111966004397,
        18446744073709551615,
        4611686018427387903
    ]
);
#[allow(dead_code)]
pub const H: Projective = mk_proj!(
    [
        7341486867992484987,
        4586814896141457814,
        12027446952718021701,
        3769587512575455815
    ],
    [
        17315885811818124458,
        13643165659743018808,
        30407301326549650,
        915560932831355023
    ],
    [
        3780891978758094845,
        11037255111966004397,
        18446744073709551615,
        4611686018427387903
    ]
);
