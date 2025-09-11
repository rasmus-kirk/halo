## Protocol Components

### Gate Constraints

Imagine we want to prove that we have a witness for $3x_1^2 + 5x_2 = 47$,
meaning we want to show that we know $x_1, x_2$ such that the equation equals
47. We can represent that equation as a simple circuit.

\begin{figure}[ht]
  \centering
  \begin{subfigure}[b]{0.40\textwidth}
    \centering
    \begin{tikzpicture}
      % First Layer
      \node (input1) at (3, 7) {$x_1$};
      \node (input2) at (5, 7) {$x_2$};
      \node (A) at (1, 7) {$3$};
      \node (B) at (7, 7) {$5$};
      % Second Layer
      \node[draw, rectangle] (mul21) at (3, 6) {$\times$};
      \node[above left=0.01cm of mul21] {$a_1$};
      \node[above right=0.01cm of mul21] {$b_1$};
      \node[below right=0.01cm of mul21] {$c_1$};
      \node[draw, rectangle] (mul22) at (6, 6) {$\times$};
      \node[above left=0.01cm of mul22] {$a_2$};
      \node[above right=0.01cm of mul22] {$b_2$};
      \node[below right=0.01cm of mul22] {$c_2$};
      \draw[->] (input1) -- (2, 7) |- (mul21);
      \draw[->] (input1) -- (4, 7) |- (mul21);
      \draw[->] (input2) -- (5, 6.5) |- (mul22);
      \draw[->] (B) -- (7, 6.5) |- (mul22);
      % Third Layer
      \node[draw, rectangle] (mul31) at (2, 5) {$\times$};
      \node[above left=0.01cm of mul31] {$a_3$};
      \node[above right=0.01cm of mul31] {$b_3$};
      \node[below right=0.01cm of mul31] {$c_3$};
      \draw[->] (mul21) -- (3, 5) |- (mul31);
      \draw[->] (A) -- (1, 5) |- (mul31);
      % Fourth Layer
      \node[draw, rectangle] (add41) at (4, 4) {$+$};
      \node[above left=0.01cm of add41] {$a_4$};
      \node[above right=0.01cm of add41] {$b_4$};
      \node[below right=0.01cm of add41] {$c_4$};
      \draw[->] (mul31) -- (2, 4) |- (add41);
      \draw[->] (mul22) -- (6, 4) |- (add41);
      % Fifth Layer
      \node (output) at (4, 3) { $47$ };
      \draw[->] (add41) -- (output);
    \end{tikzpicture}
  \end{subfigure}
  \begin{subfigure}[b]{0.40\textwidth}
    \centering
    \begin{tikzpicture}
      % First Layer
      \node (input1) at (3, 7) {$2$};
      \node (input2) at (5, 7) {$7$};
      \node (A) at (1, 7) {$3$};
      \node (B) at (7, 7) {$5$};
      % Second Layer
      \node[draw, rectangle] (mul21) at (3, 6) {$\times$};
      \node[draw, rectangle] (mul22) at (6, 6) {$\times$};
      \draw[->] (input1) -- (2, 7) |- (mul21);
      \draw[->] (input1) -- (4, 7) |- (mul21);
      \draw[->] (input2) -- (5, 6.5) |- (mul22);
      \draw[->] (B) -- (7, 6.5) |- (mul22);
      % Third Layer
      \node[draw, rectangle] (mul31) at (2, 5) {$\times$};
      \node[above right=0.01cm of mul31] {4};
      \draw[->] (mul21) -- (3, 5) |- (mul31);
      \draw[->] (A) -- (1, 5) |- (mul31);
      % Fourth Layer
      \node[draw, rectangle] (add41) at (4, 4) {$+$};
      \node[above left=0.01cm of add41] {$12$};
      \node[above right=0.01cm of add41] {$35$};
      \draw[->] (mul31) -- (2, 4) |- (add41);
      \draw[->] (mul22) -- (6, 4) |- (add41);
      % Fifth Layer
      \node (output) at (4, 3) { $47$ };
      \draw[->] (add41) -- (output);
    \end{tikzpicture}
  \end{subfigure}
  \caption{
    Two ways of viewing the circuit representing $3x_1^2 + 5x_2 = 47$. The
    left circuit is also instantiated with the witness $x_1, x_2$.
  }
  \label{fig:example-circuit}
\end{figure}

