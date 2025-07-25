use super::{plookup::PlookupOps, Arithmetizer, Wire};
use crate::utils::Scalar;

use ark_ec::short_weierstrass::SWCurveConfig;

use rand::{distributions::Standard, prelude::Distribution, Rng};

impl<Op: PlookupOps, P: SWCurveConfig> Arithmetizer<Op, P> {
    pub fn synthesize<const M: usize, R: Rng>(rng: &mut R, degree: usize) -> [Wire<Op, P>; 1]
    where
        Standard: Distribution<Scalar<P>>,
    {
        let wires: Vec<Wire<Op, P>> = Self::build::<M>().into();

        let mut cur = wires[rng.gen_range(0..M)].clone();

        while cur.arith().borrow().cache_len() < degree + M - 1 {
            let branch = rng.gen_range(0..8);
            cur = if branch < 4 {
                let rng_input = wires[rng.gen_range(0..M)].clone();
                match branch {
                    0 => cur * rng_input,
                    1 => rng_input * cur,
                    2 => cur + rng_input,
                    3 => rng_input + cur,
                    _ => unreachable!(),
                }
            } else {
                let constant: Scalar<P> = rng.gen();
                match branch {
                    4 | 5 => cur * constant,
                    6 | 7 => cur + constant,
                    _ => unreachable!(),
                }
            };
        }

        [cur]
    }
}
