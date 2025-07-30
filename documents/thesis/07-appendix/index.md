
\appendix

# Notation

\begin{longtable*}{@{} c l @{}}
\toprule
\textbf{Types and Terms} & \textbf{Description} \\
\hline
$\Uni$ & The type of all types \\
\hline
$\Nb$ & The type of natural numbers \\
\hline
$\Fb_q$ & The finite field of order $q$ \\
\hline
$T^n$ & The vector of type $T$ of length $n$ \\
$()$ & Vector of length $0$ \\
$\vec{x}$ & A vector term \\
$(x_1, x_2, \cdots, x_n)$ & A vector term literal \\
\hline
$T \times U$ & The product / tuple of $T$ and $U$ \\
\hline
$\Unit$ & The unit type; identity for product type \\
$()$ & The unit value \\
\hline
$X \to Y$ & The function type from $X$ to $Y$ \\
$\lambda x. f(x)$ & The function term / lambda abstraction \\
\hline
$X \pto Y$ & The partial function type from $X$ to $Y$ \\
$\bot$ & The empty partial function \\
$f[x \mapsto y]$ & The partial function that maps $x$ to $y$ \\
\hline
$T + U$ & The disjoint union / sum type of $T$ and $U$ \\
$\text{inl}(t)$ & The left injection of $t$ into the disjoint union \\
$\text{inr}(u)$ & The right injection of $u$ into the disjoint union \\
 & The injection constructor can be omitted if the context is clear \\
\hline
\end{longtable*}


\begin{longtable*}{@{} c l l @{}}
\toprule
\textbf{Function} & \textbf{Definition} & \textbf{Description} \\
\hline
$[n]$ & $\set{1,2,\ldots,n-1}$ & Set of naturals from $1$ to $n-1$ \\
$[n..m]$ & $\set{n,n+1,\ldots,m-1}$ & Set of naturals with lower bound $n$ to $m-1$ \\
\hline
$y \cat \vec{x}$ & $(y, x_1, x_2, \cdots, x_n)$ & Prepend $y$ to vector $\vec{x}$ \\
$\vec{x} \cat y$ & $(x_1, x_2, \cdots, x_n, y)$ & Append $y$ to vector $\vec{x}$ \\
\hline
$\text{last}(\vec{v})$ & $\maybe{v}{\vec{v} = \_ \cat (v)}$ & last element \\
\hline
$\maybe{x}{\phi(x)}$ & $\begin{cases} x & \phi(x) \\ \bot & \otherwise \end{cases}$ & Maybe: $x$ if $\phi(x)$, else $\bot$ \\
\hline
$x ? y$ & $\begin{cases} x & x \neq \bot \\ y & \otherwise \end{cases}$ & $x$ unless it is $\bot$ then $y$ \\
\hline
$(s..t)$ & $\begin{cases} () & t \leq s \\ s \cat (s+1 .. t) \end{cases}$ & Vector of naturals from $s$ to $t-1$ \\
\hline
$\vec{x} \cat \vec{y}$ & $\begin{cases} \vec{y} & \vec{x} = () \\ \vec{x}' \cat (x \cat \vec{y}) & \vec{x} = \vec{x'} \cat x \end{cases}$ & Concatenate vectors $\vec{x}$ and $\vec{y}$ \\
$X \cat \vec{x}$ &  & Concatenate set $X$ (any order) to vector $\vec{x}$ \\
\hline
$f[\vec{x}]$ & $(f(x_1), f(x_2), \ldots, f(x_n))$ & Map function $f$ over vector $\vec{x}$ \\
\hline
$\vec{x} \setminus X$ &  & Remove all elements in set $X$ from vector $\vec{x}$ \\
\hline
$\vec{x} \odot \vec{y}$ & $((x_1,y_1),\cdots,(x_n,y_n))$ & Zip vectors $\vec{x}$ and $\vec{y}$ (Hadamard product) \\
\hline
$\min(X)$ &  & Minimum of set $X$ (with total ordering) \\
\hline
$f[\vec{x} \mapsto \vec{y}]$ & $\begin{cases} & \vec{x} = x \cat \vec{x}' \\ f[x \mapsto y][\vec{x}' \mapsto \vec{y}'] & \vec{y} = y \cat \vec{y}' \\ f & \otherwise \end{cases}$ & Append vector mapping to partial function $f$ \\
\hline
\end{longtable*}

\begin{longtable*}{@{} l l l @{}}
\toprule
\textbf{Identity} & \textbf{Example} & \textbf{Description} \\
\hline
$T \times U \times V = (T \times U) \times V = T \times (U \times V)$ & $(a, b, c) = ((a, b), c) = (a, (b, c))$ & Associative product types \\
\hline
$\Unit \times T = T \times \Unit = T$ & $((), t) = (t, ()) = (t)$ & Product has unit \\
\hline
$X \to Y \to Z = (X \to Y) \to Z = X \to (Y \to Z)$ & $f(x,y)=f(\lambda x.y)=f(x)(y)$ & Associative function type \\
\hline
$X \to Y \to Z = (X \times Y) \to Z$ & $f(x,y)=f(x)(y)=f((x, y))$ & Currying \\
\hline
$\Unit \to X = X$ & $f(()) = x \leftrightarrow f = x$ & Function has unit \\
\hline
$X \to \Option(Y) = X \pto Y$ & $[x \mapsto y] = \lambda x. \maybe{y}{x \mathrm{\ is\ defined}}$ & Partial are functions to options \\
\hline
\end{longtable*}

**unused argument(s)** - we use underscores to denote that the argument is not used e.g. $\lambda \_. \cdots$, by associative product and currying, one underscore can mean multiple unused arguments.

**quantifier seperator** - $\forall x. P(x)$ instead of $\forall x: P(x)$ to disambiguate typing judgements from quantifier separators

**flattened case notation** - conditions are propagated to conditions below if they don't contradict, if a case has no term, the next termed case must satisfy it, but subsequent cases need not
$$
\begin{array}{rl}
\begin{cases}
a & \phi_1 \\
 & \phi_2 \\
b & \phi_3 \\
c & \phi_4 \\
\vdots
\end{cases} &=
\begin{cases}
a & \phi_1 \\
b & \phi_2 \land \phi_3 \\
c & \phi_2 \lor \phi_4 \\
\vdots
\end{cases}
\end{array}
$$

**abstract preprocess objects** - $\abst{x}$ is an abstract of a thing, e.g. $\abst{f}$ is an abstract circuit, $\abst{y}$ is an abstract value / wire


