**Spec**

Before defining trace, we need the complete definition for operations. $\Surkal$ performs vanishing argument on $F_{GC}$; the *gate constraint polynomial*. $F_{GC}$ is defined as an *abstract equation*: a function from a vector of polymorphic type *arguments* to the type itself. This lets us define a function to be used for scalars, polynomials, elliptic curve points[^curve-mul] and wires (with $\AState$) in one definition, the latter being synonymous to an abstract circuit.

[^curve-mul]: When $\Arg = E[\Fb_p]$, multiplication with a field element does not exist. Thus some $\Eqn$ aren't defined for curve points.

$$
\begin{array}{cccc}
\Eqn_k = \Arg^k \to \Arg
&
+ : \Arg \to \Arg \to \Arg
&
\times : \Fb_p \to \Arg \to \Arg
&
\times : \Arg \to \Arg \to \Arg
\end{array}
$$

The primitive terms in $F_{GC}$ are called *slots* and *selectors*. Slots: $A,B,C,\cdots$, hold concrete values of a circuit's wires privately, whereas selectors: $Q_l, Q_r, Q_o, \cdots$, are public values modelling the structure of the abstract circuit. Slots can be used by all operations, however selectors are operation specific. Groups of operations that use the same selectors are called *operation groups* $\Grp$ where $\Ops_{\Grp}$ are the operations and $\term_{\Grp}$ is the $F_{GC}$ term the group contributes to.

$$
\begin{array}{cccc}
\Grp: \OpGroup &
\Ops_{\Grp}: \pset{\Ops} &
\Selector_{\Grp}: \Selector^j &
\term_{\Grp}: \Eqn_{i+j}
\end{array}
$$

The collection of all operation groups is called the *operation collection* $\Col$ where $\OpGroup_{\Col}$ are the operation groups. The rest of the definitions are accumulations of the operation group's methods.

$$
\begin{array}{ccc}
\begin{array}{rl}
\Col&: \OpCollection \\
\OpGroup_{\Col} &: \pset{\OpGroup} \\
\Slot_{\Col} &: \Slot^i
\end{array} &
\begin{array}{rl}
\Ops_{\Col} &= \bigcup\limits_{\Grp \in \OpGroup_{\Col}} \Ops_{\Grp} \\
\Selector_{\Col} &= \bigcup\limits_{\Grp \in \OpGroup_{\Col}} \Selector_{\Grp}
\end{array} &
\begin{array}{l}
F_{GC}(\vec{s}, \vec{S}) \\= \sum\limits_{\Grp_i \in \OpGroup_{\Col}} \term_{\Grp_i}(\vec{s}, \vec{S}_i)
\end{array}
\end{array}
$$

A *specification*[^spec-benefit] defines a $\OpCollection$. In the previous section on arithmetize, we omitted $s:\Spec$ in $\AState$ leaving $W, \WireType, \Ops$ implicit for $W_s, \WireType_s, \Ops_s$. Moreover, it does not need to know $\term_{\Grp}, \Selector_{\Grp}, \Slot_{\Col}$ as these are specific to trace. We leave the spec instance $s$ and its conversion implicit for brevity[^more-brevity].

[^more-brevity]: We can notationally index with spec directly to get fields of gate collection e.g. $\Ops_{s} = \Ops_{\Col_s}$

$$
\begin{array}{cccc}
s : \Spec &
\OpCollection_s : \OpCollection
&
\WireType_s: \Uni
&
W_s: \WireType_s \to \Uni
\end{array}
$$

In summary, spec can be visualized as follows:

\begin{center}
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\tikzset{v/.style={draw, rounded corners, anchor=north}}
\node[v] (spec) at (0,0) {$s: \Spec$};

\node[anchor=east] (X) at ($(spec.west)+(-1,0)$) {$\cdots$};

\node[v] (wty) at ($(spec.south)+(-4,-0.5)$) {$\WireType_s$};
\node[v] (t1) at ($(wty.south)+(0,-0.25)$) {$t_1$};
\node[v] (t2) at ($(t1.south)+(0,-0.25)$) {$t_2$};

\draw[-] (wty.south) -- (t1.north);
\draw[-] (t1.south) -- (t2.north);
\draw[-,dashed] (t2.south) -- ($(t2.south)+(0,-0.5)$);

\node[v] (w) at ($(spec.south)+(-2,-0.5)$) {$W_s$};
\node[v] (w1) at ($(w.south)+(0,-0.25)$) {$W(t_1)$};
\node[v] (w2) at ($(w1.south)+(0,-0.25)$) {$W(t_2)$};

\draw[-] (w.south) -- (w1.north);
\draw[-] (w1.south) -- (w2.north);
\draw[-,dashed] (w2.south) -- ($(w2.south)+(0,-0.5)$);