This is a trivial problem, so we deduce that $x_1 = 2, x_2 = 7$. From the graphs
above, we can construct vectors representing the wire values of our circuit:

$$
\begin{aligned}
  \vec{w} &= [ 2, 7, 4, 3, 12, 5, 35, 47 ] \\
  \vec{a} &= [ 2, 7, 3, 12 ] \\
  \vec{b} &= [ 2, 5, 4, 35 ] \\
  \vec{c} &= [ 4, 35, 12, 42 ]
\end{aligned}
$$

We can then create polynomials $a(X), b(X), c(X)$ corresponding to the
left-input wire, the right-input wire and the output wire respectively:

$$
\begin{aligned}
  a(X) = \text{lagrange}(\vec{a}) \\
  b(X) = \text{lagrange}(\vec{b}) \\
  c(X) = \text{lagrange}(\vec{c})
\end{aligned}
$$

Now, we can use selector polynomials, $q_l(X), q_r(X), q_o(X), q_m(X),
q_c(X)$, to show that the constructed polynomials $a(X), b(X), c(X)$ satisfy
the circuit relations by proving that a constructed polynomial $f_{GC}(X)
= 0$ at $i = [1, 8]$:

$$f_{GC}(X) = a(X) q_l(X) + b(X) q_r(X) + c(X) q_o(X) + a(X) b(X) q_m(X) + q_c(X)$$

Where $a(X), b(X), c(X)$ are private and the selector polynomials are
public. Notice that we can represent this as a table:

\begin{center}
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|}
    \hline
    $i$ & $a(i)$ & $b(i)$ & $c(i)$ & $q_l(i)$ & $q_r(i)$ & $q_o(i)$ & $q_m(i)$ & $q_c(i)$ \\\tabucline[1pt]{-}
    $1$ & 3      & 0      & 0      & 1        & 0        & 0        & 0        & -3       \\\hline
    $2$ & 5      & 0      & 0      & 1        & 0        & 0        & 0        & -5       \\\hline
    $3$ & 47     & 0      & 0      & 1        & 0        & 0        & 0        & -47      \\\hline
    $4$ & 2      & 2      & 4      & 0        & 0        & -1       & 1        & 0        \\\hline
    $5$ & 5      & 7      & 35     & 0        & 0        & -1       & 1        & 0        \\\hline
    $6$ & 4      & 3      & 12     & 0        & 0        & -1       & 1        & 0        \\\hline
    $7$ & 35     & 12     & 47     & 1        & 1        & -1       & 0        & 0        \\\hline
    $8$ & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        \\\hline
  \end{tabu}
\end{center}

Lagrange interpolation is slow, with a runtime of $\Oc(n^2)$, we can
instead use FFT to construct our polynomials, which has a runtime of $\Oc(n
\log(n))$. For this, we construct the polynomials over the roots of unity
($\o^1, \o^2, \dots, \o^8$ where $\o$ is the 8'th root of unity), meaning
that our table becomes:

\begin{center}
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|}
    \hline
    $\o^i$ & $a(\o^i)$ & $b(\o^i)$ & $c(\o^i)$ & $q_l(\o^i)$ & $q_r(\o^i)$ & $q_o(\o^i)$ & $q_m(\o^i)$ & $q_c(\o^i)$ \\\tabucline[1pt]{-}
    $\o^1$ & 3         & 0         & 0         & 1           & 0           & 0           & 0           & -3          \\\hline
    $\o^2$ & 5         & 0         & 0         & 1           & 0           & 0           & 0           & -5          \\\hline
    $\o^3$ & 47        & 0         & 0         & 1           & 0           & 0           & 0           & -47         \\\hline
    $\o^4$ & 2         & 2         & 4         & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^5$ & 5         & 7         & 35        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^6$ & 4         & 3         & 12        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^7$ & 35        & 12        & 47        & 1           & 1           & -1          & 0           & 0           \\\hline
    $\o^8$ & 0         & 0         & 0         & 0           & 0           & 0           & 0           & 0           \\\hline
  \end{tabu}
\end{center}

Now we wish to prove that:

$$\forall \o \in H = \{ \o^1, ..., \o^6 \} : f_{GC}(X) = 0$$

And for this, we can use the **Vanishing Argument Protocol**. And in order
for the verifier to know that $f_{GC}(X)$ is constructed honestly, i.e. it
respects the public selector polynomials, we can use the **Batched Evaluations
Proofs Protocol** over each witness polynomial instead of $f_{GC}(X)$. This
securely gives the verifier $v_{f_{GC}} = f_{GC}(\xi), v_a = a(\xi), v_b = b(\xi), v_c = c(\xi)$
and the verifier can then check:

