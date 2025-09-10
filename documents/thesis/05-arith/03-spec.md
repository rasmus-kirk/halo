## Abstractions

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
\IndexMap(X: F(\Uni), Y: F(\Uni)) &= F(X(t,s) \to Y(t,s))
\end{array}
$$

- *notation*:
  - Naively, think of a thunk as $f: X \to Y$
    - $X$ is the thunk argument type.
      - $X = \Unit$ if the value is not a thunk.
    - $Y$ is the value type.
      - $Y= \Option(T)$ if the index map is partially populated.
    - Thus, index maps hold such $f$ where $X,Y$ vary per color and column.
  - If $t,s$ appears free in $F$, then it is bound to the indices
    - e.g. $F(T(t,s)) = (t: \Color) \to (s: \Column) \to T(t,s)$.
- *projections*: if $A: \IndexMap(X,Y)$ then 
  - $A^t_x(c) = A(t,c,x)$ for thunks
  - $A^t(c) = A(t,c,())$ for non thunks
- *operations*:
  - map; $-[-]: F(Y_1(t,s) \to Y_2(t,s)) \to \IndexMap(X, Y_1) \to \IndexMap(X, Y_2)$
    - $f[A]^t_x(c) = f(t,c,A^t_x(c))$
  - join; $- \sqcup_{-} -: \IndexMap(X,Y_1) \to F(Y_1(t,s) \to Y_2(t,s) \to Y_3(t,s)) \to \IndexMap(X,Y_2) \to \IndexMap(X,Y_3)$
    - $(A \sqcup_f B)^t_x(c) = f(t,c,A^t_x(c),B^t_x(c))$
    - $A \cat B = A \sqcup_{\cat} B$ where $A: \IndexMap(X, T^k), B: \IndexMap(X,T^{k'})$ 
- *motivation*: a succinct way to store and compose values that depend on color, column, and arbitrary argument type depending on the column, e.g. managing multiple wire types for plookup columns in trace table, and data structure for values for equations.

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

TODO

- gadget projection: cell wire; AWire
- gadget projection: cell
- column projection: default cell

\begin{definition}[Pre-Constraints]
The class of gates that a gadget of a specific properad contributes.
\end{definition}

- *motivation*: Pre-Constraints act as a template for a sub-table for gadgets of the properad. This makes the instantiations of gates in the concrete circuit derivable from the properads; a single source of truth.


TODO

- example
- relative wires
- gadget assert for relative
- op projects term: Eqn, columns: pow(column)
- spec
- DONE

\newcommand{\WireType}{\text{WireType}}

### Pre-Constraints

The *pre-constraints* $\ctrn_g$ of the operation $g$ is an index map of a vector of *cells*. Note that the vectors across different wire types $t$ need not be the same length, but within are length $k(t)$. Cells are defined in terms of a *reducer* type $R$ that computes a value for a wire type $W(t)$ given the thunk argument $X(t,s)$ and vector of concrete wire values selected from the input and output wires of the gadget[^sel-notation]. Columns irrelevant can be omitted and can be padded with default values from $D$. Cells have the forms tabulated below.

[^sel-notation]: Although the selection is notated $\avec{w}$, it is a vector of naturals indexing the wire types. Trace can use this to recover the wires from $\abst{f}$.

$$
\begin{array}{c}
\begin{array}{cccc}
g: \Ops &
\ctrn'_g: \PreTable_g &
\PreTable_g = \IndexMap(X, F(\Cell_g(t,s)^{k(t)})) &
\AWire_g = [n_g+m_g+1]\setminus \cdots
\end{array} 
\end{array}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
R_g(\avec{w}) &= F(X(t,s) \to W[\vec{t}^{g,\avec{w}}] \to W(t)) \\
\Cell_g &= F(X(t,s) \times (\avec{w}: \AWire_g^k) \times R_g(\avec{w}_p,t,s)) \\
\vec{t}^{g,\avec{w}} &= (\lambda i. (\tin{g} \cat \tout{g})_i)[\avec{w}] \\
\text{wires}^{\abst{f}}_g(\avec{w}) &= (\lambda i. (\gin(g) \cat \out(\abst{f}, g))_i)[\avec{w}] \\
\text{cw}&: \Cell_g \to \Nb \\
\text{cw}&= \lambda (\_, \avec{w}, \_). \maybe{i}{\avec{w} = (i)}
\end{array} &
\begin{array}{rl}
R_{\text{default}} &= F(X(t,s) \to W_s(t)) \\
\Cell_{\text{default}} &= F(X(t,s) \times \Unit \times R_{\text{default}}) \\
\text{Default} &= \IndexMap(X, \Cell_{\text{default}}) \\
D &: \text{Default} \\
D_k &= (\lambda t,\_, d. \text{repl}(d,k(t)))[D] \\
\ctrn_g &= D_k \sqcup_{\lambda \_, d,c.c} \ctrn_g'
\end{array}
\end{array}
$$

cell | notation | $X$ | $\AWire_g^k$ | $R_g$
-|-|-|-|-
constant | $c$ | $()$ | $()$ | $\lambda (). c$
wire | $\abst{w}$ | $()$ | $(\abst{w})$ | $\lambda (),w. w$
$\plookup \ \text{Tbl}_j$ | $\pcell(\avec{w})$ | $(d, \zeta)$ | $\avec{w}$ | $\lambda d,\zeta, \vec{w}. \pcell(\zeta, \vec{w}, j)$
$\plookup$ default | $\bot$ | $(d, \zeta)$ | $()$ | $\lambda d, \zeta. d$

**Pre-Constraint Example**

Let the pre-constraints for the gadget $\gpair{\ggtu{\text{Add}_p}{\abst{a}, \abst{b}}}{\abst{c}}, \gpair{\ggtu{\text{Mul}_p}{\abst{d}, \abst{c}}}{\abst{e}} \in \abst{f}$ be as follows where $A \cat B = A \sqcup_{\lambda \_, \vec{a}, \vec{b}. \vec{a} \cat \vec{b}} B$:

\begin{center}
\begin{tabular}{c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
\cline{2-19}
& \multicolumn{18}{|c|}{$\ctrn_{\text{Add}_p} \cat \ctrn_{\text{Mul}_p}$} \\
\cline{2-19}
& \multicolumn{9}{|c|}{$p$} & \multicolumn{9}{|c|}{$q$} \\
\cline{2-19}
& $A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_c$ & $PI$ & 
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_c$ & $PI$ \\
\hline\hline
$\text{Add}_p$ & $\abst{a}$ & $\abst{b}$ & $\abst{c}$ & 1 & 1 & -1 & 0 & 0 & 0 & \multicolumn{9}{|c}{} \\
\cline{1-10}
$\text{Mul}_p$ & $\abst{d}$ & $\abst{c}$ & $\abst{e}$ & 0 & 0 & -1 & 1 & 0 & 0 & \multicolumn{9}{|c}{} \\
\cline{1-10}
\end{tabular}
\end{center}
We notate $@_i \circ T^t$ as a *gate*; the row $i$ of wire type $t$ of the *trace table* $T$; resolved pre-constraints. Applying the gates to $F_{GC}^{\plonkm}$, we get zero iff the structure of the operation is respected.
\begin{center}
\begin{tabular}{c c}
\begin{math}
\begin{array}{c}
\begin{array}{rl}
F_{GC}^{\plonkm}: \Eqn &= A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI
\end{array} \\
\begin{array}{ccc}
\begin{array}{rl}
@ &: \Nb \to T^k \to T \\
@_i(\vec{y}) &= y_i
\end{array}
&
F_{GC}^{\plonkm}(@_1 \circ T^p) = a + b - c &
F_{GC}^{\plonkm}(@_2 \circ T^p) = -e + d \times c
\end{array}
\end{array}
\end{math} &
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{mul}{(0,0)}{$\abst{d}$,$\abst{c}$}{$\text{Mul}_p$}{1}
\gate{add}{($(mul.north)+(1,0)$)}{$\abst{a}$,$\abst{b}$}{$\text{Add}_p$}{1}
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] ($(add-in-2)+(0,0.25)$) -- (add-in-2);
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- (mul-in-1);
\draw[-,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\draw[-,thick] ($(add-out-1)+(0,-0.4)$) -- ($(add-out-1)+(-0.65,-0.4)$);
\draw[-,thick] ($(add-out-1)+(-0.65,-0.4)$) -- ($(add-out-1)+(-0.65,1.75)$);
\draw[-,thick] ($(add-out-1)+(-0.65,1.75)$) -- ($(mul-in-2)+(0,0.35)$);
\draw[-,thick] ($(mul-in-2)+(0,0.35)$) -- (mul-in-2);
\node[draw, thick, circle, double, double distance=1pt, anchor=north] at ($(mul-out-1)+(0,-0.4)$) {$\abst{e}$};
\draw[-,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.4)$);
\end{tikzpicture}
\end{tabular}
\end{center}

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
