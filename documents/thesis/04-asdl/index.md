# $\ASDL$: The Accumulation Scheme

## Outline

The $\ASDL$ accumulation scheme is an accumulation scheme for accumulating
polynomial commitments. This means that the corresponding predicate,
$\Phi_\AS$, that we accumulate for, represents the checking of polynomial
commitment openings, $\Phi_\AS(q_i) = \PCDLCheck(q_i)$. The instances are
assumed to have the same degree bounds. A slight deviation from the general
$\AS$ specification, is that that the algorithms don't take the old accumulator
$\acc_{i-1}$ as input, instead, since it has the same form as instances
$\mathblue{(}(C_\acc, d_\acc, z_\acc, v_\acc)\mathblue{, \pi_V)}$, it will
be prepended to the instance list $\vec{q}$. We have six main functions:

- $\ASDLSetup(1^\l, D) \to \pp_\AS$

  Outputs $\pp_\AS = \PCDLSetup(1^\l, D)$.

- $\ASDLCommonSubroutine(\vec{q}: \Instance^m \mathblue{, \pi_V: \AccHiding}) \to \Result((\Eb(\Fb_q), \Nb, \Fb_q, \Fb^d_q[X]), \bot)$

  $\ASDLCommonSubroutine$ will either succeed if the instances have consistent
  degree and hiding parameters and will otherwise fail. It accumulates
  all previous instances into a new polynomial $h(X)$, and is run by both
  $\ASDLProver$ and $\ASDLVerifier$ in order to ensure that the accumulator,
  generated from $h(X)$ correctly accumulates the instances. It returns
  $(\bar{C}, d, z, h(X))$ representing the information needed to create the
  polynomial commitment represented by $\acc_i$.

- $\ASDLProver(\vec{q}: \Instance^m) \to \Result(\Acc, \bot)$:

  Accumulates the instances $\vec{q}$, and an optional previous
  accumulator $\acc_{i-1}$, into a new accumulator $\acc_i$. If there is a
  previous accumulator $\acc_{i-1}$ then it is converted into an instance,
  since it has the same form, and prepended to $\vec{q}$, _before calling
  the prover_.

- $\ASDLVerifier(\vec{q}: \Instance^m, \acc_i: \Acc) \to \Result(\top, \bot)$:

  Verifies that the instances $\vec{q}$ (as with $\ASDLProver$, including a
  possible $\acc_{i-1}$) was correctly accumulated into the new accumulator
  $\acc_i$.

- $\ASDLDecider(\acc_i: \Acc) \to \Result(\top, \bot)$:

  Checks the validity of the given accumulator $\acc_i$ along with all
  previous accumulators that was accumulated into $\acc_i$.

This means that accumulating $m$ instances, $\vec{q} = [q_i]^m$, should
yield $\acc_i$, using the $\ASDLProver(\vec{q})$. If the verifier accepts
$\ASDLVerifier(\vec{q}, \acc_i) = \top$, and $\ASDLDecider$ accepts the
accumulator ($\ASDLDecider(\acc_i) = \top$), then all the instances,
$\vec{q}$, will be valid, by the soundness property of the accumulation
scheme. This is proved for $\ASDL$ in the soundness section. Note that this
also works recursively, since $q_{\acc_{i-1}} \in \vec{q}$ is also proven valid
by the decider.

The following subsections will describe the functions in
pseudo-code, except $\ASDLSetup$.

### $\ASDLCommonSubroutine$

\begin{algorithm}[H]
\caption{$\ASDLCommonSubroutine$}
\textbf{Inputs} \\
  \Desc{$\vec{q}: \Instance^m$}{New instances \textit{and accumulators} to be accumulated.} \\
  \Desc{$\mathblue{\pi_V: \AccHiding}$}{Necessary parameters if hiding is desired.} \\
\textbf{Output} \\
  \Desc{$\Result((\Eb(\Fb_q), \Nb, \Fb_q, \Fb^d_q[X]), \bot)$}{
    The algorithm will either succeed $(\Eb(\Fb_q), \Nb, \Fb_q, \Fb^d_q[X])$
    if the instances has consistent degree and hiding parameters and will
    otherwise fail ($\bot$).
  }
