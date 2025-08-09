

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
  \State \textbf{do:} $(s,s') := (s',f(s'))$ \textbf{while} $\neg\text{eq}(s,s')$
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

$\Downarrow_G$ computes the trace table $C$ by enqueing $\vec{g}$ the gadget (and its base recursively if any) with an output of the top of the $\vec{y}$ stack. The same gadget will not appear twice since $\Downarrow_R$ does not call the continuation on resolved wires and base duplicates are avoided by tracking added gadgets in $\Omega$. The pre-constraints of the gadgets are then resolved with vmap. Thus, tabulating the trace table.[^no-pad]

[^no-pad]: Note for $\phi=3$ that $\ctrn'_g$ without padding is used, this is how it can be appended as a column.

\begin{tabularx}{\textwidth}{@{} r Y Y Y Y Y @{}}
\toprule
kind & Basic & Relative  & Asserts  & PublicInput & \plonkup Table \\
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
& f' = \underset{G}{\curvearrowright} \circ f \circ^\uparrow \land \Omega' = \Omega \cup \set{g} \\
f' (C',\Omega', \vec{g},\phi,v) 
& \phi \leq 1 \Rightarrow C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn_g] \\
f' (C',\Omega', \vec{g},\phi,v) 
& \phi \leq 2 \Rightarrow C' = \Downarrow^{\abst{f}}_v[\ctrn_g] \cat C \\
\underset{G}{\curvearrowright} (C',\Omega', \vec{g},\phi,v) & \phi = 3 \land C' = C \cat \Downarrow^{\abst{f}}_v[\ctrn'_g] \\
f' (C, \Omega, (), \phi, v)
& \otherwise
\end{cases} \\
&\circ^\uparrow \lambda(\vec{g}, \phi, v, \avec{y}). \\
&\begin{cases}
& \phi = 0 \land \avec{y} = \abst{y} \cat \_ \\
(\Downarrow^{\abst{f}}(g) \cat \vec{g}, 0, v, \avec{y})
& \gpair{g}{\abst{y}} \in \abst{f} \land g \notin \Omega \\
(\vec{G}^{\abst{f}}_{\phi +1}, \phi + 1, v, ())
& \phi < 3 \land |\avec{y}| = |\vec{g}| = 0 \\
((), 4, v, ()) & \phi = 3 \land |\avec{y}| = |\vec{g}| = 0 \\
(\vec{g}, \phi, v, \avec{y})
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

