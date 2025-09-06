# Prerequisites

Basic knowledge of elliptic curves, groups and interactive arguments
is assumed in the following text. Basic familiarity with SNARKs is also
assumed. The polynomial commitment scheme and accumulation scheme used was
implemented and analyzed in a previous paper[@halo-accumulation] by one of
the authors of this paper.

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

## Notation

The following table denotes the meaning of the notation used throughout
the document:

|                                                                                 |                                                                                                           |
|:--------------------------------------------------------------------------------|:----------------------------------------------------------------------------------------------------------|
| $[n]$                                                                           | Denotes the integers $\{ 1, ..., n \}$                                                                    |
| $a \in \Fb_q$                                                                   | A field element of an unspecified field                                                                   |
| $a \in \Fb_q$                                                                   | A field element in a prime field of order $q$                                                             |
| $\vec{a} \in S^n_q$                                                             | A vector of length $n$ consisting of elements from set $S$                                                |
| $G \in \Eb(\Fb_q)$                                                              | An elliptic Curve point, defined over field $\Fb_q$                                                       |
| $G \in \Eb_p(\Fb_q)$                                                            | An elliptic Curve point, defined over field $\Fb_q$, where the curve has order $p$                        |
| $(a_1, \dots, a_n) = [x_i]^n = [x_i]_{i=1}^n = \vec{a} \in S^n_q$               | A vector of length $n$                                                                                    |
| $a \in_R S$                                                                     | $a$ is a uniformly randomly sampled element of $S$                                                        |
| $(S_1, \dots, S_n)$                                                             | In the context of sets, the same as $S_1 \times \dots \times S_n$                                         |
| $\vec{a} \cat \vec{b}$ where $\vec{a} \in \Fb^n_q, \vec{b} \in \Fb^m_q$         | Concatenate vectors to create $\vec{c} \in \Fb^{n+m}_q$.                                                  |
| $a \cat b$ where $a \in \Fb_q$                                                  | Create vector $\vec{c} = (a, b)$.                                                                         |
| I.K $w$                                                                         | "I Know", Used in the context of proof claims, meaning I have knowledge of the witness $w$                |
| $\Bb$                                                                           | A boolean, i.e. $\{ \bot, \top \}$                                                                        |
| $\Option(T)$                                                                    | $\{ T, \bot \}$                                                                                           |
| $\Result(T, E)$                                                                 | $\{ T, E \}$                                                                                              |

Note that the following are isomorphic $\{ \top, \bot \} \iso \Bb
\iso \Option(\top) \iso \Result(\top, \bot)$, but they have different
connotations. Generally for this report, $\Option(T)$ models optional
arguments, where $\bot$ indicates an empty argument and $\Result(T, \bot)$
models the result of a computation that may fail, especially used for
rejecting verifiers.

## Proof Systems

An Interactive Proof System consists of two Interactive Turing Machines:
a computationally unbounded Prover, $\Pc$, and a polynomial-time bounded
Verifier, $\Vc$. The Prover tries to convince the Verifier of a statement
$X \in L$, with language $L$ in NP. The following properties must be true:

- **Completeness:** $\forall \Pc \in ITM, X \in L \implies \Pr[\Vc_{out} = \bot] \leq \epsilon(X)$

  For all honest provers, $\Pc$, where $X$ is true, the probability that the
  verifier remains unconvinced ($\Vc_{out} = \bot$) is negligible in the
  length of $X$.

- **Soundness:** $\forall \Pc^* \in ITM, X \notin L \implies \Pr[\Vc_{out} = \top] \leq \epsilon(X)$

  For all provers, honest or otherwise, $\Pc^*$, that try to convince the
  verifier of a claim, $X$, that is not true, the probability that the verifier
  will be convinced ($\Vc_{out} = \top$) is negligible in the length of $X$.

An Interactive Argument is very similar, but the honest and malicious prover
are now polynomially bounded and receive a Private Auxiliary Input, $w$,
not known by $\Vc$. This is such that $\Vc$ can't just compute the answer
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

Where two distributions $D_1, D_2$ are:

- _Perfectly indistinguishable_ if they are identical, meaning no observer,
  even with unbounded power, can tell them apart:  
  $\forall x : \Pr[D_1 = x] = \Pr[D_2 = x]$
