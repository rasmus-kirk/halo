## Cycle of Curves

If we can operate our NARK over a cycle of curves, we can optimize elliptic
curve operations used in the circuit to great effect.

### Required Operations

Since our goal is to construct a minimal Halo2-style recursive proof scheme,
we start by defining the language that's needed to implement the relevant
verifiers ($\ASDLVerifier$, $\SurkalVerifier$):

$$
  \begin{matrix}
    \Fb_p: & a + b & a * b & a^{-1} & a \meq b & \text{if-then-else}(a, b, c) \\
    \Fb_q: & P + Q & aP    & -P     & P \meq Q & H(\vec{P}, \vec{a}: \Fb^k_p)
  \end{matrix}
$$

This leads to the following constraints:

- Point (non-id) ([link](https://zcash.github.io/halo2/design/gadgets/ecc/witnessing-points.html)):
  - $q_{point (non-id)} \cdot (y^2 - x^3 - 5) = 0$
- Point ([link](https://zcash.github.io/halo2/design/gadgets/ecc/witnessing-points.html)):
  - $(q_{point} \cdot x) \cdot (y^2 - x^3 - 5) = 0$
  - $(q_{point} \cdot y) \cdot (y^2 - x^3 - 5) = 0$
- Incomplete point addition ($P \incompleteadd Q = R$) ([link](https://zcash.github.io/halo2/design/gadgets/ecc/addition.html)):
  - $q_{add-incomplete} \cdot ((x_r + x_q + x_p) \cdot (x_p - x_q)^2 - (y_p - y_q)^2) = 0$, 
  - $q_{add-incomplete} \cdot ((y_r + y_q) \cdot (x_p - x_q) - (y_p - y_q) \cdot (x_q - x_r)$
- Complete point addition ([link](https://zcash.github.io/halo2/design/gadgets/ecc/addition.html)):
  - Define the following for the below constraints:
    $$
    \begin{aligned}
      \text{inv0}(x) &= \begin{cases} 
        0,   & \text{if } x = 0 \\
        1/x, & \text{otherwise} 
      \end{cases} \\
      \a &= \text{inv0}(x_q - x_p) \\
      \b &= \text{inv0}(x_p) \\
      \g &= \text{inv0}(x_q) \\
      \d &= \begin{cases} 
        \text{inv0}(y_q + y_p), & \text{if } x_q = x_p \\
        0, & \text{otherwise} 
      \end{cases} \\
      \l &= \begin{cases} 
        \frac{y_q - y_p}{x_q - x_p}, & \text{if } x_q \neq x_p \\
        \frac{3x_p^2}, & \text{if } x_q = x_p \land y_p \neq 0 \\
        \frac{3x_p^2}, & \text{otherwise}
      \end{cases} \\
    \end{aligned}
    $$
  - $q_{add} \cdot (x_q - x_p) \cdot ((x_q - x_p) \cdot \l - (y_q - y_p)) = 0$
  - $q_{add} \cdot (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = 0$
  - $q_{add} \cdot x_p \cdot x_q \cdot (x_q - x_r) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  - $q_{add} \cdot x_p \cdot x_q \cdot (x_q - x_r) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$
  - $q_{add} \cdot x_q \cdot (y_q + y_r) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  - $q_{add} \cdot x_q \cdot (y_q + y_r) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$
  - $q_{add} \cdot (1 - x_q \cdot \b) \cdot (x_r - x_q) = 0$
  - $q_{add} \cdot (1 - x_q \cdot \b) \cdot (y_r - y_q) = 0$
  - $q_{add} \cdot (1 - x_q \cdot \g) \cdot (x_r - x_p) = 0$
  - $q_{add} \cdot (1 - x_q \cdot \g) \cdot (y_r - y_p) = 0$
  - $q_{add} \cdot (1 - x_q \cdot x_p) \cdot \a \cdot (y_q + y_p) \cdot \d \cdot x_r = 0$
  - $q_{add} \cdot (1 - x_q \cdot x_p) \cdot \a \cdot (y_q + y_p) \cdot \d \cdot y_r = 0$
- $P = -Q$ (home baked):
  - $q_{point-neg} \cdot (x_p - x_q) = 0$
  - $q_{point-neg} \cdot (y_p + y_q) = 0$
- $P \meq Q$ (home baked):
  - $q_{point-eq} \cdot (x_p - x_q) = 0$
  - $q_{point-eq} \cdot (y_p - y_q) = 0$
- $a^{-1} = b$ (home baked):
  - $q_{inv} \cdot (a \cdot b) = 0$
- $c = \textbf{if } b \textbf{ then } x \textbf{ else } y \implies b \cdot x + (1-b) \cdot y$ (home baked):
  - $q_{ite} \cdot (c - (b \cdot x + (1-b) \cdot y))$
- $c = \textbf{if } b \textbf{ then } x \textbf{ else } y \implies b \cdot x + (1-b) \cdot y$ (home baked):
- Poseidon Constraints:
  - Round 1:
    - $w_6  - (r_0 + (M_{0,0} w_0^7 + M_{0,1} w_1^7 + M_{0,2} w_2^7))$
    - $w_7  - (r_1 + (M_{1,0} w_0^7 + M_{1,1} w_1^7 + M_{1,2} w_2^7))$
    - $w_8  - (r_2 + (M_{2,0} w_0^7 + M_{2,1} w_1^7 + M_{2,2} w_2^7))$
  - Round 2:
    - $w_9  - (r_3 + (M_{0,0} w_6^7 + M_{0,1} w_7^7 + M_{0,2} w_8^7))$
    - $w_{10} - (r_4 + (M_{1,0} w_6^7 + M_{1,1} w_7^7 + M_{1,2} w_8^7))$
    - $w_{11} - (r_5 + (M_{2,0} w_6^7 + M_{2,1} w_7^7 + M_{2,2} w_8^7))$
  - Round 3:
    - $w_{12} - (r_6 + (M_{0,0} w_9^7 + M_{0,1} w_{10}^7 + M_{0,2} w_{11}^7))$
    - $w_{13} - (r_7 + (M_{1,0} w_9^7 + M_{1,1} w_{10}^7 + M_{1,2} w_{11}^7))$
    - $w_{14} - (r_8 + (M_{2,0} w_9^7 + M_{2,1} w_{10}^7 + M_{2,2} w_{11}^7))$
  - Round 4:
    - $w_3 - (r_9 + (M_{0,0} w_{12}^7 + M_{0,1} w_{13}^7 + M_{0,2} w_{14}^7))$
    - $w_4 - (r_{10} + (M_{1,0} w_{12}^7 + M_{1,1} w_{13}^7 + M_{1,2} w_{14}^7))$
    - $w_5 - (r_{11} + (M_{2,0} w_{12}^7 + M_{2,1} w_{13}^7 + M_{2,2} w_{14}^7))$
  - Round 5:
    - $w_{0,\text{next}} - (r_{12} + (M_{0,0} w_3^7 + M_{0,1} w_4^7 + M_{0,2} w_5^7))$
    - $w_{1,\text{next}} - (r_{13} + (M_{1,0} w_3^7 + M_{1,1} w_4^7 + M_{1,2} w_5^7))$
    - $w_{2,\text{next}} - (r_{14} + (M_{2,0} w_3^7 + M_{2,1} w_4^7 + M_{2,2} w_5^7))$
  - The $w_{i,\text{next}}$ refers to the input of the next row so the polynomial $w(\o x)$
- Boolean range check:
  - $q_{bit} \cdot (x \cdot (x-1)) = 0$
- Range Check $a \in [0, 2^{\floor{p}}]$:
  - First decompose $a$ into 21 limbs of 12 bits (a_i^{(12)}) and 1 limb
    of 2 bits $a^{(2)}$. For each of the 12-bit limbs we can do a plookup,
    for the 2-bit limb:
  - $q_{p-range} (x \cdot (x - 1) \cdot (x - 2) \cdot (x - 3)) = 0$




And the following gadgets:

\begin{algorithm}[H]
\caption*{
  \textbf{Elliptic Curve Scalar Multiplication Gadget:} Performs scalar multiplication ($aP = Q$)
  (\href{https://zcash.github.io/halo2/design/gadgets/ecc/var-base-scalar-mul.html}{link}).
}
\textbf{Inputs} \\
  \Desc{$\vec{a}: \Fb^{255}$}{The bit-decomposition of scalar $a$.} \\
  \Desc{$P: \Eb(\Fb)$}{The point to scale.} \\
\textbf{Output} \\
  \Desc{$Q: \Eb(\Fb)$}{Q = aP}
\begin{algorithmic}[1]
  \State $Acc := 2 P$
  \For{$i = 253$ to $3$} \Comment{Incomplete addition}
    \State $P := a_{i+1} \; ? \; P \; : \; -P$
    \State $Acc = (Acc \incompleteadd P) \incompleteadd Acc$
  \EndFor
  \For{$i = 2$ to $0$} \Comment{Complete addition}
    \State $P := a_{i+1} \; ? \; P \; : \; -P$
    \State $Acc = (Acc + P) + Acc$
  \EndFor
  \If{$k_0 = 0$}
    \State \Return $Q := Acc + (-T)$
  \Else
    \State \Return $Q := Acc$
  \EndIf
\end{algorithmic}
\end{algorithm}

#### Poseidon

The Poseidon State can be one of the following values:

$$
  \textbf{SpongeState} = \begin{cases}
    \texttt{Absorbed}(0) \\
    \texttt{Absorbed}(1) \\
    \texttt{Absorbed}(2) \\
    \texttt{Squeezed}(1) \\
    \texttt{Squeezed}(2) \\
  \end{cases}
$$
The $\textbf{SpongeState}$ shouldn't be part of the circuit, it just governs
what when the full poseidon gates should be added to the circuit, i.e. when
enough values has been absorbed.

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Absorb Gadget:} Absorbs a list of field elements into the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{
    The current state condition of the sponge.
  } \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}$}{The field elements that the sponge should absorb.} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb^3)$}{
    The sponge state condition and inner state after absorption.
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
    \If{$\text{sponge\_state} = \texttt{Absorbed}(n) \land n < 2$}
      \State $\text{sponge\_state} = \texttt{Absorbed}(n + 1)$
      \State $s_n = x$
    \ElsIf{$\text{sponge\_state} = \texttt{Absorbed}(2)$}
      \For{$i \in [0, 10]$} \Comment{Permute 55 times by using the Hades Gate 11 times}
        \State $\vec{s} = PoseidonBlockCipher(i, c, \vec{s})$
      \EndFor
      \State $\text{sponge\_state} = \texttt{Absorbed}(1)$
      \State $s_0 = s_0 + x$
    \Else
      \State $\text{sponge\_state} = \texttt{Absorbed}(1)$
      \State $s_0 = s_0 + x$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Squeeze Gadget:} Squeezes a field element from the the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(c, s, x): (\textbf{SpongeState}, \Fb^3, \Fb)$}{
    The sponge state condition and inner state after absorption and the squeezed element
  }
\begin{algorithmic}[1]
  \If{$\text{sponge\_state} = \texttt{Squeezed}(n) \land n < 2$}
    \State $\text{sponge\_state} = \texttt{Squeezed}(n + 1)$
    \State \textbf{Return} $x = s_n$
  \Else
    \For{$i \in [0, 10]$} \Comment{Permute 55 times by using the Hades Gate 11 times}
      \State $\vec{s} = \text{HadesGate}_i(c, \vec{s})$
    \EndFor
    \State $\text{sponge\_state} = \texttt{Squeezed}(1)$
    \State \textbf{Return} $x = s_0$
  \EndIf
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Affine Gadget:} Absorbs affine points into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{Ps}$}{The affine points to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb^3, \Fb)$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$P$ in $\vec{Ps}$}
    \If{$P \meq \Oc$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, 0)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, 0)$
    \Else
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, P.x)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, P.y)$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Field Element Gadget:} Absorbs field elements into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}: \Fb_{BF}$}{The field elements to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{BF})$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, x)$
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Scalar Gadget:} Absorbs scalars into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb_{BF}^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs} \in (\Fb_{S})$}{The scalars to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{BF})$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
    \If{$|\text{Scalar-Field}| < |\text{Base-Field}|$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, x)$
    \Else
      \State Decompose $x$ into $h, l$ where $h$ represents the high-bits of $x$ and $l$ represents the low-bit.
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, h)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, l)$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Squeeze Scalar Gadget:} Squeezes a scalar from the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb_{BF}^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(c, s, x): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{S})$}{
    The sponge state condition and inner state after squeezing and the squeezed scalar. 
  }