\draw[|->] ($(t1.east)+(0.25,0)$) -- ($(w1.west)+(-0.25,0)$);
\draw[|->] ($(t2.east)+(0.25,0)$) -- ($(w2.west)+(-0.25,0)$);

\node[v] (col) at ($(spec.south)+(0,-0.5)$) {$\OpCollection_s$};
\node[v] (slot) at ($(col.south)+(0,-0.25)$) {$\Slot_s$};
\node[v] (A) at ($(slot.south)+(0,-0.25)$) {$A$};
\node[v] (B) at ($(A.south)+(0,-0.25)$) {$B$};
\node[v] (C) at ($(B.south)+(0,-0.25)$) {$C$};

\draw[-] (slot.south) -- (A.north);
\draw[-] (A.south) -- (B.north);
\draw[-] (B.south) -- (C.north);
\draw[-,dashed] (C.south) -- ($(C.south)+(0,-0.5)$);

\node[v,anchor=west] (G) at ($(spec.east)+(4.5,0)$) {$\OpGroup_{s}$};
\node[v] (G1) at ($(G.south |- slot.north)$) {$\Grp_1$};
\node[v,anchor=east] (Sel1) at ($(G1.west)+(-0.5,0)$) {$\Selector_{\Grp_1}$};
\node[v,anchor=north] (term1) at ($(G1.south)+(1,-0.25)$) {$\term_{\Grp_1}$};
\node[v,anchor=west] (GTy1) at ($(G1.east)+(1,0)$) {$\Ops_{\Grp_1}$};
\draw[->] (G1.west) -- (Sel1.east);
\draw[->] (G1.east) -- (term1.north);
\draw[->] (G1.east) -- (GTy1.west);
\node[v] (G2) at ($(G1.south)+(0,-1)$) {$\Grp_2$};
\node[v,anchor=east] (Sel2) at ($(G2.west)+(-0.5,0)$) {$\Selector_{\Grp_2}$};
\node[v,anchor=north] (term2) at ($(G2.south)+(1,-0.25)$) {$\term_{\Grp_2}$};
\node[v,anchor=west] (GTy2) at ($(G2.east)+(1,0)$) {$\Ops_{\Grp_2}$};
\draw[->] (G2.west) -- (Sel2.east);
\draw[->] (G2.east) -- (term2.north);
\draw[->] (G2.east) -- (GTy2.west);

\node[v,anchor=east] (Q1) at ($(Sel1.west)+(-0.5,0)$) {$Q_1$};
\node[v] (QK1) at ($(Q1.south)+(0,-0.25)$) {$Q_{k_1}$};
\node[v] (QK1S) at ($(QK1.south)+(0,-0.25)$) {$Q_{k_1 + 1}$};
\node[v] (QK2) at ($(QK1S.south)+(0,-0.25)$) {$Q_{k_2}$};

\draw[->] (Sel1.west) --(Q1.east);
\draw[->] (Sel1.west) -- (QK1.east);
\draw[->] (Sel2.west) -- (QK1S.east);
\draw[->] (Sel2.west) -- (QK2.east);
\draw[-,dashed] (Q1.south) -- (QK1.north);
\draw[-] (QK1.south) -- (QK1S.north);
\draw[-,dashed] (QK1S.south) -- (QK2.north);
\draw[-,dashed] (QK2.south) -- ($(QK2.south)+(0,-0.5)$);

\draw[-] (G.south) -- (G1.north);
\draw[-] (G1.south) -- (G2.north);
\draw[-,dashed] (G2.south) -- ($(G2.south)+(0,-0.5)$);

\node[v,anchor=west] (g1) at ($(GTy1.east)+(0.5,0)$) {$g_1$};
\node[v] (gj1) at ($(g1.south)+(0,-0.25)$) {$g_{j_1}$};
\node[v] (gj1s) at ($(gj1.south)+(0,-0.25)$) {$g_{j_1 + 1}$};
\node[v] (gj2) at ($(gj1s.south)+(0,-0.25)$) {$g_{j_2}$};

\draw[->] (GTy1.east) -- (g1.west);
\draw[->] (GTy1.east) -- (gj1.west);
\draw[->] (GTy2.east) -- (gj1s.west);
\draw[->] (GTy2.east) -- (gj2.west);
\draw[-,dashed] (g1.south) -- (gj1.north);
\draw[-] (gj1.south) -- (gj1s.north);
\draw[-,dashed] (gj1s.south) -- (gj2.north);
\draw[-,dashed] (gj2.south) -- ($(gj2.south)+(0,-0.5)$);

\draw[->] (col.south) -- (slot.north);
\draw[->] (col.north) -- (G.west);

\draw[->] (spec.west) -- (X.east);
\draw[->] (spec.south) -- (wty.north);
\draw[->] (spec.south) -- (w.north);
\draw[->] (spec.south) -- (col.north);

