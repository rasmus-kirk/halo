\newpage

# Introduction

Valiant originally described IVC in his 2008 paper[@valiant] in the following
way:

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
where transactions are processed, can then be verified with a SNARK. This
is commonly called a _succinct blockchain_.

This has notably been used by the Mina[@mina] blockchain to achieve a succinct
blockchain. This is enabled by increasingly efficient recursive proof systems,
one of the most used in practice is based on [@halo], which includes Halo2
by the Electric Coin Company (to be used in Zcash) and Kimchi developed and
used by Mina. Both can be broken down into the following main components:

- **Plonk**: A general-purpose, potentially zero-knowledge, SNARK.
- **$\PCDL$**: A Polynomial Commitment Scheme in the Discrete Log setting.
- **$\ASDL$**: An Accumulation Scheme for Evaluation Proof instances in the Discrete Log setting.
- **Pasta**: A cycle of elliptic curves, Pallas and Vesta, collectively known as Pasta.

A previous project by one of the authors of this thesis, analyzed and
implemented the accumulation and polynomial commitment schemes.

In this document we construct a recursive SNARK and use it to create a _Chain
of Signatures_.  We argue that this chain of signatures can be used in modern
BFT-based blockchains to acheive near-instant blockchain catch-up. We define
a modified Plonk based on $\PCDL$ and $\ASDL$ with custom gates. We also
define all custom gates needed to acheive an IVC-friendly SNARK. We then
define an IVC-circuit for proving the validity of a Chain of Signatures.
We also implement the IVC circuit for verifying a chain of signatures, to
benchmark the performance. Both this document and the implementation can be
found in the project's repository[@repo].
