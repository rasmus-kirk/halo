### Slots, Selectors and Gate Collection

The rest of the gate type definition includes $\eval_g$ the canonical program, and $\ctrn_g$. These will be used by trace.

$$
\begin{array}{rl}
\GateType
&= \cdots \times (W[\tin{g}] \to W[\tout{g}]) \times \text{Mapping}(g) \\ \\
\eval_g: W[\tin{g}] \to W[\tout{g}] &= (\lambda(\_,\mathtt{canonical\_program},\_).\mathtt{canonical\_program})(g) \\
\ctrn_g: \text{Mapping}(g) &= (\lambda(\_,\mathtt{mapping\_to\_rows}).\mathtt{mapping\_to\_rows})(g) \\
\end{array}
$$

Recall $\Surkal$ performs vanishing argument on $F_{GC}$. The primtive terms in $F_{GC}$ are called slots and selectors. Slots; $A,B,C,\cdots$, hold values of a (concrete) circuit's wires privately, wheras selectors; $Q_l, Q_r, Q_o, \cdots$, are public values modelling the structure of the circuit. Slots and selectors are identified with $\Nb$.

In $\ctrn_g$, slots can be used by all gates, however selectors are gate specific. Groups of gates that use the same selectors are called *gate groups*; $G$ where $\vec{g}_G$ are the gate types, $\vec{s}_G$ are the selectors the gate types can use, $\text{term}_G$ is the $F_{GC}$ term the group contributes to and pre and post are used by trace.

$$
\GateGroup = \GateType^k \times \Nb^j \times \text{Eqn} \times (\TraceTable \to \TraceTable)^2
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\vec{g}_G &= (\lambda (\vec{g}, \_). \vec{g})(G) \\
\vec{s}_G &= (\lambda (\_, \vec{s}, \_). \vec{s})(G)
\end{array} &
\begin{array}{rl}
\text{term}_G &= (\lambda (\_, \text{gc}, \_). \text{gc})(G)
\end{array} &
\begin{array}{rl}
\text{pre}_G &= (\lambda (\_, \vec{\rho}). \rho_1)(G) \\
\text{post}_G &= (\lambda (\_, \vec{\rho}). \rho_2)(G) \\
\end{array}
\end{array}
$$

An *equation* is a function from a vector of polymorphic type to the type itself. This lets us define an equation once to be used for scalars, polynomials, elliptic curve points, wires (with arithmetizer state) and so on. e.g. $\text{term}_g : \text{Eqn}$.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{Eqn} &= \text{Arg}^k \to \text{Arg} \\
+ &: \text{Arg} \to \text{Arg} \to \text{Arg}
\end{array} &
\begin{array}{rl}
\times &: \Fb_p \to \text{Arg} \to \text{Arg} \\
\times &: \text{Arg} \to \text{Arg} \to \text{Arg} \\
\end{array}
\end{array}
$$

The collection of all gate groups is called the *gate collection*; $GC$ where $\vec{G}_{GC}$ are the gate groups, $\vec{s}_{GC}$ are the slots all gates can use. The rest of the definitions are accumulations of the gate group's methods.

$$
\GateCollection = \GateGroup^k \times \Nb^j
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\vec{G}_{GC} &= (\lambda (\vec{G}, \_). \vec{G})(GC) \\
\vec{s}_{GC} &= (\lambda (\_, \vec{s}). \vec{s})(GC)
\end{array} &
\begin{array}{rl}
F_{GC} &= \sum\limits_{G \in \vec{G}_{GC}} \text{term}_G \\
\GateType_{GC} &= \bigcup\limits_{G \in \vec{G}_{GC}} \vec{g}_G
\end{array} &
\begin{array}{rl}
\text{pre}_{GC} &= \opcirc\limits_{G \in \vec{G}_{GC}} \text{pre}_G \\
\text{post}_{GC} &= \opcirc\limits_{G \in \vec{G}_{GC}} \text{post}_G \\
\end{array}
\end{array}
$$

Finally a *specification* defines the gate collection that the user can extend whilst building circuits. It also defines the wire type information. However, the arithmetizer does not need to know $\text{term}_g, \text{pre}_g, \text{post}_g, \vec{s}$ as these are specific to trace. But we will make the conversion to a type without these fields; $\abst{\text{Spec}}$, implicit.

$$
\begin{array}{rl}
\text{Spec} &= \GateCollection \times (\WireType: \Uni) \times (W: \WireType \to \Uni) \\
\end{array}
$$

TODO updates to arith

- AState contains a $\abst{\text{Spec}}$
- builder notation for adding gate groups
- builder notation for adding gate types

Lastly, we define a data structure indexed on slots and selectors called an *index map* that yields optional values or thunks; the latter necessary for $\plookup$. An instantiation of this is called the *trace table*[^index-map-notation]; $C$, which is what $\ctrn_g$ assists in computing. We also define join $\sqcup$ to combine them.

$$
\begin{array}{rl}
\IndexMap &= (X: \Nb \to \Uni) \to (Y: \Uni) \to (s: \Nb) 
\pto \begin{cases}
Y & X(s) = () \\
X(s) \to Y & \otherwise
\end{cases}
\end{array}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
-[-] &: (Y \to Z) \to \IndexMap(X,Y) \to \IndexMap(X,Z) \\
f[A] &= \maybe{B}{\forall s. B(s) = f(A(s)) \lor \forall \xi. B_\xi(s) = f(A_\xi(s))}\\
\\
\sqcup &: \IndexMap(X,Y_1) \to (Y_1 \to Y_2 \to Y_3) \\
&\to \IndexMap(X,Y_2) \to \IndexMap(X,Y_3) \\
A \sqcup_f B &= \begin{cases}
& \exists s. A(s) \neq \bot \land B(s) \neq \bot \\
& x = A \sqcup^s_f B \\
C[s \mapsto x]
& C = A[s \mapsto \bot] \sqcup_f B[s \mapsto \bot] \\
\bot & \otherwise
\end{cases} \\
A \sqcup^s_f B &= \begin{cases}
f(A(s), B(s)) & X(s) = () \\
\lambda x. f(A(s,x), B(s,x)) & \otherwise
\end{cases} \\
A \times B &= A \sqcup_{\lambda a,b.(a,b)} B \\
\end{array} &
\begin{array}{rl}
\TypedIndexMap
&= (X: \WireType \to \Nb \to \Uni) \\
&\to (Y: \WireType \to \Uni) \\
&\to (t: \WireType) \to \IndexMap(X(t),Y(t)) \\
\TraceTable &= \TypedIndexMap(X_\GateCollection, \lambda t. W(t)^n) \\
\\
-[-] &: (t:\WireType \to Y(t) \to Z(t)) \\
&\to \TypedIndexMap(X,Y) \to \TypedIndexMap(X,Z) \\
f[A] &= \lambda t. f(t)[A(t)] \\
\\
\sqcup &: \TypedIndexMap(Y_1) \\
&\to (t: \WireType \to Y_1(t) \to Y_2(t) \to Y_3(t)) \\
&\to \TypedIndexMap(Y_2) \to \TypedIndexMap(Y_3) \\
A \sqcup_f B &= \lambda t. A(t) \sqcup_{f(t)} B(t) \\
A \times B &= \lambda t. A(t) \times B(t)
\end{array}
\end{array}
$$

[^index-map-notation]: $C(q,A)$ is notated $C^q(A)$, where $q:\WireType$ and $A:\Nb$. If thunk $X_{\GateCollection}(q,T) = \Fb_q$, then $C(q,T,\xi)$ is notated $C^q_\xi(T)$.