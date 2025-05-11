#![allow(non_snake_case)]

use crate::utils::{misc::batch_op, Point, Scalar};
use anyhow::Result;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_pallas::PallasConfig;
use ark_poly::univariate::DensePolynomial;
use halo_accumulation::pcdl::{self, EvalProof, Instance};
use rand::Rng;

pub trait PCS<P: SWCurveConfig> {
    type EvalProof;

    fn commit(f: &DensePolynomial<Scalar<P>>, d: usize, w: Option<&Scalar<P>>) -> Point<P>;
    fn check(
        succint: bool,
        C: &Point<P>,
        d: usize,
        z: &Scalar<P>,
        v: &Scalar<P>,
        pi: Self::EvalProof,
    ) -> Result<()>;

    fn batch_commit<'a, I>(ps: I, d: usize, w: Option<&Scalar<P>>) -> Vec<Point<P>>
    where
        I: IntoIterator<Item = &'a DensePolynomial<Scalar<P>>>,
    {
        batch_op(ps, |f| Self::commit(f, d, w))
    }
    fn open<R: Rng>(
        rng: &mut R,
        p: DensePolynomial<Scalar<P>>,
        d: usize,
        z: &Scalar<P>,
        w: Option<&Scalar<P>>,
    ) -> (Point<P>, usize, Scalar<P>, Scalar<P>, Self::EvalProof);
}

pub struct PCSPallas {}

impl PCS<PallasConfig> for PCSPallas {
    type EvalProof = EvalProof<PallasConfig>;
    fn commit(
        f: &DensePolynomial<Scalar<PallasConfig>>,
        d: usize,
        w: Option<&Scalar<PallasConfig>>,
    ) -> Point<PallasConfig> {
        pcdl::commit(f, d, w)
    }

    fn check(
        succint: bool,
        C: &Point<PallasConfig>,
        d: usize,
        z: &Scalar<PallasConfig>,
        v: &Scalar<PallasConfig>,
        pi: Self::EvalProof,
    ) -> Result<()> {
        if succint {
            let _ = pcdl::succinct_check(*C, d, z, v, pi)?;
            return Ok(());
        }
        pcdl::check(C, d, z, v, pi)
    }

    fn open<R: Rng>(
        rng: &mut R,
        p: DensePolynomial<Scalar<PallasConfig>>,
        d: usize,
        z: &Scalar<PallasConfig>,
        w: Option<&Scalar<PallasConfig>>,
    ) -> (
        Point<PallasConfig>,
        usize,
        Scalar<PallasConfig>,
        Scalar<PallasConfig>,
        Self::EvalProof,
    ) {
        Instance::open(rng, p, d, z, w).into_tuple()
    }
}