$\Downarrow_C$ From $\ctrn_g$, we populate the *loop*; a vector modelling an equivalence class of *coordinates*; copy constraint column and row number, modulo wire, for every $g$ in the queue. After computing the loop of the full circuit, we compute the position permutation $\vec{\sigma}$.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{Coord} &= \text{CC} \times \Nb \\
\text{CLoop} &= (\abst{w}: \Wire) \pto \text{Coord}^k \\
\text{CMap} &= \text{Coord} \to \text{Coord} \\
\text{CState} &= (\text{CMap} + \text{CLoop}) \times \text{GState}^{k,k'} \\
\\
\sqcup &: \text{CLoop} \to \text{CLoop} \to \text{CLoop} \\
L_1 \sqcup L_2 &= \begin{cases}
& \exists \abst{w}. L_2(\abst{w}) \neq \bot \\
& l = L_1(\abst{w})?() \cat L_2(\abst{w}) \\
L & L = L_1[\abst{w} \mapsto l] \sqcup L_2[\abst{w} \mapsto \bot] \\
L_1 & \otherwise
\end{cases} \\
\\
\text{perm} &: \text{CLoop} \to \text{CMap} \\
\text{perm}(L) &= \lambda x. \begin{cases}
y & y = \text{perm}'(L)(x) \neq \bot \\
x & \otherwise
\end{cases} \\
\text{perm}'(L) &= \begin{cases}
\bot & l = \bot \\
& \exists \abst{w}. \vec{s} = L(\abst{w}) \\
& \sigma = \text{perm}'(L[\abst{w} \mapsto \bot]) \\
\sigma[\vec{s} \mapsto \vec{s}'] & s'_1 = s_{|\vec{s}|} \land s'_{i>1} = s_{i-1}
\end{cases}
\end{array} &
\begin{array}{c}
\begin{array}{rl}
\Downarrow &: \text{CLoop} \to \PreTable_g \to \Wire^{n_g +m_g} \to \Nb \to \text{CLoop} \\
\Downarrow_{L_\bot} &= \lambda((t, s, x, \vec{c}), \avec{w}, i). \\
&\begin{cases}
L_\bot & \vec{c} = () \\
& \vec{c} = c \cat \vec{c}' \land j = \text{cw}(c) \\
& L = \text{loop}(t,s,x,\vec{c}',\avec{w},i+1) \\
L[\abst{w}_j \mapsto \vec{s}]
& \vec{s} = L(\abst{w}_j)?() \cat (s,i) \land s \in \text{CC} \\
L & \otherwise
\end{cases} \\
\\
\Downarrow_C &: \AbsCirc \to \text{CState} \to \text{CState} \\
\Downarrow_C^{\abst{f}} &= \lambda (r, C, \Omega, \vec{g}). \\
&\begin{cases}
& r = \inr(L) \\
& \vec{g} = \vec{g}' \cat g \\
& \avec{w} = \gin(g) \cat \out(\abst{f},g) \\
(L', C, \Omega, \vec{g})  
& L' = \Downarrow_L(t,s,\ctrn^t_g(s),\avec{w}, |C^t(s)|) \\
(\sigma, C, \Omega, ()) & \sigma = \text{perm}(L) \\
(r,C,\Omega, ()) & \otherwise
\end{cases}
\end{array} \\
\begin{array}{cc}
\begin{array}{rl}
\dagger_C &: \text{CState} \to \Bb \\
\dagger_C &= \lambda (r, \_). r = \inl(\_)
\end{array} &
\begin{array}{rl}
\iota_C &: \text{GState} \to \text{CState} \\
\iota_C(s) &= (\inr(\bot), s)
\end{array}
\end{array}
\end{array}
\end{array}
$$

### Full $\Surkal$ Trace

We conclude the full trace definition as follows:

$$
\begin{array}{ccc}
\begin{array}{l}
\text{TraceResult} \\
= \text{CMap} \times \TraceTable
\end{array}&
\begin{array}{rl}
\text{res} &: \text{CState} \to \text{TraceResult} \\
\text{res} &= \lambda (\sigma, C, \_). (\sigma, C) \\
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
\Downarrow^{\abst{f}} &= \Downarrow^{\abst{f}}_C \stackrel{\to}{\circ} \Downarrow_G^{\abst{f}} \stackrel{\to}{\circ} \Downarrow_R^{\abst{f}} \\
\end{array}
&
\begin{array}{rl}
\iota &: \text{RState} \to \text{CState} \\
\iota &= \iota_C \circ \iota_G \\
\end{array}
&
\begin{array}{rl}
\text{trace} &: W[\tin{}] \to \AbsCirc \to \Wire^m \to \text{TraceResult} \\
\text{trace} &= \lambda(\vec{x}, \abst{f}, \avec{Y}). \text{res} \circ \text{sup}(\Downarrow^{\abst{f}},\text{eq},\iota(s_0),\iota(s^{\avec{Y}}_{\vec{x}}))
\end{array}
\end{array}
$$

### Public variant

The public variant for arithmetization only differs in trace. In $\Downarrow_R$, we do not have $\avec{x}: W[\tin{}]$ for input gates, but public input gates which is used to construct its column in the trace table. Thus, the vmap values are bools, that marks the wires having been resolved. This will lead to the same wire stack as the original $\Downarrow_R$, consequently trace table layout. $\Downarrow_G$ then will omit columns $c \in \text{priv}$ in $\ctrn_g$. Thus the cells that remain do not need the values to reduce, i.e. all the cells are constants. $\Downarrow_C$ remains the same. Resulting in a trace that differs by its trace table not having private columns.

**Trace Correctness Example**

TODO
