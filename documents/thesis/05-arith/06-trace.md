## Trace

With the necessary abstractions defined, we can now define the trace algorithm that transforms the abstract circuit; $\abst{f}$, into a trace table $T$. In this section, we will formalize the following:

$$
\begin{array}{rl}
(T, \sigma) = \mathrm{trace}(\vec{w},\abst{f},\avec{Y}) &
(T_{\text{pub}}, \sigma) = \mathrm{trace}_{\text{pub}}(\vec{x},\abst{f},\avec{Y})
\end{array}
$$

Recall that the abstract circuit is informally a directed acylic graph. Thus the trace algorithm amounts to a topologically sorted graph traversal. However to reason and make proofs about the algorithm, it would be more succinct if it is defined as a recursive function. As an implementation however, large graphs would cause stack overflows requiring an imperative approach instead. We bridge this gap by defining the algorithm within the framework of monotone functions [@cousot1979constructive] which can be expressed both recursively and imperatively.

\begin{definition}[Monotone Function]
A monotone function is a function $f: S \to S$ over a directed complete partial order $(S, \sqsubseteq, s_\bot)$ such that if $s_1,s_2: S$ and $s_1 \sqsubseteq s_2$ then $f(s_1) \sqsubseteq f(s_2)$, additionally $s_\bot$ is the least element.
\end{definition}

Note that the precision of our type anotation for monotone functions ends at $f: S \to S$. We will informally describe how the functions we construct are monotonic.

\motivdef informally we can think of applying the function as preserving the progress towards a final state. This is expressed formally by the definition of the Kleene fixed-point theorem below.

\begin{definition}[Ascending Kleene Chain]
Starting from the least element $s_\bot$, iteratively applying a monotone function $f$ produces an ascending Kleene chain.
\end{definition}
$$
s_\bot \sqsubseteq f(s_\bot) \sqsubseteq f(f(s_\bot)) \sqsubseteq \ldots f^n(s_\bot) \sqsubseteq \ldots
$$

\motivdef the chain progresses towards a fixed-point, where applying the function does not change the state anymore.

\begin{definition}[Least fixed-point]
Informally when the ascending Kleene chain saturates i.e. $f^n(s_\bot) = f^{n+1}(s_\bot)$ where $n$ is the minimum such number, we call $f^n(s_\bot)$ the least fixed-point of $f$.
\end{definition}
\newcommand{\lfp}{\text{lfp}}
$$
\lfp = \text{sup}(\set{f^n(s_\bot) \middle\vert n \in \Nb})
$$

- **Notation**: $\text{sup}$ computes the least upper bound from a subset of the directed complete partial order. Informally, you can think of it as the least element when the ascending Kleene chain saturates.

