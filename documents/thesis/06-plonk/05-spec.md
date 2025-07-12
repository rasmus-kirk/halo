### Gate Spec

Before defining trace, we define the rest of the gate type definition includes $\eval_g$ the canonical program, and $\ctrn_g$.

$$
\begin{array}{rl}
\GateType
&= \cdots \times (W[\tin{g}] \to W[\tout{g}]) \times \text{Mapping}(g) \\ \\
\eval_g: W[\tin{g}] \to W[\tout{g}] &= (\lambda(\_,\mathtt{canonical\_program},\_).\mathtt{canonical\_program})(g) \\
\ctrn_g: \text{Mapping}(g) &= (\lambda(\_,\mathtt{mapping\_to\_rows}).\mathtt{mapping\_to\_rows})(g) \\
\end{array}
$$

Recall $\Surkal$ performs vanishing argument on $F_{GC}$. An *equation*; such as $F_{GC}$, is a function from a vector of polymorphic type *arguments* to the type itself. This lets us define an equation once to be used for scalars, polynomials, elliptic curve points[^curve-mul], wires (with $\AState$) and so on.

[^curve-mul]: A subtype of $\text{Arg}$ does not have multiplication with itself. Concretely this is for $E[\Fb_q]$. We leave this conversion implicit.


$$
\begin{array}{cccc}
\begin{array}{rl}
\text{Eqn}_k &= \text{Arg}^k \to \text{Arg}
\end{array} &
\begin{array}{rl}
+ &: \text{Arg} \to \text{Arg} \to \text{Arg}
\end{array} &
\begin{array}{rl}
\times &: \Fb_p \to \text{Arg} \to \text{Arg}
\end{array} &
\begin{array}{rl}
\times &: \text{Arg} \to \text{Arg} \to \text{Arg}
\end{array}
\end{array}
$$

The primtive terms in $F_{GC}$ are called slots and selectors. Slots; $A,B,C,\cdots$, hold concrete values of a circuit's wires privately, wheras selectors; $Q_l, Q_r, Q_o, \cdots$, are public values modelling the structure of the circuit. Slots and selectors are uniquely identified with a $\Nb$.

In $\ctrn_g$, slots can be used by all gates, however selectors are gate specific. Groups of gates that use the same selectors are called *gate groups*; $G$ where $\vec{g}_G$ are the gate types, $\vec{s}_G$ are the selectors the gate types can use, $\text{term}_G$ is the $F_{GC}$ term the group contributes to, $\text{pre}_G$ and $\text{post}_G$ are used by trace.

$$
\GateGroup_i = \GateType^k \times \Nb^j \times \text{Eqn}_{i+j} \times (\TraceTable \to \TraceTable)^2
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\vec{g}_G &= (\lambda (\vec{g}, \_). \vec{g})(G) \\
\vec{s}_G &= (\lambda (\_, \vec{s}, \_). \vec{s})(G)
\end{array} &
\begin{array}{rl}
j_G &= |\vec{s}_G| \\
\text{term}_G &= (\lambda (\_, \text{gc}, \_). \text{gc})(G)
\end{array} &
\begin{array}{rl}
\text{pre}_G &= (\lambda (\_, (\text{pre},\_)). \text{pre})(G) \\
\text{post}_G &= (\lambda (\_, (\_, \text{post})). \text{post})(G) \\
\end{array}
\end{array}
$$

The collection of all gate groups is called the *gate collection*; $GC$ where $\vec{G}_{GC}$ are the gate groups, $\vec{s}_{GC}$ are the slots all gates can use. The rest of the definitions are accumulations of the gate group's methods.

$$
\GateCollection = (\vec{G}: \GateGroup_i^k) \times \Nb^i \times (X : \Nb \to \Uni)
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\vec{G}(GC) &= (\lambda (\vec{G}, \_). \vec{G})(GC) \\
\vec{s}_{GC} &= (\lambda (\_, \vec{s}, \_). \vec{s})(GC) \\
X_{GC} &= (\lambda (\_, X, \_). X)(GC)
\end{array} &
\begin{array}{rl}
F_{GC}(\vec{s}, \vec{S}) &= \sum\limits_{i \in |\vec{G}(GC)|} \text{term}_{{\vec{G}(GC)}_i}(\vec{s}, \vec{S}_i) \\
\GateType_{GC} &= \bigcup\limits_{G \in \vec{G}(GC)} \vec{g}_G
\end{array} &
\begin{array}{rl}
\text{pre}_{GC} &= \opcirc\limits_{G \in \vec{G}(GC)} \text{pre}_G \\
\text{post}_{GC} &= \opcirc\limits_{G \in \vec{G}(GC)} \text{post}_G \\
\end{array}
\end{array}
$$

