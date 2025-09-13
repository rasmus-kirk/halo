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

- **Notation**: $\Column = \set{A,B,C,Q_l,Q_r,Q_o,Q_m,Q_c,PI, \ldots}$
- \projs
  - $\pcol(c): \Bb$ - determines if the column belongs to $X$ or $W$ of the circuit $(R, X, W)$; private or public.
  - $\cccol(c): \Bb$ - is $c$ a copy constraint relevant column.

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

- \subdefinition{thunk} Informally, a thunk is a function $f: X \to Y$
  - $X$ is the thunk argument type. If $X = \Unit$ then it is not a thunk but a value of $Y$ by omissible argument.
  - $Y$ is the value type. If $Y= \Option(T)$, then informally the index map is partially populated.
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

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
As an example to see index map concretely, we define colors $p,q$ and columns $A,B,C$ such that $C$ is a thunk column that depends on a natural number argument. We then define two index maps $T_1$ and $T_2$.

\vspace{1em}

Let $\forall t \in \set{p,q}.X(t,A) = X(t,B) = \Unit \land X(t,C) = \Nb$

Let $Y(t,\_) = \Nb^{k(t)}$ where $k(t): \Nb$ is a natural number that varies by color.

Let $T_1, T_2: \IndexMap(X, Y)$

\vspace{1em}

\begin{center}
\begin{tabular}{ c c c}
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{6}{|c|}{$T_1$} \\
\hline
\multicolumn{3}{|c|}{$q$} & \multicolumn{3}{|c|}{$p$} \\
\hline
$A$ & $B$ & $C$ & $A$ & $B$ & $C$ \\
\hline
1 & 2 & $x+3$ & 4 & 5 & $x-6$ \\
\hline
0 & 0 & $x$ \\
\cline{1-3}
\end{tabular}
&
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{6}{|c|}{$T_2$} \\
\hline
\multicolumn{3}{|c|}{$q$} & \multicolumn{3}{|c|}{$p$} \\
\hline
$A$ & $B$ & $C$ & $A$ & $B$ & $C$ \\
\hline
7 & 8 & $x-9$ & 10 & 11 & $x \times 12$ \\
\hline
\end{tabular}
\end{tabular}

\vspace{1em}

Notice that $k(q) = 2$ for $T_1$ but for every other color and $T_2$ is $1$.

Concatenating the index maps results in the following index map:

\vspace{1em}

\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{6}{|c|}{$T_1 \cat T_2$} \\
\hline
\multicolumn{3}{|c|}{$q$} & \multicolumn{3}{|c|}{$p$} \\
\hline
$A$ & $B$ & $C$ & $A$ & $B$ & $C$ \\
\hline
1 & 2 & $x+3$ & 4 & 5 & $x-6$ \\
\hline
0 & 0 & $x$ & 10 & 11 & $x \times 12$ \\
\hline
7 & 8 & $x-9$ \\
\cline{1-3}
\end{tabular}

\vspace{1em}

Thus, if we query column $A$ of color $q$, we get:
$$
(T_1 \cat T_2)^q(A) = (1,0,7)
$$

If we supply the thunk $42$ to the $C$ column of color $p$, we get:
$$
(T_1 \cat T_2)^p_{42}(C) = (42-6, 42\times 12) = (36,504)
$$
\end{center}
\end{tcolorbox}

\begin{definition}[Equation]
An equation is a grammar that expresses a polynomial structure; a tree where leaf vertices are columns or coefficient scalars and intermediate vertices are operations of the grammar.
\end{definition}
$$
\begin{array}{ccc}
& E: Eqn  \\
\begin{array}{rl}
\langle Eqn \rangle &::= Eqn1\ Eqn' \\
\langle Eqn'\rangle &::= +\ Eqn \\
& |\ -\ Eqn\ | \epsilon \\
\end{array} &
\begin{array}{rl}
\langle Eqn1 \rangle &::= Eqn2\ Eqn1' \\
\langle Eqn1'\rangle &::= \times Eqn1\ |\ \epsilon
\end{array} &
\begin{array}{rl}
\langle Eqn2 \rangle &::= - Eqn \\
& |\ \mathtt{Scalar}\ \times \ Eqn \\
& |\ \mathtt{(}\ Eqn\ \mathtt{)} \\
& |\ \mathtt{Column} \\
\end{array}
\end{array}
$$
$$
\text{foldEqn}: (-: T \to T) \to (+: T \to T \to T) \to (\times: T \to T \to T) \to (\times_\Fb: \Fb \to T \to T) \to \Eqn \to (\Column \to T) \to T
$$

- **Notation**: If $X: \Column \to T$ then $E(X): T = \text{foldEqn}(\ldots, E, X)$ 
  - when every operation of the grammar is specified for type $T$, we can evaluate the equation.
  - curve points do not have multiplication defined, thus equations involving them cant be evaluated.


