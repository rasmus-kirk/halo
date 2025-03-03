#![allow(non_snake_case, dead_code)]

include!("src/consts.rs");

use anyhow::{bail, Result};
use std::{
    env,
    fs::{self, create_dir, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    if !cfg!(feature = "bootstrap") {
        assert!(N.is_power_of_two());
        assert!(G_BLOCKS_NO.is_power_of_two());

        let project_root_dir = env::var("CARGO_MANIFEST_DIR")?;
        let stored_params = PathBuf::from(project_root_dir)
            .join("precompute")
            .join("pp");
        if !stored_params.exists() {
            bail!("Error the precomputed parameters does not exist!")
        }

        let out_dir = env::var("OUT_DIR")?;
        let dest_path = PathBuf::from(out_dir).join("public-params");
        if !dest_path.exists() {
            create_dir(&dest_path)?;
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
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pp_paths.rs");
    println!("cargo:warning={:?}", dest_path);

    let mut content = String::from("pub(crate) const G_PATHS: [&[u8]; 64] = [\n");

    for k in 0..64 {
        if cfg!(feature = "bootstrap") {
            let line = format!("&[0]");
            content.push_str(&format!("    {},\n", line));
        } else {
            let padded_k = format!("{:02}", k); // Ensures two-digit formatting
            let line = format!("include_bytes!(concat!(env!(\"OUT_DIR\"), \"/public-params/gs-{}.bin\"))", padded_k);
            content.push_str(&format!("    {},\n", line));
        }
    }

    content.push_str("];\n");
    fs::write(dest_path, content)?;

    // Trigger rebuilds only if relevant files change
    println!("cargo:rerun-if-changed=precompute/pp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pp.rs");
    Ok(())
}
