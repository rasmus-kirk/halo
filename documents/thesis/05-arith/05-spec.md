### Abstractions

We now define the rest of the abstractions building up to the specification of the single source of truth for arithmetization which is used in $\text{trace}$.

\begin{definition}[Column]
A set of unique identifiers for the columns of the trace table / concrete circuit.
\end{definition}
$$
c: \Column
$$
\newcommand{\pcol}{\text{priv}}
\newcommand{\cccol}{\text{copy}}

- *notation*: $\Column = \set{A,B,C,Q_l,Q_r,Q_o,Q_m,Q_c,PI, \ldots}$
- *projections*:
  - $\pcol(c): \Bb$ - is $c$ a private / witness relevant column
  - $\cccol(c): \Bb$ - is $c$ a copy constraint relevant column
  - $X(t: \Color,c): \Uni$ - the argument type that the column values depend on
- *motivation*: a modular and user extendable way to define the structure of the trace table / concrete circuit. 

\begin{notation}[Unit Type]
A unit type is a type with a single value, denoted as $()$.
\end{notation}
$$
() : \Unit
$$

- *motivation*: Used as an identity / unital / nullary element.

\begin{definition}[Index Map]
A data structure that maps columns to thunks; functions of arbitrary type.
\end{definition}
$$
\begin{array}{rl}
F(T) &= \Color \to \Column \to T \\
\IndexMap(X: F(\Uni), Y: F(\Uni)) &= F(X(t,c) \to Y(t,c))
\end{array}
$$

- *notation*:
  - Naively, think of a thunk as $f: X \to Y$
    - $X$ is the thunk argument type.
      - $X = \Unit$ if the value is not a thunk.
    - $Y$ is the value type.
      - $Y= \Option(T)$ if the index map is partially populated.
    - Thus, index maps hold such $f$ where $X,Y$ vary per color and column.
  - If $t,c$ appears free in $F$, then it is bound to the indices
    - e.g. $F(T(t,c)) = (t: \Color) \to (c: \Column) \to T(t,c)$.
- *projections*: if $A: \IndexMap(X,Y)$ then 
  - $A^t_x(c) = A(t,c,x)$ for thunks
  - $A^t(c) = A(t,c,())$ for non thunks
