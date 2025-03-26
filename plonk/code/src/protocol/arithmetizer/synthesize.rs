use log::trace;
use rand::Rng;

use crate::curve::Scalar;

use super::{Arithmetizer, Wire};

impl Arithmetizer {
    pub fn synthesize<R: Rng, const N: usize>(rng: &mut R, degree: usize) -> Vec<Wire> {
        trace!("[A]: Remaining stack - {:?}", stacker::remaining_stack());
        let wires: Vec<Wire> = Arithmetizer::build::<N>().into();

        let mut cur = vec![wires[rng.gen_range(0..N)].clone()];
        let mut cur0 = cur[0].clone();

        while cur0.arith().borrow().cache_len() < degree + N {
            let branch = rng.gen_range(0..8);
            cur0 = if branch < 4 {
                let rng_input = wires[rng.gen_range(0..N)].clone();
                match branch {
                    0 => cur0 * rng_input,
                    1 => rng_input * cur0,
                    2 => cur0 + rng_input,
                    3 => rng_input + cur0,
                    _ => unreachable!(),
                }
            } else {
                let constant: Scalar = rng.gen();
                match branch {
                    4 => cur0 * constant,
                    5 => constant * cur0,
                    6 => cur0 + constant,
                    7 => constant + cur0,
                    _ => unreachable!(),
                }
            };
        }
        trace!("[B]: Remaining stack - {:?}", stacker::remaining_stack());

        cur[0] = cur0;
        cur
    }
}