\begin{algorithmic}[1]
  \Require $(D+1) = 2^k$, where $k \in \Nb$
  \State Parse $d$ from $q_1$.
  \State \textblue{Parse $\pi_V$ as $(h_0, U_0, \o)$, where $h_0(X) = aX + b \in \Fb^1_q[X], U_0 \in \Eb(\Fb_q)$ and $\o \in \Fb_q$}
  \State \textblue{Check that $U_0$ is a deterministic commitment to $h_0$: $U_0 = \PCDLCommit(h, d, \bot)$.}
  \For{$j \in [0, m]$}
    \State Parse $q_j$ as a tuple $((C_j, d_j, z_j, v_j), \pi_j)$.
    \State Compute $(h_j(X), U_j) := \PCDLSuccinctCheck^{\rho_0}(C_j, d_j, z_j, v_j, \pi_j)$.
    \State Check that $d_j \meq d$
  \EndFor
  \State Compute the challenge $\a := \rho_1(\vec{h}, \vec{U}) \in \Fb_q$
  \State Let the polynomial $h(X) := \mathblue{h_0 +} \sum^m_{j=1} \a^j h_j(X) \in \Fb_q[X]$
  \State Compute the accumulated commitment $C := \mathblue{U_0 +} \sum^m_{j=1} \a^j U_j$
  \State Compute the challenge $z := \rho_1(C, h(X)) \in \Fb_q$.
  \State Randomize $C$: $\bar{C} := C \mathblue{+ \o S} \in \Eb(\Fb_q)$.
  \State Output $(\bar{C}, D, z, h(X))$.
\end{algorithmic}
\end{algorithm}

The $\ASDLCommonSubroutine$ does most of the work of the $\ASDL$ accumulation
scheme. It takes the given instances and runs the $\PCDLSuccinctCheck$
on them to acquire $[(h_j(X), U_j)]^m_{i=0}$ for each of them. It then creates a
linear combination of $h_j(X)$ using a challenge point $\a$ and computes the
claimed commitment for this polynomial $C = \sum^m_{j=1} \a^j U_j$, possibly
along with hiding information. This routine is run by both $\ASDLProver$
and $\ASDLVerifier$ in order to ensure that the accumulator, generated from
$h(X)$ correctly accumulates the instances. To see the intuition behind why
this works, refer to the note in the $\ASDLDecider$ section.

### $\ASDLProver$

\begin{algorithm}[H]
\caption{$\ASDLProver$}
\textbf{Inputs} \\
  \Desc{$\vec{q}: \Instance^m$}{New instances \textit{and accumulators} to be accumulated.} \\
\textbf{Output} \\
  \Desc{$\Result(\Acc, \bot)$}{
    The algorithm will either succeed $((\bar{C}, d, z, v, \pi), \pi_V)
    \in \Acc)$ if the instances has consistent degree and hiding
    parameters and otherwise fail ($\bot$).
  }
  \begin{algorithmic}[1]
  \Require $\forall (\_, d_i, \_, \_, \_) \in \vec{q}, \forall (\_, d_j, \_, \_, \_) \in \vec{q} : d_i = d_j \land d_i \leq D$
  \Require $(d_i+1) = 2^k$, where $k \in \Nb$
  \State \textblue{Sample a random linear polynomial $h_0(X) \in_R F^{\leq d}_q[X]$}
  \State \textblue{Then compute a deterministic commitment to $h_0(X)$: $U_0 := \PCDLCommit(h_0, d, \bot)$}
  \State \textblue{Sample commitment randomness $\o \in_R F_q$, and set $\pi_V := (h_0, U_0, \o)$.}
  \State Then, compute the tuple $(\bar{C}, d, z, h(X)) := \ASDLCommonSubroutine(\vec{q} \mathblue{, \pi_V})$.
  \State Compute the evaluation $v := h(z) \in \Fb_q$.
  \State Generate the evaluation proof $\pi := \PCDLOpen(h(X), \bar{C}, d, z \mathblue{, \o})$.
  \State Finally, output the accumulator $\acc_i = \mathblue{(}(\bar{C}, d, z, v, \pi)\mathblue{, \pi_V)}$.
\end{algorithmic}
\end{algorithm}

Simply accumulates the the instances, $\vec{q}$, into new accumulator $\acc_i$, using $\ASDLCommonSubroutine$.

### $\ASDLVerifier$

