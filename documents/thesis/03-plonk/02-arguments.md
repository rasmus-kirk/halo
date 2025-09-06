## Arguments

We now describe the arguments used in Plonk. We can safely assume that the
degree bound of any polynomial is always much smaller than the size of the
field, $|\Fb| \gg d$.

### Vanishing Argument

The checks that the verifier makes in Plonk boils down to checking identities
of the following form:

$$\forall s \in S : f(s) \meq 0$$

For some polynomial $f(X) \in \Fb_{\leq d}$ and some set $S \subset \Fb$. The
subset, $S$, may be much smaller than $\Fb$ as is the case for Plonk where
$S$ is set to be the set of roots of unity ($S = H = \{ \o^1, \o^2, \dots,
\o^n \}$). Since we ultimately model the above check with challenge scalars,
using just $S$, might not be sound. For this purpose, we can construct the
**Single Polynomial Vanishing Argument Protocol**:

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
  \State $P \to V:$ The prover commits to $f(X)$ and sends the commitment to the verifier:
    \Statex \algind $C_f = \PCCommit(f(X), d, \bot)$
  \State $P:$ The prover constructs $t(X)$:
    \Statex \algind $t(X) = \frac{f(X)}{z_S}, \quad z_S(X) = \prod_{s \in S}(X - s)$
  \State $P \to V:$ The prover then commits to $t(X)$ and sends the commitment to the verifier:
    \Statex \algind $C_t = \PCCommit(t(X), d, \bot)$
  \State $V \to P:$ The verifier sends a challenge $\xi$ to the prover
  \State $P:$ The prover computes: $v_f = f(\xi), \pi_f = \PCOpen(f(X), C_f, d, \xi, \bot)$.
  \State $P:$ The prover computes: $t_v = t(\xi), \pi_t = \PCOpen(t(X), C_t, d, \xi, \bot)$.
  \State $P \to V:$ The prover sends $v_f, \pi_f, t_v, \pi_t$ to the verifier.
  \State $V:$ The verifier then checks:
    \Statex \algind $v_f \meq v_t \cdot z_S(\xi)$
    \Statex \algind $\PCCheck(C_f, d, \xi, v_f, \pi_f) \meq \top \; \land$
    \Statex \algind $\PCCheck(C_t, d, \xi, v_t, \pi_t) \meq \top$
  \end{algorithmic}
\end{algorithm}

$$
\renewcommand{\arraystretch}{1.75}
\begin{array}{>{\displaystyle}l >{\displaystyle}c >{\displaystyle}l}
\textbf{Prover}(f \in \Fb_{\leq d}[X])    &                                 & \textbf{Verifier}                           \\
C_f = \PCCommit(f(X), d, \bot)            &                                 &                                             \\
z_S(X) = \prod_{s \in S}(X - s)           &                                 &                                             \\
t(X) = \frac{f(X)}{z_S}                   &                                 &                                             \\
C_t = \PCCommit(t(X), d, \bot)            & \rarr{C_f, C_t}                 & \xi \in_R \Fb                               \\
v_f = f(\xi)                              & \larr{\xi}                      &                                             \\
\pi_f = \PCOpen(f(X), C_f, d, \xi, \bot)  &                                 &                                             \\
v_t = t(\xi)                              &                                 &                                             \\
\pi_t = \PCOpen(t(X), C_f, d, \xi, \bot)  & \rarr{v_f, \pi_f, v_t, \pi_t}   & v_f \meq v_t \cdot z_S(\xi)                 \\
                                          &                                 & \PCCheck(C_f, d, \xi, v_f, \pi_f) \meq \top \\
                                          &                                 & \PCCheck(C_t, d, \xi, v_t, \pi_t) \meq \top \\
\end{array}
$$

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
<!-- TODO(rasmus): Or maybe it does, should probably argue for it... -->

The factor theorem states that if $f(X)$ is a univariate polynomial,
then $x - a$ is a factor of $f(X)$ if and only if $f(a) = 0$. This means
$z_S(X)$ only divides $f(X)$ if and only if all of $s \in S : f(s) = 0$. The
Schwartz-Zippel Lemma states that evaluating a non-zero polynomial on an input
chosen randomly from a large enough set is extremely unlikely to evaluate
to zero. Specifically, it ensures that $Pr[p(\xi) = 0 \land p(X) \neq 0]
\leq \frac{deg(p(X))}{|\Fb|}$. Clearly $\xi \in_R \Fb$ is sampled from a
large enough set as $|\Fb| \gg d \geq \deg(p(X))$ and therefore $Pr[p(\xi) =
0 \mid P \neq 0]$ is negligible. Lastly, the evaluation checked depends on
the soundness of the underlying PCS scheme used, but we assume that it has
knowledge soundness and binding. From all this, we conclude that the above
vanishing argument is sound.

**Extending to multiple $f$'s**

We can use a linear combination of $\a$ to generalize the **Single Polynomial
Vanishing Argument**:

\begin{algorithm}[H]
\caption*{
  \textbf{Vanishing Argument Protocol:} Checks queries of the form $\forall
  s \in S : f(s) \meq 0$, with scalars from $\Fb \supseteq S$, for a list
  of $\vec{f} \in \Fb^k_{\leq d}[X]$.
}
\textbf{Inputs} \\
  \Desc{$\vec{f}: \Fb^k_{\leq d}[X]$}{The polynomial to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    Either the verifier accepts with $\top$ or rejects with $\bot$.
  }
