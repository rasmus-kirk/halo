pub mod inner_sponge;
mod outer_sponge;

pub use inner_sponge::{PERM_ROUNDS_FULL, SPONGE_CAPACITY, SPONGE_RATE, STATE_SIZE};
pub use outer_sponge::{Protocols, Sponge};