- _Statistically indistinguishable_ if their statistical distance is negligible,
  meaning that they may differ, but the difference is vanishingly small,
  even for an unbounded adversary:  
  $\forall x : \Delta(D_1, D_2) := \tfrac{1}{2} \sum_x \big|\Pr[D_1 = x] - \Pr[D_2 = x]\big| \leq \text{negl}(\l)$
- _Computationally indistinguishable_ if no probabilistic polynomial-time
  distinguisher $\Ac$ can tell them apart with more than negligible advantage,
  though an unbounded adversary might:  
  $\forall x : | \Pr[\Ac(x) \to D_1] - \Pr[\Ac(x) = D_2] | \leq \negl(\l)$

## Fiat-Shamir Heuristic

The Fiat-Shamir heuristic turns a public-coin (an interactive protocol where
the verifier only sends uniformly sampled challenge values) interactive
proof into a non-interactive proof, by replacing all uniformly random
values sent from the verifier to the prover with calls to a non-interactive
random oracle. In practice, a cryptographic hash function, $\rho$, is
used. Composing proof systems will sometimes require *domain-separation*,
whereby random oracles used by one proof system cannot be accessed by another
proof system. In practice one can have a domain specifier, for example $0,
1$, prepended to each message that is hashed using $\rho$:
$$\rho_0(m) = \rho(0 \cat m), \quad \rho_1(m) = \rho(1 \cat m)$$

## Pedersen Commitments

A commitment scheme is a cryptographic primitive that allows one to commit
to a chosen value while keeping it hidden to others, with the ability to
reveal the committed value later. Commitment schemes are designed so that
a the committing party cannot change the value after they have committed to
it, i.e. it is _binding_. The fact that anyone that receives the commitment
cannot compute the value from the it is called _hiding_.

To reveal a value one can simply send the value to a party that previously
received the commitment, and the receiving party can compute the commitment
themselves and compare to the previously received commitment. One such
homomorphic commitment scheme is the _Pedersen commitment scheme_[@pedersen]:

\begin{algorithm}[H]
\caption*{\textbf{Algorithm:} $\CMCommit$}
\textbf{Inputs} \\
  \Desc{$\vec{m}: \Fb^n$}{The vectors we wish to commit to.} \\
  \Desc{$\pp_\CM$}{The public parameters for the commitment scheme.} \\
  \Desc{$\o: \Option(\Fb)$}{Optional hiding factor for the commitment.} \\
\textbf{Output} \\
  \Desc{$C: \Eb(\Fb_q)$}{The Pedersen commitment.}
\begin{algorithmic}[1]
  \State Parse $\vec{G}: \Eb(\Fb)^n, S: \Eb(\Fb)$ from $\pp_\CM$.
  \State Output $C := \ip{\vec{m}}{\vec{G}} + \o S$.
\end{algorithmic}
\end{algorithm}

Notice, that the inputs is a vector of messages, not just a single
message. Inclusion of a hiding factor makes the commitment _perfectly
hiding_, but _computationally binding_. If the hiding factor is omitted, it
is commonly called a _deterministic Pedersen commitment_ and the commitment
will be perfectly binding and computationally hiding. For $C = \CMCommit(m, \vec{G}, \o)$

- **Perfect Hiding:** Given $C$, it is impossible to determine $m$, no matter your computational power.
- **Computational Hiding:** It is computationally infeasible to determine the value committed to, from the commitment
- **Perfect Binding:** It is impossible to change the value committed to, no matter your computational power.
- **Computational Binding:** It is computationally infeasible to change the value committed to.

The corresponding setup algorithm is:

\begin{algorithm}[H]
\caption*{\textbf{Algorithm:} $\CMSetup^{\rho}$}
\textbf{Inputs} \\
  \Desc{$\l$}{The security parameter, in unary form.} \\
  \Desc{$L$}{The maximum size vector that can be committed to.} \\
\textbf{Output} \\
  \Desc{$\pp_\CM$}{The public parameters to be used in $\CMCommit$}
\begin{algorithmic}[1]
  \State $(\Eb(\Fb_q), q, G) \from \text{SampleGroup}^{\rho}(1^\l)$
  \State Choose independently uniformly-sampled generators in $\Eb(\Fb_q)$, $\vec{G} \in_R \Eb(\Fb_q)^L, S \in_R \Eb(\Fb_q)$ using $\rho_0$.
  \State Output $\pp_\CM = ((\Eb(\Fb_q), q, G), \vec{G}, S)$
