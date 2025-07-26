**Spec**

Before defining trace, we discuss about more structures related to gate types. $\Surkal$ performs vanishing argument on $F_{GC}$; the *gate constraint polynomial*. $F_{GC}$ is defined as an *abstract equation*: a function from a vector of polymorphic type *arguments* to the type itself. This lets us define a function to be used for scalars, polynomials, elliptic curve points[^curve-mul] and wires (with $\AState$) in one definition.

[^curve-mul]: A subtype of $\text{Arg}$ does not have multiplication with itself. Concretely this is for curve points $E[\Fb_q]$. Conversion is implicit.

$$
\begin{array}{cccc}
\Eqn_k = \text{Arg}^k \to \text{Arg}
&
+ : \text{Arg} \to \text{Arg} \to \text{Arg}
&
\times : \Fb_p \to \text{Arg} \to \text{Arg}
&
\times : \text{Arg} \to \text{Arg} \to \text{Arg}
\end{array}
$$

The primtive terms in $F_{GC}$ are called *slots* and *selectors*. Slots; $A,B,C,\cdots$, hold concrete values of a circuit's wires privately, wheras selectors; $Q_l, Q_r, Q_o, \cdots$, are public values modelling the structure of the circuit. Slots can be used by all gates, however selectors are gate specific. Groups of gates that use the same selectors are called *gate groups* $\text{Grp}$ where $\GateType_{\text{Grp}}$ are the gate types and $\term_{\text{Grp}}$ is the $F_{GC}$ term the group contributes to.

$$
\GateGroup_i = \mathcal{P}(\GateType) \times \Selector^j \times \Eqn_{i+j}
$$
$$
\begin{array}{ccc}
\GateType_{\text{Grp}} = (\lambda (\GateType, \_). \GateType)(\text{Grp}) &
\Selector_{\text{Grp}} = (\lambda (\_, \vec{s}, \_). \vec{s})(\text{Grp}) &
\term_{\text{Grp}} = (\lambda (\_, \text{gc}, \_). \text{gc})(\text{Grp})
\end{array}
$$

The collection of all gate groups is called the *gate collection*; $GC$ where $\vec{G}({GC})$ are the gate groups. The rest of the definitions are accumulations of the gate group's methods.

$$
\begin{array}{ccc}
\begin{array}{rl}
\GateCollection &= (\vec{G}: \GateGroup_i^k) \times \Slot^i \\
\vec{G}(GC) &= (\lambda (\vec{G}, \_). \vec{G})(GC) \\
\Slot_{GC} &= (\lambda (\_, \vec{s}). \vec{s})(GC)
\end{array} &
\begin{array}{rl}
\GateType_{GC} &= \bigcup\limits_{G \in \vec{G}(GC)} \vec{g}_G \\
\Selector_{GC} &= \bigcup\limits_{G \in \vec{G}(GC)} \Selector_G
\end{array} &
\begin{array}{rl}
F_{GC}(\vec{s}, \vec{S}) &= \sum\limits_{i \in |\vec{G}(GC)|} \term_{{\vec{G}(GC)}_i}(\vec{s}, \vec{S}_i) \\
\end{array}
\end{array}
$$

A *specification*[^spec-benefit] defines a $\GateCollection$. In the previous section on arithmetize, we omitted $s:\Spec$ in $\AState$ leaving $W, \WireType, \GateType$ implicit for $W_s, \WireType_s, \GateType_s$. Moreover, it does not need to know $\term_G, \Selector_G, \Slot_{GC}$ as these are specific to trace. We leave the spec instance $s$ and its conversion implicit for brevity in this document. [^more-brevity]

[^more-brevity]: We can notationally index spec directly to get fields of gate collection e.g. $\GateType_{s} = \GateType_{GC_{s}}$

