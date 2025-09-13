## Abstractions

We now define the rest of the abstractions building up to the specification of the single source of truth for arithmetization which is used in $\text{trace}$.

### Pre-Constraints and Trace Table

\begin{definition}[Column]
A set of unique identifiers for the columns of the trace table.
\end{definition}
$$
c: \Column
$$
\newcommand{\pcol}{\text{priv}}
\newcommand{\cccol}{\text{copy}}
\newcommand{\rcol}{\text{rel}}

- **Notation**: $\Column = \set{A,B,C,Q_l,Q_r,Q_o,Q_m,Q_c,PI, \ldots}$
- \projs
  - $\pcol(c): \Bb$ - determines column of $X$ or $W$ of the circuit.
  - $\cccol(c): \Bb$ - is $c$ a copy constraint relevant column.
  - $\rcol(c): \Bb$ - is $c$ a relative wire relevant column (will be motivated when defining relative wires).
  - $X(t: \Color,c): \Uni$ - the column thunk argument type (will be motivated when defining thunk and index maps below).
  - $\text{default}(t,c,x: X(t,c)): W(t)$ - the column default value (will be motivated when defining pre-constraints).

\motivdef it provides a modular and user extensible way to define the structure of the trace table.

\begin{notation}[Unit Type]
A unit type is a type with a single value, denoted as $()$.
\end{notation}
$$
() : \Unit
$$

- \subdefinition{unit for functions} $1 \to T = T$ e.g. $f() = x$ then $f = x$
- \subdefinition{unit vector} $T^0 = 1$ i.e. $(): T^0 = () : \Unit$
- \subdefinition{omissible argument} unit arguments can be omitted.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
Example of how we can derive an argument omissible.
$$
\begin{array}{lll}
\mathrm{type} & \mathrm{example} \\
X \times 1 \to T & f(x,())  \\
= X \to 1 \to T & = f(x)() & \text{by\ currying} \\
= X \to T & = f(x) & \text{by\ unit\ for\ functions}
\end{array}
$$
\end{tcolorbox}

\motivnot it is used as an identity / unital / nullary element.

\begin{definition}[Index Map]
A data structure that maps columns to thunks; functions of arbitrary type.
\end{definition}
$$
\begin{array}{rl}
F(T) &= \Color \to \Column \to T \\
\IndexMap(X: F(\Uni), Y: F(\Uni)) &= F(X(t,c) \to Y(t,c))
\end{array}
$$

- \subdefinition{thunk} Informally, think of a thunk as a function $f: X \to Y$
  - $X$ is the thunk argument type; $X = \Unit$ if the value is not a thunk.
  - $Y$ is the value type; $Y= \Option(T)$ if the index map is partially populated.
  - Thus, index maps hold such $f$ where $X,Y$ vary per color and column. i.e. $f: X(t,c) \to Y(t,c)$
  - To bind $t$ and $c$ to the index map indices we use $F$.
  - Thus we get $F(X(t,c) \to Y(t,c))$ the type for index maps.
- **Notation**:
  - If $t,c$ appears free in $F$, then it is bound
    - e.g. $F(T(t,c)) = (t: \Color) \to (c: \Column) \to T(t,c)$.
  - If $A: \IndexMap(X,Y)$ then
    - $A^t_x(c) = A(t,c,x)$ for thunks
    - $A^t(c) = A(t,c,())$ for non thunks
