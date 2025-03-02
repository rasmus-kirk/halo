#![allow(non_snake_case)]

use crate::archive::{std_config, WrappedPoint};
use crate::consts::*;
use crate::group::{PallasAffine, PallasPoint};
use anyhow::{bail, Result};
use ark_pallas::Affine;
use seq_macro::seq;
use std::sync::OnceLock;

static PP: OnceLock<PublicParams> = OnceLock::new();

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
    pub S: PallasPoint,
    pub H: PallasPoint,
    pub D: usize,
    pub Gs: Vec<PallasAffine>,
}

impl PublicParams {
    pub fn len(&self) -> usize {
        self.Gs.len()
    }

    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        assert!(n <= N);

        let mut gs = Vec::with_capacity(n);
        let mut m = n;
        for bytes in G_PATHS.iter().take(G_BLOCKS_NO) {
            let (raw_gs, _): (Vec<WrappedPoint>, usize) =
                bincode::decode_from_slice(bytes, std_config()).unwrap();
            let mut converted_gs: Vec<Affine> =
                raw_gs.into_iter().take(m).map(|x| x.into()).collect();
            gs.append(&mut converted_gs);

            if let Some(new_m) = m.checked_sub(G_BLOCKS_SIZE) {
                m = new_m
            } else {
                break;
            }
        }

        PublicParams {
            S,
            H,
            D: n - 1,
            Gs: gs,
        }
    }

    pub fn set_pp(n: usize) -> Result<()> {
        match PP.get() {
            Some(&ref pp) if n > pp.Gs.len() => bail!(
                "Previous public parameters defined to be {}, which is smaller than new public parameters {}",
                pp.Gs.len(),
                n
            ),
            Some(&_) => Ok(()),
            None => {
                //println!("setting pp: {}", n);
                match PP.set(PublicParams::new(n)) {
                    Ok(_) => Ok(()),
                    Err(pp) => if pp.len() == n {
                        Ok(())
                    } else {
                        bail!("pp was already set with length: {}", pp.len())
                    }
                }
            }
        }
    }

    pub fn get_pp() -> &'static PublicParams {
        match PP.get() {
            Some(&_) => PP.get().unwrap(),
            // If no public params have been set, set to max.
            // This will degrade performance, but always work.
            None => {
                let _ = PP.set(PublicParams::new(N));
                PP.get().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use ark_ec::CurveGroup;
    use rand::{distributions::Uniform, Rng};

    use super::PublicParams;

    const LAMBDA: usize = 10;

    fn test_pp(n: usize, sec: usize) -> Result<()> {
        let mut rng = rand::thread_rng();

        let pp = PublicParams::new(n);

        assert_eq!(pp.Gs.len(), n);
        assert!(pp.S.into_affine().is_on_curve());
        assert!(pp.H.into_affine().is_on_curve());
        // Sample and verify LAMBDA random points from Gs
        for _ in 0..(LAMBDA * sec) {
            let j = rng.sample(&Uniform::new(n / 2, n));
            assert!(pp.Gs[j].is_on_curve());
        }

        Ok(())
    }

    #[test]
    fn test_big_pp() -> Result<()> {
        let n = (2 as usize).pow(20);
        test_pp(n, 100)?;
        Ok(())
    }

    #[test]
    fn test_many_pps() -> Result<()> {
        let mut rng = rand::thread_rng();
        for _ in 0..LAMBDA {
            let n = (2 as usize).pow(rng.sample(&Uniform::new(2, 20)));
            test_pp(n, 10)?
        }

        Ok(())
    }
}