\begin{algorithm}[H]
\caption{$\ASDLVerifier$}
\textbf{Inputs} \\
  \Desc{$\vec{q}: \Instance^m$}{New instances \textit{and possible accumulator} to be accumulated.} \\
  \Desc{$\acc_i: \Acc$}{The accumulator that accumulates $\vec{q}$. \textit{Not} the previous accumulator $\acc_{i-1}$.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    The algorithm will either succeed $(\top)$ if $\acc_i$ correctly accumulates
    $\vec{q}$ and otherwise fail ($\bot$).
  }
  \begin{algorithmic}[1]
  \Require $(D+1) = 2^k$, where $k \in \Nb$ 
    \State Parse $\acc_i$ as $\mathblue{(}(\bar{C}, d, z, v, \_)\mathblue{, \pi_V)}$
    \State The accumulation verifier computes $(\bar{C}', d', z', h(X)) := \ASDLCommonSubroutine(\vec{q} \mathblue{, \pi_V})$
    \State Then checks that $\bar{C}' \meq \bar{C}, d' \meq d, z' \meq z$, and $h(z) \meq v$.
\end{algorithmic}
\end{algorithm}

The verifier also runs $\ASDLCommonSubroutine$, therefore verifying that
$\acc_i$ correctly accumulates $\vec{q}$, which means:

- $\bar{C} = C + \o S = \sum_{j=1}^m \a^j U_j + \o S$
- $\forall (\_, d_j, \_, \_, \_) \in \vec{q} : d_j = d$
- $z = \rho_1(C, h(X))$
- $v = h(z)$
- $h(X) = \sum_{j=0}^m \a^j h_j(X)$
- $\a := \rho_1(\vec{h}, \vec{U})$

### $\ASDLDecider$

\begin{algorithm}[H]
\caption{$\ASDLDecider$}
\textbf{Inputs} \\
  \Desc{$\acc_i: \Acc$}{The accumulator.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    The algorithm will either succeed $(\top)$ if the accumulator has correctly
    accumulated all previous instances and will otherwise fail ($\bot$).
  }
  \begin{algorithmic}[1]
  \Require $\acc_i.d \leq D$
  \Require $(\acc_i.d+1) = 2^k$, where $k \in \Nb$ 
    \State Parse $\acc_i$ as $\mathblue{(}(\bar{C}, d, z, v, \pi)\mathblue{, \_)}$
    \State Check $\top \meq \PCDLCheck(\bar{C}, d, z, v, \pi)$
\end{algorithmic}
\end{algorithm}

The decider fully checks the accumulator $\acc_i$, this verifies each previous accumulator meaning that:
$$
\begin{aligned}
  &\forall i \in [n], \forall j \in [m] : \\
  &\ASDLVerifier((\ToInstance(\acc_{i-1}) \cat \vec{q}_{i-1}), \acc_i) \land \ASDLDecider(\acc_n) \implies \\
  &\Phi_\AS(q^{(i)}_j) = \PCDLCheck(q^{(i)}_j) = \top
\end{aligned}
$$
The sidenote below gives an intuition why this is the case.

\begin{quote}
\color{GbGrey}

\textbf{Sidenote: Why does checking $\acc_i$ check all previous instances
and previous accumulators?}

The $\ASDLProver$ runs the $\ASDLCommonSubroutine$ that creates an accumulated
polynomial $h$ from $[h_j(X)]^m$ that is in turn created for each instance $q_j
\in \vec{q}_i$ by $\PCDLSuccinctCheck$:
$$h_j(X) := \prod^{lg(n)}_{i=0} (1 + \xi_{\lg(n)-i} \cdot X^{2^i}) \in F_q[X]$$
We don't mention the previous accumulator $\acc_{i-1}$ explicitly as it's
treated as an instance in the protocol. We also only consider the case where
the protocol does not have zero knowledge, meaning that we omit the blue parts
of the protocol. The $\ASDLVerifier$ shows that $C$ is a commitment to $h(X)$
in the sense that it's a linear combination of all $h_j(X)$'s from the previous
instances, by running the same $\ASDLCommonSubroutine$ algorithm as the prover
to get the same output. Note that the $\ASDLVerifier$ does not guarantee that
$C$ is a valid commitment to $h(X)$ in the sense that $C = \PCDLCommit(h, d,
\bot)$, that's the $\ASDLDecider$'s job. Since $\ASDLVerifier$ does not verify
that each $U_j$ is valid, and therefore that $C = \PCDLCommit(h, d, \bot)$,
we now wish to argue that $\ASDLDecider$ verifies this for all the instances.

\textbf{Showing that $C = \PCDLCommit(h, d, \bot)$:}

The $\ASDLProver$ has a list of instances $(q_1, \dots, q_m) = \vec{q}_i$,
then runs $\PCDLSuccinctCheck$ on each of them, getting $(U_1, \dots, U_m)$
and $(h_1(X), \dots, h_m(X))$. For each element $U_j$ in the vector $\vec{U}
\in \Eb(\Fb_q)^m$ and each element $h_j(X)$ in the vector $\vec{h} \in
(\Fb^{\leq d}_q[X])^m$, the $\ASDLProver$ defines:
$$h(X) := \sum^{m}_{j=1} \a^j h_j(X)$$
$$C := \sum^{m}_{j=1} \a^j U_j$$
Since we know from the $\ASDLVerifier$:

\begin{enumerate}
  \item $\PCDLSuccinctCheck(q_j) = \top$
  \item $C_{\acc_i} = \sum_{j=1}^m \a^j U_j$
  \item $z_{\acc_i} = \rho_1(C, h(X))$
  \item $h_{\acc_i}(X) = \sum_{j=0}^m \a^j h_j(X)$
  \item $\a := \rho_1(\vec{h}, \vec{U})$
\end{enumerate}

Which implies that $\Phi_\AS(q_j) = \top$ if $U = G^{(0)}$. We then argue that
when the $\ASDLDecider$ checks that $C = \PCDLCommit(h(X), d, \bot)$, then
that implies that each $U_j$ is a valid commitment to $h_j(X)$, $U_j =
\PCDLCommit(h_j(X), d, \bot) = \ip{\vec{G}}{\vec{h_j}}$, thereby performing
the second check of $\PCDLCheck$, on all $q_j$ instances at once. We know that:

\begin{enumerate}
  \item
    $\PCDLCheck$ tells us that $C_{\acc_i} = \sum_{j=1}^m \a^j U_j$ except with
    negligible probability, since,
  \item
    The binding property of $\CM$ states that it's hard to find a different
    $C'$, s.t., $C = C'$ but $h_{\acc_i}(X) \neq h'(X)$. Which means that
    $h_{\acc_i}(X) = h'(X)$.
  \item
    Define $B_j = \ip{\vec{G}}{\vec{h_j}^{(\text{coeffs})}}$. If $\exists j
    \in [m]$ $B_j \neq U_j$ then $U_j$ is not a valid commitment to $h_j(X)$ and
    $\sum_{j=1}^m \a_j B_j \neq \sum_{j=1}^m \a_j U_j$. As such $C_{\acc_i}$
    will not be a valid commitment to $h_{\acc_i}(X)$. Unless,
  \item
    $\a := \rho_1(\vec{h}, \vec{U})$ or $z = \rho_1(C, h(X))$ is constructed
    in a malicious way, which is hard, since they're from the random oracle.
\end{enumerate}

<!-- TODO: This is wrong -->

To sum up, this means that running the $\ASDLDecider$ corresponds to checking
all $U_j$'s.

What about checking the previous instances, $\vec{q}_{i-1}$, accumulated into
the previous accumulator, $\acc_{i-1}$? The accumulator for $\vec{q}_{i-1}$
is represented by an instance $acc_{i-1} = (C = \PCDLCommit(h_{\acc_{i-1}},
d, \bot), d, z, v = h_{\acc_{i-1}}(z), \pi)$, which, as mentioned, behaves
like all other instances in the protocol and represents a PCS opening to
$h_{\acc_{i-1}}(X)$. Since $\acc_{i-1}$ is represented as an instance, and
we showed that as long as each instance is checked by $\ASVerifier$ (which
$\acc_{i-1}$ also is), running $\PCDLCheck(\acc_i)$ on the corresponding
accumulation polynomial $h_{\acc_i}(X)$ is equivalent to performing the
second check $U_j = \PCDLCommit(h_j(X), d, \bot)$ on all the $h_j(X)$ that
$h_{\acc_i}(X)$ consists of. Intuitively, if any of the previous accumulators
were invalid, then their commitment will be invalid, and the next accumulator
will also be invalid. That is, the error will propagate. Therefore, we will
also check the previous set of instances $\vec{q}_{i-1}$, and by induction,
all accumulated instances $\vec{q}$ and accumulators $\vec{\acc}$.

\end{quote}

## Completeness

$\ASDLVerifier$ runs the same algorithm ($\ASDLCommonSubroutine$) with the
same inputs and, given that $\ASDLProver$ is honest, will therefore get the
same outputs, these outputs are checked to be equal to the ones received from
the prover. Since these were generated honestly by the prover, also using
$\ASDLCommonSubroutine$, the $\ASDLVerifier$ will accept with probability 1,
returning $\top$. Intuitively, this also makes sense. It's the job of the
verifier to verify that each instance is accumulated correctly into the
accumulator. This verifier does the same work as the prover and checks that
the output matches.

As for the $\ASDLDecider$, it just runs $\PCDLCheck$ on the provided
accumulator, which represents a evaluation proof i.e. an instance. This
check will always pass, as the prover constructed it honestly.

## Soundness

In order to prove soundness, we first need a helper lemma:

---

**Lemma: Zero-Finding Game:**

Let $\CM = (\CMSetup, \CMCommit)$ be a perfectly binding commitment scheme. Fix
a maximum degree $D \in \Nb$ and a random oracle $\rho$ that takes commitments
from $\CM$ to $F_\pp$. Then for every family of functions $\{f_\pp\}_\pp$
and fields $\{F_\pp\}_\pp$ where:

- $f_\pp \in \Mc \to F_\pp^{\leq D}[X]$
- $F \in \Nb \to \Nb$
- $|F_\pp| \geq F(\l)$

That is, for all functions, $f_\pp$, that takes a message, $\Mc$ as input and
outputs a maximum D-degree polynomial. Also, usually $|F_\pp| \approx F(\l)$.
For every message format $L$ and computationally unbounded $t$-query oracle
algorithm $\Ac$, the following holds:
$$
\Pr\left[
  \begin{array}{c}
    p \neq 0 \\
    \land \\
    p(z) = 0
  \end{array}
  \middle|
  \begin{array}{c}
    \rho \from \mathcal{U}(\l) \\
    \pp_\CM \gets \CMSetup(1^\l, L) \\
    (m, \omega) \gets \Ac^\rho(\pp_\CM) \\
    C \gets \CMCommit(m, \o) \\
    z \in F_{\pp} \from \rho(C) \\
    p := f_{\pp}(m)
  \end{array}
\right] \leq \sqrt{\frac{D(t+1)}{F(\l)}}
$$
Intuitively, the above lemma states that for any non-zero polynomial $p$,
that you can create using the commitment $C$, it will be highly improbable
that a random evaluation point $z$ be a root of the polynomial $p$, $p(z)
= 0$. For reference, this is not too unlike the Schwartz-Zippel Lemma.

**Proof:**

We construct a reduction proof, showing that if an adversary $\Ac$ that wins
with probability $\d$ in the above game, then we construct an adversary $\Bc$
which breaks the binding of the commitment scheme with probability at least:
$$\frac{\delta^2}{t + 1} - \frac{D}{F(\lambda)}$$
Thus, leading to a contradiction, since $\CM$ is perfectly binding. Note,
that we may assume that $\Ac$ always queries $C \from \CMCommit(m, \o)$
for its output $(m, \o)$, by increasing the query bound from $t$ to $t + 1$.

\begin{algorithm}[H]
\caption*{\textbf{The Adversary} $\Bc(\pp_\CM)$}
\begin{algorithmic}[1]
  \State Run $(m, \omega) \gets \Ac^\rho(\pp_\CM)$, simulating its queries to $\rho$.
  \State Get $C \gets \CMCommit(m, \o)$.
  \State Rewind $\Ac$ to the query $\rho(C)$ and run to the end, drawing fresh randomness for this and subsequent oracle queries, to obtain $(p', \omega')$.
  \State Output $((m, \omega), (m', \omega'))$.
\end{algorithmic}
\end{algorithm}

<!-- TODO: Step 3 - Local Forking Lemma -->

Each $(m, \o)$-pair represents a message where $p \neq 0 \land p(z) = 0$
for $z = \rho(\CMCommit(m, \o))$ and $p = f_\pp(m)$ with probability $\d$

Let:
$$
\begin{aligned}
  C' &:= \CMCommit(p', \o') \\
  z  &:= \rho(C) \\
  z' &:= \rho(C') \\
  p  &:= f_{pp}(m) \\
  p' &:= f_{pp}(m')
