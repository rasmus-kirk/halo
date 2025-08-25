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
- Curve 2: $a \in \Fb_{p_2}, P \in \Eb_{p_2}(\Fb_{p_3})$
- Curve 3: $a \in \Fb_{p_3}, P \in \Eb_{p_3}(\Fb_{p_4})$
- ...

Which means that each $p_i$ must be able to define a valid curve, and if
this never cycles, we would need to support this infinite chain of curves. 

### Input Passing

The above section describes how each language instruction is mapped to
one of two circuits, verifying both circuits should convince the verifier
that the program $f(w, x)$ is satisfied. However, for the Elliptic Curve
Multiplication and the Poseidon Hashes, we need to pass inputs from one
circuit to another.

**Passing $v^{(q)} \to v^{(p)}$:**

We start with the simpler case. We have a circuit over $\Fb_p$, $R^{(p)}$,
and a circuit over $\Fb_q$, $R^{(q)}$, with $p > q$. We wish to pass
a value, $v^{(q)} \in \Fb_q$, from $R^{(q)}$ to $R^{(p)}$ and wish to
convince the verifier that $v^{(q)} = v^{(q)}$. Naively, if these values
are added as public inputs, the verifier could add the check that $v^{(q)}
\meq v^{(p)}$. But this won't work for IVC, since we can't check equality
across circuits, in-circuit. Instead we compute the commitment to $v^{(p)}$
on the $R^{(p)}$-side.

$$C^{(q)}_{\text{IP}} := v^{(q)} \cdot G_1^{(q)} \in \Eb_p(\Fb_q)$$

The scalar operation may seem invalid, but since we know that $v^{(q)}
\leq q - 1 < p - 1$, it can logically be computed by the usual double and
add, since the bits of $v^{(q)}$ will correspond to the bits of $v^{(p)}$
if $\text{lift}(v^{(q)}) = \text{lift}(v^{(p)})$. If $C^{(q)}$ is emitted in
the public inputs of the circuit, then the verifier will know that $C^{(q)}$
is a commitment to $v^{(q)}$. To convince the verifier of the desired relation
that $\text{lift}(v^{(q)}) = \text{lift}(v^{(p)})$, it will now suffice to
convince them that $v^{(p)}$ is a valid opening of $C^{(q)}$. So the verifier
checks manually that:

$$C^{(q)} \meq v^{(p)} \cdot G_1^{(q)}$$

Which, given that the rest of the proof verifies correctly, will then imply
that $v^{(q)} = v^{(p)}$. If the verifier is encoded as a circuit, then
we need to input pass when performing this additional check, since scalar
multiplication itself requires input passing to work. However this is no
problem, since that circuit-verifier will be verified by another verifier!

**Passing $v^{(p)} \to v^{(q)}$:**

What if we reverse the flow? We now have a value $v^{(p)}$, in $R^{(p)}$,
that we want to pass to $R^{(q)}$. Here the problem is that since $p > q$,
the value might be too large to represent in the $\Fb_q$-field. The solution
is to decompose the value:

$$v_p = 2 h + l$$

Where $h$ represents the high-bits of $v_p$ ($h \in [0, 2^{\floor{\log{p}}}]$)
and $l$ represents the low-bit ($h \in \Bb$). The value $v_p$ can now be
represented with $h, l$, both of which are less than $q$. Which means we
can pass the value to $R^{(q)}$.

The constraints added to $R^{(p)}$ then becomes:

- $C^{(p)}_{\text{PI}} \meq h \cdot G_1 + l \cdot G_2$
- $v = 2 h + l$
- $h \in [0, 2^{\floor{\log{p}}}]$ (range check)
- $l \in \Bb$ (simple boolean constraint)

We of course don't need to commit each time we input pass, we can create a
standard vector pedersen commit, containing all the passed values:

$$C^{(p)}_{\text{PI}} = h_{v_1}^{(p)} \cdot G_1^{(p)} + l_{v_1}^{(p)} \cdot G_2^{(p)} + h_{v_2}^{(p)} \cdot G_3^{(p)} + l_{v_2}^{(p)} \cdot G_4^{(p)} + \dots$$

