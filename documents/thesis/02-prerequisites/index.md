# Prerequisites

Basic knowledge of elliptic curves, groups and interactive arguments
is assumed in the following text. Basic familiarity with SNARKs is also
assumed. The polynomial commitment scheme implemented heavily relies on the
Inner Product Proof from the Bulletproofs protocol. If needed, refer to the
following resources:

- Section 3 in the original Bulletproofs[@bulletproofs] paper.
- From Zero (Knowledge) to Bulletproofs writeup[@from0k2bp].
- Rust Dalek Bulletproofs implementation notes[@dalek-docs].
- Section 4.1 of my bachelors thesis[@hacspec-bulletproofs].

## Background and Motivation

The following subsections introduce the concept of Incrementally Verifiable
Computation (IVC) along with some background concepts. These concepts lead to
the introduction of accumulation schemes and polynomial commitment schemes,
the main focus of this paper. Accumulation schemes, in particular, will be
demonstrated as a means to create more flexible IVC constructions compared
to previous approaches, allowing IVC that does not depend on a trusted setup.

As such, these subsections aim to provide an overview of the evolving field
of IVC, the succinct proof systems that lead to its construction, and the
role of accumulation schemes as an important cryptographic primitive with
practical applications.

### Proof Systems

An Interactive Proof System consists of two Interactive Turing Machines:
a computationally unbounded Prover, $\Pc$, and a polynomial-time bounded
Verifier, $\Vc$. The Prover tries to convince the Verifier of a statement
$X \in L$, with language $L$ in NP. The following properties must be true:

- **Completeness:** $\forall \Pc \in ITM, X \in L \implies \Pr[\Vc_{out} = \bot] \leq \epsilon(X)$

  For all honest provers, $\Pc$, where $X$ is true, the probability that the
  verifier remains unconvinced is negligible in the length of $X$.

- **Soundness:** $\forall \Pc^* \in ITM, X \notin L \implies \Pr[\Vc_{out} = \top] \leq \epsilon(X)$

  For all provers, honest or otherwise, $\Pc^*$, that try to convince the
  verifier of a claim, $X$, that is not true, the probability that the
  verifier will be convinced is negligible in the length of $X$.

An Interactive Argument is very similar, but the honest and malicious prover
are now polynomially bounded and receives a Private Auxiliary Input, $w$,
not known by $\Vc$. This is such that $\Vc$ don't just compute the answer
themselves. Definitions follow:

- **Completeness**: $\forall \Pc(w) \in PPT, X\in L \implies \Pr[\Vc_{out} = \bot] \leq \epsilon(X)$
- **Soundness**: $\forall \Pc^* \in PPT, X \notin L \implies \Pr[\Vc_{out} = \top] \leq \epsilon(X)$

Proofs of knowledge are another type of Proof System, here the prover claims
to know a _witness_, $w$, for a statement $X$. Let $X \in L$ and $W(X)$ be the
set of witnesses for $X$ that should be accepted in the proof. This allows
us to define the following relation: $\Rc = \{ (X,w) : X \in L , w \in W(X) \}$

A proof of knowledge for relation $\Rc$ is a two party protocol $(\Pc, \Vc)$
with the following two properties:

- **Knowledge Completeness:** $\Pr[\Pc(w) \iff \Vc_{out} = \top] = 1$, i.e. as in
  Interactive Proof Systems, after an interaction between the prover and
  verifier the verifier should be convinced with certainty.  
- **Knowledge Soundness:** Loosely speaking, Knowledge Soundness requires
  the existence of an efficient extractor $\Ec$ that, when given a possibly
  malicious prover $\Pc^*$ as input, can extract a valid witness with
  probability at least as high as the probability that $\Pc^*$ convinces the
  verifier $\Vc$.

The above proof systems may be _zero-knowledge_, which in loose terms means
that anyone looking at the transcript, that is the interaction between
prover and verifier, will not be able to tell the difference between a real
transcript and one that is simulated. This ensures that an adversary gains
no new information beyond what they could have computed on their own. We
now define the property more formally:

- **Zero-knowledge:** $\forall \Vc^*(\delta). \exists S_{\Vc^*}(X) \in PPT. S_{\Vc^*} \sim^C (\Pc,\Vc^*)$

$\Vc^*$ denotes a verifier, honest or otherwise, $\d$ represents information
that $\Vc^*$ may have from previous executions of the protocol and $(\Pc,\Vc^*)$
denotes the transcript between the honest prover and (possibly) malicious
verifier. There are three kinds of zero-knowledge:

- **Perfect Zero-knowledge:** $\forall \Vc^*(\delta). \exists S_{\Vc^*}(X) \in PPT. S_{\Vc^*} \sim^\Pc (\Pc,\Vc^*)$,
  the transcripts $S_{\Vc^*}(X)$ and $(\Pc,\Vc^*)$ are perfectly indistinguishable.
- **Statistical Zero-knowledge:** $\forall \Vc^*(\delta). \exists S_{\Vc^*}(X) \in PPT. S_{\Vc^*} \sim^S (\Pc,\Vc^*)$,
  the transcripts $S_{\Vc^*}(X)$ and $(\Pc,\Vc^*)$ are statistically indistinguishable.
- **Computational Zero-knowledge:** $\forall \Vc^*(\delta). \exists S_{\Vc^*}(X) \in PPT. S_{\Vc^*} \sim^C (\Pc,\Vc^*)$,
  the transcripts $S_{\Vc^*}(X)$ and $(\Pc,\Vc^*)$ are computationally
  indistinguishable, i.e. no polynomially bounded adversary $\Ac$ can
  distinguish them.

