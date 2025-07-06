### Slots, Selectors and Gate Registry

The rest of $\GateType$ definition includes $\eval_g$ the canonical program, and $\ctrn_g$.

$$
\begin{array}{rl}
\GateType
&= \cdots \times (W[\tin{g}] \to W[\tout{g}]) \times \text{Mapping}(g) \\ \\
\eval_g: W[\tin{g}] \to W[\tout{g}] &= (\lambda(\_,\mathtt{canonical\_program},\_).\mathtt{canonical\_program})(g) \\
\ctrn_g: \text{Mapping}(g) &= (\lambda(\_,\mathtt{mapping\_to\_rows}).\mathtt{mapping\_to\_rows})(g) \\
\end{array}
$$

Recall $\Surkal$ performs vanishing argument on $F_{GC}$. The primtive terms in $F_{GC}$ are called slots and selectors. Slots; $A,B,C,\cdots$, hold values of a (concrete) circuit's wires privately, wheras selectors; $Q_l, Q_r, Q_o, \cdots$, are public values modelling the structure of the circuit. Slots and selectors are identified with $\Nb$. We define a data structure indexed on them yielding optional values or thunks; the latter necessary for $\plookup$. An instantiation of this called the *trace table*[^index-map-notation]; $C$, which is what $\ctrn_g$ assists in computing. We also define join $\sqcup$ to compute and $\times$ to combine values.

