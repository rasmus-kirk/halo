## High-Level Arithmetization

In the implementation circuits are written in a custom-made eDSL - an embedded
Domain Specific Language - that lets a circuit-creator specify circuits as
regular rust code. The circuit is modelled as a DAG - a Directed Acyclic
Graph - with the following kinds of nodes:

\begin{figure}[H]
\centering
\begin{tikzpicture}
  % First Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node (scalars) at (-3, 0) {\textbf{Scalars:}};
  \node[draw, rectangle, minimum size=15pt] (WS) at (0, 0) {W-$\Sc$};
  \node[draw, rectangle, minimum size=15pt] (PS) at (1.5, 0) {P-$\Sc$};
  \node[draw, rectangle, minimum size=15pt] (plus) at (3, 0) {$+$};
  \node[draw, rectangle, minimum size=15pt] (minus) at (4.5, 0) {$-$};
  \node[draw, rectangle, minimum size=15pt] (mul) at (6, 0) {$\times$};
  \node[draw, rectangle, minimum size=15pt] (inv) at (7.5, 0) {$x^{-1}$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (WS-out1) at (0, -0.6) {\scriptsize $\Sc$};
  \draw (WS-out1) -- (WS);

  \node[minimum size=2pt, inner sep=1pt] (PS-out1) at (1.5, -0.6) {\scriptsize $\Sc$};
  \draw (PS-out1) -- (PS);

  \node[minimum size=2pt, inner sep=1pt] (plus-in1) at (3.2, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (plus-in2) at (2.8, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (plus-out1) at (3, -0.6) {\scriptsize $\Sc$};
  \draw (plus-in1) -- (plus);
  \draw (plus-in2) -- (plus);
  \draw (plus-out1) -- (plus);

  \node[minimum size=2pt, inner sep=1pt] (minus-in1) at (4.7, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (minus-in2) at (4.3, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (minus-out1) at (4.5, -0.6) {\scriptsize $\Sc$};
  \draw (minus-in1) -- (minus);
  \draw (minus-in2) -- (minus);
  \draw (minus-out1) -- (minus);

  \node[minimum size=2pt, inner sep=1pt] (mul-in1) at (6.2, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (mul-in2) at (5.8, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (mul-out1) at (6, -0.6) {\scriptsize $\Sc$};
  \draw (mul-in1) -- (mul);
  \draw (mul-in2) -- (mul);
  \draw (mul-out1) -- (mul);

  \node[minimum size=2pt, inner sep=1pt] (inv-in1) at (7.5, 0.6) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (inv-out1) at (7.5, -0.6) {\scriptsize $\Sc$};
  \draw (inv-in1) -- (inv);
  \draw (inv-out1) -- (inv);

  % Second Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node (booleans) at (-3, -2) {\textbf{Booleans:}};
  \node[draw, rectangle, minimum size=15pt] (WB) at (0, -2) {W-$\Bb$};
  \node[draw, rectangle, minimum size=15pt] (PB) at (1.5, -2) {P-$\Bb$};
  \node[draw, rectangle, minimum size=15pt] (eq) at (3, -2) {$=$};
  \node[draw, rectangle, minimum size=15pt] (and) at (4.5, -2) {$\land$};
  \node[draw, rectangle, minimum size=15pt] (or) at (6, -2) {$\lor$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (WB-out1) at (0, -2.6) {\scriptsize $\Bb$};
  \draw (WB-out1) -- (WB);

  \node[minimum size=2pt, inner sep=1pt] (PB-out1) at (1.5, -2.6) {\scriptsize $\Bb$};
  \draw (PB-out1) -- (PB);

  \node[minimum size=2pt, inner sep=1pt] (eq-in1) at (3.2, -1.4) {\scriptsize $\Fb$};
  \node[minimum size=2pt, inner sep=1pt] (eq-in2) at (2.8, -1.4) {\scriptsize $\Fb$};
  \node[minimum size=2pt, inner sep=1pt] (eq-out1) at (3, -2.6) {\scriptsize $\Bb$};
  \draw (eq-in1) -- (eq);
  \draw (eq-in2) -- (eq);
  \draw (eq-out1) -- (eq);

  \node[minimum size=2pt, inner sep=1pt] (and-in1) at (4.7, -1.4) {\scriptsize $\Bb$};
  \node[minimum size=2pt, inner sep=1pt] (and-in2) at (4.3, -1.4) {\scriptsize $\Bb$};
  \node[minimum size=2pt, inner sep=1pt] (and-out1) at (4.5, -2.6) {\scriptsize $\Bb$};
  \draw (and-in1) -- (and);
  \draw (and-in2) -- (and);
  \draw (and-out1) -- (and);

  \node[minimum size=2pt, inner sep=1pt] (or-in1) at (6.2, -1.4) {\scriptsize $\Bb$};
  \node[minimum size=2pt, inner sep=1pt] (or-in2) at (5.8, -1.4) {\scriptsize $\Bb$};
  \node[minimum size=2pt, inner sep=1pt] (or-out1) at (6, -2.6) {\scriptsize $\Bb$};
  \draw (or-in1) -- (or);
  \draw (or-in2) -- (or);
  \draw (or-out1) -- (or);

  % Third Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node (elliptic-curves) at (-3, -4) {\textbf{Elliptic Curves:}};
  \node[draw, rectangle, minimum size=15pt] (WP) at (0, -4) {W-$P$};
  \node[draw, rectangle, minimum size=15pt] (PP) at (1.5, -4) {P-$P$};
  \node[draw, rectangle, minimum size=15pt] (EC-add) at (3, -4) {$+_{\text{EC}}$};
  \node[draw, rectangle, minimum size=15pt] (EC-mul) at (4.5, -4) {$\times_{\text{EC}}$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (WP-out1) at (-0.2, -4.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (WP-out2) at (0.2, -4.6) {\scriptsize $\Bc$};
  \draw (WP-out1) -- (WP);
  \draw (WP-out2) -- (WP);

  \node[minimum size=2pt, inner sep=1pt] (PP-out1) at (1.3, -4.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (PP-out2) at (1.7, -4.6) {\scriptsize $\Bc$};
  \draw (PP-out1) -- (PP);
  \draw (PP-out2) -- (PP);

  \node[minimum size=2pt, inner sep=1pt] (EC-add-in1) at (2.5, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-add-in2) at (2.85, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-add-in3) at (3.15, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-add-in4) at (3.5, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-add-out1) at (2.8, -4.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-add-out2) at (3.2, -4.6) {\scriptsize $\Bc$};
  \draw (EC-add-in1) -- (EC-add);
  \draw (EC-add-in2) -- (EC-add);
  \draw (EC-add-in3) -- (EC-add);
  \draw (EC-add-in4) -- (EC-add);
  \draw (EC-add-out1) -- (EC-add);
  \draw (EC-add-out2) -- (EC-add);

  \node[minimum size=2pt, inner sep=1pt] (EC-mul-in1) at (4.2, -3.4) {\scriptsize $\Sc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-mul-in2) at (4.5, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-mul-in3) at (4.8, -3.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-mul-out1) at (4.7, -4.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (EC-mul-out2) at (4.3, -4.6) {\scriptsize $\Bc$};
  \draw (EC-mul-in1) -- (EC-mul);
  \draw (EC-mul-in2) -- (EC-mul);
  \draw (EC-mul-in3) -- (EC-mul);
  \draw (EC-mul-out1) -- (EC-mul);
  \draw (EC-mul-out2) -- (EC-mul);

  % Fourth Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node (misc) at (-3, -6) {\textbf{Misc:}};
  \node[draw, rectangle, minimum size=15pt] (hash) at (0, -6) {$\Hc$};
  \node[draw, rectangle, minimum size=15pt] (mp-pq) at (1.5, -6) {$p \to q$};
  \node[draw, rectangle, minimum size=15pt] (mp-qp) at (3, -6) {$q \to p$};
  \node[draw, rectangle, minimum size=15pt] (assert-eq) at (4.5, -6) {$=_{\text{Assert}}$};
  \node[draw, rectangle, minimum size=15pt] (C) at (6, -6) {$\text{C}_x$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (hash-in1) at (-0.3, -5.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (hash-in2) at (0, -5.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (hash-in3) at (0.3, -5.4) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (hash-out1) at (-0.3, -6.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (hash-out2) at (0, -6.6) {\scriptsize $\Bc$};
  \node[minimum size=2pt, inner sep=1pt] (hash-out3) at (0.3, -6.6) {\scriptsize $\Bc$};
  \draw (hash-in1) -- (hash);
  \draw (hash-in2) -- (hash);
  \draw (hash-in3) -- (hash);
  \draw (hash-out1) -- (hash);
  \draw (hash-out2) -- (hash);
  \draw (hash-out3) -- (hash);

  \node[minimum size=2pt, inner sep=1pt] (mp-pq-in1) at (1.5, -5.4) {\scriptsize $\Fb_p$};
  \node[minimum size=2pt, inner sep=1pt] (mp-pq-out1) at (1.3, -6.6) {\scriptsize $\Fb_q$};
  \node[minimum size=2pt, inner sep=1pt] (mp-pq-out2) at (1.7, -6.6) {\scriptsize $\Fb_q$};
  \draw (mp-pq-in1) -- (mp-pq);
  \draw (mp-pq-out1) -- (mp-pq);
  \draw (mp-pq-out2) -- (mp-pq);

  \node[minimum size=2pt, inner sep=1pt] (mp-qp-in1) at (3, -5.4) {\scriptsize $\Fb_q$};
  \node[minimum size=2pt, inner sep=1pt] (mp-qp-out1) at (3, -6.6) {\scriptsize $\Fb_p$};
  \draw (mp-qp-in1) -- (mp-qp);
  \draw (mp-qp-out1) -- (mp-qp);

  \node[minimum size=2pt, inner sep=1pt] (assert-eq-in1) at (4.3, -5.4) {\scriptsize $\Fb$};
  \node[minimum size=2pt, inner sep=1pt] (assert-eq-in2) at (4.7, -5.4) {\scriptsize $\Fb$};
  \draw (assert-eq-in1) -- (assert-eq);
  \draw (assert-eq-in2) -- (assert-eq);

  \node[minimum size=2pt, inner sep=1pt] (C-out1) at (6, -6.6) {\scriptsize $\Fb$};
  \draw (C-out1) -- (C);
\end{tikzpicture}
\caption{The gates available as DAG nodes.}
\end{figure}

Wires can have two types $\Fb_p$ or $\Fb_q$. The other symbols denote:

- $\Bb$: Either $\Fb_p$ or $\Fb_q$, but whatever the concrete field element, it is constrained to be either 0, or 1, a bit.
- $\Sc$: A scalar-field element, either $\Fb_p$ or $\Fb_q$, depending on if the whether you model the Pallas ($\Sc = \Fb_p$) or Vesta ($\Sc = \Fb_q$) curve.
- $\Bc$: A base-field element, either $\Fb_p$ or $\Fb_q$, depending on if the whether you model the Pallas ($\Bc = \Fb_q$) or Vesta ($\Bc = \Fb_p$) curve.
- $\Fb$: The gate works when instantiated with either $\Fb_p$ or $\Fb_q$.

For both $\Fb$ and $\Bb$ it is invalid to provide $\Bb = \Fb_p$ as one of
the inputs and $\Bb = \Fb_q$ as the other.

- **Scalars:**
  - $\text{W-}\Sc$: Witness scalar.
  - $\text{P-}\Sc$: Public input scalar.
  - $(+)$: Add two scalars.
  - $(-)$: Subtract two scalars.
  - $(\times)$: Multiply two scalars.
  - $(x^{-1})$: Compute the inverse of $x$.
- **Booleans:**
  - $\text{W-}\Bb$: Witness boolean, of either $\Fb_p$ or $\Fb_q$.
  - $\text{P-}\Bb$: Public input boolean, of either $\Fb_p$ or $\Fb_q$.
  - $(=)$: Equality gate, taking two inputs of the same type, of either
    $\Fb_p$ or $\Fb_q$. If two $\Fb_p$ elements are inputted, the resulting
    boolean element will be an $\Fb_p$ element constrained to be either 0
    or 1, with the converse being true if both inputs are $\Fb_q$ elements.
  - $(\land)$: And gate, taking two boolean-constrained inputs of the same
    type, of either $\Fb_p$ or $\Fb_q$. As with the equality gate, the output
    has the same type as the input.
  - $(\lor)$: Or gate, taking two boolean-constrained inputs of the same
    type, of either $\Fb_p$ or $\Fb_q$. As with the equality gate, the output
    has the same type as the input.
- **Elliptic Curves:**
  - $\text{W-}P$: Witness curve point, in affine form, constrained to fit
    the curve equation.
  - $\text{P-}P$: Public input curve point, in affine form, constrained to
    fit the curve equation.
  - $(+_{\text{EC}})$: Add two elliptic curve points.
  - $(\times_{\text{EC}})$: Scale a point with a scalar, it is implicit that
    the scalar is input passed from the scalar-field to the base-field.
- **Miscelanious:**
  - $\Hc$: Performs five rounds of the Poseidon hashing on the three given
    base-field elements, representing the Poseidon sponge state.
  - $p \to q$: Message passes an $\Fb_p$ element to the $\Fb_q$ circuit.
  - $q \to p$: Message passes an $\Fb_q$ element to the $\Fb_p$ circuit.
  - $(=_{\text{Assert}})$: Asserts that the two field elements of the same type are equal.
  - $\text{C}_x$: A constant gate, outputting a fixed value $x$ of either $\Fb_p$ or $\Fb_q$.

We can represent our previous example circuit from Figure
\ref{fig:example-circuit} using these nodes:

\begin{figure}[H]
\centering
\begin{tikzpicture}
  % First Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (C-3) at (0, 0) {$\text{C}_3$};
  \node[draw, rectangle, minimum size=15pt] (WI-x1) at (2, 0) {W-$\Sc_{x_1}$};
  \node[draw, rectangle, minimum size=15pt] (WI-x2) at (5, 0) {W-$\Sc_{x_2}$};
  \node[draw, rectangle, minimum size=15pt] (C-5) at (7, 0) {$\text{C}_5$};

  % Second Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (times-21) at (2, -1.5) {$\times$};
  \node[draw, rectangle, minimum size=15pt] (times-22) at (6, -1.5) {$\times$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (1, -0.75) {\scriptsize $\Sc$};
  \draw[arrow] (WI-x1) -- (1, 0) -- (mid) -- (1, -1.5) -- (times-21);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (3, -0.75) {\scriptsize $\Sc$};
  \draw[arrow] (WI-x1) -- (3, 0) -- (mid) -- (3, -1.5) -- (times-21);

  \node[minimum size=2pt, inner sep=1pt] (mid) at (5, -0.75) {\scriptsize $\Sc$};
  \draw[arrow] (WI-x2) -- (mid) -- (5, -1.5) -- (times-22);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (7, -0.75) {\scriptsize $\Sc$};
  \draw[arrow] (C-5) -- (mid) -- (7, -1.5) -- (times-22);

  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (times-31) at (1, -3) {$\times$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (0, -2.25) {\scriptsize $\Sc$};
  \draw[arrow] (C-3) -- (mid) -- (0, -3) -- (times-31);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (2, -2.25) {\scriptsize $\Sc$};
  \draw[arrow] (times-21) -- (mid) -- (2, -3) -- (times-31);

  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (times-41) at (3.5, -4.5) {$\times$};
  \node[draw, rectangle, minimum size=15pt] (C-47) at (8, -4.5) {$\text{C}_{47}$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (1, -3.75) {\scriptsize $\Sc$};
  \draw[arrow] (times-31) -- (mid) -- (1, -4.5) -- (times-41);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (6, -3.75) {\scriptsize $\Sc$};
  \draw[arrow] (times-22) -- (mid) -- (6, -4.5) -- (times-41);

  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (assert-eq) at (5.75, -6) {$=_\text{Assert}$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (3.5, -5.25) {\scriptsize $\Sc$};
  \draw[arrow] (times-41) -- (mid) -- (3.5, -6) -- (assert-eq);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (8, -5.25) {\scriptsize $\Sc$};
  \draw[arrow] (C-47) -- (mid) -- (8, -6) -- (assert-eq);
\end{tikzpicture}
\caption{The example circuit from the Figure \ref{fig:example-circuit}, as a DAG, using the defined DAG nodes.}
\end{figure}

To arithmetize our program, yielding the polynomials required by the Plonk
prover and verifier, we need to extract the necessary constraint table from
the circuit and interpolate the columns to get the polynomials. We first
define some useful objects:

$$
\begin{alignedat}{3}
  &\textbf{WireId}   &&= \Nb                &&\quad \text{(A unique sequential id for each wire)}, \\
  &\textbf{SlotId}   &&= \Nb \times \Nb     &&\quad \text{(An entry in the constraint table, e.g. (4,3) refers to the fourth row, third column)}, \\
  &\textbf{WireType} &&= \{ \Fb_p, \Fb_q \} &&\quad \text{(Wires have a type, because a wire value can either be } \Fb_p \text{ or } \Fb_q\text{)}, \\
\end{alignedat}
$$

And a $\textbf{GateType}$, describing what kind of gate a node is:

$$
\begin{alignedat}{2}
  &\textbf{GateType} &&= \{ \\
  & &&"\text{W-}\Sc", "\text{P-}\Sc", "(-)", "(+)", "(\times)", "(x^{-1})", \\
  & &&"\text{W-}\Bb", "\text{P-}\Sc", "(=)", "(\land)", "(\lor)",           \\
  & &&"\text{W-}P", "\text{P-}P", "(+_{\text{EC}})", "(\times_{\text{EC}})", \\
  & &&"\Hc", "p \to q", "q \to p" \\
  &\} &&
\end{alignedat}
$$

The DAG can then be defined, with each vertex containing a $\textbf{GateType}$,
$n$ $\textbf{WireId}$'s representing the id's of the incoming wires and $n$
$\textbf{WireId}$'s representing the id's of the outgoing wires. The edges
has no associated data:

$$G = (V \in \textbf{GateType} \times \textbf{WireId}^n \times \textbf{WireId}^m, E \in \{\})$$

We iterate through the DAG in topological order, processing each node such
that all its predecessors, the nodes with edges pointing to it, are processed
before it. Throughout the iteration we store and get values from two maps,
$\text{ev}$, mapping each wire to a value, and $\text{cc}$, mapping each
wire to a set of slot-ids:

$$\text{ev} \in \textbf{WireId} \to \{ \Fb_p, \Fb_q \}, \; \text{cc} \in \textbf{WireId} \to \{ \textbf{SlotId}\}$$

$\text{ev}$ represents the evaluated values of each wire, and $\text{cc}$
represents the slot-ids, the entries of the constraint table, that should
be copy constrained to be equal. Then for each node in this iteration we
have a node:

$$\forall v \in V : v =  (t \in \textbf{GateType}, \, \vec{i} \in \textbf{WireId}^n, \, \vec{o} \in \textbf{WireId}^m)$$

1. We look up all inputs for the current node $v$, in the $\text{ev}$
   map. These lookups will always yield a value since the value has been written
   in a previous iteration, due to the topological iteration order:
   $$\vec{\mathrm{ev}^{(i)}} = [\text{ev}(i_1), \dots, \text{ev}(i_n)]$$
2. We compute the evaluation of the operation applied to the input values. The
   gate type $t$ decides what operation, $\text{op} \in \Fb^n \to \Fb^m$, shall be
   performed:
   $$\vec{\mathrm{ev}^{(o)}} = \text{op}(\vec{\mathrm{ev}^{(i)}}), \quad \forall k \in [n] : \text{ev}(o_k) = \text{ev}^{(i)}_k$$
   For example, for the addition gate, $\text{op}$ would be defined as:
   $\text{op}_{(+)}([\text{ev}^{(i)}_1, \text{ev}^{(i)}_2]) = \text{ev}^{(i)}_1 + \text{ev}^{(i)}_2$
3. Now we can append a row to the constraint table with the computed values
   according to the specification in the custom gates section. We also add
   any relevant coefficient rows.
4. Finally, we add the input and output wires to the copy constraint map:
   $$
   \begin{aligned}
     \forall k \in [n] : \text{cc}(i_k) = i^{(\text{SlotId})}_k, \\
     \forall k \in [m] : \text{cc}(o_k) = o^{(\text{SlotId})}_k
   \end{aligned}
   $$

It is important that we designate the first $\ell_2$ rows for public inputs,
but other than that, the above loop describes how to construct the trace
table. From here we just interpolate each column to get the witness, selector,
coefficient and copy constraint polynomials.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]

Consider the following small example circuit:

\begin{figure}[H]
\centering
\begin{tikzpicture}
  % First Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (WI-x1) at (0, 0) {W-$\Sc_{x_1}$};
  \node[draw, rectangle, minimum size=15pt] (PI-x2) at (2, 0) {P-$\Sc_{x_2}$};

  % Second Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (add) at (1, -1.5) {$+$};
  \node[draw, rectangle, minimum size=15pt] (C-5) at (3, -1.5) {$\text{C}_5$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (0, -0.75) {\scriptsize $\Sc, \texttt{wire\_id} = 1$};
  \draw[arrow] (WI-x1) -- (mid) -- (0, -1.5) -- (add);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (2, -0.75) {\scriptsize $\Sc, \texttt{wire\_id} = 2$};
  \draw[arrow] (PI-x2) -- (mid) -- (2, -1.5) -- (add);

  % Third Layer
  %%%%%%%%%% Nodes %%%%%%%%%%
  \node[draw, rectangle, minimum size=15pt] (eq) at (2, -3) {$=_{\text{CC}}$};

  %%%%%%%%%% Arrows %%%%%%%%%%
  \node[minimum size=2pt, inner sep=1pt] (mid) at (1, -2.25) {\scriptsize $\Sc, \texttt{wire\_id} = 3$};
  \draw[arrow] (add) -- (mid) -- (1, -3) -- (eq);
  \node[minimum size=2pt, inner sep=1pt] (mid) at (3, -2.25) {\scriptsize $\Sc, \texttt{wire\_id} = 4$};
  \draw[arrow] (C-5) -- (mid) -- (3, -3) -- (eq);
\end{tikzpicture}
\caption{A smaller example circuit representing the claim that I know a private scalar $x_1$ and a public scalar $x_2$, s.t. $x_1 + x_2 = 5$.}
\end{figure}

We iterate through this circuit in topological order, so we can start with
either of the two inputs. We instantiate the arithmetization with $x_1 = 2,
x_2 = 3$, but we leave them as variables in the following description:

\begin{itemize}
  \item $\text{Node}_1 = ("\text{W-}\Sc_{x_1}", \vec{i} = [ \; ], \vec{o} = [ 1 ])$:
  \begin{enumerate}
    \item There are no inputs.
    \item There is no computation so: $\text{op}_{\text{W-}\Sc_{x_1}}([ \; ]) = \vec{\mathrm{ev}^{(o)}} = [x_1], \quad \text{ev}(o_1) = \text{ev}_1^{(o)} = x_1$.
    \item For private inputs, there is no rows added to the constraint table.
    \item Since there is no row, there is no \textbf{SlotId} to add to the copy constraints.
  \end{enumerate}
  \item $\text{Node}_2 = ("\text{P-}\Sc_{x_2}", \vec{i} = [ \; ], \vec{o} = [ 2 ])$:
  \begin{enumerate}
    \item There are no inputs.
    \item There is no computation so: $\text{op}_{\text{P-}\Sc_{x_2}}([ \; ]) = \vec{\mathrm{ev}^{(o)}} = [x_2], \quad \text{ev}(o_1) = \text{ev}_1^{(o)} = x_2$.
    \item For public inputs, we add the following witnesses and selector polynomials:
    \begin{table}[H]
      \centering
      \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
        \hline
        $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$   & $w_9$         & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
        $x_2$ & 0     & 0     & 0      & 0      & 0      & 0      & 0       & 0             & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline\hline
        $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_\Hc$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$   &           &           &           &           &           \\\tabucline[1pt]{-}
        1     & 0     & 0     & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
      \end{tabu}
    \end{table}
    \item We add the slot-id of $x_2$ $(1, 1)$ to the copy constraints of the output wire with id $2$:
      $$\text{cc}(o_1 = 2) = \text{cc}(o_1) \cup \{ \, o_1^{\text{SlotId}} = (1,1) \, \}$$
  \end{enumerate}
  \item $\text{Node}_3 = ("(+)", \vec{i} = [ 1, 2 ], \vec{o} = [ 3 ])$:
  \begin{enumerate}
    \item We lookup the two inputs: $\vec{\mathrm{ev}^{(i)}} = [\text{ev}(i_1 = 1) = x_1, \text{ev}(i_2 = 2) = x_2]$.
    \item Perform the computation:
      $$\text{op}_{(+)}(\vec{\mathrm{ev}^{(i)}}) = \vec{\mathrm{ev}^{(o)}} = [\text{ev}^{(i)}_1 + \text{ev}^{(i)}_2] = [x_1 + x_2] = [x_3], \quad \text{ev}(o_1) = \text{ev}_1^{(i)} = x_3$$
    \item For addition, we add the following witnesses and selector polynomials:
    \begin{table}[H]
      \centering
      \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
        \hline
        $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$   & $w_9$         & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
        $x_1$ & $x_2$ & $x_3$ & 0      & 0      & 0      & 0      & 0       & 0             & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline\hline
        $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_\Hc$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$   &           &           &           &           &           \\\tabucline[1pt]{-}
        1     & 1     & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
      \end{tabu}
    \end{table}
    \item We add the slot-ids to the copy constraints:
      $$
      \begin{aligned}
        \text{cc}(i_1 = 1) &= \text{cc}(i_1) \cup \{ \, i_1^{\text{SlotId}} = (2,1) \, \} \\
        \text{cc}(i_2 = 2) &= \text{cc}(i_2) \cup \{ \, i_2^{\text{SlotId}} = (2,2) \, \} \\
        \text{cc}(o_1 = 3) &= \text{cc}(o_1) \cup \{ \, o_1^{\text{SlotId}} = (2,3) \, \}
      \end{aligned}
      $$
  \end{enumerate}
  \item $\text{Node}_4 = ("\text{C}_5", \vec{i} = [ \; ], \vec{o} = [ 4 ])$:
  \begin{enumerate}
    \item There are no inputs.
    \item There is no computation so:
       $$\text{op}_{\text{C}_5}([ \; ]) = 5, \quad \vec{\mathrm{ev}} = [ \text{ev}^{(i)}_1 = 5 ], \quad \text{ev}(o_1) = \text{ev}_1^{(i)} = 5$$
    \item For a constant gate, we add the following witnesses and selector polynomials:
    \begin{table}[H]
      \centering
      \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
        \hline
        $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$   & $w_9$         & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
        5     & 0     & 0     & 0      & 0      & 0      & 0      & 0       & 0             & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline\hline
        $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_\Hc$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$   &           &           &           &           &           \\\tabucline[1pt]{-}
        1     & 0     & 0     & 0     & -5    & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
      \end{tabu}
    \end{table}
    \item We add the slot-id to the copy constraints:
      $$\text{cc}(o_1 = 4) = \text{cc}(o_1) \cup \{ \, o_1^{\text{SlotId}} = (3,1) \, \}$$
  \end{enumerate}
  \item $\text{Node}_5 = ("(=_{\text{CC}})", \vec{i} = [ 3, 4 ], \vec{o} = [ \; ])$:
  \begin{enumerate}
    \item We lookup the two inputs: $\vec{\mathrm{ev}^{(i)}} = [\text{ev}(i_1 = 3) = x_3, \text{ev}(i_2 = 4) = 5]$
    \item There is no output, so no computation.
    \item For a constant gate, we add the following witnesses and selector polynomials:
    \begin{table}[H]
      \centering
      \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
        \hline
        $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$   & $w_9$         & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
        $x_3$ & 5     & 0     & 0      & 0      & 0      & 0      & 0       & 0             & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline\hline
        $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_\Hc$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$   &           &           &           &           &           \\\tabucline[1pt]{-}
        1     & -1    & 0     & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
      \end{tabu}
    \end{table}
    \item We add the slot-id to the copy constraints:
      $$
      \begin{aligned}
        \text{cc}(i_1 = 3) &= \text{cc}(i_1) \cup \{ \, i_1^{\text{SlotId}} = (4,1) \, \} \\
        \text{cc}(i_2 = 4) &= \text{cc}(i_2) \cup \{ \, i_2^{\text{SlotId}} = (4,2) \, \}
      \end{aligned}
      $$
  \end{enumerate}
\end{itemize}

Which finishes the arithmetization loop. If we arithmetized with $x_1 = 2,
x_2 = 3$ we would get the following table:

\begin{table}[H]
  \centering
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$     & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$   & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $x_2 = 3$ & 0         & 0         & 0      & 0      & 0      & 0      & 0       & 0         & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $x_1 = 2$ & $x_2 = 3$ & $x_3 = 5$ & 0      & 0      & 0      & 0      & 0       & 0         & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $x_4 = 5$ & 0         & 0         & 0      & 0      & 0      & 0      & 0       & 0         & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $x_3 = 5$ & $x_4 = 5$ & 0         & 0      & 0      & 0      & 0      & 0       & 0         & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline\hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_\Hc$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$   &           &           &           &           &           \\\tabucline[1pt]{-}
    1     & 0     & 0     & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
    1     & 1     & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
    1     & 0     & 0     & 0     & -5    & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
    1     & -1    & 0     & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0         &           &           &           &           &           \\\hline
  \end{tabu}
\end{table}

The copy constraints were defined as:

$$
\begin{aligned}
  \text{cc}(1) &= \{ \, (2,1) \, \} \\
  \text{cc}(2) &= \{ \, (1,1), (2,2) \, \} \\
  \text{cc}(3) &= \{ \, (2,3), (4,1) \, \} \\
  \text{cc}(4) &= \{ \, (3,1), (4,2) \, \} \\
\end{aligned}
$$

Which gives us the following copy-constraint table:

\begin{table}[H]
  \centering
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $\omega^i$ & $id_1$ & $id_2$ & $id_3$ & $id_4$ & $id_5$ & $id_6$ & $\s_1$ & $\s_2$ & $\s_3$ & $\s_4$ & $\s_5$ & $\s_6$ \\\tabucline[1pt]{-}
    $\omega^1$ & 1      & 5      & 9      & 13     & 17     & 21     & 6      & 5      & 9      & 13     & 17     & 21     \\\hline
    $\omega^2$ & 2      & 6      & 10     & 14     & 18     & 22     & 2      & 1      & 4      & 14     & 18     & 22     \\\hline
    $\omega^3$ & 3      & 7      & 11     & 15     & 19     & 23     & 8      & 7      & 11     & 15     & 19     & 23     \\\hline
    $\omega^4$ & 4      & 8      & 12     & 16     & 20     & 24     & 10     & 3      & 12     & 16     & 20     & 24     \\\hline
  \end{tabu}
\end{table}

From here, we just need to interpolate each column into a polynomial.

\end{tcolorbox}