\begin{algorithmic}[1]
    \If{$x < |\text{Base-Field}|$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{x}, x)$
    \Else
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, 0)$
    \EndIf
\end{algorithmic}
\end{algorithm}


This gadget returns zero for values that are too large. This means that
there is a bias for the value zero (in one of the curves). An attacker
could try to target that seed, in order to predict the challenges produced
by the weaker sponge. This would allow the attacker to mess with the result
of the proofs. Previously the attacker's odds were $\frac{1}{q}$, now it's
$\frac{q-p}{q}$. Since $lg(q-p) \approx 86$ and $lg(q) \approx 254$ the odds
of a successful attack are still negligible, but we do lose some security.

$$\frac{q - p}{q} \approx \frac{2^{86}}{2^{254}} = 2^{86 - 254} = 2^{-168}$$

### Input Passing

The above section describes how each language instruction is mapped to
one of two circuits, verifying both circuits should convince the verifier
that the program $f(w, x)$ is satisfied. However, for the Elliptic Curve
Multiplication and the Poseidon Hashes, we need to pass inputs from one circuit
to another. We have a circuit over $\Fb_p$, $C(\Fb_p)$, and a circuit over
$\Fb_q$, $C(\Fb_q)$, with $p > q$. We wish to pass a message from $C(\Fb_q)$
to $C(\Fb_p)$ and want to convince the verifier that $v_q = v_p$. Naively,
if these values are added as public inputs, the verifier could add the check
that $v_q \meq v_p$, but once we get to IVC, this becomes unfeasible, as the
set of values to check will grow as the IVC chain grows, which in worst-case
makes the proof size $\Oc(n)$.

