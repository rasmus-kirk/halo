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

criterion_main!(misc);
