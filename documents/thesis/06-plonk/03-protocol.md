## Protocol

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
\end{figure}

This is a trivial problem, so we deduce that $x_1 = 2, x_2 = 7$. From the graphs
above, we can construct some vectors representing the wire values of our circuit:

$$
\begin{aligned}
  \vec{w} &= [ 2, 7, 4, 3, 12, 5, 35, 47 ] \\
  \vec{a} &= [ 2, 7, 3, 12 ] \\
  \vec{b} &= [ 2, 5, 4, 35 ] \\
  \vec{c} &= [ 4, 35, 12, 42 ]
\end{aligned}
$$

We can then create polynomials $a(x), b(x), c(x)$ corresponding to the
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
= 0$ at $i = [1, 6]$:

$$f_{GC}(X) = a(X) q_l(X) + b(X) q_r(X) + c(X) q_o(X) + a(X) b(X) q_m(X) + q_c(X)$$

Where $a(X), b(X), c(X)$ are private and the selector polynomials are
public. Notice that we can represent this as a table:

\begin{center}
  \begin{tabular}{|c|c|c|c|c|c|c|c|c|}
    \hline
    $i$ & $a(i)$ & $b(i)$ & $c(i)$ & $q_l(i)$ & $q_r(i)$ & $q_o(i)$ & $q_m(i)$ & $q_c(i)$ \\
    \hline
    $1$ & $w_1 = 2$ & $w_1 = 2$ & $w_3 = 4$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $2$ & $w_4 = 3$ & & & 1 & 0 & 0 & 0 & -3 \\
    \hline
    $3$ & $w_3 = 4$ & $w_4 = 3$ & $w_5 = 12$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $4$ & $w_6 = 5$ & & & 1 & 0 & 0 & 0 & -5 \\
    \hline
    $5$ & $w_2 = 7$ & $w_6 = 5$ & $w_7 = 35$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $6$ & $w_5 = 12$ & $w_7 = 35$ & $w_8 = 47$ & 1 & 1 & -1 & 0 & 0 \\
    \hline
  \end{tabular}
\end{center}

Lagrange interpolation is slow, with a runtime of $\Oc(n^2)$, we can
instead use FFT to construct our polynomials, which has a runtime of $\Oc(n
\log(n))$. For this, we construct the polynomials over the roots of unity
($\o^1, \o^2, \dots, \o^6$ where $\o$ is the 6'th root of unity), meaning
that our table becomes:

\begin{center}
  \begin{tabular}{|c|c|c|c|c|c|c|c|c|}
    \hline
    $\o^i$ & $a(\o^i)$ & $b(\o^i)$ & $c(\o^i)$ & $q_l(\o^i)$ & $q_r(\o^i)$ & $q_o(\o^i)$ & $q_m(\o^i)$ & $q_c(\o^i)$ \\
    \hline
    $\o^1$ & $w_1 = 2$ & $w_1 = 2$ & $w_3 = 4$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $\o^2$ & $w_4 = 3$ & & & 1 & 0 & 0 & 0 & -3 \\
    \hline
    $\o^3$ & $w_3 = 4$ & $w_4 = 3$ & $w_5 = 12$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $\o^4$ & $w_6 = 5$ & & & 1 & 0 & 0 & 0 & -5 \\
    \hline
    $\o^5$ & $w_2 = 7$ & $w_6 = 5$ & $w_7 = 35$ & 0 & 0 & -1 & 1 & 0 \\
    \hline
    $\o^6$ & $w_5 = 12$ & $w_7 = 35$ & $w_8 = 47$ & 1 & 1 & -1 & 0 & 0 \\
    \hline
  \end{tabular}
\end{center}

Now we wish to prove that:

$$\forall \o \in H = \{ \o^1, ..., \o^6 \} : f_{GC}(X) = 0$$

And for this, we can use the **Vanishing Argument Protocol**. And in order for
the verifier to know that $f_{GC}$ is constructed honestly, i.e. it respects
the public selector polynomials, we can use the **Batched Evaluations Proofs
Protocol** over each witness polynomial instead of $f_{GC}$. This securely
gives the verifier $v_a = a(\xi), v_b = b(\xi), v_c = c(\xi)$ and the verifier
can then check:

$$v_f = v_a q_l(\xi) + v_b q_r(\xi) + v_c q_o(\xi) + v_a v_b q_m(\xi) + q_c(\xi)$$

We still need to handle copy constraints, because as can be seen in the table,
we need to verify identities between wires (like $a(\o^1) = b(\o^1)$). For
this we need _Copy Constraints._

### Copy Constraints

For the copy constraints it helps to visualize the list of wires, recall
from the previous section:

$$\vec{w} = [ 2, 7, 4, 3, 12, 5, 35, 47 ]$$

For example we want to show that $a(\o^1) = b(\o^1)$, first we concatinate
the lists $\vec{a}, \vec{b}, \vec{c}$:

$$\vec{w} = [ 2, 7, 3, 12 ] \cat [ 2, 5, 4, 35 ] \cat [ 4, 35, 12, 42 ] = [ 2, 7, 3, 12, 2, 5, 4, 35, 4, 35, 12, 42 ]$$

Now, we wish to show, that for some permutation $\pi: \Fb^k \to \Fb^k$,
the list remains unchanged once permuted:

$$\vec{w} = \pi(\vec{w})$$

This permutation permutes the list according to what wires we wish to show are equal:

$$\vec{w} = [ 2, 7, 3, 12 ] \cat [ 2, 5, 4, 35 ] \cat [ 4, 35, 12, 42 ]$$

From the circuit in Figure <!-- TODO --> we gather that the following wires
must be equal:

$$a_1 = b_1, \quad c_1 = b_3, \quad c_3 = a_4, \quad c_2 = b_4$$

To highlight the values of $\vec{w}$ and $\pi(\vec{w})$, the specific values
have been subbed out for variables below:

$$
\begin{aligned}
  \vec{w}      &= [ a_1, a_2, a_3, a_4 ] \cat [ b_1, b_2, b_3, b_4 ] \cat [ c_1, c_2, c_3, c_4 ] \\
  \pi(\vec{w}) &= [ b_1, a_2, a_3, c_3 ] \cat [ a_1, b_2, c_1, c_2 ] \cat [ b_3, b_4, a_4, c_4 ]
\end{aligned}
$$

If the prover is honest, it's easy to see that these lists will match,
in fact, that's why we have to use variables in the above list, otherwise
the permutation _seems_ to do nothing. But as can also be seen above,
if the prover tries to cheat by violating $a_1 = b_1$ then the permuted
$\pi(\vec{w})$ will not be equal to the original $\vec{w}$. As in the above
section we can model the vectors as polynomials using FFT, such that $w(\o^1)
= w_1, w(\o^2) = w_2 \dots$. This reduces the problem to succinctly showing
equality of two polynomials for all elements in a set $H$, which is exactly
what the **Grand Product Argument** does.

#### Grand Product Argument

Given two polynomials $f(X), g(X)$ we want to check whether:

$$\forall s \in S : f(s) = g(s)$$

We can provide the challenge $\g$ and compare the products

$$\prod_{s \in S} f(s) + \g = \prod_{s \in S} g(s) + \g$$
