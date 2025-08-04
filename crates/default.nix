{ pkgs, self, rust-overlay }:
let
  rustNightlyVersion = "2025-06-01";
  rust-bin = (rust-overlay.lib.mkRustBin {} pkgs);
  rustToolchain = rust-bin.nightly.${rustNightlyVersion}.default.override {
    extensions = [ "rust-analyzer" "rust-src" ];
  };
  # rustVersion = "1.88.0";
  # rustToolchain = rust-bin.stable.${rustVersion}.default.override {
  #   extensions = [ "rust-analyzer" "rust-src" ];
  # };
in {
  devShells = {
    default = pkgs.mkShell {
      RUST_BACKTRACE = "1";
      buildInputs = [
        rust-bin.nightly."${rustNightlyVersion}".rustfmt
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
