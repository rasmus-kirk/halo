# \Surkal: The Ultra-\plonk-ish NARK protocol

The features of \plonk-ish protocols are incrementally defined as follows:

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