#### Fiat-Shamir Heuristic

The Fiat-Shamir heuristic turns a public-coin (an interactive protocol where
the verifier only sends uniformly sampled challenge values) interactive
proof into a non-interactive proof, by replacing all uniformly random
values sent from the verifier to the prover with calls to a non-interactive
random oracle. In practice, a cryptographic hash function, $\rho$, is
used. Composing proof systems will sometimes require *domain-separation*,
whereby random oracles used by one proof system cannot be accessed by another
proof system. This is the case for the zero-finding game that will be used
in the soundness discussions of implemented accumulation scheme $\ASDL$. In
practice one can have a domain specifier, for example $0, 1$, prepended to
each message that is hashed using $\rho$:
$$ \rho_0(m) = \rho(0 \cat m), \quad \rho_1(m) = \rho(1 \cat m)$$

#### SNARKS

**S**uccinct **N**on-interactive **AR**guments of **K**nowledge -
have seen increased usage due to their application in blockchains and
cryptocurrencies. They also typically function as general-purpose proof
schemes. This means that, given any solution to an NP-problem, the SNARK prover
will produce a proof that they know the solution to said NP-problem. Most
SNARKs also allow for zero-knowledge arguments, making them zk-SNARKs.

More concretely, imagine that Alice has today's Sudoku problem $X \in
\text{NP}$: She claims to have a solution to this problem, her witness, $w$,
and wants to convince Bob without having to reveal the entire solution. She
could then use a SNARK to generate a proof for Bob. To do this she must first
encode the Sudoku verifier as a circuit $R_X$, then let $x$ represent public
inputs to the circuit, such as today's Sudoku values/positions, etc, and then
give the SNARK prover the public inputs and her witness, $\SNARKProver(R_X,
x, w) = \pi$. Finally she sends this proof, $\pi$, to Bob along with the
public Sudoku verifying circuit, $R_X$, and he can check the proof and be
convinced using the SNARK verifier ($\SNARKVerifier(R_X, x, \pi)$).

Importantly, the 'succinct' property means that the proof size and
verification time must be sub-linear. This allows SNARKs to be directly used
for _Incrementally Verifiable Computation_.

#### Trusted and Untrusted Setups

Many SNARK constructions, such as the original Plonk specification, depend on a
_trusted setup_ to ensure soundness. A trusted setup generates a _Structured
Reference String_ (SRS) with a particular internal structure. For Plonk,
this arises from the KZG[@kzg] commitments used. These commitments allow
the SNARK verifier to achieve sub-linear verification time. However, this
comes at the cost of requiring a trusted setup, whereas $\PCDL$ for example,
uses an _untrusted setup_.

An untrusted setup, creates a _Uniform Random String_ of the form:
$$\text{URS} = \{ a_1G, a_2G, \dots, a_DG \}$$
Where $D$ represents the maximum degree bound of a polynomial (in a PCS
context) and $G$ is a generator. The URS must consist solely of generators and
all the scalars must be uniformly random. $\PCDL$ is then sound, provided that
no adversary knows the scalars. Extracting $\vec{a}$ from the URS would require
solving the Discrete Logarithm problem (DL), which is assumed to be hard.

To generate the URS transparently, a collision-resistant hash function
$\Hc : \Bb^* \to \Eb(\Fb_q)$ can be used to produce the generators. The URS
can then be derived using a genesis string $s$:
$$\text{URS} = \{ \Hc(s \cat 1), \Hc(s \cat 2), \dots, \Hc(s \cat D) \}$$
This method is used in our implementation, as detailed in the implementation
section

#### Bulletproofs

In 2017, the Bulletproofs paper[@bulletproofs] was released. Bulletproofs
rely on the hardness of the Discrete Logarithm problem, and uses an untrusted
setup. It has logarithmic proof size, linear verification time and lends
itself well to efficient range proofs. It's also possible to generate proofs
for arbitrary circuits, yielding a zk-NARK. It's a NARK since we lose the
succinctness in terms of verification time, making bulletproofs less efficient
than SNARKs.

At the heart of Bulletproofs lies the Inner Product Argument (IPA), wherein a
prover demonstrates knowledge of two vectors, $\vec{a}, \vec{b} \in \Fb_q^n$,
with commitment $P \in \Eb(\Fb_q)$, and their corresponding inner product,
$c = \ip{\vec{a}}{\vec{b}}$. It creates a non-interactive proof, with only
$\lg(n)$ size, by compressing the point and vectors $\lg(n)$ times, halving
the size of the vectors each iteration in the proof. Unfortunately, since the IPA,
and by extension Bulletproofs, suffer from linear verification time,
bulletproofs are unsuitable for IVC.

### Incrementally Verifiable Computation

Valiant originally described IVC in his 2008 paper[@valiant] in the following
way:

\begin{quote}
\color{GbGrey}

\textit{Suppose humanity needs to conduct a very long computation which will span
superpolynomially many generations. Each generation runs the computation
until their deaths when they pass on the computational configuration to the
next generation. This computation is so important that they also pass on a
proof that the current configuration is correct, for fear that the following
generations, without such a guarantee, might abandon the project. Can this
be done?}

\end{quote}

If a computation runs for hundreds of years and ultimately outputs 42, how can
we check its correctness without re-executing the entire process? In order
to do this, the verification of the final output of the computation must be
much smaller than simply running the computation again. Valiant creates the
concept of IVC and argues that it can be used to achieve the above goal.

