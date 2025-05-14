use std::{
    env,
    fs::File,
    time::{Duration, Instant},
};

use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10;
const SECONDS: u64 = 2;

use ark_std::test_rng;
use csv::Writer;
use itertools::Itertools;
use log::info;
use plonk::{arithmetizer::PallasBitArith, pcs::PCSPallas, protocol};

// const WARMUP: Duration = Duration::from_millis(100);
const MIN: usize = 4;
const MAX: usize = 20;

pub fn plonk_proof_verify(c: &mut Criterion) {
    env_logger::init();

    let group = c.benchmark_group("plonk_proof_verify");
    let rng = &mut test_rng();

    // let mut new_pis = Vec::new();

    // let mut circuits = Vec::new();
    const SERIES_COUNT: usize = 5;
    const SAMPLE_SIZE: usize = 10;
    let mut data: Vec<Vec<f32>> = Vec::with_capacity((MAX - MIN + 1) * SERIES_COUNT);
    for _ in 0..(MAX - MIN + 1) * SERIES_COUNT {
        data.push(Vec::with_capacity(SAMPLE_SIZE))
    }
    for i in 0..SAMPLE_SIZE {
        println!("Sample {} / {}", i + 1, SAMPLE_SIZE);
        println!(
            "|‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|"
        );
        println!(
            "| n  | gen_circ (s) | to_circ (s)  | Prover (s)   | Verifier (s) | Succ Ver (s) |"
        );
        println!(
            "|====|==============|==============|==============|==============|==============|"
        );
        for size in MIN..MAX + 1 {
            let d = 2usize.pow(size as u32) - 1;
            let input_values = &[3, 4, 5, 6];

            let off = (size - MIN) * SERIES_COUNT;
            info!("A1");
            let start_time = Instant::now();
            let output_wires = &PallasBitArith::synthesize::<4, _>(rng, d);
            let rand_circuit_time = start_time.elapsed().as_secs_f32();
            data[off].push(rand_circuit_time);
            info!("lens: {:?}, {:?}", output_wires.len(), output_wires[0].id());

            info!("A2");
            let start_time = Instant::now();
            let (x, w) = &PallasBitArith::to_circuit::<_, _, PCSPallas>(
                rng,
                input_values,
                output_wires,
                Some(d),
            )
            .unwrap();
            let to_circuit_time = start_time.elapsed().as_secs_f32();
            data[off + 1].push(to_circuit_time);

            info!("A3");
            // circuits.push((size, x.clone(), w.clone()));

            let start_time = Instant::now();
            let pi = protocol::prove::<_, _, PCSPallas>(rng, &x, &w);
            let new_p_time = start_time.elapsed().as_secs_f32();
            data[off + 2].push(new_p_time);
            info!("D");
            // new_pis.push(new_pi.clone());

            let new_pi = pi.clone();
            let start_time = Instant::now();
            protocol::verify(false, &x, new_pi).unwrap();
            let new_v_time = start_time.elapsed().as_secs_f32();
            data[off + 3].push(new_v_time);

            let start_time = Instant::now();
            protocol::verify(true, &x, pi).unwrap();
            let new_v_succ_time = start_time.elapsed().as_secs_f32();
            data[off + 4].push(new_v_succ_time);

            println!(
                "| {:02} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} |",
                size, rand_circuit_time, to_circuit_time, new_p_time, new_v_time, new_v_succ_time
            );
        }
        println!(
            "|____|______________|______________|______________|______________|______________|"
        );
    }

    let sqrt_size = (SAMPLE_SIZE as f32).sqrt();
    data = data
        .iter()
        .flat_map(|xs| {
            let n = SAMPLE_SIZE as f32;
            let mean = xs.iter().sum::<f32>() / n;
            let variance = xs.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / n;
            let error = variance.sqrt() / sqrt_size;
            [mean, error]
        })
        .chunks(SERIES_COUNT * 2)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    let _ = write_csv(&data);
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

fn write_csv(data: &Vec<Vec<f32>>) -> Result<()> {
    // Get the current directory (project root)
    let current_dir = env::current_dir()?;

    // Construct the full file path
    let path = current_dir.join("target/bench_data.csv");

    // Create the file and write to it
    let file = File::create(path)?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&[
        "n",
        "gen_circ",
        "gen_circ_err",
        "to_circ",
        "to_circ_err",
        "Prover",
        "Prover_err",
        "Verifier",
        "Verifier_err",
        "SuccVerifier",
        "SuccVerifier_err",
    ])?;
    for (i, row) in data.into_iter().enumerate() {
        let n = i + MIN;
        let mut row_with_n = vec![n as f32];
        row_with_n.extend(row);
        wtr.serialize(row_with_n)?;
    }

    wtr.flush()?;
    Ok(())
}

criterion_group! {
    name = plonks;
    config = Criterion::default().sample_size(SAMPLE_SIZE).measurement_time(Duration::from_secs(SECONDS));
    targets =
        plonk_proof_verify,
}

criterion_main!(plonks);