Since the values of $v_q$ is committed to in the public input, the verifier
has the commitment $C^{(q)}_{PI}$ and likewise for $p$, the verifier knows
$v_p, C^{(q)}_{PI}$. So we pass $v_q$ to $C(\Fb_p)$ as $v_p$ and then verify
add the following constraint to $C(\Fb_p)$:

$$C^{(p)}_{PI} \meq v_p \cdot G_1 \in \Eb_p(\Fb_q)$$

This proves that I know the openings of the commitment $C^{(p)}_{PI}$,
and since I know that this opening correspond to the public inputs, by the
binding property of the commitment scheme, I know the public inputs of the
other circuit.

Now, what if we reverse the flow? I now have a value $v_p$, in $C(\Fb_p)$,
that I want to pass to $C(\Fb_q)$. Here the problem is that since $p > q$,
the value might be too large to represent in $\Fb_q$-field. The solution is
to decompose the value as such:

$$v_p = 2 h + l$$

Where $h$ represents the high-bits of $v_p$ ($h \in [0, 2^{\floor{\log{p}}}]$)
and $l$ represents the low-bit ($h \in \Bb$). The value $v_p$ can now be
represented with $h, l$, both of which are less than $q$. Which means we
can pass the value to $C(\Fb_q)$.

The final constraints on each side then becomes:

