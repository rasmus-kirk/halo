#![allow(non_snake_case)]

use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
    time::SystemTime,
};

use anyhow::Result;
use rayon::prelude::*;

pub mod consts;
pub mod group;
mod poseidon_consts;
pub mod pp;
pub mod wrappers;

pub type Fq = ark_pallas::Fq;
pub type Fp = ark_pallas::Fr;
pub use ark_ec;
pub use ark_ff;
pub use ark_pallas::PallasConfig;
pub use ark_poly;
pub use ark_std;
pub use ark_vesta::VestaConfig;
pub use group::*;
pub use pp::PublicParams;
pub use wrappers::PastaAffine;
pub use wrappers::PastaConfig;
pub use wrappers::PastaFE;
pub use wrappers::PastaFieldId;
pub use wrappers::PastaScalar;
pub use wrappers::WrappedPoint;

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

/// Generate public params
fn main() -> Result<()> {
    // Pallas
    let pp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".precompute")
        .join("pallas");
    if !pp_dir.exists() {
        println!("cargo:warning=creating {:?}", pp_dir);
        create_dir_all(&pp_dir)?;
    }
    gen_pp::write_pp::<ark_pallas::PallasConfig>(pp_dir)?;

    // Vesta
    let pp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".precompute")
        .join("vesta");
    if !pp_dir.exists() {
        println!("cargo:warning=creating {:?}", pp_dir);
        create_dir_all(&pp_dir)?;
    }
    gen_pp::write_pp::<ark_vesta::VestaConfig>(pp_dir)?;

    Ok(())
}
