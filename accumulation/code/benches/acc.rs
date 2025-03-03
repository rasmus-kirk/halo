#![allow(dead_code)]

use std::time::Duration;

use ark_std::test_rng;
use bincode::config::standard;
use criterion::{BenchmarkId, Criterion};

use halo_accumulation::{
    acc::{self, Accumulator},
    pcdl::Instance,
    wrappers::{WrappedAccumulator, WrappedInstance},
};

const PRE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/precompute/qs/qs.bin"));

/// Helper function: Gets precomputed linear-time computation dummy values.
fn get_cheap_linears(n: usize) -> ([Instance; 1], Accumulator) {
    let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
        bincode::decode_from_slice(&PRE, standard()).unwrap();
    let q_acc = val.into_iter().find(|x| x.0 == n).unwrap();
    ([q_acc.1.into()], q_acc.2.into())
}

const WARMUP: Duration = Duration::from_millis(100);
const MIN: usize = 2;
const MAX: usize = 20;

pub fn acc_prover(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("acc_prover");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let (qs, _) = get_cheap_linears(n);

                b.iter(|| acc::prover(rng, &qs));
            },
        );
    }
    group.finish();
}

pub fn acc_decider(c: &mut Criterion) {
    let mut group = c.benchmark_group("acc_decider");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let (_, acc) = get_cheap_linears(n);

                b.iter(|| acc.clone().decider());
            },
        );
    }
    group.finish();
}

pub fn acc_verifier(c: &mut Criterion) {
    let mut group = c.benchmark_group("acc_verifier");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let (qs, acc) = get_cheap_linears(n);

                b.iter(|| acc.clone().verifier(&qs));
            },
        );
    }
    group.finish();
}
