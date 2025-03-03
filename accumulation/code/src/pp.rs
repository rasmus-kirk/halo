#![allow(non_snake_case)]

use crate::consts::*;
use crate::group::{PallasAffine, PallasPoint};
use crate::wrappers::WrappedPoint;
use anyhow::{bail, Result};
use ark_ff::BigInt;
use ark_pallas::Affine;
use ark_pallas::{Fq, Projective};
use bincode::config::standard;
use std::sync::OnceLock;

macro_rules! mk_proj {
    ($x:tt, $y:tt, $z:tt) => {
        Projective::new_unchecked(
            Fq::new_unchecked(BigInt::new($x)),
            Fq::new_unchecked(BigInt::new($y)),
            Fq::new_unchecked(BigInt::new($z)),
        )
    };
}

pub(crate) const S: Projective = mk_proj!(
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
pub(crate) const H: Projective = mk_proj!(
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

static PP: OnceLock<PublicParams> = OnceLock::new();
include!(concat!(env!("OUT_DIR"), "/pp_paths.rs"));

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

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.Gs.len() == 0
    }

    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        assert!(n <= N);

        let mut gs = Vec::with_capacity(n);
        let mut m = n;
        for bytes in G_PATHS.iter().take(G_BLOCKS_NO) {
            let (raw_gs, _): (Vec<WrappedPoint>, usize) =
                bincode::decode_from_slice(bytes, standard()).unwrap();
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
            Some(pp) if n > pp.Gs.len() => bail!(
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
