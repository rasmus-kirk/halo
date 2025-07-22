## Cycle of Curves

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

If we can operate our NARK over a cycle of curves, we can optimize elliptic
curve operations used in the circuit to great effect. We start by defining
the operations that we will need to support for our language:

$$
  \begin{matrix}
    \Fb_p: & a + b & a * b & a^{-1} & a \meq b & \text{if-then-else}(a, b, c) \\
    \Fb_q: & P + Q & aP    & -P     & P \meq Q & \text{if-then-else}(a: \Fb_q, P, Q) & H(\vec{P}, \vec{a}: \Fb^k_p)
  \end{matrix}
$$

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

**TODO:**

- Constraints
  - [x] $a^{-1}$
  - [x] $P + Q$
  - [x] $P = -Q$ should be $x_p \meq x_q \land y_p \meq -y_q$
  - [x] $P \meq Q$ should be $x_p \meq x_q \land y_p \meq y_q$
  - [x] ite
  - [ ] Poseidon
- Gadgets
  - [x] $aP$
  - [ ] $H(\dots)$

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
