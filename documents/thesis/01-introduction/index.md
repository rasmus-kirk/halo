\newpage

# Introduction

Incrementally Verifiable Computation (IVC) has seen increased practical usage,
notably by the Mina[@mina] blockchain to achieve a succinct blockchain. This
is enabled by increasingly efficient recursive proof systems, one of the
most used in practice is based on [@halo], which includes Halo2 by the
Electric Coin Company (to be used in Zcash) and Kimchi developed and used
by Mina. Both can be broken down into the following main components:

- **Plonk**: A general-purpose, potentially zero-knowledge, a SNARK.
- **$\PCDL$**: A Polynomial Commitment Scheme in the Discrete Log setting.
- **$\ASDL$**: An Accumulation Scheme in the Discrete Log setting.
- **Pasta**: A cycle of elliptic curves, Pallas and Vesta, collectively known as Pasta.

A previous project by one of the author's analyzed and implemented the
accumulation and polynomial commitment schemes. This project is focused on
the Plonk component, including all additions and amendments needed to acheive
an IVC-friendly SNARK. We also define an IVC-circuit for proving the validity
of a Chain of Signatures. Finally,A we implement the IVC circuit to analyze
the performance.Both this document and the implementation can be found in
the project's repository[@repo].

## Chain of Signatures

We aim to specify a recursive SNARK construction and instantiate it over
a chain of signatures. This would allow safe catchup and light clients on
BFT-style blockchains with committees, such as Concordium or Partesia both
of which are based on the HotStuff consensus. Taking Concordium as the main
example, they elect a committee once a day, that signs blocks. Concordium
is a proof of stake blockchain so the committee is elected according to the
size of thier staked tokens. They could create a parallel _IVC blockchain_,
one where each block contains:
$$B_i = \{ \s^{(pk)}_i, j_i = i, pk_i, ptr_i \in \Bb^{32}, \s^{(ptr)} \}$$
Let's break it down:

- $\s^{(pk)}_i$: A signature on the current public key ($pk_i$), signed by the previous public key $pk_{i-1}$.
- $j_i$: A block-id counter.
- $pk_i$: The public key of the current committee.
- $ptr_i$: A hash of the most recent block on the main blockchain.
- $\s^{(ptr)}_i$: A signature on $ptr$, signed by the current public key.

Traditionally, a blockchain would need the hash of the previous block to tie
together blocks, but we can omit that since we already have the signature
$\s_i^{(pk)}$, linking together $pk_{i-1}$ and $pk_i$, thus also linking
together $B_{i-1}$ and $B_i$. To verify this IVC blockchain, one would need
all blocks from the genesis block $B_0$, until the most recent block $B_n$.
Then they may verify the relation:
$$\text{Verify}_{pk_{i-1}}(\s^{(pk)}, pk_i) \land \text{Verify}_{pk_i}(\s^{(ptr)}, ptr_i) \land j_i \meq j_{i-1} + 1$$
Now we have a chain of signatures from the first genesis committee, all the
way to the final committee at block $n$. Assuming that the first committee
is honest, it should only sign the next honestly elected committee, which
by the security of the blockchain should also be majority-honest. That
committee will then also only sign the next honest committee. We can continue
this argument until reaching committee $n$, which contains a pointer to the
most recent block on the main blockchain. We can now trust that block, and
trust the blockchain, given that we trust the genesis committee, and that
the other committees have been honest.

This is of course not much of an improvement, to catch up on the main
blockchain you need to catch up on some other blockchain... The second
blockchain is however constructed to be SNARK-friendly. There is only a
single public key, representing the committee, the signature scheme can be
Schnorr's with poseidon hashes, which works really well SNARK constructions.
Importantly, this secondary blockchain can use Poseidon hashes, while the
main blockchain may prefer Sha3 for the slightly higher security, and the
secondary blockchain may use schnorr signatures, while the main blockchain
may prefer BLS signatures that support weighted signatures.

The main committee still needs to generate and sign using
the Schnorrr signature scheme, but for this they can use
[FROST](https://doi.org/10.1007/978-3-030-81652-0_2) or, if weighted signatures
are a hard requirement, [WSTS](https://stacks-sbtc.github.io/wsts/wsts.pdf). In
both schemes, signatures are checked using a single public key.

To turn this into a SNARK, we need a Recursive SNARK construction to model
this problem in terms of Incrementally Verifiable Computatiton. A Recursive
SNARK construction that can prove the relation mentioned earlier while also
verifying a previous proof. Then, when a blockchain node wants to catch up,
they can simply download the latest IVC-block. Verify it using the SNARK and
start participating in the main chain, with only negligible security overhead.

