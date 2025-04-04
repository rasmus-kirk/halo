use std::time::{Duration, Instant};

use ark_pallas::PallasConfig;
use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

use ark_std::test_rng;
use log::info;
use plonk::{arithmetizer::PallasEmptyArith, protocol};

// const WARMUP: Duration = Duration::from_millis(100);
const MIN: usize = 4;
const MAX: usize = 20;

pub fn plonk_proof_verify(c: &mut Criterion) {
    env_logger::init();

    let group = c.benchmark_group("plonk_proof_verify");
    let rng = &mut test_rng();

    // let mut new_pis = Vec::new();

    let mut circuits = Vec::new();
    println!("|‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|");
    println!("| n  | gen_circ (s) | to_circ (s)  | Prover (s)   | Verifier (s) |");
    println!("|====|==============|==============|==============|==============|");
    for size in MIN..MAX + 1 {
        let d = 2usize.pow(size as u32) - 1;
        let input_values = vec![3, 4, 5, 6];

        info!("A1");
        let start_time = Instant::now();
        let output_wires = &PallasEmptyArith::synthesize::<4, _>(rng, 2usize.pow(size as u32) - 2);
        let rand_circuit_time = start_time.elapsed().as_secs_f32();
        info!("lens: {:?}, {:?}", output_wires.len(), output_wires[0].id());

        info!("A2");
        let start_time = Instant::now();
        let (x, w) =
            &PallasEmptyArith::to_circuit(rng, input_values, output_wires, Some(d)).unwrap();
        let to_circuit_time = start_time.elapsed().as_secs_f32();

        info!("A3");
        circuits.push((size, x.clone(), w.clone()));

        let start_time = Instant::now();
        let new_pi = protocol::prove(rng, &x, &w);
        let new_p_time = start_time.elapsed().as_secs_f32();
        info!("D");
        // new_pis.push(new_pi.clone());

        let start_time = Instant::now();
        protocol::verify(&x, new_pi).unwrap();
        let new_v_time = start_time.elapsed().as_secs_f32();

        println!(
            "| {:02} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} |",
            size, rand_circuit_time, to_circuit_time, new_p_time, new_v_time
        );
    }
    println!("|____|______________|______________|______________|______________|");
    // for (i, x, w) in &circuits {
    //     group.warm_up_time(WARMUP).bench_with_input(
    //         BenchmarkId::new("prover", i),
    //         &i,
    //         |b, _| {
    //             b.iter(|| {
    //                 let pi = protocol::proof(rng, x, w);
    //                 pis.push(pi.clone());
    //             })
    //         },
    //     );
    // }

    // for ((i, x, _), pi) in circuits.iter().zip(pis) {
    //     group
    //         .warm_up_time(WARMUP)
    //         .bench_with_input(BenchmarkId::new("verifier", i), &i, |b, _| {
    //             b.iter(|| {
    //                 protocol::verify(&x, pi.clone());
    //             })
    //         });
    // }

    // for (i, x, w) in circuits.iter() {
    //     group.warm_up_time(WARMUP).bench_with_input(
    //         BenchmarkId::new("new_prover", i),
    //         &i,
    //         |b, _| {
    //             b.iter(|| {
    //                 protocol::prove_w_lu(rng, &x, &w);
    //             })
    //         },
    //     );
    // }
    // for ((i, x, _), pi) in circuits.iter().zip(new_pis) {
    //     group.warm_up_time(WARMUP).bench_with_input(
    //         BenchmarkId::new("new_verifier", i),
    //         &i,
    //         |b, _| {
    //             b.iter(|| {
    //                 protocol::verify_lu_with_w(&x, pi.clone()).unwrap();
    //             })
    //         },
    //     );
    // }
    group.finish();
}

criterion_group! {
    name = plonks;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        plonk_proof_verify,
}

criterion_main!(plonks);
