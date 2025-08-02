Before defining trace, we need to define the rest of the abstractions. 

**Equations**

$\Surkal$ performs vanishing argument on $F_{GC}$; the *gate constraint polynomial*. $F_{GC}$ is defined as an *abstract equation* structured as an abstract syntax tree. The atomics are *columns* and *scalars* $\Fb$. This lets us define a function for polymorphic types in place of the columns: scalars $\Fb_p$, polynomials $\Fb_p[X]$, elliptic curve points[^curve-mul] $E[\Fb]$ and wires $\abst{w}$ (with $\AState$) in one definition, the latter being synonymous to circuit building. Computing the function then is tree traversal with combinators for each equation operation in the grammar.

[^curve-mul]: Multiplication with curve points does not exist. Thus some $\Eqn$ aren't defined for curve points.


\vspace{1em}
\begin{center}
\begin{tabularx}{\textwidth}{X X X}
\begin{grammar}
<Eqn> ::= \lit{-} \synt{Eqn2}
\alt \lit{(} \synt{Eqn2} \lit{)}
\alt \synt{Column}
\end{grammar} &
\begin{grammar}
<Eqn1> ::= \synt{Eqn} \synt{Eqn1'}
\alt \synt{Scalar} \lit{×} \synt{Eqn2}

<Eqn1'> ::= \lit{×} \synt{Eqn} \synt{Eqn1'}
\alt $\epsilon$
\end{grammar} &
\begin{grammar}
<Eqn2> ::= \synt{Eqn1} \synt{Eqn2'}

<Eqn2'> ::= \lit{+} \synt{Eqn1} \synt{Eqn2'}
\alt \lit{-} \synt{Eqn1} \synt{Eqn2'}
\alt $\epsilon$
\end{grammar}
\end{tabularx}
\end{center}
$$
\begin{array}{cc}
\begin{array}{rl}
\text{foldEqn}_i &: (T \to T) \to (T \to T \to T) \to (T \to T \to T) \to (\Fb \to T \to T) \\
&\to \Eqn \to (\Column \pto T) \to T
\end{array} &
e(C) = \text{foldEqn}(-,+,\times,\times_\Fb,e,C)
\end{array}
$$

Every operation $g$ declares the columns $\Column_g$ and a $\term_g$ of $F_{GC}$. 
$$
\begin{array}{ccc}
g:\Ops &
\Column_g: \pset{\Column} &
\term_{g}: \Eqn
\end{array}
$$
e.g. in basic \plonk we only have a single term
$$
F_{GC}^{\plonkm}: \Eqn = A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI
$$

**Index Map**

An *index map*[^index-map-notation] maps wire types and columns to thunks of $Y$ of argument $X$; most have no arguments except for $\plookup$ columns. We also define map $-[-]$ and join $\sqcup$ with $F(T)$ as a function type from the indices to $T$.[^free-F] If $Y(t,s)$ is a vector, then you can think of an index map as a table.

[^free-F]: If $t,s$ appears free in $F$, then it is bound to the indices. i.e. $F(T(t,s)) = (t: \WireType) \to (s: \Column) \to T(t,s)$.


[^index-map-notation]: Let $C:\IndexMap(X,Y)$, then $C^q(A)$ is short for $C(q,A,())$ and $C^q_\xi(f)$ is short for $C(q,f,\xi :W(q))$ if $X(q,f) = W(q)$

$$
\begin{array}{ccc}
F(T) = \WireType \to \pset{\Column} \to T &
X_{s: \Spec}: F(\Uni)
\end{array}
$$
$$
\begin{array}{c}
\IndexMap = (X,Y: F(\Uni)) \to F(\Option(X(t,s) \to Y(t,s))) 
\end{array}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
-[-] &: F(Y_1(t,s) \to Y_2(t,s)) \\
&\to \IndexMap(X,Y_1) \to \IndexMap(X,Y_2) \\
f[A] &= \lambda t. f(t)[A(t)] \\
f[a] &= \begin{cases}
& \exists s. a(s) \neq \bot \\
& a' = f[a[s \mapsto \bot]] \\
a'[s \mapsto y] & y = \lambda x. f(s,a(s,x)) \\
\bot & \otherwise
\end{cases}
\end{array} &
\begin{array}{c}
\begin{array}{rl}
\sqcup &: \IndexMap_s(X,Y_1) \to F(Y_1(t,s) \to Y_2(t,s) \to Y_3(t,s)) \\
& \to \IndexMap_s(X,Y_2) \to \IndexMap_s(X,Y_3) \\
A \sqcup_f B &= \lambda t. A(t) \sqcup_{f(t)} B(t) \\
a \sqcup_f b &= \begin{cases}
a \sqcup^s_f b [s \mapsto a f_s b] & \exists s. a(s) \neq \bot \land b(s) \neq \bot \\
a \sqcup^s_f b [s \mapsto a(s)] & \exists s. a(s) \neq \bot \\
a \sqcup^s_f b [s \mapsto b(s)] & \exists s. b(s) \neq \bot \\
\bot & \otherwise
\end{cases}\\
a \sqcup^s_f b &= a[s \mapsto \bot] \sqcup_f b[s \mapsto \bot] \\
a f_s b &= \lambda x. f(s,a(s,x), b(s,x))
\end{array}
\end{array}
\end{array}
$$

**Pre-Constraints**

The *pre-constraints* $\ctrn_g$ of the operation $g$ is an index map of a vector of *cells*. Note that the vectors across different wire types $t$ need not be the same length. Cells are defined in terms of a *reducer* type $R$ that computes a value for a wire type $W(t)$ given the thunk argument $X(t,s)$ and vector of concrete wire values selected from the input and output wires of the gadget[^sel-notation].

[^sel-notation]: Although the selection is notated $\avec{w}$, it is a vector of naturals indexing the wire types. Trace can use this to recover the wires from $\abst{f}$.

$$
\begin{array}{cc}
\begin{array}{c}
\begin{array}{ccc}
g: \Ops &
\ctrn_g: \PreTable_g &
\PreTable_g = \IndexMap(X, \lambda t,s. \Cell_g(t,s)^k)
\end{array} \\
\begin{array}{ccccc}
\Cell_g = F(X(t,s) \times (\avec{w}: \AWire_g^k) \times R_g(\avec{w}_p,t,s)) &
R_g(\avec{w}) = F(X(t,s) \to W[\vec{t}^{g,\avec{w}}] \to W(t))
\end{array} \\
\begin{array}{cc}
\AWire_g = [n_g+m_g+1]\setminus \cdots &
\vec{t}^{g,\avec{w}} = (\lambda i. (\tin{g} \cat \tout{g})_i)[\avec{w}]
\end{array}
\end{array} &
\begin{array}{rl}
\text{wnat} &: \Cell_g \to \Nb \\
\text{wnat} &= \lambda (\_, \avec{w}, \_). \\
&\maybe{i}{\avec{w} = (i)}
\end{array}
\end{array}
$$

Typically a cell is of the following forms:

cell | notation | $X$ | $\AWire_g^k$ | $R_g$
-|-|-|-|-
constant | $c$ | $()$ | $()$ | $\lambda (). c$
wire | $\abst{w}$ | $()$ | $(\abst{w})$ | $\lambda (),w. w$
$\plookup$ cell[^plookupdefn] | $\top$ | $(\_, \xi)$ | $\avec{w}$ | $\lambda \_,\xi,\vec{w}. \cdots$
$\plookup$ default cell | $\bot$ | $(d,\_)$ | $\_$ | $\lambda d,\_.d$

[^plookupdefn]: Defining and motivating $\plookup$ cells is deferred to when we define the lookup chips

**Pre-Constraint Example**

Let the pre-constraints for the gadget $\gpair{\ggtu{\text{Add}_p}{\abst{a}, \abst{b}}}{\abst{c}}, \gpair{\ggtu{\text{Mul}_p}{\abst{d}, \abst{c}}}{\abst{e}} \in \abst{f}$ be as follows:

\begin{center}
\begin{tabular}{c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
\cline{2-19}
& \multicolumn{18}{|c|}{$\ctrn_{\text{Add}_p}$} \\
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
We notate $@_i[T]^t$ as a *gate*; the row $i$ of wire type $t$ of the *trace table* $T$; resolved pre-constraints. Applying the gates to $F_{GC}^{\plonkm}$ from the previous example, we can see that it will resolve to zero only if the structure of the operation is respected. This motivates how the vanishing argument enforces the structure of the gadgets.
\begin{center}
\begin{tabular}{c c}
\begin{math}
\begin{array}{c}
\begin{array}{rl}
F_{GC}^{\plonkm}: \Eqn &= A \times Q_l + B \times Q_r + C \times Q_o + A \times B \times Q_m + Q_c + PI
\end{array} \\
\begin{array}{ccc}
\begin{array}{rl}
@ &: \Nb \to F(T^k \to T) \\
@_i(\_,\vec{y}) &= y_i
\end{array}
&
F_{GC}^{\plonkm}(@_1[T]^p) = a + b - c &
F_{GC}^{\plonkm}(@_2[T]^p) = d \times c - e
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

**Loop**

From $\ctrn_g$, we can also populate the *loop*; a vector modelling an equivalence class of positions modulo wire. This is used in copy constraints where $\text{CC}$ are columns that will be copy constrained.
$$
\begin{array}{ccc}
\text{Coord} = \text{CC} \times \Nb &
\text{CLoopN} = (\abst{w}: \Nb) \pto \text{Coord}^k
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{coords} &: (\Nb \to \text{Coord}^m) \to \text{CC} \\
&\to \Nb \to \Cell_g \to \Nb \to \text{Coord}^k \\
\text{coords} &= \lambda (l,s,o,c,j). \\
&l(j) \cat \left[ ((s,i+o)) \middle\vert \text{wnat}(c) = j \right] ? ()
\end{array} &
\begin{array}{rl}
\text{loopN} &: \Nb \to \PreTable_g \to \Wire^{n_g +m_g} \to \Nb \to \text{CLoopN} \\
\text{loopN} &= \lambda(o, (t, s, x, \vec{p}), \avec{w}, i). \\
&\begin{cases}
\bot & \vec{p} = () \lor x \neq () \\
& \vec{p} = p \cat \vec{p}' \land l = \text{loopN}(o,t,s,x,\vec{p}',\avec{w},i+1) \\
l[\avec{w} \mapsto \vec{s}]
& \vec{s} = \text{coord}(\lambda j. l(\abst{w}_j) ? (),s,o,p)[n_g+m_g+1] \\
\end{cases}
\end{array}
\end{array}
$$

**Relative Wires**

An operation is called *relative* if it has $b_g > 0$. The last $b_g$ input wires are *relative wires* which are wires from another gadget called the *base gadget*. Thus, they are excluded in $\ctrn_g$ via $\AWire_g$. A base gadget's constraints will appear immediately after its relative gadget[^dupe]. $\Rel_g$ declares the columns that, the relative gadget can reference from its base gadget. Thus, constructing a relative gadget checks that the relative wires must exist in the first row of the base gadget's constraints in the declared positions in $\Rel_g$.

[^dupe]: If different relative chips have the same base chip, there will be duplicate base chip rows such that both relative chips have a copy


$$
\begin{array}{cc}
\begin{array}{rl}
g &: \Ops \\
b_g &: \set{x : \Nb \middle\vert x \leq n_g} \\
\AWire_g &= [n_g+m_g+1]\setminus[n_g-b_g+1..n_g+1] \\
\Rel_g &: \pset{\Column}
\end{array} &
\begin{array}{rl}
\base &: \AbsCirc \to \Wire^k \to \Ggt \\
\base(\abst{f}, \avec{w}) &= \maybe{g}{
\begin{array}{l}
  \exists i. (g,\abst{w}_i) \in \abst{f} \\
  \bigwedge(\lambda \abst{w}. \abst{w} \in \gin(g) \cup \out(\abst{f},g))[\avec{w}] \\
  \bigwedge(\lambda \abst{w}. \text{pos}(\abst{f}, g, \abst{w}) \neq \emptyset)[\avec{w}]
\end{array}} \\
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
\abst{v}_{\text{wnat}\circ @_1 \circ \ctrn_g^{\ty(\abst{w})}(s)} = \abst{w}
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

**Canonical Program**

$$
\eval_g: W[\tin{g}] \to W[\tout{g}]
$$

*Canonical programs* are how the values of output wires are computed from the values of input wires. e.g. in $\eval_{\text{Mul}_p}(x,y) = x \times y$. Moreover, due to the way relative wires are defined as input wires, we have them in the canonical program too. e.g. $\eval_{\text{CMul}_p}(d,e,c) = d \times e \times c$.

**Spec**

A *specification*[^spec-benefit] defines the config of the protocol. This includes marking columns as private or enabling it for copy constraints. In the previous section on arithmetize, we omitted $s:\Spec$ in $\AState$ leaving $W, \WireType, \Ops$ implicit for $W_s, \WireType_s, \Ops_s$. We will keep the spec instance $s$ implicit beyond this section as well.

[^spec-benefit]: With spec as a data structure, it is dynamic and can be extended whilst arithmetizing.


$$
\begin{array}{ccccc}
s : \Spec &
X_s: F(\Uni) &
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

In summary, these are the structures and their uses

abstraction level | atomics | semantic groups | structure | use
-|-|-|-|-
0 | $w: W(t)$ | $@_i[T]^t: \text{Gate}$ | $T: \TraceTable$ | protocol
1 | $\abst{w}: \Wire$ | $g: \Ggt$ | $\abst{f}: \AbsCirc$ | arithmetization
2 | $\boxdot : \Cell$ | $\ty(g): \Ops$ | $s: \Spec$ | config

**Trace Table Update Behaviours**

When adding constraints for a gadget, depending the the kind of operation, we will update the trace table in the following ways:

\begin{tabularx}{\textwidth}{@{} Y Y Y Y Y @{}}
\toprule
Basic & Relative  & Asserts  & PublicInput  & LookupTable  \\
 & $b_g>0$ & $m_g=0$ & $\ty(g)=\text{PI}_t$ & $\ty(g)=\text{Tbl}_t$
\\\hline 
append &
append with base &
appends after trace &
prepends after trace &
as column after trace
\\\hline \\
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, anchor=north] (b3) at (b2.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, pattern=north west lines, anchor=north] (b3) at (b2.south) {$\base_g$};
\node[draw, minimum width=1.5cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
& 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, anchor=north] (b2) at (b1.south) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, anchor=north] (b3) at (b2.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, anchor=north] (b4) at (b3.south) {$g_k$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm, pattern=north east lines] (b1) at (0,0) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, anchor=north] (b2) at (b1.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, anchor=north] (b3) at (b2.south) {$g_k$};
\node[draw, minimum width=1.5cm,minimum height=0.75cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=2.25cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern= north west lines] (b2) at (b1.north east) {$g_1$};
\node[draw, minimum width=0.6cm, minimum height=0.73cm, anchor=north west, pattern= north east lines] (b3) at (b2.south west) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern= north west lines] (b4) at (b3.south west) {$g_k$};
\end{tikzpicture} \\
\\\toprule
\end{tabularx}