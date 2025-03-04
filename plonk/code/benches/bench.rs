use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

use ark_std::test_rng;
use plonk::protocol::{arithmetizer::Arithmetizer, plonk as plonker};

const WARMUP: Duration = Duration::from_millis(100);
const MIN: usize = 5;
const MAX: usize = 12;

pub fn plonk_proof_verify(c: &mut Criterion) {
    let rng = &mut test_rng();
    let mut pis = Vec::new();

    let mut group = c.benchmark_group("plonk_proof_verify");
    for size in MIN..MAX + 1 {
        let out = Arithmetizer::synthesize::<_, 4>(rng, 2usize.pow(size as u32));
        let input_values = vec![3,4,5,6];
        let output_wires = &[out];
        let ((x, w), _) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
        let pi = plonker::proof(rng, x, w);
        pis.push((size, x.clone(), pi.clone()));

        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::new("prover", size),
            &size,
            |b, _| {
                b.iter(|| {
                    plonker::proof(rng, x, w);
                })
            },
        );
    }
    for (i, x, pi) in pis {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::new("verifier", i),
            &i,
            |b, _| {
                b.iter(|| {
                    plonker::verify(&x, pi.clone());
                })
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = plonks;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        plonk_proof_verify,
}

criterion_main!(plonks);
