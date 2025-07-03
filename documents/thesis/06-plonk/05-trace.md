
### Trace

We define the trace computation as follows:
$$
(TODO) = \mathrm{trace}(\abst{f},\abst{\vec{Y}},\vec{x})
$$

**Gate Type**

First, we define $\text{GateType}$ as the operational structure i.e. $n,m,\vec{t^{in}},\vec{t^{out}}$. We now introduce:

- $\text{eval}$; computes the output values via the gate's canonical program
- $\text{ctrn}$; constructs the mapping of values to the gate constraints; $gc$.

$$
\begin{array}{rl}
\text{GateType}
&= (n: \Nb) \times (m: \Nb) \times (\vec{t^{in}}: \WireType^n) \times (\vec{t^{out}}: \WireType^m) \\
&\times (\text{eval}: W[\vec{t_{in}}] \to W[\vec{t_{out}}]) 
\times (\text{ctrn}: (t: \WireType) \to C_{row}(\vec{t^{in}} \cat \vec{t^{out}}, t)^k)
\end{array}
$$
$$
\begin{array}{rl}
C_{idx} &= (\vec{t}: \WireType^k) \to (t': \WireType) \to \set{i | i \in \Nb \land t_i = t'}\\
C_{val} &= (\vec{t}: \WireType^k) \to (t': \WireType) \to (C_{idx}(\vec{t}, t') + W(t)) \\
C_{row} &= (\vec{t}: \WireType^k) \to (t': \WireType) \to (C_{val}(\vec{t}, t')^{|\Slot| + |\Selector|})
\end{array}
$$

Recall $\Surkal$ performs vanishing argument on $F_{GC}$. The primtive terms in $F_{GC}$ are called slots and selectors. Slots; $A,B,C,\cdots$, hold values of a run of the circuit privately, wheras selectors; $Q_l, Q_r, Q_o, Q_c, \cdots$, are public values reflecting the structure of the circuit. The trace table has slots and selectors for columns and $\text{ctrn}$ constructs the rows. Thus $\forall t. W(t)$ is a valid field to construct a polynomial for the vanishing argument.

The construction of subterms of $F_{GC}$ is done by $\GateGroup$ via $gc$:

$$
\begin{array}{rl}
\GateGroup &= \GateType^k \times (gc: X^l \to X^{|\Slot| + |\Selector|} \to X)
\end{array}
$$

TODO

- extend group function?; takes a group and vector of gates returns new group.
- gate group construction needs to be done in reln.. not necessary in arith and trace

- join cloops (in copy constraint section)
- cloop; from ctrn

**Monotonic Functions**

$\text{trace}$ computes the least fixed point of a composition of monotonic functions using $\text{sup}$. We also call a monotonic function a continuation if it is called by another. We call lift, to extend the argument of a monotonic function.

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

The monotonic functions defined below are specific to the $\Surkal$ protocol. Thus if the arithmetizer abover were to be extended for a different \plonk-ish protocol, the functions would be different.

Note: for each monotonic function below, we notate $\dagger$ as a check if the state has saturated in which the fixpoint compute can terminate. Wheras $s$ is the initial state and $\iota$ a constructor of it. 

**Resolve**

$\Downarrow_R$ computes the values of wires $\abst{\vec{Y}}$ and inputs to assert gates given the input wire values $\vec{x}$.
 
It does this by peeking from the stack $\abst{\vec{y}}$, querying $\text{?}$ for unresolved input wires, otherwise it will evaluate the output wire values and cache it in the value map $v$ with $[\cdot]$. The continuation $f$ and stack pop $\curvearrowleft$ are called after.

Every gate type has an program for its output value(s). e.g. $\text{eval}(\text{Add}, (1,2)) = (3)$.

