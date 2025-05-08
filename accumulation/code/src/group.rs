#![allow(
    non_snake_case,
    unused_macros,
    dead_code,
    unused_imports,
    type_alias_bounds
)]

use std::{
    cmp::{max, min},
    mem::{self, transmute},
    ops::Deref,
    thread,
};

use crate::wrappers::PastaConfig;
use ark_ec::{
    short_weierstrass::{Projective, SWCurveConfig},
    CurveGroup, VariableBaseMSM,
};
use ark_ff::PrimeField;
use ark_ff::{AdditiveGroup, BigInt, Field};
use ark_poly::{univariate::DensePolynomial, Polynomial};
use ark_serialize::CanonicalSerialize;
use ark_std::One;
use rayon::prelude::*;
use sha3::{Digest, Sha3_256};

pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = ark_ec::short_weierstrass::Affine<P>;
pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type Poly<P: SWCurveConfig> = DensePolynomial<P::ScalarField>;

pub type PallasPoint = ark_pallas::Projective;
pub type PallasAffine = ark_pallas::Affine;
pub type PallasScalar = ark_pallas::Fr;
pub type PallasPoly = DensePolynomial<PallasScalar>;

/// The ideal number of cores for parallel processing
pub const IDEAL_CORES: usize = 16;

/// Dot product of scalars
pub fn scalar_dot<P: PastaConfig>(xs: &[Scalar<P>], ys: &[Scalar<P>]) -> Scalar<P> {
    xs.par_iter().zip(ys).map(|(x, y)| *x * *y).sum()
}

/// Dot product of affine points
pub(crate) fn point_dot_affine<P: PastaConfig>(xs: &[Scalar<P>], Gs: &[Affine<P>]) -> Point<P> {
    Point::msm_unchecked(Gs, xs)
}

/// Dot product of projective points
pub fn point_dot<P: PastaConfig>(xs: &[Scalar<P>], Gs: &[Point<P>]) -> Point<P> {
    let gs = Point::<P>::normalize_batch(Gs);
    Point::<P>::msm_unchecked(&gs, xs)
}

pub(crate) fn construct_powers<P: PastaConfig>(z: &Scalar<P>, n: usize) -> Vec<Scalar<P>> {
    let mut zs = Vec::with_capacity(n);
    let mut current = Scalar::<P>::one();
    for _ in 0..n {
        zs.push(current);
        current *= z;
    }
    zs
}

fn rho<P: PastaConfig>(domain_sep: &[u8], scalars: &[Scalar<P>], points: &[Point<P>]) -> Scalar<P> {
    let size = scalars.iter().map(|x| x.compressed_size()).sum::<usize>()
        + points.iter().map(|x| x.compressed_size()).sum::<usize>();

    let mut data = Vec::with_capacity(size);
    for scalar in scalars {
        scalar.serialize_compressed(&mut data).unwrap()
    }

    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    hasher.update(domain_sep);
    let hash_result = hasher.finalize();

    // Interpret the hash as a scalar field element
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(hash_result.as_slice());
    Scalar::<P>::from_le_bytes_mod_order(&hash_bytes)
}

pub(crate) fn rho0<P: PastaConfig>(scalars: &[Scalar<P>], points: &[Point<P>]) -> Scalar<P> {
    rho(&[0], scalars, points)
}

pub(crate) fn rho1<P: PastaConfig>(scalars: &[Scalar<P>], points: &[Point<P>]) -> Scalar<P> {
    rho(&[1], scalars, points)
}
