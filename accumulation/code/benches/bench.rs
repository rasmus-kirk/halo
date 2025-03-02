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
        h_get_poly_10,
        // h_get_poly_11,
        // h_get_poly_12,
        // h_get_poly_13,
        // h_get_poly_14,
        // h_get_poly_15,
        // h_get_poly_16,
        // h_get_poly_17,
        // h_get_poly_18,
        // h_get_poly_19,
        // h_get_poly_20,

        // h_eval_10,
        // h_eval_11,
        // h_eval_12,
        // h_eval_13,
        // h_eval_14,
        // h_eval_15,
        // h_eval_16,
        // h_eval_17,
        // h_eval_18,
        // h_eval_19,
        // h_eval_20,

        // h_eval_naive_10,
        // h_eval_naive_11,
        // h_eval_naive_12,
        // h_eval_naive_13,
        // h_eval_naive_14,
        // h_eval_naive_15,
        // h_eval_naive_16,
        // h_eval_naive_17,
        // h_eval_naive_18,
        // h_eval_naive_19,
        // h_eval_naive_20,

        // h_hs_get_poly_10,
        // h_hs_get_poly_11,
        // h_hs_get_poly_12,
        // h_hs_get_poly_13,
        // h_hs_get_poly_14,
        // h_hs_get_poly_15,
        // h_hs_get_poly_16,
        // h_hs_get_poly_17,
        // h_hs_get_poly_18,
        // h_hs_get_poly_19,
        // h_hs_get_poly_20,

        // h_hs_eval_10,
        // h_hs_eval_11,
        // h_hs_eval_12,
        // h_hs_eval_13,
        // h_hs_eval_14,
        // h_hs_eval_15,
        // h_hs_eval_16,
        // h_hs_eval_17,
        // h_hs_eval_18,
        // h_hs_eval_19,
        // h_hs_eval_20,
}

mod acc;
use acc::*;

criterion_group! {
    name = acc_prover;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        acc_prover_2,
        acc_prover_3,
        acc_prover_4,
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
}

criterion_group! {
    name = acc_decider;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        acc_decider_2,
        acc_decider_3,
        acc_decider_4,
        acc_decider_5,
        acc_decider_6,
        acc_decider_7,
        acc_decider_8,
        acc_decider_9,
        acc_decider_10,
        acc_decider_11,
        acc_decider_12,
        acc_decider_13,
        acc_decider_14,
        acc_decider_15,
        acc_decider_16,
        acc_decider_17,
        acc_decider_18,
        acc_decider_19,
        acc_decider_20,
}

criterion_group! {
    name = acc_verifier;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        acc_verifier_2,
        acc_verifier_3,
        acc_verifier_4,
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
}

mod pcdl;
use halo_accumulation::{consts, group::{self, PallasPoint, PallasScalar}, pp::{self, PublicParams}};
use pcdl::*;

criterion_group! {
    name = pcdl_commit;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        pcdl_commit_2,
        pcdl_commit_3,
        pcdl_commit_4,
        pcdl_commit_5,
        pcdl_commit_6,
        pcdl_commit_7,
        pcdl_commit_8,
        pcdl_commit_9,
        pcdl_commit_10,
        pcdl_commit_11,
        pcdl_commit_12,
        pcdl_commit_13,
        pcdl_commit_14,
        pcdl_commit_15,
        pcdl_commit_16,
        pcdl_commit_17,
        pcdl_commit_18,
        pcdl_commit_19,
        pcdl_commit_20,
}

criterion_group! {
    name = pcdl_open;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        pcdl_open_2,
        pcdl_open_3,
        pcdl_open_4,
        pcdl_open_5,
        pcdl_open_6,
        pcdl_open_7,
        pcdl_open_8,
        pcdl_open_9,
        pcdl_open_10,
        pcdl_open_11,
        pcdl_open_12,
        pcdl_open_13,
        pcdl_open_14,
        pcdl_open_15,
        pcdl_open_16,
        pcdl_open_17,
        pcdl_open_18,
        pcdl_open_19,
        pcdl_open_20,
}