\end{algorithmic}
\end{algorithm}

Pedersen commitments are an instance of a very useful type of commitment
scheme for proof systems is that of a _homomorphic commitment scheme_, where:

$$
\begin{aligned}
  C_1 &= \CMCommit(m_1, r_1) \\
  C_2 &= \CMCommit(m_2, r_2) \\
  C_3 &= \CMCommit(m_1 + m_2, r_1 + r_2) \\
  C_3 &= C_1 + C_2
\end{aligned}
$$

That is, you can add the commitments which corresponds to adding the committed
inputs and then commititing. This lets a verifying party check the properties
of committed values without needing to know them. Since the public parameters
can be chosen uniformly randomly, this type of setup is _untrusted_.

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
context) and $G$ is a generator of $\Eb(\Fb)$. The URS must consist solely
of generators and all the scalars must be uniformly random. $\PCDL$ is then
sound, provided that no adversary knows the scalars. Extracting $\vec{a}$
from the URS would require solving the Discrete Logarithm problem (DL),
which is assumed to be hard.

To generate the URS transparently, a collision-resistant hash function
$\Hc : \Bb^* \to \Eb(\Fb_q)$ can be used to produce the generators. The URS
can then be derived using a genesis string $s$:
$$\text{URS} = \{ \Hc(s \cat 1), \Hc(s \cat 2), \dots, \Hc(s \cat D) \}$$
The genesis string can be any arbitrary string, that convinces outsiders
that it's not maliciously chosen. This is commonly refered to as
nothing-up-my-sleeve numbers. We used the string:

\begin{quote}
\color{GbGrey}

\textit{To understand recursion, one must first understand recursion.}

\end{quote}

Anyone can verify that the URS was generated from this string, and the
probability that such a specific string, hashed, would lead to a known
discrete log, should be negligible.

## SNARKS

SNARKs - Succinct Non-interactive Arguments of Knowledge - have seen increased
usage due to their application in blockchains and cryptocurrencies. They
also typically function as general-purpose proof schemes. This means that,
given any solution to an NP-problem, the SNARK prover will produce a proof
that they know the solution to said NP-problem. Most SNARKs also allow for
zero-knowledge arguments, making them zk-SNARKs.

More concretely, imagine that Alice has today's Sudoku problem $X \in
\text{NP}$: She claims to have a solution to this problem, her witness, $w$,
and wants to convince Bob without having to reveal the entire solution. She
could then use a SNARK to generate a proof for Bob. To do this she must
first encode the Sudoku verifier as a circuit $R_X$, then let $x$ represent
public inputs to the circuit, such as today's Sudoku values/positions, etc,
and then give the SNARK prover the public inputs and her witness, $\pi =
\SNARKProver(R_X, x, w)$. Finally she sends this proof, $\pi$, to Bob along
with the public Sudoku verifying circuit, $R_X$, and he can check the proof
and be convinced using the SNARK verifier ($\SNARKVerifier(R_X, x, \pi)$).

Importantly, the 'succinct' property means that the proof size and
verification time must be sub-linear. This allows SNARKs to be directly used
for _Incrementally Verifiable Computation_.

## Bulletproofs

In 2017, the Bulletproofs paper[@bulletproofs] was
released[^bulletproofs-intro]. Bulletproofs rely on the hardness of the
Discrete Logarithm problem, and uses an untrusted setup. It has logarithmic
proof size, linear verification time and lends itself well to efficient
range proofs. It's also possible to generate proofs for arbitrary circuits,
yielding a zk-NARK. It's a NARK since we lose the succinctness in terms of
verification time, making Bulletproofs less efficient than SNARKs.

At the heart of Bulletproofs lies the Inner Product Argument (IPA), wherein a
prover demonstrates knowledge of two vectors, $\vec{a}, \vec{b} \in \Fb_q^n$,
with commitment $P \in \Eb(\Fb_q)$, and their corresponding inner product,
$c = \ip{\vec{a}}{\vec{b}}$. It creates a non-interactive proof, with only
$\lg(n)$ size, by compressing the point and vectors $\lg(n)$ times, halving
the size of the vectors each iteration in the proof. Unfortunately, since the IPA,
and by extension Bulletproofs, suffer from linear verification time,
Bulletproofs are unsuitable for IVC.

