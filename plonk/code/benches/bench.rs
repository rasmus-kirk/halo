use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

use ark_std::test_rng;
use plonk::protocol::{arithmetizer::Arithmetizer, plonk as plonker};

const WARMUP: Duration = Duration::from_millis(100);

pub fn plonk_proof_verify(c: &mut Criterion) {
    let rng = &mut test_rng();

    let [x, y] = &Arithmetizer::build();
    let input_values = vec![1, 2];
    let output_wires = &[3 * (x * x) + (y * 5) - 47 - (x * x * x * y) + y * x + 12 - 12445];
    let ((x, w), _) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
    let pi = plonker::proof(rng, x, w);

    let mut group = c.benchmark_group("acc_prover");
    group.warm_up_time(WARMUP).bench_function("prover", |b| {
        b.iter(|| {
            plonker::proof(rng, x, w);
        })
    });
    group.warm_up_time(WARMUP).bench_function("verifier", |b| {
        b.iter(|| {
            plonker::verify(x, pi.clone());
        })
    });
    group.finish();
}

criterion_group! {
    name = plonks;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        plonk_proof_verify,
}

criterion_main!(plonks);