- \opers
  - \subdefinition{map} $-[-]: F(X(t,c) \to Y_1(t,c) \to Y_2(t,c)) \to \IndexMap(X, Y_1) \to \IndexMap(X, Y_2)$
    - $f[A]^t_x(c) = f(t,c,x,A^t_x(c))$
  - \subdefinition{join} $- \sqcup_{-} -: \IndexMap(X,Y_1) \to F(X(t,c) \to Y_1(t,c) \to Y_2(t,c) \to Y_3(t,c)) \to \IndexMap(X,Y_2) \to \IndexMap(X,Y_3)$
    - $(A \sqcup_f B)^t_x(c) = f(t,c,x,A^t_x(c),B^t_x(c))$
  - \subdefinition{concat} $A \cat B = A \sqcup_{f} B$ where:
    - $A: \IndexMap(X, T^k), B: \IndexMap(X,T^{k'})$
    - $f(\_,\_, \_, \vec{a},\vec{b}) = \vec{a} \cat \vec{b}$

\motivdef it is a succinct way to store and compose values that depend on color, column, and an argument of arbitrary type, e.g. managing multiple wire types for $\plookup$  columns in trace table, and data structure for values for equations.

\begin{definition}[Equation]
An equation is a grammar that expresses a polynomial structure; a tree where leaf vertices are columns or coefficient scalars and intermediate vertices are operations of the grammar.
\end{definition}
$$
\begin{array}{ccc}
& E: Eqn  \\
\begin{array}{rl}
\langle Eqn \rangle &::= - Eqn1 \\
& |\ \mathtt{Scalar}\ \times \ Eqn1 \\
& |\ \mathtt{(}\ Eqn1 \mathtt{)} \\
& |\ \mathtt{Column} \\
\end{array} &
\begin{array}{rl}
\langle Eqn1 \rangle &::= Eqn2\ Eqn1' \\
\langle Eqn1'\rangle &::= +\ Eqn1 \\
& |\ -\ Eqn1\ | \epsilon \\
\end{array} &
\begin{array}{rl}
\langle Eqn2 \rangle &::= Eqn\ Eqn2' \\
\langle Eqn2'\rangle &::= \times Eqn2\ |\ \epsilon
\end{array}
\end{array}
$$
$$
\text{foldEqn}: (-: T \to T) \to (+: T \to T \to T) \to (\times: T \to T \to T) \to (\times_\Fb: \Fb \to T \to T) \to \Eqn \to (\Column \to T) \to T
$$

- **Notation**: If $X: \Column \to T$ then $E(X): T = \text{foldEqn}(\ldots, E, X)$ 
  - when every operation of the grammar is specified for type $T$, we can evaluate the equation.
  - curve points do not have multiplication defined, thus equations involving them cant be evaluated.

\motivdef it is the single source of truth for an equational definitions that can vary over operand types: scalars, polynomials, curve points, wires and state via build. Examples of equations are gate constraint polynomials, grand product polynomials, quotient polynomial, $\plookup$  compression equation, etc. When implementing in a programming language such as rust, it is possible to use type variables / generics to define a function over $T$, without having an explicit syntax tree data structure of the $Eqn$'s grammar.

\begin{definition}[Cell Wire]
Cell wires are natural numbers that represent the wires of a gate in a properad. Naively, an abstract wire. It does exclude some wires which will be defined later.
\end{definition}
\newcommand{\CWire}{\text{CellWire}}
\newcommand{\aavec}[1]{\bar{\vec{#1}}}
$$
\begin{array}{rl}
\CWire(c, \abst{g})&: \pset{\Nb} = \begin{cases}
  [n(\abst{g}) + m(\abst{g}) + 1] \setminus \ldots & \pcol(c) \lor c = \text{PI} \\
  \emptyset & \otherwise
\end{cases} \\
\bar{w} &: \CWire(c, \abst{g})
\end{array}
$$

- **Notation**:
  - $\bar{w}$ the bar denotes it is an abstract wire $\abst{w}$ thats an abstract value $w$.
  - If $n(\abst{g})=2, m(\abst{g})=1$ and $\bar{w} \in \set{1,2}$ then $\abst{w} \in \gin(g)$ else if $\bar{w} = 3$ then $\gpair{g}{\abst{w}} \in \abst{f}$.

A public column (with the exception of the public input column) should not be able to reference wires, as they are private values, thus the second case of the emptyset in the definition.

\motivdef it allows the definition of constraints to be declarative. We will later define pre-constraints as templates for constraints that are projected from properads. Recall that properads do not have wires, but the gates it instantiate does. However, we need a way to refer to them to define the pre-constraints. This is done with cell wires.

\begin{notation}[Vector index function]
A function that indexes a vector.
\end{notation}
$$
\begin{array}{rl}
- @ -&: T^k \to [k+1] \to T \\
\vec{x} @ i &= x_i
\end{array}
$$

$[k+1]: \pset{\Nb}$ guarantees that $i$ is a valid index for the vector.

\motivnot it allows us to index vectors over a higher order function such as mapping over a vector.

\begin{definition}[More Properad Projections]\end{definition}
$$
\abst{g}: \Prpd
$$

- \projs
  - $\vec{t}(\abst{g}) = \pin(\abst{g}) \cat \pout(\abst{g})$ - the full profile of the properad; input and output wire colors.
  - $\vec{t}(\abst{g}, \aavec{w}) = (\vec{t}(\abst{g}) @ -)[\aavec{w}]$ - the profile of a vector of cell wires.
  - $\term(\abst{g}): \Eqn$ - the equation term that the properad contributes to the gate constraint polynomial.
  - We continue to define more projections as their own definitions to eventually define cells and pre-constraints

\motivdef it allows us to define the single source of truth for values in the trace table.

\begin{definition}[Cell Resolvers]
Functions that reduce a cell to a value.
\end{definition}
$$
R(\abst{g}, \aavec{w}) = F(X(t,c) \to W [\vec{t}(\abst{g}, \aavec{w})] \to W(t))
$$

A resolver takes in a color $t$, column $c$, thunk argument $x$ and the values the cell wires $\aavec{w}$ correspond to, and computes a value of type $W(t)$; the resolved value.

The specific construction of cells below will motivate the use of cell resolvers.

\begin{definition}[Cell]
Cells represent an abstract value in an index map modelling the trace table. It contains the cell wires that it depends on, and a resolver that computes the value of the cell. 
\end{definition}
$$
\begin{array}{rl}
\Cell(\abst{g}) &= F((\aavec{w}: \CWire(c, \abst{g})^k) \times R(\abst{g}, \aavec{w},t,c)) \\
\boxdot &: \Cell(\abst{g})
\end{array}
$$

- **Notation**: $\boxdot$ a boxed symbol denotes a cell.

The specific construction of cells below will motivate the use of cells.

\begin{definition}[Constant Cell]
A cell that does not depend on any wire nor thunk argument.
\end{definition}
$$
\begin{array}{rll}
\boxed{v} &: \Cell(\abst{g}, t, c) &= ((), f) \\
f(t, c, x, ()) &: W(t) &= v
\end{array}
$$

The resolved value is independent of wire values or thunks, it is simply a constant.

\motivdef it allows us to have a concise notation for a cell that resolves to a constant value.

\begin{definition}[Wire Cell]
A wire cell that resolves to the value of a wire.
\end{definition}
$$
\begin{array}{rll}
\boxed{w} &: \Cell(\abst{g}, t, c) &= (\bar{w}, f) \\
f(t, c, x, w) &: W(t) &= w
\end{array}
$$

The resolved value is the value of the single cell wire $\bar{w}$.

\motivdef it allows us to have a concise notation for a cell that resolves to a single wire value.

\newcommand{\cw}{\text{cellWire}}
- \subdefinition{get cell wire} $\cw(\bar{w}, f): \Option(\CWire(\abst{g})) = \bar{w}$
  - Yields the cell wire of the wire cell. 
  - Performing this operation on any other kind of cell will yield $\bot$.
  - This is necessary for relative wires defined later.

The following example showcases a nontrivial resolver if we were to implement lookup arguments as part of our protocol. In this case from [@plonkup] at page 4 in constructing the query wire. It is not necessary to understand the $\plookup$ protocol, but rather to see the utility of having a general resolver.
\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
\textbf{Lookup Cell}:  The following is an example of a cell for a hypothetical properad called $\frak{P}$ for $\plookup$  operations.
$$
\begin{array}{rll}
\boxed{\frak{p}(\aavec{w})} &: \Cell(\frak{P}, t, c) &= (\aavec{w}, f) \\
f(t, c, \zeta, \aavec{w}) &: W(t) &= \bar{w}_1 + \zeta \bar{w}_2 + \zeta^2 \bar{w}_3 \\
\end{array}
$$

\begin{itemize}
  \item \textit{motivation}: Notice how $\zeta: X(t,c)$ is a thunk argument, this is because we do not know $\zeta$ until the core $\Plonk$ prover has sufficiently progressed in the transcript. But with our abstractions, we can account for it cleanly before the protocol even begins.
\end{itemize}
\end{tcolorbox}

\begin{definition}[Default Cell]
A cell representing default values for a column.
\end{definition}
$$
\begin{array}{rll}
\boxed{-}&: \Cell(\abst{g}, t, c) &= ((), \text{default})
\end{array}
$$

\motivdef for notational brevity, columns irrelevant to a properad may be omitted, though vectors of a specific color must maintain uniform length. Default cells are padded for omitted columns. For most columns, the default value is $0$, while specialized columns like $\plookup$  for example use $\text{default}(t,c,t_{last}) = t_{last}$, where $t_{last}$ is the last entry of the lookup table as described in page 4 of [@plonkup], which like $\zeta$ in the example of $\boxed{\frak{P}(\aavec{w})}$ from before, can only be computed further in the protocol. This is yet another advantage of having thunk arguments in index maps.

Recall that $\text{default}(t,c,x): W(t)$ is a projection of a column, whose type signature conveniently matches that of the default cell resolver

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
\textbf{Resolver simplification}: The resolver of a cell that does not depend on any cell wire simplifies as follows:

\begin{math}
\begin{array}{ll}
R(\abst{g},()) & \text{resolver\ type\ notation} \\
= F(X(t,c) \to W[()] \to W(t)) & \text{defn\ of\ resolver}\\
= (t: \Color) \to (c: \Column) \to X(t,c) \to W[()] \to W(t) & \text{defn\ of\ } F \\
= (t: \Color) \to (c: \Column) \to X(t,c) \to \Uni^0 \to W(t) & \text{mapping\ zero\ length\ vector} \\
= (t: \Color) \to (c: \Column) \to X(t,c) \to W(t) & \text{zero\ length\ is\ unit\ thats\ omissible}
\end{array}
\end{math}

This results in the same type for '$\text{default}$'.
\end{tcolorbox}

\begin{definition}[Pre-Constraints]
The class of constraints of a specific properad. Naively, a template for a sub-table in the trace table.
\end{definition}
$$
\begin{array}{rl}
\PreTable&: \Prpd \to \IndexMap(X, F(\Cell(\abst{g}, t,c)^{k(t)}))  \\
\ctrn(\abst{g})&: \PreTable(\abst{g})
\end{array}
$$

We use $k(t)$ to denote that the vectors are of uniform length per color.

- **Notation**: the hat denotes that it is a pre-constraint, not a constraint; an abstract constraint.

\motivdef pre-Constraints act as a template for a sub-table for gates of the properad. This makes the constraints in the trace table derivable from the properads; a single source of truth.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
The Pre-constraints for $\build{a + b}{}{}$ and $\build{a \times b}{}{}$. Let $\text{Add}^t, \text{Mul}^t: \Prpd$

\begin{center}
\begin{tabular}{ c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[minimum width=2cm, minimum height=1.5cm] (tab) {
\begin{tabular}{|c|c|c|c|c|c|c|}
\hline
$\abst{g}$ & $n$ & $m$ & $\pin$ & $\pout$ & $\eval(a,b)$ & $\term$\\
\hline
$\text{Add}^t$ & $2$ & $1$ & $(t,t)$ & $t$ & $a + b$ & $F_{GC}^{\plonkm}$ \\
\hline
$\text{Mul}^t$ & $2$ & $1$ & $(t,t)$ & $t$ & $a \times b$ & $F_{GC}^{\plonkm}$ \\
\hline
\end{tabular}
};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{add}{(0,0)}{$\abst{a}$, $\abst{b}$}{$\text{Add}^t$}{1}
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] ($(add-in-2)+(0,0.25)$) -- (add-in-2);
\draw[->,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\node[anchor=north east] at (add-out-1) {$\abst{c}$};

\gate{mul}{($(add.north east)+(0.5,0)$)}{$\abst{a}$, $\abst{b}$}{$\text{Mul}^t$}{1}
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- (mul-in-1);
\draw[-,thick] ($(mul-in-2)+(0,0.25)$) -- (mul-in-2);
\draw[->,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.4)$);
\node[anchor=north east] at (mul-out-1) {$\abst{c}$};
\end{tikzpicture}
\end{tabular}
\end{center}
\begin{center}
\begin{tabular}{c}
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{10}{|c|}{$\ctrn(\text{Add}^t)$} \\
\hline
\multicolumn{9}{|c|}{$t$} & $\cdots$ \\
\hline
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_c$ & $PI$ & $\cdots$ \\
\hline
$a$ & $b$ & $c$ & $1$ & $1$ & $-1$ & $0$ & $0$ & $0$ \\
\cline{1-9}
\end{tabular}
\\ \\
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{10}{|c|}{$\ctrn(\text{Mul}^t)$} \\
\hline
\multicolumn{9}{|c|}{$t$} & $\cdots$ \\
\hline
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_c$ & $PI$ & $\cdots$ \\
\hline
$a$ & $b$ & $c$ & $0$ & $0$ & $-1$ & $1$ & $0$ & $0$  \\
\cline{1-9}
\end{tabular}
\end{tabular}
$$
F_{GC}^{\plonkm}: \Eqn = A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI
$$
\end{center}

Notice that $\boxed{a}, \boxed{b}, \boxed{c}$ are wire cells and $\boxed{1}, \boxed{-1}, \boxed{0}$ are constant cells. Our notation conveniently aligns with drawing tables.

The pre-constraints in this example only define a gate for the color $t$ wheras the rest of the colors are empty. Recall that the length of the vector of cells must be uniform within a color, but not across colors. So this definition is fine.
\end{tcolorbox}

In the above example, we use the notation $F_{GC}^{\plonkm}$ for the gate constraint polynomial from [@plonk], the gate constraint in our $\Plonk$ protocol is dependent on the properads that the user defines which we will see later when we define $\Spec$.


\begin{definition}[Trace Table]
When an index map is a composition of pre-constraints and all of the cells are resolved, we have a trace table.
\end{definition}
\newcommand{\TraceTable}{\text{TraceTable}}
$$
\begin{array}{rl}
\TraceTable&: \IndexMap(X, F(W(t)^{k(t)})) \\
T &: \TraceTable
\end{array}
$$

\motivdef it serves as the last intermediate data structure before we can interpolate them into polynomials and other data forming the circuit $(R,X,W)$.

\begin{definition}[Row]
Get the $i$th row of the table; informally an index map of vectors, for a specific color.
\end{definition}
\newcommand{\row}{\text{row}}
\newcommand{\gatef}{\text{gate}}
$$
\begin{array}{rl}
\row &: \IndexMap(X, F(Y^{k(t)})) \to (t': \Color) \to [k(t')+1] \to ((c:\Column) \to X(t',c) \to Y)\\
\row(A, t', i) &= f[A](t') \\
f(t,c,x,\vec{y}) &= y_i
\end{array}
$$

Let's break down the type signature and definition:

- An index map of vectors / table has the type $\IndexMap(X, F(Y^{k(t)}))$.
- We bind $k$ to construct the type for $i$ where $[k(t')+1]: \pset{\Nb}$ guarantees that $i$ is a valid index in color $t'$.
- Recall mapping an index map expects a function of type $F(X(t,c) \to Y_1(t,c) \to Y_2(t,c))$
- $f$ ignores $t,c,x$ arguments and simply indexes the vector typed $Y^{k(t)}$ i.e. $Y_1(t,c)$, at $i$.
- The resulting index map then is partially applied to $t'$; a function of type $(c: \Column) \to X(t',c) \to Y$
- Conveniently, this matches the last argument type for $\text{foldEqn}$ where $T = X(t',c) \to Y$.
- If $X(t',c) = \Unit$ then $T = Y$, this motivates the use of this function on trace tables.

\begin{definition}[Constraint]
Similar to the row function, but specialized for $Y= W(t')$.
\end{definition}
\newcommand{\cctrn}{\text{constraint}}
$$
\begin{array}{rl}
\cctrn &: \IndexMap(X, F(W(t')^{k(t')})) \to (t': \Color) \to [k(t')+1] \to ((c:\Column) \to X(t',c) \to W(t')) \\
\cctrn &= \row
\end{array}
$$

\motivdef it allows us to extract rows from the table of a specific color which can be used to evaluate equations. Typically, the gate constraint polynomial.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
Let the trace table for $\build{w_1 + (w_2 \times w_3) = z^*}{}{}$ where $\pwit = (q,q,q)$ be the following (we will describe how this is computed when we define trace later):
\begin{center}
\begin{tabular}{ c c }
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{10}{|c|}{$T$} \\
\hline
\multicolumn{9}{|c|}{$q$} & $\cdots$
\\
\hline
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_c$ & $PI$ & $\cdots$ \\
\hline
$w_2$ & $w_3$ & $t$ & $0$ & $0$ & $-1$ & $1$ & $0$ & $0$  \\
\cline{1-9}
$w_1$ & $t$ & $z$ & $1$ & $1$ & $-1$ & $0$ & $0$ & $0$ \\
\cline{1-9}
\end{tabular}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{in0}{(0,0)}{}{$\Input^q_1$}{1}
\gate{in1}{($(in0.north east)+(0.1,0)$)}{}{$\Input^q_2$}{1}
\gate{in2}{($(in1.north east)+(0.1,0)$)}{}{$\Input^q_3$}{1}

\gate{add}{($(in0.south west)+(0.1875,-0.5)$)}{$\abst{w_1}$, $\abst{t}$}{$\text{Add}^q$}{1}
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] ($(add-in-2)+(0,0.25)$) -- (add-in-2);
\draw[-,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\node[draw, thick, circle, double, double distance=1pt, anchor=north] at ($(add-out-1)+(0,-0.4)$) {$\abst{z}$};


\gate{mul}{($(add.north east)+(0.5,0)$)}{$\abst{w_2}$, $\abst{w_3}$}{$\text{Mul}^q$}{1}
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- (mul-in-1);
\draw[-,thick] ($(mul-in-2)+(0,0.25)$) -- (mul-in-2);
\draw[-,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.4)$);
\draw[-,thick] ($(mul-out-1)+(0,-0.4)$) -- ($(mul-out-1)+(-0.85,-0.4)$);
\draw[-,thick] ($(mul-out-1)+(-0.85,-0.4)$) -- ($(mul-out-1)+(-0.85,1.575)$);
\draw[-,thick] ($(mul-out-1)+(-0.85,1.575)$) -- ($(add-in-2)+(0,0.25)$);

\draw[-,thick] (in0-out-1) -- ($(in0-out-1)+(0,-0.25)$);
\draw[-,thick] ($(in0-out-1)+(0,-0.25)$) -- ($(add-in-1)+(0,0.25)$);
\draw[-,thick] (in1-out-1) -- ($(in1-out-1)+(0,-0.25)$);
\draw[-,thick] ($(in1-out-1)+(0,-0.25)$) -- ($(mul-in-1)+(0,0.25)$);
\draw[-,thick] (in2-out-1) -- ($(in2-out-1)+(0,-0.25)$);
\draw[-,thick] ($(in2-out-1)+(0,-0.25)$) -- ($(mul-in-2)+(0,0.25)$);
\end{tikzpicture}
\end{tabular}
\end{center}

Note that the table entries are not cells, these are values; $W(q)$, since it is a trace table and not pre-constraints.

Recall $\term(\text{Add}^t) = \term(\text{Mul}^t) = F_{GC}^{\plonkm}: \Eqn = A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI$, thus:

$$
\begin{array}{rll}
F_{GC}^{\plonkm}(\cctrn(T,q,1)) &= w_2 + w_3 - t &\stackrel{?}{=} 0 \\
F_{GC}^{\plonkm}(\cctrn(T,q,2)) &= -z + (w_1 \times t) &\stackrel{?}{=} 0 \\
\end{array}
$$

Thus $F_{GC}^{\plonkm}$ implies the structural integrity of the gadgets when it evaluates to zero given the constraints.
\end{tcolorbox}

At this point, we want to emphasize the expressivity of index map as an abstraction. Pre-constraints, trace table and constraints are all defined as / from an index map. Index maps also interface nicely with equations. Subsequently, we will see that even the circuit $(R,X,W)$ are also index maps. All whilst accounting for incomplete protocol specific information via thunk arguments.

### Relative Wires

\begin{definition}[Relative Wires]
Relative wires, are wires that are suffixed to the vector of input wires of a gate; it is a subvector at the end. Relative wires cannot be mentioned in the gate's pre-constraints, thus there are no cell wires of it defined by its properad.
\end{definition}

\begin{definition}[Relative Gate]
We call the gate of a relative wire, the relative gate.
\end{definition}

\begin{definition}[Base Gate]
The gate whose pre-constraints first row contains the relative wire; where the wire is not relative, is called the base gate. 
\end{definition}

The last row of the relative gate's pre-constraints when fed to its term equation can refer to its relative wires in the next row. Thus, the base gate's pre-constraints must appear immediately after the relative gate's pre-constraints in the trace table. This will be ellucidated concretely in the penultimate example of this section.

The motivation for these definitions is that relative wires allows us to "reuse" cells in the trace table. This could potentially reduce the number of constraints in the trace table whilst still being able to express the same computation. This is especially useful for gates that have a large number of inputs, such as the poseidon gate.

We now continue defining more projections for properads and gates to define relative wires.

$$
\begin{array}{cc}
\abst{g}: \Prpd &
g: \Ggt
\end{array}
$$

\newcommand{\rel}{\text{rel}}
\newcommand{\grcol}{\text{rcol}}
\begin{definition}[Get number of relative wires]
Get the number of relative wires a properad has.
\end{definition}
$$
b(\abst{g}): \Nb
$$

\begin{definition}[Get relative wires]
Get the relative wires of a gate.
\end{definition}
$$
\rel(g): \Wire^{b \circ \ty(g)} = \maybe{\avec{r}}{\gin(g) = \avec{x} \cat \avec{r} \land |\avec{r}| = b(\abst{g})}
$$

\begin{definition}[Get relative columns]
Get the columns that the relative wires must be found in the base gate's pre-constraints first row.
\end{definition}
$$
\grcol(\abst{g}): \Column^{b(\abst{g})}
$$

\begin{definition}[Complete definition of Cell Wires]
Refining the definition of cell wires to exclude relative wires.
\end{definition}
$$
\begin{array}{rl}
\CWire(c, \abst{g})&: \pset{\Nb} = \begin{cases}
  [n(\abst{g}) + m(\abst{g}) + 1] \setminus [n(\abst{g}) - b(\abst{g}) + 1 .. n(\abst{g}) + 1] & \pcol(c) \lor c = \text{PI} \\
  \emptyset & \otherwise
\end{cases}
\end{array}
$$

\motivdef as mentioned before, relative wires are supposed to be found in the base gate, not the relative gate itself, thus we exclude them from being mentioned in the relative gate's pre-constraints.

\begin{definition}[Get relative wire position]
Given a gate, column and a relative wire, verify that the gate's first row of pre-constraints contains the relative wire in the column specified. i.e. we are assuming the gate is a base gate.
\end{definition}
$$
\begin{array}{rl}
\text{pos} &: \AbsCirc \to \Ggt \to \Column \to \Wire \to \Bb \\
\text{pos}(\abst{f}, g, c, \abst{w}) &= \begin{array}{ll}
\rcol(c) &\land \\
\exists \bar{w}: \CWire(c, \ty(g)). \text{wires}(\abst{f}, g) @ \bar{w} = \abst{w} &\land \\
\cw \circ \row(\ctrn \circ \ty(g), \ty(\abst{w}), 1)(c) = \bar{w} 
\end{array}
\end{array}
$$

Lets break down the definition:

- Given a gate $g$ and wire $\abst{w}$, get its cell wire representation $\bar{w}$ by comparing it with the gate's wires.
- Recall $\rcol(c)$ is a projection of columns; it checks if the column is marked as a candidate to host relative wires.
- Check the first row of $g$'s pre-constraints at the color of $\abst{w}$ that the cell in that column is indeed $\boxed{w}$.

\motivdef knowing if a relative wire exists in a valid pre-constraint allows us to determine if the gate is a candidate for a relative gate's base gate. We need to know this to verify if the relative gate can be structurally sound in the circuit.

In future work, it is possible to precompute and cache the set of properads that can make base gates for all properads that can make relative gates. Thus, we simply have to check for $\ty(g)$.

\begin{definition}[Get base gate]
Get the base gate given a relative gate
\end{definition}
$$
\begin{array}{rl}
\base &: \AbsCirc \to \Ggt \to \Option(\Ggt) \\
\base(\abst{f}, g) &= \maybe{g'}{\begin{array}{l}
\exists \gpair{g'}{\abst{y}} \in \abst{f}. \forall i. \text{pos}(\abst{f}, g', \grcol(g) @ i, \rel(g) @ i)
\end{array}}
\end{array}
$$

Given a gate $g$, the base gate $g'$ should exist in the abstract circuit $\abst{f}$, such that it holds wire cells of all of the relative wires of $g$ in the first row of its pre-constraints, in the expected columns.

\motivdef it succinctly expresses the base gate of a relative gate if it exists.

TODO

- complete assertion for gate construction:
  - $g = \abst{g}(\avec{x} \cat \avec{r})$ denotes that $\ty[\avec{x} \cat \avec{r}] = \pin(\abst{g})$ and if $|\avec{r}| = b(\abst{g})$ and $b(\abst{g}) > 0$  then $\base(\abst{f}, g) \neq \bot$

TODO

- Column + to refer to next row in eqn
- relGate: index i and next where next can loop back to first row at last,
- relative wire example
- spec
- DONE

\newcommand{\WireType}{\text{WireType}}

**Relative Wires Example**

Let parts of the pre-constraints for the gadgets $\gpair{\ggtu{\text{CMul}_p}{\abst{d},\abst{e},\abst{c}}}{\abst{r}}, \gpair{\ggtu{\text{Mul}_p}{\abst{a}, \abst{b}}}{\abst{c}} \in \abst{f}$ where $b_{\text{CMul}_p} = 1$, $b_{\text{Mul}_p} = 0$ and $\Rel_{\text{CMul}_p} = \set{C}$, $\Rel_{\text{Mul}_p} = \emptyset$ be as follows:

\begin{center}
\begin{tabular}{c c}
\begin{tabular}{r|c|c|c|c|c|c|c|c|c}
\cline{2-9}
\multirow{2}{*}{$\Ops$} & \multicolumn{8}{c|}{$p$} & \multirow{2}{*}{$\term$} \\
\cline{2-9}
& $A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_s$ \\
\hline\hline
$\text{CMul}_p$ & $\abst{d}$ & $\abst{e}$ & $\abst{r}$ & 0 & 0 & 0 & 0 & 1 &
$Q_s \times (C_1 \times A \times B - C)$ \\
\hline
$\text{Mul}_p$ & $\abst{a}$ & $\abst{b}$ & $\abst{c}$ & 0 & 0 & -1 & 1 & 0 &
$A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m$ \\
\hline
\end{tabular} &
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{cmul}{(0,0)}{$\abst{d}$,$\abst{e}$,$\abst{c}$}{$\text{CMul}_p$}{1}
\gate{mul}{($(cmul.north)+(1,0)$)}{$\abst{a}$,$\abst{b}$}{$\text{Mul}_p$}{1}
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- (mul-in-1);
\draw[-,thick] ($(mul-in-2)+(0,0.25)$) -- (mul-in-2);
\draw[-,thick] ($(cmul-in-1)+(0,0.25)$) -- (cmul-in-1);
\draw[-,thick] ($(cmul-in-2)+(0,0.25)$) -- (cmul-in-2);
\draw[-,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.4)$);
\draw[-,thick] ($(mul-out-1)+(0,-0.4)$) -- ($(mul-out-1)+(-0.65,-0.4)$);
\draw[-,thick] ($(mul-out-1)+(-0.65,-0.4)$) -- ($(mul-out-1)+(-0.65,1.75)$);
\draw[-,thick] ($(mul-out-1)+(-0.65,1.75)$) -- ($(cmul-in-3)+(0,0.35)$);
\draw[-,thick] ($(cmul-in-3)+(0,0.35)$) -- (cmul-in-3);
\node[draw, thick, circle, double, double distance=1pt, anchor=north] at ($(cmul-out-1)+(0,-0.4)$) {$\abst{r}$};
\draw[-,thick] (cmul-out-1) -- ($(cmul-out-1)+(0,-0.4)$);
\end{tikzpicture}
\end{tabular}
\end{center}

Using the terms, we have $-c + a \cdot b = 0$ enforcing the structure of $\text{Mul}_p$ and $c \cdot d \cdot e - r = 0$ enforcing the structure of $\text{CMul}_p$. Notice $C_1$ is a distinct column that refers to the same column $C$ but one row below current. In this case it is the row for $\ctrn_{\text{Mul}_p}$. Thus, $\build{a \times b \times d \times e = r}{}{}$ is expressed in two rows instead of of three, if we were to use all $\text{Mul}_p$.

### Spec

A *specification*[^spec-benefit] defines the config of the protocol. This includes marking columns as private or enabling it for copy constraints. Here is also where default values for columns are defined, used when constructing the trace table. In the previous section on arithmetize, we omitted $s:\Spec$ in $\AState$ leaving $W, \WireType, \Ops$ implicit for $W_s, \WireType_s, \Ops_s$. We will keep the spec instance $s$ implicit beyond this section as well. We conclude with tabulating all objects according to their abstraction levels.

[^spec-benefit]: With spec as a data structure, it is dynamic and can be extended whilst arithmetizing.

$$
\begin{array}{cccccc}
s : \Spec &
X_s: F(\Uni) &
D_s: \text{Default} &
\WireType_s: \Uni &
W_s: \WireType_s \to \Uni &
\Ops_s : \pset{\Ops}
\end{array}
$$
$$
\begin{array}{cccc}
\Column_s: \pset{\Column} = \bigcup\limits_{g \in \Ops_s} \Column_g &
F_{GC}: \Eqn = \sum \bigcup\limits_{g \in \Ops_s} \term_g &
\text{priv}_s: \pset{\Column_s} &
\text{CC}_s: \pset{\Column_s}
\end{array}
$$

abstraction level | atomics | semantic groups | structure | use
-|-|-|-|-
0 | $w: W(t)$ | $@_i \circ T^t: \text{Gate}$ | $T: \TraceTable$ | protocol
1 | $\abst{w}: \Wire$ | $g: \Ggt$ | $\abst{f}: \AbsCirc$ | circuit building
2 | $\boxdot : \Cell$ | $\ty(g): \Ops$ | $s: \Spec$ | config