\node[anchor=north] (FGC) at ($(term1.north |- col.north)$) {$F_{GC}$};
\node[anchor=north] (GTys) at ($(g1.north |- col.north)$) {$\Ops_s$};
\node[anchor=north] (Sels) at ($(Q1.north |- col.north)$) {$\Selector_s$};
\end{tikzpicture}
\end{center}

[^spec-benefit]: With spec as a data structure, it is dynamic can be extended whilst arithmetizing.


**Index Map**

An *index map*[^index-map-notation] maps wire types and slots or selectors to thunks of $Y$ of argument[^not-spec] $X$; most have no arguments except for $\plookup$ columns. We also define map $-[-]$ and join $\sqcup$ with $F(T)$ as a function type from the indices to $T$.[^free-F]

[^free-F]: If $t,s$ appears free in $F$, then it is bound to the indices. i.e. $F(T(t,s)) = (t: \WireType) \to (s: \SlotNSelector) \to T(t,s)$.


[^index-map-notation]: Let $C:\IndexMap(X,Y)$, then $C^q(A)$ is short for $C(q,A,())$ and $C^q_\xi(f)$ is short for $C(q,f,\xi :W(q))$ if $X(q,f) = W(q)$


[^not-spec]: This may not necessarily be $X_s$ from $s: \Spec$, but if it is not specified, we may assume it is $X_s$.

$$
\begin{array}{ccc}
F(T) = \WireType \to \SlotNSelector \to T &
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

The *pre-constraints* $\ctrn_g$ of the operation $g$ is an index map of a vector of *pre-values*. Note that the vectors across different wire types $t$ need not be the same length. Pre-values are defined in terms of a *reducer* type $R$ that computes a value for a wire type $W(t)$ given the thunk argument $X(t,s)$ and vector of concrete wire values; selected from the input and output wires of the gate[^sel-notation].

[^sel-notation]: Although the selection is notated $\avec{w}$, it is a vector of naturals indexing the wire types. Trace can use this to recover the wires from $\abst{f}$.

$$
\begin{array}{cc}
\begin{array}{c}
\begin{array}{ccc}
g: \Ops &
\ctrn_g: \PreTable_g &
\PreTable_g = \IndexMap(X, \lambda t,s. \Pre_g(t,s)^k)
\end{array} \\
\begin{array}{ccccc}
\Pre_g = F(X(t,s) \times \AWire_g^k \times R_g(\avec{w}_p,t,s)) &
R_g(\avec{w}) = F(X(t,s) \to W[\vec{t}^{g,\avec{w}}] \to W(t))
\end{array} \\
\begin{array}{cc}
\AWire_g = [1..n_g+m_g+1]\setminus \cdots &
\vec{t}^{g,\avec{w}} = (\lambda i. (\tin{g} \cat \tout{g})_i)[\avec{w}]
\end{array}
\end{array} &
\begin{array}{rl}
\text{wnat} &: \Pre_g \to \Nb \\
\text{wnat} &= \lambda (\_, \avec{w}, \_). \\
&\maybe{i}{\avec{w} = (i)}
\end{array}
\end{array}
$$

Typically a pre-value is of the following forms:

pre-value | notation | $X$ | $\AWire_g^k$ | $R_g$
-|-|-|-|-
constant | $c$ | $()$ | $()$ | $\lambda (). c$
wire | $\abst{w}$ | $()$ | $(\abst{w})$ | $\lambda (),w. w$
$\plookup$ cell[^plookupdefn] | $\top$ | $(\_, \xi)$ | $\avec{w}$ | $\lambda \_,\xi,\vec{w}. \cdots$
$\plookup$ default cell | $\bot$ | $(d,\_)$ | $\_$ | $\lambda d,\_.d$

[^plookupdefn]: Defining and motivating $\plookup$ cells is deferred to when we define the lookup gates


e.g. the pre-constraints for the gate $(\text{Add}_p(\abst{a}, \abst{b}), \abst{c}) \in \abst{f}$

\begin{center}
\begin{tabular}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{14}{|c|}{$\ctrn_{\text{Add}_p}$} \\
\hline
\multicolumn{7}{|c|}{$p$} & \multicolumn{7}{|c|}{$q$} \\
\hline
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & 
$A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ \\
\hline\hline
$\abst{a}$ & $\abst{b}$ & $\abst{c}$ & 1 & 1 & -1 & 0 & \multicolumn{7}{|c}{} \\
\cline{1-7}
\end{tabular}
\end{center}

Let $t:\Eqn_7 = A \cdot Q_l + B \cdot Q_r + C \cdot Q_o + A \cdot B \cdot Q_m$, thus when the reducers are resolved, we have $t=a+b-c=0$.

