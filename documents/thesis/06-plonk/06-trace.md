

## Trace

The trace computation is defined as follows:
$$
(C, \sigma) = \mathrm{trace}(\vec{x},\abst{f},\avec{Y})
$$

### Monotonic Functions

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

### Resolve

$\Downarrow_R$ computes the values of wires $\avec{Y}$ and input wires to assert gates given the input gates wire values $\vec{x}$. It does this by peeking from the stack $\avec{y}$, querying $\text{?}$ for unresolved input wires, otherwise it will evaluate the output wire values and cache it in the value map $v$ with $[\cdot]$. The continuation $f$ and stack pop $\curvearrowleft$ are called after.

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
v \text{?} \avec{y} &= \begin{cases}
() & \avec{y} = () \\
& \avec{y} = \abst{y} \cat \avec{y}' \\
\abst{y} \cat v \text{?} \avec{y}' & v(\abst{y}) = \bot \\
v \text{?} \avec{y}' & \otherwise
\end{cases} \\
\\
\left[ \cdot \right] &: \VMap \to \AbsCirc \to \Wire \to \VMap \\
v_{\abst{f}}\left[\abst{y}\right] &= \maybe{
  v[\avec{y} \mapsto \vec{y}]
}{\begin{array}{rl}
  \abst{f} &\ni \gpair{g}{\abst{y}} \\
  \avec{y} &= \out(\abst{f},g) \\
  \vec{y} &= \eval_g(v[\gin(g)]) \\
\end{array}}
\end{array}
&
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_R &: (T \times \RState \to T \times \RState) \to \AbsCirc \\
&\to T \times \RState \to T \times \RState \\
f \stackrel{\to}{\circ} \Downarrow^{\abst{f}}_R(t,v, \avec{y}) &= \begin{cases}
f(t,v,()) & \avec{y} = () \\
& \avec{y} = \abst{y} \cat \_ \\
\underset{R}{\curvearrowleft} (t, v, \avec{y}) & v(\abst{y}) \neq \bot \\
& \gpair{g}{\abst{y}} \in \abst{f} \\
(t, v, \avec{x} \cat \avec{y}) 
& \avec{x} = v \text{?} \gin(g) \neq () \\
\underset{R}{\curvearrowleft} \circ f(t, v_{\abst{f}}[\abst{y}], \avec{y}) 
& \otherwise
\end{cases} \\
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\dagger_R &: \RState \to \Bb \\
\dagger_R(\_, \avec{y}) &= |\avec{y}| = 0 
\end{array}
&
\begin{array}{rl}
s &: \Wire^m \to W[\tin{}] \to \RState \\
s^{\avec{Y}}_{\vec{x}} &= (\bot[(0..|\vec{x}|) \mapsto \vec{x}], \avec{Y} \cat \set{\abst{x} \middle\vert \gpair{g}{\bot} \in \abst{f} \land \abst{x} \in \gin(g) \setminus \avec{Y}})
\end{array}
&
\begin{array}{rl}
s_0 &: \RState \\
s_0 &= (\bot, ())
\end{array}
\end{array}
$$

### Gate Constraints

$\Downarrow_G$ computes the trace table $C$ by enqueing $\vec{g}$ the gadget (and its base recursively if any) with an output of the top of the $\vec{y}$ stack. The same gadget will not appear twice since $\Downarrow_R$ does not call the continuation on resolved wires and base duplicates are avoided by tracking added gadgets in $\Omega$. The pre-constraints of the gadgets are then resolved with vmap. Thus, tabulating the trace table.