[^bulletproofs-intro]: A gentle introduction can be found in "From Zero
(Knowledge) to Bulletproofs"[@from0k2bp], which also describes Pedersen
commitments and the concept of zero-knowledge.

## Incrementally Verifiable Computation

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
values. Each state, $s_i$, including the genesis state, $s_0$, must also
contain the current iteration, $i$, for soundness to hold. The $\SNARKVerifier$
represents the verification circuit in the proof system we're using. This
means, that we're taking the verifier, representing it as a circuit, and then
feeding it to the prover. This is not a trivial task in practice! Note also,
that the verification time must be sub-linear to achieve an IVC scheme,
otherwise the verifier could just have computed $F^n(s_0)$ themselves,
as $s_0$ and $F(x)$ necessarily must be public.

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

## The Schwarz-Zippel Lemma

The Schwarz-Zippel lemma is commonly used in succinct proof systems to test
polynomial identities. Formally it states:

$$\xi \in_R \Fb : \Pr[p(\xi) = 0 \mid p(X) \neq 0] = \frac{d}{\Fb}$$

Meaning that if $p(X)$ is not the zero-polynomial, the evaluation at a
uniformly random point from $\Fb$, will equal zero with at most $d \, / \, \Fb$
probability. This can also be used to check equality between polynomials:

$$
\begin{aligned}
  \xi &\in_R \Fb \\
  r(X) &= p(X) - q(X) \\
  r(\xi) &\meq 0
\end{aligned}
$$

Or equivalently:

$$p(\xi) \meq q(\xi)$$

Meaning that $p(X) = q(X)$ with probability at least $d \, / \, \Fb$.

## Polynomial Interpolation

It is well known that given $d+1$ evaluations, an evaluation domain,
$\vec{p^{(e)}} := [ p^{(e)}_1, \dots, p^{(e)}_{d+1} ]$, of a polynomial
$p(X)$, you can reconstruct the polynomial using lagrange interpolation:

$$p(X) = \lagrange(\vec{p^{(e)}})$$

With a worst-case runtime of $\Oc(n^2)$. However, if the evaluation points
are chosen to be the $n$-th roots of unity, i.e. the set:
$$\{ \o^1, \o^2, \dots, \o^n \}$$
where $\o$ is a primitive $n$-th root of unity, then interpolation can be
reduced to applying a discrete Fourier transform. We can then choose $n \geq
d+1$, and evaluate at all $n$ points of the domain. Setting $n$ to be the next
power of 2 above $d+1$ allows us to use the very very efficient radix-2 FFT:

$$
\begin{aligned}
  \vec{p^{(e)}} &:= [ p^{(e)}_1, \dots, p^{(e)}_{n} ] \\
  p(X)          &= \ifft(\vec{p^{(e)}}) \\
  \vec{p^{(e)}} &= \fft(p(X))
\end{aligned}
$$

Where both the evaluation domain and polynomial can be computed efficiently
using the Fast Fourier Transform in time $\Oc(n \log n)$. This approach can
be used whenever the underlying field contains a primitive $n$-th root of
unity and $n$ is invertible in the field, which is the case for many finite
fields used in cryptography including the fields used in this project.

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

A PCS has soundness and completeness properties, as well as a binding property:

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

**Binding:** For every maximum degree bound $D = \poly(\l)
\in \Nb$ and publicly agreed upon $d$, no polynomial-size adversary $\Ac$
can find two polynomials s.t:

$$
\Pr \left[
  \begin{array}{c|c}
    \begin{array}{c}
      p_1 \in \Fb[X]_{\leq d}, \;
      p_2 \in \Fb[X]_{\leq d}, \;
      p_1 \neq p_2 \\
      \wedge \\
      C_1 = C_2
    \end{array}
  &
    \begin{aligned}
      \rho                      &\leftarrow \Uc(\l) \\
      \pp_\PC                   &\leftarrow \PCSetup^\rho(1^\l, D) \\
      (p_1, p_2, d, \o_1, \o_2) &\leftarrow \Ac^\rho(\pp_\PC) \\
      C_1                       &\leftarrow \PCCommit(p_1, d, \o_1) \\
      C_2                       &\leftarrow \PCCommit(p_2, d, \o_2) \\
    \end{aligned}
  \end{array}
\right] \leq \negl(\lambda).
$$

