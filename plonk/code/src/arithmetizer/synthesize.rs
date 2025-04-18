use super::{Arithmetizer, Wire};

use halo_accumulation::group::PallasScalar;

use log::info;
use rand::Rng;

type Scalar = PallasScalar;

impl Arithmetizer {
    pub fn synthesize<R: Rng, const N: usize>(rng: &mut R, degree: usize) -> [Wire; 1] {
        info!("[A]: Remaining stack - {:?}", stacker::remaining_stack());
        let wires: Vec<Wire> = Arithmetizer::build::<N>().into();

        let mut cur = wires[rng.gen_range(0..N)].clone();

        while cur.arith().borrow().cache_len() < degree + N {
            let branch = rng.gen_range(0..8);
            cur = if branch < 4 {
                let rng_input = wires[rng.gen_range(0..N)].clone();
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
                    4 | 5 => cur * constant,
                    6 | 7 => cur + constant,
                    _ => unreachable!(),
                }
            };
        }
        info!("[B]: Remaining stack - {:?}", stacker::remaining_stack());

        [cur]
    }
}
