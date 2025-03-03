use rand::{ rngs::ThreadRng, Rng};

use crate::curve::Scalar;

use super::{Arithmetizer, Wire};

impl Arithmetizer {

    pub fn synthesize<const N: usize>(rng: &mut ThreadRng, degree: usize) -> Wire {
        let wires = &Arithmetizer::build::<N>();

        let mut cur = wires[rng.gen_range(0..N)].clone();
        while cur.arith().borrow().cache_len() < degree + N {
            let branch = rng.gen_range(0..8);
            cur = if branch < 4 {
                let rng_input = &wires[rng.gen_range(0..N)];
                match branch {
                    0 => cur * rng_input,
                    1 => rng_input * cur,
                    2 => cur + rng_input,
                    3 => rng_input + cur,
                    _ => unreachable!(),
                }
            } else {
                let constant: Scalar = rng.gen();
                match branch {
                    4 => cur * constant,
                    5 => constant * cur,
                    6 => cur + constant,
                    7 => constant + cur,
                    _ => unreachable!(),
                }
            }
        }

        cur
    }
}