Recently, IVC has seen renewed interest with cryptocurrencies, as this
concept lends itself well to the structure of blockchains. It allows a
blockchain node to omit all previous transaction history in favour of only
a single state, for example, containing all current account balances. This
is commonly called a _succinct blockchain_..

In order to achieve IVC, you need a function $F(x) \in S \to S$ along with
some initial state $s_0 \in S$. Then you can call $F(x)$ $n$ times
to generate a series of $s$'s, $\vec{s} \in S^{n+1}$:

\begin{figure}[!H]
\centering
\begin{tikzpicture}[node distance=2cm]

  % Nodes
  \node (s0) [node] {$s_0$};
  \node (s1) [node, right=of s0] {$s_1$};
  \node (s2) [node, right=of s1] {$s_2$};
  \node (dots) [right=2cm of s2] {$\dots$};
  \node (sn) [node, right=2cm of dots] {$s_n$};

  % Arrows with labels
  \draw[thick-arrow] (s0) -- node[above] {$F(s_0)$} (s1);
  \draw[thick-arrow] (s1) -- node[above] {$F(s_1)$} (s2);
  \draw[thick-arrow] (s2) -- node[above] {$F(s_2)$} (dots);
  \draw[thick-arrow] (dots) -- node[above] {$F(s_{n-1})$} (sn);

\end{tikzpicture}
\caption{
  A visualization of the relationship between $F(x)$ and $\vec{s}$ in a non-IVC setting.
}
\end{figure}

In a blockchain setting, you might imagine any $s_i \in \vec{s}$
as a set of accounts with corresponding balances, and the transition
function $F(x)$ as the computation happening when a new
block is created and therefore a new state, or set of accounts, $s_i$ is
computed[^ivc-blockchain].

In the IVC setting, we have a proof, $\pi$, associated with each state,
so that anyone can take only a single pair $(s_m, \pi_m)$ along with the
initial state and transition function ($s_0, F(x)$) and verify that said
state was computed correctly.

\begin{figure}[!H]
\centering
\begin{tikzpicture}[node distance=2cm]

  % Nodes
  \node (s0) [node] {$s_0$};
  \node (s1) [node, right=of s0] {$(s_1, \pi_1)$};
  \node (dots) [right=2cm of s1] {$\dots$};
  \node (sn) [node, right=3cm of dots] {$(s_n, \pi_n)$};

  % Arrows with labels
  \draw[thick-arrow] (s0) -- node[above] {$\Pc(s_0, \bot)$} (s1);
  \draw[thick-arrow] (s1) -- node[above] {$\Pc(s_1, \pi_1)$} (dots);
  \draw[thick-arrow] (dots) -- node[above] {$\Pc(s_{n-1}, \pi_{n-1})$} (sn);

\end{tikzpicture}
\caption{
  A visualization of the relationship between $F, \vec{s}$ and $\vec{\pi}$
  in an IVC setting using traditional SNARKs. $\Pc(s_i, \pi_i)$ denotes
  running the $\SNARKProver(R_F, x = \{ s_0, s_i \}, w = \{ s_{i-1}, \pi_{i-1} \})
  = \pi_i$ and $F(s_{i-1}) = s_i$, where $R_F$ is the transition function $F$
  expressed as a circuit.
}
\end{figure}

The proof $\pi_i$ describes the following claim:

\begin{quote}
\color{GbGrey}

\textit{"The current state $s_i$ is computed from applying the function,
$F$, $i$ times to $s_0$ $(s_i = F^i(s_0) = F(s_{i-1}))$ and the associated
proof $\pi_{i-1}$ for the previous state is valid."}

\end{quote}

Or more formally, $\pi_i$ is a proof of the following claim, expressed as
a circuit $R$:
$$R := \text{I.K.} \; w = \{ \pi_{i-1}, s_{i-1} \} \; \text{ s.t. } \; s_i \meq F(s_{i-1}) \; \land \; (s_{i-1} \meq s_0 \lor \SNARKVerifier(R_F, x = \{ s_0, s_i \}, \pi_{i-1}) \meq \top))$$
Note that $R_F, s_i, s_0$ are not quantified above, as they are public
values. The $\SNARKVerifier$ represents the verification circuit in the proof
system we're using. This means, that we're taking the verifier, representing
it as a circuit, and then feeding it to the prover. This is not a trivial
task in practice! Note also, that the verification time must be sub-linear
to achieve an IVC scheme, otherwise the verifier could just have computed
$F^n(s_0)$ themselves, as $s_0$ and $F(x)$ necessarily must be public.

To see that the above construction works, observe that $\pi_1, \dots,
\pi_n$ proves:
$$
\begin{alignedat}{7}
  &\text{I.K.} \; \pi_{n-1} \; &&\text{ s.t. } \; &&s_n     &&= F(s_{n-1}) \; &&\land \; (s_{n-1} = s_0  &&\lor \SNARKVerifier(R, x, \pi_{n-1}) = \top), \\
  &\text{I.K.} \; \pi_{n-2} \; &&\text{ s.t. } \; &&s_{n-1} &&= F(s_{n-2}) \; &&\land \; (s_{n-2} = s_0  &&\lor \SNARKVerifier(R, x, \pi_{n-2}) = \top), \; \dots \\
  &                            &&              \; &&s_1     &&= F(s_0)     \; &&\land \; (s_0 = s_0      &&\lor \SNARKVerifier(R, x, \pi_0) = \top)
