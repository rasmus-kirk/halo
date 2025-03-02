#![allow(non_snake_case, dead_code)]

include!("src/archive.rs");

use anyhow::Result;
use ark_ec::PrimeGroup;
use ark_ff::PrimeField;
use rayon::prelude::*;
use sha3::{Digest, Sha3_256};
use std::{
    env,
    fs::{create_dir, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

// Function to generate a random generator for the Pallas Curve.
// Since the order of the curve is prime, any point that is not the identity point is a generator.
fn get_generator_hash(i: usize) -> WrappedPoint {
    let genesis_string = "To understand recursion, one must first understand recursion".as_bytes();

    // Hash `genesis_string` concatinated with `i`
    let mut hasher = Sha3_256::new();
    hasher.update(i.to_le_bytes());
    hasher.update(genesis_string);
    let hash_result = hasher.finalize();

    // Generate a uniformly sampled point from the uniformly sampled field element
    let point = Projective::generator() * ark_pallas::Fr::from_le_bytes_mod_order(&hash_result);
    point.into_affine().into()
}

fn print_sh() -> (String, String) {
    let s: Projective = get_generator_hash(0).into();
    let h: Projective = get_generator_hash(1).into();
    let f = |s, p: Projective| {
        format!(
            "const {}: PallasPoint = mk_proj({:?}, {:?}, {:?})",
            s, p.x.0 .0, p.y.0 .0, p.z.0 .0
        )
    };

    (f("S", s), f("H", h))
}

fn handle_g(i: usize, chunksize: usize, filepath: String) -> Result<()> {
    let f = Path::new(&filepath);
    for j in 0..chunksize {
        let index = j + i * chunksize;
        let g_file = format!("gs-{}.bin", index);
        let g_path = f.join(Path::new(&g_file));

        // Skip regeneration if the file already exists
        if !g_path.exists() {
            let gs: Vec<WrappedPoint> = (0..G_BLOCKS_SIZE)
                .into_par_iter()
                .map(|k| get_generator_hash(index + k + 2))
                .collect();
            let bytes = bincode::encode_to_vec(gs, std_config())?;

            // Write serialized data to file
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(g_path.clone())?;

            Write::write_all(&mut file, &bytes)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    const CHUNKSIZE: usize = 4;
    assert!(N.is_power_of_two());
    assert!(G_BLOCKS_NO.is_power_of_two());

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = PathBuf::from(out_dir).join("public-params");

    // Run the async tasks and wait for them to complete
    let now = SystemTime::now();

    if !dest_path.exists() {
        create_dir(&dest_path)?;
    }
    (0..G_BLOCKS_NO / CHUNKSIZE)
        .into_par_iter()
        .try_for_each(|i| handle_g(i, CHUNKSIZE, dest_path.to_str().unwrap().to_string()))?;

    let t = now.elapsed()?;
    let time = format!("Compiling Public Parameters took {} s", t.as_secs_f32());
    println!("cargo:warning={}", time);

    // Uncommenting these will print the S and H constants
    //let (s, h) = print_sh();
    //println!("cargo:warning={}", s);
    //println!("cargo:warning={}", h);

    // Trigger rebuilds only if relevant files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pp.rs");
    Ok(())
}
