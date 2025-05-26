#![allow(non_snake_case)]

use std::fmt::Display;

use crate::consts::*;
use crate::group::{Affine, Point};
use crate::wrappers::{PastaConfig, WrappedPoint};
use anyhow::{bail, Result};
use bincode::config::standard;

#[derive(Debug)]
pub struct PublicParams<P: PastaConfig> {
    pub S: Point<P>,
    pub H: Point<P>,
    pub D: usize,
    pub Gs: Vec<Affine<P>>,
}

impl<P: PastaConfig> PublicParams<P> {
    pub fn len(&self) -> usize {
        self.Gs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.Gs.len() == 0
    }

    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        assert!(n <= N);

        let mut gs = Vec::with_capacity(n);
        let mut m = n;
        for (i, bytes) in P::get_g_data().iter().enumerate() {
            let raw_gs = bincode::decode_from_slice::<Vec<WrappedPoint>, _>(bytes, standard())
                .expect(format!("Failed to decode G_BLOCKS_NO {}", i).as_str())
                .0
                .into_iter()
                .take(m)
                .map(P::unwrap_affine);
            gs.extend(raw_gs);

            if let Some(new_m) = m.checked_sub(G_BLOCKS_SIZE) {
                m = new_m
            } else {
                break;
            }
        }
        let ((S, H), _): ((WrappedPoint, WrappedPoint), usize) =
            bincode::decode_from_slice(P::get_sh_data(), standard()).expect(format!("Failed to get SH data for curve {}", P::CURVE_NAME).as_str());

        PublicParams {
            S: P::unwrap_projective(S),
            H: P::unwrap_projective(H),
            D: n - 1,
            Gs: gs,
        }
    }

    pub fn set_pp(n: usize) -> Result<()> {
        match P::get_loaded_public_params().get() {
            Some(pp) if n > pp.Gs.len() => bail!(
                "Previous public parameters defined to be {}, which is smaller than new public parameters {}",
                pp.Gs.len(),
                n
            ),
            Some(&_) => Ok(()),
            None => {
                match P::get_loaded_public_params().set(PublicParams::new(n)) {
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

    pub fn get_pp() -> &'static PublicParams<P> {
        match P::get_loaded_public_params().get() {
            Some(&_) => P::get_loaded_public_params().get().unwrap(),
            // If no public params have been set, set to max.
            // This will degrade performance, but always work.
            None => {
                let _ = P::get_loaded_public_params().set(PublicParams::new(N));
                P::get_loaded_public_params().get().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use ark_ec::CurveGroup;
    use ark_pallas::PallasConfig;
    use rand::{distributions::Uniform, Rng};

    use super::PublicParams;

    const LAMBDA: usize = 1;

    fn test_pp(n: usize, sec: usize) -> Result<()> {
        let mut rng = rand::thread_rng();

        let pp = PublicParams::<PallasConfig>::new(n);

        assert_eq!(pp.Gs.len(), n);
        assert!(pp.S.into_affine().is_on_curve());
        assert!(pp.H.into_affine().is_on_curve());
        // Sample and verify LAMBDA random points from Gs
        for _ in 0..(LAMBDA * sec) {
            let j = rng.sample(Uniform::new(n / 2, n));
            assert!(pp.Gs[j].is_on_curve());
        }

        Ok(())
    }

    #[test]
    fn test_big_pp() -> Result<()> {
        let n = 2usize.pow(20);
        test_pp(n, 100)?;
        Ok(())
    }

    #[test]
    fn test_many_pps() -> Result<()> {
        let mut rng = rand::thread_rng();
        for _ in 0..LAMBDA {
            let n = 2usize.pow(rng.sample(Uniform::new(2, 20)));
            test_pp(n, 10)?
        }

        Ok(())
    }
}
