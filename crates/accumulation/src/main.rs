#![allow(non_snake_case)]

use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    time::SystemTime,
};

use acc::Accumulator;
use anyhow::Result;
use ark_pallas::PallasConfig;
use ark_serialize::CanonicalSerialize;
use pcdl::Instance;

pub mod acc;
pub mod pcdl;
mod pedersen;

// -------------------- Benchmarking Functions --------------------

fn gen(n: usize) -> Result<(usize, Instance<PallasConfig>, Accumulator<PallasConfig>)> {
    let q = gen_q(n)?;
    let acc = gen_acc(q.clone())?;
    Ok((n, q, acc))
}

fn gen_q(n: usize) -> Result<Instance<PallasConfig>> {
    let rng = &mut rand::thread_rng();

    let now = SystemTime::now();
    let q = Instance::rand(rng, n);
    let elapsed = now.elapsed()?;

    q.check().unwrap();

    println!("[q {}]: Finished in {} s", n, elapsed.as_secs_f32());

    Ok(q)
}

fn gen_acc(q: Instance<PallasConfig>) -> Result<Accumulator<PallasConfig>> {
    let rng = &mut rand::thread_rng();
    let n = q.d + 1;

    let now = SystemTime::now();
    let acc = acc::prover(rng, &[q])?;
    let elapsed = now.elapsed()?;

    acc.clone().decider().unwrap();

    println!("[acc {}]: Finished acc in {} s", n, elapsed.as_secs_f32());

    Ok(acc)
}

/// Run `cargo run` to generate the benchmarking accumulators and instances
fn main() -> Result<()> {
    // Handle cmd inputs
    let path = env!("CARGO_MANIFEST_DIR");

    let min = 2;
    let max = 20;

    // Handle destination dir
    // TODO: Create a file for q and acc
    let qs_dir = PathBuf::from(path).join(".precompute");
    if !qs_dir.exists() {
        println!("cargo:warning=creating {:?}", qs_dir);
        create_dir_all(&qs_dir)?;
    }
    let qs_path = qs_dir.join("qs.bin");

    #[allow(clippy::type_complexity)]
    let res: Result<Vec<(usize, Instance<PallasConfig>, Accumulator<PallasConfig>)>> =
        (min..max + 1).map(|i| gen(2usize.pow(i))).collect();
    let qs = res?;

    let mut bytes = Vec::with_capacity(qs.compressed_size());
    qs.serialize_compressed(&mut bytes)?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(qs_path)?;

    Write::write_all(&mut file, &bytes)?;

    Ok(())
}
