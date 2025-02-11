use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

mod h;
use h::*;

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

criterion_group! {
    name = h;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        h_get_poly,
        h_eval,
        h_eval_naive,
        random_poly_eval_naive,
        h_eval_multiple,
        h_eval_multiple_naive
}

mod acc;
use acc::*;

criterion_group! {
    name = acc;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        public_parameters,

        acc_prover_5,
        acc_prover_6,
        acc_prover_7,
        acc_prover_8,
        acc_prover_9,
        acc_prover_10,
        acc_prover_11,
        acc_prover_12,
        acc_prover_13,
        acc_prover_14,
        acc_prover_15,
        acc_prover_16,
        acc_prover_17,
        acc_prover_18,
        acc_prover_19,
        acc_prover_20,

        // acc_decider_5,
        // acc_decider_6,
        // acc_decider_7,
        // acc_decider_8,
        // acc_decider_9,
        // acc_decider_10,
        // acc_decider_11,
        // acc_decider_12,
        // acc_decider_13,
        // acc_decider_14,
        // acc_decider_15,
        // acc_decider_16,
        // acc_decider_17,
        // acc_decider_18,
        // acc_decider_19,
        // acc_decider_20,

        acc_verifier_5,
        acc_verifier_6,
        acc_verifier_7,
        acc_verifier_8,
        acc_verifier_9,
        acc_verifier_10,
        acc_verifier_11,
        acc_verifier_12,
        acc_verifier_13,
        acc_verifier_14,
        acc_verifier_15,
        acc_verifier_16,
        acc_verifier_17,
        acc_verifier_18,
        acc_verifier_19,
        acc_verifier_20,

        // acc_common_subroutine_5,
        // acc_common_subroutine_6,
        // acc_common_subroutine_7,
        // acc_common_subroutine_8,
        // acc_common_subroutine_9,
        // acc_common_subroutine_10,
        // acc_common_subroutine_11,
        // acc_common_subroutine_12,
        // acc_common_subroutine_13,
        // acc_common_subroutine_14,
        // acc_common_subroutine_15,
        // acc_common_subroutine_16,
        // acc_common_subroutine_17,
        // acc_common_subroutine_18,
        // acc_common_subroutine_19,
        // acc_common_subroutine_20,
}

mod pcdl;
use pcdl::*;

criterion_group! {
    name = pcdl;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        // pcdl_commit_5,
        // pcdl_commit_6,
        // pcdl_commit_7,
        // pcdl_commit_8,
        // pcdl_commit_9,
        // pcdl_commit_10,
        // pcdl_commit_11,
        // pcdl_commit_12,
        // pcdl_commit_13,
        // pcdl_commit_14,
        // pcdl_commit_15,
        // pcdl_commit_16,
        // pcdl_commit_17,
        // pcdl_commit_18,
        // pcdl_commit_19,
        // pcdl_commit_20,

        //pcdl_open_5,
        //pcdl_open_6,
        pcdl_open_7,
        pcdl_open_8,
        pcdl_open_9,
        pcdl_open_10,
        //pcdl_open_11,
        //pcdl_open_12,
        //pcdl_open_13,
        //pcdl_open_14,
        //pcdl_open_15,
        //pcdl_open_16,
        //pcdl_open_17,
        //pcdl_open_18,
        //pcdl_open_19,
        //pcdl_open_20,

        //pcdl_check_5,
        //pcdl_check_6,
        pcdl_check_7,
        pcdl_check_8,
        pcdl_check_9,
        pcdl_check_10,
        //pcdl_check_11,
        //pcdl_check_12,
        //pcdl_check_13,
        //pcdl_check_14,
        //pcdl_check_15,
        //pcdl_check_16,
        //pcdl_check_17,
        //pcdl_check_18,
        //pcdl_check_19,
        //pcdl_check_20,

        pcdl_succinct_check_5,
        pcdl_succinct_check_6,
        pcdl_succinct_check_7,
        pcdl_succinct_check_8,
        pcdl_succinct_check_9,
        pcdl_succinct_check_10,
        pcdl_succinct_check_11,
        pcdl_succinct_check_12,
        pcdl_succinct_check_13,
        pcdl_succinct_check_14,
        pcdl_succinct_check_15,
        pcdl_succinct_check_16,
        pcdl_succinct_check_17,
        pcdl_succinct_check_18,
        pcdl_succinct_check_19,
        pcdl_succinct_check_20,
}

criterion_main!(acc, pcdl, h);