$$
\begin{array}{cc}
\Spec = (GC: \GateCollection) \times (\WireType: \Uni) \times (W: \WireType \to \Uni) \times (X: F(\Uni))
\end{array}
$$
$$
\begin{array}{cccc}
GC_s = (\lambda(GC, \_). GC)(s)
&
X_s = (\lambda(\_, X).X)(s)
&
\WireType_s = (\lambda(\_, \WireType, \_). \WireType)(s)
&
W_s = (\lambda(\_, \_, W). W)(s)
\end{array}
$$

In summary, spec can be visualized as follows:

\begin{center}
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\tikzset{v/.style={draw, rounded corners, anchor=north}}
\node[v] (spec) at (0,0) {$s: \Spec$};

\node[v,anchor=east] (X) at ($(spec.west)+(-1,0)$) {$X_s$};

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

\node[v] (col) at ($(spec.south)+(1,-0.5)$) {$GC_s$};
\node[v] (slot) at ($(col.south)+(-1,-0.25)$) {$\Slot_s$};
\node[v] (A) at ($(slot.south)+(0,-0.25)$) {$A$};
\node[v] (B) at ($(A.south)+(0,-0.25)$) {$B$};
\node[v] (C) at ($(B.south)+(0,-0.25)$) {$C$};

\draw[-] (slot.south) -- (A.north);
\draw[-] (A.south) -- (B.north);
\draw[-] (B.south) -- (C.north);
\draw[-,dashed] (C.south) -- ($(C.south)+(0,-0.5)$);

\node[v] (G) at ($(col.south)+(4,-0.25)$) {$\vec{G}(GC_s)$};
\node[v] (G1) at ($(G.south)+(0,-0.25)$) {$\text{Grp}_1$};
\node[v,anchor=east] (Sel1) at ($(G1.west)+(-0.5,0)$) {$\Selector_{\text{Grp}_1}$};
\node[v,anchor=north] (term1) at ($(G1.south)+(1,-0.25)$) {$\term_{\text{Grp}_1}$};
\node[v,anchor=west] (GTy1) at ($(G1.east)+(1,0)$) {$\GateType_{\text{Grp}_1}$};
\draw[->] (G1.west) -- (Sel1.east);
\draw[->] (G1.east) -- (term1.north);
\draw[->] (G1.east) -- (GTy1.west);
\node[v] (G2) at ($(G1.south)+(0,-1)$) {$\text{Grp}_2$};
\node[v,anchor=east] (Sel2) at ($(G2.west)+(-0.5,0)$) {$\Selector_{\text{Grp}_2}$};
\node[v,anchor=north] (term2) at ($(G2.south)+(1,-0.25)$) {$\term_{\text{Grp}_2}$};
\node[v,anchor=west] (GTy2) at ($(G2.east)+(1,0)$) {$\GateType_{\text{Grp}_2}$};
\draw[->] (G2.west) -- (Sel2.east);
\draw[->] (G2.east) -- (term2.north);
\draw[->] (G2.east) -- (GTy2.west);

\node[v,anchor=east] (Q1) at ($(Sel1.west)+(-0.5,0.5)$) {$Q_1$};
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

\node[v,anchor=west] (g1) at ($(GTy1.east)+(0.5,0.5)$) {$g_1$};
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
\draw[->] (col.south) -- (G.north);

\draw[->] (spec.west) -- (X.east);
\draw[->] (spec.south) -- (wty.north);
\draw[->] (spec.south) -- (w.north);
\draw[->] (spec.south) -- (col.north);

\node[anchor=north] (FGC) at ($(term2.south)+(0,-0.5)$) {$F_{GC}$};
\node[anchor=east] (GTys) at ($(FGC.east)+(3.1,0)$) {$\GateType_s$};
\node[anchor=west] (Sels) at ($(FGC.west)+(-5.25,0)$) {$\Selector_s$};
\end{tikzpicture}
\end{center}

[^spec-benefit]: With spec as a data structure, the user can extend it adding new gate types and terms ad hoc while arithmetizing.