\end{aligned}
$$
By the Local Forking Lemma[@forking-lemma], the probability that $p(z) =
p'(z') = 0$ and $C = C'$
is at least $\frac{\d^2}{t + 1}$. Let's call this event $E$:
$$E := (p(z) = p'(z') = 0 \land C = C')$$
Then, by the triangle argument:
$$
\Pr[E] \leq \Pr[E \land (p = p')] + \Pr[E \land (p \neq p')]
$$
And, by Schwartz-Zippel:
$$
\begin{aligned}
\Pr[E \land (p = p')] &\leq \frac{D}{|F_\pp|} \implies \\
                      &\leq \frac{D}{F(\lambda)}
\end{aligned}
$$
Thus, the probability that $\Bc$ breaks binding is:
$$
\begin{aligned}
\Pr[E \land (p = p')] + \Pr[E \land (p \neq p')] &\geq \Pr[E] \\
\Pr[E \land (p \neq p')] &\geq \Pr[E] - \Pr[E \land (p = p')] \\
\Pr[E \land (p \neq p')] &\geq \frac{\d^2}{t + 1} - \frac{D}{F(\lambda)} \\
\end{aligned}
$$
Yielding us the desired probability bound. Isolating $\d$ will give us the
probability bound for the zero-finding game:
$$
\begin{aligned}
  0 &= \frac{\delta^2}{t + 1} - \frac{D}{F(\lambda)} \\
  \frac{\delta^2}{t + 1} &= \frac{D}{F(\lambda)} \\
  \delta^2 &= \frac{D(t + 1)}{F(\lambda)} \\
  \delta &= \sqrt{\frac{D(t + 1)}{F(\lambda)}}
\end{aligned}
$$

$\qed$

For the above Lemma to hold, the algorithms of $\CM$ must not have access to
the random oracle $\rho$ used to generate the challenge point $z$, but
$\CM$ may use other oracles. The lemma still holds even when $\Ac$ has
access to the additional oracles. This is a concrete reason why domain
separation, as mentioned in the Fiat-Shamir subsection, is important.

---

With this lemma, we wish to show that given an adversary $\Ac$, that breaks
the soundness property of $\ASDL$, we can create a reduction proof that then
breaks the above zero-finding game. We fix $\Ac, D = \poly(\l)$ from the $\AS$
soundness definition:
$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      \ASDLVerifier^{\rho_1}((q_{\acc_{i-1}} \cat \vec{q}), \acc_i) = \top, \\
      \ASDLDecider^{\rho_1}(\acc_i) = \top \\
      \land \\
      \exists i \in [n] : \Phi_\AS(q_i) = \bot
    \end{array}
  & \quad
    \begin{aligned}
      \rho_0 &\leftarrow \Uc(\l), \rho_1 \leftarrow \Uc(\l), \\
      \pp_\PC &\leftarrow \PCDLSetup^{\rho_0}(1^\l, D), \\
      \pp_\AS &\leftarrow \ASDLSetup^{\rho_1}(1^\l, \pp_\PC), \\
      (\vec{q}, \acc_{i-1}, \acc_i) &\leftarrow \Ac^{\rho_1}(\pp_\AS, \pp_\PC) \\
      q_{acc_{i-1}} &\leftarrow \ToInstance(\acc_{i-1}) \\
    \end{aligned}
  \end{array}
\right] \leq \negl(\l)
$$
We call the probability that the adversary $\Ac$ wins the above game
$\d$. We bound $\d$ by constructing two adversaries, $\Bc_1, \Bc_2$, for
the zero-finding game. Assuming:

- $\Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{wins}] = \delta - \negl(\l)$
- $\Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{wins}] = 0$

These assumptions will be proved after defining the adversaries concretely. So,
we claim that the probability that either of the adversaries wins is $\delta -
\negl(\l)$ and that both of the adversaries cannot win the game at the same
time. With these assumptions, we can bound $\d$:
$$
\begin{aligned}
  \Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{ wins}] &= \Pr[\Bc_1 \text{ wins}] + \Pr[\Bc_2\text{ wins}] - \Pr[\Bc_1 \text{ wins} \land \Bc_2 \text{ wins}]\\
  \Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{ wins}] &= \Pr[\Bc_1 \text{ wins}] + \Pr[\Bc_2\text{ wins}] - 0 \\
  \delta - \negl(\l)                              &\leq  \sqrt{\frac{D(t+1)}{F(\l)}} + \sqrt{\frac{D(t+1)}{F(\l)}} \\
  \delta - \negl(\l)                              &\leq  2 \cdot \sqrt{\frac{D(t+1)}{|\Fb_q|}}                     \\
  \delta                                          &\leq  2 \cdot \sqrt{\frac{D(t+1)}{|\Fb_q|}} + \negl(\l)         \\
