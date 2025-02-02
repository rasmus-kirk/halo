# Report:

Building, running, etc...

## Rust

### Testing

```bash
  cargo test
```

### Building

```bash
  cargo build
```

### Running

```bash
  cargo run
```

## Nix

### Testing

```bash
  nix develop -c cargo test
```

### Building

```bash
  nix build
```

### Running

```bash
  nix run
```

# User Guide

```rust
use plonk::protocol::arithmetizer::Arithmetizer;
use plonk::protocol::plonk;

pub fn main() {
  let rng = &mut rand::thread_rng();

  // 1. Arithmetize your program
  let [x, y] = &Arithmetizer::build();
  let out = &[3 * (x * x) + (y * 5) - 47];

  // 2. Create a circuit from the arithmetized program
  let ((x, w), _) = &Arithmetizer::to_circuit(rng, vec![1, 2], out).unwrap();

  // 3. Run the PLONK protocol
  let pi = plonk::proof(rng, x, w);
  let sat = plonk::verify(x, pi);
  assert!(sat);
}
```

# Developer Guide

The plonk library has the following structure:
- `plonk::curve` contains structs that wrap the underlying curve
  - `::point` point of the elliptic curve
  - `::scalar` scalar of the field for the curve
  - `::poly` polynomial over the scalar field
- `plonk::protocol`
  - `::arithmetizer` arithmetizes a program
    - `::cache` cache of unique identifiers and computation used to minimize circuit size
    - `::wire` the variables users use to arithmetize their program
    - `::trace` computes the values of the wires prior to circuit construction
  - `::scheme` contains the arithmetization scheme constants and constraint structure
  - `::circuit` $x R w$ where $x$ are public polynomials, $w$ are private, and $R$ is the arithmetized program as a relation
  - `::coset` set of elements generated from the root of unity of the curve used as indices for wires in the circuit
  - `::plonk` the PLONK protocol, contains `proof, verify`
