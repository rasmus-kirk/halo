use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

mod misc;
use misc::*;

criterion_group! {
    name = misc;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        poly_naive_mul,
        poly_evals_mul,
        poly_evals_mul_raw,
        poly_evals_fft,
}

mod protocol;
use protocol::*;

criterion_group! {
    name = protocol;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        // prover_verifier_scalar_mul,
        prover_verifier_pcdl,
}

criterion_main!(misc, protocol);