\begin{definition}[Kleene fixed-point theorem]
The computation of the least fixed-point can be expressed as Kleene iteration, where we repeatedly apply the monotone function until saturation.
\end{definition}
$$
\begin{array}{rl}
\lfp &: (S \to S) \to (S \to S \to \Bb) \to S \to S \\
\lfp(f, \text{eq}, s) &= \lfp'(f, \text{eq}, s, f(s)) \\
\lfp'(f, \text{eq}, s, s') &= \begin{cases}
s & \text{eq}(s, s') \\
\lfp'(f, \text{eq}, s', f(s')) & \otherwise
\end{cases} \\
\text{eq}(\_, s) &= \text{sat}(s)
\end{array}
$$

\begin{algorithm}[H]
\caption*{
  \textbf{imperative Kleene iteration:} $\lfp(f, \text{eq}, s)$
}
\begin{algorithmic}[1]
  \State $s' := f(s)$
  \State \textbf{while} $\neg\text{eq}(s,s')$ \textbf{do} $(s,s') := (s', f(s'))$
  \State \textbf{return} $s$
  \end{algorithmic}
\end{algorithm}

\motivdef the equality function $\text{eq}$ is necessary to determine saturation. Having it as an argument allows us to perform cheaper checks for saturation via $\text{sat}$ instead of comparing the entire element.

\begin{definition}[Continuation]
Our monotone function will be composed by other monotone functions conditionally, we call the composite functions continuations.
\end{definition}

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
$$
f(g,x) = \begin{cases}
  g(x) & \phi(x) \\
  x & \otherwise
\end{cases}
$$
Here $g$ is a continuation of $f$.
\end{tcolorbox}

Note that the monotone functions defined in this section are not hardcoded into the arithmetizer. It is simply a candidate for the \Plonk protocol feasible for IVC. Thus it is possible to define different monotone functions for different Plonk-ish protocols, or even different protocols entirely that uses an abstract circuit structure as an intermediate data structure.

We now define some definitions and notations that will be useful in defining the monotone functions used in trace for \Plonk.

\begin{definition}[Pop vector as stacks]
We model a stack data structure with a vector, and notate pop as follows.
\end{definition}
\newcommand{\pop}{\text{pop}}
$$
\begin{array}{rl}
\pop &: T^k \to T^{k'} \\
\pop(\vec{x}) &= \begin{cases}
() & \vec{x} = () \\
\vec{x}' & \vec{x} = \_ \cat \vec{x}' \\
\end{cases}
\end{array}
$$

\motivdef our directed complete partial order will have stacks as data, and this allows us to succinctly express popping from the stack.


\begin{notation}[Partial Map]
A partial map is a map that is not defined for every element in its domain. We denote undefined values with $\bot$. It is syntactic sugar for functions that returns an option type.
\end{notation}
$$
X \pto Y = X \to \Option(Y)
$$

- **Notation**:
  - The $\pto$ half arrow denotes a partial map instead of a standard function.
  - $\bot: X \pto Y$ denotes an empty partial map where $\bot(x) = \bot$ for all $x: X$.
- \opers
  - \subdefinition{update} $-[- \mapsto -]: (X \pto Y) \to X \to Y \to (X \pto Y)$ - extends or overwrites the partial map with a new entry.
  - \subdefinition{k-update} $-[- \mapsto -]: (X \pto Y) \to X^k \to Y^k \to (X \pto Y)$ - extends or overwrites the partial map with $k$ new entries.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
Let $f: X \pto Y$ be a partial map. Then we have:
$$
\begin{array}{rl}
f[x \mapsto y](x) = y &
(f[\vec{x} \mapsto \vec{y}])[\vec{x}] = \vec{y}
\end{array}
$$
\end{tcolorbox}

\begin{notation}[Sum Type]
A sum type is a type that can be one of multiple types. We denote it with $+$.
\end{notation}
$$
\begin{array}{cc}
a: A &
b: B \\
\inl(a): A + B &
\inr(b): A + B \\
\end{array}
$$

- **Notation**: If the context is clear, $\inl$ and $\inr$ can be elided.


### Resolve

Resolve is the first monotone function where its role is to compute the values of wires given the public input or witness values.

\begin{definition}[Value Map]
A value map is a partial map from wires to their values or a unit value in the case of the pulic variant that is unable to compute private wire values, but still needs to mark it as resolved.
\end{definition}
$$
\VMap = (\abst{w}: \Wire) \pto (W \circ \ty(\abst{w}) + ())
$$

\motivdef it is the data in the state used by resolve defined below.

\begin{definition}[State]
The state is the directed complete partial order for the full monotone function.
\end{definition}
$$
s: S
$$

\newcommand{\update}{\text{up}}
\newcommand{\rstack}{\text{rStack}}
- \projs
  - $\abst{f}(s): \AbsCirc$ - the abstract circuit from build.
  - $v(s): \VMap$ - the current value map of the state.
  - $\rstack(s): \Wire^k$ - the resolve stack, a stack of wires to resolve.
  - More projections are defined later.
- \opers
  - $\update_{\text{proj}}(s, x: X): S$ - it returns the same state $s$ with the exception that the projection 'proj' is replaced.
  - $\update'_{\text{proj}}(f: X \to X, s): S$ - similar to the above but it takes a function to update the projection.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
$\update_v(s, v'): S$ returns $s'$ such that $v(s') = v'$ and all other projections are the same as $s$

\vspace{1em}

$\update'_v(f, s): S$ returns $s'$ such that $v(s') = f(v(s))$ and all other projections are the same as $s$.
\end{tcolorbox}

\motivdef having the state projecting data allows us to extend it with more data as necessary for different monotone functions.

\begin{definition}[Get Unresolved]
The function takes a wire, and returns the wire if it is unresolved, otherwise it returns unit.
\end{definition}
\newcommand{\unresolved}{\text{unresolved}}
$$
\begin{array}{rl}
\unresolved &: S \to \Wire^k \to \Wire^{\leq k} \\
\unresolved(s, \avec{y}) &= \unresolved'(s)[\avec{y}] \\
\unresolved'(s,\abst{y}) &= \begin{cases}
() & v(s, \abst{y}) \neq \bot \\
\abst{y} & \otherwise
\end{cases}
\end{array}
$$

- **Notation**: The superscript $\leq k$ denotes that the output vector can be of length less than or equal to $k$.

\motivdef it is a helper function for resolve.

\begin{notation}[Unital Product]
The unit type when in a product is omissible.
\end{notation}
\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
$$
\begin{array}{rl}
X \times \Unit \times Y &= X \times Y \\
(x, (), y) &= (x, y)
\end{array}
$$
Thus, when we map unresolved over a vector of wires, we can coerce it into a product. By unital product, the units are omitted. We then coerce it back into a vector. Thus, filtering resolved wires from the original vector. Let $v(s) = \bot[\abst{x} \mapsto 1]$, then:
$$
\begin{array}{rl}
& \unresolved(s, (\abst{x}, \abst{y})) \\
&= \unresolved'(s)[(\abst{x}, \abst{y})] \\
&= (\unresolved'(s, \abst{x}), \unresolved'(s, \abst{y})) \\
&= ((),  \abst{y}) \\
&= (\abst{y})
\end{array}
$$
\end{tcolorbox}

\begin{definition}[Update value map]
The update function is an operation on state; given a wire, it finds the gate it is an output wire of. It then gets all the output wires of that gate and evaluates its wire value using the gate's canonical program. It then updates the value map with the newly computed wire values. It also handles the public variant by mapping unit to the output wires instead of computing their values.
\end{definition}
\newcommand{\updatev}{\text{updateVmap}}
$$
\begin{array}{rl}
\updatev &: S \to \Wire \to S \\
\updatev(s, \abst{y}) &= \maybe{\update'_v(-[\avec{y} \mapsto \vec{y}], s)}{\begin{array}{rl}
  \abst{f}(s) &\ni \gpair{g}{\abst{y}} \\
  \avec{y} &= \out(\abst{f}(s),g) \\
  \vec{y} &= (\text{compute} \circ \ty(g))(v[\gin(g)]) \\
\end{array}} \\
\text{compute}(\abst{g}, \vec{x}) &= \begin{cases}
() & () \in \vec{x} \\
\eval(\abst{g}, \vec{x}) & \otherwise
\end{cases}
\end{array}
$$

- **Notation**:
  - $(-[\avec{y} \mapsto \vec{y}])$ uses the placeholder notation to describe a function that takes a vmap and does k-update.
  - Recall $\update'_v$ is the operator that takes a function to update the projection of the state's value map.
  - By vectors coercable to products and unit for products, we have that $((),(), \ldots, ()) = ()$, thus $()$ is sufficient to map all $\avec{y}$ with units.

\begin{definition}[Resolve]
We can now define the resolve monotone function.
\end{definition}
\newcommand{\continue}{\text{continue}}
\newcommand{\resolve}{\text{resolve}}
$$
\begin{array}{rl}
\resolve &: (S \to S) \to S \to S \\
\resolve(\continue, s) &= \begin{cases}
\continue(s) & \rstack(s) = () \\
& \avec{y} = \abst{y} \cat \_ \\
\update'_{\rstack}(\pop, s) & v(s, \abst{y}) \neq \bot \\
& \gpair{g}{\abst{y}} \in \abst{f}(s) \\
\update'_{\rstack}(\avec{x} \cat -, s) & \avec{x} = \unresolved(s, \gin(g)) \neq () \\
\update'_{\rstack}(\pop) \circ \continue \circ \updatev(s, \abst{y}) & \otherwise
\end{cases} 
\end{array}
$$

Let's break down the cases and notation:

- The first case checks if the stack is empty, if so it simply calls the continuation.
- Before the second case, we have syntactic sugar for peeking the stack i.e. $\avec{y} = \abst{y} \cat \_$.
- The second case checks if the wire is already resolved, if so it pops the stack.
- Before the third case, we have syntactic sugar for querying the gate that outputs the wire i.e. $\gpair{g}{\abst{y}} \in \abst{f}(s)$.
- The third case checks if the gate has any unresolved input wires, if so it pushes them to the stack.
- The last case implies there are no unresolved input wires, yet the output wire is still unresolved. Thus we compute it via $\updatev$, call the continuation, then pop the stack.
- Notice that the continuation only gets called when the top of the stack is a newly resolved wire, or if the stack is empty. This is essential for the continuation that we will define later.

\begin{definition}[Resolve contributes to initial state]
Resolve contributes to the least element of $S$ with the following function.
\end{definition}
$$
\begin{array}{rl}
s_\bot^{\resolve} &: \Color^n \to (\avec{x}: \Wire^{k}) \to W \circ \ty[\avec{x}] \to \AbsCirc \to \Wire^{k'} \to S \\
s_\bot^{\resolve}(\vec{t}_{wit}, \avec{x}, \vec{x}, \abst{f}, \avec{Y})
&= \update'_v(-[\avec{x} \mapsto \vec{x}]) \\
&\circ \update'_v(-[\wire{-}{-}[(0..n) \odot \vec{t}_{wit}] \mapsto ()]) \\
&\circ \update_{\abst{f}}(\abst{f}) \\
&\circ \update'_{\rstack}(\avec{Y} \cat -)
(s_\bot)
\end{array}
$$
$$
\begin{array}{ccc}
v(s_\bot) = \bot &
\rstack(s_\bot) = () &
\abst{f}(s_\bot) = \emptyset
\end{array}
$$

Let's break down the definition:

- We start from the bottom update
- $\update'_{\rstack}(\avec{Y} \cat -)$ pushes the global output wires $\avec{Y}$ to the wire stack.
- $\update_{\abst{f}}(\abst{f})$ stores the abstract circuit from build.
- $\update'_v(-[\wire{-}{-}[(0..n) \odot \vec{t}_{wit}] \mapsto ()])$ maps all the witness wires to unit. This is for the public variant where we do not have access to the witness values, but we still want to mark them as resolved.
- $\update'_v(-[\avec{x} \mapsto \vec{x}])$ maps wires and their values as the initial value map. In the public case, these are public input wires. In the private case, these are witness wires from the input properad. This will replace the unit mapping from before.

\begin{definition}[Resolve Saturation]
The saturation function for resolve checks if the stack is empty.
\end{definition}
$$
\begin{array}{rl}
\text{sat}^{\resolve} &: S \to \Bb \\
\text{sat}^{\resolve}(s) &= |\rstack(s)| = 0
\end{array}
$$

Informally we can reason why resolve is monotone: The abstract circuit is finite. Thus the largest the stack can grow, is all the wires in the circuit. Additionally, the wires have been guaranteed to structurally type check by the definition of gate literals i.e. $\abst{g}(\avec{x})$, and resolvable by the definition of the canonical programs of the properads for every gate. Thus the stack will be eventually empty.

This works for the public version as well because of how $\updatev$ simply maps units if the wire values are not known. Behaviourally, the stack will update the exact same way in both public and private variants. This is integral to exhibit the structural integrity property of the circuit $(R,X,W)$ and $(R,X,\bot)$ as mentioned in the introduction of this section.

<!-- TODO example resolve? -->

### Gate Constraints

Gate is the next monotone function that is a continuation of resolve. Its role is to peek the stack for newly resolved wires, and then instantiate the pre-constraints into a sub-table to compose the trace table.

\begin{definition}[Marked base gate]
A marked base gate is a base gate that has been marked as a dependency by a relative gate in the abstract circuit.
\end{definition}
\newcommand{\isbase}{\text{isBase}}
$$
\isbase: \Ggt \to \Bb
$$

It is guaranteed that the base gate of the relative gate is resolved, since the relative wires / inputs to the relative gates are resolved when the output wire is. The relative gate is always declared after its base, because it depends on the wires of the base as input. Thus resolve will always resolve the base gate first. However, we want the base gate constraints to compose after the relative; not before.

This can be resolved by modifying the build's put algorithm. When adding a relative gate to the abstract circuit. It needs to remove entries of its base gate, mark the base gate as a dependency, and re-add it to the abstract circuit. We leave this modification informal, and assume $\isbase$ will determine if the gate is marked. This is then used in the monotone function, to skip adding the base gate's constraints to the trace table. The relative gate instead will be responsible for adding the base gate's constraints after its own.

\begin{definition}[State projections for gate]
We define more projections for the state to be used in gate.
\end{definition}
$$
s: S
$$

\newcommand{\gqueue}{\text{gQueue}}
- \projs
  - $\gqueue(s): \Ggt^k$ - the gate queue, a queue of gates to instantiate pre-constraints from.
  - $\phi(s): \Nb$ - the current phase of the trace table being constructed.
  - $T(s): \TraceTable$ - the current trace table being constructed.

\motivdef is that the gate queue allows us to push multiple gates given one wire from the wire stack. This is for the case of wires belonging to relative gates. Such that we can tabulate the dependencies in order. The trace table projection is the result we want. The phase however determines the kinds of gates whose sub-tables we are populating. They are defined as follows:

\newcommand{\Pubinp}{\text{PI}}
\begin{tabularx}{\textwidth}{@{} r|Y Y Y Y Y @{}}
\toprule
\multirow{3}{*}{phase} & Basic & Relative  & Asserts  & PublicInput & \plookup  Tables \\
\cline{2-6}
& $b(\abst{g})=0$ & $b(\abst{g})>0$ & $m(\abst(g)) = 0$ & $\ty(g)=\Pubinp^t_i$ & $\ty(g)=\text{Tbl}^t_j$
\\\cline{2-6} 
 & $\phi=0$ & $\phi=0$ & $\phi=1$ & $\phi=2$ & $\phi=3$
\\\hline\\
placement &
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, minimum height=0.75cm, anchor=north] (b3) at (b2.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$\base_g$};
\node[draw, minimum width=1.5cm, minimum height=0.75cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
& 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b4) at (b3.south) {$g_k$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50] (b1) at (0,0) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$g_k$};
\node[draw, minimum width=1.5cm,minimum height=0.75cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=2.25cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern=north west lines, pattern color=gray!50] (b2) at (b1.north east) {$g_1$};
\node[draw, minimum width=0.6cm, minimum height=0.73cm, anchor=north west, pattern=north east lines, pattern color=gray!50] (b3) at (b2.south west) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern=north west lines, pattern color=gray!50] (b4) at (b3.south west) {$g_k$};
\end{tikzpicture} \\
\\\toprule
\end{tabularx}

- **Notation**:
  - Asserts are gates with no output wires
  - As mentioned at the end of resolve, public input wires have a specialized properad (defined later). Its constraints are conventionally placed at the top of the table. 
  - Tables for lookup arguments are defined as a properad with a singleton gate with no wires at all. They strictly define the values for the compressed table column. We will not define it here. This is just to illustrate its feasibility.
  - Every other gate thats not relative, is a basic gate.

\begin{definition}[Public input Properad]
Informally, you can think of the input wire as the computed value from the witness. In the private variant, the output wire represents the same value as the input wire. However in the public variant, since the input wire value is unknown; mapped to unit, the output wire is from the public input $\vec{x}$. This is part of the $(\vec{x}, \vec{w}) \in R_f$ check.
\end{definition}


\begin{center}
\begin{tabular}{ c c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[minimum width=2cm, minimum height=1.5cm] (tab) {
\begin{tabular}{|c|c|c|c|c|c|}
\hline
\multicolumn{6}{|c|}{$\Pubinp_i$} \\
\hline
$n$ & $m$ & $\pin$ & $\pout$ & $\eval()$ & $\term$ \\
\hline
$1$ & $1$ & ${t_{pub}}_i$ & ${t_{pub}}_i$ & $x_i$ OR $w$ & $F^{\plonkm}_{GC}$ \\
\hline
\end{tabular}
};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{inp}{(0,0)}{$\ \ \abst{w}\ \ $}{$\Pubinp_i$}{1}
\draw[-, thick] ($(inp-in-1)+(0,0.4)$) -- (inp-in-1);
\draw[->,thick] (inp-out-1) -- ($(inp-out-1)+(0,-0.4)$);
\node[anchor=north east] at (inp-out-1) {$\abst{x_{i}}$};
\end{tikzpicture}
&
\begin{tabular}{|c|c|c|c|}
\hline
\multicolumn{4}{|c|}{$\ctrn(\Pubinp_i)$} \\
\hline
\multicolumn{3}{|c|}{${t_{wit}}_i$} & $\cdots$ \\
\hline
$A$ & $Q_l$ & $PI$ & $\cdots$ \\
\hline
$w$ & $1$ & $-x_i$ OR $-w$ \\
\cline{1-3}
\end{tabular}
\end{tabular}
\end{center}

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
If $X$ were a public input constraint from a public variant trace table, we can visualize that the value $w$ from the witness computed value will be constrained as follows:
$$
\begin{array}{rl}
F_{GC}^{\plonkm}: \Eqn &= A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI \\
F_{GC}^{\plonkm}(X) &= w - x_i \stackrel{?}{=} 0
\end{array}
$$

This is the motivation for the public input properad.
\end{tcolorbox}

\begin{definition}[Public columns]
Public columns are columns that are not private.
\end{definition}
$$
\begin{array}{rl}
\Column^{pub}&: \pset{\Column} = \set{c \in \Column | \neg \pcol(c)} \\
F^{pub}(T) &: (t: \Color) \to (c: \Column^{pub}) \to T \\
\end{array}
$$

\motivdef the type safety in the case of the public variant can be guaranteed.

\begin{definition}[Resolve cell]
This function takes a cell and state, uses the value map to resolve the cell to its value. It assumes that if the variant is public, it is not resolving a private column's cell.
\end{definition}
\newcommand{\resolvecell}{\text{resolveCell}}
$$
\begin{array}{rl}
\resolvecell&: S \to (g: \Ggt) \to F(\Cell(\ty(g), t,c) \to X(t,c) \to W(t)) \\
\resolvecell(s, g, t, c, \aavec{w}, r, x) &= r(x) \circ v(s)[\text{wires}(\abst{f}(s), g) @ -][\aavec{w}]\\
\\
\resolvecell_{pub}&: S \to (g: \Ggt) \to F^{pub}(\Cell(\ty(g), t,c) \to X(t,c) \to W(t)) \\
\resolvecell_{pub} = \resolvecell
\end{array}
$$

Let's break down the definition:

- The arguments $t,c$ come from $F(\ldots)$; the index map indices.
- $\aavec{w}, r$ is the cell itself, recall its type $\Cell(\abst{g},t,c) = (\aavec{w}: \CWire(c, \abst{g})^k) \times R(\abst{g}, \aavec{w},t,c)$
- $x$ is the thunk argument, recall the resolver type is $R(\abst{g}, \aavec{w},t,c) = X(t,c) \to W [\vec{t}(\abst{g}, \aavec{w})] \to W(t)$
- Thus we partial apply $x$ i.e. $r(x)$, now we are left ith the values for the cell wires.
- We use $\text{wires}(\abst{f}(s), g)$ to get the wires of the gate $g$ represented by the cell wires $\aavec{w}$ via the vector index function $- @ -$; transitioning from $\bar{w}$ to $\abst{w}$.
- With the wires at hand, we can finally query the value map $v(s)$ to get the concrete values of the wire; transitioning from $\abst{w}$ to $w$.

\motivdef this function is used to concretize pre-constraints into sub-tables ready to be composed into the trace table.

\begin{definition}[Enqueue gates to state]
This function manages the logic to determine the gates to be pushed to the stack of gates in the state.
\end{definition}
\newcommand{\enqueg}{\text{enqueueGates}}
$$
\begin{array}{rl}
\enqueg &: S \to \Wire \to S \\
\enqueg(s, \abst{y}) &= \maybe{\enqueg'(g, s)}{\abst{f}(s) \ni \gpair{g}{\abst{y}}}\\
\enqueg'(g,s) &= \begin{cases}
s & \isbase(g) \\
& s' = \update'_{\gqueue}(- \cat g, s) \\
\enqueg'(\base(\abst{f}, g), s') & b \circ \ty(g) > 0 \\
s' & \otherwise
\end{cases}
\end{array}
$$

Let's break down the definition:

- The first case handles if the gate is marked as a base gate of an existing relative gate, we sill skip it as the relative gate will manage it.
- Before the second case is syntactic sugar for the state with the gate enqueued.
- The second case checks if the gate is relative, if so it recursively enqueues the base gate in case the base gate is also relative, i.e. a dependent chain of relative gates.
- The third case is the base case, we just return the state after we enqueued the gate.

\motivdef this is a queue and not a stack. Thus, the first gate will be the first to be dequeued. i.e. a relative gate will appear before its base gate. We will use this function to process the wire stack from the resolve monotone function.

TODO 

- remember asserts check ismarked, if true, then dont add it again
- gate batch per phase
- actual gate monotone function
- initial state
- sat
- DONE

\newcommand{\WireType}{\text{WireType}}

$$
\begin{array}{rl}
\begin{array}{rl}
A \cat B &= A \sqcup_{\lambda \_,\vec{a}, \vec{b}. \vec{a} \cat \vec{b}} B \\
\TraceTable &= \IndexMap(X, \lambda t,\_. W(t)^k) \\
\text{GState}^{k,k'} &= \TraceTable \times \pset{\Ggt} \times \Ggt^{k'} \\
&\times \Nb \times \RState^k \\
\vec{G}^{\abst{f}} &: \Ggt^j \times \Ggt^k \times \Ggt^l \\
\vec{G}^{\abst{f}} &= \left(\begin{array}{l}
  \left[g \middle\vert \gpair{g}{\abst{y}} \in \abst{f} \land
  \begin{array}{l}
    \exists i,t. \ty(g) = \Input^t_i \\
    \lor \abst{y} = \bot \land \ty(g) \notin \cdots
  \end{array}\right]\\
  \left[g \middle\vert \gpair{g}{\_} \in \abst{f} \land \exists i,t. \ty(g) = \text{PI}^t_i\right] \\
  \left[g \middle\vert \gpair{g}{\_} \in \abst{f} \land \exists j,t. \ty(g) = \text{Tbl}^t_j\right]
\end{array}\right) \\
\\
\Downarrow &: \AbsCirc \to \VMap \to F(\Cell(t,s)^k \to W(t)^k) \\
\Downarrow^{\abst{f}}_v(\_,\vec{r}) &= (\lambda (x, \avec{w}, f).f(x,v[\text{wires}^{\abst{f}}_g(\avec{w})]))[\vec{r}] \\
\\
\Downarrow &: \AbsCirc \to \Ggt \to \Ggt^k \\
\Downarrow^{\abst{f}}(g) &= \begin{cases}
\Downarrow^{\abst{f}}(g') \cat g & g' = \base^{\abst{f}}_g \neq \bot \\
(g)
\end{cases}
\end{array} &
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \\
&\to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \underset{G}{\curvearrowright} \circ f \circ^\uparrow \lambda (C, \Omega, \vec{g}, \phi, v). \\
&\begin{cases}
& \vec{g} = \_ \cat g \land \Omega' = \Omega \cup \set{g} \\
& \phi \leq 1 \Rightarrow C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn_g] \\
& \phi = 2 \Rightarrow C' = \Downarrow^{\abst{f}}_v[\ctrn_g] \cat C \\
(C',\Omega', \vec{g},\phi,v) & \phi = 3 \Rightarrow C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn'_g] \\
(C, \Omega, (), \phi, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, \phi, v, \avec{y}). \\
&\begin{cases}
& \phi = 0 \land \avec{y} = \abst{y} \cat \_ \\
(\Downarrow^{\abst{f}}(g) \cat \vec{g}, 0, v, \avec{y})
& \gpair{g}{\abst{y}} \in \abst{f} \land g \notin \Omega \\
(\vec{G}^{\abst{f}}_{\phi +1}, \phi + 1, v, ())
& \phi < 3 \land |\avec{y}| = |\vec{g}| = 0 \\
((), 4, v, ()) & \phi = 3 \land |\avec{y}| = |\vec{g}| = 0 \\
(\vec{g}, \phi, v, \avec{y})
& \otherwise
\end{cases}
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\underset{G}{\curvearrowright} &= \lift(\curvearrowright : \Ggt^k \to \Ggt^{k'})\\
\curvearrowright (\vec{x}) &= \begin{cases}
() & \vec{x} = () \\
\vec{x}' & \vec{x} = \vec{x}' \cat \_ \\
\end{cases}
\end{array} &
\begin{array}{rl}
\dagger_G &: \text{GState} \to \Bb \\
\dagger_G &= \lambda(\_, \vec{g}, b, \_). |\vec{g}| = 0 \land b = 4
\end{array} &
\begin{array}{rl}
\iota_G &: \text{RState} \to \text{GState} \\
\iota_G(s) &= (\lambda \_.\bot[s \mapsto ()], \emptyset, (), 0, s)
\end{array}
\end{array}
$$

### Copy Constraints

$\Downarrow_C$ From $\ctrn_g$, we populate the *loop*; a vector modelling an equivalence class of *coordinates*; copy constraint column and row number, modulo wire, for every $g$ in the queue. After computing the loop of the full circuit, we compute the position permutation $\sigma$.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{Coord} &= \text{CC} \times \Nb \\
\text{CLoop} &= (\abst{w}: \Wire) \pto \text{Coord}^k \\
\text{CMap} &= \text{Coord} \to \text{Coord} \\
\text{CState} &= (\text{CMap} + \text{CLoop}) \times \text{GState}^{k,k'} \\
\\
\sqcup &: \text{CLoop} \to \text{CLoop} \to \text{CLoop} \\
L_1 \sqcup L_2 &= \begin{cases}
& \exists \abst{w}. L_2(\abst{w}) \neq \bot \\
& l = L_1(\abst{w})?() \cat L_2(\abst{w}) \\
L & L = L_1[\abst{w} \mapsto l] \sqcup L_2[\abst{w} \mapsto \bot] \\
L_1 & \otherwise
\end{cases} \\
\\
\text{perm} &: \text{CLoop} \to \text{CMap} \\
\text{perm}(L) &= \lambda x. \begin{cases}
y & y = \text{perm}'(L)(x) \neq \bot \\
x & \otherwise
\end{cases} \\
\text{perm}'(L) &= \begin{cases}
\bot & l = \bot \\
& \exists \abst{w}. \vec{s} = L(\abst{w}) \\
& \sigma = \text{perm}'(L[\abst{w} \mapsto \bot]) \\
\sigma[\vec{s} \mapsto \vec{s}'] & s'_1 = s_{|\vec{s}|} \land s'_{i>1} = s_{i-1}
\end{cases}
\end{array} &
\begin{array}{c}
\begin{array}{rl}
\Downarrow &: \text{CLoop} \to \PreTable_g \to \Wire^{n_g +m_g} \to \Nb \to \text{CLoop} \\
\Downarrow_{L_\bot} &= \lambda((t, s, x, \vec{c}), \avec{w}, i). \\
&\begin{cases}
L_\bot & \vec{c} = () \\
& \vec{c} = c \cat \vec{c}' \land j = \text{cw}(c) \\
& L = \text{loop}(t,s,x,\vec{c}',\avec{w},i+1) \\
L[\abst{w}_j \mapsto \vec{s}]
& \vec{s} = L(\abst{w}_j)?() \cat (s,i) \land s \in \text{CC} \\
L & \otherwise
\end{cases} \\
\\
\Downarrow_C &: \AbsCirc \to \text{CState} \to \text{CState} \\
\Downarrow_C^{\abst{f}} &= \lambda (r, C, \Omega, \vec{g}). \\
&\begin{cases}
& r = \inr(L) \\
& \vec{g} = \vec{g}' \cat g \\
& \avec{w} = \gin(g) \cat \out(\abst{f},g) \\
(L', C, \Omega, \vec{g})  
& L' = \Downarrow_L(t,s,\ctrn^t_g(s),\avec{w}, |C^t(s)|) \\
(\sigma, C, \Omega, ()) & \sigma = \text{perm}(L) \\
(r,C,\Omega, ()) & \otherwise
\end{cases}
\end{array} \\
\begin{array}{cc}
\begin{array}{rl}
\dagger_C &: \text{CState} \to \Bb \\
\dagger_C &= \lambda (r, \_). r = \inl(\_)
\end{array} &
\begin{array}{rl}
\iota_C &: \text{GState} \to \text{CState} \\
\iota_C(s) &= (\inr(\bot), s)
\end{array}
\end{array}
\end{array}
\end{array}
$$

### Full Plonk Trace

We conclude the full trace definition as follows:

$$
\begin{array}{ccc}
\begin{array}{l}
\text{TraceResult} \\
= \text{CMap} \times \TraceTable
\end{array}&
\begin{array}{rl}
\text{res} &: \text{CState} \to \text{TraceResult} \\
\text{res} &= \lambda (\sigma, C, \_). (\sigma, C) \\
\end{array}
&
\begin{array}{rl}
\text{eq} &: \text{CState} \times \text{CState} \to \Bb \\
\text{eq}(\_, x) &= \bigwedge\limits_{\dagger \in \set{\dagger_C, \dagger_G, \dagger_R}} \maybe{\dagger(s)}{x=(\_,s)} \\
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\Downarrow &: \AbsCirc \to \text{CState} \to \text{CState} \\
\Downarrow^{\abst{f}} &= \Downarrow^{\abst{f}}_C \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} \stackrel{\to}{\circ} \Downarrow_R^{\abst{f}} \\
\end{array}
&
\begin{array}{rl}
\iota &: \text{RState} \to \text{CState} \\
\iota &= \iota_C \circ \iota_G \\
\end{array}
&
\begin{array}{rl}
\text{trace} &: W[\tin{}] \to \AbsCirc \to \Wire^m \to \text{TraceResult} \\
\text{trace} &= \lambda(\vec{x}, \abst{f}, \avec{Y}). \text{res} \circ \text{sup}(\Downarrow^{\abst{f}},\text{eq},\iota(s_0),\iota(s^{\avec{Y}}_{\vec{x}}))
\end{array}
\end{array}
$$

### Public variant

The public variant for arithmetization only differs in trace. In $\Downarrow_R$, the $\avec{x}: W[\tin{}]$ for input gates is instead for public input gates. The vmap values are bools, that marks the wires having been resolved. This will lead to the same wire stack as the original $\Downarrow_R$, consequently trace table layout. $\Downarrow_G$ then will omit columns $c \in \text{priv}$ in $\ctrn_g$. Thus the cells that remain do not need the values to reduce, i.e. all the cells are constants. $\Downarrow_C$ remains the same. Resulting in a trace that differs by its trace table not having private columns.

$$
\begin{array}{rl}
(R,x,w) 
&= \Arithmetize_{\text{public}}(f,\vec{x}') \\ 
&= \mathrm{interpolate} \circ \mathrm{trace}_{\text{public}}(\vec{x}') \circ \mathrm{build}(f)
\end{array}
$$
