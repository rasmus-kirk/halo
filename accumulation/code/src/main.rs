#![allow(non_snake_case)]

use std::{
    env,
    fs::{self, create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    time::SystemTime,
};

use acc::Accumulator;
use anyhow::{bail, Result};
use ark_pallas::PallasConfig;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Valid};
use pcdl::Instance;
use rayon::prelude::*;

pub mod acc;
pub mod consts;
pub mod group;
pub mod pcdl;
pub mod pedersen;
mod pp;
pub mod wrappers;

// -------------------- Public Parameter Generation --------------------

mod gen_pp {
    use super::*;

    use ark_ec::PrimeGroup;
    use ark_ff::PrimeField;
    use bincode::config::standard;
    use consts::{G_BLOCKS_NO, G_BLOCKS_SIZE, N};
    use group::Scalar;
    use sha3::{Digest, Sha3_256};
    use std::path::Path;
    use wrappers::{PastaConfig, WrappedPoint};

    use crate::group::Point;

    // Function to generate a random generator for the Curve.
    // Since the order of the curve is prime, any point that is not the identity point is a generator.
    fn get_generator_hash<P: PastaConfig>(i: usize) -> WrappedPoint {
        let genesis_string =
            "To understand recursion, one must first understand recursion".as_bytes();

        // Hash `genesis_string` concatinated with `i`
        let mut hasher = Sha3_256::new();
        hasher.update(i.to_le_bytes());
        hasher.update(genesis_string);
        let hash_result = hasher.finalize();

        // Generate a uniformly sampled point from the uniformly sampled field element
        let point = Point::<P>::generator() * Scalar::<P>::from_le_bytes_mod_order(&hash_result);
        P::wrap_projective(point)
    }

    pub fn write_pp<P: PastaConfig>(out_dir: PathBuf) -> Result<()> {
        const CHUNKSIZE: usize = 4;

        assert!(N.is_power_of_two());
        assert!(G_BLOCKS_NO.is_power_of_two());
        assert!(CHUNKSIZE.is_power_of_two());

        if !out_dir.exists() {
            create_dir_all(&out_dir)?;
        }

        let now = SystemTime::now();

        // Create S, H file
        let sh_path = out_dir.join(Path::new("sh.bin"));
        if sh_path.exists() {
            println!("sh file already exists at {:?}", sh_path);
        } else {
            println!("Creating {:?}", sh_path);
            let s = get_generator_hash::<P>(0);
            let h = get_generator_hash::<P>(1);
            let bytes = bincode::encode_to_vec((s, h), standard())?;

            // Write serialized data to file
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(sh_path)?;

            Write::write_all(&mut file, &bytes)?;
        }

        // Create Gs files
        for i in 0..G_BLOCKS_NO {
            let g_file = format!("gs-{:02}.bin", i);
            let g_path = out_dir.join(Path::new(&g_file));

            // Skip regeneration if the file already exists
            if g_path.exists() {
                println!("{} already exists at {:?}", g_file, g_path);
            } else {
                println!("Creating {} at {:?}", g_file, g_path);
                let gs: Vec<WrappedPoint> = (0..G_BLOCKS_SIZE)
                    .into_par_iter()
                    .map(|k| get_generator_hash::<P>(i + k + 2))
                    .collect();
                let bytes = bincode::encode_to_vec(gs, standard())?;

                // Write serialized data to file
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(g_path.clone())?;

                Write::write_all(&mut file, &bytes)?;
            }
        }
        let t = now.elapsed()?;

        println!("Compiling Public Parameters took {} s", t.as_secs_f32());

        Ok(())
    }
}

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

fn bench(n: usize, q: Instance<PallasConfig>, acc: Accumulator<PallasConfig>) -> Result<()> {
    let q: Instance<PallasConfig> = q.into();
    test_succinct_check(q.clone(), n)?;
    test_acc_ver(q, acc.into())
}

fn test_succinct_check(q: Instance<PallasConfig>, n: usize) -> Result<()> {
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

fn test_acc_ver(q: Instance<PallasConfig>, acc: Accumulator<PallasConfig>) -> Result<()> {
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

    let min = 2;
    let max = 20;

    let curve = match args.get(2) {
        Some(s) if s == "pallas" || s == "vesta" => s,
        Some(_) => bail!("Valid arguments are \"pallas\" and \"vesta\""),
        None => bail!("Second argument is required"),
    };

    // Handle destination dir
    // TODO: Create a file for q and acc
    let qs_dir = PathBuf::from(path).join(curve).join("qs");
    if !qs_dir.exists() {
        println!("cargo:warning=creating {:?}", qs_dir);
        create_dir_all(&qs_dir)?;
    }
    let qs_path = qs_dir.join("qs.bin");

    match args.get(3) {
        Some(s) if s == "gen" => {
            if curve == "vesta" {
                bail!("qs cannot be created from vesta!")
            }
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
        }
        Some(s) if s == "bench" => {
            let bytes = fs::read(qs_path)?;
            let val = Vec::<(usize, Instance<PallasConfig>, Accumulator<PallasConfig>)>::deserialize_compressed(bytes.as_slice())?;

            val.into_par_iter()
                .try_for_each(|(n, q, acc)| bench(n, q, acc))?;
        }
        Some(s) if s == "pp" => {
            let pp_dir = PathBuf::from(path).join(curve).join("pp");
            if !pp_dir.exists() {
                println!("cargo:warning=creating {:?}", pp_dir);
                create_dir_all(&pp_dir)?;
            }

            if curve == "pallas" {
                gen_pp::write_pp::<ark_pallas::PallasConfig>(pp_dir)?
            } else {
                gen_pp::write_pp::<ark_vesta::VestaConfig>(pp_dir)?
            }
        }
        Some(s) => bail!("Invalid second argument {}", s),
        None => bail!("No second argument given"),
    };

    Ok(())
}
