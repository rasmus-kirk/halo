#![allow(non_snake_case)]

use std::cmp::min;

use crate::archive::{std_config, WrappedPoint, G_BLOCKS_NO, G_BLOCKS_SIZE};
use crate::consts::*;
use crate::group::{PallasAffine, PallasPoint};
use ark_pallas::Affine;
use seq_macro::seq;

macro_rules! define_g_paths {
    ($limit:literal) => {
        seq!(K in 0..$limit {
            pub const G_PATHS: [&[u8]; $limit] = [
                #(
                    include_bytes!(concat!(env!("OUT_DIR"), "/public-params/gs-", K, ".bin")),
                )*
            ];
        });
    }
}
get_no_of_blocks!(define_g_paths); //G_PATHS

#[derive(Debug)]
pub struct PublicParams {
    pub(crate) S: PallasPoint,
    pub(crate) H: PallasPoint,
    pub(crate) D: usize,
    pub(crate) Gs: Vec<PallasAffine>,
}

impl PublicParams {
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        assert!(n <= N);

        let mut gs = Vec::with_capacity(n);
        let mut m = n;
        for i in 0..G_BLOCKS_NO {
            let data = G_PATHS[i];
            let (raw_gs, _): (Vec<WrappedPoint>, usize) =
                bincode::decode_from_slice(&data, std_config()).unwrap();
            let mut converted_gs: Vec<Affine> =
                raw_gs.into_iter().take(m).map(|x| x.into()).collect();
            gs.append(&mut converted_gs);

            if let Some(new_m) = m.checked_sub(G_BLOCKS_SIZE) {
                m = new_m
            } else {
                break;
            }
        }

        PublicParams { S, H, D, Gs: gs }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use anyhow::Result;
    use ark_ec::CurveGroup;
    use rand::{distributions::Uniform, Rng};

    use super::PublicParams;

    const LAMBDA: usize = 100;

    #[test]
    fn test_random_pp() -> Result<()> {
        let mut rng = rand::thread_rng();
        let n = (2 as usize).pow(rng.sample(&Uniform::new(2, 20)));

        let pp = PublicParams::new(n);

        println!("n = {}", n);
        assert!(pp.S.into_affine().is_on_curve());
        assert!(pp.H.into_affine().is_on_curve());
        for i in 0..min(LAMBDA, pp.Gs.len()) {
            assert!(pp.Gs[i].is_on_curve());
        }
        assert_eq!(n, pp.Gs.len());

        Ok(())
    }

    #[test]
    fn test_many_pps() -> Result<()> {
        for _ in 0..LAMBDA {
            test_random_pp()?
        }

        Ok(())
    }
}