A *specification* defines a $\GateCollection$ that the user can extend whilst building circuits[^arith-spec] and wire type information. 

[^arith-spec]: $\AState$ implicitly contains a $s:\Spec$ but it does not need to know $\text{term}_g, \text{pre}_g, \text{post}_g, \vec{s}_G, \vec{s}_{GC}$ as these are specific to trace. Moreover, types $W, \WireType, \GateType$ in the previous section on arithmetizer are implicit for $W_s, \WireType_s, \GateType_s$.


$$
\begin{array}{cc}
\begin{array}{rl}
\Spec &= \GateCollection \times (\WireType: \Uni) \times (W: \WireType \to \Uni) \\
\end{array} &
\begin{array}{rl}
\GateCollection_s &= (\lambda(GC, \_). GC)(s)
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\GateType_{s} &= \GateType_{\GateCollection_{s}}
\end{array} &
\begin{array}{rl}
\WireType_s &= (\lambda(\_, \WireType, \_). \WireType)(s)
\end{array} &
\begin{array}{rl}
W_s &= (\lambda(\_, \_, W). W)(s)
\end{array}
\end{array}
$$

Lastly, we define a data structure indexed on slots and selectors called an *index map* that yields optional values or thunks of argument $X(s)$; the latter necessary for $\plookup$. An instantiation of this is called the *trace table*[^index-map-notation]; $C$, which is what $\ctrn_g$ assists in computing. We also define map $-[-]$ and join $\sqcup$ to compute on and combine index maps.

$$
\begin{array}{rl}
\IndexMap &= (X: \Nb \to \Uni) \to (Y: \Uni) \to (s: \Nb) 
\pto \begin{cases}
Y & X(s) = () \\
X(s) \to Y & \otherwise
\end{cases} \\
\TypedIndexMap
&= (X: \WireType \to \Nb \to \Uni)
\to (Y: \WireType \to \Uni) 
\to (t: \WireType) \to \IndexMap(X(t),Y(t)) \\
\TraceTable &= (GC: \GateCollection) \to \TypedIndexMap(X_{GC}, \lambda t. W(t)^n)
\end{array}
$$
$$
\begin{array}{c}
-[-] : (t: \WireType \to Y \to Z) \to \IndexMap(X,Y) \to \TypedIndexMap(X,Z) \\
\begin{array}{rlrl}
f[A] &= \lambda t. f(t)[A^t] &
f[A] &= \begin{cases}
& A' = \maybe{f[A[s \mapsto \bot]]}{\exists s. A(s) \neq \bot} \\
A'[s \mapsto f(A(s))]
& X(s) = () \\
A'[s \mapsto \lambda x. f(A(s,x))]
& \otherwise
\end{cases}
\end{array}
\end{array}
$$
$$
\sqcup : \TypedIndexMap(Y_1) \to (t: \WireType \to Y_1(t) \to Y_2(t) \to Y_3(t)) \to \TypedIndexMap(Y_2) \to \TypedIndexMap(Y_3)
$$
$$
\begin{array}{cc}
\begin{array}{rl}
A \sqcup_f B &= \lambda t. A(t) \sqcup_{f(t)} B(t) \\
A \sqcup_f B &= \begin{cases}
& \exists s. A(s) \neq \bot \land B(s) \neq \bot \\
& x = A \sqcup^s_f B \\
C[s \mapsto x]
& C = A[s \mapsto \bot] \sqcup_f B[s \mapsto \bot] \\
\bot & \otherwise
\end{cases}
\end{array} &
\begin{array}{rl}
A \sqcup^s_f B &= \begin{cases}
f(A(s), B(s)) & X(s) = () \\
\lambda x. f(A(s,x), B(s,x)) & \otherwise
\end{cases} \\ \\
A \times B &= \lambda t. A(t) \times B(t) \\
A \times B &= A \sqcup_{\lambda a,b.(a,b)} B
\end{array}
\end{array}
$$

[^index-map-notation]: Let $C$ be a trace table, $C(q,A)$ is notated $C^q(A)$. If thunk $X_{\GateCollection}(q,T) = \Fb_q$, then $C(q,T,\xi)$ is notated $C^q_\xi(T)$.