\end{alignedat}
$$
Which means that:
$$
\begin{alignedat}{4}
  &\SNARKVerifier(R, x, \pi_n) = \top \implies \\
  &s_n = F(s_{n-1}) \; \land \; \\
  &\SNARKVerifier(R, x, \pi_{n-1}) = \top \; \land \; \\
  &s_{n-1} = F(s_{n-2}) \implies \dots \\
  &\SNARKVerifier(R, x, \pi_1) = \top \implies \\
  &s_1 = F(s_0)
\end{alignedat}
$$
Thus, by induction $s_n = F^n(s_0)$

[^ivc-blockchain]: In the blockchain setting, the transition function would
also take an additional input representing new transactions, $F(x: S, T:
\Pc(T))$.

### Polynomial Commitment Schemes

In the SNARK section, general-purpose proof schemes were described. Modern
general-purpose (zero-knowledge) proof schemes, such as Sonic[@sonic],
Plonk[@plonk] and Marlin[@marlin], commonly use _Polynomial Commitment Schemes_
(PCSs) for creating their proofs. This means that different PCSs can be used
to get security under weaker or stronger assumptions.

- **KZG PCSs:** Uses a trusted setup, which involves generating a Structured
  Reference String for the KZG commitment scheme[@kzg]. This would give you
  a traditional SNARK.
- **Bulletproofs PCSs:** Uses an untrusted setup, assumed secure if the
  Discrete Log problem is hard, the verifier is linear.
- **FRI PCSs:** Also uses an untrusted setup, assumes secure one way functions
  exist. It has a higher constant overhead than PCSs based on the Discrete
  Log assumption, but because it instead assumes that secure one-way functions
  exist, you end up with a quantum secure PCS.