$$v_f = v_a q_l(\xi) + v_b q_r(\xi) + v_c q_o(\xi) + v_a v_b q_m(\xi) + q_c(\xi)$$

We still need to handle copy constraints, because as can be seen in the table,
we need to verify identities between wires (like $a(\o^1) = b(\o^1)$). For
this we need _Copy Constraints._

### Copy Constraints

For example we want to show that $a(\o^1) = b(\o^1)$, first we concatinate
the lists $\vec{a}, \vec{b}, \vec{c}$:

$$\vec{f} = [ 2, 7, 3, 12 ] \cat [ 2, 5, 4, 35 ] \cat [ 4, 35, 12, 42 ] = [ 2, 7, 3, 12, 2, 5, 4, 35, 4, 35, 12, 42 ]$$

Now, we wish to show, that for some permutation $\sigma: \Fb^k \to \Fb^k$,
the list remains unchanged once permuted:

$$\vec{f} = \sigma(\vec{f})$$

This permutation permutes the list according to what wires we wish to show are equal:

$$\vec{f} = [ 2, 7, 3, 12 ] \cat [ 2, 5, 4, 35 ] \cat [ 4, 35, 12, 42 ]$$

From the circuit in Figure \ref{fig:example-circuit} we gather that the following wires
must be equal:

$$a_1 = b_1, \quad c_1 = b_3, \quad c_3 = a_4, \quad c_2 = b_4$$

To highlight the values of $\vec{f}$ and $\sigma(\vec{f})$, the specific values
have been subbed out for variables below:

$$
\begin{aligned}
  \vec{f} &= [ a_1, a_2, a_3, a_4 ] \cat [ b_1, b_2, b_3, b_4 ] \cat [ c_1, c_2, c_3, c_4 ] \\
  \sigma(\vec{f}) &= [ b_1, a_2, a_3, c_3 ] \cat [ a_1, b_2, c_1, c_2 ] \cat [ b_3, b_4, a_4, c_4 ]
\end{aligned}
$$

If the prover is honest, it's easy to see that these lists will match,
in fact, that's why we have to use variables in the above list, otherwise
the permutation _seems_ to do nothing. But as can also be seen above,
if the prover tries to cheat by violating $a_1 = b_1$ then the permuted
$\sigma(\vec{f})$ will not be equal to the original $\vec{f}$. As in the above
section we can model the vectors as polynomials using FFT, such that $f(\o^1)
= f_1, f(\o^2) = f_2 \dots$.

Then, given the polynomial $f(X)$ we want to check whether:

$$\forall i \in [n] : f(\o^i) \meq f(\o^{\s(i)})$$

Where $n = |H|$. One approach is to use the **Grand Product Argument**,
defined earlier, which would show:

$$\prod_{i = 1}^n f(\o^i) = \prod_{i = 1}^n f(\o^{\s(i)})$$

But this only proves there exists _some_ permutation between $f(X)$ and
itself, not necessarily $\s$. We can start by trying to model sets of
pairs, rather than just sets:
$$\{ (a_i, b_i) \mid i \in [1,n] \} = \{ (c_i, d_i) \mid i \in [1,n] \}$$
Which can be modelled with:
$$f(X) = a_i(X) + \beta b_i(X), \quad g(X) = c_i(X) + \beta d_i(X)$$

**Correctness:**

We need to prove that:
$$(a(\o^i), b(\o^i)) = (c(\o^i), d(\o^i)) \implies a(\o^i) + \beta b(\o^i) = c(\o^i) + \beta d(\o^i)$$
Which holds trivially, since the LHS implies $a(\o^i) = c(\o^i), b(\o^i) = d(\o^i)$, meaning
we can rewrite:
$$a(\o^i) + \beta b(\o^i) = a(\o^i) + \beta b(\o^i)$$

$\qed$

**Soundness:**

We need to prove that for a uniformly random $\b$:

$$a(\o^i) + \b b(\o^i) = c(\o^i) + \b d(\o^i) \implies a(\o^i) = c(\o^i) \land b(\o^i) = d(\o^i)$$

Except with negligible probability. Assuming $a(\o^i) + \b b(\o^i) \neq
c(\o^i) + \b d(\o^i) \land g(\o^i)$:

