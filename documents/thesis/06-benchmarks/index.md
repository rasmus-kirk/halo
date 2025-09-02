# Implementation and Benchmarks

We implemented the Plonk prover and verifier in Rust, using the previous
implementations of $\ASDL$ and $\PCDL$ as submodules. Both submodules still
needed pretty significant changes however. Neither submodule supported
generic curves, which would be needed for Plonk instatiated over a cycle of
curves. A new infrastructure for setup parameters had to be implemented,
that efficiently supported much higher degree polynomials, since the IVC
circuit is still quite large. The Fiat-Shamir hashing also needed to be
changed over to a poseidon sponge-based construction, rather than sha3,
which we implemented ourselves. The poseidon implementation was inspired by
Mina's work, so we used the same parameters (Since we also use the fields
from Pallas and Vesta in the hashing, just like Mina's Kimchi) and unit-tested
that the hash-behaviour of our implementation was identical to theirs.

After this, the Plonk arithmetizer, prover, and verifier could be implemented
and parametrized by a given curve, either Pallas or Vesta. The implemented
arithmetizer supports standard elliptic curve operations, fiat-shamir oriented
sponge-based hashing using poseidon, regular scalar operations and boolean
operations. This is implemented as a circuit (modelled as an acyclic directed
graph) with wrappers around it, effectively creating an embedded domain
specific language for writing circuits in rust. The frontend is so similar
that the code to implement the in-circuit verifiers for IVC, looks almost
identical to the rust/arkworks implementations. This made it much easier to
implement the relevant verifying circuits. Here's an overview of the rust
crates:

- Plonk (17116 LOC): The Plonk Prover/Verifier, including arithmetization
  and IVC-circuit. This also includes all the subcircuits needed for IVC
  (Poseidon, $\PCDLSuccinctCheck$, $\ASDLVerifier$, $\PlonkVerifier$).
- Accumulation (2940 LOC): Compromising of the PCS, $\PCDL$, and the
  accumulation scheme, $\ASDL$. This was already implemented.
- Group (4240 LOC): Code relating to evaluation domains, public parameters
  for $\PCDL$ (including caching them to binary files), and wrapper traits
  and struct for the cycle of curves.
- Poseidon (948 LOC): The Poseidon hash function, implemented in Rust,
  not in-circuit.
- Schnorr (169 LOC): A simple schnorr signature implementation, using poseidon
  for the message hash function.

As the purpose of the code is to prototype the ideas presented, and get some
benchmarks on the performance of the scheme, there are a few known soundness
bugs in the implementation (and probably more unknown ones!). Obviously,
the code should not be used in production. However, none of the soundness
bugs should affect performance to any significant degree. The benchmarks
ran multithreaded on a 20 thread Thinkpad P50:

- **IVC-Prover:** ~300 s
- **IVC-Verifier:** ~3 s

Which is not all that bad, if the use-case is to create a single proof for
a new blockchain committee once a day, ~5 minutes on a modern laptop is not
at all unreasonable. As for the verifier, it only takes ~3 s, which is
much faster than if traditional catch-up methods are used.