**Index Map**

An *index map*[^index-map-notation] maps wire types and slots or selectors to thunks of $Y$ of argument[^not-spec] $X$; most have no arguments except for $\plookup$ columns. We also define map $-[-]$ and join $\sqcup$ with $F(T)$ as a function type from the indices to $T$.[^free-F]

[^free-F]: If $t,s$ appears free in $F$, then it refers to the indices. i.e. $F(T(t,s)) = (t: \WireType) \to (s: \SlotNSelector) \to T(t,s)$.


[^index-map-notation]: Let $C:\IndexMap(X,Y)$, then $C^q(A)$ is short for $C(q,A,())$ and $C^q_\xi(f)$ is short for $C(q,f,\xi :W(q))$ if $X(q,f) = W(q)$


[^not-spec]: This may not necessarily be $X_s$ from $s: \Spec$, but if it is not specified, we may assume it is $X_s$.


$$
\begin{array}{cc}
\begin{array}{rl}
F(T) &= \WireType \to \SlotNSelector \to T \\
\IndexMap &= (X,Y: F(\Uni)) \\
&\to F(\Option(X(t,s) \to Y(t,s))) \\
\\
-[-] &: F(Y_1(t,s) \to Y_2(t,s)) \\
&\to \IndexMap(X,Y_1) \to \IndexMap(X,Y_2) \\
f[A] &= \lambda t. f(t)[A(t)] \\
f[a] &= a^s_f[s \mapsto \lambda x. f(s,a(s,x))] \\
a^s_f &= \maybe{f[a[s \mapsto \bot]]}{\exists s. a(s) \neq \bot} \\
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
$$
\begin{array}{cc}
A \times B = A \sqcup_{\lambda \_,a,b.(a,b)} B &
A \cat B = A \sqcup_{\lambda \_,\vec{a}, \vec{b}. \vec{a} \cat \vec{b}} B
\end{array}
$$

**Pre-Constraints**

We now introduce two more projections from gate type.

$$
\begin{array}{c}
\GateType
= \cdots \times (\ctrn: \PreTable) \times (\base: \mathcal{P}(\GateType)) \times \cdots \\
\begin{array}{cc}
\ctrn_g = (\lambda(\_,\ctrn).\ctrn)(g) &
\base_g = (\lambda(\_,\base,\_).\base)(g)
\end{array}
\end{array}
$$

The *pre-constraints* $\ctrn_g$ of the gate type $g$ is an index map of a vector of *pre-values*. Note that the vectors across different wire types $t$ need not be the same length. Pre-values are defined in terms of a reducer type $R$ that computes a value for a wire type $W(t)$ given the thunk argument $X(t,s)$ and vector of concrete wire values; selected from the input and output wires of the gate.

$$
\begin{array}{cc}
R(\abst{\vec{w}}) = F(X(t,s) \to W[\ty[\abst{\vec{w}}]] \to W(t)) &
\text{Pre} = F(X(t,s) \times (\abst{\vec{w}} : \Wire^k) \times R({\abst{\vec{w}}},t,s))
\end{array}
$$
$$
\text{PreTable} = \IndexMap(X, \lambda t,s. \text{Pre}(t,s)^k)
$$

Typically a pre-value is of the following forms:

pre-value | notation | reducer form
-|-|-
constant | $c$ | $((),(),\lambda (). c)$
wire | $\abst{w}$ | $((),(\abst{w}), \lambda (),w. w)$
$\plookup$ cell[^plookupdefn] | $\top$ | $((\_, \xi), \abst{\vec{w}}, \lambda \_,\xi,\vec{w}. w_1 + \xi \cdot w_2 + \xi^2 \cdot w_3)$
$\plookup$ default cell | $\bot$ | $((d,\_), \_, \lambda d,\_.d)$

[^plookupdefn]: we defer defining and motivating $\plookup$ cells to when we define the lookup gates


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

