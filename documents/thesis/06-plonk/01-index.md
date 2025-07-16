# Surkål: The Ultra-\plonk-ish NARK protocol

We construct a NARK with support for recursive proofs, heavily inspired by
the Z-Cash's Halo2 and Mina's Kimchi proof systems. As with both of these
protocols, we instantiate them over a bulletproofs-style PCS and corresponding
accumulation scheme. We have taken liberties to try to simplify the protocol
at the cost of performance, but have taken an effort to preserve the following
properties:

1. The prover is bounded by $\Oc(n \log(n))$
2. The verifier is bounded by $\Oc(\lg(n))$
3. The proof size is bounded by $\Oc(\lg(n))$

Thus the optimizations omitted or simplified in this work only affects
constant-time factors.

The below sections will describe this protocol in detail.

<!--

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

-->
