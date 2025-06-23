# Surkål: The Ultra-\plonk-ish NARK protocol

Our NARK protocol has the following:

1. add and mul gates, copy constraints, vanishing arguments (\plonk)
2. arbitrary fan-in and fan-out custom gates (Turbo-\plonk)
3. arbitrary lookup tables via \plookup (Ultra-\plonk)
4. pedersen polynomial commitment scheme (Halo2)
5. ergonomic multi type wire arithmetization (Surkål)
6. circuits over cycle of curves via pasta curves (Surkål)

The arithmetization scheme is agnostic of types of values, gates, lookup tables and trace, thus can potentially be extended for other variants of \plonk-ish protocols.

We will present our protocol[^our-plonk] by constructing and arguing for the individual arguments in the next sections.

[^our-plonk]: There are many variations of \plonk, our variant has the
feature-set of [Ultra-\plonk](https://zkjargon.github.io/definitions/plonkish_arithmetization.html#plonkish-variants-and-extensions),
is based on a Discrete Log PCS and omits the Mary Maller optimization from
the original paper.