$$
\begin{aligned}
  a(\o^i) + \b b(\o^i) &= c(\o^i) + \b d(\o^i) \implies \\
  a(\o^i) - c(\o^i) &= \b d(\o^i) - \b b(\o^i) \implies \\
  a(\o^i) - c(\o^i) &= \b \cdot (d(\o^i) - b(\o^i)) \implies \\
  \b &= \frac{a(\o^i) - c(\o^i)}{d(\o^i) - b(\o^i)}
\end{aligned}
$$

Since all the left-hand sides are constant and $\b$ is uniformly random in
$\Fb$, there is a $1 / |\Fb|$ probability that the claim doesn't hold. Thus,
we have soundness.

$\qed$

The prover then wants to prove that for $i \in [n] : f_i = f_{\s(i)}$,
for a specific permutation $\s$:
$$\{ (f_i, i) \mid i \in [1,n] \} = \{ (f_i, \s(i)) \mid i \in [1,n] \} \implies f_i = f_{\s_i} \implies \vec{f} = \s(\vec{f})$$
Which for polynomials:
$$\{ (f(\o^i), \id(\o^i)) \mid i \in [1,n] \} = \{ (f(\o^i), \s(\o^i)) \mid i \in [1,n] \} \implies f(\o^i) = f(\o^{\s(i)})$$
So _now_ we can use the **Grand Product Argument** to prove
that $\forall i \in [n] : f(\o^i) \meq f(\o^{\s(i)})$, with:
$$f'(X) = f(X) + \beta \id(X), \quad g'(X) = f(X) + \beta \s(X)$$

\begin{tcolorbox}[colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]

An example, without soundness values $\b, \g$, for why this approach to
proving $\sigma(\vec{f}) = \vec{f}$ is sensible:

$$
  \begin{alignedat}{10}
    \id(1) &= 1 \quad & \id(2) &= 2 \quad & \id(3) &= 3 \quad & \id(4) &= 4 \quad & \id(5) &= 5 \quad & \id(6) &= 6 \quad \\
    \s(1)  &= 1 \quad & \s(2)  &= 4 \quad & \s(3)  &= 5 \quad & \s(4)  &= 6 \quad & \s(5)  &= 3 \quad & \s(6)  &= 2 \quad
  \end{alignedat}
$$

$$
\begin{aligned}
  \prod_{\o \in H} (f(\o) + \id(\o)) &= (f(\o^1) + 1)(f(\o^2) + 2)(f(\o^3) + 3)(f(\o^4) + 4)(f(\o^5) + 5)(f(\o^6) + 6) \\
  \prod_{\o \in H} (f(\o) + \s(\o))  &= (f(\o^1) + 1)(f(\o^2) + 4)(f(\o^3) + 5)(f(\o^4) + 6)(f(\o^5) + 3)(f(\o^6) + 2) \\
                                     &= (f(\o^1) + 1)(f(\o^4) + 4)(f(\o^5) + 5)(f(\o^6) + 6)(f(\o^3) + 3)(f(\o^2) + 2) \\
                                     &= (f(\o^1) + 1)(f(\o^2) + 2)(f(\o^3) + 3)(f(\o^4) + 4)(f(\o^5) + 5)(f(\o^6) + 6)
\end{aligned}
$$
\end{tcolorbox}

#### Permutation Argument Over Multiple Polynomials

In Plonk, we don't have a single polynomial spanning over each $\vec{a},
\vec{b}, \vec{c}$. Since the Grand Product Argument operates over products,
we can define:

$$
\begin{aligned}
  f_a(X) &= a(X) + \id(X) \b + \g \\
  f_b(X) &= b(X) + \id(n + X) \b + \g \\
  f_c(X) &= c(X) + \id(2n + X) \b + \g \\
  g_a(X) &= a(X) + \s(X) \b + \g \\
  g_b(X) &= b(X) + \s(n + X) \b + \g \\
  g_c(X) &= c(X) + \s(2n + X) \b + \g \\
  f(X) &= f_a(X) \cdot f_b(X) \cdot f_c(X) \\
  g(X) &= g_a(X) \cdot g_b(X) \cdot g_c(X)
\end{aligned}
$$

Which means that for our example circuit in Figure \ref{fig:example-circuit},
we now get the table:

