## Cycle of Curves

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
- Curve 2: $a \in \Fb_{p_2}, P \in \Eb_q(\Fb_{p_3})$
- Curve 3: $a \in \Fb_{p_3}, P \in \Eb_q(\Fb_{p_4})$
- ...

Which means that each $p_i$ must be able to define a valid curve, and if
this never cycles, we would need to support this infinite chain of curves. 

### Input Passing

<!-- The above section describes how each language instruction is mapped to -->
<!-- one of two circuits, verifying both circuits should convince the verifier -->
<!-- that the program $f(w, x)$ is satisfied. However, for the Elliptic Curve -->
<!-- Multiplication and the Poseidon Hashes, we need to pass inputs from one circuit -->
<!-- to another. We have a circuit over $\Fb_p$, $C(\Fb_p)$, and a circuit over -->
<!-- $\Fb_q$, $C(\Fb_q)$, with $p > q$. We wish to pass a message from $C(\Fb_q)$ -->
<!-- to $C(\Fb_p)$ and want to convince the verifier that $v_q = v_p$. Naively, -->
<!-- if these values are added as public inputs, the verifier could add the check -->
<!-- that $v_q \meq v_p$, but once we get to IVC, this becomes unfeasible, as the -->
<!-- set of values to check will grow as the IVC chain grows, which in worst-case -->
<!-- makes the proof size $\Oc(n)$. -->

<!-- Since the values of $v_q$ is committed to in the public input, the verifier -->
<!-- has the commitment $C^{(q)}_{PI}$ and likewise for $p$, the verifier knows -->
<!-- $v_p, C^{(q)}_{PI}$. So we pass $v_q$ to $C(\Fb_p)$ as $v_p$ and then verify -->
<!-- add the following constraint to $C(\Fb_p)$: -->

<!-- $$C^{(p)}_{PI} \meq v_p \cdot G_1 \in \Eb_p(\Fb_q)$$ -->

<!-- This proves that I know the openings of the commitment $C^{(p)}_{PI}$, -->
<!-- and since I know that this opening correspond to the public inputs, by the -->
<!-- binding property of the commitment scheme, I know the public inputs of the -->
<!-- other circuit. -->

<!-- Now, what if we reverse the flow? I now have a value $v_p$, in $C(\Fb_p)$, -->
<!-- that I want to pass to $C(\Fb_q)$. Here the problem is that since $p > q$, -->
<!-- the value might be too large to represent in $\Fb_q$-field. The solution is -->
<!-- to decompose the value as such: -->

<!-- $$v_p = 2 h + l$$ -->

<!-- Where $h$ represents the high-bits of $v_p$ ($h \in [0, 2^{\floor{\log{p}}}]$) -->
<!-- and $l$ represents the low-bit ($h \in \Bb$). The value $v_p$ can now be -->
<!-- represented with $h, l$, both of which are less than $q$. Which means we -->
<!-- can pass the value to $C(\Fb_q)$. -->

<!-- The final constraints on each side then becomes: -->

<!-- - $\Fb_q$: -->
  <!-- - $C^{(p)}_{PI} \meq h G_1 + l G_2$ -->
  <!-- - $C^{(p)}_{PI} \meq h G_1 + l G_2$ -->
<!-- - $\Fb_p$: -->
  <!-- - $v_p = 2 h + l$ -->
  <!-- - $h \in [0, 2^{\floor{\log{p}}}]$ (range check) -->
  <!-- - $l \in \Bb$ (simple boolean constraint) -->

### The New IVC-Scheme

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
function $F$.

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
  \node[draw, rectangle] (F) at (3.25, 2) {$F(s_{i-1}) \meq s_i$};

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

In our case the state contains

<!-- Before describing the IVC protocol, we first describe the circuit for the -->
<!-- IVC relation as it's more complex than for the naive SNARK-based approach. Let: -->

<!-- - $\pi_{i-1} = \vec{q}, \acc_{i-1}, s_{i-1}$ from the previous iteration. -->
<!-- - $s_i = F(s_{i-1})$ -->
<!-- - $\acc_i = \ASProver(\vec{q}, \acc_{i-1})$ -->

<!-- Giving us the public inputs $x = \{ R_{IVC}, s_0, s_i, \acc_i \}$ and witness -->
<!-- $w = \{ s_{i-1}, \pi_{i-1} = \vec{q}, \acc_{i-1} \}$, which will be used to -->
<!-- construct the the IVC circuit $R_{IVC}$: -->
<!-- $$ -->
<!-- \begin{aligned} -->
  <!-- x_{i-1} &:= \{ R_{IVC}, s_{i-1}, \acc_{i-1} \} \\ -->
  <!-- \Vc_1   &:= \NARKVerifierFast(R_{IVC}, x_{i-1}, \pi_{i-1}) \meq \top \\ -->
  <!-- \Vc_2   &:= \ASVerifier(\pi_{i-1} = \vec{q}, \acc_{i-1}, \acc_i) \meq \top \\ -->
  <!-- R_{IVC} &:= \text{I.K } w \text{ s.t. } F(s_{i-1}) \meq s_i \land (s_{i-1} \meq s_0 \lor ( \Vc_1 \land \Vc_2 ) ) \\ -->
<!-- \end{aligned} -->
<!-- $$ -->


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


