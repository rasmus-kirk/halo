\newpage

# Introduction

Valiant originally described IVC - Incrementally Verifiable Computation -
in his 2008 paper[@valiant]:

\begin{quote}
\color{GbGrey}

\textit{Suppose humanity needs to conduct a very long computation which will span
superpolynomially many generations. Each generation runs the computation
until their deaths when they pass on the computational configuration to the
next generation. This computation is so important that they also pass on a
proof that the current configuration is correct, for fear that the following
generations, without such a guarantee, might abandon the project. Can this
be done?}

\end{quote}

If a computation runs for hundreds of years and ultimately outputs 42, how can
we check its correctness without re-executing the entire process? In order
to do this, the verification of the final output of the computation must be
much smaller than simply running the computation again. Valiant creates the
concept of IVC and argues that it can be used to achieve the above goal.

Recently, IVC has seen renewed interest with cryptocurrencies, as this concept
lends itself well to the structure of blockchains. It allows a blockchain node
to omit all previous transaction history in favour of only a single state,
for example, containing all current account balances. Each state-transition,
where transactions are processed, can then be verified with a _SNARK_,
Succinct Non-interactive Argument of Knowledge, very small proofs of large
statements. This type of blockchain is commonly called a _succinct blockchain_.

A softer approach is to use IVC to prove that the current block corresponds
to the most recent block in a chain originating from the genesis block. This
allows for near-instant blockchain catch-up for light nodes. A light node is a
blockchain node with lower security standards than a full node, but it allows
the node to dedicate fewer computational resources and hard drive space. This
light node still need to catch-up to the current block in the blockchain,
which involves downloading and verifying all previous blocks. This yields
less security than in a succinct blockchain, but has the advantage of being
much simpler than proving the validity of a the transactions in a block
in-circuit. It also requires minimal changes to an existing blockchain.

IVC has notably been used by the Mina[@mina] succinct blockchain
blockchain. This is enabled by increasingly efficient recursive proof systems,
one of the most used in practice is based on Halo[@halo], which includes Halo2
by the Electric Coin Company (to be used in Zcash) and Kimchi developed and
used by Mina. Both can be broken down into the following main components:

- **Plonk**: A general-purpose, potentially zero-knowledge, SNARK.
- **$\PCDL$**: A Polynomial Commitment Scheme in the Discrete Log setting.
- **$\ASDL$**: An Accumulation Scheme for Evaluation Proof instances in the Discrete Log setting.
- **Pasta**: A cycle of elliptic curves, Pallas and Vesta, collectively known as Pasta.

A previous project[@halo-accumulation] by one of the authors of this thesis,
analyzed and implemented the accumulation and polynomial commitment schemes.

We aim to create a simplified recursive SNARK based on Halo and use it to
create a _chain of signatures_. We argue that this chain of signatures can be
used in certain modern blockchains to acheive near-instant blockchain catch-up.

In section 3 we define a modified Plonk based on $\PCDL$ and $\ASDL$ with all
custom gates needed to acheive an IVC-friendly SNARK. In section 4 define an
IVC-circuit for proving the validity of a chain of signatures. In section
5 we formally define the arithmetization pipeline needed for the defined
Plonk Scheme. In section 6 we discuss the implementation of the IVC circuit
for verifying a chain of signatures, and benchmark the performance. Both
this document and the implementation can be found in the project's
repository[@repo].