\begin{center}
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $\omega^i$ & $a$   & $b$   & $c$   & $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $id_a$ & $id_b$ & $id_c$ & $\s_a$ & $\s_b$ & $\s_c$ \\\tabucline[1pt]{-}
    $\omega^1$ & 3     & 0     & 0     & 1     & 0     & 0     & 0     & -3    & 1      & 9      & 17     & 14     & 9      & 17     \\\hline
    $\omega^2$ & 5     & 0     & 0     & 1     & 0     & 0     & 0     & -5    & 2      & 10     & 18     & 5      & 10     & 18     \\\hline
    $\omega^3$ & 47    & 0     & 0     & 1     & 0     & 0     & 0     & -47   & 3      & 11     & 19     & 23     & 11     & 19     \\\hline
    $\omega^4$ & 2     & 2     & 4     & 0     & 0     & -1    & 1     & 0     & 4      & 12     & 20     & 12     & 4      & 6      \\\hline
    $\omega^5$ & 5     & 7     & 35    & 0     & 0     & -1    & 1     & 0     & 5      & 13     & 21     & 2      & 13     & 7      \\\hline
    $\omega^6$ & 4     & 3     & 12    & 0     & 0     & -1    & 1     & 0     & 6      & 14     & 22     & 20     & 1      & 15     \\\hline
    $\omega^7$ & 35    & 12    & 47    & 1     & 1     & -1    & 0     & 0     & 7      & 15     & 23     & 21     & 22     & 3      \\\hline
    $\omega^8$ & 0     & 0     & 0     & 0     & 0     & 0     & 0     & 0     & 8      & 16     & 24     & 8      & 16     & 24     \\\hline
  \end{tabu}
\end{center}

### Public Inputs

It might be useful to have public inputs for a circuit. This is not to be confused with constants in the circuits:

- _A constant_ is always the value set by the circuit, and is public, known by both the prover and verifier.
- _A witness_ is an input value to the circuit, and is private, known only by the prover.
- _A public input_ is an input value to the circuit, and is public, known by both the prover and verifier.

We have a vector of public inputs:
$$\vec{x} : |\vec{x}| = \ell_2$$
Naturally leading to a polynomial:
$$x(X) = \ifft(\vec{x})$$
The number of public inputs, $\ell_2$, is embedded in the circuit
specification. The first $\ell_2$ rows of the witness table is reserved for
public inputs. For each $x_i \in \vec{x}$, we set $q_l(\o^i) = 1, a(\o^i) =
x_i$ and the rest of the witness and selector polynomials to zero. $F_{GC}$
must then also be updated:

$$f_{GC}(X) = a(X) q_l(X) + b(X) q_r(X) + c(X) q_o(X) + a(X) b(X) q_m(X) + q_c(X) + x(X)$$

### Input Passing

Since we use a cycle of curves, each language instruction is mapped to one of
two circuits, verifying both circuits should convince the verifier that the
program $f(w, x)$ is satisfied. However, for Elliptic Curve Multiplication
and the Poseidon Hashes, we need to pass inputs from one circuit to another.

**Passing $v^{(q)} \to v^{(p)}$:**

We start with the simple case. We have a circuit over $\Fb_p$, $R^{(p)}$,
and a circuit over $\Fb_q$, $R^{(q)}$, with $p > q$. We wish to pass a value,
$v^{(q)} \in \Fb_q$, from $R^{(q)}$ to $R^{(p)}$. We can add $v^{(p)}$ to the
public inputs to $R^{(q)}$, but then we still need to convince the verifier
that $v^{(q)} = v^{(q)}$. Naively, the verifier could add the check that
$v^{(q)} \meq v^{(p)}$. But this won't work for IVC, since we can't check
equality across circuits, in-circuit. Instead we compute the commitment to
$v^{(q)}$ on the $R^{(q)}$-side.

$$C^{(q)}_\IP := v^{(q)} \cdot G_1^{(q)} \in \Eb_p(\Fb_q)$$

The scalar operation may seem invalid, but since we know that $v^{(q)}
\leq q - 1 < p - 1$, it can logically be computed by the usual double and
add, since the bits of $v^{(q)}$ will correspond to the bits of $v^{(p)}$
if $\text{lift}(v^{(q)}) = \text{lift}(v^{(p)})$. If $C^{(q)}_\IP$ is emitted in
the public inputs of the circuit, then the verifier will know that $C^{(q)}_\IP$
is a commitment to $v^{(q)}$. To convince the verifier of the desired relation
that $\text{lift}(v^{(q)}) = \text{lift}(v^{(p)})$, it will now suffice to
convince them that $v^{(p)}$ is a valid opening of $C^{(q)}_\IP$. So the verifier
checks manually that:

