#![allow(non_snake_case)]

use std::{
    env,
    fs::{self, create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
    time::SystemTime,
};

use acc::Accumulator;
use anyhow::Result;
use archive::std_config;
use pcdl::Instance;
use rayon::prelude::*;
use wrappers::{WrappedAccumulator, WrappedInstance};

pub mod acc;
mod archive;
pub mod consts;
pub mod group;
pub mod pcdl;
pub mod pedersen;
mod pp;
pub mod wrappers;

// -------------------- Benchmarking Functions --------------------

fn gen(n: usize) -> Result<(usize, WrappedInstance, WrappedAccumulator)> {
    let q = gen_q(n)?;
    let acc = gen_acc(q.clone())?;

    let wq: WrappedInstance = q.clone().into();
    let wacc: WrappedAccumulator = acc.into();
    Ok((n, wq, wacc))
}

fn gen_q(n: usize) -> Result<Instance> {
    let rng = &mut rand::thread_rng();

    let now = SystemTime::now();
    let q = Instance::rand(rng, n);
    let elapsed = now.elapsed()?;

    println!("[q {}]: Finished in {} s", n, elapsed.as_secs_f32());

    Ok(q)
}

fn gen_acc(q: Instance) -> Result<Accumulator> {
    let rng = &mut rand::thread_rng();
    let n = q.d + 1;

    let now = SystemTime::now();
    let acc = acc::prover(rng, &[q])?;
    let elapsed = now.elapsed()?;

    println!("[acc {}]: Finished acc in {} s", n, elapsed.as_secs_f32());

    Ok(acc)
}

fn bench(n: usize, q: WrappedInstance, acc: WrappedAccumulator) -> Result<()> {
    let q: Instance = q.into();
    test_succinct_check(q.clone(), n)?;
    test_acc_ver(q, acc.into())
}

fn test_succinct_check(q: Instance, n: usize) -> Result<()> {
    let now = SystemTime::now();
    let _ = q.succinct_check()?;
    let elapsed = now.elapsed()?;

    let n_f = n as f64;
    let t_f = elapsed.as_millis() as f64;
    let score = t_f.powi(2) / n_f;
    println!(
        "[sc - {}]: Finished in {} s (score: {})",
        n,
        elapsed.as_secs_f32(),
        score
    );

    Ok(())
}

fn test_acc_ver(q: Instance, acc: Accumulator) -> Result<()> {
    let n = acc.q.d + 1;

    let now = SystemTime::now();
    acc.verifier(&[q])?;
    let elapsed = now.elapsed()?;

    let n_f = n as f64;
    let t_f = elapsed.as_millis() as f64;
    let score = t_f.powi(2) / n_f;
    println!(
        "[acc - {}]: Finished in {} s (score: {})",
        n,
        elapsed.as_secs_f32(),
        score
    );

    Ok(())
}

// TODO: Benchmark pcdl::open, and acc::prover properly. Idea branch on provers/verifiers otherwise error.
/// A hacky script to do some rough benchmarks. We parallelize what we can to bring down the time as much as possible, this hurts the benchmarks, but they take so long that it's the only feasible option.
/// Run `cargo run -- /path/to/gen/dir gen` to generate the benchmarking accumulators and instances
/// Run `cargo run -- /path/to/gen/dir` to benchmark the cached generated values
fn main() -> Result<()> {
    // Handle cmd inputs
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No path specified!");
    let is_gen = match args.get(2) {
        Some(s) => s == "gen",
        None => false,
    };
    // Handle destination dir
    // TODO: Create a file for q and acc
    let dest_path = PathBuf::from(path).join("qs");
    if !dest_path.exists() {
        println!("cargo:warning=creating {:?}", dest_path);
        create_dir(&dest_path)?;
    }
    let q_path = dest_path.join("qs.bin");

    let min = 12;
    let max = min;
    let n = 2usize.pow(max);
    acc::setup(n)?;

    // Branch on `gen` argument
    if is_gen {
        let res: Result<Vec<(usize, WrappedInstance, WrappedAccumulator)>> = (min..max + 1)
            .into_par_iter()
            .map(|i| gen(2usize.pow(i)))
            .collect();
        let bytes = bincode::encode_to_vec(res?, std_config())?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(q_path)?;

        Write::write_all(&mut file, &bytes)?;
    } else {
        let bytes = fs::read(q_path)?;
        let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
            bincode::decode_from_slice(&bytes, std_config())?;

        val.into_par_iter()
            .try_for_each(|(n, q, acc)| bench(n, q, acc))?;
    }

    Ok(())
}