\begin{algorithmic}[1]
  \State $P \to V:$ The prover commits to each $f_i$ and the commitments to the verifier:
    \Statex \algind $C_{f_i} = \PCCommit(f_i(X), d, \bot)$
  \State $V:$ The verifier sends a random challenge $\a$ to the prover.
  \State $P:$ The prover constructs $t(X)$:
    \Statex \algind $t(X) = \sum_{i \in [k]} \frac{\a^i f_i(X)}{z_S}, \quad z_S(X) = \prod_{s \in S}(X - s)$
  \State $P \to V:$ The prover then commits to $t(X)$ and sends the commitment to the verifier:
    \Statex \algind $C_t = \PCCommit(t(X), d, \bot)$
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
  \State $P \to V:$ The prover sends evaluations of all $f_i(X)$ ($v_{f_i} = f_i(\xi))$.
  \State $V \to P:$ The verifier sends challenge $\alpha$ to the prover.
  \State $P \to V:$ The prover finally sends a single opening proof $\pi_w$ for the batched polynomial $w(X) = \sum_{i = 0}^k \a^i f_i(X)$
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

Suppose a prover, given polynomials $f(X), g(x)$ wanted to prove that these
polynomials when viewed as sets are equal to each other. This is called
multi-set equality, i.e, $\forall \o \in H : f(X) = g(X)$. This relation
can be modelled with a grand product:

$$
\begin{aligned}
  f'(X) &= f(X) + \gamma \\
  g'(X) &= g(X) + \gamma \\
  \prod_{i \in [n]} f'(\o^i) &= \prod_{i \in [n]} g'(\o^i)
\end{aligned}
$$

Completeness is trivial. As for soundness. We can interpretate each side of
the equality as a polynomial variable in $\g$:

$$
\begin{aligned}
  p(X) &= \prod_{i \in [n]} f(\o^i) + X \\
  q(X) &= \prod_{i \in [n]} g(\o^i) + X \\
\end{aligned}
$$

Then by Schwarz-Zippel, if we consider $r(X) = p(X) - q(X)$, if $r(\g) = 0$ and $r(\g) :
\g \in_R \Fb$ then $p(X) = q(X)$. Now, we just need to prove that $p(X) =
q(X) \implies \{ a_1, \dots, a_n \} = \{ b_1, \dots, b_n \}$.

Consider the roots of $p(X)$ and $q(X)$, starting with $p(X)$:

$$
\begin{aligned}
  p(X) &= \prod_{i \in [n]} f(\o^i) + X
\end{aligned}
$$

This polynomial evaluates to zero only if one of the factors equals
$f(\o^i)$. The same argument for $q(X)$ can also be applied:

$$
\begin{aligned}
  \text{roots}(p(X)) &= \{ -f(\o^1), \dots, -f(\o^n) \} \\
                     &= \{ -a_1, \dots, -a_n \} \\
\end{aligned}
$$
$$
\begin{aligned}
  \text{roots}(q(X)) &= \{ -g(\o^1), \dots, -g(\o^n) \} \\
                     &= \{ -b_1, \dots, -b_n \}
\end{aligned}
$$

Since the two polynomials are equal, they must have the same roots. Thus:

$$
\begin{aligned}
  \text{roots}(p(X)) &= \text{roots}(q(X)) \implies \\
  \{ -a_1, \dots, -a_n \} &= \{ -b_1, \dots, -b_n \} \implies \\
  \{ a_1, \dots, a_n \} &= \{ b_1, \dots, b_n \}
\end{aligned}
$$

$\qed$

We still need to convert $\prod_{i \in n} f'(\o^i) = \prod_{i \in n} g'(\o^i)$
to a polynomial that can be checked by the verifier. The prover can create
a polynomial, $z(X)$, to check the relation:

$$
\begin{aligned}
  z(\o^1) &= 1 \\
  z(\o^i) &= \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} \\
\end{aligned}
$$

The prover needs to convince the verifier that $z(X)$ has the expected form:

$$
\begin{aligned}
  z(\o^i)              &= \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} \\
  z(\o^i)              &= z(\o^{i-1}) \frac{f'(\o^{i-1})}{g'(\o^{i-1})} \\
  z(\o^{i+1})          &= z(\o^i) \frac{f'(\o^i)}{g'(\o^i)} \\
  z(\o^{i+1}) g'(\o^i) &= z(\o^i) f'(\o^i) \\
  z(\o^i) f'(\o^i)     &= z(\o \cdot \o^i) g'(\o^i) \\
\end{aligned}
$$

While also proving that $z(\o^1) = 1$. This leads to the following polynomials:

$$
\begin{aligned}
  f_{CC_1}(X) &= L_1(X)(Z(X) - 1) \\
  f_{CC_2}(X) &= z(X) f'(X) - z(\o X) g'(X) \\
\end{aligned}
$$

That should be zero for all $\o \in H$, which can be checked using the
**Vanishing Argument Protocol**. Finally, it needs to be argued that checking
these constraints lead to the desired goal of checking whether $\prod_{\o
\in H} f'(\o) \meq \prod_{\o \in H} g'(\o)$. Notice that in the last case,
$i = n$:

$$
\begin{aligned}
  z(\o^n) f'(\o^n)                                                 &= z(\o^{n+1}) g'(\o) \\
  \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} f'(\o^n)          &= g'(\o) \\
  \prod_{1 \leq j < i} \frac{f'(\o^j) f'(\o^n)}{g'(\o^j) g'(\o^n)} &= 1 \\
  \frac{\prod_{i \in [n]} f'(\o^i)}{\prod_{i \in [n]} g'(\o^i)}    &= 1 \\
  \prod_{i \in [n]} f'(\o^i)                                       &= \prod_{i \in [n]} g'(\o^i) \\
\end{aligned}
$$

And since, by the Vanishing Argument, $f_{CC_1}(X)$ and $f_{CC_2}(X)$ holds
for all $\o \in H$, it also holds for $\o^n$.
