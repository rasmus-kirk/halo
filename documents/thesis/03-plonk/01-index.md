# IVC-friendly Plonk Scheme

We construct a NARK with support for recursive proofs, heavily inspired by
the Z-Cash's Halo2 and Mina's Kimchi proof systems. As with both of these
protocols, we instantiate them over a Bulletproofs-style PCS and corresponding
accumulation scheme[@halo-accumulation]. We have taken liberties to try to
simplify the protocol at the cost of performance, but have taken an effort
to ensure we only affect constant-time factors. Meaning that the following
still hold for our protocol:

1. The prover is bounded by $\Oc(n \lg(n))$
2. The verifier time is linear ($\Oc(n)$)
3. The proof size is bounded by $\Oc(\lg(n))$

The below sections will describe this protocol in detail.