criterion_group! {
    name = pcdl_check;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        pcdl_check_2,
        pcdl_check_3,
        pcdl_check_4,
        pcdl_check_5,
        pcdl_check_6,
        pcdl_check_7,
        pcdl_check_8,
        pcdl_check_9,
        pcdl_check_10,
        pcdl_check_11,
        pcdl_check_12,
        pcdl_check_13,
        pcdl_check_14,
        pcdl_check_15,
        pcdl_check_16,
        pcdl_check_17,
        pcdl_check_18,
        pcdl_check_19,
        pcdl_check_20,
}

criterion_group! {
    name = pcdl_succinct_check;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        pcdl_succinct_check_2,
        pcdl_succinct_check_3,
        pcdl_succinct_check_4,
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

// fn msm_affine(c: &mut Criterion) {
//     let rng = &mut test_rng();
//     static MIN: usize = 2;
//     static MAX: usize = 20;
//     let scalars: Vec<PallasScalar> = vec![PallasScalar::rand(rng); 2usize.pow(MAX as u32)];
//     let pp = PublicParams::get_pp();

//     let mut group = c.benchmark_group("msm_affine_new");
//     for size in MIN..(MAX+1) {
//         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::new_point_dot_affine(&scalars[0..size], &pp.Gs[0..size]));
//         });
//     }
//     group.finish();

//     let mut group_old = c.benchmark_group("msm_affine_old");
//     for size in MIN..(MAX+1) {
//         group_old.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::old_point_dot_affine(&scalars[0..size], &pp.Gs[0..size]));
//         });
//     }
//     group_old.finish();
// }

// fn msm_scalar(c: &mut Criterion) {
//     let rng = &mut test_rng();
//     static MIN: usize = 2;
//     static MAX: usize = 20;
//     let scalars_1: Vec<PallasScalar> = vec![PallasScalar::rand(rng); 2usize.pow(MAX as u32)];
//     let scalars_2: Vec<PallasScalar> = vec![PallasScalar::rand(rng); 2usize.pow(MAX as u32)];

//     let mut group_new = c.benchmark_group("msm_scalar_new");
//     for size in MIN..(MAX+1) {
//         group_new.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::scalar_dot_new(&scalars_1[0..size], &scalars_2[0..size]));
//         });
//     }
//     group_new.finish();

//     let mut group = c.benchmark_group("msm_scalar");
//     for size in MIN..(MAX+1) {
//         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::scalar_dot(&scalars_1[0..size], &scalars_2[0..size]));
//         });
//     }
//     group.finish();
// }

// fn msm_rand(c: &mut Criterion) {
//     let rng = &mut test_rng();
//     static MIN: usize = 2;
//     static MAX: usize = 20;
//     let scalars: Vec<PallasScalar> = vec![PallasScalar::rand(rng); consts::N];
//     let points: Vec<PallasPoint> = PublicParams::get_pp().Gs[0..consts::N].to_vec().into_iter().map(|g| g.into_group()).collect();

//     let mut group_new = c.benchmark_group("msm_rand_new");
//     for size in MIN..(MAX+1) {
//         group_new.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::new_point_dot(&scalars[0..size], &points[0..size]));
//         });
//     }
//     group_new.finish();

//     let mut group_old = c.benchmark_group("msm_rand_old");
//     for size in MIN..(MAX+1) {
//         group_old.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//             b.iter(|| group::old_point_dot(&scalars[0..size], points[0..size].to_vec()));
//         });
//     }
//     group_old.finish();
// }

// criterion_group! {
//     name = msm;
//     config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS)).warm_up_time(Duration::from_millis(100));
//     targets =
//         // msm_affine,
//         // msm_scalar,
//         // msm_rand,
//         msm_powers
// }

// -------------------- Main -------------------- //

criterion_main!(
    pcdl_commit,
    pcdl_open,
    pcdl_check,
    pcdl_succinct_check,
    acc_prover,
    acc_verifier,
    acc_decider,
    h,
);
