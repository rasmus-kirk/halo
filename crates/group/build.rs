#![allow(non_snake_case, dead_code)]

include!("src/consts.rs");

use anyhow::{bail, Result};
use std::{
    env,
    fs::{self, create_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

static PUBLIC_PARAMS: &str = "public-params";

fn write_pp_files(curve: &str) -> Result<()> {
    assert!(N.is_power_of_two());
    assert!(G_BLOCKS_NO.is_power_of_two());

    let project_root_dir = env::var("CARGO_MANIFEST_DIR")?;
    let stored_params = PathBuf::from(project_root_dir)
        .join(".precompute")
        .join(curve);
    if !stored_params.exists() {
        bail!("Error the precomputed parameters does not exist!")
    }

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = PathBuf::from(out_dir).join("public-params").join(curve);
    create_dir_all(&dest_path)?;

    let sh_path = stored_params.join(Path::new("sh.bin"));
    let sh_out_path = dest_path.join(Path::new("sh.bin"));

    if !sh_path.exists() {
        bail!("{:?} was supposed to exist, but did not!", sh_path)
    } else {
        let bytes = fs::read(sh_path)?;

        // Write serialized data to file
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(sh_out_path)?;

        Write::write_all(&mut file, &bytes)?;
    }

    for i in 0..G_BLOCKS_NO {
        let index = i;
        let g_file = format!("gs-{:02}.bin", index);
        let g_path = stored_params.join(Path::new(&g_file));
        let g_out_path = dest_path.join(Path::new(&g_file));

        // Skip regeneration if the file already exists
        if !g_path.exists() {
            bail!("{:?} was supposed to exist, but did not!", g_path)
        } else {
            let bytes = fs::read(g_path)?;

            // Write serialized data to file
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(g_out_path)?;

            Write::write_all(&mut file, &bytes)?;
        }
    }
    Ok(())
}

fn write_pp_paths(curve: &str) -> Result<()> {
    let CURVE = curve.to_uppercase();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out_dir).join(curve);
    create_dir_all(&dest_dir)?;
    let dest_path = dest_dir.join("pp_paths.rs");

    let mut content = String::from(format!("const G_PATHS_{CURVE}: [&[u8]; 64] = [\n"));

    for k in 0..G_BLOCKS_NO {
        if cfg!(feature = "bootstrap") {
            content.push_str(&format!("    {},\n", "&[0]"));
        } else {
            let line = format!(
                "include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{PUBLIC_PARAMS}/{curve}/gs-{:02}.bin\"))",
                k
            );
            content.push_str(&format!("    {},\n", line));
        }
    }

    content.push_str("];\n");

    if cfg!(feature = "bootstrap") {
        content.push_str(&format!("const SH_PATH_{CURVE}: &[u8] = &[0];\n"));
    } else {
        content.push_str(&format!("const SH_PATH_{CURVE}: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{PUBLIC_PARAMS}/{curve}/sh.bin\"));\n"));
    }

    fs::write(dest_path, content)?;

    Ok(())
}

fn main() -> Result<()> {
    if !cfg!(feature = "bootstrap") {
        write_pp_files("pallas")?;
        write_pp_files("vesta")?;
    }

    write_pp_paths("pallas")?;
    write_pp_paths("vesta")?;

    // Trigger rebuilds only if relevant files change
    println!("cargo:rerun-if-changed=precompute/pallas/pp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pp.rs");

    Ok(())
}