I.e. The adversary cannot change the polynomial that he committed to.

## Accumulation Schemes

In 2019 _Halo_[@halo] was introduced, the first practical example of recursive
proof composition without a trusted setup. Using a modified version of the
Bulletproofs-style Inner Product Argument (IPA), they present a polynomial
commitment scheme. Computing the evaluation of a polynomial $p(z)$ as
$v = \ip{\vec{\vec{p}^{\text{(coeffs)}}}}{\vec{z}}$ where $\vec{z} =
(z^0, z^1, \dots, z^{d})$ and $\vec{p}^{\text{(coeffs)}} \in \Fb^{d+1}$
is the coefficient vector of $p(X)$, using the IPA. However, since the the
vector $\vec{z}$ is not private, and has a certain structure, we can split
the verification algorithm in two: A sub-linear $\PCDLSuccinctCheck$ and
linear $\PCDLCheck$. Using the $\PCDLSuccinctCheck$ we can accumulate $n$
instances, and only perform the expensive linear check (i.e. $\PCDLCheck$)
at the end of accumulation.

In 2020 a paper[@pcd] was released where the authors presented a generalized
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

## Cycles of Curves

To simplify elliptic curve operations, a _cycle of curves_ can be used. A
cycle of curves use the other's scalar field as their base field and
vice-versa. This means that field operations can be handled natively in the
scalar field circuit $\Fb_S$ and elliptic curve operations are handled natively
in the basefield circuit $\Fb_B$. This improves performance drastically,
since the SNARK never need to handle foreign field arithmetic. The cycle of
curves used in this project is the Pasta curves[@pasta], Pallas and Vesta,
both of which have the curve equation $y^2 = x^3 + 5$:

- Pallas: $a \in \Fb_p, P \in \Eb_p(\Fb_q)$
- Vesta:  $a \in \Fb_q, P \in \Eb_q(\Fb_p)$

Where:

- $| \Fb_p | = p, | \Fb_q | = q, | \Eb_p(\Fb_q) | = p, | \Eb_p(\Fb_q) | = q, p > q$
- $p = 2^{254} + 45560315531419706090280762371685220353$
- $q = 2^{254} + 45560315531506369815346746415080538113$

This is useful when creating proofs. Starting in the first proof in an
IVC-setting, we need a proof that verifies some relation, the simplest
minimal example would be $R := a \cdot P \meq \Oc$. This then creates two
constraint tables and two proofs, one over $\Fb_S = \Fb_p$ and one over
$\Fb_B = \Fb_q$. Then, in the next IVC-step, we need to verify both proofs,
but the proof over $\Fb_p$ produces scalars over $\Fb_p$ and points over
$\Eb_p(\Fb_q)$ and the proof over $\Fb_q$ produces scalars over $\Fb_q$ and
points over $\Eb_p(\Fb_q)$. This is because a proof of $R$ needs to contain
both scalars and points. If we did _not_ have a cycle of curves this pattern
would result in a chain:

- Curve 1: $a \in \Fb_{p_1}, P \in \Eb_{p_1}(\Fb_{p_2})$
- Curve 2: $a \in \Fb_{p_2}, P \in \Eb_{p_2}(\Fb_{p_3})$
- Curve 3: $a \in \Fb_{p_3}, P \in \Eb_{p_3}(\Fb_{p_4})$
- ...

Which means that each $p_i$ must be able to define a valid curve, and if
this never cycles, we would need to support this infinite chain of curves. 

## Poseidon Hash

Traditionally, SNARKs are defined over somewhat large prime fields, ill-suited
for bit-level operation such as XOR. As such, many modern cryptographic
hash functions, particularly SHA3, are not particularly well suited for use
in many SNARK circuits. The Poseidon[@poseidon] Hash specification aims to
solve this. The specification defines a cryptographic sponge construction,
with the state permutation only consisting of native field operations. This
makes poseidon in SNARKs much more efficient. The cryptographic sponge
structure, is also particularly well suited for Fiat-Shamir, as messages
from the prover to the verifier can be modelled with sponge absorption and
challenge messages from the verifier to the prover can be modelled with
sponge squeezing. Poseidon is still a very new hash function though, and
is not nearly as used and "battle-tested" as SHA3, so using it can pose a
potential security risk compared to SHA3.
