# IVC Scheme

**Draft Note:** The idea here is to combine these two sections into something coherent...

## IVC from Accumulation Schemes (Stub)

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

## The New IVC-Scheme

We now operate over two curves, with two accumulators, proofs and a single state:

$s_i, \acc_i = (\acc_{i}^{(p)}, \acc_{i}^{(q)}), \pi_i = (\pi_{i}^{(p)}, \pi_{i}^{(q)})$

Which means that the IVC state chain remains unchanged:

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
  $\vec{\acc}$ in an IVC setting using Accumulation Schemes and a cycle of
  curves. Where $\Pc$ is defined to be $\Pc(s_{i-1}, \pi_{i-1}, \acc_{i-1})
  = \IVCProver(s_{i-1}, \pi_{i-1}, \acc_{i-1}) \to (s_i, \pi_i, \acc_i)$.
}
\end{figure}

But the circuit needs to be updated. Let:

$$
\begin{aligned}
  c_{\text{VF}_p} &= \NARKVerifierFast(R_{IVC}^{(q)}, x_{i-1}^{(p)}, \pi_{i-1}^{(p)}) \meq \top \\
  c_{\text{AS}_p} &= \ASVerifier(\vec{q}^{(p)}, \acc_{i-1}^{(p)}, \acc_i^{(p)}) \meq \top \\
  c_{\text{VF}_q} &= \NARKVerifierFast(R_{IVC}^{(q)}, x_{i-1}^{(q)}, \pi_{i-1}^{(q)}) \meq \top \\
  c_{\text{AS}_q} &= \ASVerifier(\vec{q}^{(q)}, \acc_{i-1}^{(q)}, \acc_i^{(q)}) \meq \top \\
  c_V &= c_{\text{VF}_p} \land c_{\text{AS}_p} \land c_{\text{VF}_q} \land c_{\text{AS}_q} \\ 
  c_0 &= s_{i-1}^{(i)} \meq 0 \\
  c_F &= F(s_{i-1}) \meq s_i \\
  c_{\text{IVC}} &= (c_0 \lor c_V) \land c_F
\end{aligned}
$$

We also need to check that the next $i$ is equals to the previous $i$
incremented once, but this can be modelled as part of the state transition
check function $F'$.

