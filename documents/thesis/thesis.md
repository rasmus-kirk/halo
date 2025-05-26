---
title: Cryptographic Computing Project - Plonk
author:
  - Rasmus Kirk Jakobsen - 201907084
  - Abdul Haliq Abdul Latiff - 202303466
geometry: margin=2cm
---

\tableofcontents
\newpage

# New

## SECURITY PROOFS

## Utils

### Vanishing

- Rasmus

### Batched Evaluation Proofs

- Rasmus

### Grand Product argument(s)

- Haliq

## Copy Constraint Rewrite

- Haliq

## Plonkup

- Haliq

## How do we write a circuit

- Haliq

## Gadgets

### XOR

### Poseidon

### Range Check

### Foreign Field stuff

## Signatures

## IVC Verifier from Gadgets

### NARK (PLONK)

### Accumulation Verifier

### SuccinctCheck

### Signatures

# Introduction

SNARKs - **S**uccinct **N**on-interactive **AR**guments of **K**nowledge
have seen increased usage due to their application in blockchains and
cryptocurrencies. Since it's an _argument of knowledge_, we have a prover
and a verifier, $(P,V)$, where the prover must prove knowledge of their
witness $w$. This might seem trivial so far, couldn't the prover just send
the verifier $w$? However, we also have the requirement of _succinctness_;
the communication between the prover and verifier must be sublinear.

They usually also function as so called _general-purpose proof schemes_. This
means that, given any solution to an NP-problem, it will produce a proof
that the prover knows a solution. Usually, a verifier of the NP-problem
is compiled to a circuit $R$, then it's proven that $R$ is satisfied
i.e outputs one ($1 \from R(w)$). Snark constructions are also commonly
used for zero-knowledge arguments, making them zk-SNARKs, and Plonk also
supports zero-knowledge arguments.

Notably, this project focuses on implementing the mechanics of the Plonk
protocol using a commitment scheme, $\PCDL$, based on the discrete log relation,
as described in the paper "Proof-Carrying Data from Accumulation
Schemes"[^pcd]. The implementation was done by one of the others in a
concurrent project[^pcdl]. This means that although original Plonk paper uses
KZG[^kzg], which relies on a Trusted Setup in order to work, our implementation
instead uses an Transparent Setup.

Plonk is being used as a part of the Halo recursive proof scheme[^halo],
that's used by both Zcash's Halo2[^halo2] and Mina's Pickles[^pickles]. Both are
very similar and can be broken down into the following components:

- **Plonk**: A general-purpose, potentially zero-knowledge, proof scheme.
- **$\PCDL$**: A Polynomial Commitment Scheme in the Discrete Log setting.
- **$\ASDL$**: An Accumulation Scheme in the Discrete Log setting.
- **Pasta**: A Cycle of Elliptic Curves, namely **Pa**llas and Ve**sta**.

This project only focuses on Plonk.