A PCS allows a prover to prove to a verifier that a committed polynomial
evaluates to a certain value, $v$, given an evaluation input $z$. There are
five main functions used to prove this ($\PCTrim$ omitted as it's unnecessary):

- $\PCSetup(\l, D)^\rho \to \pp_\PC$

  The setup routine. Given security parameter $\l$ in unary and a maximum
  degree bound $D$. Creates the public parameters $\pp_\PC$.

- $\PCCommit(p: \Fb^{d'}_q[X], d: \Nb, \o: \Option(\Fb_q)) \to \Eb(\Fb_q)$

  Commits to a degree-$d'$ polynomial $p$ with degree bound $d$ where $d'
  \leq d$ using optional hiding $\o$.

- $\PCOpen^\rho(p: \Fb^{d'}_q[X], C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, \o: \Option(\Fb_q)) \to \EvalProof$

  Creates a proof, $\pi \in \EvalProof$, that the degree $d'$ polynomial $p$,
  with commitment $C$, and degree bound $d$ where $d' \leq d$, evaluated at
  $z$ gives $v = p(z)$, using the hiding input $\o$ if provided.

- $\PCCheck^\rho(C: \Eb(\Fb_q), d: \Nb, z: \Fb_q, v: \Fb_q, \pi: \EvalProof) \to \Result(\top, \bot)$

  Checks the proof $\pi$ that claims that the degree $d'$ polynomial $p$,
  with commitment $C$, and degree bound $d$ where $d' \leq d$, evaluates to
  $v = p(z)$.

Any NP-problem, $X \in NP$, with a witness $w$ can be compiled into a
circuit $R_X$. This circuit can then be fed to a general-purpose proof scheme
prover $\Pc_X$ along with the witness and public input $(x, w) \in X$, that
creates a proof of the statement $"R_X(x, w) = \top"$. Simplifying slightly,
they typically consists of a series of pairs representing opening proofs:
$$(q_1 = (C_1, d, z_1, v_1, \pi_1), \dots, q_m = (C_m, d, z_m, v_m, \pi_m))$$
These pairs will henceforth be more generally referred to as _instances_,
$\vec{q} \in \Instance^m$. They can then be verified using $\PCCheck$:
$$\PCCheck(C_1, d, z_1, v_1, \pi_1) \meq \dots \meq \PCCheck(C_m, d, z_m, v_m, \pi_m) \meq \top$$
Along with some checks that the structure of the underlying polynomials
$\vec{p}$, that $\vec{q}$ was created from, satisfies any desired relations
associated with the circuit $R_X$. We can model these relations, or _identities_,
using a function $I_X \in \Instance \to \{ \top, \bot \}$. If,
$$\forall j \in [m] : \PCCheck(C_j, d, z_j, v_j, \pi_j) \meq \top \land I_X(q_j) \meq \top$$
Then the verifier $\Vc_X$ will be convinced that $w$ is a
valid witness for $X$. In this way, a proof of knowledge of a witness for
any NP-problem can be represented as a series of PCS evaluation proofs,
including our desired witness that $s_n = F^n(s_0)$.

A PCS of course also has soundness and completeness properties:

**Completeness:** For every maximum degree bound $D = \poly(\l) \in \Nb$
and publicly agreed upon $d \in \Nb$:
$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      \deg(p) \leq d \leq D, \\
      \PCCheck^\rho(C, d, z, v, \pi) = 1
    \end{array}
  & \quad
    \begin{aligned}
      \rho          &\leftarrow \Uc(\l) \\
      \pp_\PC       &\leftarrow \PCSetup^\rho(1^\l, D), \\
      (p, d, z, \o) &\leftarrow \Ac^\rho(\pp_\PC), \\
      v             &\leftarrow p(z), \\
      C             &\leftarrow \PCCommit^\rho(p, d, \o), \\
      \pi           &\leftarrow \PCOpen^\rho(p, C, d, z, \o)
    \end{aligned}
  \end{array}
\right] = 1.
$$
I.e. an honest prover will always convince an honest verifier.

**Knowledge Soundness:** For every maximum degree bound $D = \poly(\l)
\in \Nb$, polynomial-size adversary $\Ac$ and publicly agreed upon $d$,
there exists an efficient extractor $\Ec$ such that the following holds:
$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      \PCCheck^\rho(C, d, z, v, \pi) = 1 \\
      \Downarrow \\
      C = \PCCommit^\rho(p, d, \o) \\
      v = p(z), \; \deg(p) \leq d \leq D
    \end{array}
  & \quad
    \begin{aligned}
      \rho              &\leftarrow \Uc(\l) \\
      \pp_\PC           &\leftarrow \PCSetup^\rho(1^\l, D) \\
      (C, d, z, v, \pi) &\leftarrow \Ac^\rho(\pp_\PC) \\
      (p, \o)           &\leftarrow \Ec^\rho(\pp_\PC) \\
    \end{aligned}
  \end{array}
\right] \geq 1 - \negl(\lambda).
$$
I.e. for any adversary, $\Ac$, outputting an instance, the knowledge extractor
can recover $p$ such that the following holds: $C$ is a commitment to $p$,
$v = p(c)$, and the degree of $p$ is properly bounded. Note that for this
protocol, we have _knowledge soundness_, meaning that $\Ac$, must actually
have knowledge of $p$ (i.e. the $\Ec$ can extract it).

### Accumulation Schemes

The authors of a 2019 paper[@halo] presented _Halo,_ the first practical
example of recursive proof composition without a trusted setup. Using a
modified version of the Bulletproofs-style Inner Product Argument (IPA),
they present a polynomial commitment scheme. Computing the evaluation of
a polynomial $p(z)$ as $v = \ip{\vec{\vec{p}^{\text{(coeffs)}}}}{\vec{z}}$
where $\vec{z} = (z^0, z^1, \dots, z^{d})$ and $\vec{p}^{\text{(coeffs)}}
\in \Fb^{d+1}$ is the coefficient vector of $p(X)$, using the IPA. However,
since the the vector $\vec{z}$ is not private, and has a certain structure, we
can split the verification algorithm in two: A sub-linear $\PCDLSuccinctCheck$
and linear $\PCDLCheck$. Using the $\PCDLSuccinctCheck$ we can accumulate $n$
instances, and only perform the expensive linear check (i.e. $\PCDLCheck$)
at the end of accumulation.

In the 2020 paper[@pcd] _"Proof-Carrying Data from Accumulation Schemes"_
, that this project heavily relies on, the authors presented a generalized
version of the previous accumulation structure of Halo that they coined
_Accumulation Schemes_. Simply put, given a predicate $\Phi: \Instance \to
\{ \top, \bot \}$, and $m$ representing the number of instances accumulated
for each proof step and may vary for each time $\ASProver$ is called. An
accumulation scheme then consists of the following functions:

- $\ASSetup(\l) \to \pp_\AS$

    When given a security parameter $\l$ (in unary), $\ASSetup$ samples and
    outputs public parameters $\pp_\AS$.

- $\ASProver(\vec{q}: \Instance^m, acc_{i-1}: \Acc) \to \Acc$

    The prover accumulates the instances $\{ q_1, \dots, q_m \}$ in $\vec{q}$
    and the previous accumulator $acc_{i-1}$ into the new accumulator $acc_i$.

- $\ASVerifier(\vec{q}: \Instance^m, acc_{i-1}: \Option(\Acc), acc_i: \Acc) \to \Result(\top, \bot)$

    The verifier checks that the instances $\{ q_1, \dots, q_m \}$ in
    $\vec{q}$ was correctly accumulated into the previous accumulator
    $acc_{i-1}$ to form the new accumulator $acc_i$. The second argument
    $acc_{i-1}$ is modelled as an $\Option$ since in the first accumulation,
    there will be no accumulator $acc_0$. In all other cases, the second
    argument $acc_{i-1}$ must be set to the previous accumulator.

- $\ASDecider(acc_i: \Acc) \to \Result(\top, \bot)$

    The decider performs a single check that simultaneously ensures that all
    the instances $\vec{q}$ accumulated in $acc_i$ satisfy the predicate,
    $\forall j \in [m] : \Phi(q_j) = \top$. Assuming the $\ASVerifier$ has
    accepted that the accumulator, $\acc_i$ correctly accumulates $\vec{q}$
    and the previous accumulator $\acc_{i-1}$.

The completeness and soundness properties for the Accumulation Scheme is
defined below:

**Completeness.** For all (unbounded) adversaries $\Ac$, where $f$ represents
an algorithm producing any necessary public parameters for $\Phi$:
$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      \ASDecider^\rho(\acc_i) = \top \\
      \forall j \in [m] : \Phi^\rho_{\pp_\Phi}(q_j) = \top \\
      \Downarrow \\
      \ASVerifier^\rho(\vec{q}, \acc_{i-1}, \acc_i) = \top \\
      \ASDecider^\rho(\acc) = \top
    \end{array}
    & \quad
    \begin{aligned}
      \rho                  &\leftarrow \Uc(\l) \\
      \pp_\Phi              &\leftarrow f^\rho \\
      \pp_\AS               &\leftarrow \ASSetup^\rho(1^{\l}) \\
      (\vec{q}, \acc_{i-1}) &\leftarrow \Ac^\rho(\pp_\AS, \pp_\Phi) \\
      \acc_i                &\leftarrow \ASProver^{\rho}(\vec{q}, \acc_{i-1})
    \end{aligned}
  \end{array}
\right] = 1.
$$
I.e, ($\ASVerifier, \ASDecider$) will always accept the accumulation performed
by an honest prover.

**Soundness:** For every polynomial-size adversary $\Ac$:
$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      \ASVerifier^\rho(\vec{q}, \acc_{i-1}, \acc_i) = \top \\
      \ASDecider^\rho(\acc_i) = \top \\
      \Downarrow \\
      \ASDecider^\rho(\acc_{i-1}) = \top \\
      \forall j \in [m], \Phi^{\rho}_{\pp_{\Phi}}(q_j) = \top
    \end{array}
    &\quad
    \begin{aligned}
      \rho                          &\leftarrow \Uc(\l) \\
      \pp_\Phi                      &\leftarrow f^{\rho} \\
      \pp_\AS                       &\leftarrow \ASSetup^\rho(1^{\l}) \\
      (\vec{q}, \acc_{i-1}, \acc_i) &\leftarrow \Ac^\rho(\pp_\AS, \pp_\Phi)
    \end{aligned}
  \end{array}
\right] \geq 1 - \text{negl}(\lambda).
$$
I.e, For all efficiently-generated accumulators $acc_{i-1}, acc_i \in \Acc$
and predicate inputs $\vec{q} \in \Instance^m$, if $\ASDecider(acc_i) =
\top$ and $\ASVerifier(\vec{q}_i, acc_{i-1}, acc_i) = \top$ then, with all
but negligible probability, $\forall j \in [m] : \Phi(\pp_\Phi, q_j) = \top$
and $\ASDecider(acc_i) = \top$.

### IVC from Accumulation Schemes

For simplicity, as in the PCS section, we assume we have an underlying NARK[^NARK]
which proof consists of only instances $\pi \in \Proof = \{ \vec{q} \}$. We
assume this NARK has three algorithms:

- $\NARKProver(R: \Circuit, x: \PublicInfo, w: \Witness) \to \Proof$
- $\NARKVerifier(R: \Circuit, x: \PublicInfo, \pi) \to \Result(\top, \bot)$
- $\NARKVerifierFast(R: \Circuit, x: \PublicInfo) \to \Result(\top, \bot)$

The $(\NARKProver, \NARKVerifier)$ pair is just the usual algorithms,
but the verifier may run in linear time. The $\NARKVerifierFast$ _must_
run in sub-linear time however, but may assume each $q_j \in \vec{q}$ is
a valid instance, meaning that $\forall q_j \in \vec{q} : \PCCheck(q_j)
= \top$. This means that $\NARKVerifierFast$ only performs linear checks
to ensure that the instances, $\vec{q}$, representing information about
the witness $w$, satisfies the constraints dictated by the circuit $R$
and the public inputs $x$. It also means that when the $\NARKVerifierFast$
accepts with $\top$, then we don't know that these relations hold until we
also know that all the instances are valid.

Each step in the IVC protocol built from accumulation schemes, consists of the
triple ($s_{i-1}, \pi_{i-1}, \acc_{i-1}$), representing the previous proof,
accumulator and value. As per usual, the base-case is the exception, that
only consists of $s_0$. This gives us the following chain:

\begin{figure}[!H]
\centering
\begin{tikzpicture}[node distance=2.25cm]

  % Nodes
  \node (s0) [node] {$s_0$};
  \node (s1) [node, right=of s0] {$(s_1, \pi_1, \acc_1)$};
  \node (dots) [right=2.75cm of s1] {$\dots$};
  \node (sn) [node, right=4cm of dots] {$(s_n, \pi_n, \acc_n)$};

  % Arrows with labels
  \draw[thick-arrow] (s0) -- node[above] {$\Pc(s_0, \bot, \bot)$} (s1);
  \draw[thick-arrow] (s1) -- node[above] {$\Pc(s_1, \pi_1, \acc_1)$} (dots);
  \draw[thick-arrow] (dots) -- node[above] {$\Pc(s_{n-1}, \pi_{n-1}, \acc_{n-1})$} (sn);

\end{tikzpicture}
\caption{
  A visualization of the relationship between $F, \vec{s}, \vec{\pi}$ and
  $\vec{\acc}$ in an IVC setting using Accumulation Schemes. Where $\Pc$ is
  defined to be $\Pc(s_{i-1}, \pi_{i-1}, \acc_{i-1}) = \IVCProver(s_{i-1},
  \pi_{i-1}, \acc_{i-1}) = \pi_i$, $s_i = F(s_{i-1})$, $\acc_i =
  \ASProver(\vec{q}, \acc_{i-1})$.
}
\end{figure}

Before describing the IVC protocol, we first describe the circuit for the
IVC relation as it's more complex than for the naive SNARK-based approach. Let:

- $\pi_{i-1} = \vec{q}, \acc_{i-1}, s_{i-1}$ from the previous iteration.
- $s_i = F(s_{i-1})$
- $\acc_i = \ASProver(\vec{q}, \acc_{i-1})$

Giving us the public inputs $x = \{ R_{IVC}, s_0, s_i, \acc_i \}$ and witness
$w = \{ s_{i-1}, \pi_{i-1} = \vec{q}, \acc_{i-1} \}$, which will be used to
construct the the IVC circuit $R_{IVC}$:
$$
\begin{aligned}
  x_{i-1} &:= \{ R_{IVC}, s_{i-1}, \acc_{i-1} \} \\
  \Vc_1   &:= \NARKVerifierFast(R_{IVC}, x_{i-1}, \pi_{i-1}) \meq \top \\
  \Vc_2   &:= \ASVerifier(\pi_{i-1} = \vec{q}, \acc_{i-1}, \acc_i) \meq \top \\
  R_{IVC} &:= \text{I.K } w \text{ s.t. } F(s_{i-1}) \meq s_i \land (s_{i-1} \meq s_0 \lor ( \Vc_1 \land \Vc_2 ) ) \\
\end{aligned}
$$
\begin{figure}[H]
\centering
\begin{tikzpicture}
  % First Layer
  \node[draw, rectangle] (q) at (6, 6.5) {$\vec{q}$};
  \node[draw, rectangle] (acc_prev) at (7.5, 6.5) {$\acc_{i-1}$};
  \node[draw, rectangle] (acc_next) at (9, 6.5) {$\acc_i$};

  \node[draw, rectangle] (R_ivc) at (2.25, 6.5) {$R_{IVC}$};
  \node[draw, rectangle] (x_prev) at (3.5, 6.5) {$x_{i-1}$};
  \node[draw, rectangle] (pi_prev) at (4.75, 6.5) {$\pi_{i-1}$};

  \node[draw, rectangle] (s_next) at (-1.5, 6.5) {$s_i$};
  \node[draw, rectangle] (s_prev) at (-0.25, 6.5) {$s_{i-1}$};
  \node[draw, rectangle] (s_0) at (1, 6.5) {$s_0$};

  \draw[dashed-arrow] (pi_prev) -- (4.75, 7) -- (6, 7) -- (q);

  \draw[dashed-arrow] (R_ivc) -- (2.25, 7) -- (3.5, 7) -- (x_prev);
  \draw[dashed-arrow] (s_prev) -- (-0.25, 7.1) -- (3.5, 7.1) -- (x_prev);
  \draw[dashed-arrow] (acc_prev) -- (7.5, 7.2) -- (3.5, 7.2) -- (x_prev);

  % Second Layer
  \node[draw, rectangle] (svf) at (3.5, 5.5) {$\NARKVerifierFast$};
  \node[draw, rectangle] (asv) at (7.5, 5.5) {$\ASVerifier$};

  \draw[arrow] (R_ivc) -- (svf);
  \draw[arrow] (x_prev) -- (svf);
  \draw[arrow] (pi_prev) -- (svf);

  \draw[arrow] (q) -- (asv);
  \draw[arrow] (acc_prev) -- (asv);
  \draw[arrow] (acc_next) -- (asv);

  % Third Layer
  \node[draw, rectangle] (asv_svf_and) at (5.75, 4.5) {$\land$};
  \node[draw, rectangle] (base_case) at (1, 4.5) {$s_{i-1} \meq s_0$};

  \draw[arrow] (asv) -- (asv_svf_and);
  \draw[arrow] (svf) -- (asv_svf_and);

  \draw[arrow] (s_prev) -- (base_case);
  \draw[arrow] (s_0) -- (base_case);

  % Fourth Layer
  \node[draw, rectangle] (or) at (4, 3.5) {$\lor$};
  \node[draw, rectangle] (F) at (-1, 3.5) {$F(s_{i-1}) \meq s_i$};

  \draw[arrow] (asv_svf_and) -- (or);
  \draw[arrow] (base_case) -- (or);

  \draw[arrow] (s_next) -- (F);
  \draw[arrow] (s_prev) -- (F);

  % Fifth Layer
  \node[draw, rectangle] (end_and) at (3, 2.5) { $\land$ };
  \draw[arrow] (or) -- (end_and);
  \draw[arrow] (F) -- (end_and);

\end{tikzpicture}
\caption{A visualization of $R_{IVC}$}
\end{figure}

The verifier and prover for the IVC scheme can be seen below:

\begin{algorithm}[H]
\caption*{\textbf{Algorithm} $\IVCProver$}
\textbf{Inputs} \\
  \Desc{$R_{IVC}: \Circuit$}{The IVC circuit as defined above.} \\
  \Desc{$x: \PublicInputs$}{Public inputs for $R_{IVC}$.} \\
  \Desc{$w: \Option(\Witness)$}{Private inputs for $R_{IVC}$.} \\
\textbf{Output} \\
  \Desc{$(S, \Proof, \Acc)$}{The values for the next IVC iteration.}
\begin{algorithmic}[1]
  \Require $x = \{ s_0 \}$
  \Require $w = \{ s_{i-1}, \pi_{i-1}, \acc_{i-1} \} \lor w = \bot$
  \State Parse $s_0$ from $x = \{ s_0 \}$.
  \If{$w = \bot$}
    \State $w = \{ s_{i-1} = s_0 \}$ (base-case).
  \Else
    \State Run the accumulation prover: $\acc_i = \ASProver(\pi_{i-1} = \vec{q}, \acc_{i-1})$.
    \State Compute the next value: $s_i = F(s_{i-1})$.
    \State Define $x' = x \cup \{ R_{IVC}, s_i, \acc_i \}$.
  \EndIf
  \State Then generate a NARK proof $\pi_i$ using the circuit $R_{IVC}$: $\pi_i = \NARKProver(R_{IVC}, x', w)$.
  \State Output $(s_i, \pi_i, \acc_i)$
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{\textbf{Algorithm} $\IVCVerifier$}
\textbf{Inputs} \\
  \Desc{$R_{IVC}: \Circuit$}{The IVC circuit.} \\
  \Desc{$x: \PublicInputs$}{Public inputs for $R_{IVC}$.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Returns $\top$ if the verifier accepts and $\bot$ if the verifier rejects.}
\begin{algorithmic}[1]
  \Require $x = \{ s_0, s_i, \acc_i \}$
  \State Define $x' = x \cup \{ R_{IVC} \}$.
  \State Verify that the accumulation scheme decider accepts: $\top \meq \ASDecider(\acc_i)$.
  \State Verify the validity of the IVC proof: $\top \meq \NARKVerifier(R_{IVC}, x', \pi_i)$.
  \State If the above two checks pass, then output $\top$, else output $\bot$.
\end{algorithmic}
\end{algorithm}

Consider the above chain run $n$ times. As in the "simple" SNARK IVC
construction, if $\IVCVerifier$ accepts at the end, then we get a chain
of implications:
$$
\begin{alignedat}[b]{2}
  &\IVCVerifier(R_{IVC}, x_n = \{ s_0, s_n, \acc_i \}, \pi_n) = \top           &&\then \\
  &\forall i \in [n], \forall q_j \in \pi_i = \vec{q} : \PCDLCheck(q_j) = \top &&\;\; \land \\
  &F(s_{n-1}) = s_n     \land (s_{n-1} = s_0 \lor ( \Vc_1 \land \Vc_2 ))       &&\then \\
  &\ASVerifier(\pi_{n-1}, \acc_{n-1}, \acc_n) = \top                           &&\;\; \land \\
  &\NARKVerifierFast(R_{IVC}, x_{n-1}, \pi_{n-1}) = \top                      &&\then \dots \\
  &F(s_0) = s_1 \land (s_0 = s_0 \lor ( \Vc_1 \land \Vc_2 ))                   &&\then \\
  &F(s_0) = s_1                                                                &&\then \\
\end{alignedat}
$$
Since $\IVCVerifier$ runs $\ASDecider$, the previous accumulator is valid,
and by recursion, all previous accumulators are valid, given that each
$\ASVerifier$ accepts. Therefore, if a $\ASVerifier$ accepts, that means that
$\vec{q} = \pi_i$ are valid evaluation proofs. We defined $\NARKVerifierFast$,
s.t. it verifies correctly provided the $\vec{q}$'s are valid evaluation
proofs. This allows us to recurse through this chain of implications.

From this we learn:

1. $\forall i \in [2, n] : \ASVerifier(\pi_{i-1}, \acc_{i-1}, \acc_i) = \top$, i.e, all accumulators are accumulated correctly.
2. $\forall i \in [2, n] : \NARKVerifierFast(R_{IVC}, x_{i-1}, \pi_{i-1})$, i.e, all the proofs are valid.

These points in turn imply that $\forall i \in [n] : F(s_{i-1}) = s_i$,
therefore, $s_n = F^n(s_0)$. From this discussion it should be clear that an
honest prover will convince an honest verifier, i.e. completeness holds. As
for soundness, it should mostly depend on the soundness of the underlying PCS,
accumulation scheme and NARK[^unsoundness].

As for efficiency, assuming that:

- The runtime of $\NARKProver$ scales linearly with the degree-bound, $d$, of the polynomial, $p_j$, used for each $q_j \in \vec{q}_m$ ($\Oc(d)$)
- The runtime of $\NARKVerifierFast$ scales logarithmically with the degree-bound, $d$, of $p_j$ ($\Oc(\lg(d))$)
- The runtime of $\NARKVerifier$ scales linearly with the degree-bound, $d$, of $p_j$ ($\Oc(d)$)
- The runtime of $F$ is less than $\Oc(d)$, since it needs to be compiled to a circuit of size at most $\approx d$

Then we can conclude:

- The runtime of $\IVCProver$ is:
  - Step 5: The cost of running $\ASDLProver$, $\Oc(d)$.
  - Step 6: The cost of computing $F$, $\Oc(F(x))$.
  - Step 7: The cost of running $\NARKProver$, $\Oc(d)$.

  Totalling $\Oc(F(x) + d)$. So $\Oc(d)$.
- The runtime of $\IVCVerifier$ is:
  - Step 2: The cost of running $\ASDLDecider$, $\Oc(d)$ scalar multiplications.
  - Step 3: The cost of running $\NARKVerifier$, $\Oc(d)$ scalar multiplications.

  Totalling $\Oc(2d)$. So $\Oc(d)$

Notice that although the runtime of $\IVCVerifier$ is linear, it scales
with $d$, _not_ $n$. So the cost of verifying does not scale with the number
of iterations.

[^unsoundness]: A more thorough soundness discussion would reveal that running
the extractor on a proof-chain of length $n$ actually fails, as argued by
Valiant in his original 2008 paper. Instead he constructs a proof-tree of
size $\Oc(\lg(n))$ size, to circumvent this. However, practical applications
conjecture that the failure of the extractor does not lead to any real-world
attack, thus still achieving constant proof sizes, but with an additional
security assumption added.

[^NARK]: Technically it's a NARK since verification may be linear.

## Cycle of Curves

- Motivation
- Graph
- Description

## Poseidon Hash

- Reference
