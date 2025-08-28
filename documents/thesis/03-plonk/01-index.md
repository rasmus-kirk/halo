# IVC-friendly Plonk Scheme

We construct a NARK with support for recursive proofs, heavily inspired by
the Z-Cash's Halo2 and Mina's Kimchi proof systems. As with both of these
protocols, we instantiate them over a bulletproofs-style PCS and corresponding
accumulation scheme. We have taken liberties to try to simplify the protocol
at the cost of performance, but have taken an effort to preserve the following
properties:

1. The prover is bounded by $\Oc(n \lg(n))$
2. The verifier time is linear ($\Oc(n)$)
3. The proof size is bounded by $\Oc(\lg(n))$

Thus the optimizations omitted or simplified in this work only affects
constant-time factors.

The below sections will describe this protocol in detail.

<!--

Our NARK protocol has the following:

| Protocol| Features | References |
|-|-----|--|
| \plonk        | Add and mul gates, copy constraints, vanishing arguments | @plonk |
| Turbo-\plonk  | Arbitrary fan-in and fan-out custom gates | |
| Ultra-\plonk  | Arbitrary lookup tables via \plookup, \plonkup | @plonkup |
| Halo2         | Pedersen polynomial commitment scheme and cycle of curves circuits | |

Our NARK protocol $\Surkal$ is a simplified variant of Halo2[^our-plonk] with circuits defined over the pasta curves.

In preprocessing, we feature an ergonomic multi type wire arithmetizer that is agnostic to types of values, gates, lookup tables and trace, thus a candidate for the preprocessor of other variants of \plonk-ish protocols.

[^our-plonk]: There are many variations of \plonk, our variant has the
feature-set of [Ultra-\plonk](https://zkjargon.github.io/definitions/plonkish_arithmetization.html#plonkish-variants-and-extensions),
is based on a Discrete Log PCS and omits the Mary Maller optimization from
the original paper.

-->
