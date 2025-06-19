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

This project is focused on the components of $\PCDL$ and $\ASDL$ from the
2020 paper _"Proof-Carrying Data from Accumulation Schemes"_[@pcd]. The
project examines the theoretical aspects of the scheme described in the
paper, and then implements this theory in practice with a corresponding Rust
implementation. Both the report and the implementation can be found in the
project's repository[@repo].

## Applications

- Blockchains
- Neural Nets
- [MixedNet]()


