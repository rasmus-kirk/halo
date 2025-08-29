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

## Proof Systems

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

## Fiat-Shamir Heuristic

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

## SNARKS

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

## Trusted and Untrusted Setups

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

## Bulletproofs

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

## Incrementally Verifiable Computation

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

## Polynomial Commitment Schemes

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

## Accumulation Schemes

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
, the authors presented a generalized version of the previous accumulation
structure of Halo that they coined _Accumulation Schemes_. Simply put, given
a predicate $\Phi: \Instance \to \{ \top, \bot \}$, and $m$ representing
the number of instances accumulated for each proof step and may vary for
each time $\ASProver$ is called. An accumulation scheme then consists of
the following functions:

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

## Cycles of Curves

We operate our IVC-circuit over a cycle of curves. This means that field
operations can be handled natively in the scalar field circuit $\Fb_S$
and elliptic curve operations are handled natively in the basefield circuit
$\Fb_B$. This improves performance drastically, since we don't need to handle
foreign field arithmetic at any point. The Pallas and Vesta curves use the
other's scalar field as their base field and vice-versa:

- Pallas: $a \in \Fb_p, P \in \Eb_p(\Fb_q)$
- Vesta:  $a \in \Fb_q, P \in \Eb_q(\Fb_p)$
- $| \Fb_p | = p , | \Fb_q | = q, | \Eb_p(\Fb_q) | = p, p > q$

This is useful when creating proofs. Starting in the first proof in an
IVC-setting, we need a proof that verifies some relation, the simplest
minimal example would be $a \cdot P \meq \Oc$. This then creates two constraint
tables, one over $\Fb_S = \Fb_p$ and one over $\Fb_B = \Fb_B$. Then, in the
next IVC-step, we need to verify both proofs, but the proof over $\Fb_p$
produces scalars over $\Fb_p$ and points over $\Eb_p(\Fb_q)$ and the proof
over $\Fb_q$ produces scalars over $\Fb_q$ and points over $\Eb_p(\Fb_q)$. This
is because the proof both contains scalars and points. If we did _not_
have a cycle of curves this pattern would result in a chain:

- Curve 1: $a \in \Fb_{p_1}, P \in \Eb_{p_1}(\Fb_{p_2})$
- Curve 2: $a \in \Fb_{p_2}, P \in \Eb_{p_2}(\Fb_{p_3})$
- Curve 3: $a \in \Fb_{p_3}, P \in \Eb_{p_3}(\Fb_{p_4})$
- ...

Which means that each $p_i$ must be able to define a valid curve, and if
this never cycles, we would need to support this infinite chain of curves. 

## Poseidon Hash

Traditionally, SNARKs are defined over somewhat large prime fields, ill-suited
for bit-level operation such as XOR. As such, many modern cryptographic hash
functions, particularly SHA3, are not particularly well suited for use in
many SNARK circuits. The Poseidon Hash specification aims to solve this. The
specification defines a cryptographic sponge construction, with the state
permutation only consisting of native field operations. This makes use of
poseidon in SNARKs much more efficient. The cryptographic sponge structure,
is also particularly well suited for Fiat-Shamir, as messages from the prover
to the verifier can be modelled with sponge absorbtion and challenge messages
from the verifier to the prover can be modelled with sponge squeezing. Poseidon
is still a very new hash function though, and is not nearly as used and
"battle-tested" as SHA3, so using it can pose a potential security risk
compared to SHA3.