\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
Let $f: Eqn = A + B \times -C$

Let $X: \Column \to \Nb$

Let $X(A) = 1, X(B) = 2, X(C) = 3$

Then $f(X) = 1 + 2 \times -3 = -5$ since the operators required by $\text{foldEqn}$ are well defined for $\Nb$.
\end{tcolorbox}

\motivdef it is the single source of truth for an equational definitions that can vary over operand types: scalars, polynomials, curve points, wires and state via build. Examples of equations are gate constraint polynomials, grand product polynomials, quotient polynomial, $\plookup$  compression equation, etc.

When implemented in a programming language such as rust, it is possible to use type variables / generics to define a function over $T$, without having an explicit syntax tree data structure of the $Eqn$'s grammar.

\begin{definition}[Cell Wire]
Cell wires are natural numbers that represent the wires of a gate in a properad. Intuitively, an abstract wire. It does exclude some wires which will be defined later.
\end{definition}
\newcommand{\CWire}{\text{CellWire}}
\newcommand{\aavec}[1]{\bar{\vec{#1}}}
$$
\begin{array}{rl}
\CWire(c, \abst{g})&: \pset{\Nb} = \begin{cases}
  [n(\abst{g}) + m(\abst{g}) + 1] \setminus \cdots & \pcol(c) \lor c = \text{PI} \\
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

\motivnot it allows us to index vectors over a higher order function such as mapping over a vector. Moreover if a function call returns a vector and we wish to index it immediately, this notation is more legible.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
If $f(a,b,c,d): T^k$ is a vector and $g(t,c): [k+1]$ is a natural number that is a valid index.

Then $f(a,b,c,d) @ g(t,c)$ is more legible than $f(a,b,c,d)_{g(t,c)}$.

And we can use it in higher order functions such as $(f(a,b,c,d) @ -)\circ g(t) [\vec{c}]$
\end{tcolorbox}

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

Notice how $\zeta: X(t,c)$ is a thunk argument, this is because we do not know $\zeta$ until the core $\Plonk$ prover has sufficiently progressed in the protocol; $\zeta$ is a transcript hash at some point of the protocol. But with our abstractions, we can account for it cleanly before the protocol even begins.
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
= F(X(t,c) \to W[()] \to W(t)) & \text{definition\ of\ resolver}\\
= (t: \Color) \to (c: \Column) \to X(t,c) \to W[()] \to W(t) & \text{definition\ of\ } F \\
= (t: \Color) \to (c: \Column) \to X(t,c) \to \Uni^0 \to W(t) & \text{mapping\ zero\ length\ vector} \\
= (t: \Color) \to (c: \Column) \to X(t,c) \to W(t) & \text{zero\ length\ is\ unit\ thats\ omissible}
\end{array}
\end{math}

This results in the same type for '$\text{default}$'.
\end{tcolorbox}

\begin{definition}[Pre-Constraints]
The class of constraints of a specific properad. Intuitively, a template for a sub-table in the trace table.
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
Let the pre-constraints for $\build{a + b}{}{}$ and $\build{a \times b}{}{}$ where $\text{Add}^t, \text{Mul}^t: \Prpd$ be defined as follows:

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

\vspace{1em}

The pre-constraints in this example only define a row for the color $t$ wheras the rest of the colors are empty. Recall that the length of the vector of cells must be uniform within a color, but not across colors. So these pre-constraints type checks / are valid.
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
\row(A, t', i) &= f(i)[A](t') \\
f(i,t,c,x,\vec{y}) &= y_i
\end{array}
$$

Let's break down the type signature and definition:

- An index map of vectors / table has the type $\IndexMap(X, F(Y^{k(t)}))$.
- We bind $k$ to construct the type for $i$ where $[k(t')+1]: \pset{\Nb}$ guarantees that $i$ is a valid index in color $t'$.
- Recall mapping an index map expects a function of type $F(X(t,c) \to Y_1(t,c) \to Y_2(t,c))$
- $f$ ignores $t,c,x$ arguments and simply indexes the vector typed $Y^{k(t)}$ i.e. $Y_1(t,c)$, at $i$.
- The index map then is partially applied with $t'$. This returns $(c: \Column) \to X(t',c) \to Y$
- Conveniently, this is the type of the last argument of $\text{foldEqn}$ when the equation operands are $T = X(t',c) \to Y$.
- If $X(t',c) = \Unit$ then $T = Y$ by omissible argument.
- This motivates its use on trace tables to project rows that can be used to evaluate equations.

\begin{definition}[Constraint]
Similar to the row function, but specialized for $Y= W(t')$.
\end{definition}
\newcommand{\cctrn}{\text{constraint}}
$$
\begin{array}{rl}
\cctrn &: \IndexMap(X, F(W(t)^{k(t)})) \to (t': \Color) \to [k(t')+1] \to ((c:\Column) \to X(t',c) \to W(t')) \\
\cctrn &= \row
\end{array}
$$

\motivdef it allows us to extract rows from the table of a specific color which can be used to evaluate equations. Typically, the gate constraint polynomial. We can see its use in the following example:

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
Assume the definition of the properads $\text{Add}^t, \text{Mul}^t$ from the previous example.

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
F_{GC}^{\plonkm}(\cctrn(T,q,1)) &= -t + w_2 \times w_3 &\stackrel{?}{=} 0 \\
F_{GC}^{\plonkm}(\cctrn(T,q,2)) &= w_1 + t -z &\stackrel{?}{=} 0 \\
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

We now continue defining more projections for properads and gates to complete the formal specification of relative wires.

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

\begin{definition}[Check is Relative Column]
This projection of a column determines if it is a relative wire relevant column.
\end{definition}
\newcommand{\rcol}{\text{rel}}
$$
\rcol(c): \Bb
$$

\motivdef it allows the user to mark columns that can host relative wires.

\begin{definition}[Pseudo Columns]
A pseudo column is used to refer to the next row of a column that is relative wire relevant. The plus in $c^+$ denotes it is the pseudo column of $c$.
\end{definition}
$$
\text{pseudo}(c^+): \Bb
$$

\begin{definition}[Un-pseudo a column]
Get the original column from a pseudo column. If the column is not pseudo, it returns itself.
\end{definition}
$$
\text{unpseudo}(c^+): \Column = c
$$

\motivdef it allows us to refer to the next row of the column $c$ when dealing with index maps. In the context of the last row, the pseudo column refers to the first row.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
If our column is $Q_x$, we refer to the next row of the same column as $Q_x^+$.
\end{tcolorbox}

Pre-constraints are banned from defining any such $c^+$ columns, via a $\text{pseudo}(c)$ check. But it is not excluded as an argument for index maps. We can thus construct a constraint from an index map that also includes the relative cells; values from next row. This will be made formal in the definition of relative constraints defined later.

\begin{definition}[Get relative wire position]
Given a gate, column and a relative wire, verify that the gate's first row of pre-constraints contains the relative wire in the column specified. Intuitively, the function corroborates if the gate is a base gate.
\end{definition}
$$
\begin{array}{rl}
\text{pos} &: \AbsCirc \to \Ggt \to \Column \to \Wire \to \Bb \\
\text{pos}(\abst{f}, g, c, \abst{w}) &= \left(\begin{array}{ll}
\rcol(c) &\land \\
\exists \bar{w}: \CWire(c, \ty(g)). \text{wires}(\abst{f}, g) @ \bar{w} = \abst{w} &\land \\
\cw \circ \row(\ctrn \circ \ty(g), \ty(\abst{w}), 1)(c) = \bar{w} 
\end{array}\right)
\end{array}
$$

Lets break down the definition:

- Verify that the column is marked as a relative wire relevant column via $\rcol(c)$.
- Given a gate $g$ and wire $\abst{w}$, get its cell wire representation $\bar{w}$ by comparing it with the gate's wires.
- Check the first row of $g$'s pre-constraints at the color of $\abst{w}$ that the cell in that column is indeed $\boxed{w}$.

\motivdef knowing if a relative wire exists in a valid pre-constraint allows us to determine if the gate is a candidate for a relative gate's base gate. We need to know this to verify if the relative gate can be structurally sound in the circuit.

In future work, it is possible to precompute and cache the set of properads that can make base gates for every relative gate. Thus, we simply have to check $\ty(g)$ and if the relative wires are in $\text{wires}(\abst{f}, g)$ not as relative wires in $g$ to determine if $g$ is a candidate base gate.

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

Let's break down the definition:

- We find any gate $g'$ in the abstract circuit $\abst{f}$; ignoring its output wire $\abst{y}$.
- For each relative wire in $\grcol(g)$, run $\text{pos}$ to verify with their respective expected column in $\rel(g)$. 

\motivdef it succinctly expresses the base gate of a relative gate if it exists.

In future work, we can seek to find the base gate with the minimum cost, i.e. least amount of rows / constraints.

\begin{notation}[Full assertion for gate construction]
Recall before that constructing a gate, will type check its inputs and we mentioned an assertion to be defined later. Here it is.
\end{notation}
$$
g = \abst{g}(\avec{x} \cat \avec{r}) \text{\ denotes\ that\ } 
\left(\begin{array}{ll}
& \ty[\avec{x}] = \pin(\abst{g})\\
\land & |\avec{r}| = b(\abst{g})\\
\land & (b(\abst{g})) > 0 \implies \base(\abst{f}, g) \neq \bot
\end{array}\right)
$$

\begin{definition}[Relative Constraint]
Recall that constraint is a function that extracts a row. We redefine it to account for the next row mapped via pseudo columns.
\end{definition}
$$
\begin{array}{rl}
\cctrn &: \IndexMap(X, F(W(t')^{k(t')})) \to (t': \Color) \to [k(t')+1] \to ((c:\Column) \to X(t',c) \to W(t')) \\
\cctrn(T,t',i) &= f(T,i)[A](t') \\
f(T,i,t,c,x,\vec{y}) &= \begin{cases} 
  \text{nextRow}(T,i,t,x) \circ \text{unpseudo}(c) & \text{pseudo}(c) \\
  y_i & \otherwise
\end{cases} \\
\text{nextRow}(T,i,t,x) &= \row(T,t,i+1 \mod (k(t)+1))(x) \\
\end{array}
$$

Let's break down the definition:

- We get the next row by using the preexisting row function, and incrementing the index $i$ by $1$.
- We use modulo to wrap around to the first row if we are at the last row.
- The last row is supplied with the thunk argument $x$, thus we are guaranteed to get the non thunk value.
- If the column is marked as a pseudo column, we use the next row's value for that column.

The following example illustrates the use case of relative wires in reducing the number of constraints. In our IVC implementation, this feature is exploited heavily by the poseidon gate for the feasibility of the IVC circuit.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]
We introduce a hypothetical relative gate of the properad $\text{CMul}$.

\begin{center}
\begin{tabular}{|c|c|c|c|c|c|c|c|c|}
\hline
$\abst{g}$ & $n$ & $m$ & $b$ & $\grcol$ & $\pin$ & $\pout$ & $\eval(\vec{x})$ & $\term$ \\
\hline
$\text{CMul}^t$ & $3$ & $1$ & $1$ & $C$ & $(t,t,t)$ & $t$ & $x_1 \times x_2 \times x_3$ & $\term(\text{CMul}^t)$ \\
\hline
$\text{Mul}^t$ & $2$ & $1$ & $0$ & $()$ & $(t,t)$ & $t$ & $x_1 \times x_2$ & $F_{GC}^{\plonkm}$ \\
\hline
\end{tabular}

$$
\begin{array}{rl}
\term(\text{CMul}^t): \Eqn &= Q_s \times (C^+ \times A \times B - C) \\
F_{GC}^{\plonkm}: \Eqn &= A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI
\end{array}
$$
\end{center}

Let parts of the trace table for the gates $\gpair{\ggtu{\text{CMul}_p}{\abst{d},\abst{e},\abst{c}}}{\abst{r}}, \gpair{\ggtu{\text{Mul}_p}{\abst{a}, \abst{b}}}{\abst{c}} \in \abst{f}$ be as follows:


\begin{center}
\begin{tabular}{c c}
\begin{tabular}{r|c|c|c|c|c|c|c|c|}
\cline{2-9}
\multirow{3}{*}{$\Ggt$} & \multicolumn{8}{c|}{$T$} \\
\cline{2-9}
& \multicolumn{8}{c|}{$q$} \\
\cline{2-9}
& $A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_s$ \\
\hline\hline
$\text{CMul}^q(\abst{d}, \abst{e}, \abst{c})$ & $d$ & $e$ & $r$ & 0 & 0 & 0 & 0 & 1 \\
\hline
$\text{Mul}^q(\abst{a}, \abst{b})$ & $a$ & $b$ & $c$ & 0 & 0 & -1 & 1 & 0 \\
\hline
\end{tabular} &
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{cmul}{(0,0)}{$\abst{d}$,$\abst{e}$,$\abst{c}$}{$\text{CMul}^q$}{1}
\gate{mul}{($(cmul.north)+(1,0)$)}{$\abst{a}$,$\abst{b}$}{$\text{Mul}^q$}{1}
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

Thus, $\text{Mul}^q(\abst{a}, \abst{b})$ is the base gate to $\text{CMul}^q(\abst{d}, \abst{e}, \abst{c})$ where $\abst{c}$ is the relative wire expected to be in column $C$.

$$
\begin{array}{rll}
\term(\text{CMul}^t)(\cctrn(T,q,1)) &= c \times d \times e - r &\stackrel{?}{=} 0 \\
F_{GC}^{\plonkm}(\cctrn(T,q,2)) &= -c + a \times b &\stackrel{?}{=} 0
\end{array}
$$

Notice how the relative wire in the first row for the pseudo column $C^+$ maps to the wire $\abst{c}$ in the next row of the column $C$.

Thus, the structural integrity of the gates hold if the equations evaluate to zero. If we were to use three multiplication gates instead, it will take three constraints instead of two.
\end{tcolorbox}

We now conclude this section on abstractions by defining $\Spec$, the penultimate single source of truth object.

\begin{definition}[Spec]
TODO
\end{definition}


\newcommand{\WireType}{\text{WireType}}

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
