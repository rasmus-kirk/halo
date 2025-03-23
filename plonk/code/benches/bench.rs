use std::time::{Duration, Instant};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

use ark_std::test_rng;
use plonk::protocol::{arithmetizer::Arithmetizer, plonk as plonker};

const WARMUP: Duration = Duration::from_millis(100);
const MIN: usize = 17; //5;
const MAX: usize = 20;

pub fn plonk_proof_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("plonk_proof_verify");
    let rng = &mut test_rng();

    let mut old_pis = Vec::new();
    let mut new_pis = Vec::new();

    let mut circuits = Vec::new();
    println!("|‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|");
    println!("| n  | circuit (s)  | new_P (s)    | old_P (s)    | new_V (s)    | old_V (s)    |");
    println!("|====|==============|==============|==============|==============|==============|");
    for size in MIN..MAX + 1 {
        let start_time = Instant::now();
        let output_wires = &Arithmetizer::synthesize::<_, 4>(rng, 2usize.pow(size as u32));
        let d = 2usize.pow(size as u32) - 1;
        let input_values = vec![3, 4, 5, 6];
        let circuit_time = start_time.elapsed().as_secs_f32();
        println!("A1");
        let ((x, w), _) = &Arithmetizer::to_circuit(rng, d, input_values, output_wires).unwrap();
        println!("A2");

        circuits.push((size, x.clone(), w.clone()));

        let start_time = Instant::now();
        let old_pi = plonker::proof(rng, &x, &w);
        let old_p_time = start_time.elapsed().as_secs_f32();
        println!("B");
        old_pis.push(old_pi.clone());

        let start_time = Instant::now();
        let _ = plonker::verify(&x, old_pi.clone());
        let old_v_time = start_time.elapsed().as_secs_f32();
        println!("C");

        let start_time = Instant::now();
        let new_pi = plonker::prove(rng, &x, &w);
        let new_p_time = start_time.elapsed().as_secs_f32();
        println!("D");
        new_pis.push(new_pi.clone());

        let start_time = Instant::now();
        let _ = plonker::verifier(&x, &new_pi);
        let new_v_time = start_time.elapsed().as_secs_f32();
        // println!("E");

        println!(
            "| {:02} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} |",
            size, circuit_time, new_p_time, old_p_time, new_v_time, old_v_time
        );
    }
    println!("|____|______________|______________|______________|______________|______________|");

    // for (i, x, w) in &circuits {
    //     group.warm_up_time(WARMUP).bench_with_input(
    //         BenchmarkId::new("prover", i),
    //         &i,
    //         |b, _| {
    //             b.iter(|| {
    //                 let pi = plonker::proof(rng, x, w);
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
    //                 plonker::verify(&x, pi.clone());
    //             })
    //         });
    // }

    for (i, x, w) in circuits.iter() {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::new("new_prover", i),
            &i,
            |b, _| {
                b.iter(|| {
                    plonker::prove(rng, &x, &w);
                })
            },
        );
    }
    for ((i, x, _), pi) in circuits.iter().zip(new_pis) {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::new("new_verifier", i),
            &i,
            |b, _| {
                b.iter(|| {
                    plonker::verifier(&x, &pi).unwrap();
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
