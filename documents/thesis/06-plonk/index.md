# Surkål: The Ultra-\plonk-ish NARK protocol

Our NARK protocol has the following:

1. add and mul gates, copy constraints, vanishing arguments (\plonk)
2. arbitrary fan-in and fan-out custom gates (Turbo-\plonk)
3. arbitrary lookup tables via \plookup (Ultra-\plonk)
4. pedersen polynomial commitment scheme (Halo2)
5. ergonomic multi type wire arithmetization (Surkål)
6. circuits over cycle of curves via pasta curves (Surkål)

The arithmetization scheme is agnostic of types of values, gates, lookup tables and trace, thus can potentially be extended for other variants of \plonk-ish protocols.

We will present our \plonk-ish protocol[^our-plonk] by constructing and arguing for the
individual arguments in the next sections.

[^our-plonk]: There are many variations of \plonk, our variant has the
feature-set of [Ultra-\plonk](https://zkjargon.github.io/definitions/plonkish_arithmetization.html#plonkish-variants-and-extensions),
is based on a Discrete Log PCS and omits the Mary Maller optimization from
the original paper.

## Vanishing Argument

<!-- TODO: Generally fine, but cleanup -->

The checks that the verifier makes in Plonk boils down to checking identities
of the following form:

$$\forall a \in S : f(a) \meq 0$$

For some polynomial $f(X) \in \Fb_{\leq d}$ and some set $S \subset \Fb$. The
subset, $S$, may be much smaller than $\Fb$ as is the case for Plonk where
$S = H$. Since we ultimately model the above check with challenge scalars,
using the entirety of $\Fb$ should lead to much better security. We therefore
end up with the following checks of the following form instead:

$$\forall \xi \in \Fb : F'(\xi) \meq 0$$

Where $S \subset \Fb$ and $F'$ is defined by combining $F$ with a challenge
scalar $\a$. Below we present the protocol that lets the verifier query
polynomial identities of the form $\forall a \in S : F(s) \meq 0$ using a
PCS. For a series of polynomials, $\{ F_1, F_2, \dots, F_k \} \in \Fb_{\leq
d}$, we have the following protocol:

\begin{algorithm}[H]
\caption*{
  \textbf{Single Polynomial Vanishing Argument Protocol:} Converts queries for polynomial
  identities ranging over all values $a \in H \subset S$ to a secure
  non-interactive protocol using polynomial commitments.
}
\textbf{Inputs} \\
  \Desc{$f: \Fb_{\leq d}[X]$}{The polynomial to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    Either the verifier accepts with $\top$ or rejects with $\bot$.
  }
\begin{algorithmic}[1]
  \State $P:$ The prover constructs $t(X)$:
    \Statex \algind $t(X) = \frac{f(X)}{z_S}, \quad z_S(X) = \prod_{s \in S}(X - s)$
  \State $P \to V:$ then commits to $f(X), t(X)$:
    \Statex \algind $C_f = \PCCommit(f(X), d, \bot), \quad C_t = \PCCommit(t(X), d, \bot)$
  \State $V \to P:$ The verifier sends challenge $\xi$ to the prover
  \State $P \to V:$ The prover sends $(f(\xi) = v_f, \pi_f, t(\xi) = v_t, \pi_f)$ to the verifier.
  \State $V:$ The verifier then checks:
    \Statex \algind $\PCCheck(C_f, d, \xi, v_f, \pi_f) \meq \top \; \land$
    \Statex \algind $\PCCheck(C_t, d, \xi, v_t, \pi_t) \meq \top$
  \end{algorithmic}
\end{algorithm}

**Correctness**

For any $\xi \in \Fb \setminus H$, the following holds:

$$
\begin{aligned}
p(X) &= f_i(\xi) - t(\xi) z_S(\xi) \\
     &= f_i(\xi) - \left( \frac{f_i(\xi)}{z_S(\xi)} \right) z_S(\xi) \\
     &= 0
\end{aligned}
$$
$\qed$

**Soundness**

Due to the factor theorem[^factor-theorem] $z_S(X)$ only divides $f(X)$ if and
only if all of $\o \in H : f(\o) = 0$. Then from this the Schwartz-Zippel
Lemma[^schwartz-zippel] states that evaluating a nonzero polynomial on
inputs chosen randomly from a large enough set is likely to find an input
that produces a nonzero output. Specifically it ensures that $Pr[P(\xi)]
\leq \frac{deg(P)}{|\Fb|}$. Clearly $\xi \in \Fb$ is a large enough set as
$|\Fb| \gg |H|$ and therefore $Pr[P(\xi) | P \neq 0]$ is negligible. Lastly,
the evaluation checked depends on the soundness of the underlying PCS scheme
used, but we assume that it has knowledge soundness and binding. From all
this, we conclude that the above vanishing argument is sound.

[^schwartz-zippel]: The wikipedia page for the Schwartz-Zippel Lemma: [https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma](https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma)
[^factor-theorem]: The wikipedia page for the Factor Theorem: [https://en.wikipedia.org/wiki/Factor_theorem](https://en.wikipedia.org/wiki/Factor_theorem)

**Extending to multiple $f$'s**

We can use a linear combination of $\a$ to generalize the Single Polynomial
Vanishing Argument:

\begin{algorithm}[H]
\caption*{
  \textbf{Vanishing Argument Protocol:} Converts queries for polynomial
  identities ranging over all values $a \in H \subset S$ to a secure
  non-interactive protocol using polynomial commitments.
}
\textbf{Inputs} \\
  \Desc{$\vec{f}: \Fb^k_{\leq d}[X]$}{The polynomial to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    Either the verifier accepts with $\top$ or rejects with $\bot$.
  }
\begin{algorithmic}[1]
  \State $P:$ The prover constructs $t(X)$:
    \Statex \algind $t(X) = \sum_{i \in [k]} \frac{\a^i f_i(X)}{Z_s}, \quad z_S(X) = \prod_{s \in S}(X - s)$
  \State $P \to V:$ then commits to $t(X)$ and each $f_i(X)$:
    \Statex \algind $C_{f_i} = \PCCommit(f_i(X), d, \bot), \quad C_t = \PCCommit(t(X), d, \bot)$
  \State $V \to P:$ The verifier sends challenge $\xi$ to the prover.
  \State $P \to V:$ The prover sends $(f_i(\xi) = v_{f_i}, \pi_{f_i}, t(\xi) = v_t, \pi_f)$ to the verifier.
  \State $V:$ The verifier then checks:
    \Statex \algind $\forall i \in [k] : \PCCheck(C_{f_i}, d, \xi, v_{f_i}, \pi_{f_i}) \meq \top \; \land$
    \Statex \algind $\PCCheck(C_t, d, \xi, v_t, \pi_t) \meq \top$
  \end{algorithmic}
\end{algorithm}

Note that for the Plonk protocol specifically, $S = H = \{ 1, \o, \o^2,
\dots, \o^{n-1} \}$ for the reason that the vanishing polynomial $z_S(X)$
then becomes $z_S(X) = X^n - 1$ because $\o$ is a root of unity of order
$n$. This is much more efficient to compute. The $\a$'s are used since we
need a linearly independent combination of $f$.

### Batched Evaluation Proofs

<!-- TODO: Soundness -->

If we have $m$ polynomials, $\vec{f}$, that all need to evaluate to
zero at the same challenge $\xi$, normally, we could construct $m$ opening
proofs, and verify these. We can, however, use the following technique to
only create a single opening proofs.

- The prover starts by sending commitments for each $f_i(X)$: $C_{f_i} = \PCCommit(f_i(X), d)$.
- The verifier sends the challenge $\xi$.
- The prover sends the evaluations of all $f_i$ ($v_{f_i} = f_i(\xi)$) as well as the single opening proof $\pi_w$ for the batched polynomial $w(X) = \sum_{i = 0}^k \a^i f_i(X)$.

Now, the verifier can construct the commitment ($C_w$) and evaluation ($v_w$)
to $w$ themselves:

$$
\begin{aligned}
  C_w &= \sum_{i = 0}^k \a^i C_{f_i} \\
  v_w &= \sum_{i = 0}^k \a^i v_{f_i}
\end{aligned}
$$

Finally, the verifier finally checks that $\PCCheck(C_w, d, \xi, v_w, \pi_w) \meq \top$

**Correctness:**

The correctness of the protocol is trivial

## Grand Product argument(s)

TODO

### Copy Constraints

TODO

### \plookup

TODO

## Full Protocol

\begin{algorithm}[H]
\caption*{
  \textbf{Surkål:} The Ultra-\plonk-ish NARK protocol.
}
\textbf{Inputs} \\
  \Desc{$f: W[\vec{t_{in}}] \to W[\vec{t_{out}}]$}{NP problem / program.} \\
  \Desc{$\vec{x} \in W[\vec{t_{in}}]$}{The possibly private input to the program $f$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State $(R: \Circuit, x: \PublicInputs, w : \Witness) = \mathrm{circuit} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$ 
  \State TODO vanishing argument call?
  \end{algorithmic}
\end{algorithm}