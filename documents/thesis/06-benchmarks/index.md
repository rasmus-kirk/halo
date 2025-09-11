# Implementation and Benchmarks

We implemented the Plonk prover and verifier in Rust, using the previous
implementations[@halo-accumulation] of $\ASDL$ and $\PCDL$ as submodules. Both
submodules still needed pretty significant changes however. Neither submodule
supported generic curves, which would be needed for Plonk instatiated
over a cycle of curves. A new infrastructure for setup parameters had to
be implemented, that efficiently supported much higher degree polynomials,
since the IVC circuit is still quite large. The Fiat-Shamir hashing also
needed to be changed over to a Poseidon sponge-based construction, rather
than Sha3, which we implemented ourselves. The Poseidon implementation was
inspired by Mina's work, so we used the same parameters (Since we also use
the fields from Pallas and Vesta in the hashing, just like Mina's Kimchi)
and unit-tested that the hash-behaviour of our implementation was identical
to theirs.

After this, the Plonk arithmetizer, prover, and verifier could be implemented
and parametrized by a given curve, either Pallas or Vesta. The implemented
arithmetizer supports standard elliptic curve operations, Fiat-Shamir oriented
sponge-based hashing using Poseidon, regular scalar operations and Boolean
operations. This is implemented as a circuit (modelled as a directed acyclic
graph) with wrappers around it, effectively creating an embedded domain
specific language for writing circuits in Rust. The frontend is so similar
that the code to implement the in-circuit verifiers for IVC looks almost
identical to the Rust/Arkworks implementations. This made it much easier to
implement the relevant verifying circuits. Here's an overview of the Rust
crates[^loc]:

- Plonk: The Plonk Prover/Verifier, including arithmetization
  and IVC-circuit. This also includes all the subcircuits needed for IVC
  (Poseidon, $\PCDLSuccinctCheck$, $\ASDLVerifier$, $\PlonkVerifier$).
- Accumulation: Compromising of the PCS, $\PCDL$, and the
  accumulation scheme, $\ASDL$. This was already implemented.
- Group: Code relating to evaluation domains, public parameters
  for $\PCDL$ (including caching them to binary files), and wrapper traits
  and struct for the cycle of curves.
- Poseidon: The Poseidon hash function, implemented in Rust,
  not in-circuit.
- Schnorr: A simple Schnorr signature implementation, using Poseidon
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
at all unreasonable, especially considering that further optimization should
be possible. As for the verifier, it only takes ~3 s, which is much faster
than if traditional catch-up methods are used.

[^loc]: The Plonk crate is 17,116 LOC, the accumulation crate is 2,940 LOC,
the group crate is 4,240 LOC, the Poseidon crate is 948 LOC and the Schnorr
crate is 169 LOC
