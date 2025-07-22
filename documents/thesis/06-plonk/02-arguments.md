## Arguments

### Vanishing Argument

<!-- TODO: Generally fine, but cleanup -->

The checks that the verifier makes in Plonk boils down to checking identities
of the following form:

$$\forall a \in S : f(a) \meq 0$$

For some polynomial $f(X) \in \Fb_{\leq d}$ and some set $S \subset \Fb$. The
subset, $S$, may be much smaller than $\Fb$ as is the case for Plonk where
$S$ is set to be the set of roots of unity ($S = H = \{ \o^1, \o^2, \dots,
\o^n \}$). Since we ultimately model the above check with challenge scalars,
using just $S$, might not be sound. For this purpose construct the **Single
Polynomial Vanishing Argument Protocol**:

\begin{algorithm}[H]
\caption*{
  \textbf{Single Polynomial Vanishing Argument Protocol:} Checks queries
  of the form $\forall a \in S : f(a) \meq 0$, but with scalars from $\Fb
  \supseteq S$.
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
  \State $P \to V:$ The prover sends $(f(\xi) = v_f, \pi_f, t(\xi) = v_t, \pi_t)$ to the verifier.
  \State $V:$ The verifier then checks:
    \Statex \algind $v_f \meq v_t \cdot z_S(\xi)$
    \Statex \algind $\PCCheck(C_f, d, \xi, v_f, \pi_f) \meq \top \; \land$
    \Statex \algind $\PCCheck(C_t, d, \xi, v_t, \pi_t) \meq \top$
  \end{algorithmic}
\end{algorithm}

**Correctness**

Define $p(X) = f_i(\xi) - t(\xi) z_S(\xi)$. For any $\xi \in \Fb \setminus
S$, the following holds:

$$
\begin{aligned}
p(\xi) &= f_i(\xi) - t(\xi) z_S(\xi) \\
       &= f_i(\xi) - \left( \frac{f_i(\xi)}{z_S(\xi)} \right) z_S(\xi) \\
       &= 0
\end{aligned}
$$
$\qed$

**Soundness**

<!-- TODO(rasmus): The soundness argument doesn't limit the degree of p(X)! -->

Due to the factor theorem[^factor-theorem] $z_S(X)$ only divides $f(X)$ if and
only if all of $s \in S : f(s) = 0$. Then from this the Schwartz-Zippel
Lemma[^schwartz-zippel] states that evaluating a nonzero polynomial on
inputs chosen randomly from a large enough set is likely to find an input
that produces a nonzero output. Specifically, it ensures that $Pr[p(\xi) = 0]
\leq \frac{deg(p(X))}{|\Fb|}$. Clearly $\xi \in \Fb$ is a large enough set as
$|\Fb| \gg |S|$ and therefore $Pr[p(\xi) = 0 | P \neq 0]$ is negligible. Lastly,
the evaluation checked depends on the soundness of the underlying PCS scheme
used, but we assume that it has knowledge soundness and binding. From all
this, we conclude that the above vanishing argument is sound.

[^schwartz-zippel]: The wikipedia page for the Schwartz-Zippel Lemma: [https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma](https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma)
[^factor-theorem]: The wikipedia page for the Factor Theorem: [https://en.wikipedia.org/wiki/Factor_theorem](https://en.wikipedia.org/wiki/Factor_theorem)

**Extending to multiple $f$'s**

We can use a linear combination of $\a$ to generalize the **Single Polynomial
Vanishing Argument**:

<!-- TODO: verify this $\sum \a^i v_{f_i} = v_t \cdot z_S(\xi)$ -->

\begin{algorithm}[H]
\caption*{
  \textbf{Vanishing Argument Protocol:} Checks queries of the form $\forall
  a \in S : f(a) \meq 0$, with scalars from $\Fb \supseteq S$, for a list
  of $\vec{f} \in \Fb^k_{\leq d}[X]$.
}
\textbf{Inputs} \\
  \Desc{$\vec{f}: \Fb^k_{\leq d}[X]$}{The polynomial to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    Either the verifier accepts with $\top$ or rejects with $\bot$.
  }
\begin{algorithmic}[1]
  \State $V:$ The verifier sends a random challenge $\a$ to the prover.
  \State $P:$ The prover constructs $t(X)$:
    \Statex \algind $t(X) = \sum_{i \in [k]} \frac{\a^i f_i(X)}{z_S}, \quad z_S(X) = \prod_{s \in S}(X - s)$
  \State $P \to V:$ then commits to $t(X)$ and each $f_i(X)$:
    \Statex \algind $C_{f_i} = \PCCommit(f_i(X), d, \bot), \quad C_t = \PCCommit(t(X), d, \bot)$
  \State $V \to P:$ The verifier sends challenge $\xi$ to the prover.
  \State $P \to V:$ The prover sends $(f_i(\xi) = v_{f_i}, \pi_{f_i}, t(\xi) = v_t, \pi_f)$ to the verifier.
  \State $V:$ The verifier then checks:
    \Statex \algind $\sum \a^i v_{f_i} = v_t \cdot z_S(\xi)$
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

If we have $m$ polynomials, $\vec{f}$, that all need to evaluate to
zero at the same challenge $\xi$, normally, we could construct $m$ opening
proofs, and verify these. We can, however, use the following protocol to
only create a single opening proof:

\begin{algorithm}[H]
\caption*{
  \textbf{Batched Evaluations Proofs Protocol:}
}
\textbf{Inputs} \\
  \Desc{$\vec{f}: \Fb^k_{\leq d}[X]$}{The polynomials to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    Either the verifier accepts with $\top$ or rejects with $\bot$.
  }
\begin{algorithmic}[1]
  \State $P \to V:$ The prover commits to each polynomial $C_{f_i}(X)$ and sends these to the verifier.
  \State $V \to P:$ The verifier sends challenge $\xi$ to the prover.
  \State $P \to V:$ The prover sends the evaluations of all $f_i(X)$ ($v_{f_i} = f_i(\xi))$.
  \State $P \to V:$ The prover also sends a single opening proof $\pi_w$ for the batched polynomial $w(X) = \sum_{i = 0}^k \a^i f_i(X)$
  \State $V:$ The verifier then constructs:
    \Statex \algind $C_w = \sum_{i = 0}^k \a^i C_{f_i}$
    \Statex \algind $v_w = \sum_{i = 0}^k \a^i v_{f_i}$
  \State $V:$ The verifier checks:
    \Statex \algind $\PCCheck(C_w, d, \xi, v_w, \pi_w) \meq \top$
  \end{algorithmic}
\end{algorithm}

**Correctness:**

Since:

- $C_w = \sum_{i = 0}^k \a^i C_{f_i} = \PCCommit(f(X), d, \bot)$ (Assuming a homomorphic commitment scheme)
- $v_w = \sum_{i = 0}^k \a^i v_{f_i} = w(\xi)$ (By definition of $w(X)$)

The correctness of the protocol is derived from the correctness of the underlying PCS.

**Soundness:**

<!-- TODO: Mind the soundness, should be fine -->

Recall that:

$$\sum \a^i f_i(\xi) = \sum \a^i v_i$$

This can be recontextualized as a polynomial:

$$p(\a) = \sum \a^i (f_i(\xi) - v_i) = 0$$

Then from Schwartz-Zippel, we acheive soundness, since the probability that
this polynomial evaluates to zero given that it's not the zero-polynomial
is $\frac{k}{|\Fb|}$.

### Grand Product Argument

- two polys f(X) g(X)
- $\prod_{s \in S} f(s) \meq \prod_{s \in S} g(s)$
- $z(\o) = 1$
- $z(\o X) = \dots$
- $f_1(X) = l_1(X)(z(X) - 1)$
- $f_2(X) = f(X)z(X) - g(X)z(X)$

TODO

<!--

## Outline

We now define the $\Surkal$ protocol using the above arguments.

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
  \State $(R: \Circuit, x: \PublicInputs, w : \Witness) = \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$ 
  \State $\pi = \SurkalProver(R,x,w)$
  \State \textbf{return} $\SurkalVerifier(R,x,\pi)$
  \end{algorithmic}
\end{algorithm}

-->
