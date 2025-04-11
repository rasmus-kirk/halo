#![allow(non_snake_case, unused_macros, dead_code, unused_imports)]

use std::{
    cmp::{max, min},
    mem::{self, transmute},
    ops::Deref,
    thread,
};

use ark_ec::{short_weierstrass::{Projective, SWCurveConfig}, CurveGroup, VariableBaseMSM};
use ark_ff::{AdditiveGroup, BigInt, Field};
use ark_poly::{univariate::DensePolynomial, Polynomial};
use ark_std::One;
use rayon::prelude::*;

pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = ark_ec::short_weierstrass::Affine<P>;
pub type Scalar<P: SWCurveConfig> = P::ScalarField;

pub type PallasPoint = ark_pallas::Projective;
pub type PallasAffine = ark_pallas::Affine;
pub type PallasScalar = ark_pallas::Fr;
pub type PallasPoly = DensePolynomial<PallasScalar>;

/// The ideal number of cores for parallel processing
pub const IDEAL_CORES: usize = 16;

/// Dot product of scalars
pub fn scalar_dot(xs: &[PallasScalar], ys: &[PallasScalar]) -> PallasScalar {
    if cfg!(feature = "parallel") {
        xs.par_iter().zip(ys).map(|(x, y)| x * y).sum()
    } else {
        xs.iter().zip(ys).map(|(x, y)| x * y).sum()
    }
}

/// Dot product of affine points
pub fn point_dot_affine(xs: &[PallasScalar], Gs: &[PallasAffine]) -> PallasPoint {
    if cfg!(feature = "parallel") {
        const IDEAL_CORES: usize = 16;
        let chunks = max(1 << 10, xs.len() / IDEAL_CORES);
        Gs.par_chunks(chunks)
            .zip(xs.par_chunks(chunks))
            .map(|(gs, xs)| PallasPoint::msm_unchecked(gs, xs))
            .sum()
    } else {
        PallasPoint::msm_unchecked(Gs, xs)
    }
}

/// Dot product of projective points
pub fn point_dot(xs: &[PallasScalar], Gs: &[PallasPoint]) -> PallasPoint {
    let gs = PallasPoint::normalize_batch(Gs);
    point_dot_affine(xs, &gs)
}

pub(crate) fn construct_powers(z: &PallasScalar, n: usize) -> Vec<PallasScalar> {
    if cfg!(feature = "parallel") {
        let mut result = vec![PallasScalar::ZERO; n];
        let chunk_size = max(1 << 10, n / IDEAL_CORES);

        result
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, chunk)| {
                let mut power = z.pow([(chunk_idx * chunk_size) as u64]);
                for x in chunk.iter_mut() {
                    *x = power;
                    power *= z; // Sequential multiplications within each chunk
                }
            });

        result
    } else {
        let mut zs = Vec::with_capacity(n);
        let mut current = PallasScalar::one();
        for _ in 0..n {
            zs.push(current);
            current *= z;
        }
        zs
    }
}

use sha3::{Digest, Sha3_256};

// These are ugly, but it really cleans up the implementation
// They just hash either points or scalars to a single scalar, prepending either 0 or 1
macro_rules! rho_0 {
    ($($a:expr),+ $(,)?) => {{
        use ::sha3::{Digest, Sha3_256};
        use $crate::group::PallasScalar;

        let mut size = 0;
        $(
            size += $a.compressed_size();
         )+
        let mut data = Vec::with_capacity(size);
        $(
            $a.serialize_compressed(&mut data).unwrap();
         )+

        let mut hasher = Sha3_256::new();
        hasher.update(&data);
        hasher.update(&0u32.to_le_bytes());
        let hash_result = hasher.finalize();

        // Interpret the hash as a scalar field element
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash_result[..32]);
        let scalar = PallasScalar::from_le_bytes_mod_order(&hash_bytes);

        scalar
    }};
}

macro_rules! rho_1 {
    ($($a:expr),+ $(,)?) => {{
        use ::sha3::{Digest, Sha3_256};
        use $crate::group::PallasScalar;

        let mut size = 0;
        $(
            size += $a.compressed_size();
         )+
        let mut data = Vec::with_capacity(size);
        $(
            $a.serialize_compressed(&mut data).unwrap();
         )+

        let mut hasher = Sha3_256::new();
        hasher.update(&data);
        hasher.update(&1u32.to_le_bytes());
        let hash_result = hasher.finalize();

        // Interpret the hash as a scalar field element
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash_result[..32]);
        let scalar = PallasScalar::from_le_bytes_mod_order(&hash_bytes);

        scalar
    }};
}

pub(crate) use rho_0;
pub(crate) use rho_1;
