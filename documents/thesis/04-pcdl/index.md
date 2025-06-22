# $\PCDL$: The Polynomial Commitment Scheme


## Outline

The Polynomial Commitment Scheme, $\PCDL$, is based on the Discrete Log
assumption, and does not require a trusted setup. Most of the functions simply
works as one would expect for a PCS, but uniquely for this scheme, we have
the function $\PCDLSuccinctCheck$ that allows deferring the expensive part
of checking PCS openings until a later point. This function is what leads
to the accumulation scheme, $\ASDL$, which is also based the Discrete Log
assumption. We have five main functions:

- $\PCDLSetup(\l, D)^{\rho_0} \to \pp_\PC$

  The setup routine. Given security parameter $\l$ in unary and a maximum
  degree bound $D$:
    - Runs $\pp_\CM \from \CMSetup(\l, D + 1)$,
    - Samples $H \in_R \Eb(\Fb_q)$ using the random oracle $H \from \rho_0(\pp_\CM)$,
    - Finally, outputs $\pp_\PC = (\pp_\CM, H)$.

- $\PCDLCommit(p: \Fb^{d'}_q[X], d: \Nb\mathblue{, \o: \Option(\Fb_q)}) \to \Eb(\Fb_q)$:

  Creates a commitment to the coefficients of the polynomial $p$ of degree
  $d' \leq d$ with optional hiding $\o$, using a Pedersen commitment.

- $\PCDLOpen^{\rho_0}(p: \Fb^{d'}_q[X], C: \Eb(\Fb_q), d: \Nb, z: \Fb_q\mathblue{, \o: \Option(\Fb_q)}) \to \EvalProof$:

  Creates a proof $\pi$ that states: "I know $p \in \Fb^{d'}_q[X]$ with
  commitment $C \in \Eb(\Fb_q)$ s.t. $p(z) = v$ and $\deg(p) = d' \leq d$"
  where $p$ is private and $d, z, v$ are public.

- $\PCDLSuccinctCheck^{\rho_0}(C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, v: \Fb_q, \pi: \EvalProof) \to \Result((\Fb^d_q[X], \Eb(\Fb_q)), \bot)$:

  Cheaply checks that a proof $\pi$ is correct. It is not a full check however,
  since an expensive part of the check is deferred until a later point.

- $\PCDLCheck^{\rho_0}(C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, v: \Fb_q, \pi: \EvalProof) \to \Result(\top, \bot)$:

  The full check on $\pi$.

The following subsections will describe them in pseudo-code, except for $\PCDLSetup$.

### $\PCDLCommit$

\begin{algorithm}[H]
\caption{$\PCDLCommit$}
\textbf{Inputs} \\
  \Desc{$p: \Fb^{d'}_q[X]$}{The univariate polynomial that we wish to commit to.} \\
  \Desc{$d: \Nb$}{A degree bound for $p$.} \\
  \Desc{$\mathblue{\o: \Option(\Fb_q)}$}{Optional hiding factor for the commitment.} \\
\textbf{Output} \\
  \Desc{$C: \Eb(\Fb_q)$}{The Pedersen commitment to the coefficients of polynomial $p$.}
\begin{algorithmic}[1]
  \Require $d \leq D$
  \Require $(d+1)$ is a power of 2.
  \State Let $\vec{p}^{\text{(coeffs)}}$ be the coefficient vector for $p$.
  \State Output $C := \CMCommit(\vec{G}, \vec{p}^{\text{(coeffs)}}, \mathblue{\o})$.
\end{algorithmic}
\end{algorithm}

$\PCDLCommit$ is rather simple, we just take the coefficients of the polynomial and
commit to them using a Pedersen commitment.

### $\PCDLOpen$

\begin{algorithm}[H]
\caption{$\PCDLOpen^{\rho_0}$}
\textbf{Inputs} \\
  \Desc{$p: \Fb^{d'}_q[X]$}{The univariate polynomial that we wish to open for.} \\
  \Desc{$C: \Eb(\Fb_q$)}{A commitment to the coefficients of $p$.} \\
  \Desc{$d: \Nb$}{A degree bound for $p$.} \\
  \Desc{$z: \Fb_q$}{The element that $z$ will be evaluated on $v = p(z)$.} \\
  \Desc{$\mathblue{\o: \Option(\Fb_q)}$}{Optional hiding factor for $C$. \textit{Must} be included if $C$ has hiding!} \\
\textbf{Output} \\
  \Desc{$\EvalProof$}{
    Proof of: "I know $p \in \Fb^{d'}_q[X]$ with commitment $C$ s.t. $p(z) = v$".
  }
\begin{algorithmic}[1]
  \Require $d \leq D$
  \Require $(d+1)$ is a power of 2.
  \State Let $n = d+1$
  \State Compute $v = p(z)$ and let $n = d+1$.
  \State \textblue{Sample a random polynomial $\bar{p} \in_R \Fb^{\leq d}_q[X]$ such that $\bar{p}(z) = 0$}.
  \State \textblue{Sample corresponding commitment randomness $\bar{\o} \in_R \Fb_q$.}
  \State \textblue{Compute a hiding commitment to $\bar{p}$: $\bar{C} \gets \PCDLCommit(\bar{p}, d, \bar{\o}) \in \Eb(\Fb_q)$.}
  \State \textblue{Compute the challenge $\a := \rho_0(C, z, v, \bar{C}) \in \Fb_q$.}
  \State \textblue{Compute commitment randomness $\o' := \o + \a \bar{\o} \in \Fb_q$}.
  \State Compute the polynomial $p' := p \mathblue{+ \a \bar{p}} = \sum_{i=0} c_i X_i \in \Fb^{\leq d}_q[X]$.
  \State Compute a non-hiding commitment to $p'$: $C' := C \mathblue{+ \a \bar{C} - \o' S} \in \Eb(\Fb_q)$.
  \State Compute the 0-th challenge field element $\xi_0 := \rho_0(C', z, v) \in \Fb_q$, then $H' := \xi_0 H \in \Eb(\Fb_q)$.
  \State Initialize the vectors ($\vec{c_0}$ is defined to be coefficient vector of $p'$):
    \Statex \algind $
      \begin{alignedat}[b]{1}
        \vec{c_0} &:= (c_0, c_1, \dots, c_d) \in F^n_q \\ 
        \vec{z_0} &:= (1, z^1, \dots, z^d) \in F^n_q \\
        \vec{G_0} &:= (G_0, G_1, \dots, G_d) \in \Eb(\Fb_q)_n \\
      \end{alignedat}
    $
  \For{$i \in [\lg(n)]$}
    \State Compute $L_i := \CMCommit(l(\vec{G_{i-1}}) \cat H', \; \;  r(\vec{c_{i-1}}) \cat \langle r(\vec{c_{i-1}}), l(\vec{z_{i-1}}) \rangle, \; \; \bot)$
    \State Compute $R_i := \CMCommit(r(\vec{G_{i-1}}) \cat H', \; \; l(\vec{c_{i-1}}) \cat \langle l(\vec{c_{i-1}}), r(\vec{z_{i-1}}) \rangle, \; \; \bot)$
    \State Generate the i-th challenge $\xi_i := \rho_0(\xi_{i-1}, L_i, R_i) \in \Fb_q$.
    \State Compress values for the next round: 
      \Statex \algindd $
        \begin{alignedat}[b]{3}
          \vec{G_i} &:= l(\vec{G_{i-1}}) &&+ \xi_i      &&\cdot r(\vec{G_{i-1}}) \\ 
          \vec{c_i} &:= l(\vec{c_{i-1}}) &&+ \xi^{-1}_i &&\cdot r(\vec{c_{i-1}}) \\
          \vec{z_i} &:= l(\vec{z_{i-1}}) &&+ \xi_i      &&\cdot r(\vec{z_{i-1}}) \\
        \end{alignedat}
      $
  \EndFor
  \State Finally output the evaluation proof $\pi := (\vec{L},\vec{R}, U := G^{(0)}, c := c^{(0)}, \mathblue{\bar{C}, \o'})$
\end{algorithmic}
\end{algorithm}

Where $l(x), r(x)$ returns the respectively left and right half of the
vector given.

The $\PCDLOpen$ algorithm mostly follows the IPA algorithm from
Bulletproofs. Except, in this case we are trying to prove we know polynomial
$p$ s.t. $p(z) = v = \dotp{\vec{c_0}}{\vec{z_0}}$. So because $z$ is public, we
can get away with omitting the generators, $(\vec{H})$, for $\vec{b}$ which
we would otherwise need in the Bulletproofs IPA. For efficiency we also
send along the curve point $U = G^{(0)}$, which the original IPA does not
do. The $\PCDLSuccinctCheck$ uses $U$ to make its check and $\PCDLCheck$
verifies the correctness of $U$.

### $\PCDLSuccinctCheck$

\begin{algorithm}[H]
\caption{$\PCDLSuccinctCheck^{\rho_0}$}
\textbf{Inputs} \\
  \Desc{$C: \Eb(\Fb_q)$}{A commitment to the coefficients of $p$.} \\
  \Desc{$d: \Nb$}{A degree bound on $p$.} \\
  \Desc{$z: \Fb_q$}{The element that $p$ is evaluated on.} \\
  \Desc{$v: \Fb_q$}{The claimed element $v = p(z)$.} \\
  \Desc{$\pi: \EvalProof$}{The evaluation proof produced by $\PCDLOpen$.} \\
\textbf{Output} \\
  \Desc{$\Result((\Fb^d_q[X], \Eb(\Fb_q)), \bot)$}{
    The algorithm will either succeed and output ($h: \Fb^d_q[X], U: \Eb(\Fb_q)$) if $\pi$ is a valid proof and otherwise fail ($\bot$).
  }
\begin{algorithmic}[1]
  \Require $d \leq D$
  \Require $(d+1)$ is a power of 2.
  \State Parse $\pi$ as $(\vec{L},\vec{R}, U := G^{(0)}, c := c^{(0)}, \mathblue{\bar{C}, \o'})$ and let $n = d + 1$.
  \State \textblue{Compute the challenge $\a := \rho_0(C, z, v, \bar{C}) \in \Fb_q$.}
  \State Compute the non-hiding commitment $C' := C \mathblue{+ \a \bar{C} - \o'S} \in \Eb(\Fb_q)$.
  \State Compute the 0-th challenge: $\xi_0 := \rho_0(C', z, v)$, and set $H' := \xi_0 H \in \Eb(\Fb_q)$.
  \State Compute the group element $C_0 := C' + vH' \in \Eb(\Fb_q)$.
  \For{$i \in [\lg(n)]$}
    \State Generate the i-th challenge: $\xi_i := \rho_0(\xi_{i-1}, L_i, R_i) \in \Fb_q$.
    \State Compute the i-th commitment: $C_i := \xi^{-1}_i L_i + C_{i-1} + \xi_i R_i \in \Eb(\Fb_q)$.
  \EndFor
\State Define the univariate polynomial $h(X) := \prod^{\lg(n)-1}_{i=0} (1 + \xi_{\lg(n) - i} X^{2^i}) \in \Fb_q[X]$.
\State Compute the evaluation $v' := c \cdot h(z) \in \Fb_q$.
\State Check that $C_{lg(n)} \meq cU + v'H'$
\State Output $(h(X), U)$.
\end{algorithmic}
\end{algorithm}

The $\PCDLSuccinctCheck$ algorithm performs the same check as in the
Bulletproofs protocol. With the only difference being that instead of
calculating $G^{(0)}$ itself, it trusts that the verifier sent the correct $U
= G^{(0)}$ in the prover protocol, and defers the verification of this claim
to $\PCDLCheck$. Notice also the "magic" polynomial $h(X)$, which has a degree $d$,
but can be evaluated in $\lg(d)$ time.

### $\PCDLCheck$

\begin{algorithm}[H]
\caption{$\PCDLCheck^{\rho_0}$}\label{alg:pcdl_check}
\textbf{Inputs} \\
  \Desc{$C: \Eb(\Fb_q)$}{A commitment to the coefficients of $p$.} \\
  \Desc{$d: \Nb$}{A degree bound on $p$} \\
  \Desc{$z: \Fb_q$}{The element that $p$ is evaluated on.} \\
  \Desc{$v: \Fb_q$}{The claimed element $v = p(z)$.} \\
  \Desc{$\pi: \EvalProof$}{The evaluation proof produced by $\PCDLOpen$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{The algorithm will either succeed ($\top$) if $\pi$ is a valid proof and otherwise fail ($\bot$).}
\begin{algorithmic}[1]
  \Require $d \leq D$
  \Require $(d+1)$ is a power of 2.
  \State Check that $\PCDLSuccinctCheck(C, d, z, v, \pi)$ accepts and outputs $(h, U)$.
  \State Check that $U \meq \CMCommit(\vec{G}, \vec{h}^{\text{(coeffs)}}, \bot)$, where $\vec{h}^{\text{(coeffs)}}$ is the coefficient vector of the polynomial $h$.
\end{algorithmic}
\end{algorithm}

Since $\PCDLSuccinctCheck$ handles the verification of the IPA given that
$U = G^{(0)}$, we run $\PCDLSuccinctCheck$, then check that $U \meq (G^{(0)}
= \CMCommit(\vec{G}, \vec{h}^{\text{(coeffs)}}, \bot) = \ip{\vec{G}}{\vec{h}^{\text{(coeffs)}}})$.

## Completeness

**Check 1** ($C_{lg(n)} \meq cU + v'H'$) **in $\PCDLSuccinctCheck$:**

Let's start by looking at $C_{\lg(n)}$. The verifier computes $C_{\lg(n)}$ as:
$$
\begin{aligned}
  C_0        &= C' + vH' = C + vH' \\
  C_{\lg(n)} &= C_0 + \sum^{\lg(n)-1}_{i=0} \xi^{-1}_{i+1} L_i + \xi_{i+1} R_i \\
\end{aligned}
$$
Given that the prover is honest, the following invariant should hold:
$$
\begin{alignedat}[b]{1}
  C_{i+1} &= \ip{\vec{c}_{i+1}}{\vec{G}_{i+1}} + \ip{\vec{c}_{i+1}}{\vec{z}_{i+1}} H'\\ 
          &= \ip{l(\vec{c}_i) + \xi^{-1}_{i+1} r(\vec{c}_i)}{l(\vec{G}_i) + \xi_{i+1} r(\vec{G}_i)} 
            + \ip{l(\vec{c}_i) + \xi^{-1}_{i+1} r(\vec{c}_i)}{l(\vec{z}_i) + \xi_{i+1} r(\vec{z}_i)} H'\\
          &= \ip{l(\vec{c}_i)}{l(\vec{G}_i)} + \xi_{i+1} \ip{l(\vec{c}_i))}{r(\vec{G}_i}
            + \xi^{-1}_{i+1} \ip{r(\vec{c}_i)}{l(\vec{G}_i)} + \ip{r(\vec{c}_i)}{r(\vec{G}_i)}\\
          &+ (\ip{l(\vec{c}_i)}{l(\vec{z}_i)} + \xi_{i+1} \ip{l(\vec{c}_i)}{r(\vec{z}_i)} 
            + \xi^{-1}_{i+1} \ip{r(\vec{c}_i)}{l(\vec{z}_i)} + \ip{r(\vec{c}_i)}{l(\vec{z}_i)}) H'
\end{alignedat}
$$
If we group these terms:
$$
\begin{alignedat}[b]{4}
  C_{i+1} &= \ip{l(\vec{c}_i)}{l(\vec{z}_i)}  &&+ \ip{r(\vec{c}_i)}{r(\vec{G}_i)}     &&+ \xi_{i+1} \ip{l(\vec{c}_i)}{r(\vec{G}_i)}    &&+ \xi^{-1}_{i+1} \ip{r(\vec{c}_i)}{l(\vec{G}_i)} \\
          &+ (\ip{l(\vec{c}_i)}{l(\vec{z}_i)} &&+ \ip{r(\vec{c}_i)}{r(\vec{z}_i)}) H' &&+ \xi_{i+1} \ip{l(\vec{c}_i)}{r(\vec{z}_i)} H' &&+ \xi^{-1}_{i+1} \ip{r(\vec{c}_i)}{l(\vec{z}_i)} H' \\
          &= C_i                              &&                                      &&+ \xi_{i+1} R_i                                &&+ \xi^{-1}_{i+1} L_i \\
          &\mkern-18mu\mkern-18mu \textbf{Where:} && && && \\
  L_i     &= \ip{r(\vec{c}_i)}{l(\vec{G}_i)} &&+ \ip{r(\vec{c}_i)}{l(\vec{z}_i)} H' && && \\
  R_i     &= \ip{l(\vec{c}_i)}{r(\vec{G}_i)} &&+ \ip{l(\vec{c}_i)}{r(\vec{z}_i)} H' && && 
\end{alignedat}
$$
We see why $\vec{L}, \vec{R}$ is defined the way they are. They help
the verifier check that the original relation hold, by showing it for the
compressed form $C_{i+1}$. $\vec{L}, \vec{R}$ is just the minimal information
needed to communicate this fact.

This leaves us with the following vectors (notice the slight difference in length):
$$
\begin{alignedat}[b]{1}
  \vec{L}    &= (L_1, \dots, L_{\lg(n)}) \\
  \vec{R}    &= (R_1, \dots, R_{\lg(n)}) \\
  \vec{C}    &= (C_0, \dots, C_{\lg(n)}) \\
  \vec{\xi}  &= (\xi_0, \dots, \xi_{\lg(n)}) \\
\end{alignedat}
$$
This means an honest prover will indeed produce $\vec{L}, \vec{R}$
s.t. $C_{\lg(n)} = C_0 + \sum^{\lg(n)-1}_{i=0} \xi^{-1}_{i+1} L_i + \xi_{i+1}
R_i$

Let's finally look at the left-hand side of the verifying check:

$$C_{\lg(n)} = C_0 + \sum^{\lg(n)-1}_{i=0} \xi^{-1}_{i+1} L_i + \xi_{i+1} R_i$$
The original definition of $C_i$:
$$C_{\lg(n)} = \ip{\vec{c}_{\lg(n)}}{\vec{G}_{\lg(n)}} + \ip{\vec{c}_{\lg(n)}}{\vec{z}_{\lg(n)}} H'$$
Vectors have length one, so we use the single elements $c^{(0)}, G^{(0)}, c^{(0)}, z^{(0)}$ of the vectors:
$$C_{\lg(n)} = c^{(0)}G^{(0)} + c^{(0)}z^{(0)} H'$$
The verifier has $c^{(0)} = c, G^{(0)} = U$ from $\pi \in \EvalProof$:
$$C_{\lg(n)} = cU + cz^{(0)} H'$$
Then, by construction of $h(X) \in \Fb^d_q[X]$:
$$C_{\lg(n)} = cU + ch(z) H'$$
Finally we use the definition of $v'$:
$$C_{\lg(n)} = cU + v'H'$$

Which corresponds exactly to the check that the verifier makes.

**Check 2** ($U \meq \CMCommit(\vec{G}, \vec{h}^{\text{(coeffs)}}, \bot)$) **in $\PCDLCheck$:**

The honest prover will define $U = G^{(0)}$ as promised and the right-hand
side will also become $U = G^{(0)}$ by the construction of $h(X)$.

## Knowledge Soundness

This subsection will not contain a full knowledge soundness proof, but it
will be briefly discussed that the _non-zero-knowledge_ version of $\PCDL$
should be knowledge sound. The knowledge soundness property of $\PCDL$ states:
$$
\Pr \left[
  \begin{array}{c}
    \PCCheck^\rho(C, d, z, v, \pi) = 1 \\
    \Downarrow \\
    C = \PCCommit^\rho(p, d, \o) \\
    v = p(z), \; \deg(p) \leq d \leq D
  \end{array}
  \middle|
  \begin{array}{r}
    \rho \leftarrow \Uc(\l) \\
    \pp_\PC \leftarrow \PCSetup^\rho(1^\l, D) \\
    (C, d, z, v, \pi) \leftarrow \Ac^\rho(\pp_\PC) \\
    (p, \o) \leftarrow \Ec^\rho(\pp_\PC) \\
  \end{array}
\right] \geq 1 - \negl(\lambda).
$$
So, we need to show that:

1. $C = \PCCommit^\rho(p, d, \o)$
2. $v = p(z)$
3. $\deg(p) \leq d \leq D$

The knowledge extractability of $\PCDL$ is almost identical to the IPA
from bulletproofs[@bulletproofs], so we assume that we can use the same
extractor[^ipa-extractor], with only minor modifications. The IPA extractor
extracts $\vec{a}, \vec{b} \in \Fb_q^n$ s.t:
$$P = \ip{\vec{G}}{\vec{a}} + \ip{\vec{H}}{\vec{b}} \land v = \ip{\vec{c}}{\vec{z}}$$
Running the extractor for $\PCDL$ should yield:
$$P = \ip{\vec{G}}{\vec{c}} + \ip{\vec{G}}{\vec{z}} \land v = \ip{\vec{c}}{\vec{z}}$$
We should be able to remove the extraction of $\vec{z}$ since it's public:
$$C = \ip{\vec{G}}{\vec{c}} \land v = \ip{\vec{c}}{\vec{z}}$$

1. $C = \ip{\vec{G}}{\vec{c}} = \PCCommit(c, G, \bot) = \PCCommit^\rho(p,
   d, \bot)$, $\o = \bot$ since we don't consider zero-knowledge.
2. $v = \ip{\vec{c}}{\vec{z}} = \ip{\vec{p}^{\text{(coeffs)}}}{\vec{z}} =
   p(z)$ by definition of $p$.
3. $\deg(p) \leq d \leq D$. The first bound holds since the vector committed
   to is known to have length $n = d+1$, the second bound holds trivially,
   as it's checked by $\PCDLCheck$

The authors, of the paper followed[@pcd], note that the soundness technically
breaks down when turning the IPA into a non-interactive protocol (which is
the case for $\PCDL$), and that transforming the IPA into a non-interactive
protocol such that the knowledge extractor does not break down is an open
problem:

\begin{quote}
\color{GbGrey}

\textbf{Security of the resulting non-interactive argument.} It is known
from folklore that applying the Fiat–Shamir transformation to a public-coin
$k$-round interactive argument of knowledge with negligible soundness error
yields a non-interactive argument of knowledge in the random-oracle model
where the extractor $\Ec$ runs in time exponential in $k$. In more detail, to
extract from an adversary that makes $t$ queries to the random oracle, $\Ec$
runs in time $t^{\Oc(k)}$. In our setting, the inner-product argument has $k
= \Oc(\log d)$ rounds, which means that if we apply this folklore result, we
would obtain an extractor that runs in superpolynomial (but sub-exponential)
time $t^{\Oc(\log d)} = 2^{\Oc(log(\l)^2)}$. It remains an interesting open
problem to construct an extractor that runs in polynomial time.

\end{quote}

This has since been solved in a 2023 paper[@attema]. The abstract of the
paper describes:

\begin{quote}
\color{GbGrey}

Unfortunately, the security loss for a $(2\mu + 1)$-move protocol is, in
general, approximately $Q^\mu$, where $Q$ is the number of oracle queries
performed by the attacker. In general, this is the best one can hope for,
as it is easy to see that this loss applies to the $\mu$-fold sequential
repetition of $\Sigma$-protocols, $\dots$, we show that for $(k^1, \dots,
k^\mu)$-special-sound protocols (which cover a broad class of use cases),
the knowledge error degrades linearly in $Q$, instead of $Q^\mu$.

\end{quote}

The IPA is exactly such a $(k^1, \dots,k^\mu)$-special-sound protocol, they
even directly state that this result applies to bulletproofs. As such we get
a knowledge error that degrades linearly, instead of superpolynomially, in
number of queries, $t$, that the adversary makes to the random oracle. Thus,
the extractor runs in the required polynomial time ($\Oc(t) = \Oc(\poly(\l))$).

[^ipa-extractor]: Admittedly, this assumption is not a very solid one if the
purpose was to create a proper knowledge soundness proof, but as the section is
more-so devoted to give a justification for why $\PCDL$ _ought to be_ sound,
it will do. In fact, the authors of the accumulation scheme paper[@pcd],
use a similar argument more formally by stating (without direct proof!),
that the $\PCDL$ protocol is a special case of the IPA presented in another
paper[@ipa] by mostly the same authors.

## Efficiency

Given two operations $f(x), g(x)$ where $f(x)$ is more expensive than $g(x)$,
we only consider $f(x)$, since $\Oc(f(n) + g(n)) = \Oc (f(n))$. For all the
algorithms, the most expensive operations will be scalar multiplications. We
also don't bother counting constant operations, that does not scale with
the input. Also note that:
$$
  \Oc\left(\sum_{i=2}^{\lg(n)} \frac{n}{i^2}\right) = \Oc\left(n \sum_{i=2}^{\lg(n)} \frac{1}{i^2}\right)
                                       = \Oc(n \cdot c)
                                       = \Oc(n)
$$
Remember that in the below contexts $n = d+1$

- $\PCDLCommit$: $n = \Oc(d)$ scalar multiplications and $n = \Oc(d)$ point additions.
- $\PCDLOpen$:
  - Step 1: 1 polynomial evaluation, i.e. $n = \Oc(d)$ field multiplications.
  - Step 13 & 14: Both commit $\lg(n)$ times, i.e. $2 (\sum_{i=2}^{\lg(n)} (n+1)/i) = \Oc(2n)$
    scalar multiplications. The sum appears since we halve the vector
    length each loop iteration.
  - Step 16: $\lg(n)$ vector dot products, i.e. $\sum_{i=2}^{\lg(n)} n/i = \Oc(n)$ scalar multiplications.

  In total, $\Oc(3d) = \Oc(d)$ scalar multiplications.
- $\PCDLSuccinctCheck$:
  - Step 7: $\lg(n)$ hashes.
  - Step 8: $3 \lg(n)$ point additions and $2 \lg(n)$ scalar multiplications.
  - step 11: The evaluation of $h(X)$ which uses $\Oc(\lg(n))$ field additions.

  In total, $\Oc(2 \lg(n)) = \Oc(\lg(d))$ scalar multiplications.
- $\PCDLCheck$:
  - Step 1: Running $\PCDLSuccinctCheck$ takes $\Oc(2 \lg(d))$ scalar multiplications.
  - Step 2: Running $\CMCommit(\vec{G}, \vec{h}^{\text{(coeffs)}}, \bot)$ takes $\Oc(d)$ scalar multiplications.

  Since step two dominates, we have $\Oc(d)$ scalar multiplications.

So $\PCDLOpen$, $\PCDLCheck$ and $\PCDLCommit$ is linear and, importantly, $\PCDLSuccinctCheck$ is sub-linear.

\begin{quote}
\color{GbGrey}

\textbf{Sidenote: The runtime of $h(X)$}

Recall the structure of $h(X)$:
$$h(X) := \prod^{\lg(n)-1}_{i=0} (1 + \xi_{\lg(n) - i} X^{2^i}) \in \Fb_q[X]$$
First note that $\left(\prod^{\lg(n)-1}_{i=0} a\right)$ leads to $\lg(n)$
factors. Calculating $X^{2^i}$ can be computed as:
$$X^{2^0}, X^{2^1} = (X^{2^0})^2, X^{2^2} = (X^{2^1})^2, \dots$$
So that part of the evaluation boils down to the cost of squaring in the
field. We therefore have $\lg(n)$ squarings (from $X^{2^i}$), and $\lg(n)$
field multiplications from $\xi_{\lg(n) - i} \cdot X^{2^i}$. Each squaring
can naively be modelled as a field multiplication ($x^2 = x \cdot x$). We
therefore end up with $2\lg(n) = \Oc(\lg(n))$ field multiplications
and $\lg(n)$ field additions. The field additions are ignored as the
multiplications dominate.

Thus, the evaluation of $h(X)$ requires $\Oc(\lg(n))$ field multiplications,
which dominate the runtime.

\end{quote}

