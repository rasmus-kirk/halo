# PLONK

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
- `::coset` set of elements generated from the root of unity of the curve used as indices for wires in the circuit
- `::arithmetizer` arithmetizes a program
  - `::cache` cache of unique identifiers and computation used to minimize circuit size
  - `::wire` the variables users use to arithmetize their program
  - `::trace` computes the values of the wires prior to circuit construction
  - `::plookup` the table and other abstractions used in the lookup argument
- `::scheme` contains the arithmetization scheme constants and constraint structure
- `::circuit` $x R w$ where $x$ are public polynomials, $w$ are private, and $R$ is the arithmetized program as a relation
- `::protocol` the PLONK protocol
  - `::proof` generates the SNARK proof
  - `::verify` verifies the SNARK proof
  - `::pi` the SNARK proof
  - `::transcript` the hash scheme used (merlin)
- `::pcs` polynomial commitment scheme interface

## Constructing Circuits

To construct a circuit, you first need to choose an arithmetization scheme.

An arithmetization scheme can be parameterized by a set of operations offered by plookup `Op` and also the underlying curve used `P`.
```rust
Arithmetizer<Op, P>
```

The op sets are defined in `::plookup::opsets` as enums.

As an example, the following arithmetization scheme has binary xor and or lookups using the Pallas curve:
```rust
Arithmetizer<BinXorOr, PallasConfig>
```

For each curve, you will need to have a polynomial commitment scheme. You can define them using the `PCS` trait in `::pcs`. As of now the codebase only has implementation for `Pallas` curve as `PCSPallas`.

For the purpose of this example, we will let our arithmetization scheme be `MyArith` and our polynomial commitment scheme be `MyPCS`.

To construct a circuit, you will have the following structure in your code:
```rust
// construct circuit
let rng: &mut rand::thread_rng();
let [a, b] = MyArith::build();
let out = a.clone() * a + b;
let (x, w) = &MyArith::to_circuit::<_,_, MyPCS>(rng, &[1, 2], &[out], None)

// run protocol
let pi = protocol::prove::<_,_,MyPCS>(rng, x, w);
let sat = protocol::verify(false, x, pi);
assert!(sat);
```

`[a, b]` is the list of input wires for the circuit, you can destructure it immediately into wires as shown here. Otherwise, you can specify how many wires you want with the following:
```rust
let wires = MyArith::build::<CONST_NUMBER_OF_WIRES_IN_USIZE>();
```

`&[1, 2]` are the input values for `[a, b]` that will be used to compute the trace; concrete values of the wires.

`out` is the list of output wires for the circuit, in this case we only have one.
The output wire corresponds to the circuit `a × a + b`; output wire will have value `1 × 1 + 2 = 3`.

`(x, w)` is the circuit; as a relation of polynomials, that will be used by the plonk protocol.

## Modular Circuit Construction

Use functions to model higher order circuits

```rust
fn double<Op, P>(a: Wire<Op, P>) -> Wire<Op, P> {
  a + a
}

let out = double(a);
```

## Circuit Operations

For the list of operations available to you, look at `arithmetizer::wire`.
In the struct for wire we have
- `w.is_bit()` which enforces the wire to the range `{0,1}`
- `w.is_public()` which reveals the value of the wire to the public input polynomial
- `w.inv()` which returns the multiplicative inverse of the wire
- `w1.lookup(Op::Table, w2)` which returns `w3` where `(w1,w2,w3)` exists in the table
- TODO `w.predicate(Op::Table) = w1.exists(Op::Table, w2=0, w3=0)` which enforces that `(w,0,0)` exists in the table TODO NOTE: this should work like a constraint i.e. bit and public, not a lookup operation; i.e. have a `wires.is_predicate(wire)` check and then `self.constraints.push`

Of course there are sugar syntax operations as well, these are listed in `arithmetizer::wire::op_wire` and `arithmetizer::wire::op_scalar` where wires are operated on other wires and types that can `.into()` scalars respectively.
- `w1 + w2` and `w + s` addition
- `w1 - w2` and `w - s` subtraction
- `w1 * w2` and `w * s` multiplication
- `w1 / w2` and `w / s` division
- `-w` negation i.e. `w * -1`
- `!w` logical bit negation i.e. `1 - w`
- `w1 & w2` logical bit and i.e. multiplication

Scalars can also be private, i.e. it won't be exposed in $Q_c$ polynomial. This is done by wrapping the scalar in a `Witness` struct.
```rust
x + Witness::new(2) // 2 is private
x + 2 // 2 is public
```

There are also sugar syntax for specific lookup tables, for now there is only for `BinXorOr`
- `w1 | w2` logical bit or; `w1.lookup(BinXorOr::Or, w2)`
- `w1 ^ w2` logical bit xor; `w1.lookup(BinXorOr::Xor, w2)`

Thus, if you were to implement your own operation set with sugar syntax for it, you will implement the sugar here.

## Custom Operation Sets (Plookup Tables)

Define your custom operation set in `arithmetizer::plookup::opsets` as an enum.

An Operation Set is an enum of tables, e.g.

```rust
#[derive(Educe)]
#[educe(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum MyOpSet {
  #[educe(Default)]
  Table1,
  Table2,
  Table3,
}
```

You then populate the values of the table as follows:
```rust
impl PlookupOps for MyTable {
  fn to_table<P: SWCurveConfig>(self) -> Table<P> {
    match self {
      MyTable::Table1 => Table::new(vec![
          [Scalar::<P>::ZERO, Scalar::<P>::ZERO, Scalar::<P>::ZERO],
          [Scalar::<P>::ONE, Scalar::<P>::ONE, Scalar::<P>::ONE],
        ],
      ),
      ...
    }
  }
}
```

In this example, Table1 has entries (0,0,0) and (1,1,1) in the table.

You will also define if your table has the commutative property, i.e. if `(a, b, c)` and `(b, a, d)` are in the table, then c = d. This commutative property will be exploited by trace computation to use cached values.

```rust
impl PlookupOps for MyTable {
  fn is_commutative(&self) -> bool {
    match self {
      MyTable::Table1 => true,
      MyTable::Table2 => false,
      MyTable::Table3 => true,
    }
  }
}
```

You must also implement `EnumIter` for your enum so that the code can iterate over the tables.

```rust
impl EnumIter for MyTable {
    const COUNT: usize = 3;

    fn iter() -> impl Iterator<Item = MyTable> {
        [MyTable::Table1, MyTable::Table2, MyTable::Table3].into_iter()
    }

    fn id(self) -> usize {
        self as usize
    }
}
```

Since the enum has `#[repr(usize)]`, the `id` is the same as the enum value.

Lastly, you will need to implement Display for your enum so that when debugging wire AST, we know what to print.
```rust
impl Display for MyTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MyTable::Table1 => "T1",
            MyTable::Table2 => "T2",
            ...
        };
        write!(f, "{}", s)
    }
}
```

Now you are ready to use your custom operation set in the arithmetization scheme!
```rust
Arithmetizer<MyOpSet, P>
```