This motivates how the vanishing argument enforces the structure of the gate if $t$ were a term of $F_{GC}$

TODO: elaborate on loop (copy constraints here instead of trace cc)

**Relative Wires**

An operation is called *relative* if it has $b_g > 0$; the number of relative wires. The last $b_g$ input wires are *relative wires* which are wires from another chip called the *base chip*. Thus, they are excluded in $\ctrn_g$ via $\AWire_g$. A base chip's constraints will appear immediately before its relative chip[^dupe]. $\Rel_g$ declares the slots or selectors that, the relative chip can reference from its base chip. Thus, constructing a relative gate checks that the relative wires must exist in the last row of the base chip's constraints in the declared positions in $\Rel_g$.

[^dupe]: If different relative chips have the same base chip, there will be duplicate base chip rows such that both relative chips have a copy


$$
\begin{array}{cc}
\begin{array}{rl}
g &: \Ops \\
b_g &: \set{x : \Nb \middle\vert x \leq n_g} \\
\AWire_g &= [1..n_g+m_g+1]\setminus[n_g-b_g+1..n_g+1] \\
\Rel_g &: \pset{\SlotNSelector} \\
\Rel_{\Grp} &= \bigcup\limits_{g \in \Ops_{\Grp}} \Rel_g 
\end{array} &
\begin{array}{rl}
\base &: \AbsCirc \to \Wire \to \Chip \\
\base(\abst{f}, \avec{w}) &= \maybe{g}{
\begin{array}{l}
  \exists i. (g,\abst{w}_i) \in \abst{f} \\
  \bigwedge(\lambda \abst{w}. \abst{w} \in \gin(g) \cup \out^{\abst{f}}(g))[\avec{w}] \\
  \bigwedge(\lambda \abst{w}. \text{pos}(g, \abst{w}) \neq \emptyset)[\avec{w}]
\end{array}} \\
\end{array}
\end{array}
$$
$$
\begin{array}{cc} 
\begin{array}{rl}
\text{pos} &: \Chip \to \Wire \to \pset{\SlotNSelector} \\
\text{pos}(g,w) &= \maybe{s \in \Rel_g}{
\begin{array}{l}
i = \text{wnat} \circ \text{last} \circ \ctrn_g(\ty(\abst{w}), s) \\
i \neq \bot \\
\gin(g)_{i} = \abst{w}
\end{array}}
\end{array} &
\begin{array}{rl}
- ( - ) &: (g: \Ops) \to \Wire^{n_g} \to (\abst{f}: \AbsCirc) \to \Chip \\
g(\avec{x} \cat \avec{r})_{\abst{f}} &= \maybe{g'}{
\begin{array}{l}
\cdots \land |\avec{r}| = b_g \\
b_g > 0 \implies \base(\abst{f}, \avec{r}) \neq \bot \\
\end{array}}
\end{array}
\end{array}
$$

e.g. parts of the pre-constraints for the gates $\chipu{\text{ChainMul}_p}{\abst{d},\abst{e},\abst{c}}, (\chipu{\text{Mul}_p}{\abst{a}, \abst{b}}, \abst{c}) \in \abst{f}$

\begin{center}
\begin{tabular}{r|c|c|c|c|c|c|c|c|c}
\cline{2-9}
\multirow{2}{*}{$\Ops$} & \multicolumn{8}{c|}{$p$} & \multirow{2}{*}{$\term$} \\
\cline{2-9}
& $A$ & $B$ & $C$ & $Q_l$ & $Q_r$ & $Q_o$ & $Q_m$ & $Q_s$ \\
\hline\hline
$\text{Mul}_p$ & $\abst{a}$ & $\abst{b}$ & $\abst{c}$ & 0 & 0 & -1 & 1 & 0 &
$A \cdot Q_l + B \cdot Q_r + C \cdot Q_o + A \cdot B \cdot Q_m$ \\
\hline
$\text{ChainMul}_p$ & $\abst{d}$ & $\abst{e}$ & $\abst{r}$ & 0 & 0 & -1 & 1 & 1 &
$Q_s \cdot (C_1 \cdot A \cdot B - C)$ \\
\hline
\end{tabular}
\end{center}

Using the terms, we have $-c + a \cdot b = 0$ enforcing the structure of $\text{Mul}_p$ and $c \cdot d \cdot e - r = 0$ enforcing the structure of $\text{ChainMul}_p$. Notice how $C_1$ refers to the column $C$ one row above the first row for $\ctrn_{\text{ChainMul}_p}$ i.e. the row for $\ctrn_{\text{Mul}_p}$. Thus, $\build{a \times b \times d \times e = r}{}{}$ is expressed in two rows instead of of three.

In summary, relative wires allows $F_{GC}$ to have terms that uses cells from its previous row.

TODO talk about W(omega X) and why this works