- $\Fb_q$:
  - $C^{(p)}_{PI} \meq h G_1 + l G_2$
  - $C^{(p)}_{PI} \meq h G_1 + l G_2$
- $\Fb_p$:
  - $v_p = 2 h + l$
  - $h \in [0, 2^{\floor{\log{p}}}]$ (range check)
  - $l \in \Bb$ (simple boolean constraint)

### The New IVC-Scheme

Recall the graph from the prerequisites section about IVC based on accumulation
schemes:

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

We now operate over two curves, with two accumulators, proofs and states:

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

  % Second chain (bottom row)
  \node (t0) [node, below=1.5cm of s0] {$s_0$};
  \node (t1) [node, right=of t0] {$(s_1, \pi_1, \acc_1)$};
  \node (dots2) [right=2.75cm of t1] {$\dots$};
  \node (tn) [node, right=4cm of dots2] {$(s_n, \pi_n, \acc_n)$};
  % Arrows for second chain
  \draw[thick-arrow] (t0) -- node[above] {$\Pc(t_0, \bot, \bot)$} (t1);
  \draw[thick-arrow] (t1) -- node[above] {$\Pc(t_1, \pi_1, \acc_1)$} (dots2);
  \draw[thick-arrow] (dots2) -- node[above] {$\Pc(t_{n-1}, \pi_{n-1}, \acc_{n-1})$} (tn);
\end{tikzpicture}
\caption{
  lol.
}
\end{figure}
