// TODO: Move this to pp

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