$$
\begin{array}{cc}
\begin{array}{rl}
\text{IndexMap} &= (X: \Nb \to \mathcal{U}) \to (Y: \mathcal{U}) \to (s: \Nb) \\
&\pto \begin{cases}
Y & X(s) = \bot \\
X(s) \to Y & \text{otherwise}
\end{cases}
\end{array} &
\begin{array}{rl}
\text{TypedIndexMap}
&= (X: \WireType \to \Nb \to \mathcal{U}) \\
&\to (Y: \WireType \to \mathcal{U}) \\
&\to (t: \WireType) \\
&\to \text{IndexMap}(X(t),Y(t))
\end{array}
\end{array}
$$
$$
\text{TraceTable} = \text{TypedIndexMap}(X_\text{GateRegistry}, \lambda t. W(t)^n)
$$
$$
\begin{array}{cc}
\begin{array}{rl}
\sqcup &: \text{IndexMap}(X,Y) \to (Y \to Y \to Y) \\
&\to \text{IndexMap}(X,Y) \to \text{IndexMap}(X,Y) \\
A \sqcup_f B &= \begin{cases}
B 
& A = \bot \\
A[s \mapsto \bot] \sqcup_f B[s \mapsto A \sqcup^s_f B] 
& \exists s. A(s) \neq \bot
\end{cases} \\
A \sqcup^s_f B &= \begin{cases}
A(s) & B(s) = \bot \\
f(A(s), B(s)) & X(s) = \bot \\
\lambda x. f(A(s,x), B(s,x)) & \text{otherwise}
\end{cases}
\end{array} &
\begin{array}{rl}
\sqcup &: \text{TypedIndexMap}(Y) \\
&\to (t: \WireType \to Y(t) \to Y(t) \to Y(t)) \\
&\to \text{TypedIndexMap}(Y) \\
&\to \text{TypedIndexMap}(Y) \\
A \sqcup_f B &= \lambda t. A(t) \sqcup_{f(t)} B(t)
\end{array}
\end{array}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
\times &: \text{IndexMap}(X,Y) \to \text{IndexMap}(X,Y') \\
&\to \text{IndexMap}(X,Y \times Y') \\
A \times B &= \lambda s. \begin{cases}
(A(s),B(s)) & X(s) = \bot \\
\lambda x. (A(s,x), B(s,x)) & \text{otherwise}
\end{cases}
\end{array} &
\begin{array}{rl}
\times &: \text{TypedIndexMap}(X,Y) \\
&\to \text{TypedIndexMap}(X,Y') \\
&\to \text{TypedIndexMap}(X,Y \times Y') \\
A \times B &= \lambda t. A(t) \times B(t)
\end{array}
\end{array}
$$

[^index-map-notation]: $C(q,A)$ is notated $C^q(A)$, where $q:\WireType$ and $A:\Nb$. If thunk $X_{\text{GateRegistry}}(q,T) = \Fb_q$, then $C(q,T,\xi)$ is notated $C^q_\xi(T)$.

Slots can be shared by all gates, however selectors are gate specific. Groups of gates that use the same selectors are called *gate groups*.

TODO GateGroup defn

- vector of gate type
- vector of selectors (in formalism it is implicit no selector are equal, in rust u have to enforce by computing via offsets, thus in rust its just SELECTOR_COUNT)
- gc
- pre, post

The collection of all gate groups is called the *gate registry*.

TODO GateRegistry defn

- vector of GateGroup
- vector of slots (in rust its SLOT_COUNT)
- f_gc
- Gate = concat of GateGroup's GateTypes
- pre post of all groups

TODO contemplate consequences of theres no dynamic on the fly new gatetype construction / group update / registry update as in $\build{-}{}{}$.

- registry is a variable in AState!!!

### Trace

The trace computation is defined as follows:
$$
(C, \vec{\sigma}, L) = \mathrm{trace}(\abst{f},\abst{\vec{Y}},\vec{x})
$$

**Monotonic Functions**

$\text{trace}$ computes the least fixed point of a composition of monotonic functions; $\Downarrow_R, \Downarrow_G, \Downarrow_C$, using $\text{sup}$. We also call a monotonic function a continuation if it is called by another. We call lift, to extend the argument of a monotonic function.

$$
\begin{array}{rl}
\begin{array}{rl}
\lift(f) &= \lambda (v,t,u). (v, f(t),u) \\
g \circ^{\uparrow} f &= \lift(g) \circ \lift(f) 
\end{array} &
\begin{array}{rl}
\text{sup} &: (T \to T) \to (T \to T \to \Bb) \to T \to T \to T \\
\text{sup}(f, \text{eq}, s, s') &= \begin{cases}
s & \text{eq}(s, s') \\
\text{sup}(f, \text{eq}, s', f(s')) & \otherwise
\end{cases}
\end{array}
\end{array}
$$

The monotonic functions[^dagger] defined below are specific to the $\Surkal$ protocol. Thus if the arithmetizer abover were to be extended for a different \plonk-ish protocol, the functions would be different.

[^dagger]: for each monotonic function, we notate $\dagger$ as a check if the state has saturated in which the fixpoint compute can terminate. Wheras $s$ are the initial states and $\iota$ a constructor of it. 

**Resolve**

$\Downarrow_R$ computes the values of wires $\abst{\vec{Y}}$ and inputs to assert gates given the input wire values $\vec{x}$.
 
It does this by peeking from the stack $\abst{\vec{y}}$, querying $\text{?}$ for unresolved input wires, otherwise it will evaluate the output wire values and cache it in the value map $v$ with $[\cdot]$. The continuation $f$ and stack pop $\curvearrowleft$ are called after.

$$
\begin{array}{ccc}
\begin{array}{rl}
\VMap &= (w: \Wire) \pto W(\ty(w)) \\
\RState^k &= \VMap \times \Wire^k \\
\end{array}
&
\begin{array}{rl}
\curvearrowleft &: X^k \to X^{k'} \\
\curvearrowleft (\vec{x}) &= \begin{cases}
() & \vec{x} = () \\
\vec{x}' & \vec{x} = \_ \cat \vec{x}' \\
\end{cases}
\end{array}
&
\begin{array}{rl}
\underset{R}{\curvearrowleft} &: T \times \Wire^k \to T \times \Wire^{k'} \\
\underset{R}{\curvearrowleft} &= \lift(\curvearrowleft)
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{?} &: \VMap \to \Wire^k \to \Wire^{k'} \\
v \text{?} \abst{\vec{y}} &= \begin{cases}
() & \abst{\vec{y}} = () \\
& \abst{\vec{y}} = \abst{y} \cat \abst{\vec{y}}' \\
\abst{y} \cat v \text{?} \abst{\vec{y}}' & v(\abst{y}) = \bot \\
v \text{?} \abst{\vec{y}}' & \otherwise
\end{cases} \\
\\
\left[ \cdot \right] &: \VMap \to \AbsCirc \to \Wire \to \VMap \\
v_{\abst{f}}\left[\abst{y}\right] &= \maybe{
  v[\abst{\vec{y}} \mapsto \vec{y}]
}{\begin{array}{rl}
  \abst{f} &\ni (g, \abst{y}) \\
  \abst{\vec{y}} &= \out^{\abst{f}}(g) \\
  \vec{y} &= \eval_g(v[\gin(g)]) \\
\end{array}}
\end{array}
&
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_R &: (T \times \RState \to T \times \RState) \to \AbsCirc \\
&\to T \times \RState \to T \times \RState \\
f \stackrel{\to}{\circ} \Downarrow^{\abst{f}}_R(t,v, \abst{\vec{y}}) &= \begin{cases}
f(t,v,()) & \abst{\vec{y}} = () \\
 & \abst{\vec{y}} = \abst{y} \cat \_ \\
\underset{R}{\curvearrowleft} (t, v, \abst{\vec{y}}) & v(\abst{y}) \neq \bot \\
 & (g, \abst{y}) \in \abst{f} \\
 & \abst{\vec{x}} = v \text{?} \gin(g) \\
\underset{R}{\curvearrowleft} \circ f(t, v_{\abst{f}}[\abst{y}], \abst{\vec{y}}) 
 & \abst{\vec{x}} = () \\
(t, v, \abst{\vec{x}} \cat \abst{\vec{y}}) & \otherwise
\end{cases} \\
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\dagger_R &: \RState \to \Bb \\
\dagger_R(\_, \abst{\vec{y}}) &= |\abst{\vec{y}}| = 0 
\end{array}
&
\begin{array}{rl}
s &: \Wire^m \to W[\tin{}] \to \RState \\
s^{\abst{\vec{Y}}}_{\vec{x}} &= (\bot[(0..|\vec{x}|) \mapsto \vec{x}], \abst{\vec{Y}} \cat \set{\abst{x} \middle\vert (g, \bot) \in \abst{f} \land \abst{x} \in \gin(g) \setminus \abst{\vec{Y}}})
\end{array}
&
\begin{array}{rl}
s_0 &: \RState \\
s_0 &= (\bot, ())
\end{array}
\end{array}
$$

**Gate Constraints**

Each gate corresponds to some rows in $C$ via $\ctrn_g$.

TODO fix Mapping definition

\begin{center}
\begin{tabular}{ c c c c }
\begin{math}
\begin{array}{c}
\begin{array}{rl}
C_{idx}(g,t') &= \set{i | i \in \Nb \land (\tin{g} \cat \tout{g})_i = t'}\\
C_{val}(g,t') &= C_{idx}(g, t') + W(t') \\
C_{row}(g,t') &= C_{val}(\ty(g), t')^{|\Slot| + |\Selector|} \\
\text{Mapping}(g) &= (t': \WireType) \to C_{row}(g, t')^{k_{t'}}\\
\end{array} \\
\begin{array}{rl}
-[-]^{-}_{-} &: \VMap \to C_{row}(g,t)^k \to \AbsCirc \to (g: \Gate) \\
&\to W(t)^k \\
v[\vec{m}]^{\abst{f}}_g &= \left(\lambda m. \begin{cases}
x & m = \text{inr}(x) \\
& m = \text{inl}(x) \\
v(\gin(g)_x) & x < n_g \\
v(\out^{\abst{f}}(g)_{x-n_g}) & \text{otherwise}
\end{cases} \right) [\vec{m}]
\end{array}
\end{array}
\end{math}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{id}{(0,0)}{$\ $}{$g$}{1}
\draw[->,thick] (id-out-1) -- ($(id-out-1)+(0,-0.5)$);
\draw[-,thick] ($(id.north)+(0,0.5)$) -- (id.north);
\end{tikzpicture}
&
\begin{tabular}{|c|c|c|c|c|c|c|c|c|}
\hline
\multicolumn{9}{|c|}{$C$} \\
\hline
\multicolumn{4}{|c|}{$W(p)=\Fb_p$} & \multicolumn{4}{|c|}{$W(q)=\Fb_q$} & $\cdots$ \\
\hline
$A$ & $\cdots$ & $Q_l$ & $\cdots$ & $A$ & $\cdots$ & $Q_l$ & $\cdots$ & $\cdots$ \\
\hline\hline
\multicolumn{9}{|c|}{$\vdots$} \\
\hline
\multicolumn{4}{|c|}{$v [\ctrn_g(p)]^{\abst{f}}_g$} & \multicolumn{5}{|c|}{$\vdots$} \\
\hline
\multicolumn{9}{|c|}{$\vdots$} \\
\hline
\multicolumn{4}{|c|}{$\vdots$} & \multicolumn{4}{|c|}{$v [\ctrn_g(q)]^{\abst{f}}_g$} & $\ddots$ \\
\hline
\multicolumn{9}{|c|}{$\vdots$} \\
\hline
\end{tabular}
\end{tabular}
\end{center}


$\Downarrow_G$ computes the trace table $C$ by pushing the gate with an output of the top of the wire id stack via push; $\underset{G}{\curvearrowright}$. The same gate will not appear twice since we do not call the continuation (including $\Downarrow_G$), on resolved wires in $\Downarrow_R$.

When the wire id stack $\abst{\vec{y}}$ is empty, $\underset{G}{\curvearrowright}$ will push assert gates and input gates $X^{\abst{f}}$ to the stack.
$$
\begin{array}{rl}
\begin{array}{rl}
\text{TraceTable} &= (t: \WireType) \pto W(t)^{(|\Slot| + |\Selector|) \times k_t} \\
\text{GState}^{k,k'} &= \text{TraceTable} \times \Gate^{k'} \times \Bb \times \RState^k \\
\abst{X}^{\abst{f}} &= \set{g \middle\vert (g, \abst{y}) \in \abst{f} \land (\abst{y} = \bot \lor \exists i,t. \abst{y} = \text{Input}^t_i) } \\
\\
\underset{G}{\curvearrowleft} &: T \times \text{GState}^{k,k''} \to T \times \text{GState}^{k',k''} \\
\underset{G}{\curvearrowleft} &= \lift(\curvearrowleft : \Gate^k \to \Gate^{k'}) \\
\\
\dagger_G &: \text{GState} \to \Bb \\
\dagger_G &= \lambda(\_, \vec{g}, b, \_). |\vec{g}| = 0 \land b = \top \\
\\
\iota_G &: \text{RState} \to \text{GState} \\
\iota_G(s) &= (\bot, (), \bot, s)
\end{array} &
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \\
&\to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \underset{G}{\curvearrowleft} \circ f \circ^\uparrow \lambda (C, \vec{g}, b, v). \\
&\begin{cases}
& \vec{g} = g \cat \_ \\
(C',\vec{g},b,v)
& C' = \lambda t. C(t) \cat v[\ctrn_g(t)]^{\abst{f}}_g \\
(C, (), b, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, b, v, \abst{\vec{y}}). \\
&\begin{cases}
& b = \bot \\
(X^{\abst{f}} \cat (), \top, v, ())
& |\abst{\vec{y}}| = |\vec{g}| = 0 \\
& \abst{\vec{y}} = \abst{y} \cat \_ \\
(g \cat \vec{g}, b, v, \abst{\vec{y}})
& (g,\abst{y}) \in \abst{f} \\
(\vec{g}, b, v, \abst{\vec{y}})
& \otherwise
\end{cases}
\end{array}
\end{array}
$$

**Copy Constraints**

$\Downarrow_C$ computes coordinate loops; equivalence class of slot positions of $C$ modulo wire, by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate using $\sqcup$.

After computing the coordinate loop of the full circuit, we mark a flag $\Bb$ that starts computing the coordinate map $m$ from coordinate to its neighbour in $c$ which then is used to compute the permutation $\vec{\sigma}$ of the slots in $C$.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{Coord} &= \text{Slot} \times \Nb \\
\text{CLoop} &= (\abst{y} : \Nb) \pto \text{Coord}^{k_{\abst{y}}} \\
\text{CMap} &= \text{Coord} \pto \text{Coord} \\
\text{loop} &: \text{Row} \to \GateType \to \text{CLoop} \\
\\
\text{CState}^{k,k'} &= \Nb \times \text{Coord}^{|\text{Slot}| \times k} \times \text{CMap} \times \\
&\Bb \times \text{CLoop} \times \text{GState}^{k'}\\
\\
\sqcup &: \text{CLoop} \to \text{CLoop} \to \text{CLoop} \\
x \sqcup y &= \begin{cases}
x & y = \bot \\
& \exists i. y(i) = \vec{l} \\
& y' = y[i \mapsto \bot] \\
x[i \mapsto x(i) \cat \vec{l}] \sqcup y'
& x(i) \neq \bot \\
x[i \mapsto \vec{l}] \sqcup y'
& \otherwise
\end{cases} \\
\\
\dagger_C &: \text{CState} \to \Bb \\
\dagger_C &= \lambda (N, \_, \_, b, c, \_). \\
&N = 0 \land b = \top \land c = \bot \\
\\
\iota_C &: \text{GState} \to \text{CState} \\
\iota_C(s) &= (0, (), \bot, \bot, \bot, s)
\end{array}
&
\begin{array}{rl}
\Downarrow_C &: \text{CState} \to \text{CState} \\
\Downarrow_C &= \lambda (N, \vec{\sigma}, m). \\
&\begin{cases}
(0, \vec{\sigma}, m) & N = 0 \\
& f = \lambda s.m(s, N) \\
(N-1, \sigma \cat \vec{\sigma},m)
& \sigma = f[(\text{Slot}..)]
\end{cases} \\
& \circ^\uparrow \lambda(N, \vec{\sigma}, m, b,c, C). \\
&\begin{cases}
& b \land c = \bot \\
(|C| / |\text{Term}|, (), m, \top, \bot, C)
& N = 0 \land  \vec{\sigma} = () \\
(N, \vec{\sigma}, m, b, c, C)
& \otherwise
\end{cases} \\
& \circ^\uparrow \lambda(m, b, c). \\
&\begin{cases}
& b \land \exists \abst{y}. c(\abst{y}) \neq \bot \\
& c' = c[\abst{y} \mapsto \bot] \\
& \vec{l} = l \cat \vec{l}' = c(\abst{y}) \\
(m', \top, c')
& m' = m[\vec{l} \mapsto \vec{l}' \cat l] \\
(m, b, c) & \otherwise
\end{cases} \\
& \circ^\uparrow \lambda (b, c,C,\vec{g}). \\
&\begin{cases}
& \neg b \land \vec{g} = g \cat \_ \\
& r = |C|/|\text{Term}| \\
(\bot, c \sqcup l, C, \vec{g})
& l = \text{loop}(r, \ty(g)) \\
(\top, c, C, \vec{g}) & \otherwise
\end{cases} \\
\end{array}
\end{array}
$$

TODO update for types; permutation $\vec{\sigma}$ per type, compute loop from ctrn

**Full $\Surkal$ Trace**

We conclude the full trace definition as follows:

$$
\begin{array}{cc}
\begin{array}{rl}
\text{res} &: \text{CState} \to \text{TraceResult} \\
\text{res} &= \lambda (\_, \vec{\sigma}, \_, \_, \_, C, \_, \_, \_, \_). (\vec{\sigma}, C) \\
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
\Downarrow^{\abst{f}} &= \Downarrow_C \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} \stackrel{\to}{\circ} \Downarrow_R^{\abst{f}} \\
\end{array}
&
\begin{array}{rl}
\iota &: \text{RState} \to \text{CState} \\
\iota &= \iota_C \circ \iota_G \\
\end{array}
&
\begin{array}{rl}
\text{trace} &: \AbsCirc \to \Nb^m \to \Fb^n_q \to \text{TraceResult} \\
\text{trace}(\abst{f}, \abst{\vec{Y}}, \vec{x})
&= \text{res} \circ \text{sup}(\Downarrow^{\abst{f}},\text{eq},\iota(s_0),\iota(s^{\abst{\vec{Y}}}_{\vec{x}}))
\end{array}
\end{array}
$$

**Iterative Fixpoint Compute**

The fixpoint of a monotone function $f$ notated $\mu s. f(s)$ can be computed using the kleene fixpoint theorem $\text{sup}_{n \in \Nb} f^n(s_0)$ where $n$ is the smallest $n$ such that $f^n(s_0) = f^{n+1}(s_0)$.

Or as per defined in $\text{trace}$ recursively with the call $\text{sup}(f,=,\bot,s_0)$. Now we present the iterative definition used in code implementations to circumvent the recursion limit or stack overflow errors.

\begin{algorithm}[H]
\caption*{
  \textbf{sup:} iterative kleene fixpoint theorem
}
\textbf{Inputs} \\
  \Desc{$f: \text{State} \to \text{State}$}{Monotonic function.} \\
  \Desc{$s_0 : \text{State}$}{Initial state.} \\
  \Desc{$\text{eq}: \text{State} \to \text{State} \to \Bb$}{Equality predicate on states.} \\
\textbf{Output} \\
  \Desc{$s : \text{State}$}{Fixpoint of $f$.}
\begin{algorithmic}[1]
  \State $s := \bot$
  \State $s' := s_0$ 
  \State \textbf{do:}
    \State \algind $s := s'$
    \State \algind $s' := f(s')$
    \State \textbf{while} $\neg\text{eq}(s,s')$
  \State \textbf{return} $s$
  \end{algorithmic}
\end{algorithm}


**Monotonicity Proof**

We can show that the function is monotonic by defining the order on the state, and showing that the function preserves the order. The order is defined as follows:

$$
(t,v,b,\vec{s}) \sqsubseteq (t',v',b',\vec{s'}) \iff
\begin{aligned}
  &t \not\sqsubseteq t' \Rightarrow \text{dom}(v) \not\subseteq \text{dom}(v') \Rightarrow |s| < |s'|
\end{aligned}
$$

We never remove the mappings in $v$ thus the order is preserved for $v$ despite the stack $s$ can grow and shrink. To show $t \sqsubseteq t'$ then is to investigate the remaining monotonic continuations for $\Surkal$.

TODO: cleanup and make full preorder relation definition, i.e. $s \sqsubseteq f(s)$

### Concrete Gate Definitions

TODO: all gates also does booleanity check, i.e. every (single output) gate has a flag to also assert if its bool

TODO assert WireType = {p,q} here onwards; bar notation $\bar{p} = q$

TODO: ctrn and loop too; term (in ctrn) includes $j$ lookup table index

| $g: \Gate$                | $\text{eval}(g, \vec{x})$     | remarks                 |
|:-------------------------:|:-----------------------------:|:------------------------|
| Input$_i()$               | $(x_i)$                       | from trace              |
| Const$_{s,p}()$           | $(s)$                         |                         |
| Add$(x,y)$                | $(x+y)$                       |                         |
| Mul$(x,y)$                | $(x \times y)$                |                         |
| Inv$(x)$                  | $(x^{-1})$                    |                         |
| Pow7$(x)$                 | $(x^7)$                       |                         |
| If$(b,x,y)$               | $(b ? x : y)$                 |                         |
| Lookup$_T(x,y)$           | $\maybe{(z)}{(x,y,z) \in T}$  |                         |
| PtAdd$(x_P,y_P,x_Q,y_Q)$  | $(x_R, y_R)$                  | Arkworks point add      |
| Poseidon$(a,b,c)$         | $(a',b',c')$                  | Mina poseidon 5 rounds  |
| Public$(x)$               | $()$                          |                         |
| Bit$(b)$                  | $()$                          |                         |
| IsAdd$(x,y,z)$            | $()$                          |                         |
| IsMul$(x,y,z)$            | $()$                          |                         |
| IsLookup$_T(x,y,z)$       | $()$                          |                         |

TODO Concrete lookup table definitions here as well

**Trace Correctness Example**

TODO
