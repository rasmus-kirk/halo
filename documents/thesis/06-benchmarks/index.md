# Implementation and Benchmarks

We implemented the Plonk prover and verifier in Rust, using the previous
implementations[@halo-accumulation] of $\ASDL$ and $\PCDL$ as submodules. Both
submodules still needed pretty significant changes however. Neither submodule
supported generic curves, which would be needed for Plonk instantiated
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
bugs should affect performance to any significant degree. 

Before presenting the benchmarks, we first briefly discuss what performance
is needed for our IVC approach to be preferred. If Concordium created a light
node implementation, there are several available ways to catch-up to the
current block and trust that the block is correct:

- Catch up as a full node would, validating each block, which would take days.
- Simply trust a full node, which is very insecure.
- Ask a lot of full nodes and if a quorum agrees that a given block is the
  current one, then use it. This is more secure, but requires a lot of network
  traffic from previously unconnected peers.
- Verify only the blocks where the committee changes, which is one block per day.

The last option is definitely the preferred one, and thus our chain of
signatures SNARK should compete with that solution. Here we would need to
verify 2 signatures per day (the other signature arises from Concordium's
Finality layer) and some hashes. For this comparison we just focus on the
signatures. We can now present the benchmarks which ran on a 20 thread
Thinkpad P50:

- **IVC-Prover:** Parallel: ~300 s. Single Threaded: ~900 s.
- **IVC-Verifier:** Parallel: ~3 s. Single Threaded: ~9 s.
- **Naive Signature Verification:** Parallel: ~1300 signatures per
  second. Single Threaded: ~310 signatures per second.

Assuming that the only bottleneck in this process is processing power would
mean that for a multithreaded verifier, it would take $1300 \cdot 3 \; / \;
2 = 1950$ days before the IVC solution was faster. Which is not ideal given
the complexity of the IVC construction, but it's not far from being viable.
If we instead look at the size of each "proof" involved, starting with the
IVC proof:

- Signature: 1 point, 1 scalar.
- EvalProof: $1 + 2 \lg(n) = 1 + 16 = 17$ points, 1 scalar.
- PlonkProof: 2 EvalProofs, 33 commitments (points), 74 polynomial evaluations
  (scalars). 67 points, 76 scalars.
- Accumulator: 1 EvalProof, 1 point, 3 scalars. 18 points, 4 scalars.
- IVC-Proof: 2 Accumulators, 2 PlonkProofs, 2 signatures, 2 public-keys
  (2 points), 2 $j$ scalars. 174 points, 164 scalars,

Modelling each scalar as 256 bits and each point as 256 bits (255 bit field
element and 1 additional sign bit), gives us ~10 kB for a single IVC
proof. Comparing to just verifying the signatures, after 87 days the IVC
proof will be smaller than the 174 signatures needed to verify the same
claim. Obviously, if the committee changes more ofter (say once an hour),
the IVC approach will much more quickly become economical.

If the use-case is to create a single proof for a new blockchain committee
once a day, ~5 minutes on a modern laptop is not at all unreasonable. As
for the verifier, it takes ~3 s, which isn't ideal, but will be better than
the naive solution after 1950 days. The proof size is okay comparatively
though, as the IVC proof will be smaller than the naive solution after only
87 days. These results are pretty promising, especially considering that
further optimization should be possible.

[^loc]: The Plonk crate is 17,116 LOC, the accumulation crate is 2,940 LOC,
the group crate is 4,240 LOC, the Poseidon crate is 948 LOC and the Schnorr
crate is 169 LOC
