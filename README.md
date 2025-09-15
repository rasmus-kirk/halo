## Rust Implementation  

### Developement Environment using Nix

This project has nix support, as such, navigating to `/crates` and typing
`nix develop`, will install the necessary rust version, with the correct
formatter and rust-analyzer included.

### Unit Tests

Unit tests can be run with `cargo test` in the `/code` directory.

### Benchmark

To run the benchmark, go into the `/crates/plonk` and run `cargo run`.

## Report  

The full report, *["Investigating IVC with Accumulation
Schemes"](https://halo.rasmuskirk.com/thesis/thesis.pdf)*, is included in this
repository and provides a detailed explanation of the theory, constructions,
and benchmarks.

## License  

This project is licensed under the MIT License. See the `LICENSE` file for details.