\begin{tabularx}{\textwidth}{@{} r Y Y Y Y Y @{}}
\toprule
kind & Basic & Relative  & Asserts  & PublicInput  & \plonkup Table  \\
& & $b_g>0$ & $m_g=0$ & $\ty(g)=\text{PI}^t_i$ & $\ty(g)=\text{Tbl}^t_j$
\\\hline 
phase & $\phi=0$ & $\phi=0$ & $\phi=1$ & $\phi=2$ & $\phi=3$
\\\hline\\
join &
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, minimum height=0.75cm, anchor=north] (b3) at (b2.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$\base_g$};
\node[draw, minimum width=1.5cm, minimum height=0.75cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
& 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=0.75cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b4) at (b3.south) {$g_k$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50] (b1) at (0,0) {$g_1$};
\node[draw, minimum width=1.5cm, pattern=north west lines, pattern color=gray!50, anchor=north] (b2) at (b1.south) {$\vdots$};
\node[draw, minimum width=1.5cm, pattern=north east lines, pattern color=gray!50, anchor=north] (b3) at (b2.south) {$g_k$};
\node[draw, minimum width=1.5cm,minimum height=0.75cm, anchor=north] (b4) at (b3.south) {$\vdots$};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.5cm,minimum height=2.25cm] (b1) at (0,0) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern=north west lines, pattern color=gray!50] (b2) at (b1.north east) {$g_1$};
\node[draw, minimum width=0.6cm, minimum height=0.73cm, anchor=north west, pattern=north east lines, pattern color=gray!50] (b3) at (b2.south west) {$\vdots$};
\node[draw, minimum width=0.6cm, minimum height=0.75cm, anchor=north west, pattern=north west lines, pattern color=gray!50] (b4) at (b3.south west) {$g_k$};
\end{tikzpicture} \\
\\\toprule
\end{tabularx}