\end{aligned}
$$
Meaning that $\delta$ is negligible, since $q = |\Fb_q|$ is superpolynomial
in $\l$. We define two perfectly binding commitment schemes to be used for
the zero-finding game:

- $\CM_1$:
  - $\CM_1.\Setup^{\rho_0}(1^\l, D) := \pp_\PC \from \PCDLSetup^{\rho_0}(1^\lambda, D)$
  - $\CM_1.\Commit((p(X), h(X)), \_) := (C \from \PCDLCommit(p(X), d, \bot), h)$
  - $\Mc_{\CM_1} := \{(p(X), h(X) = \a^j h_j(X))\} \in \Pc((\Fb_q^{\leq D}[X])^2)$
  - $z_{\CM_1} := \rho_1(\CM_1.\Commit((p(X), h(X)), \_)) = \rho_1((C \from \PCDLCommit(p(X), d, \bot), h)) = z_\acc$
- $\CM_2$:
  - $\CM_2.\Setup^{\rho_0}(1^\l, D) := \pp_\PC \from \PCDLSetup^{\rho_0}(1^\lambda, D)$
  - $\CM_2.\Commit([(h_j(X), U_j)]^m, \_) := [(h_j(X), U_j)]^m$:
  - $\Mc_{\CM_2} := \{[(h_j(X), U_j)]^m\} \in \Pc((\Fb_q^{\leq D}[X] \times \Eb(\Fb_q))^m)$
  - $z_{\CM_2} := \rho_1(\CM_2.\Commit([(h_j(X), U_j)]^m, \_)) = \rho_1([(h_j(X), U_j)]^m) = \a$

