#![allow(non_snake_case)]

use crate::utils::{misc::batch_op, Point, Poly, Scalar};
use anyhow::Result;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_pallas::PallasConfig;
use halo_accumulation::pcdl::{self, EvalProof, Instance};
use rand::Rng;

pub trait PCS<P: SWCurveConfig> {
    type EvalProof;

    fn commit(f: &Poly<P>, d: usize, w: Option<&Scalar<P>>) -> Point<P>;
    fn check(
        C: &Point<P>,
        d: usize,
        z: &Scalar<P>,
        v: &Scalar<P>,
        pi: Self::EvalProof,
    ) -> Result<()>;

    fn batch_commit<'a, I>(ps: I, d: usize, w: Option<&Scalar<P>>) -> Vec<Point<P>>
    where
        I: IntoIterator<Item = &'a Poly<P>>,
    {
        batch_op(ps, |f| Self::commit(f, d, w))
    }
    fn open<R: Rng>(
        rng: &mut R,
        p: Poly<P>,
        d: usize,
        z: &Scalar<P>,
        w: Option<&Scalar<P>>,
    ) -> (Point<P>, usize, Scalar<P>, Scalar<P>, Self::EvalProof);
}

pub struct PCSPallas {}

impl PCS<PallasConfig> for PCSPallas {
    type EvalProof = EvalProof;
    fn commit(
        f: &Poly<PallasConfig>,
        d: usize,
        w: Option<&Scalar<PallasConfig>>,
    ) -> Point<PallasConfig> {
        pcdl::commit(f, d, w)
    }

    fn check(
        C: &Point<PallasConfig>,
        d: usize,
        z: &Scalar<PallasConfig>,
        v: &Scalar<PallasConfig>,
        pi: EvalProof,
    ) -> Result<()> {
        pcdl::check(C, d, z, v, pi)
    }

    fn open<R: Rng>(
        rng: &mut R,
        p: Poly<PallasConfig>,
        d: usize,
        z: &Scalar<PallasConfig>,
        w: Option<&Scalar<PallasConfig>>,
    ) -> (
        Point<PallasConfig>,
        usize,
        Scalar<PallasConfig>,
        Scalar<PallasConfig>,
        EvalProof,
    ) {
        Instance::open(rng, p, d, z, w).into_tuple()
    }
}
