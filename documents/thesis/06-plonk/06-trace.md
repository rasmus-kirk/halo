

### Trace

The trace computation is defined as follows:
$$
(C, \vec{\sigma}, L) = \mathrm{trace}(\abst{f},\abst{\vec{Y}},\vec{x})
$$

**Monotonic Functions**

$\text{trace}$ computes the least fixed point of a composition of monotonic functions; $\Downarrow_R, \Downarrow_G, \Downarrow_C$, using $\text{sup}$. We also call a monotonic function a continuation if it is called by another. We call lift, to extend the argument of a monotonic function. By the Kleene fixpoint theorem, the least fixed point can be computed by iterating the function until saturation, i.e. when the state does not change anymore.

$$
\begin{array}{rl}
\begin{array}{rl}
\lift &: (T \to T') \\
&\to V \times T \times U \to V \times T' \times U\\
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
\begin{algorithm}[H]
\caption*{
  \textbf{sup:} iterative fixpoint theorem
}
\begin{algorithmic}[1]
  \State $(s,s') := (\bot, s_0)$
  \State \textbf{do:}
    \State \algind $(s,s') := (s',f(s'))$
    \State \textbf{while} $\neg\text{eq}(s,s')$
  \State \textbf{return} $s$
  \end{algorithmic}
\end{algorithm}

The monotonic functions defined here are specific to the $\Surkal$ protocol, i.e. it can be different for a different \plonk-ish protocol. For each monotonic function, we notate $\dagger$ as a check if the state has saturated. $s$ are the initial states and $\iota$ a constructor of it. 

**Resolve**

$\Downarrow_R$ computes the values of wires $\abst{\vec{Y}}$ and inputs to assert gates given the input wire values $\vec{x}$.[^respect]

[^respect]: all assert gates (gates with no output wires) declared in $\abst{f}$ are respected.

 
It does this by peeking from the stack $\abst{\vec{y}}$, querying $\text{?}$ for unresolved input wires, otherwise it will evaluate the output wire values and cache it in the value map $v$ with $[\cdot]$. The continuation $f$ and stack pop $\curvearrowleft$ are called after.

TODO relative gates

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

$\Downarrow_G$ computes the trace table $C$ by pushing the gate with an output of the top of the wire id stack via push; $\underset{G}{\curvearrowright}$. The same gate will not appear twice since we do not call the continuation (including $\Downarrow_G$), on resolved wires in $\Downarrow_R$. When the wire id stack $\abst{\vec{y}}$ is empty, $\underset{G}{\curvearrowright}$ will push assert gates and input gates $\vec{g}^{\abst{f}}$ to the stack. The pre-constraints of the gates are then resolved with vmap. Thus, tabulating the trace table.

TODO relative gates

$$
\begin{array}{rl}
\begin{array}{rl}
\TraceTable &= \IndexMap(X, \lambda t,\_. W(t)^k) \\
\text{GState}^{k,k'} &= \TraceTable \times \Gate^{k'} \times \Bb \times \RState^k \\
\vec{g}^{\abst{f}} &= \left[g \middle\vert (g, \abst{y}) \in \abst{f} \land (\abst{y} = \bot \lor \exists i,t. \abst{y} = \text{Input}^t_i) \right] \\
\\
\Downarrow &: \VMap \to F(\text{Pre}(t,s)^k \to W(t)^k) \\
\Downarrow_v(\_,\vec{r}) &= \text{reduce}[\vec{r}]\\
\text{reduce}(r) &= \begin{cases}
f(x,v[\abst{\vec{w}}]) & r = (x, \abst{\vec{w}}, f) \\
? & \otherwise
\end{cases} \\
\\
\underset{G}{\curvearrowleft} &: T \times \text{GState}^{k'',k} \to T \times \text{GState}^{k'',k'} \\
\underset{G}{\curvearrowleft} &= \lift(\curvearrowleft : \Gate^k \to \Gate^{k'})
\end{array} &
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \\
&\to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \underset{G}{\curvearrowleft} \circ f \circ^\uparrow \lambda (C, \vec{g}, b, v). \\
&\begin{cases}
& \vec{g} = g \cat \_ \\
(C',\vec{g},b,v)
& C' = C \cat \Downarrow_v[\ctrn_g] \\
(C, (), b, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, b, v, \abst{\vec{y}}). \\
&\begin{cases}
& b = \bot \\
(\vec{g}^{\abst{f}}, \top, v, ())
& |\abst{\vec{y}}| = |\vec{g}| = 0 \\
& \abst{\vec{y}} = \abst{y} \cat \_ \\
(g \cat \vec{g}, \bot, v, \abst{\vec{y}})
& (g,\abst{y}) \in \abst{f} \\
(\vec{g}, \top, v, ())
& \otherwise
\end{cases}
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\dagger_G &: \text{GState} \to \Bb \\
\dagger_G &= \lambda(\_, \vec{g}, b, \_). |\vec{g}| = 0 \land b = \top
\end{array} &
\begin{array}{rl}
\iota_G &: \text{RState} \to \text{GState} \\
\iota_G(s) &= (\lambda \_.\bot[s \mapsto ()], (), \bot, s)
\end{array}
\end{array}
$$

**Copy Constraints**

TODO rephrase the following to new formalism

$\Downarrow_C$ computes coordinate loops; equivalence class of slot positions of $C$ modulo wire, by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate using $\sqcup$.

After computing the coordinate loop of the full circuit, we mark a flag $\Bb$ that starts computing the coordinate map $m$ from coordinate to its neighbour in $c$ which then is used to compute the permutation $\vec{\sigma}$ of the slots in $C$.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{Coord} &= \Slot \times \Nb \\
\text{CLoop} &= (\abst{w}: \Wire) \pto \text{Coord}^{k_{\abst{w}}} \\
\text{CMap} &= \text{Coord} \pto \text{Coord} \\
\\
\text{perm} &: \text{CLoop} \to \text{CMap} \\
\text{perm}(l) &= \begin{cases}
\bot[x \mapsto x] & l = \bot \\
& \exists \abst{w}. \vec{s} = l(\abst{w}) \\
& l' = \text{perm}(l[\abst{w} \mapsto \bot]) \\
l'[\vec{s} \mapsto \vec{s}'] & s'_1 = s_{|\vec{s}|} \land s'_{i>1} = s_{i-1}
\end{cases}
\end{array} &
\begin{array}{rl}
\text{loop} &: \text{CLoop} \to \Nb \to \PreTable \to \text{CLoop} \\
\text{loop}(X, i, t, \vec{p}) &= \begin{cases}
X & \vec{p} = () \\
& \vec{p} = p \cat \vec{p}' \\
& l = \text{loop}(X,i+1,t,\vec{p}') \\
& \vec{s}_{\abst{w}} = \left[ (s,i) \middle\vert p(t,s)_2 = (\abst{w}) \right] \\
l[\abst{w} \mapsto \vec{s}_{\abst{w}}]
& l(\abst{w}) = \bot \\
l[\abst{w} \mapsto l(\abst{w}) \cat \vec{s}] & \otherwise
\end{cases}
\end{array}
\end{array}
$$

TODO diagram, ctrn, to loop, to perm like in excalidraw

TODO compute a CMap per wire type

1. start with loop bot
2. for each gate call it on ctrn
3. get i from TraceTable in gate constraints
4. At end of trace sup, map it with perm and into TypedIndexMap of perm columns.

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
