use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

mod acc;
use acc::*;

criterion_group! {
    name = acc;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        acc_prover,
        acc_decider,
        acc_verifier,
}

mod pcdl;
use pcdl::*;

criterion_group! {
    name = pcdl;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        pcdl_eval,
        pcdl_open,
        pcdl_commit,
        pcdl_check,
        pcdl_succinct_check,
}

criterion_main!(pcdl, acc,);