[^pcd]: Proof-Carrying Data from Accumulation Schemes paper: [https://eprint.iacr.org/2020/499](https://eprint.iacr.org/2020/499)
[^pcdl]: Halo Accumulation Project: [https://github.com/rasmus-kirk/halo-accumulation](https://github.com/rasmus-kirk/halo-accumulation)
[^kzg]: KZG paper: [https://iacr.org/cryptodb/data/paper.php?pubkey=23846](https://iacr.org/cryptodb/data/paper.php?pubkey=23846)
[^halo]: Halo paper: [https://eprint.iacr.org/2019/1021](https://eprint.iacr.org/2019/1021)
[^halo2]: Halo2: [https://zcash.github.io/halo2/](https://zcash.github.io/halo2/)
[^pickles]: Pickles: [https://o1-labs.github.io/proof-systems/specs/pickles.html](https://o1-labs.github.io/proof-systems/specs/pickles.html)

# Prerequisites

Basic knowledge on elliptic curves, groups, interactive arguments are
assumed in the following text.

## Polynomial Commitment Schemes

Modern general-purpose (zero-knowledge) proof schemes, such as Sonic[^sonic],
Marlin[^marlin], and of course Plonk, commonly use PCS's _Polynomial Commitment Schemes_
for creating their proofs. A PCS allows a prover to prove to a verifier that
a committed polynomial evaluates to a certain value, $v$, given an evaluation
input $z$. There are three main functions used to prove this, along with a
setup routine:

- $\PCSetup(\l, D) \to \pp$

  The setup routine. Given security parameter $\l$ in unary and a maximum
  degree bound $D$. Creates the public parameters $\pp$.

- $\PCCommit(p: \Fb^{d'}_q[X], d: \Nb, \o: \Option(\Fb_q)) \to \Eb(\Fb_q)$

  Commits to a polynomial $p$ with degree bound $d$ where $d \geq d'$ using
  optional hiding $\o$.

- $\PCOpen(p: \Fb^{d'}_q[X], C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, \o: \Option(\Fb_q)) \to \EvalProof$

  Creates a proof, $\pi \in \EvalProof$, that the polynomial $p$, with
  commitment $C$, evaluated at $z$ gives $v = p(z)$, using the hiding input
  $\o$ if provided.

- $\PCCheck(C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, v: \Fb_q, \pi: \EvalProof) \to \Result(\top, \bot)$

  Checks the proof $\pi$ that claims that the polynomial $p$ that $C$ is a
  commitment to, evaluates to $v = p(z)$.

A polynomial commitment scheme of course also has soundness and completeness
properties:

**Completeness:** For every maximum degree bound $D = \poly(\l) \in \Nb$:

$$
\Pr \left[
  \begin{array}{c}
    d \in [d_i]^n_{i=1}, \; \deg(p) \leq d \leq D \\
    \PCCheck^\rho(C, d, z, v, \pi) = 1
  \end{array}
  \middle|
  \begin{array}{r}
    \pp \leftarrow \PCSetup^\rho(1^\l, D) \\
    ([d_i]^n_{i=1}, p, d, z, \o) \leftarrow \Ac^\rho(\pp) \\
    C \leftarrow \PCCommit^\rho(p, d, \o) \\
    v \leftarrow p(z) \\
    \pi \leftarrow \PCOpen^\rho(p, C, d, z; \o)
  \end{array}
\right] = 1.
$$

I.e. an honest prover will convince an honest verifier.

**Soundness:** For every maximum degree bound $D = \poly(\l) \in \Nb$ and
polynomial-size adversary $\Ac$, there exists an efficient extractor $\Ec$
such that the following holds:

$$
\Pr \left[
  \begin{array}{c}
    \PCCheck^\rho(C, d, z, v, \pi) = 1 \\
    \Downarrow \\
    C = \PCCommit^\rho(p, d, \o) \\
    v = p(z), \; d \in [d_i]^n_{i=1}, \; \deg(p) \leq d \leq D
  \end{array}
  \middle|
  \begin{array}{r}
    \rho \leftarrow \Uc(\l) \\
    \pp \leftarrow \PCSetup^\rho(1^\l, D) \\
    ([d_i]^n_{i=1}, (C, d, z, v, \pi)) \leftarrow \Ac^\rho(\pp) \\
    (p, \o) \leftarrow \Ec^\rho(\pp) \\
  \end{array}
\right] \leq \negl(\lambda).
$$

I.e. any adversary, $\Ac$, will not be able to open on a different polynomial,
than the one they committed to.

[^sonic]: Sonic paper: [https://eprint.iacr.org/2019/099](https://eprint.iacr.org/2019/099)
[^marlin]: Marlin paper: [https://eprint.iacr.org/2019/1047](https://eprint.iacr.org/2019/1047)

## Fiat-Shamir Heuristic

We use the Fiat-Shamir heuristic to make the entire protocol
non-interactive. The Fiat-Shamir heuristic turns a public-coin interactive
proof into a into a non-interactive interactive proof. This is done by
replacing all random values sent by the verifier to the prover with calls to a
non-interactive random oracle. In practice a cryptographic hash function $\Hc$
is used, where the transcript along with any public information[^frozen-heart]
is hashed using $\Hc$.

[^frozen-heart]: Not using public inputs can lead to a
vulnerability called "The Frozen Heart Vulnerability". This
specific vulnerability [have affected some Plonk
implementations](https://blog.trailofbits.com/2022/04/18/the-frozen-heart-vulnerability-in-plonk/)

# The Protocol

The goal of Plonk is for a prover to convince a verifier of the following
claim:

**The Claim:** "I know private inputs[^pi] $\vec{x} \in \Fb^n$ s.t. when given
a public circuit $R$, then $R(\vec{x}) = \vec{y} \in \Fb^m$"

Where the number of the inputs for circuit $R$ is $n$ and the number of the
outputs is $m$. Let's look at a simple circuit representing the computation
of $3x^2_1 + 5x_2$:

\begin{figure}
\centering
\begin{tikzpicture}
% First Layer
\node (input1) at (3, 7) {$x_1$};
\node (input2) at (5, 7) {$x_2$};
\node (A) at (1, 7) {$3$};
\node (B) at (7, 7) {$5$};

    % Second Layer
    \node[draw, rectangle] (mul21) at (3, 5.5) {$\times$};
    \node[above left=0.01cm of mul21] {$a_1$};
    \node[above right=0.01cm of mul21] {$b_1$};
    \node[below right=0.01cm of mul21] {$c_1$};

    \node[draw, rectangle] (mul22) at (6, 5.5) {$\times$};
    \node[above left=0.01cm of mul22] {$a_2$};
    \node[above right=0.01cm of mul22] {$b_2$};
    \node[below right=0.01cm of mul22] {$c_2$};

    \draw[->] (input1) -- (2, 7) |- (mul21);
    \draw[->] (input1) -- (4, 7) |- (mul21);

    \draw[->] (input2) -- (5, 6.5) |- (mul22);
    \draw[->] (B) -- (7, 6.5) |- (mul22);

    % Third Layer
    \node[draw, rectangle] (mul31) at (2, 4) {$\times$};
    \node[above left=0.01cm of mul31] {$a_3$};
    \node[above right=0.01cm of mul31] {$b_3$};
    \node[below right=0.01cm of mul31] {$c_3$};

    \draw[->] (mul21) -- (3, 4) |- (mul31);
    \draw[->] (A) -- (1, 4) |- (mul31);

    % Fourth Layer
    \node[draw, rectangle] (add41) at (4, 2.5) {$+$};
    \node[above left=0.01cm of add41] {$a_3$};
    \node[above right=0.01cm of add41] {$b_3$};
    \node[below right=0.01cm of add41] {$c_3$};

    \draw[->] (mul31) -- (2, 3.5) |- (add41);
    \draw[->] (mul22) -- (6, 3.5) |- (add41);

    % Fifth Layer
    \node (output) at (4, 1) { $y_1$ };

    \draw[->] (add41) -- (output);

\end{tikzpicture}
\caption{A simple circuit}
\end{figure}

In the above figure the output of the multiplication gate on the right, $c_2$,
should be equal to the value of the input wire of the addition gate, $b_3$. Plonk
enforces this using a _copy constraint._ Using this, we can split our desired
claim into two constraints, that must hold:

- **Gate Constraints:** The gates of circuit $R$ was computed correctly.
- **Copy Constraints:** Different wires indices representing the same wire,
  should have the same value.

We will explore how the machinery of Plonk achieves this in the next sections.

[^pi]: Technically, Plonk also supports public inputs, but these
can also be modelled as constant gates, so we omit public inputs for simplicity.

## Check Conversions

The checks that the verifier makes in Plonk boils down to checking identities
of the following form:

$$\forall a \in S : F(a) \meq 0$$

For some polynomial $F(X) \in \Fb_{\leq d}$ and some set $S \subset \Fb$. The
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
  \textbf{Vanishing Argument Protocol:} Converts queries for polynomial
  identities ranging over all values $a \in H \subset S$ to a secure
  non-interactive protocol using polynomial commitments
}
\textbf{Inputs} \\
  \Desc{$\vec{F}: \Fb^k_{\leq d}[X]$}{The polynomial to check identity for.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State The prover $P$ constructs $T(X)$:
    \Statex \algind $T = \sum_{i \in [k]} F_i(X) \cdot \frac{\a^i}{Z_s}$
    \Statex \algind Where $Z_s(X) = \prod_{s \in S}(X - s)$. 
  \State $P$ then commits to $F(X), T(X)$:
    \Statex \algind $C_F = \PCCommit(F(X), d, \bot), C_T = \PCCommit(T(X), d, \bot)$
  \State The verifier $V$ then checks:
    \Statex \algind $\sum_{i \in [k]} \a^i F_i(\xi) - T(\xi) \cdot Z_s(\xi) \meq 0 \; \land$
    \Statex \algind $\PCCheck(C_T, d, \xi, v_T, \pi_T) \meq \top \; \land$
    \Statex \algind $\forall i \in [k] : \PCCheck(C_{F_i}, d, \xi, v_{F_i}, \pi_{F_i}) \meq \top$
  \end{algorithmic}
\end{algorithm}

Note that for the Plonk protocol specifically, $S = H = \{ 1, \o, \o^2, \dots,
\o^{n-1} \}$ for the reason that $Z_s$ then becomes $Z_s = X^n - 1$ because
$\o$ is a root of unity of order $n$. This is much more efficient to
compute. The $\alpha$'s are used since we need a linearly independent
combination of $\vec{F}$. We use powers of $\alpha$ instead of a vector of
$\alpha$'s ($\vec{\a}$) to reduce the necessary communication between the
prover and verifier, thusly reducing proof size.

**Correctness**

The check is correct by definition of $T(X)$

For any $\xi \in \Fb \setminus H$, the following holds:

$$
\begin{aligned}
P(X) = & \sum_{i\in [k]} \a^i F_i(\xi) - T(\xi) Z_S(\xi) \\
= & \sum_{i\in [k]} \a^i F_i(\xi) - \left( \sum_{i\in [k]} F_i(\xi) \cdot \frac{\a^i}{Z_S(\xi)} \right) Z_S(\xi) \\
= & \sum_{i\in [k]} \a^i F_i(\xi) - \sum_{i\in [k]} F_i(\xi) \cdot \a^i \\
= & 0
\end{aligned}
$$

However, for $\xi \in H$ we know that $Z_S(\xi) = 0$, thus the following holds

$$
\begin{aligned}
P(X) = & \sum_{i\in [k]} \a^i F_i(\xi) - T(\xi) Z_S(\xi) \\
= & \sum_{i\in [k]} \a^i F_i(\xi) - T(\xi) \times 0 \\
= & \sum_{i\in [k]} \a^i F_i(\xi) \\
\meq & 0
\end{aligned}
$$

For the above to hold true, it must be the case that $\forall i : F_i(\xi)
= 0$ which is the intended construction of $F_i$.

**Soundness**

The Schwartz-Zippel Lemma[^schwartz-zippel] states that evaluating a nonzero
polynomial on inputs chosen randomly from a large enough set is likely to
find an input that produces a nonzero output. Specifically it ensures that
$Pr[P(\xi)] \leq \frac{deg(P)}{|\Fb|}$. Clearly $\xi \in \Fb$ is a large enough
set as $|\Fb| \gg |H|$ and therefore $Pr[P(\xi) | P \neq 0]$ is negligible. The
rest of the soundness argument depends on the underlying PCS used.

See Lemma 4.5 and Claim 4.6 in the Plonk paper for more details.

[^schwartz-zippel]: The wikipedia page for the Schwartz-Zippel Lemma: [https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma](https://en.wikipedia.org/wiki/Schwartz%E2%80%93Zippel_lemma)

## The Constraint System

To construct a Plonk circuit, we start from an abstraction of gates identified
by an index $i$ that take in two inputs $a_i, b_i$ and an arbitrary fan out
of outputs $c_i$. There are two kinds of gates $\times$ and $+$ corresponding
to multiplication and addition. The wires for these gates carry values of
elements in $\Fb$.

The constraint system models the following equation

$$a_i q_l + b_i q_r + c_i q_o + a_i b_i q_m  + q_c = 0$$

$q_l, q_r, q_o, q_m, q_c$ are called the selector values. These are their
values for the different gates:

- $q_l = \begin{cases} 
      0 & \text{if the gate is a multiplication gate} \\
      1 & \text{if the gate is an addition gate}
    \end{cases}$
- $q_r = q_l$
- $q_o = -1$
- $q_m = 1 - q_l$
- $q_c = 0$

which consequently gives the following constraint for the different gates:

$$
\begin{aligned}
  (+) &\mapsto a_i + b_i - c_i + 0 + 0 &= 0 \\
  (\times) &\mapsto 0 + 0 - c_i + a_i b_i + 0& = 0 \\
\end{aligned}
$$

There are also wires that are constants which are fixed values by definition
of the circuit. For some constant $y$ they are defined with the same constraint
system. We assign the wires and selector values as follows:

- $a_i = y$
- $b_i = c_i = 0$
- $q_l = 1$
- $q_r = q_o = q_m = 0$
- $q_c = -y$

Which consequently gives the following constraint for the constant wire:

$$y + 0 + 0 + 0 - y = 0$$

In our circuits, our inputs are all private. In other words the verifier does
not know these values. Thus, we treat private inputs the same way we treat
constants. However, the verifier does know the structure of the circuits;
the selector values and which wires are "connected". More on connected wires
in the section on copy constraints.

### Example: Circuit Evaluations in a Constraint System

Take the following circuit as a concrete example:

$$((\mathtt{\_a} \times \mathtt{\_a}) \times 3) + (\mathtt{\_b} \times 5) - 47$$

Let the private inputs be $\vec{x} = (\mathtt{\_a} = 1, \mathtt{\_b} = 2)$
and let the constants be $\mathtt{\_d} = 3, \mathtt{f} = 5, \mathtt{\_i}= -47$.
We then have the following gates:

- $\mathtt{\_a}:1 \times \mathtt{\_a}:1 = \mathtt{\_c}:1$
- $\mathtt{\_c}:1 \times \mathtt{\_d}:3 = \mathtt{\_e}:3$
- $\mathtt{\_b}:2 \times \mathtt{\_f}:5 = \mathtt{\_g}:10$
- $\mathtt{\_e}:3 + \mathtt{\_g}:10 = \mathtt{\_h}:13$
- $\mathtt{\_h}:13 + \mathtt{\_i}:-47 = \mathtt{\_j}:-34$

When modelled as constraints, we can tabulate the following

| i   | $a_i$              | $b_i$              | $c_i$              | $q_l$ | $q_r$ | $q_o$ | $q_m$ | $q_c$ |
| --- | ------------------ | ------------------ | ------------------ | ----- | ----- | ----- | ----- | ----- |
| 0   | $\mathtt{\_a}:1$   | $0$                | $0$                | $1$   | $0$   | $0$   | $0$   | $-1$  |
| 1   | $\mathtt{\_b}:2$   | $0$                | $0$                | $1$   | $0$   | $0$   | $0$   | $-2$  |
| 2   | $\mathtt{\_f}:5$   | $0$                | $0$                | $1$   | $0$   | $0$   | $0$   | $-5$  |
| 3   | $\mathtt{\_i}:-47$ | $0$                | $0$                | $1$   | $0$   | $0$   | $0$   | $47$  |
| 4   | $\mathtt{\_d}:3$   | $0$                | $0$                | $1$   | $0$   | $0$   | $0$   | $-3$  |
| 5   | $\mathtt{\_h}:13$  | $\mathtt{\_i}:-47$ | $\mathtt{\_j}:-34$ | $1$   | $1$   | $-1$  | $0$   | $0$   |
| 6   | $\mathtt{\_a}:1$   | $\mathtt{\_a}:1$   | $\mathtt{\_c}:1$   | $0$   | $0$   | $-1$  | $1$   | $0$   |
| 7   | $\mathtt{\_c}:1$   | $\mathtt{\_d}:3$   | $\mathtt{\_e}:3$   | $0$   | $0$   | $-1$  | $1$   | $0$   |
| 8   | $\mathtt{\_e}:3$   | $\mathtt{\_g}:10$  | $\mathtt{\_h}:13$  | $1$   | $1$   | $-1$  | $0$   | $0$   |
| 9   | $\mathtt{\_b}:2$   | $\mathtt{\_f}:5$   | $\mathtt{\_g}:10$  | $0$   | $0$   | $-1$  | $1$   | $0$   |

Notice that the constraints $[0,4]$ are the constants and input wires,
whilst $[5,9]$ are the gates.

We can then form vectors from the columns of the table. e.g. $\vec{a} =
(a_0, a_1, \dots a_9)$

These vectors along with indices $\vec{i} = (0, \cdots, 9)$ are then used to
interpolate into a polynomial. Naively we could use lagrange interpolation
from the set of evaluations.

$$A(X) = \mathrm{lagrange}(\{ (\vec{i}_j, \vec{a}_j ) \mid j \in [9] \})$$

However, for optimization purposes, we will use FFT interpolation instead.

### Optimization: FFT Interpolation and Roots of Unity

To use FFT to compute the Plonk circuit polynomials, we will need to understand
the roots of unity $\o$ of a field $\Fb$.

The roots of unity $\o$ are generators to a cyclic subgroup $G$ of the
multiplicative group of a finite field $\Fb$.

- $H \subseteq \Fb$
- $H = \{ 1, \o^1, \o^2, \dots, \o^{n-1} \}$
- where $\o^n = 1$

$n$ then is called the order of the root of unity $\o$. Depending on the
curve, the roots of unity are only available to specific orders. In our
case of using the Pallas curve, the orders are of $2^k$ up to $k \in [2,
32]$. However, to optimize computation we should pick the tightest bound that
is able to express all gate constraints. In our previous example with $9$
the closest bound would be $2^4 = 16$. i.e. $\o^{16} = 1$.

Thus instead of $[9]$ we would use $H = \{\o^0, \cdots, \o^9\} \subseteq H$
to interpolate the polynomials.

$$A(X) := \mathrm{fft}(\{ (\o^i, a_i) \mid \o^i \in H \})$$

So, $A(\o^i) = a_i$. Thus our Gate Constraint system is:

$$F_{GC}(X) = A(X) Q_l(X) + B(X) Q_r(X) + C(X) Q_o(X) + A(X) B(X) Q_m(X) + Q_c(X)$$

Such that $\forall i \in [ \; |H| \; ] : F(\o^i) = 0$.

### The Protocol

\begin{algorithm}[H]
\caption*{\textbf{Plonk Gate Constraints Protocol:} Protocol for proving the Plonk Gate Constraints}
\textbf{Inputs} \\
  \Desc{$\vec{R} \in \Fb^n \to \Fb^m$}{The public circuit R} \\
  \Desc{$\vec{x} \in \Fb^n$}{The private input to the circuit R} \\
\textbf{Preprocessed Inputs} \\
  \Desc{$Q_l(X), Q_r(X), Q_o(X),$}{} \\
  \Desc{$Q_m(X), Q_c(X) \in \Fb_{\leq d}[X]$}{The public selector polynomials.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State The prover $P$ computes the three polynomials $A(X), B(X), C(X)$ s.t. for $i \in [n]$:
    \Statex \algind $A(\o^i) = x_{a_i}, B(\o^i) = x_{b_i}, C(\o^i) = x_{c_i}$
  \State $P$ creates commitments to $A(X), B(X), C(X)$ and send them to the verifier $V$:
    \Statex \algind $C_A = \PCCommit(A(X), d, \bot), C_B = \PCCommit(B(X), d, \bot), C_C = \PCCommit(C(X), d, \bot)$
  \State $V$ checks that $\forall a \in H$: 
    \Statex $F_{GC}(X) = A(X) Q_l(X) + B(X) Q_r(X) + C(X) Q_o(X) + A(X) B(X) Q_m(X) + Q_c(X)$
    \Statex $F_{GC}(a) \meq 0$
  \State Using the Vanishing Argument Protocol, $P$ creates commitment, $C_T$, to $T(X)$, and sends it to the verifier $V$:
    \Statex \algind $T = \frac{F_{GC}(X)}{Z_s(X)}, C_T = \PCCommit(T, d, \bot)$
  \State $V$ responds with challenge $\xi$:
  \State $P$ creates opening proofs to $A(\xi), B(\xi), C(\xi), T(\xi)$, and sends them to the verifier $V$ along with the evaluations:
    \Statex \algind $\pi_{A} = \PCOpen(A, C_A, d, z, \bot)$
    \Statex \algind $\pi_{B} = \PCOpen(B, C_B, d, z, \bot)$
    \Statex \algind $\pi_{C} = \PCOpen(C, C_C, d, z, \bot)$
    \Statex \algind $\pi_{T} = \PCOpen(T, C_T, d, z, \bot)$
  \State $V$ checks:
    \Statex \algind $F_{GC}(\xi) := A(\xi) Q_l(\xi) + B(\xi) Q_r(\xi) + C(\xi) Q_o(\xi) + A(\xi) B(\xi) Q_m(\xi) + Q_c(\xi)$
    \Statex \algind $F_{GC}(\xi) - T(\xi) \cdot Z_s(\xi) = 0$
    \Statex \algind $\PCCheck(C_T, d, \xi, T(\xi))$
    \Statex \algind $\PCCheck(C_A, d, \xi, F_A(\xi))$
    \Statex \algind $\PCCheck(C_B, d, \xi, F_B(\xi))$
    \Statex \algind $\PCCheck(C_C, d, \xi, F_C(\xi))$
  \end{algorithmic}
\end{algorithm}

## Copy Constraints

The above section only deals with the Gate Constraints, not the Copy
Constraints. For these types of constraints we wish to show equality of
different wires, we do so by using the _Plonk Copy Constraints Protocol_
from the paper.

In a similar spirit to the gate constraint check we will construct a $F_{CC}(X)$ polynomial that models the copy constraint check and use the same vanishing argument protocol on it to verify. We will begin by first constructing the index permutation polynomial $S_\sigma$, which will require prerequisites as follows.

### Cosets

Gate constraints: constants, inputs and gates, are identified
with indices $\o^i \in H$. However, to distinguish between the polynomial
evaluations; wires of the gates $A(\o^i), B(\o^i), C(\o^i)$, we need to
introduce two new cosets.

$$
\begin{aligned}
  k_1 &\not\in H \\
  H_1 &= \{ k_1 \o^i \mid \o^i \in H \} \\
  k_2 &\not\in H \cup H_1 \\
  H_2 &= \{ k_2 \o^i \mid \o^i \in H \} \\
  H' &= H \cup H_1 \cup H_2
\end{aligned}
$$

Thus, our new distinct indices now map to wires as follows:

$$
\begin{aligned}
  \o^i &\mapsto A(\o^i) = a_i \\
  k_1 \o^i &\mapsto B(\o^i) = b_i \\
  k_2 \o^i &\mapsto C(\o^i) = c_i \\
\end{aligned}
$$

We will define partitions of wires $\mathcal{T}$ such that wires that are
connected belong to the same partition.

### Example: Copy Constraints in a Constraint System

Refer to the table of evaluations in our example circuit in the definition
of the constraint system. We have conveniently notated unique identifiers
to wires. We can thus compute a partition of wires as follows:

| Identifier     | Wires           | Partition of Indices                                              |
| -------------- | --------------- | ----------------------------------------------------------------- |
| $\mathtt{\_a}$ | $a_0, a_6, b_6$ | $\mathcal{T}_\mathtt{\_a} = ( \o^0, \o^6, k_1 \o^6 )$ |
| $\mathtt{\_b}$ | $a_1, a_9$      | $\mathcal{T}_\mathtt{\_b} = ( \o^1, \o^9 )$           |
| $\mathtt{\_c}$ | $a_7, c_6$      | $\mathcal{T}_\mathtt{\_c} = ( \o^7, k_2 \o^6 )$       |
| $\mathtt{\_d}$ | $a_4, b_7$      | $\mathcal{T}_\mathtt{\_d} = ( \o^4, k_1 \o^7 )$       |
| $\mathtt{\_e}$ | $a_8, c_7$      | $\mathcal{T}_\mathtt{\_e} = ( \o^8, k_2 \o^7 )$       |
| $\mathtt{\_f}$ | $a_2, b_9$      | $\mathcal{T}_\mathtt{\_f} = ( \o^2, k_1 \o^9 )$       |
| $\mathtt{\_g}$ | $b_8, c_9$      | $\mathcal{T}_\mathtt{\_g} = ( k_1 \o^8, k_2 \o^9 )$   |
| $\mathtt{\_h}$ | $a_5, c_8$      | $\mathcal{T}_\mathtt{\_h} = ( \o^5, k_2 \o^8 )$       |
| $\mathtt{\_i}$ | $a_3, b_5$      | $\mathcal{T}_\mathtt{\_i} = ( \o^3, k_1 \o^5 )$       |
| $\mathtt{\_j}$ | $c_5$           | $\mathcal{T}_\mathtt{\_j} = ( k_2 \o^5 )$             |

### Copy Constraints: Index Permutation Polynomial

We introduce the index permutation polynomial $S_\sigma : H' \to H'$. Think
of it as a map or pointer from one wire to another. The trivial permutation
is the identity permutation, meaning it models pointing to itself.

$$
S_{ID}(X) = \begin{cases}
  i + 1 & X = \o^i \\
  n + i + 1 & X = k_1 \o^i \\
  2n + i + 1 & X = k_2 \o^i \\
\end{cases}\\
$$

Note that the index arguments are in $H'$, but outputs in $[3n]$.

A copy constraint $S_\s$ is an index permutation polynomial such that it models
a transitive closure of equal wires for each partition of $\mathcal{T}$. The
transitive closure is defined by asserting an ordering in the partition and
forming a loop.

Let $\mathcal{T}_x$ be an arbitrarily ordered partition of $N+1$ elements
where $N \geq 0$.

$$
\begin{aligned}
  \mathcal{T}_x &= ( X_0, X_1, \cdots X_N ) \\
  S_\sigma(X_i \in \mathcal{T}_x) &= \begin{cases}
    S_{ID}(X_{i+1}) & \text{if } i < N \\
    S_{ID}(X_0) & \text{if } i = N \\
  \end{cases}
\end{aligned}
$$

### Example: Copy Constraints Evaluations

More concretely for the partition $\mathcal{T}_\mathtt{\_a} = ( \o^0,
\o^6, k_1 \o^6 )$ we have the following looping sequence when applying
$S_\sigma$:

$$
\begin{aligned}
S_\s (\o^0) &= 6 + 1 \mapsto \o^0 \to \o^6 \\
S_\s (\o^6) &= 9 + 6 + 1 \mapsto \o^6 \to k_1 \o^6 \\
S_\s (k_1 \o^6) &= 0 + 1 \mapsto k_1 \o^6 \to \o^0 \\
\end{aligned}
$$

For our example circuit, the copy constraints are defined as follows:

| Gate Index | Maps out of $A(X)$          | Maps out of $B(X)$              | Maps out of $C(X)$              |
| ---------- | --------------------------- | ------------------------------- | ------------------------------- |
| $\o^0$     | $S_\sigma(\o^0) = 6 + 1$     | -                               | -                               |
| $\o^1$     | $S_\sigma(\o^1) = 9 + 1$     | -                               | -                               |
| $\o^2$     | $S_\sigma(\o^2) = 9 + 9 + 1$ | -                               | -                               |
| $\o^3$     | $S_\sigma(\o^3) = 9 + 5 + 1$ | -                               | -                               |
| $\o^4$     | $S_\sigma(\o^4) = 9 + 7 + 1$ | -                               | -                               |
| $\o^5$     | $S_\sigma(\o^5) = 18 + 8 + 1$ | $S_\sigma(k_1 \o^5) = 3 + 1$     | $S_\sigma(k_2 \o^5) = 18 + 5 + 1$ |
| $\o^6$     | $S_\sigma(\o^6) = 9 + 6 + 1$ | $S_\sigma(k_1 \o^6) = 0 + 1$     | $S_\sigma(k_2 \o^6) = 7 + 1$     |
| $\o^7$     | $S_\sigma(\o^7) = 18 + 6 + 1$ | $S_\sigma(k_1 \o^7) = 4 + 1$     | $S_\sigma(k_2 \o^7) = 8 + 1$     |
| $\o^8$     | $S_\sigma(\o^8) = 18 + 7 + 1$ | $S_\sigma(k_1 \o^8) = 18 + 9 + 1$ | $S_\sigma(k_2 \o^8) = 5 + 1$     |
| $\o^9$     | $S_\sigma(\o^9) = 1 + 1$     | $S_\sigma(k_1 \o^9) = 2 + 1$     | $S_\sigma(k_2 \o^9) = 9 + 8 + 1$ |

Notationally however, we split the copy constraints to
polynomials $S_{\sigma a}, S_{\sigma b}, S_{\sigma c}$ for each coset.

$$
S_\sigma(X \in H') = \begin{cases}
  S_{\sigma a}(X) & X \in H \\
  S_{\sigma b}(X') & k_1 X' = X \in H_1 \\
  S_{\sigma c}(X') & k_2 X' = X \in H_2 \\
\end{cases}
$$

**Permuted Wire Polynomials**

Let the permuted wire polynomial of $A(X)$ be notated as $A'(X)$. Then we
have the following.

$$
A'(X) = \begin{cases}
  A(\o^i) & i + 1 = S_{\sigma a}(X) \in [3n] \\
  B(\o^i) & n + i + 1 = S_{\sigma b}(X) \in [3n] \\
  C(\o^i) & 2n + i + 1 = S_{\sigma c}(X) \in [3n] \\
\end{cases}
$$

Concretely, let's take the partition for $\mathcal{T}_\mathtt{\_a} =
( \o^0, \o^6, k_1 \o^6 )$ where $a_0 = 1, a_6 = 1, b_6 = 1$
as an example. Note that since they are connected wires, it is justified
why their values are equal. Moreover, we can see the following:

$$
  S_{\sigma a}(\o^6) = 16 = n + 6 + 1 = \implies A'(\o^6) = B(\o^6) = 1
$$

Despite $A$ and $A'$ being distinct polynomials, are equal at evaluations
of the coset $H'$ they are always equal if the connected wires in the same
partition have the same values, which they should if it is a valid circuit.

**Applying the permutation argument to Plonk**

To verify the copy constraint we would need to construct some polynomial that zeroes as well.

We first construct $f', g'$ from $f,g$. Concretely we will use $f(X) = A(X)$ and $g(X)
= A'(X)$. However, as explored before $A$ and $A'$ have equal evaluations
in $H$. Since these are our evaluations of interest, we can use $A(X)$
in constructing both $f', g'$ where $f(X) = g(X) = A(X)$

$$
\begin{aligned}
f'_A(X) &:= \left(f(X) + \b \cdot S_{ID}(X) + \g \right) \\
      &= \left(A(X) + \b \cdot S_{ID}(X) + \g \right) \\
g'_A(X) &:= \left(g(X) + \b \cdot S_{\s}(X) + \g \right) \\
      &= \left(A(X) + \b \cdot S_{\s}(X) + \g \right) \\
\end{aligned}
$$

We then construct the permutation polynomial as follows:

$$Z(\o^1) = 1$$
$$Z(\o^i) = \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)}$$
$$Z(\o^i)= \prod_{1 \leq j < i} \frac{f'_A(\o^j) f'_B(\o^j) f'_C(\o^j)}{g'_A(\o^j) g'_B(\o^j) g'_C(\o^j)} $$
$$Z(\o^i)= \prod_{1 \leq j < i} \frac{(A(\o^j) + \b \cdot S_{ID a}(\o^j) + \g)(B(\o^j) + \b \cdot S_{ID b}(\o^j) + \g)(C(\o^j) + \b \cdot S_{ID c}(\o^j) + \g)}{(A(\o^j) + \b \cdot S_{\s a}(\o^j) + \g)(B(\o^j) + \b \cdot S_{\s b}(\o^j) + \g)(C(\o^j) + \b \cdot S_{\s c}(\o^j) + \g)}$$

Notice how the next iteration of the product factors out from the previous

$$
Z(\o^{i + 1}) = Z(\o^i) \frac{f'(\o^i)}{g'(\o^i)}
$$

If we factor out $\o^i$ from the arguments and let it be a variable $a$,
we will have the following:

$$
\begin{aligned}
Z(a \o) &= Z(a) \frac{f'(a)}{g'(a)} \\
g'(a) Z(a \o) &= Z(a) f'(a)\\
\end{aligned}
$$

To check $Z$ we will construct a polynomial $F_{CC}$ that models the check,
this is split into two cases $F_{CC_1}, F_{CC_2}$ where $F_{CC_1}$ is the
base case whereas we define the check $\forall a \in H$ in the inductive
case $F_{CC_2}$.

$$
\begin{aligned}
  F_{CC_1} &= L_1(X)(Z(X) - 1) \\
  F_{CC_2} &= Z(a) f'(a) - g'(a)Z(a \cdot \o) \\
\end{aligned}
$$

The check done for $F_{CC_1}$ simply checks the necessary condition that
$Z(\o) = 1$. This is due to the fact that $L_1(X)$ will only evaluate to
$1$ for $L_1(\o^1)$, hence $L_1(\xi)(Z(\xi) - 1) = Z(\xi) - 1 = 1 - 1 = 0$
and in all other cases $L_1(X)$ evaluates to zero.

The second check for $F_{CC_2}$ ensures that $Z(X)$ was constructed
correctly.

### Plonk Copy Constraints Protocol

Now we can construct the protocol used for Copy Constraints in Plonk:

\begin{algorithm}[H]
\caption*{\textbf{Plonk Copy Constraints Protocol:} Protocol for proving Copy Constraints in Plonk}
\textbf{Inputs} \\
  \Desc{$A(X), B(X), C(X): \Fb_{<d}[X]$}{The wire inputs representing the provers private inputs $\vec{x}$ to the circuit $R$} \\
\textbf{Preprocessed Inputs} \\
  \Desc{$S_{ID}: \Fb_{<d}[X]$}{As defined in the section above} \\
  \Desc{$S_{\s}: \Fb_{<d}[X]$}{As defined in the section above} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State The prover commits to $A(X), B(X), C(X)$:
    \Statex \algind $C_A = \PCCommit(A(X), d, \bot), C_B = \PCCommit(B(X), d, \bot), C_C = \PCCommit(C(X), d, \bot)$
  \State The verifier $V$ chooses random challenge scalars $\b, \g \in_R \Fb$ and sends them to the prover $P$.
  \State Define $f'(X), g'(X)$:
    \Statex \algind $f'(X) = (A(X) + \b \cdot S_{ID a}(X) + \g)(B(X) + \b \cdot S_{ID b}(X) + \g)(C(X) + \b \cdot S_{ID c}(X) + \g)$
    \Statex \algind $g'(X) = (A(X) + \b \cdot S_{\s a}(X) + \g)(B(X) + \b \cdot S_{\s b}(X) + \g)(C(X) + \b \cdot S_{\s c}(X) + \g)$
  \State $P$ computes $Z(X) \in \Fb_{\leq d}[X]$, s.t. $z(\o) = 1$, and $\bar{Z}$:
    \Statex \algind $Z(\o) = \prod_{1 \leq j < i} f'(\o^j) \; / \; g'(\o^j)$ for all $i \in \{ 2, \dots d \}$.
    \Statex \algind $\bar{Z}(a) = Z(a \o)$ for all $a \in H$.
  \State $P$ commits to $Z(X)$:
    \Statex \algind $C_Z = \PCCommit(Z(X), d, \bot)$
  \State $V$ checks that $\forall a \in H$:
    \Statex \algind $F_{CC_1}(X) := L_1(X)(Z(X) - 1)$
    \Statex \algind $F_{CC_2}(X) := Z(X)f'(X) - g'(X)Z(a \cdot \o)$
    \Statex \algind $F_{CC_1}(a) \meq F_{CC_2}(a) \meq 0$
  \State Using the Vanishing Argument Protocol, $P$ creates commitment, $C_T$, to $T(X)$, and sends it to the verifier $V$:
    \Statex \algind $T = \frac{F_{CC_1}(X) + \a F_{CC_2}(X)}{Z_s(X)}, C_T = \PCCommit(T, d, \bot)$
  \State $V$ responds with challenge $\xi$:
  \State $P$ creates opening proofs to $A(\xi), B(\xi), C(\xi), T(\xi)$, and sends them to the verifier $V$ along with the evaluations:
    \Statex \algind $\pi_{A} = \PCOpen(A, C_A, d, \xi, \bot)$
    \Statex \algind $\pi_{B} = \PCOpen(B, C_B, d, \xi, \bot)$
    \Statex \algind $\pi_{C} = \PCOpen(C, C_C, d, \xi, \bot)$
    \Statex \algind $\pi_{Z} = \PCOpen(Z, C_Z, d, \xi, \bot)$
    \Statex \algind $\pi_{\bar{Z}} = \PCOpen(Z, C_Z, d, \xi \cdot \o, \bot)$
    \Statex \algind $\pi_{T} = \PCOpen(T, C_T, d, \xi, \bot)$
  \State $V$ checks:
    \Statex \algind $f_\xi := (A(\xi) + \b \cdot S_{ID a}(\xi) + \g)(B(\xi) + \b \cdot S_{ID b}(\xi) + \g)(C(\xi) + \b \cdot S_{ID c}(\xi) + \g)$
    \Statex \algind $g_\xi := (A(\xi) + \b \cdot S_{\s}(\xi) + \g)(B(\xi) + \b \cdot S_{\s}(\xi) + \g)(C(\xi) + \b \cdot S_{\s}(\xi) + \g)$
    \Statex \algind $F_{CC_1}(\xi) := L_1(\xi)(Z(\xi) - 1)$
    \Statex \algind $F_{CC_2}(\xi) := Z(\xi)f'_{\xi} - g'_{\xi}Z(\xi \cdot \o)$
    \Statex \algind $F_{CC_1}(\xi) + \a F_{CC_2}(\xi) - T(\xi) \cdot Z_s(\xi) \meq 0$
    \Statex \algind $\PCCheck(C_T, d, \xi, T(\xi), \pi_T) \meq \top$
    \Statex \algind $\PCCheck(C_A, d, \xi, A(\xi), \pi_A) \meq \top$
    \Statex \algind $\PCCheck(C_B, d, \xi, B(\xi), \pi_B) \meq \top$
    \Statex \algind $\PCCheck(C_C, d, \xi, C(\xi), \pi_C) \meq \top$
    \Statex \algind $\PCCheck(C_Z, d, \xi, Z(\xi), \pi_Z) \meq \top$
    \Statex \algind $\PCCheck(C_Z, d, \xi \o, Z(\xi \cdot \o), \pi_{\bar{Z}}) \meq \top$
\end{algorithmic}
\end{algorithm}

## The Unrolled Protocol

Combining the Plonk Copy Constraints Protocol and the Plonk Gate Constraints Protocol,
we get the desired protocol that proves the claim presented in the beginning:

**The Claim:** "I know private inputs $\vec{x} \in \Fb^n$ s.t. when given
a public circuit $R$, then $R(\vec{x}) = \vec{y} \in \Fb^m$"

The full unrolled protocol, combining the Plonk Permutation Protocol and
the Plonk Gate Constraints Protocol can be seen below:

\begin{algorithm}[H]
\caption{
  \textbf{Full Plonk Protocol:} Protocol for proving that a private input
  $\vec{x}$ evaluated on a public circuit $R$ outputs $\vec{y}$
}
\textbf{Inputs} \\
  \Desc{$\vec{R} \in \Fb^n \to \Fb^m$}{The public circuit R} \\
  \Desc{$\vec{x} \in \Fb^n$}{The private input to the circuit R} \\
\textbf{Preprocessed Inputs} \\
  \Desc{$Q_l(X), Q_r(X), Q_o(X),$}{} \\
  \Desc{$Q_m(X), Q_c(X) \in \Fb_{\leq d}[X]$}{The public selector polynomials.} \\
  \Desc{$S_{ID}: \Fb_{<d}[X]$}{Defined such that $S_{ID}(\o^i) = i$ for each $i \in [n]$} \\
  \Desc{$S_{\s}: \Fb_{<d}[X]$}{Defined such that $S_{\s}(\o^i) = \s(i)$ for each $i \in [n]$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State The prover $P$ computes the three polynomials $A(X), B(X), C(X)$ s.t. for $i \in [n]$:
    \Statex \algind $A(\o^i) = x_{a_i}, B(\o^i) = x_{b_i}, C(\o^i) = x_{c_i}$
  \State $P$ creates commitments to $A(X), B(X), C(X)$ and send them to the verifier $V$:
    \Statex \algind $C_A = \PCCommit(A(X), d, \bot)$
    \Statex \algind $C_B = \PCCommit(B(X), d, \bot)$
    \Statex \algind $C_C = \PCCommit(C(X), d, \bot)$
  \State The verifier $V$ chooses random challenge scalars $\b, \g \in_R \Fb$ and sends them to the prover $P$.
  \State The prover defines $\bar{f}(X), \bar{g}(X)$.
  \State $P$ computes $Z(X) \in \Fb_{<n}[X]$, s.t. $z(\o) = 1$:
    \Statex \algind $Z(\o) = \prod_{1 \leq j < i} \bar{f}(\o^j) \; / \; \bar{g}(\o^j)$ for $i \in \{ 2, \dots n \}$.
    \Statex \algind $\bar{Z}(a) = Z(a \o)$ for all $a \in H$.
  \State $P$ commits to $Z(X), \bar{f}(x), \bar{g}(x)$:
    \Statex \algind $C_Z = \PCCommit(Z(X), d_Z, \bot)$
  \State $V$ checks that $\forall a \in H$:
  \State Using the Vanishing Argument Protocol, $P$ creates commitment, $C_T$, to $T(X)$, and sends it to the verifier $V$:
    \Statex \algind $T = \frac{F_{GC}(X) + \a F_{CC_1}(X) + \a^2 F_{CC_2}(X)}{Z_s(X)}, C_T = \PCCommit(T, d, \bot)$
  \State $V$ responds with challenge $\xi$:
  \State $P$ creates opening proofs to $A(\xi), B(\xi), C(\xi), T(\xi), Z(\xi), Z(\xi \cdot \o)$, and sends them to the verifier $V$ along with the evaluations:
    \Statex \algind $\pi_{A} = \PCOpen(A, C_A, d, \xi, \bot)$
    \Statex \algind $\pi_{B} = \PCOpen(B, C_B, d, \xi, \bot)$
    \Statex \algind $\pi_{C} = \PCOpen(C, C_C, d, \xi, \bot)$
    \Statex \algind $\pi_{Z} = \PCOpen(Z, C_Z, d, \xi, \bot)$
    \Statex \algind $\pi_{\bar{Z}} = \PCOpen(Z, C_Z, d, \xi \cdot \o, \bot)$
    \Statex \algind $\pi_{T} = \PCOpen(T, C_T, d, \xi, \bot)$
  \State $V$ checks:
    \Statex \algind $f_\xi := (A(\xi) + \b \cdot S_{ID a}(\xi) + \g)(B(\xi) + \b \cdot S_{ID b}(\xi) + \g)(C(\xi) + \b \cdot S_{ID c}(\xi) + \g)$
    \Statex \algind $g_\xi := (A(\xi) + \b \cdot S_{\s}(\xi) + \g)(B(\xi) + \b \cdot S_{\s}(\xi) + \g)(C(\xi) + \b \cdot S_{\s}(\xi) + \g)$
    \Statex \algind $F_{GC}(\xi) := A(\xi) Q_l(\xi) + B(\xi) Q_r(\xi) + C(\xi) Q_o(\xi) + A(\xi) B(\xi) Q_m(\xi) + Q_c(\xi)$
    \Statex \algind $F_{CC_1}(\xi) := L_1(\xi)(Z(\xi) - 1)$
    \Statex \algind $F_{CC_2}(\xi) := Z(\xi)f'_{\xi} - g'_{\xi}Z(\xi \cdot \o)$
    \Statex \algind $F_{GC}(\xi) + \a F_{CC_1}(\xi) + \a^2 F_{CC_2}(\xi) - T(\xi) \cdot Z_s(\xi) \meq 0$
    \Statex \algind $\PCCheck(C_A, d, \xi, A(\xi), \pi_A) \meq \top$
    \Statex \algind $\PCCheck(C_B, d, \xi, B(\xi), \pi_B) \meq \top$
    \Statex \algind $\PCCheck(C_C, d, \xi, C(\xi), \pi_C) \meq \top$
    \Statex \algind $\PCCheck(C_Z, d, \xi, Z(\xi), \pi_Z) \meq \top$
    \Statex \algind $\PCCheck(C_Z, d, \xi \o, Z(\xi \cdot \o), \pi_{\bar{Z}}) \meq \top$
    \Statex \algind $\PCCheck(C_T, d, \xi, T(\xi), \pi_T) \meq \top$
  \end{algorithmic}
\end{algorithm}

There is one more optimization that Plonk uses that is omitted in the above
protocol, namely the optimization by Mary Maller, that reduces the required
field elements used in the proof, reducing overall proof size. This
optimization was omitted in the implementation due to time constraints.
As such we will also not discuss it here.

Unlike the unrolled protocol in the paper, the above protocol does not add
blinding. This is largely due to us having to unroll the protocol ourselves
from the paper, as their unrolled protocol was highly specific to the
underlying KZG scheme. It's not really a problem since:

a. Blinding in Plonk is optional, it adds a small overhead which may not be
   preferable if you only need a SNARK, not a ZK-SNARK.
b. Plonk's version of blinding is not fully sufficient for the Discrete Log
   based PCS used in our implementation[^blinding]. We would have liked to
   add said efficient blinding, but was unable to in time.

As for succinctness, our unrolled protocol should still preserve succinctness
as the only real optimization omitted, compared to the unrolled Plonk
specification in the paper, is the one by Mary Maller.

## PCS Security Implications

Although the implementation of the Polynomial Commitment Scheme used is not
part of our implementation, its security implications for our implementation
still holds. The underlying cryptographic assumptions needed for Plonk rely heavily on
the PCS used, we will therefore give a brief analysis of them.

As mentioned in the introduction, Plonk relies on a Trusted Setup in order
to achieve its soundness guarantees, since it's defined using a PCS based on KZG
commitments. The security of the KZG PCS require a trusted setup while our PCS
relies on an untrusted setup.

A trusted setup generates a _Structured Reference String_ (SRS), that has
a certain internal structure that's needed for some operations in the KZG
commitment scheme. Specifically for KZG, given abelian groups of prime order
$q$, $\Gb_1, \Gb_2, \Gb_t$, and uniformly chosen generators $G_1 \in \Gb_1,
G_2 \in \Gb_2$ the SRS is:

$$SRS = \{ G_1, \a \cdot G_1, \a^2 \cdot G_1, \dots, \a^t \cdot G_1, G_2, \a \cdot G_2 \}$$

Where $t$ represents the maximum degree of committed polynomials. If any
adversary knows $\a$ they can break soundness. It is exactly the structure
between elements of the above set that requires a trusted setup. The setup
is universal and updatable which means that one or more parties participate
in a setup protocol and if at least a single party is honest, only then will
Plonk achieve its soundness.

In contrast, the polynomial commitment scheme is based on bulletproofs where
the setup creates a _Uniform Random String_[^srsurs] of the form:

$$URS = \{ \a_1 \cdot G, \a_2 \cdot G, \dots, \a_t \cdot G \}$$

Where $G$ is a generator for the abelian group $\Gb$, $\vec{\a}$ are uniformly
random sampled and each element of the above set represents a generator. The
polynomial commitment scheme then achieves soundness given that an adversary
does not know the values of $\vec{\a}$. Getting the values of $\vec{a}$
from $URS$ is equivalent to breaking the Discrete Log problem which is
assumed to be hard.

To generate the URS transparently, we can use a collision resistant
hash function $\Hc : \Bb \to G$[^generators], we can generate the values
using a genesis string $s$:

$$URS = \{ \Hc(s \cat 1), \Hc(s \cat 2), \dots, \Hc(s \cat t) \}$$

It should be noted that although 

[^srsurs]: The specific terminology of SRS and URS is defined in the [ZKProof Community Reference](https://docs.zkproof.org/pages/reference/reference.pdf)
[^generators]: $\Hc$ must produce generators for $\Gb$, but the Pallas curve used, all elements (barring the identity element $\Oc$ are generators).

# Implementation Details

## Program Architecture

To allow an ergonomic experience constructing circuits we have defined the
following structs

- `AbstractCircuit<L>`
- `Wire<L>`
- `Circuit<L>`

Where `L` is an const generic of the number of input wires to the circuit.

The `AbstractCircuit` has a running unique identifier for wires and
gates. When building new gates, it first queries its state if a wire with
the same computation exists. Thus this memoization technique reduces the
number of wires and constraints required.

The first step is to generate the input wires to the circuit. The following
is how we would generate two input wires:

```rust
  let [x, y] = &AbstractCircuit::<2>::build();
```

The `Wire` struct has a reference to the circuit it belongs to. Thus we are
able to implement the traits `Add, Mul, Sub` to `Wire` and field elements
to construct circuits since the operators now correspond to function calls
to create new constraints.

Thus the following is how a trivial circuit might look like:

```rust
  let output_wire = 3 * (x + y);
```

To start performing the protocol we would need to instantiate the input
wires with their private inputs. This is done by concretizing it to a
`Circuit` struct.

```rust
  let circuit = output_wire.input([1, 2]);
```

Here we have instantiate `x` with `1` and `y` with `2`.

In the concrete `Circuit` struct is where the methods for the prover and
verifier are defined. Thus the protocol simply looks as follows:

```rust
  let snark = circuit.prove(&mut ThreadRng::default());
  assert!(circuit.verify(snark));
```

## Polynomial Degree Bound Requirements in PCS

The Polynomial Commitment Scheme requires that the degree of the polynomials
is some $2^n - 1$. However our polynomials are not guaranteed to always have
a degree of such a form. Thus the PCS allows us to define a degree bound to
automatically pad the polynomial. We can simply compute the degree bound by
using rust's `next_power_of_two` method.

However when we open the polynomial we would have to provide the same degree
bound. Thus to ease bookkeeping, we created a wrapper struct `CommitData`
that stores the degree bound and the commitment.

```rust {.numberLines}
  /// Succinct PCDL commit call
  pub fn commit(poly: &Poly) -> CommitData {
      let mut d = poly.degree().next_power_of_two() - 1;
      if poly.degree() >= d {
          d += 2;
          d = d.next_power_of_two() - 1;
      }
      CommitData {
          d,
          pt: pcdl::commit(&poly.poly, d, None),
      }
  }
```

Thus committing, opening and checking polynomials is very ergonomic in
our code.

```rust
  let F_xi = &F.evaluate(xi);
  let F_com = &commit(F);
  let F_pi = &open(rng, F, F_com, xi);
```

However commitments with large degree bounds are expensive to compute. Our
circuit example has 9 gates so theoretically it should be possible to define
a polynomial with such a degree.

But by using FFT interpolation we ended up with a polynomial of degree of about
16. Applying a blinding factor (enabling our protocol to be zero knowledge)
to the polynomial brings it to 17. Thus our degree bound required to commit
it is 31.

This large difference in degree bounds presents an opportunity for further
optimization.

The team at Mina protocol has proposed a solution
[Kimchi](https://o1-labs.github.io/proof-systems/plonk/zkpm.html) that
enables such an optimization.

However in our final implementation we have omitted the use of blinding
factors. Our implementation thus is not zero knowledge.

## Concluding Implementation

We have successfully implemented the pre-computation polynomials. These are
$A(X), B(X), C(X)$ and $Q_l(X), Q_r(X), Q_o(X), Q_m(X), Q_c(X)$ and also the copy
constraints split into three $S_{\s a}(X), S_{\s b}(X), S_{\s c}(X)$.

```rust
  let [fa, fb, fc, ql, qr, qo, qm, qc] = &self.wire_polynomials();
  let (k1, k2) = &self.compute_coset(rng);
  let [sa, sb, sc] = &self.copy_constraints(*k1, *k2);
```

We used the library `merlin` that models the Fiat-Shamir heuristic using
a running transcript. We can then conveniently append scalars and points
(or commitments) to the transcript as follows

```rust
  transcript.append_scalar(b"my_scalar", scalar);
  transcript.append_point(b"my_point", point);
  transcript.append_comm(b"my_commitment", commit);
```

We can then compute challenge scalars as follows:

```rust
  let alpha = transcript.challenge_scalar(b"alpha");
```
We then computed the polynomials $F_{GC}, F_{CC_1}, F_{CC_2}$ and their respective $T$ polynomials for the vanishing argument protocol.

Computing $F_{GC}$ and $F_{CC_1}$ were successful except for $F_{CC_2}$. The culprit was the definition of $f', g'$. We found that $f'(\xi) \neq f'_a(\xi) f'_b(\xi) f'_c(\xi)$ despite holding for $f(\o^i) = f'_a(\o^i) f'_b(\o^i) f'_c(\o^i)$. Thus, since the verifier needs to compute $f'(\xi)$ from valuations of $f'_a(\xi), f'_b(\xi), f'_c(\xi)$ from $A(\xi), B(\xi), C(\xi), \beta, \gamma$ we were unable to complete the protocol.

We conjecture that it could be of either two reasons:

- The polynomial $f'(X), g'(X)$ were not correctly defined
- The quotient polynomial does not successfully apply the vanishing argument on $F_{CC_2}$

\newpage

# Conclusion

While we aimed for a more comprehensive implementation of the Plonk protocol,
including the Mary Maller optimization, and possibly including comparing
the performance of the underlying KZG and discrete log commitment schemes,
the complex nature of the Plonk paper limited our ability to do so. Much of
our effort was devoted to unrolling the protocol from its highly abstract
presentation, especially due to its reliance on the underlying KZG

Nevertheless, we are quite proud of the achieved insight into an otherwise
esoteric, yet powerful, area of cryptography and are very satisfied with
our implementation and our understanding of it. The project has also been a
great opportunity to hone our skills in implementing efficient cryptographic
protocols in Rust

# Appendix

## Notation

|                                                                                 |                                                                                                           |
|:--------------------------------------------------------------------------------|:----------------------------------------------------------------------------------------------------------|
| $[n]$                                                                           | Denotes the integers $\{ 1, ..., n \}$                                                                    |
| $[n,m]$                                                                         | Denotes the integers $\{ n, ..., m \}$                                                                    |
| $\maybe{x}{\phi(x)}$                                                            | Returns $x$ if $\phi(x)$ is true, otherwise $\bot$ or errors.|
| $f[x \mapsto y]$ | Updates the partial map $f$ with a new mapping $f(x)=y$.|
| $f[\vec{x} \mapsto \vec{y}]$ | Updates the partial map $f$ with mappings from vectors. |
| $a \in \Fb$                                                                     | A field element in a prime field of order $q$                                                             |
| $\vec{a} \in S^n_q$                                                             | A vector of length $n$ consisting of elements from set $S$                                                |
| $G \in \Eb(\Fb)$                                                                | An elliptic Curve point, defined over field $\Fb$                                                         |
| $\vec{a}$                                                                       | A vector                                                                                                  |
| $(a_1, a_2, \dots, a_n)$                                                        | A vector                                                                                                  |
| $(s..t)$                                                                        | A vector from $s$ to $t-1$, if $t \leq s$ then $()$, and $(..t) = (0..t)$.                   |
| $a \in_R S$                                                                     | $a$ is a uniformly randomly sampled element of $S$                                                        |
| $(S_1, \dots, S_n)$                                                             | In the context of sets, the same as $S_1 \times \dots \times S_n$                                         |
| $\vec{a} \cat \vec{b}$ where $\vec{a} \in \Fb^n_q, \vec{b} \in \Fb^m_q$         | Concatenate vectors to create $\vec{c} \in \Fb^{n+m}_q$.                                                  |
| $a \cat b$ where $a \in \Fb_q$                                                  | Create vector $\vec{c} = (a, b)$.                                                                         |
| $\Bb$                                                                           | Represents a boolean $\{ \top, \bot \}$                                                                   |
| $\Option(T)$                                                                    | $\{ T, \bot \}$                                                                                           |
| $\Result(T, E)$                                                                 | $\{ T, E \}$                                                                                              |
| $\EvalProof$                                                                    | The evaluation proof produced by the PCS                                                                  |

Note that the following are isomorphic $\Bb \iso \Option(\top) \iso
\Result(\top, \bot)$, but they have different connotations.

\newpage

# Protocol

\begin{algorithm}[H]
\caption*{
  \textbf{Surkål:} a plonkish NARK protocol.
}
\textbf{Inputs} \\
  \Desc{$f: \Fb^n_q \to \Fb^m_q$}{The program being proved.} \\
  \Desc{$\vec{x} \in \Fb^n_q$}{The possibly private input to the program $f$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State Precompute the corresponding entry of the circuit relation; $x\ R\ w$:
    \Statex \algind $(x,w) \gets \mathrm{circuit}(\mathrm{trace}(\mathrm{arithmetize}(f), \vec{x}))$ 
  \State The prover $P$ computes the proof:
    \Statex \algind $\pi \gets P(x,w)$
  \State The verifier $V$ then checks:
    \Statex \algind $V(x, \pi)$
  \end{algorithmic}
\end{algorithm}

## Arithmetize

Arithmetize turns a program $f$ into an abstract circuit $\wave{f}$, which is a one-to-many-or-none relation between gates $g$ and output wire id(s) $\wave{y} : \Nb_\bot$ where $\bot$ denotes no output wires. e.g. $(\text{Add}(a,b), c) \in \wave{f}$ corresponds to the circuit $\build{a+b=c}{}{}$. We notate inserting a gate or gadget $f$ to the circuit with $\build{f = \wave{\vec{y}}}{s}{s'}$, $\build{f = \wave{y}}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state from $s$ to $s'$. State has the form $(u, \wave{f})$ where $u$ is the current uuid for wires. Wires annotated as the final output will be appended to $\wave{\vec{Y}}$, i.e. $\build{f=\wave{y}^*}{(\_,\wave{\vec{Y}})}{(\_, \wave{\vec{Y}} \cat \wave{y})}$, which is omitted notationally if unchanged. Gates are primitive operations with $n\geq 0$ fan in inputs and $m \geq 0$ fan out outputs. A circuit is a composition of gadget(s) and/or gate(s). These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \wave{f}$, then $\wave{\vec{y}}$ in $\build{g=\wave{\vec{y}}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

$$
\begin{aligned}
\AbsCirc &= \set{
  \wave{f} \subset \Gate \times \Nb_\bot \middle\vert
  \forall (g,\wave{y}),(h,\wave{y}) \in \wave{f}. \wave{y} \neq \bot \implies g = h
} \\
\Gate^{\wave{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \wave{f} \land h \equiv g
}
\\
\ArithState &= \Nb \times \AbsCirc
\end{aligned}
$$
$$
\begin{array}{rlrl}
\text{out} &: (\Nb_\bot + \AbsCirc \times \Gate) \to \Nb^m &
\text{get} &: \ArithState \to \Gate \to \ArithState \times \Nb^m
\\
\text{out}(\bot) &= () &
\multirow{3}{*}{$\text{get}(u, \wave{f}, g)$} &
\multirow{3}{*}{$=\begin{cases}
    (u, \wave{f}, \text{out}(\wave{f}, h)) & h \in \Gate^{\wave{f}}_g \\
    (\text{put}(g, u, \wave{f}), \text{out}(u)) & \text{otherwise}
  \end{cases}
$}
\\
\text{out}(u) &= (u..u+m) \\
\text{out}(\wave{f}, g)
&= \text{out} \circ \min\left(
  \set{\wave{y} \middle\vert (g,\wave{y}) \in \wave{f}}
\right)
\\ \\
\text{entries}  &: \Gate \to \Nb \to \AbsCirc &
\build{g = \wave{\vec{y}}}{s}{s'} &= \left(\text{get}(s,g) \overset{?}{=} (s', \wave{\vec{y}})\right) 
\\
\text{entries}(g, u) &= \begin{cases}
  \set{(g,\wave{y}) \middle\vert \wave{y} \in \text{out}(u)} & m > 0 \\
  \set{(g,\bot)}                      & m = 0
\end{cases} &
\build{f}{s_1}{s_{k+1}} &= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}} 
\\ \\
\text{put} &: \Gate \to \ArithState \to \ArithState &
\text{arithmetize} &: (\Fb^n_q \to \Fb^m_q) \to \AbsCirc \times \Nb^{m'}
\\
\text{put}(g, u, \wave{f}) &= (
  u + m, \wave{f} \cup \text{entries}(g, u)
) &
\text{arithmetize}(f) &= \maybe{(\wave{f}, \wave{\vec{Y}})}{
  \build{f}{(\text{put}(\text{Input})^n(0,\emptyset), \emptyset)}{(\_, \wave{f}, \wave{\vec{Y}})}
}
\end{array}
$$

## Trace

Trace takes in $(\wave{f}, \wave{\vec{Y}})$ and $\vec{x}$; input values for the program $f$, to compute the values corresponding to $\wave{\vec{Y}}$ recursively via $\Downarrow$, using the gates $g$ in $(g,\wave{y}) \in \wave{f}$. A gate is of the form $(\wave{g}, \wave{\vec{x}}, f)$ where $\wave{g}$ is the gate type id, $\wave{\vec{x}}$ are the vector of input wire ids and $f$ the program $g$ corresponds to that computes the output wires' values.The values are cached by the value map $v$ and are used by $e$ to compute the output $t$.

$$
\begin{array}{rl}
\begin{array}{rl}
\Gate &=
  \text{GateType} \times
  \Nb^n \times
  (\Fb^n_q \to \Fb^m_q)
\\
\text{VMap} &= \Nb \rightharpoonup \Fb_q
\\
\text{State}^T &= \text{VMap} \times T
\\ 
\text{Tab}_{\wave{f}}^T &= (\Nb^m + \Nb) \to \text{State}^T \to \text{State}^T
\\ \\
\text{trace} &:
  T \to
  \text{Tab}_{\wave{f}}^T \to
  \Nb^m \to
  \Fb^n_q \to
  T
\\
\text{trace}^t_e(\wave{\vec{Y}}, \vec{x}) &= \maybe{t'}{
  e \left(\wave{\vec{Y}}, 
    \wave{\vec{Y}}_{\Downarrow e}^{(\bot[(..n) \mapsto \vec{x}], t)}
  \right) = ( \_, t')
}
\end{array}
&
\begin{array}{rl}
{\Downarrow } &:
  \text{Tab}_{\wave{f}}^T \to
  \Nb^l \to
  \text{State}^T \to
  \text{State}^T
\\
\wave{\vec{w}}_{\Downarrow e}^{(v,t)} &=
\begin{cases}\begin{array}{lrl}
  (v,t) & |\wave{\vec{w}}| &= 0\\
  & \wave{\vec{w}} &= \wave{w} \cat \wave{\vec{r}} \\
  \wave{\vec{r}}_{\Downarrow e}^{(v,t)}
  & v(\wave{w}) &\neq \bot \\
  \multirow{6}{*}{$\wave{\vec{r}}_{\Downarrow e}^{s'}$}
  & \wave{f} &\ni (\wave{g}, \wave{\vec{x}}, f, \wave{w}) \\
  & \wave{\vec{x}}^{(v,t)}_{\Downarrow e} &= (v', t') \\
  & \forall i. \vec{x}_i &= v'(\wave{\vec{x}}_i) \\
  & \wave{\vec{y}} &= \text{out}(\wave{f}, \wave{g}, \wave{\vec{x}}, f) \\
  & v'' &= v'[\wave{\vec{y}} \mapsto f(\vec{x})]\\
  & s' &= e(\wave{w}, v'', t')
\end{array}
\end{cases}
\end{array}
\end{array}
$$

Note: $e(\wave{y},s)$ is computing constraints while $\Downarrow$, but $e(\wave{\vec{Y}},s)$ is after $\text{trace}$.

### Gate Constraints

The Gate Constraints is matrix with $M$ rows and $N$ columns where each row has the form of the constraint equation $F_{GC}$. The rows for the matrix are computed by $c$ that takes in a gate type id $\wave{g}$, the input and output values of the gate and returns $k$ rows / constraints. The protocol also populates a set of wire ids involved as $W$, this is used at the end to populate constraints for gates with no output wires; checking if their inputs exists in $W$.

$$
\begin{array}{rl}
\text{Constraint} &= \Fb^{w}_q \to \Fb^{N \times k}_q
\\
\text{gate} &: (\text{GateType} \to \text{Constraint}) \to \text{Tab}^{\mathcal{P}(\Nb) \times\Fb_q^{\_ \times N}}_{\wave{f}}
\\
\text{gate}_c(w, v, W, M_0) &= \begin{cases}
\begin{array}{lrl}
\multirow{5}{*}{$(W', M')$}
& \wave{f} &\ni ((\wave{g}, \wave{\vec{x}}, f), w) \\
& \forall i. \vec{x}_i &= v(\wave{\vec{x}}_i) \\
& \vec{y} &= f(\text{out}(\wave{f}, \wave{g}, \wave{\vec{x}}, f)) \\
& W' & W \cup \set{w} \cup \set{\wave{w} \middle\vert \wave{w} \in \wave{\vec{x}}} \\
& M' &= M_0 \cat c(\wave{g}, \vec{x} \cat \vec{y}) \\
\multirow{3}{*}{$(W, \vec{M}_{|\vec{M}|})$}
& \forall i. \wave{f} &\ni ((\wave{\vec{g}}_i, \wave{\vec{xs}}_i, \_), \bot) \\
& \forall j. \vec{xs}_{i,j} &= v(\wave{\vec{xs}}_{i,j}) \\
& \vec{M}_i &= \vec{M}_{i-1} \cat
\begin{cases}
  c(\wave{\vec{g}}_i, \vec{xs}_i)
    & \forall j. \wave{\vec{xs}}_{i,j}\in W \\
  () & \text{otherwise}
\end{cases}
\end{array}
\end{cases}
\end{array}
$$

### Copy Constraints

permutation matrix (when appending to $M$ you need to track row number, column number to cell id, cell id are quotiented into an equivalence class modulo wire id, then made into an ordered loop, then you can compute the permutation matrix... thus maybe make a function for appen $M$, that internally tracks this.. THIS IS A NEW STATE, i.e. trace state = (v,p) where p is a relation between wire id and (row, col) coordinates)

### Lookup Argument Constraints

- $t$ poly eval thunk
- $f$: get eval corresponding to $(x,y,z)$ when resolve lookup else get 

### Full Surkål Trace

... construct $t$ and $e$ and define $\text{trace} = \text{trace}^t_e$

## Circuit

The gates evaluate their output values as follows:

| $\Gate$             | $f : \Fb^n_q \to$   | $\Fb^m_q$                     | remarks                 |
|:-------------------:|:-------------------:|:-----------------------------:|:------------------------|
| Input               | $()$                | $(x)$                         | from trace              |
| Const$(s,\_)$       | $()$                | $(s)$                         |                         |
| Add$(x,y)$          | $(x,y)$             | $(x+y)$                       |                         |
| Mul$(x,y)$          | $(x,y)$             | $(x \times y)$                |                         |
| Inv$(x)$            | $(x)$               | $(x^{-1})$                    |                         |
| Pow7$(x)$           | $(x)$               | $(x^7)$                       |                         |
| If$(b,x,y)$         | $(b,x,y)$           | $(b ? x : y)$                 |                         |
| Lookup$(T,x,y)$     | $(x,y)$             | $(\maybe{z}{(x,y,z) \in T})$  |                         |
| PtAdd$(P,Q)$        | $(x_P,y_P,x_Q,y_Q)$ | $(x_R, y_R)$                  | Arkworks point add      |
| Poseidon$(a,b,c)$   | $(a,b,c)$           | $(a',b',c')$                  | Mina poseidon 5 rounds  |
| Public$(x)$         | $(x)$               | $()$                          |                         |
| Bit$(b)$            | $(b)$               | $()$                          |                         |
| IsAdd$(x,y,z)$      | $(x,y,z)$           | $()$                          |                         |
| IsMul$(x,y,z)$      | $(x,y,z)$           | $()$                          |                         |
| IsLookup$(T,x,y,z)$ | $(x,y,z)$           | $()$                          |                         |

# Appendix

## Arithmetize Example

Example of the arithmetization of $x^2 + y$ with gates Input, Mul$(a,b)$ and Add$(a,b)$ all with $m=1$:
$$
\begin{aligned}
&\text{arithmetize}((x,y) \mapsto (x^2 + y))
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{
  \build{x^2 + y = z^*}
    {((u, \wave{f}) = \text{put}(\text{Input})^2(0, \emptyset), \emptyset)}
    {(\_, \wave{f}'', (z))}
  }
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \wave{f}, \emptyset)}{(\_, \wave{f}'', (z))}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \wave{f})}{(u', \wave{f}')} \\
  \build{t + y = z^*}{(u', \wave{f}', \emptyset)}{(\_, \wave{f}'', (z))}
\end{array}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{rl}
  \text{get}(u, \wave{f}, \text{Mul}(x,x)) &= (u', \wave{f}', (t)) \\
  \text{get}(u', \wave{f}', \text{Add}(t,y)) &= (\_, \wave{f}'', (z))
\end{array}}
\\ 
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{rl}
  (u+1, \wave{f} \cup \set{(\text{Mul}(x,x), u)}, (u)) &= (u', \wave{f}', (t)) \\
  \text{get}(u', \wave{f}', \text{Add}(t,y)) &= (\_, \wave{f}'', (z))
\end{array}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{
  \text{get}(u+1, \wave{f} \cup \set{(\text{Mul}(x,x))}, \text{Add}(u,y)) = (\_, \wave{f}'', (z))
}
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(x,x) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
  (u, \wave{f}) = \text{put}(\text{Input})^2(0, \emptyset)
}
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
    (u, \wave{f}) = \text{put}(\text{Input}, 1, \set{(\text{Input}, 0)}, \emptyset)
  }
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,1) & u+1
  \end{array}}, (u+1) \right)}
  {(u, \wave{f}) = \left(2, \set{\begin{array}{rl}
    \text{Input} & 0 \\
    \text{Input} & 1
  \end{array}}\right)}
\\
&= \left(\set{\begin{array}{rl}
  \text{Input} & 0 \\
  \text{Input} & 1 \\
  \text{Mul}(0,0) & 2 \\
  \text{Add}(2,1) & 3
\end{array}}, (3)\right)
\end{aligned}
$$

## Wire Reuse in Arithmetize

The equivalence class we use is defined by the associative and commutative properties of the gates. We do not exploit the full algebraic structure induced by all gates, which would yield the optimal equivalence class, i.e. output wires modulo distributivity of multiplication over addition is not reused, as it is beyond the scope of this paper.
