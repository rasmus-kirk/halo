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

pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = ark_ec::short_weierstrass::Affine<P>;
pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type BaseField<P: SWCurveConfig> = P::BaseField;
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
pub fn point_dot_affine<P: PastaConfig>(xs: &[Scalar<P>], Gs: &[Affine<P>]) -> Point<P> {
    Point::msm_unchecked(Gs, xs)
}

/// Dot product of projective points
pub fn point_dot<P: PastaConfig>(xs: &[Scalar<P>], Gs: &[Point<P>]) -> Point<P> {
    let gs = Point::<P>::normalize_batch(Gs);
    Point::<P>::msm_unchecked(&gs, xs)
}

pub fn construct_powers<P: PastaConfig>(z: &Scalar<P>, n: usize) -> Vec<Scalar<P>> {
    let mut zs = Vec::with_capacity(n);
    let mut current = Scalar::<P>::one();
    for _ in 0..n {
        zs.push(current);
        current *= z;
    }
    zs
}