Note that the $\CM_1, \CM_2$ above are perfectly binding, since they either
return a Pedersen commitment, without binding, or simply return their
input. $\Mc_{\CM_1}$ consists of pairs of polynomials of a maximum
degree $D$, where $\forall j \in [m] : h(X) = \a^j h_j(X)$. $\Mc_{\CM_2}$
consists of a list of pairs of a maximum degree $D$ polynomial, $h_j(X)$,
and $U_j$ is a group element. Notice that $z_a = z_\acc$ and $z_b
= \a$ where $z_\acc$ and $\a$ are from the $\ASDL$ protocol.

We define the corresponding functions $f^{(1)}_{\pp}, f^{(2)}_{\pp}$ for
$\CM_1, \CM_2$ below:

- $f^{(1)}_\pp(p(X), h(X) = [h_j(X)]^m) := a(X) = p(X) - \sum_{j=1}^m \a^j h_j(X)$,
- $f^{(2)}_\pp(p = [(h_j(X), U_j)]^m) := b(Z) = \sum_{j=1}^m a_j Z^j$ where for each $j \in [m]$:
  - $B_j \leftarrow \PCDLCommit(h_j, d, \bot)$
  - Compute $b_j : b_j G = U_j - B_j$

We then construct an intermediate adversary, $\Cc$, against $\PCDL$, using $\Ac$:

