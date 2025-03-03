#![allow(dead_code)]

use std::time::Duration;

use ark_ff::UniformRand;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_std::test_rng;
use bincode::config::standard;
use criterion::{BenchmarkId, Criterion};

use halo_accumulation::{
    group::{self, PallasPoly, PallasScalar},
    pcdl::{self, commit, Instance},
    wrappers::{WrappedAccumulator, WrappedInstance},
};

const PRE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/precompute/qs/qs.bin"));

/// Helper function: Gets precomputed linear-time computation dummy values.
fn get_cheap_linears(n: usize) -> [Instance; 1] {
    let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
        bincode::decode_from_slice(&PRE, standard()).unwrap();
    let q_acc = val.into_iter().find(|x| x.0 == n).unwrap();
    [q_acc.1.into()]
}

const WARMUP: Duration = Duration::from_millis(500);
const MIN: usize = 12;
const MAX: usize = 20;

pub fn pcdl_open(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("pcdl_open");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let d = n - 1;

                let w = Some(PallasScalar::rand(rng));
                let p = PallasPoly::rand(d, rng);
                let z = &PallasScalar::rand(rng);
                let comm = commit(&p, d, w.as_ref());

                b.iter(|| pcdl::open(rng, p.clone(), comm, d, z, w.as_ref()));
            },
        );
    }
    group.finish();
}

pub fn pcdl_commit(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("pcdl_commit");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let d = n - 1;

                let w = Some(PallasScalar::rand(rng));
                let p = PallasPoly::rand(d, rng);

                b.iter(|| pcdl::commit(&p, d, w.as_ref()));
            },
        );
    }
    group.finish();
}

pub fn pcdl_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("pcdl_check");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let qs = get_cheap_linears(n);

                b.iter(|| qs[0].check().unwrap());
            },
        );
    }
    group.finish();
}

pub fn pcdl_succinct_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("pcdl_succinct_check");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let qs = get_cheap_linears(n);

                b.iter(|| qs[0].succinct_check().unwrap());
            },
        );
    }
    group.finish();
}

pub fn pcdl_eval(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("pcdl_eval");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let p = PallasPoly::rand(n - 1, rng);
                let z = PallasScalar::rand(rng);

                b.iter(|| group::eval(&p, &z));
            },
        );
    }
    group.finish();
}