\begin{figure}[H]
\centering
\begin{tikzpicture}
  % First Layer
  %% Nodes
  \node[draw, rectangle] (R_ivc_p) at (2.25, 6.5) {$R_{IVC}^{(p)}$};
  \node[draw, rectangle] (x_prev_p) at (3.5, 6.5) {$x_{i-1}^{(p)}$};
  \node[draw, rectangle] (pi_prev_p) at (4.75, 6.5) {$\pi_{i-1}^{(p)}$};

  \node[draw, rectangle] (q_p) at (6, 6.5) {$\vec{q}^{(p)}$};
  \node[draw, rectangle] (acc_prev_p) at (7.5, 6.5) {$\acc_{i-1}^{(p)}$};
  \node[draw, rectangle] (acc_next_p) at (9, 6.5) {$\acc_i^{(p)}$};

  \node[draw, rectangle] (R_ivc_q) at (10.75, 6.5) {$R_{IVC}^{(q)}$};
  \node[draw, rectangle] (x_prev_q) at (12, 6.5) {$x_{i-1}^{(q)}$};
  \node[draw, rectangle] (pi_prev_q) at (13.25, 6.5) {$\pi_{i-1}^{(q)}$};

  \node[draw, rectangle] (q_q) at (14.5, 6.5) {$\vec{q}^{(p)}$};
  \node[draw, rectangle] (acc_prev_q) at (16, 6.5) {$\acc_{i-1}^{(p)}$};
  \node[draw, rectangle] (acc_next_q) at (17.5, 6.5) {$\acc_i^{(p)}$};

  %% Arrows
  \draw[dashed-arrow] (R_ivc_p) -- (2.25, 7.1) -- (3.5, 7.1) -- (x_prev_p);
  \draw[dashed-arrow] (pi_prev_p) -- (4.75, 7.1) -- (6, 7.1) -- (q_p);
  \draw[dashed-arrow] (acc_prev_p) -- (7.5, 7.4) -- (3.5, 7.4) -- (x_prev_p);

  \draw[dashed-arrow] (R_ivc_q) -- (10.75, 7.1) -- (12, 7.1) -- (x_prev_q);
  \draw[dashed-arrow] (pi_prev_q) -- (13.25, 7.1) -- (14.5, 7.1) -- (q_q);
  \draw[dashed-arrow] (acc_prev_q) -- (16, 7.4) -- (12, 7.4) -- (x_prev_q);

  % Second Layer
  \node[draw, rectangle] (svf_p) at (3.5, 5) {$\NARKVerifierFast$};
  \node[draw, rectangle] (asv_p) at (7.5, 5) {$\ASVerifier$};

  \node[draw, rectangle] (svf_q) at (12, 5) {$\NARKVerifierFast$};
  \node[draw, rectangle] (asv_q) at (16, 5) {$\ASVerifier$};

  %% Arrows
  \draw[arrow] (R_ivc_p) -- (2.25, 6) -- (3.5, 5.75) -- (svf_p);
  \draw[arrow] (x_prev_p) -- (3.5, 6) -- (3.5, 5.75) -- (svf_p);
  \draw[arrow] (pi_prev_p) -- (4.75, 6) -- (3.5, 5.75) -- (svf_p);

  \draw[arrow] (q_p) -- (6, 6) -- (7.5, 5.75) -- (asv_p);
  \draw[arrow] (acc_prev_p) -- (7.5, 6) -- (7.5, 5.75) -- (asv_p);
  \draw[arrow] (acc_next_p) -- (9, 6) -- (7.5, 5.75) -- (asv_p);

  \draw[arrow] (R_ivc_q) -- (10.75, 6) -- (12, 5.75) -- (svf_q);
  \draw[arrow] (x_prev_q) -- (12, 6) -- (12, 5.75) -- (svf_q);
  \draw[arrow] (pi_prev_q) -- (13.25, 6) -- (12, 5.75) -- (svf_q);

  \draw[arrow] (q_q) -- (14.5, 6) -- (16, 5.75) -- (asv_q);
  \draw[arrow] (acc_prev_q) -- (16, 6) -- (16, 5.75) -- (asv_q);
  \draw[arrow] (acc_next_q) -- (17.5, 6) -- (16, 5.75) -- (asv_q);

  % Third Layer
  \node[draw, rectangle] (and) at (9.5, 3) {$\land$};

  %% Arrows
  \draw[arrow] (svf_p) -- (3.5, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (asv_p) -- (7.5, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (svf_q) -- (12, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (asv_q) -- (16, 4.5) -- (9.5, 3.75) -- (and);

  % Fourth Layer
  \node[draw, rectangle] (s_next) at (3.25, 3.5) {$s_i$};
  \node[draw, rectangle] (s_prev) at (4.5, 3.5) {$s_{i-1}$};
  \node[draw, rectangle] (s_0) at (5.75, 3.5) {$s_0$};

  % Fifth Layer
  \node[draw, rectangle] (zero) at (5.75, 2) {$s_{i-1} \meq s_0$};
  \node[draw, rectangle] (F) at (3.25, 2) {$F'(s_{i-1}, s_i)$};

  \draw[arrow] (s_next) -- (3.25, 3) -- (3.25, 2.75) -- (F);
  \draw[arrow] (s_prev) -- (4.25, 3) -- (3.25, 2.75) -- (F);

  \draw[arrow] (s_prev) -- (4.75, 3) -- (5.75, 2.75) -- (zero);
  \draw[arrow] (s_0) -- (5.75, 3) -- (5.75, 2.75) -- (zero);

  % Sixth Layer
  \node[draw, rectangle] (or1) at (8, 0.5) {$\lor$};
  
  \draw[arrow] (zero) -- (5.75, 1.5) -- (8, 1.25) -- (or1);
  \draw[arrow] (and) -- (9.5, 1.5) -- (8, 1.25) -- (or1);

  % Sixth Layer
  \node[draw, rectangle] (and2) at (5.75, -1) {$\land$};
  \draw[arrow] (or1) -- (8, 0) -- (5.75, -0.25) -- (and2);
  \draw[arrow] (F) -- (3.25, 0) -- (5.75, -0.25) -- (and2);

\end{tikzpicture}
\caption{A visualization of $R_{\text{IVC}}$.}
\end{figure}

For the purpose of creating the chain of signatures we can define:

$$
\begin{aligned}
  s_0 &= (\s_0, j_0 = 0, pk_0) \\
  s_i &= (\s_i, j_i, pk_i) \\
  F'(s_{i-1}, s_i) &= \textsc{Schnorr.Verify}_{pk_{i-1}}(\s_i, pk_i) \land j_i \meq j_{i-1} + 1\\
\end{aligned}
$$

The first signature, $s_0$, can be invalid, since it's never checked. The
$j_i \meq j_{i-1}$ is required for soundness, it means that each iteration
will terminate. The $s_{i-1} \meq s_0$ will thus also check whether we are
in the base-state with $j = 0$ and that $pk_0$ is the genesis public-key. 

The verifier and prover for the IVC scheme can be seen below:

\begin{algorithm}[H]
\caption*{\textbf{Algorithm} $\IVCProver$}
\textbf{Constants} \\
  \Desc{$R_{\text{IVC}} = \left( R_{\text{IVC}}^{(p)}, R_{\text{IVC}}^{(q)} \right)$}{The IVC circuit as defined above.} \\
  \Desc{$s_0 = \left( \s_0, 0, pk_0 \right)$}{The base IVC-state.} \\
\textbf{Inputs} \\
  \Desc{$s_{i-1} = \left( \s_{i-1}, j_{i-1}, pk_{i-1} \right)$}{The previous IVC-state.} \\
  \Desc{$\pi_{i-1} = \left( \pi_{i-1}^{(p)}, \pi_{i-1}^{(q)} \right)$}{The previous IVC-proof.} \\
  \Desc{$\acc_{i-1} = \left( \acc_{i-1}^{(p)}, \acc_{i-1}^{(q)} \right)$}{The previous IVC-accumulator.} \\
  \Desc{$s_i = \left( \s_i, j_i, pk_i \right)$}{The next IVC-state} \\
\textbf{Output} \\
  \Desc{$(S, \Proof, \Acc)$}{The values for the next IVC iteration.}
\begin{algorithmic}[1]
  \Require $F'(s_{i-1}, s_i) = \top$
  \Require $j_i = j_{i-1} + 1$
  \State Compute the next IVC-proof, $\pi_i$:
    \State \algind Define the witness for the IVC-circuit:
      \Statex \algind \algind $x_{i-1}^{(p)} := \lbrace R_{\text{IVC}}^{(p)}, s_0, s_{i-1}, acc_{i-1}^{(p)} \rbrace$
      \Statex \algind \algind $x_{i-1}^{(q)} := \lbrace R_{\text{IVC}}^{(q)}, acc_{i-1}^{(q)} \rbrace$
      \Statex \algind \algind $w_i^{(p)} := \lbrace x_{i-1}^{(p)}, \pi_{i-1}^{(p)}, \acc_{i-1}^{(p)}, s_{i-1} \rbrace$
      \Statex \algind \algind $w_i^{(q)} := \lbrace x_{i-1}^{(q)}, \pi_{i-1}^{(q)}, \acc_{i-1}^{(q)} \rbrace$
    \State \algind Define the public inputs for the IVC-circuit:
      \Statex \algind \algind $x_i^{(p)} := \lbrace R_{\text{IVC}}^{(p)}, s_0, s_i, acc_i^{(p)} \rbrace$
      \Statex \algind \algind $x_i^{(q)} := \lbrace R_{\text{IVC}}^{(q)}, acc_i^{(q)} \rbrace$
    \State \algind Compute the proofs:
      \Statex \algind \algind $\pi_i^{(p)} := \NARKProver \left( R_{\text{IVC}}^{(p)}, x_i^{(p)}, w_i^{(p)} \right)$
      \Statex \algind \algind $\pi_i^{(q)} := \NARKProver \left( R_{\text{IVC}}^{(q)}, x_i^{(q)}, w_i^{(q)} \right)$
      \Statex \algind \algind $\pi_i := \left( \pi_i^{(p)}, \pi_i^{(q)} \right)$
  \State Compute the next accumulator, $\acc_i$:
    \State \algind Parse $\vec{q}^{(p)}$ from $\pi_{i-1}^{(p)}$, and $\vec{q}^{(q)}$ from $\pi_{i-1}^{(q)}$.
    \State \algind Run the $\ASProver$.
    \Statex \algind \algind $\acc_i^{(p)} = \ASProver \left( \vec{q}^{(p)}, \acc_{i-1}^{(p)} \right)$
    \Statex \algind \algind $\acc_i^{(q)} = \ASProver \left( \vec{q}^{(q)}, \acc_{i-1}^{(q)} \right)$
    \Statex \algind \algind $\acc_i = \left( \acc_i^{(p)}, \acc_i^{(q)} \right)$
  \State Output $(s_i, \pi_i, \acc_i)$
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{\textbf{Algorithm} $\IVCVerifier$}
\textbf{Constants} \\
  \Desc{$R_{\text{IVC}} = \left( R_{\text{IVC}}^{(p)}, R_{\text{IVC}}^{(q)} \right)$}{The IVC circuit as defined above.} \\
  \Desc{$s_0 = \left( \s_0, 0, pk_0 \right)$}{The base IVC-state.} \\
\textbf{Inputs} \\
  \Desc{$s_i = \left( \s_i, j_i, pk_i \right)$}{The current IVC-state.} \\
  \Desc{$\pi_i = \left( \pi_i^{(p)}, \pi_i^{(q)} \right)$}{The current IVC-proof.} \\
  \Desc{$\acc_i = \left( \acc_i^{(p)}, \acc_i^{(q)} \right)$}{The current IVC-accumulator.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Returns $\top$ if the verifier accepts and $\bot$ if the verifier rejects.}
\begin{algorithmic}[1]
  \If{$s_i \meq s_0$} \Comment{If this is true, then the proofs will be invalid and unnecessary.}
    \State \Return $\top$.
  \EndIf
  \State Verify the accumulators using the accumulation scheme decider:
    \Statex \algind $\ASDecider \left( \acc_i^{(p)} \right) \meq \ASDecider \left( \acc_i^{(q)} \right) \meq \top$
  \State Verify the NARK-proofs:
    \Statex \algind $x_i^{(p)} := \lbrace R_{\text{IVC}}^{(p)}, s_0, s_i, acc_i^{(p)} \rbrace$
    \Statex \algind $x_i^{(q)} := \lbrace R_{\text{IVC}}^{(q)}, acc_i^{(q)} \rbrace$
    \Statex \algind $\NARKVerifier \left( R_{\text{IVC}}^{(p)}, x_i^{(p)}, \pi_i^{(p)} \right) \meq \NARKVerifier \left( R_{\text{IVC}}^{(q)}, x_i^{(q)}, \pi_i^{(q)} \right) \meq \top$
  \State If the above two checks pass, then output $\top$, else output $\bot$.
\end{algorithmic}
\end{algorithm}