$$
\begin{array}{rl}
\begin{array}{rl}
A \cat B &= A \sqcup_{\lambda \_,\vec{a}, \vec{b}. \vec{a} \cat \vec{b}} B \\
\TraceTable &= \IndexMap(X, \lambda t,\_. W(t)^k) \\
\text{GState}^{k,k'} &= \TraceTable \times \pset{\Ggt} \times \Ggt^{k'} \\
&\times \Nb \times \RState^k \\
\vec{G}^{\abst{f}} &: \Ggt^j \times \Ggt^k \times \Ggt^l \\
\vec{G}^{\abst{f}} &= \left(\begin{array}{l}
  \left[g \middle\vert \gpair{g}{\abst{y}} \in \abst{f} \land
  \begin{array}{l}
    \exists i,t. \ty(g) = \Input^t_i \\
    \lor \abst{y} = \bot \land \ty(g) \neq \text{PI}^{\_}_\_
  \end{array}\right]\\
  \left[g \middle\vert \gpair{g}{\_} \in \abst{f} \land \exists i,t. \ty(g) = \text{PI}^t_i\right] \\
  \left[g \middle\vert \gpair{g}{\_} \in \abst{f} \land \exists j,t. \ty(g) = \text{Tbl}^t_j\right]
\end{array}\right) \\
\\
\Downarrow &: \AbsCirc \to \VMap \to F(\Cell(t,s)^k \to W(t)^k) \\
\Downarrow^{\abst{f}}_v(\_,\vec{r}) &= (\lambda (x, \avec{w}, f).f(x,v[\text{wires}^{\abst{f}}_g(\avec{w})]))[\vec{r}] \\
\\
\Downarrow &: \AbsCirc \to \Ggt \to \Ggt^k \\
\Downarrow^{\abst{f}}(g) &= \begin{cases}
\Downarrow^{\abst{f}}(g') \cat g & g' = \base^{\abst{f}}_g \neq \bot \\
(g)
\end{cases} \\
\\
\underset{G}{\curvearrowright} &: T \times \text{GState}^{k'',k} \to T \times \text{GState}^{k'',k'} \\
\underset{G}{\curvearrowright} &= \lift(\curvearrowright : \Ggt^k \to \Ggt^{k'})
\end{array} &
\begin{array}{rl}
- \stackrel{\to}{\circ} \Downarrow^{-}_G &: (T \times \text{GState} \to T \times \text{GState}) \to \AbsCirc \\
&\to T \times \text{GState} \to T \times \text{GState} \\
f \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} &= \lambda (C, \Omega, \vec{g}, \phi, v). \\
&\begin{cases}
& \vec{g} = \_ \cat g \\
\underset{G}{\curvearrowright} (C, \Omega, \vec{g}, \phi, v)
& g \in \Omega \\
& f' = \underset{G}{\curvearrowright} \circ f \circ^\uparrow \land \Omega' = \Omega \cup \set{g} \\
f' (C',\Omega', \vec{g},\phi,v) 
& \phi \leq 1 \Rightarrow C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn_g] \\
f' (C',\Omega', \vec{g},\phi,v) 
& \phi \leq 2 \Rightarrow C' = \Downarrow^{\abst{f}}_v[\ctrn_g] \cat C \\
\underset{G}{\curvearrowright} (C',\Omega', \vec{g},\phi,v) & \phi = 3 \land C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn_g] \\
f' (C, \Omega, (), \phi, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, \phi, v, \avec{y}). \\
&\begin{cases}
& \phi = 0 \land \avec{y} = \abst{y} \cat \_ \\
(\vec{g}\cat \Downarrow(g), 0, v, \avec{y})
& \gpair{g}{\abst{y}} \in \abst{f} \\
(\vec{G}^{\abst{f}}_{\phi +1}, \phi + 1, v, ())
& \phi < 3 \land |\avec{y}| = |\vec{g}| = 0 \\
((), 4, v, ()) & \phi = 3 \land |\avec{y}| = |\vec{g}| = 0 \\
(\vec{g}, \phi, v, ())
& \otherwise
\end{cases}
\end{array}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
\curvearrowright &: X^k \to X^{k'} \\
\curvearrowright (\vec{x}) &= \begin{cases}
() & \vec{x} = () \\
\vec{x}' & \vec{x} = \vec{x}' \cat \_ \\
\end{cases}
\end{array} &
\begin{array}{rl}
\dagger_G &: \text{GState} \to \Bb \\
\dagger_G &= \lambda(\_, \vec{g}, b, \_). |\vec{g}| = 0 \land b = 4
\end{array} &
\begin{array}{rl}
\iota_G &: \text{RState} \to \text{GState} \\
\iota_G(s) &= (\lambda \_.\bot[s \mapsto ()], \emptyset, (), 0, s)
\end{array}
\end{array}
$$

### Copy Constraints

TODO rephrase the following to new formalism

$\Downarrow_C$ computes coordinate loops; equivalence class of slot positions of $C$ modulo wire, by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate using $\sqcup$.

After computing the coordinate loop of the full circuit, we mark a flag $\Bb$ that starts computing the coordinate map $m$ from coordinate to its neighbour in $c$ which then is used to compute the permutation $\vec{\sigma}$ of the slots in $C$.

$$
\begin{array}{rl}
\text{CLoop} &= (\abst{w}: \Wire) \pto \text{Coord}^k \\
\text{CMap} &= \text{Coord} \pto \text{Coord} \\
\\
\text{perm} &: \text{CLoop} \to \text{CMap} \\
\text{perm}(l) &= \begin{cases}
\bot[x \mapsto x] & l = \bot \\
& \exists \abst{w}. \vec{s} = l(\abst{w}) \\
& \sigma = \text{perm}(l[\abst{w} \mapsto \bot]) \\
\sigma[\vec{s} \mapsto \vec{s}'] & s'_1 = s_{|\vec{s}|} \land s'_{i>1} = s_{i-1}
\end{cases} \\
\\
\text{loop} &: \Nb \to \Ggt \to \text{CLoop} \\
\text{loop}(o,g) &= \text{loopN}(o,\ctrn_g, \gin(g) \cat \out(\abst{f},g))[n_g+m_g+1]\\
\\
\sqcup &: \text{CLoop} \to \text{CLoop} \to \text{CLoop} \\
f \sqcup g &= \begin{cases}
\bot & \not\exists \abst{w}. f(\abst{w}) \neq \bot \lor g(\abst{w}) \neq \bot \\
& h = f[\abst{w} \mapsto \bot] \sqcup g[\abst{w} \mapsto \bot] \\
h[\abst{w} \mapsto f(\abst{w}) \cat g(\abst{w})] & f(\abst{w}) \neq \bot \land g(\abst{w}) \neq \bot \\
h[\abst{w} \mapsto f(\abst{w})] & f(\abst{w}) \neq \bot \\
h[\abst{w} \mapsto g(\abst{w})] & \land g(\abst{w}) \neq \bot \\
\end{cases}
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
\text{loop} &: \text{Row} \to \Ops \to \text{CLoop} \\
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

### Full $\Surkal$ Trace

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
\text{trace}(\abst{f}, \avec{Y}, \vec{x})
&= \text{res} \circ \text{sup}(\Downarrow^{\abst{f}},\text{eq},\iota(s_0),\iota(s^{\avec{Y}}_{\vec{x}}))
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

### Public variant

TODO