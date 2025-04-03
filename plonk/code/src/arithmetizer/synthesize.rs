use super::{plookup::PlookupOps, Arithmetizer, Wire};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{Fp, FpConfig};

use log::info;
use rand::Rng;

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Arithmetizer<Op, N, C, P> {
    pub fn synthesize<R: Rng, const M: usize>(
        rng: &mut R,
        degree: usize,
    ) -> [Wire<Op, N, C, P>; 1] {
        info!("[A]: Remaining stack - {:?}", stacker::remaining_stack());
        let wires: Vec<Wire<Op, N, C, P>> = Self::build::<M>().into();

        let mut cur = wires[rng.gen_range(0..M)].clone();

        while cur.arith().borrow().cache_len() < degree + M {
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
                let constant: Fp<C, N> = rng.gen();
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