\begin{algorithm}[H]
\caption*{\textbf{The Adversary} $\Cc^{\rho_1}(\pp_\PC)$}
\begin{algorithmic}[1]
  \State Parse $\pp_\PC$ to get the security parameter $1^\l$ and set $\AS$ public parameters $\pp_{\AS} := 1^\l$.
  \State Compute $(\vec{q}, \acc_{i-1}, \acc_i) \leftarrow \Ac^{\rho_1}(\pp_\AS)$.
  \State Parse $\pp_\PC$ to get the degree bound $D$.
  \State Output $(D, \acc_i = (C_\acc, d_\acc, z_\acc, v_\acc), \vec{q})$.
\end{algorithmic}
\end{algorithm}

The above adversary also outputs $\vec{q}$ for convenience, but the
knowledge extractor simply ignores this. Running the knowledge extractor,
$\Ec_\Cc^{\rho_1}$, on $\Cc$, meaning we extract $\acc_i$, will give us $p$.
Provided that $\ASDLDecider$ accepts, the following will hold with probability
$(1 - \negl)$:

- $C_\acc$ is a deterministic commitment to $p(X)$.
- $p(z_\acc) = v_\acc$
- $\deg(p) \leq d_\acc \leq D$

Let's denote successful knowledge extraction s.t. the above points holds
as $E_\Ec$. Furthermore, the $\ASDLDecider$ (and $\ASDLVerifier$'s) will accept
with probability $\d$, s.t. the following holds:

- $\ASDLVerifier^{\rho_1}((q_{\acc_{i-1}} \cat \vec{q}), \acc_i) = \top$
- $\ASDLDecider^{\rho_1}(\acc_i) = \top$
- $\exists i \in [n] : \Phi_\AS(q_i) = \bot \implies \PCDLCheck^{\rho_0}(C_i, d_i, z_i, v_i, \pi_i) = \bot$

Let's denote this event as $E_\Dc$. We're interested in the probability $\Pr[E_\Ec
\land E_\Dc]$. Using the chain rule we get:
$$
\begin{aligned}
  \Pr[E_\Ec \land E_\Dc] &= \Pr[E_\Ec \; | \; E_\Dc] \cdot \Pr[E_\Ec] \\
                         &= \d \cdot (1 - \negl(\l)) \\
                         &= \d - \d \cdot \negl(\l) \\
                         &= \d - \negl(\l)
\end{aligned}
$$
Now, since $\ASDLVerifier^{\rho_1}((q_{\acc_{i-1}} \cat \vec{q}), \acc_i)$ accepts,
then, by construction, all the following holds:

1. For each $j \in [m]$, $\PCDLSuccinctCheck$ accepts.
2. Parsing $\acc_i = (C_\acc, d_\acc, z_\acc, v_\acc)$ and setting $\a := \rho_1([(h_j(X), U_j)]^m)$, we have that:
    - $z_\acc = \rho_1(C_\acc, [h_j(X)]^m)$
    - $C_\acc = \sum_{j=1}^m \a^j U_j$
    - $v_\acc = \sum_{j=1}^m \a^j h_j(z)$

Also by construction, this implies that either:

- $\PCDLSuccinctCheck$ rejects, which we showed above is not the case, so therefore,
- The group element $U_j$ is not a commitment to $h_j(X)$.

We utilize this fact in the next two adversaries, $\Bc_1, \Bc_2$,
constructed, to win the zero-finding game for $\CM_1, \CM_2$ respectively,
with non-negligible probability:

\begin{algorithm}[H]
\caption*{\textbf{The Adversary} $\Bc_k^{\rho_1}(\pp_\AS)$}
\begin{algorithmic}[1]
  \State Compute $(D, \acc_i, \vec{q}) \leftarrow C^{\rho_1}(\pp_\AS)$.
  \State Compute $p \leftarrow \Ec_C^\rho(\pp_\AS)$.
  \State For each $q_j \in \vec{q}$ : $(h_j, U_j) \from \PCDLSuccinctCheck(q_j)$.
  \State Compute $\a := \rho_1([(h_j, U_j)]^m)$.
  \If{$k = 1$}
    \State Output $((n, D), (p, h := ([h_j]^m)))$
  \ElsIf{$k = 2$}
    \State Output $((n, D), ([(h_j, U_j)]^m))$
  \EndIf
\end{algorithmic}
\end{algorithm}

Remember, the goal is to find an evaluation point, s.t. $a(X) \neq 0 \land
a(z_a) = 0$ for $\CM_1$ and $b(X) \neq 0 \land b(z_b) = 0$ for $\CM_2$. We
set $z_a = z_\acc$ and $z_b = \a$. Now, there are then two cases:

1. $C_\acc \neq \sum_{j=1}^m \a^j B_j$: This means that for some $j \in [m]$,
   $U_j \neq B_j$. Since $C_\acc$ is a commitment to $p(X)$, $p(X) - h(X)$ is
   not identically zero, but $p(z_\acc) = h(z_\acc)$. Thusly, $a(X) \neq 0$
   and $a(z_\acc) = 0$. Because $z_\acc = z_a$ is sampled using the random
   oracle $\rho_1$, $\Bc_1$ wins the zero-finding game against $(\CM_1,
   \{f_\pp^{(1)}\}_\pp)$.

2. $C = \sum_{j=1}^n \a^j B_j$. Which means that for all $j \in [m]$, $U_j =
   B_j$. Since $C = \sum_{j=1}^n \a^j U_j$, $\a$ is a root of the
   polynomial $a(Z)$, $a(\a) = 0$. Because $\a$ is sampled using the random
   oracle $\rho_1$, $\Bc_2$ wins the zero-finding game against $(CM_2,
   \{f_\pp^{(2)}\}_\pp)$.

So, since one of these adversaries always win if $E_\Ec \land E_\Dc$, the
probability that $\Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{wins}]$ is indeed
$\delta - \negl(\l)$. And since the above cases are mutually exclusive we
also have $\Pr[\Bc_1 \text{ wins} \lor \Bc_2 \text{wins}]$. Thus, we have
proved that, given the zero-finding game Lemma, the probability that an
adversary can break the soundness property of the $\ASDL$ accumulation scheme
is negligible.