$$C^{(q)}_\IP \meq v^{(p)} \cdot G_1^{(q)}$$

Which, given that the rest of the proof verifies correctly, will then imply
that $v^{(q)} = v^{(p)}$. If the verifier is encoded as a circuit, then
we need to input pass when performing this additional check, since scalar
multiplication itself requires input passing to work. However this is no
problem, since that circuit-verifier will be verified by another verifier! At
some point, this deferral will end with a regular verifier, that can compute
the commitment outside the circuit.

**Passing $v^{(p)} \to v^{(q)}$:**

What if we reverse the flow? We now have a value $v^{(p)}$, in $R^{(p)}$,
that we want to pass to $R^{(q)}$. Here the problem is that since $p > q$,
the value might be too large to represent in the $\Fb_q$-field. The solution
is to decompose the value:

$$v^{(p)}_p = 2 h^{(p)} + l^{(p)}$$

Where $h^{(p)}$ represents the high-bits of $v^{(p)}$ ($h^{(p)} \in [0,
2^{\floor{\log{p}}}]$) and $l^{(p)}$ represents the low-bit ($h^{(p)} \in
\Bb$). The value $v^{(p)}$ can now be represented with $h^{(p)}, l^{(p)}$, both
of which are less than $q$. Which means we can pass the value to $R^{(q)}$.

The constraints added to $R^{(p)}$ then becomes:

- $C^{(p)}_\IP \meq h \cdot G_1^{(p)} + l \cdot G_2^{(p)}$
- $v = 2 h^{(p)} + l^{(p)}$
- $h \in [0, 2^{\floor{\log{p}}}]$ (Using the rangecheck gate, corresponding to a range proof)
- $l \in \Bb$ (A Simple boolean constraint)

**Combining Commitments:**

We of course don't need to commit each time we pass inputs, we can create
a standard vector pedersen commit, containing all the passed values:

$$C^{(p)}_\IP = h_{v_1}^{(p)} \cdot G_1^{(p)} + l_{v_1}^{(p)} \cdot G_2^{(p)} + h_{v_2}^{(p)} \cdot G_3^{(p)} + l_{v_2}^{(p)} \cdot G_4^{(p)} + \dots$$

Now, the $R_q$-verifier and $R_p$-verifier, would each also take in a
single input pass vector, in addition to the standard public input vector:
$$\text{InputPass}^{(q \to p)} \in \Fb_p^k, \qquad \text{InputPass}^{(p \to q)} \in \Fb_q^k$$

Each passed input is of course public, so the public input vector is then
defined as:

$$\text{PublicInputs}^{(p)}_{\text{new}} := \text{PublicInputs}^{(p)} \cat \text{InputPass}^{(p)}$$

For both the verifier and prover of course. Each of the $R^{(p)}$ and $R^{(q)}$
verifier can then use $\text{InputPass}^{(q \to p)}, \text{InputPass}^{(p \to q)}$
to verify $C^{(p)}_\IP, C^{(q)}_\IP$:

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
    \item $C^{(p)}_\IP := h_z^{(p)} \cdot G_1^{(p)} + l_z^{(p)} \cdot G_2^{(p)} + h_x^{(p)} \cdot G_3^{(p)} + l_x^{(p)} \cdot G_4^{(p)}$
    \item $z := 2 \cdot h_z^{(p)} + l_z^{(p)}$ (Decomposition correctness check)
    \item $h_z^{(p)} \in [0, 2^{\floor{\log{p}}}]$ (Range check)
    \item $l_z^{(p)} \in \Bb$ (Boolean check)
  \end{itemize}
  \item $R^{(q)}$:
  \begin{itemize}
    \item $C^{(q)}_\IP := \a^{(q)} \cdot G_1^{(q)}$
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
  C^{(p)}_\IP &\meq h_z^{(q)} \cdot G_1^{(p)} + l_z^{(q)} \cdot G_2^{(p)} + h_x^{(q)} \cdot G_3^{(p)} + l_x^{(q)} \cdot G_4^{(p)} \\
  C^{(q)}_\IP &\meq \a^{(p)} \cdot G_1^{(q)} \\
\end{aligned}
$$

\end{tcolorbox}
