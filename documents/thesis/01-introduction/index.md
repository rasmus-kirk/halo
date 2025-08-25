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

This project is focused on the components of Plonk and Pasta. Both the report
and the implementation can be found in the project's repository[@repo].