**Relative Gates**

$\base_g$ is a set of gate types that specify which gate types can be its *base gate*. If the set is non empty, we call $g$ a *relative gate* type. A relative gate will always have its base gate constraints appear immediately before it.[^dupe] The full definition of a gate is as follows:

[^dupe]: If different relative gates have the same base gate, we can expect duplicate rows such that both relative gates have a copy of the base gate's constraints.


$$
\begin{array}{cc}
\Gate = (g: \GateType) \times \Wire^{n_g} \times (b: \base_g) & b_g = (\lambda \_, b. b)(g) \\
\end{array}
$$

If $g_1$ has base gate $g_2$, we notate $g_1 \otimes g_2$ as the relative gate. 

e.g. parts of the pre-constraints for the gates $\text{ChainMul}_p(\abst{d},\abst{e}) \otimes \text{Mul}_p(\abst{a}, \abst{b})$

\begin{center}
\begin{tabular}{r|c|c|c|c|c|c|c|c|c}
\cline{2-9}
\multirow{2}{*}{$\GateType$} & \multicolumn{8}{c|}{$p$} & \multirow{2}{*}{$\term$} \\
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

Using the terms, we have $-c + a \cdot b = 0$ enforcing the structure of $\text{Mul}_p$ and $c \cdot d \cdot e - r = 0$ enforceing the structure of $a \cdot b \cdot d \cdot e = r$. Notice how $C_1$ refers to the column $C$ at one row above the first row for $\ctrn_{\text{ChainMul}_p}$ i.e. the row for $\ctrn_{\text{Mul}_p}$. Thus, instead of three rows if we were to use three $\text{Mul}_p$ gates, we just have two using $\text{ChainMul}_p$.

Finally, the last two projections of $\GateType$ are as follows:

$$
\GateType = \cdots \times \left(\refg : F\left(\Option\left(\left[1+\min\limits_{b \in \base_g} \left|\ctrn_b(t,s) \right|\right]^k\right)\right)\right) \times (\eval: W[\tin{g} \cat ?] \to W[\tout{g}])
$$
$$
\begin{array}{cc}
\refg_g = (\lambda(\_,\refg,\_).\refg)(g) &
\eval_g = (\lambda(\_,\eval).\eval)(g)
\end{array}
$$

$\refg_g$ defines the offsets for relative gates. It is a function from index map indices to a vector of natural number offsets. The definition guarantees, that the offset stays within the pre-constraints of the base gate. $\abst{w}_{r \otimes b}$ maps the offset position with the wires in that cell. If the offset position is not a wire i.e. constant or $\plookup$ cell, then it is omitted. These wires will be available to $\eval_g$ when computing wires.

$$
\begin{array}{rl}
\abst{w}_{r \otimes b} &= (\lambda t,s,x,\vec{o}. \text{lookup}(\ctrn_b(t,s,x),\vec{o}))[\refg_r] \\
\text{lookup}(\vec{c},\vec{o}) &= \begin{cases}
() & \vec{o} = () \\
& \vec{o} = o \cat \vec{o}' \\
& \abst{\vec{w}} = \text{lookup}(\vec{c},\vec{o}') \\
\abst{w} \cat \abst{\vec{w}} & c_{|\vec{c}| - o + 1} = (\_, (\abst{w}), \_) \\
\abst{\vec{w}} & \otherwise
\end{cases}
\end{array}
$$

TODO

linearize the imap of wires into a single vector of wires - linearize is cat each column i.e. Y(t,s) is constant over t,s

map it by ty and concat with $\tin{g}$ is the arg for eval

the linearize must also be done on ref mapped as (s,o) for gate group to populate all offsets its term can use

In summary, relative gates allows $F_{GC}$ to have terms that uses cells from multiple rows instead of strictly one.

$\eval_g$ is the canonical program.

eval nows needs another argument of its base gate's type to find the wire via lookup linearize etc