$$
\begin{array}{rl}
\text{eval} &: (g: \GateType) \to \Fb^{n_g}_q \to \Fb^{m_g}_q 
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\VMap &= \Nb \pto \Fb_q \\
\RState^k &= \VMap \times \Nb^k \\
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
\underset{R}{\curvearrowleft} &: T \times \Nb^k \to T \times \Nb^{k'} \\
\underset{R}{\curvearrowleft} &= \lift(\curvearrowleft)
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{?} &: \VMap \to \Nb^k \to \Nb^{k'} \\
v \text{?} \abst{\vec{y}} &= \begin{cases}
() & \abst{\vec{y}} = () \\
& \abst{\vec{y}} = \abst{y} \cat \abst{\vec{y}}' \\
\abst{y} \cat v \text{?} \abst{\vec{y}}' & v(\abst{y}) = \bot \\
v \text{?} \abst{\vec{y}}' & \otherwise
\end{cases} \\
\\
\left[ \cdot \right] &: \VMap \to \AbsCirc \to \Nb \to \VMap \\
v_{\abst{f}}\left[\abst{y}\right] &= \maybe{
  v[\abst{\vec{y}} \mapsto \vec{y}]
}{\begin{array}{rl}
  \abst{f} &\ni (g, \abst{y}) \\
  \abst{\vec{y}} &= \out(\abst{f}, g) \\
  \vec{y} &= \text{eval}(\ty(g), v[\tin(g)]) \\
\end{array}}
\end{array}
&
\begin{array}{rl}
\stackrel{\to}{\circ} \Downarrow_R &: (T \times \RState \to T \times \RState) \to \AbsCirc \\
&\to T \times \RState \to T \times \RState \\
f \stackrel{\to}{\circ} \Downarrow^{\abst{f}}_R(t,v, \abst{\vec{y}}) &= \begin{cases}
f(t,v,()) & \abst{\vec{y}} = () \\
 & \abst{\vec{y}} = \abst{y} \cat \_ \\
\underset{R}{\curvearrowleft} (t, v, \abst{\vec{y}}) & v(\abst{y}) \neq \bot \\
 & (g, \abst{y}) \in \abst{f} \\
 & \abst{\vec{x}} = v \text{?} \tin(g) \\
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
s &: \Nb^m \to \Fb_q^n \to \RState \\
s^{\abst{\vec{Y}}}_{\vec{x}} &= (\bot[(0..|\vec{x}|) \mapsto \vec{x}], \abst{\vec{Y}} \cat \set{\abst{x} \middle\vert (g, \bot) \in \abst{f} \land \abst{x} \in \tin(g) \setminus \abst{\vec{Y}}})
\end{array}
&
\begin{array}{rl}
s_0 &: \RState \\
s_0 &= (\bot, ())
\end{array}
\end{array}
$$

TODO update for types; use idx for vmap defn

**Gate Constraints**

TODO diagram for ctrn mapping of gate to rows in $\vec{C}$.. vector parallel to Term, values are index of input and output value vector.. this can be used for loop too?!?!?!

$\Downarrow_G$ computes the gate constraints by pushing the gate with an output of the top of the wire id stack via push; $\underset{G}{\curvearrowright}$. The same gate will not appear twice since we do not call the continuation on resolved wires in $\Downarrow_R$.

When the wire id stack $\abst{\vec{y}}$ is empty, $\underset{G}{\curvearrowright}$ will push assert gates and input gates $A^{\abst{f}}$ to the stack.
$$
\begin{array}{rl}
\begin{array}{rl}
\text{ctrn} &: (g : \GateType) \to \Fb_q^{n_g + m_g} \to \Fb_q^{|\text{Term}| \times k} \\
\\
\text{GState}^{k,k',k''} &= \Fb_q^{|\text{Term}| \times k''} \times \Gate^{k'} \times \Bb \times \RState^k \\
A^{\abst{f}} &= \set{g \middle\vert (g, \abst{y}) \in \abst{f} \land (\abst{y} = \bot \lor \exists i. \abst{y} = \text{Input}_i) } \\
\\
\underset{G}{\curvearrowleft} &: T \times \text{GState}^{k''',k,k''} \to T \times \text{GState}^{k''',k',k''} \\
\underset{G}{\curvearrowleft} &= \lift(\curvearrowleft : \Gate^k \to \Gate^{k'}) \\
\\
\dagger_G &: \text{GState} \to \Bb \\
\dagger_G(\_, \vec{g}, b, \_) &= |\vec{g}| = 0 \land b = \top \\
\\
\iota_G &: \text{RState} \to \text{GState} \\
\iota_G(s) &= ((), (), \bot, s)
\end{array}
&
\begin{array}{rl}
\stackrel{\to}{\circ} \Downarrow_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \\
&\to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \underset{G}{\curvearrowleft} \circ f \circ^\uparrow \lambda (\vec{C}, \vec{g}, b, v). \\
&\begin{cases}
& \vec{g} = g \cat \_ \\
& \vec{v} = v[\tin(g) \cat \out(\abst{f},g)] \\
(\vec{C}', \vec{g}, b, v)
& \vec{C}' = \vec{C} \cat \text{ctrn}(\ty(g), \vec{v}) \\
(\vec{C}, (), b, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, b, v, \abst{\vec{y}}). \\
&\begin{cases}
& b = \bot \\
(A^{\abst{f}} \cat (), \top, v, ())
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

TODO update for types; table $\vec{C}$ per type

TODO draw tikzpicture of gate correspond to rows in $\vec{C}$

**Copy Constraints**

$\Downarrow_C$ computes coordinate loops; equivalence class of slot positions of $\vec{C}$ modulo wire, by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate using $\sqcup$.

After computing the coordinate loop of the full circuit, we mark a flag $\Bb$ that starts computing the coordinate map $m$ from coordinate to its neighbour in $c$ which then is used to compute the permutation $\vec{\sigma}$ of the slots in $\vec{C}$.

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
& \circ^\uparrow \lambda(N, \vec{\sigma}, m, b,c, \vec{C}). \\
&\begin{cases}
& b \land c = \bot \\
(|\vec{C}| / |\text{Term}|, (), m, \top, \bot, \vec{C})
& N = 0 \land  \vec{\sigma} = () \\
(N, \vec{\sigma}, m, b, c, \vec{C})
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
& \circ^\uparrow \lambda (b, c,\vec{C},\vec{g}). \\
&\begin{cases}
& \neg b \land \vec{g} = g \cat \_ \\
& r = |\vec{C}|/|\text{Term}| \\
(\bot, c \sqcup l, \vec{C}, \vec{g})
& l = \text{loop}(r, \ty(g)) \\
(\top, c, \vec{C}, \vec{g}) & \otherwise
\end{cases} \\
\end{array}
\end{array}
$$

TODO update for types; permutation $\vec{\sigma}$ per type

**Full $\Surkal$ Trace**

We conclude the full trace definition as follows:

$$
\begin{array}{cc}
\begin{array}{rl}
\text{res} &: \text{CState} \to \text{TraceResult} \\
\text{res} &= \lambda (\_, \vec{\sigma}, \_, \_, \_, \vec{C}, \_, \_, \_, \_). (\vec{\sigma}, \vec{C}) \\
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