- *operations*:
  - map; $-[-]: F(Y_1(t,c) \to Y_2(t,c)) \to \IndexMap(X, Y_1) \to \IndexMap(X, Y_2)$
    - $f[A]^t_x(c) = f(t,c,A^t_x(c))$
  - join; $- \sqcup_{-} -: \IndexMap(X,Y_1) \to F(Y_1(t,c) \to Y_2(t,c) \to Y_3(t,c)) \to \IndexMap(X,Y_2) \to \IndexMap(X,Y_3)$
    - $(A \sqcup_f B)^t_x(c) = f(t,c,A^t_x(c),B^t_x(c))$
    - $A \cat B = A \sqcup_{f} B$ where $A: \IndexMap(X, T^k), B: \IndexMap(X,T^{k'})$ and $f(\_,\_,\vec{a},\vec{b}) = \vec{a} \cat \vec{b}$
- *motivation*: a succinct way to store and compose values that depend on color, column, and an argument of arbitrary type, e.g. managing multiple wire types for plookup columns in trace table, and data structure for values for equations.

\begin{definition}[Equation]
An equation is a grammar that expresses a polynomial like structure over columns. 
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

- *projection*: $\text{foldEqn}: (-: T \to T) \to (+: T \to T \to T) \to (\times: T \to T \to T) \to (\times_\Fb: \Fb \to T \to T) \to \Eqn \to (\Column \to T) \to T$
  - when every operation of the grammar is specified for type $T$, we can evaluate the equation.
  - curve points do not have multiplication defined, thus equations involving them cant be evaluated.
- *notation*: if $X:\IndexMap(\Unit, T)$ and $X^t: \Column \to T$ then $E(X^t): T = \text{foldEqn}(\ldots, E, X^t)$
- *motivation*: Single source of truth for equational definitions that vary over operand types, e.g. scalars, polynomials, curve points, wires and state via build. Examples are gate constraint polynomials, grand product polynomials, quotient polynomial, plookup compression equation, etc.
- *implementation note*: it is possible to use traits and type variables / generics in rust to define a function over $T$, without having an explicit syntax tree construction of the $Eqn$'s grammar.

\begin{definition}[Cell Wire]
Cell wires are natural numbers that represent the wires of a gadget in a properad. Naively, an abstract wire. It does exclude some wires which will be defined later.
\end{definition}
\newcommand{\CWire}{\text{CellWire}}
\newcommand{\aavec}[1]{\bar{\vec{#1}}}
$$
\begin{array}{rl}
\CWire(\abst{g}) = [n(\abst{g}) + m(\abst{g}) + 1] \setminus \ldots
\end{array}
$$

- *notation*: $\bar{w}$ the bar denotes the cell wire is an abstraction of the wire $\abst{w}$; an abstraction of the value $w$.
- *motivation*: For properads to be a single source of truth in constructing the concrete circuit, it needs a way to represent the wires that gadgets that instantiate the properad has.
- *note*: This is a projection of a properad.

\begin{notation}[Vector index function]
A function that indexes a vector.
\end{notation}
$$
\begin{array}{rl}
- @ -&: T^k \to \Nb \to T \\
\vec{x} @ i &= x_i
\end{array}
$$

- *motivation*: Allows us to index vectors over a higher order function such as mapping over a vector.

\begin{definition}[More Properad Projections]\end{definition}
$$
\abst{g}: \Prpd
$$

- *projections*:
  - $\vec{t}(\abst{g}) = \pin(\abst{g}) \cat \pout(\abst{g})$ - the full profile of the properad; input and output wire colors.
  - $\vec{t}(\abst{g}, \aavec{w}) = (\vec{t}(\abst{g}) @ -)[\aavec{w}]$ - the profile of a vector of cell wires.
- *motivation*: Allows querying the profile of cell wires succinctly.

\begin{definition}[Cell Resolvers]
Functions that reduce a cell to a value.
\end{definition}
$$
R(\abst{g}, \aavec{w}) = F(X(t,c) \to W [\vec{t}(\abst{g}, \aavec{w})] \to W(t))
$$

- *motivation*: Think of a cell as a projection of a properad defining the single source of truth of an abstract value in the concrete circuit. The resolver instantiates it to a concrete value.
- *note*: This is a projection of a properad.

\begin{definition}[Cell]
Cells represent an abstract value in an index map modelling the trace table / concrete circuit. It contains the thunk argument, the cell wires that it depends on, and a resolver that computes the value of the cell. 
\end{definition}
$$
\begin{array}{rl}
\Cell(\abst{g}) &= F(X(t,c) \times (\aavec{w}: \CWire(\abst{g})) \times R(\abst{g}, \aavec{w},t,c)) \\
\boxdot &: \Cell(\abst{g})
\end{array}
$$

- *notation*: $\boxdot$ a boxed symbol denotes a cell.
- *motivation*: the following cell constructions will hopefully motivate the definitions above.
- *note*: This is a projection of a properad.

\begin{definition}[Constant Cell]
A cell that does not depend on any wire nor thunk argument.
\end{definition}
$$
\begin{array}{rl}
\boxed{c} &: \Cell(\abst{g}) = ((), (), f) \\
f() &= c
\end{array}
$$

- *motivation*: Concise notation for a cell that resolves to a constant value.

\begin{definition}[Wire Cell]
A wire cell that resolves to the value of a wire.
\end{definition}
$$
\begin{array}{rl}
\boxed{w} &: \Cell(\abst{g}) = ((), \bar{w}, f) \\
f(w) &= w
\end{array}
$$

- *motivation*: Concise notation for a cell that resolves to a wire value.

\newcommand{\cw}{\text{cellWire}}
- *projections*: $\cw((), \bar{w}, f) = \bar{w}$ - yields the cell wire of the wire cell. performing this operation on any other kind of cell will yield $\bot$. This is necessary for relative wires defined later.

\begin{example}[Lookup Cell]
The following is an example of a cell for a theoretical properad called $\text{Lookup}$ modelling plookup operations.
\end{example}
$$
\begin{array}{rl}
\boxed{\frak{p}(1,2,3)} &: \Cell(\text{Lookup}) = (\zeta, (1,2,3), f) \\
f(\zeta, a,b,c) &= a + \zeta b + \zeta^2 c \\
\end{array}
$$

- *motivation*: Notice how $\zeta: X(t,c)$ is a thunk argument, this is because we do not know $\zeta$ until the core plonk prover has sufficiently progressed in the transcript. But with our abstractions, we can account for it cleanly before the protocol even begins.

\begin{definition}[Default Cell]
A cell representing default values for a column.
\end{definition}
$$
\begin{array}{rl}
\boxed{-}&: \Cell(\abst{g}) = (x: X(t,c), (), f) \\
f(x) &: W[t]
\end{array}
$$

- *motivation*: For notational brevity, columns irrelevant to a properad may be omitted, though vectors of a specific color must maintain uniform length. Default cells are padded for omitted columns. For most columns, $f()= 0$, while specialized columns such as plookup use $f(t_{last}: X(t,c)) = t_{last}$, where $t_{last}$ denotes the last entry of the lookup table which can only be computed further in the protocol. This is yet another advantage of having thunk arguments in index maps.

\begin{definition}[Pre-Constraints]
The class of gates that a gadget of a specific properad contributes.
\end{definition}
$$
\begin{array}{rl}
\PreTable&: \Prpd \to \IndexMap(X, F(\Cell(\abst{g}, t,c)^{k(t)}))  \\
\ctrn(\abst{g})&: \PreTable(\abst{g})
\end{array}
$$

- *notation*: note we use $k(t)$ to denote that the vectors are of uniform length per color.
- *motivation*: Pre-Constraints act as a template for a sub-table for gadgets of the properad. This makes the instantiations of gates in the concrete circuit derivable from the properads; a single source of truth.

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

- *motivation*: This is the last intermediate data structure before we can interpolate them into polynomials and other data forming the concrete circuit $(R,X,W)$.

\begin{definition}[Gate]
A gate is the $i$th row of the trace table for color $t$.
\end{definition}
\newcommand{\gatef}{\text{gate}}
$$
\gatef(T, i) = (- @ i)[T]
$$

- *notation*: We side step the issue of index out of bounds, this can be managed by wrapping the result in an option type.
- *motivation*: A gate is a semantic group of resolved cells, specifically a row of a trace table. This generally corresponds to the operands of an equation typically the gate constraint polynomial.
- *note*: This is a projection of a trace table.

\begin{example} Pre-constraints for $\build{a + b}{}{}$ and $\build{a \times b}{}{}$. Let $\text{Add}^t, \text{Mul}^t: \Prpd$
\end{example}

\begin{center}
\begin{tabular}{ c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[minimum width=2cm, minimum height=1.5cm] (tab) {
\begin{tabular}{|c|c|c|c|c|c|}
\hline
$\abst{g}$ & $n$ & $m$ & $\pin$ & $\pout$ & $\eval(a,b)$ \\
\hline
$\text{Add}^t$ & $2$ & $1$ & $(t,t)$ & $t$ & $a + b$ \\
\hline
$\text{Mul}^t$ & $2$ & $1$ & $(t,t)$ & $t$ & $a \times b$ \\
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
\begin{tabular}{c c}
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
&
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
\end{center}

- *note*:
  - The pre-constraints only define a gate for the color $t$ wheras the rest of the colors is empty.
  - Recall that the length of the vector of cells must be uniform within a color, but not across colors.

Let the trace table for $\build{w_1 + (w_2 \times w_3) = z^*}{}{}$ be the following (we will describe how this is computed when we define trace later):
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
Let $F_{GC}^{\plonkm}: \Eqn = A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI$, thus:

$$
\begin{array}{rll}
F_{GC}^{\plonkm}(\gatef(T,1,q)) &= w_2 + w_3 - t &\stackrel{?}{=} 0 \\
F_{GC}^{\plonkm}(\gatef(T,2,q)) &= -z + (w_1 \times t) &\stackrel{?}{=} 0 \\
\end{array}
$$

Thus $F_{GC}^{\plonkm}$ implies the structural integrity of the gadgets when it evaluates to zero.

At this point, we want to emphasize the expressivity of index map as an abstraction. Pre-constraints, trace table and gates are all defined as an index map. Subsequently, we will see that even the circuit $(R,X,W)$ are also index maps.

TODO

- relative wires
- cell wire additional assertion
- gadget additional assertion
- properad projections; term: Eqn, columns: pow(column)
- spec
- DONE

\newcommand{\WireType}{\text{WireType}}

### Relative Wires

An operation is called *relative* if it has $b_g > 0$. The last $b_g$ input wires are *relative wires* which are wires from another gadget called the *base gadget*. Thus, they are excluded in $\ctrn_g$ via $\AWire_g$. A base gadget's constraints will appear immediately after its relative gadget[^dupe]. $\Rel_g$ declares the columns that, the relative gadget can reference from its base gadget. Thus, constructing a relative gadget checks that the relative wires must exist in the first row of the base gadget's constraints in the declared positions in $\Rel_g$.

[^dupe]: If different relative gadgets have the same base gadget, there will be duplicate rows of the base.


$$
\begin{array}{cc}
\begin{array}{rl}
g &: \Ops \\
b_g &: \set{x : \Nb \middle\vert x \leq n_g} \\
\AWire_g &= [n_g+m_g+1]\setminus[n_g-b_g+1..n_g+1] \\
\Rel_g &: \pset{\Column}
\end{array} &
\begin{array}{rl}
\base &: \AbsCirc \to \Ggt \to \Ggt \\
\base^{\abst{f}}_g &= \base(\abst{f}, \gin(g)[n_g-b_g+1..n_g+1]) \\
\base(\abst{f}, \avec{w}) &= \maybe{g}{
\begin{array}{l}
  \exists i. \gpair{g}{\abst{w}_i} \in \abst{f} \\
  \bigwedge(\lambda \abst{w}. \abst{w} \in \gin(g) \cup \out(\abst{f},g))[\avec{w}] \\
  \bigwedge(\lambda \abst{w}. \text{pos}(\abst{f}, g, \abst{w}) \neq \emptyset)[\avec{w}]
\end{array}} 
\end{array}
\end{array}
$$
$$
\begin{array}{cc} 
\begin{array}{rl}
\text{pos} &: \AbsCirc \to \Ggt \to \Wire \to \pset{\Column} \\
\text{pos}(\abst{f}, g,\abst{w}) &= \maybe{s \in \Rel_g}{
\begin{array}{l}
\avec{v} = \gin(g) \cat \out(\abst{f},g) \\
\abst{v}_{\text{cw}\circ @_1 \circ \ctrn_g^{\ty(\abst{w})}(s)} = \abst{w}
\end{array}}
\end{array} &
\begin{array}{rl}
- ( - ) &: (g: \Ops) \to \Wire^{n_g} \to (\abst{f}: \AbsCirc) \to \Ggt \\
g(\avec{x} \cat \avec{r})_{\abst{f}} &= \maybe{g'}{
\begin{array}{l}
\cdots \land |\avec{r}| = b_g \\
b_g > 0 \implies \base(\abst{f}, \avec{r}) \neq \bot \\
\end{array}}
\end{array}
\end{array}
$$

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