$\qed$

## Efficiency

- $\ASDLCommonSubroutine$:
  - Step 6: $m$ calls to $\PCDLSuccinctCheck$, $m \cdot \Oc(2\lg(d)) = \Oc(2m\lg(d))$ scalar multiplications.
  - Step 11: $m$ scalar multiplications.

  Step 6 dominates with $\Oc(2m\lg(d)) = \Oc(m\lg(d))$ scalar multiplications.
- $\ASDLProver$:
  - Step 4: 1 call to $\ASDLCommonSubroutine$, $\Oc(md)$ scalar multiplications.
  - Step 5: 1 evaluation of $h(X)$, $\Oc(\lg(d))$ scalar multiplications.
  - Step 6: 1 call to $\PCDLOpen$, $\Oc(3d)$ scalar multiplications.

  Step 6 dominates with $\Oc(3d) = \Oc(d)$ scalar multiplications.
- $\ASDLVerifier$:
  - Step 2: 1 call to $\ASDLCommonSubroutine$, $\Oc(2m\lg(d))$ scalar multiplications.

  So $\Oc(2m\lg(d)) = \Oc(m\lg(d))$ scalar multiplications.
- $\ASDLDecider$:
  - Step 2: 1 call to $\PCDLCheck$, with $\Oc(d)$ scalar multiplications.

  $\Oc(d)$ scalar multiplications.

So $\ASDLProver$ and $\ASDLDecider$ are linear and $\ASDLDecider$ is sub-linear.

