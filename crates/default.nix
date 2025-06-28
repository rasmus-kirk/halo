{ pkgs, self, rust-overlay }:
let
  rustVersion = "1.87.0";
  rustFmtVersion = "2024-12-01";
  rust-bin = (rust-overlay.lib.mkRustBin {} pkgs);
  rustToolchain = rust-bin.stable.${rustVersion}.default.override {
    extensions = [ "rust-analyzer" "rust-src" ];
  };
in {
  devShells = {
    default = pkgs.mkShell {
      RUST_BACKTRACE = "1";
      buildInputs = [
        rust-bin.nightly."${rustFmtVersion}".rustfmt
        rustToolchain
        (pkgs.writeShellScriptBin "check-all" ''
          check-fmt &&
          echo "-------------------- Format ✅ --------------------" &&
          check-lint &&
          echo "-------------------- Lint ✅ --------------------" &&
          check-test &&
          echo "-------------------- Test ✅ --------------------"
        '')
        (pkgs.writeShellScriptBin "check-fmt" ''
          cargo fmt --check --all
        '')
        (pkgs.writeShellScriptBin "check-lint" ''
          cargo clippy --all-targets --all-features --all -- -D warnings
        '')
        (pkgs.writeShellScriptBin "check-test" ''
          cargo test --all
        '')
      ];
    };
  };
  packages = let
    rustPlatform = pkgs.makeRustPlatform {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
    buildCrate = crateName: let
      manifest = (pkgs.lib.importTOML ./${crateName}/Cargo.toml).package;
    in rustPlatform.buildRustPackage {
      pname = manifest.name;
      version = manifest.version;
      src = pkgs.lib.cleanSource ./.;
      cargoLock.lockFile = ./Cargo.lock;
      cargoBuildFlags = [ "-p" manifest.name ];
      doCheck = false;
    };
  in {
    accumulation = buildCrate "accumulation";
    plonk = buildCrate "plonk";
    group = buildCrate "group";
    poseidon = buildCrate "poseidon";
    schnorr = buildCrate "schnorr";
  };
}
