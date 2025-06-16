---
title: Master's Thesis - Investigating feasibility of Halo2 for IVC in Rust
author:
  - Rasmus Kirk Jakobsen - 201907084
  - Abdul Haliq Abdul Latiff - 202303466
geometry: margin=2cm
bibliography: bibliography.bib
---


\newcommand{\maybe}[2]{ \left[ #1 \middle\vert #2 \right]}
\newcommand{\abst}[1]{ \hat{#1} }
\newcommand{\set}[1]{ \left\{ #1 \right\}}
\newcommand{\build}[3]{\left\llbracket #1 \right\rrbracket^{#2}_{#3}}
\newcommand{\AbsCirc}{\text{Circ}}
\newcommand{\Gate}{\text{Gate}}
\newcommand{\AState}{\text{AState}}
\newcommand{\Mono}[1]{\text{Mono}^{#1}}
\newcommand{\MonoC}[1]{\text{MonoC}^{#1}}
\newcommand{\RState}{\text{RState}}
\newcommand{\VMap}{\text{VMap}}
\newcommand{\pto}{\rightharpoonup}

\tableofcontents
\newpage


# Abstract

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

TODO (new abstract)

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

# Security Proofs

# High Level Protocol

# The Protocol

TODO (IVC)

The goal of Plonk is for a prover to convince a verifier of the following
claim:

**The Claim:** "I know the possibly private input output pair of some program $f$"

$$
f(\vec{x}) = \vec{y}
$$

Let's look at a simple arithmetization $\build{3x^2_1 + 5x_2}{}{}$:

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

In the above figure the output of the multiplication gate on the right, $c_2$, should be equal to the value of the input wire of the addition gate, $b_3$.
Plonk enforces this with _copy constraints_.

Moreover, some gates are defined as a lookup table.
The gates of the circuit also have 0 or more fan-in inputs and outputs.
This leads to a good foundation for custom gates.
We also use bulletproofs / discrete log for our polynomial commitment scheme.
This makes our NARK a hyperplonk-ish protocol.
We also introduce an ergonomic way to write circuits that does not require manually populating the trace matrix. Thus, at a high level our NARK protocol is as follows:

\begin{algorithm}[H]
\caption*{
  \textbf{Surkål:} a plonkish NARK protocol.
}
\textbf{Inputs} \\
  \Desc{$f: \Fb^n_q \to \Fb^m_q$}{The program.} \\
  \Desc{$\vec{x} \in \Fb^n_q$}{The possibly private input to the program $f$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State \textbf{let} $(x,w) = \mathrm{circuit} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$ 
  \State $\pi \gets P(x,w)$
  \State \textbf{return} $V(x, \pi)$
  \end{algorithmic}
\end{algorithm}

# General Protocols

## Vanishing Argument


The checks that the verifier makes in Plonk boils down to checking identities of the following form:

$$\forall a \in S : f(a) \meq 0$$

For some polynomial $f(X) \in \Fb_{\leq d}$ and some set $S \subset \Fb$. The subset, $S$, may be much smaller than $\Fb$ as is the case for Plonk where $S = H$. Since we ultimately model the above check with challenge scalars, using the entirety of $\Fb$ should lead to much better security. We therefore end up with the following checks of the following form instead:

$$\forall \xi \in \Fb : F'(\xi) \meq 0$$

Where $S \subset \Fb$ and $F'$ is defined by combining $F$ with a challenge scalar $\a$. Below we present the protocol that lets the verifier query polynomial identities of the form $\forall a \in S : F(s) \meq 0$ using a PCS. For a series of polynomials, $\{ F_1, F_2, \dots, F_k \} \in \Fb_{\leq d}$, we have the following protocol:

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

Due to the factor theorem[^factor-theorem] $z_S(X)$ only divides $f(X)$ if and only if all of $\o \in H : f(\o) = 0$. Then from this the Schwartz-Zippel Lemma[^schwartz-zippel] states that evaluating a nonzero polynomial on inputs chosen randomly from a large enough set is likely to find an input that produces a nonzero output. Specifically it ensures that $Pr[P(\xi)] \leq \frac{deg(P)}{|\Fb|}$. Clearly $\xi \in \Fb$ is a large enough set as $|\Fb| \gg |H|$ and therefore $Pr[P(\xi) | P \neq 0]$ is negligible. Lastly, the evaluation checked depends on the soundness of the underlying PCS scheme used, but we assume that it has knowledge soundness and binding. From all this, we conclude that the above vanishing argument is sound.

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

Note that for the Plonk protocol specifically, $S = H = \{ 1, \o, \o^2, \dots, \o^{n-1} \}$ for the reason that the vanishing polynomial $z_S(X)$ then becomes $z_S(X) = X^n - 1$ because $\o$ is a root of unity of order $n$. This is much more efficient to compute. The $\a$'s are used since we need a linearly independent combination of $f$.

## Batched Evaluation Proofs

If we have $m$ polynomials, $\vec{f}$, that all need to evaluate to zero at the same challenge $\xi$, normally, we could construct $m$ opening proofs, and verify these. We can, however, use the following technique to only create a single opening proofs.

- The prover starts by sending commitments for each $f_i(X)$: $C_{f_i} = \PCCommit(f_i(X), d)$.
- The verifier sends the challenge $\xi$.
- The prover sends the evaluations of all $f_i$ ($v_{f_i} = f_i(\xi)$) as well as the single opening proof $\pi_w$ for the batched polynomial $w(X) = \sum_{i = 0}^k \a^i f_i(X)$.

Now, the verifier can construct the commitment ($C_w$) and evaluation ($v_w$) to $w$ themselves:

$$
\begin{aligned}
  C_w &= \sum_{i = 0}^k \a^i C_{f_i} \\
  v_w &= \sum_{i = 0}^k \a^i v_{f_i}
\end{aligned}
$$

Finally, the verifier finally checks that $\PCCheck(C_w, d, \xi, v_w, \pi_w) \meq \top$

**Correctness:**

The correctness of the protocol is trivial

## Grand Product Argument

- motivation
- naive construction
- correctness
- optimized (single inverse)

TODO

\newpage

# General Arithmetization Scheme

We define the functions in the following pipeline:
$$
(x,w) = \mathrm{circuit} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})
$$

## Abstract Gates

Gates $g$ are primitive operations with $n_g \geq 0$ fan in inputs and $m_g \geq 0$ fan out outputs defined with its input wire id(s) of type $\Nb$. i.e. $x \neq a \land y \neq b \leftrightarrow \text{Add}(x,y) \neq \text{Add}(a,b)$.

$\text{Add}(x,y)$ is a function call that returns $(\text{Add}, (x,y))$ where $\text{Add}$ in the latter is a term of $\text{GateType}$; not a function.

$$
\begin{array}{rl}
\text{Gate} &= (g: \text{GateType}) \times \Nb^{n_g} \\
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
n &: \text{Gate} + \text{GateType} \to \Nb \\
m &: \text{Gate} + \text{GateType} \to \Nb
\end{array}
&
\begin{array}{rl}
\text{ty} &: \text{Gate} \to \text{GateType} \\
\text{ty}(t, \_) &= t
\end{array}
&
\begin{array}{rl}
\text{in} &: (g: \text{Gate}) \to \Nb^{n_g} \\
\text{in}(\_, \abst{\vec{x}}) &= \abst{\vec{x}} \\
\end{array}
\end{array}
$$

## Arithmetize

Arithmetize turns a program $f$ into an abstract circuit $\abst{f}$, which is a one-to-many-or-none relation between gates $g$ and output wire id(s) $\abst{y}$ or $\bot$ which denotes no output wires. e.g. $(\text{Add}(a,b), c) \in \abst{f}$ corresponds to $\build{a+b=c}{}{}$.

We notate inserting a gate or gadget $f$ to the circuit with $\build{f = \abst{\vec{y}}}{s}{s'}$, $\build{f = \abst{y}}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state from $s$ to $s'$. State is of the form $(u, \abst{f})$ where $u$ is the current uuid for wires. 
A circuit / gadget is a composition of gates.

Wires annotated with $*$, i.e. $\build{f = \abst{y}^*}{}{}$ are the final output and are appended to $\abst{\vec{Y}}$. They, may be omitted notationally.

These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \abst{f}$, then $\abst{\vec{y}}$ in $\build{g=\abst{\vec{y}}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

$$
\begin{aligned}
\AbsCirc &= \set{
  \abst{f} \subset \Gate \times \Nb_\bot \middle\vert
  \forall (g,\abst{y}),(h,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies g = h
} \\
\Gate^{\abst{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
\\
\AState &= \Nb \times \AbsCirc
\end{aligned}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{out} &: (\Nb_\bot + \AbsCirc) \to (g: \Gate) \to \Nb^{m_g} \\
\text{out}(\bot, \_) &= () \\
\text{out}(u,g) &= (u..u+m_g) \\
\text{out}(\abst{f}, g)
&= \text{out}(\min\left(
  \set{\abst{y} \middle\vert (g,\abst{y}) \in \abst{f}}
\right), g) \\
\\
\text{entries}  &: \Nb \to \Gate \to \AbsCirc \\
\text{entries}(u,g) &= \begin{cases}
  \set{(g,\abst{y}) \middle\vert \abst{y} \in \text{out}(u,g)}
  & m_g > 0 \\
  \set{(g,\bot)}
  & m_g = 0
\end{cases} \\
\\
\text{put} &: \Gate \to \AState \to \AState \\
\text{put}(g, u, \abst{f}) &= (
  u + m_g, \abst{f} \cup \text{entries}(u, g)
)
\end{array}
&
\begin{array}{rl}
\text{get} &: \AState \to (g: \Gate) \to \AState \times \Nb^{m_g} \\
\text{get}(u, \abst{f}, g)
&= \begin{cases}
  (u, \abst{f}, \text{out}(\abst{f}, h)) & h \in \Gate^{\abst{f}}_g \\
  (\text{put}(g, u, \abst{f}), \text{out}(u,g)) & \text{otherwise}
\end{cases} \\
\\
\build{g = \abst{\vec{y}}}{s}{s'}
&= \left(\text{get}(s,g) \overset{?}{=} (s', \abst{\vec{y}})\right)  \\
\build{f=\abst{y}^*}{s}{s'} &= \build{f=\abst{y}}{(s,\abst{\vec{Y}})}{(s', \abst{\vec{Y}} \cat \abst{y})} \\
\build{f}{s_1}{s_{k+1}}
&= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}} \\
\\
\text{arithmetize} &: (\Fb^n_q \to \Fb^m_q) \to \AbsCirc \times \Nb^{m'} \\
\text{arithmetize}(f) &= \maybe{(\abst{f}, \abst{\vec{Y}})}{
  \build{f}{\left(\circ_{i \in [0..n]}\text{put}(\text{Input}_i)(0,\emptyset), () \right)}{(\_, \abst{f}, \abst{\vec{Y}})}
}
\end{array}
\end{array}
$$

Note: $\text{Input}_i$ is a family of gates with no inputs and one output wire corresponding to an input of the final circuit. The list of gates available are defined in section on Gates and Gadgets.

## Trace

$\text{trace}$ computes the least fixed point of a composition of monotonic functions using $\text{sup}$. We also call a monotonic function a continuation if it is called by another. We call lift, to extend the argument of a monotonic function.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{lift}(f) &= \lambda (v,t). (v, f(t)) \\
\text{liftR}(f) &= \lambda(t, v). (f(t), v) \\
g \circ^{\uparrow} f &= \text{liftR}(g) \circ \text{lift}(f) 
\end{array} &
\begin{array}{rl}
\text{sup} &: (T \to T) \to (T \to T \to \Bb) \to T \to T \to T \\
\text{sup}(f, \text{eq}, s, s') &= \begin{cases}
s & \text{eq}(s, s') \\
\text{sup}(f, \text{eq}, s', f(s')) & \text{otherwise}
\end{cases}
\end{array}
\end{array}
$$

### Resolve

$\Downarrow_R$ computes the values of wires $\abst{\vec{Y}}$ and inputs to assert gates given the input wire values $\vec{x}$.
 
It does this by peeking from the stack $\abst{\vec{y}}$, querying $\text{?}$ for unresolved input wires, otherwise it will evaluate the output wire values and cache it in the value map $v$ with $[\cdot]$. Every gate type has an evaluation of its output(s). e.g. $\text{eval}(\text{Add}, (1,2)) = (3)$.

$$
\begin{array}{rl}
\text{eval} &: (g: \text{GateType}) \to \Fb^{n_g}_q \to \Fb^{m_g}_q 
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\VMap &= \Nb \pto \Fb_q \\
\RState^k &= \VMap \times \Nb^k \\
\end{array}
&
\begin{array}{rl}
\curvearrowleft &: X^k \to X^{k'} \\
\curvearrowleft (\vec{x}) &= \begin{cases}
() & \vec{x} = () \\
\vec{x}' & \vec{x} = \_ \cat \vec{x}' \\
\end{cases}
\end{array}
&
\begin{array}{rl}
\underset{R}{\curvearrowleft} &: T \times \Nb^k \to T \times \Nb^{k'} \\
\underset{R}{\curvearrowleft} &= \text{lift}(\curvearrowleft)
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{?} &: \VMap \to \Nb^k \to \Nb^{k'} \\
v \text{?} \abst{\vec{y}} &= \begin{cases}
() & \abst{\vec{y}} = () \\
& \abst{\vec{y}} = \abst{y} \cat \abst{\vec{y}}' \\
\abst{y} \cat v \text{?} \abst{\vec{y}}' & v(\abst{y}) = \bot \\
v \text{?} \abst{\vec{y}}' & \text{otherwise}
\end{cases} \\
\\
\left[ \cdot \right] &: \VMap \to \AbsCirc \to \Nb \to \VMap \\
v_{\abst{f}}\left[\abst{y}\right] &= \maybe{
  v[\abst{\vec{y}} \mapsto \vec{y}]
}{\begin{array}{rl}
  \abst{f} &\ni (g, \abst{y}) \\
  \abst{\vec{y}} &= \text{out}(\abst{f}, g) \\
  \vec{y} &= \text{eval}(\text{ty}(g), v @ \text{in}(g)) \\
\end{array}}
\end{array}
&
\begin{array}{rl}
\Downarrow_R &: (T \times \RState \to T \times \RState) \to \AbsCirc \\
&\to T \times \RState \to T \times \RState \\
f \stackrel{\to}{\circ} \Downarrow^{\abst{f}}_R(t,v, \abst{\vec{y}}) &= \begin{cases}
f(t,v,()) & \abst{\vec{y}} = () \\
 & \abst{\vec{y}} = \abst{y} \cat \_ \\
\underset{R}{\curvearrowleft} (t, v, \abst{\vec{y}}) & v(\abst{y}) \neq \bot \\
 & (g, \abst{y}) \in \abst{f} \\
 & \abst{\vec{x}} = v \text{?} \text{in}(g) \\
\underset{R}{\curvearrowleft} \circ f(t, v_{\abst{f}}[\abst{y}], \abst{\vec{y}}) 
 & \abst{\vec{x}} = () \\
(t, v, \abst{\vec{x}} \cat \abst{\vec{y}}) & \text{otherwise}
\end{cases} \\
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\Omega_R &: \RState \to \Bb \\
\Omega_R(\_, \abst{\vec{y}}) &= |\abst{\vec{y}}| = 0 
\end{array}
&
\begin{array}{rl}
s_0^R &: \RState \\
s_0^R &= (\bot, ())
\end{array}
\end{array}
$$

### Gate Constraints

$\Downarrow_G$ computes the gate constraints by pushing the gate with an output of the top of the wire id stack via push; $\underset{G}{\curvearrowright}$. The same gate will not appear twice since we do not call the continuation on resolved wires in $\Downarrow$.

When the wire id stack $\abst{\vec{y}}$ is empty, $\underset{G}{\curvearrowright}$ will push assert gates and input gates $A^{\abst{f}}$ to the stack.
$$
\begin{array}{rl}
\text{Term} &= \text{Slot} + \text{Selector} \\
\text{ctrn} &: (g : \text{GateType}) \to \Fb_q^{n_g + m_g} \to \Fb_q^{|\text{Term}| \times k}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{GState}^{k,k',k''} &= \Fb_q^{|\text{Term}| \times k''} \times \Gate^{k'} \times \Bb \times \RState^k \\
A^{\abst{f}} &= \set{g \middle\vert (g, \abst{y}) \in \abst{f} \land (\abst{y} = \bot \lor \exists i. \abst{y} = \text{Input}_i) }
\end{array}
&
\begin{array}{rl}
\underset{G}{\curvearrowleft} &: T \times \text{GState}^{k''',k',k} \to T \times \text{GState}^{k''',k'',k} \\
\underset{G}{\curvearrowleft} &= \text{lift} \circ \text{liftR}(\curvearrowleft : \text{Gate}^k \to \text{Gate}^{k'})
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\Downarrow_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \underset{G}{\curvearrowleft} \circ f \circ^\uparrow \lambda (\vec{C}, \vec{g}, b, v).
\begin{cases}
(\vec{C}, (), b, v) & \vec{g} = () \\
& \vec{g} = g \cat \_ \\
& \vec{v} = v @ (\text{in}(g) \cat \text{out}(\abst{f},g)) \\
(\vec{C}', \vec{g}, b, v)
& \vec{C}' = \vec{C} \cat \text{ctrn}(\text{ty}(g), \vec{v}) \\
\end{cases} \circ^\uparrow \lambda(\vec{g}, b, v, \abst{\vec{y}}).
\begin{cases}
& b = \bot \\
(A^{\abst{f}} \cat \vec{g}, \top, v, \abst{\vec{y}})
& |\abst{\vec{y}}| = |\vec{g}| = 0 \\
& \abst{\vec{y}} = \abst{y} \cat \_ \\
(g \cat \vec{g}, b, v, \abst{\vec{y}})
& (g,\abst{y}) \in \abst{f} \\
(\vec{g}, b, v, \abst{\vec{y}})
& \text{otherwise}
\end{cases}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\Omega_G &: \text{GState} \to \Bb \\
\Omega_G(\_, \vec{g}, b, \_) &= |\vec{g}| = 0 \land b = \top
\end{array}
&
\begin{array}{rl}
s_0^G &: \text{RState} \to \text{GState} \\
s_0^G(s) &= ((), (), \bot, s)
\end{array}
\end{array}
$$

### Copy Constraints

$\Downarrow_C$ quotients an ordered set of coordinates in slot positions of $\vec{C}$ by the wire id corresponding to the value there.

This is done by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate. This corresponds to $\mathtt{ctrn}$.

After computing the loops as quotients $c$ with all gates, we mark a flag $\Bb$ which starts the next phase computing the coordinate map $m$ from coordinate to its neighbour in $c$.

With $m$ we compute the permutation of the slots in $\vec{C}$.
$$
\begin{array}{rl}
\begin{array}{c}
\begin{array}{rl}
\text{Coord} &= \text{Slot} \times \Nb \\
\text{CLoop} &= (\abst{y} : \Nb) \pto \text{Coord}^{k_{\abst{y}}} \\
\text{CMap} &= \text{Coord} \pto \text{Coord} \\
\text{loop} &: \text{Row} \to \text{GateType} \to \text{CLoop} \\
\\
\text{CState}^{k,k'} &= \Nb \times \text{Coord}^{|\text{Slot}| \times k} \times \text{CMap} \times \\
&\Bb \times \text{CLoop} \times \text{GState}^{k'}\\
\\
\sqcup &: \text{CLoop} \to \text{CLoop} \to \text{CLoop} \\
x \sqcup y &= \begin{cases}
x & y = \bot \\
& \exists i. y(i) = \vec{l} \\
& y' = y[i \mapsto \bot] \\
x[i \mapsto x(i) \cat \vec{l}] \sqcup y'
& x(i) \neq \bot \\
x[i \mapsto \vec{l}] \sqcup y'
& \text{otherwise}
\end{cases} \\
\end{array} \\
\begin{array}{rl}
\begin{array}{rl}
\Omega_C &: \text{CState} \to \Bb \\
\Omega_C &= \lambda (N, \_, \_, b, c, \_). \\
&N = 0 \land b = \top \land c = \bot
\end{array}
&
\begin{array}{rl}
s_0^C &: \text{GState} \to \text{CState} \\
s_0^C(s) &= (0, (), \bot, \bot, \bot, s)
\end{array}
\end{array}
\end{array}
&
\begin{array}{rl}
\Downarrow_C &: \text{CState} \to \text{CState} \\
\Downarrow_C &= \text{liftR} @ \lambda (N, \vec{\sigma}, m). \\
&\begin{cases}
(0, \vec{\sigma}, m) & N = 0 \\
(N-1, m @ (\text{Slot}..) \cat \vec{\sigma},m) & \text{otherwise}
\end{cases} \\
& \circ^\uparrow \lambda(N, \vec{\sigma}, m, b,c, \vec{C}). \\
&\begin{cases}
(0, (), m,\bot, c, \vec{C}) & b = \bot \\
& \exists \abst{y}. c(\abst{y}) \neq \bot \\
& c' = c[\abst{y} \mapsto \bot] \\
& \vec{l} = l \cat \vec{l}' = c(\abst{y}) \\
(0, (), m', \top, c', \vec{C})
& m' = m[\vec{l} \mapsto \vec{l}' \cat l] \\
& N' = |\vec{C}| / |\text{Term}| \\
(N', (), m, \top, \bot, \vec{C}) & N = 0 \land \vec{\sigma} = () \\
(N, \vec{\sigma}, m, \top, \bot, \vec{C}) &\text{otherwise} 
\end{cases} \\
& \circ^\uparrow \lambda (b, c,\vec{C},\vec{g}). \\
&\begin{cases}
(\top, c, \vec{C}, \vec{g})
& \vec{g} = () \lor b = \top \\
& \vec{g} = g \cat \_ \\
(\bot, c', \vec{C}, \vec{g}) & c' = c \sqcup \text{loop}(|\vec{C}|, \text{ty}(g))
\end{cases} \\
\end{array}
\end{array}
$$

### Lookup Argument Constraints

- $t$ table
  - introduce tables; compression formula
  - sort
  - capture last entry as default
- $f$ gate
  - if gate is lookup, compute compression from $v$
  - DONT SORT
  - else use default
- $h1,h2$; concat $t,f$ sort, then split odds and evens

### Full Surkål Trace


$$
\begin{array}{rl}
\text{trace} &: \AbsCirc \to \Nb^m \to \Fb^n_q \to \text{Coord}^{|\text{Slot}| \times k} \times \Fb_q^{|\text{Term}| \times k} \\
\text{trace}(\abst{f}, \abst{\vec{Y}}, \vec{x})
&= \lambda(\_, \vec{\sigma}, \_, \_, \_, \vec{C}, \_, \_, \_ , \_) . (\vec{\sigma}, \vec{C}) \circ \\
&\text{sup}\left(\begin{array}{cccl}
  \Downarrow^{\abst{f}}_L \circ^\uparrow &\Downarrow_C \stackrel{\to}{\circ} &\Downarrow_G^{\abst{f}} \stackrel{\to}{\circ} &\Downarrow_R^{\abst{f}}, \\
  \Omega_L \circ &\Omega_C \circ &\Omega_G \circ &\Omega_R \circ \lambda(\_,x).x, \\
  s^L_0 \circ &s^C_0 \circ &s^G_0 @ &s^R_0, \\
  s^L_0 \circ &s^C_0 \circ &s^G_0 @ &\left( 
    \begin{array}{l}
      \bot[(0..|\vec{x}|) \mapsto \vec{x}] \\
      \abst{\vec{Y}} \cat \set{\abst{x} \middle\vert (g, \bot) \in \abst{f} \land \abst{x} \in \text{in}(g) \setminus \abst{\vec{Y}}}
    \end{array}
  \right)
\end{array}\right)
\end{array}
$$

# Plonk Protocol

- cosets
- fft
- F_GC
- grand product
  - cc
  - pl


## Circuit

- compute $N$, $\omega$, $h$
- expand $\vec{C}$, plookup thunk to N
- polys
  - slots
  - permutation polys
- commits
- lookup thunk

## Prover

- list of formulas
  - table compression
  - F_GC
- cc, pl: Z, f, g
- quotient poly: t
- opening poly: W

## Verifier

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

TODO

# Surkål Circuits

# Gates and Gadgets

TODO: ctrn and loop too

| $g: \Gate$                | $\text{eval}(g, \vec{x})$     | remarks                 |
|:-------------------------:|:-----------------------------:|:------------------------|
| Input$_i()$               | $(x_i)$                       | from trace              |
| Const$_{s,p}()$           | $(s)$                         |                         |
| Add$(x,y)$                | $(x+y)$                       |                         |
| Mul$(x,y)$                | $(x \times y)$                |                         |
| Inv$(x)$                  | $(x^{-1})$                    |                         |
| Pow7$(x)$                 | $(x^7)$                       |                         |
| If$(b,x,y)$               | $(b ? x : y)$                 |                         |
| Lookup$_T(x,y)$           | $\maybe{(z)}{(x,y,z) \in T}$  |                         |
| PtAdd$(x_P,y_P,x_Q,y_Q)$  | $(x_R, y_R)$                  | Arkworks point add      |
| Poseidon$(a,b,c)$         | $(a',b',c')$                  | Mina poseidon 5 rounds  |
| Public$(x)$               | $()$                          |                         |
| Bit$(b)$                  | $()$                          |                         |
| IsAdd$(x,y,z)$            | $()$                          |                         |
| IsMul$(x,y,z)$            | $()$                          |                         |
| IsLookup$_T(x,y,z)$       | $()$                          |                         |

## XOR

## Poseidon

## Range Check

## Foreign Field stuff

# Signatures

# IVC Verifier from Gadgets

## Surkål Verifier

## Accumulation Verifier

## SuccinctCheck

## Signatures

# Appendix

## Notation

TODO make to a neat table, and include notation in plonk report

types and type formers

- naturals $\Nb$
- pointed type $T_\bot$, has an (additional) smallest element $\bot$
- finite fields $\Fb_q$
- vector type $T^n$
- matrix / tensor type $T^{n \times m}$
- tuple / product type $T \times U$
- function type $X \to Y$
- partial function type $X \pto Y$
- disjoint union / sum type $T + U$

term constructors

- empty vector / unit tuple $()$
- vector term / tuple term $\vec{x} = (x_1, x_2, \cdots , x_n)$
- vector append / cons $y \cat \vec{x} = (y, x_1, x_2, \cdots x_n), \vec{x} \cat y = (x_1, x_2, \cdots, x_n, y)$
- vector of enumeration of a finite ordered type $(X..) = (x_1, x_2, \ldots x_n)$
- matrix / tensors as vectors $\vec{m}: T^{w \times h}, \vec{m}[i,j] = m_{i + h(j-1)}$
- function term / lambda abstraction $\lambda x. f(x)$
- function term by evaluations $\lambda[x \mapsto f(x)]$, implying $f(x)$ is evaluated upon construction for all $x$
- empty partial function $\bot$
- partial function append $f[x \mapsto y]$
- disjoint union implictly has no constructors, however we can $\text{inl}(t), \text{inr}(u)$ to avoid ambiguity

util functions

- maybe notation $\maybe{x}{\phi(x)} = \begin{cases} x & \phi(x) \\ \bot & \text{otherwise} \end{cases}$
- maybe with default $\maybe{x \lor y}{\phi(x)} = \begin{cases} x & \phi(x) \\ y & \text{otherwise} \end{cases}$
- vector of naturals builder $(s..t) = \begin{cases} () & t \leq s \\ s \cat (s+1 .. t) \end{cases}$
- vector concat $\vec{x} \cat \vec{y} = \begin{cases} \vec{y} & \vec{x} = () \\ \vec{x}' \cat (x \cat \vec{y}) & \vec{x} = \vec{x'} \cat x \end{cases}$
- vector concat with set $X \cat \vec{x}$; any random ordering of $X$; recursive application of axiom of choice
- function application $f @ x = f(x)$
- vector map $f @ \vec{x} = (f(x_1), f(x_2), \ldots, f(x_n))$
- vector minus set $\vec{x} \setminus X$ turns $\vec{x}$ to a set and removes all elements in $X$
- min of a set with total ordering $\min(X)$
- partial function append vector $f[\vec{x} \mapsto \vec{y}] = \begin{cases} & \vec{x} = x \cat \vec{x}' \\ f[x \mapsto y][\vec{x}' \mapsto \vec{y}'] & \vec{y} = y \cat \vec{y}' \\ f & \text{otherwise} \end{cases}$

identities

- associative product and function types
- currying $T \to U \to V = (T \times U) \to V$
- curried / associative tuples $((a,b),c) = (a,b,c) = (a,(b,c))$

set theoretic notations

- set of naturals from one $[n] = \set{1,2,\ldots,n-1}$
- set of naturals with lower bound $[n..m] = \set{n,n+1,\ldots,m-1}$
- flattened case notation, conditions are propagated to conditions below if they don't contradict.
- if a case has no term, the next termed case must satisfy it, but subsequent cases need not (note the $\land \phi_2(a))$
$$
\begin{array}{rl}
\begin{cases}
a & \phi_1(a) \\
 & \phi_2(a) \\
b & \phi_3(b) \\
c & \phi_4(c) \\
\vdots
\end{cases} &=
\begin{cases}
a & \phi_1(a) \\
b(a) & (\phi_3(b(a)) \lor \phi_1(a)) \land \phi_2(a) \\
c(b(a),a) & \phi_4(c(b(a),a)) \lor \phi_1(a) \lor \phi_2(a) \lor \phi_3(b(a)) \\
\vdots
\end{cases}
\end{array}
$$

conventions
- $\abst{x}$ is an abstract of a thing, e.g. $\abst{f}$ is an abstract circuit, $\abst{y}$ is an abstract output wire (id)

## Arithmetize Example

Example of the arithmetization of $x^2 + y$ with gates Input, Mul$(a,b)$ and Add$(a,b)$ all with $m=1$:
$$
\begin{aligned}
&\text{arithmetize}((x,y) \mapsto (x^2 + y))
\\
&= \maybe{\left(\abst{f}'', (z)\right)}{
  \build{x^2 + y = z^*}
    {(u, \abst{f}) = (\text{put}(\text{Input}_0) \circ \text{put}(\text{Input}_1)(0, \emptyset), \emptyset)}
    {(\_, \abst{f}'', (z))}
  }
\\
&= \maybe{\left(\abst{f}'', (z)\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \abst{f}, \emptyset)}{(\_, \abst{f}'', (z))}}
\\
&= \maybe{\left(\abst{f}'', (z)\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \abst{f})}{(u', \abst{f}')} \\
  \build{t + y = z^*}{(u', \abst{f}', \emptyset)}{(\_, \abst{f}'', (z))}
\end{array}}
\\
&= \maybe{\left(\abst{f}'', (z)\right)}{\begin{array}{rl}
  \text{get}(u, \abst{f}, \text{Mul}(x,x)) &= (u', \abst{f}', (t)) \\
  \text{get}(u', \abst{f}', \text{Add}(t,y)) &= (\_, \abst{f}'', (z))
\end{array}}
\\ 
&= \maybe{\left(\abst{f}'', (z)\right)}{\begin{array}{rl}
  (u+1, \abst{f} \cup \set{(\text{Mul}(x,x), u)}, (u)) &= (u', \abst{f}', (t)) \\
  \text{get}(u', \abst{f}', \text{Add}(t,y)) &= (\_, \abst{f}'', (z))
\end{array}}
\\
&= \maybe{\left(\abst{f}'', (z)\right)}{
  \text{get}(u+1, \abst{f} \cup \set{(\text{Mul}(x,x))}, \text{Add}(u,y)) = (\_, \abst{f}'', (z))
}
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(x,x) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
  (u, \abst{f}) = \text{put}(\text{Input}_0) \circ \text{put}(\text{Input}_1)(0, \emptyset)
}
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
    (u, \abst{f}) = \text{put}(\text{Input}_1, 1, \set{(\text{Input}_0, 0)})
  }
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,1) & u+1
  \end{array}}, (u+1) \right)}
  {(u, \abst{f}) = \left(2, \set{\begin{array}{rl}
    \text{Input}_0 & 0 \\
    \text{Input}_1 & 1
  \end{array}}\right)}
\\
&= \left(\set{\begin{array}{rl}
  \text{Input}_0 & 0 \\
  \text{Input}_1 & 1 \\
  \text{Mul}(0,0) & 2 \\
  \text{Add}(2,1) & 3
\end{array}}, (3)\right)
\end{aligned}
$$

## Trace Example

TODO

## Defining Equivalence of Gates with Egglog

TODO

## Kleene Fixedpoint Theorem in Trace

Trace is defined as a composition of monotonic functions that has control over their continuations. Thus if the full composition is $f$, then the trace is $\mu x. f(x)$. Given an initial state, it is notated as the supremum. $\text{sup}_{n \in \Nb} f^n(s_0)$, where $n$ is the smallest $n$ such that $f^n(s_0) = f^{n+1}(s_0)$, i.e. the least fixedpoint of $f$. We have shown the recursive definition before. Now we present the iterative definition which will be useful in code implementations to circumvent the recursion limit or stack overflow errors.

\begin{algorithm}[H]
\caption*{
  \textbf{sup:} iterative kleene least fixedpoint protocol.
}
\textbf{Inputs} \\
  \Desc{$f: \text{State}^T \to \text{State}^T$}{Monotonic function.} \\
  \Desc{$s_0 : \text{State}^T$}{Initial state.} \\
  \Desc{$\text{eq}: \text{State}^T \to \text{State}^T \to \Bb$}{Equality predicate on states.} \\
\textbf{Output} \\
  \Desc{$s_n : \text{State}^T$}{The state corresponding to the least fixedpoint of $f$.}
\begin{algorithmic}[1]
  \State Initialize variables:
    \Statex \algind $s := \bot$
    \Statex \algind $s' := s_0$ 
  \State Recursive compute:
    \Statex \textbf{do:}
    \Statex \algind $s := s'$
    \Statex \algind $s' := f(s')$
    \Statex \textbf{while} $\text{eq}(s,s') = \bot$
  \State Return the least fixedpoint:
    \Statex \textbf{return} $x$
  \end{algorithmic}
\end{algorithm}

We can show that the function is monotonic by defining the order on the state, and showing that the function preserves the order. The order is defined as follows:

$$
(t,v,b,\vec{s}) \sqsubseteq (t',v',b',\vec{s'}) \iff
\begin{aligned}
  &t \not\sqsubseteq t' \Rightarrow \text{dom}(v) \not\subseteq \text{dom}(v') \Rightarrow |s| < |s'|
\end{aligned}
$$

We never remove the mappings in $v$ thus the order is preserved for $v$ despite the stack $s$ can grow and shrink. To show $t \sqsubseteq t'$ then is to investigate the remaining monotonic continuations for Surkål.

TODO: cleanup and make full preorder relation definition, i.e. $s \sqsubseteq f(s)$

# Bibliography