Now, the $R_q$-verifier and $R_p$-verifier, would each also take in a
single input pass vector, in addition to the standard public input vector:
$$\text{InputPass}^{(q \to p)} \in \Fb_p^k, \qquad \text{InputPass}^{(p \to q)} \in \Fb_q^k$$

Each passed input is of course public, so the public input vector is then
defined as:

$$\text{PublicInputs}^{(p)}_{\text{new}} := \text{PublicInputs}^{(p)} \cat \text{InputPass}^{(p)}$$

For both the verifier and prover of course. Each of the $R^{(p)}$ and $R^{(q)}$
verifier can then use $\text{InputPass}^{(q \to p)}, \text{InputPass}^{(p \to q)}$
to verify $C^{(p)}, C^{(q)}$:

\begin{tcolorbox}[colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]

Take the following example circuit:

\begin{algorithm}[H]
\caption*{\textbf{Example Circuit}}
\textbf{Inputs} \\
  \Desc{$x, y \in \Fb_p$}{ } \\
  \Desc{$P \in \Eb_p(\Fb_q)$}{ }
\begin{algorithmic}[1]
  \State $z := x + y \in \Fb_p$
  \State $Q_1 := z \cdot P \in \Eb_p(\Fb_q)$
  \State $Q_2 := x \cdot P \in \Eb_p(\Fb_q)$
  \State $\a := \Hc(Q_1, Q_2) \in \Fb_p$
\end{algorithmic}
\end{algorithm}

Which means that we pass $z, x$ from $R^{(p)}$ to $R^{(q)}$ and $\a$ from
$R^{(q)}$ to $R^{(p)}$. Thus, we need to split $z, x$ but not $\a$. We add the
constraints:

\begin{itemize}
  \item $R^{(p)}$:
  \begin{itemize}
    \item $C^{(p)}_{\text{IP}} := h_z^{(p)} \cdot G_1^{(p)} + l_z^{(p)} \cdot G_2^{(p)} + h_x^{(p)} \cdot G_3^{(p)} + l_x^{(p)} \cdot G_4^{(p)}$
    \item $z := 2 \cdot h_z^{(p)} + l_z^{(p)}$ (Decomposition correctness check)
    \item $h_z^{(p)} \in [0, 2^{\floor{\log{p}}}]$ (Range check)
    \item $l_z^{(p)} \in \Bb$ (Boolean check)
  \end{itemize}
  \item $R^{(q)}$:
  \begin{itemize}
    \item $C^{(q)}_{\text{IP}} := \a^{(q)} \cdot G_1^{(q)}$
  \end{itemize}
\end{itemize}

\

Now the $R_q$-verifier and $R_p$-verifier, would each also take in the input
pass vectors:

$$
\begin{aligned}
  \text{InputPass}^{(p \to q)} &= [ h_z^{(q)}, l_z^{(q)}, h_x^{(q)}, l_x^{(q)} ] \\
  \text{InputPass}^{(q \to p)} &= [ \a^{(p)} ] \\
\end{aligned}
$$

Each passed input is of course public, so the public input vector is
then defined as:

$$\text{PublicInputs}^{(p)}_{\text{new}} := \text{PublicInputs}^{(p)} \cat \text{InputPass}^{(p)}$$

For both the verifier and prover of course. Now the verifier needs to verify
what it otherwise would, but also that:
$$
\begin{aligned}
  C^{(p)}_{\text{IP}} &\meq h_z^{(q)} \cdot G_1^{(p)} + l_z^{(q)} \cdot G_2^{(p)} + h_x^{(q)} \cdot G_3^{(p)} + l_x^{(q)} \cdot G_4^{(p)} \\
  C^{(q)}_{\text{IP}} &\meq \a^{(p)} \cdot G_1^{(q)} \\
\end{aligned}
$$

Note, that when recursing, these extra checks require input passing themselves,
but this is not an issue as that's handled by the next verifier.

\end{tcolorbox}

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
