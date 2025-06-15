{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      rustVersion = "1.87.0";
      rustFmtVersion = "2024-12-01";

      allSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            self.overlays.default
          ];
        };
      });

      # Helper to build a crate from the workspace
      buildCrate = { pkgs, crateName, src }:
        let
          manifest = (pkgs.lib.importTOML "${src}/${crateName}/Cargo.toml").package;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rustToolchain;
            rustc = pkgs.rustToolchain;
          };
        in
        rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = pkgs.lib.cleanSource src; # Use workspace root as source
          cargoLock = {
            lockFile = "${src}/Cargo.lock"; # Use workspace-level Cargo.lock
          };
          cargoBuildFlags = [ "-p" "${manifest.name}" ];
          doCheck = false; # Tests run in CI separately
        };
    in
    {
      overlays.default = final: prev: {
        rustToolchain = final.rust-bin.stable."${rustVersion}".default.override {
          extensions = [ "rust-analyzer" "rust-src" ];
        };
      };

      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          shellHook = ''export RUST_BACKTRACE=1'';
          buildInputs = [
            pkgs.rust-bin.nightly."${rustFmtVersion}".rustfmt
            pkgs.rustToolchain
            (pkgs.writeShellScriptBin "check-all" ''
              cd ${self}
              check-fmt &&
              echo "-------------------- Format ✅ --------------------" &&
              check-lint &&
              echo "-------------------- Lint ✅ --------------------" &&
              check-test &&
              echo "-------------------- Test ✅ --------------------"
            '')
            (pkgs.writeShellScriptBin "check-fmt" ''
              cargo fmt -- --check
            '')
            (pkgs.writeShellScriptBin "check-lint" ''
              cargo clippy --all-targets --all-features -- -D warnings
            '')
            (pkgs.writeShellScriptBin "check-test" ''
              cargo test
            '')
          ];
        };
      });

      packages = forAllSystems ({ pkgs }: {
        accumulation = buildCrate { inherit pkgs; crateName = "accumulation"; src = ./.; };
        plonk = buildCrate { inherit pkgs; crateName = "plonk"; src = ./.; };
        group = buildCrate { inherit pkgs; crateName = "group"; src = ./.; };
        poseidon = buildCrate { inherit pkgs; crateName = "poseidon"; src = ./.; };
        schnorr = buildCrate { inherit pkgs; crateName = "schnorr"; src = ./.; };
      });
    };
}
