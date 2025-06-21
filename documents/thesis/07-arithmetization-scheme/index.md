## Arithmetize

We now define the preprocessing pipeline: $(R,x,w) = \mathrm{circuit} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$

Wires $\abst{x}$ are abstract representations of values $x$, defined as a triple of unique identifier; uuid, output index of its gate and a wire type tag. $W$ maps the tag to the value's type e.g. $W(p) = \Fb_p, W(q) = \Fb_q$.

$$
\begin{array}{cccc}
\begin{array}{rl}
\WireType &= \set{t_1, t_2, \ldots, t_n} \\
\Wire &= \Nb \times \Nb \times \WireType \\
W &: \WireType \to \mathcal{U} \\
\end{array}
&
\begin{array}{rl}
\id &: \Wire \to \Nb \\
\id &= \lambda(i, \_). i \\
\end{array}
&
\begin{array}{rl}
\idx &: \Wire \to \Nb \\
\idx &= \lambda(\_, i, \_). i \\
\end{array}
&
\begin{array}{rl}
\ty &: \Wire \to \WireType \\
\ty &= \lambda(\_, t). t \\
\end{array}
\end{array}
$$

Gates $g$ are primitive operations with $n_g \geq 0$ fan-in inputs and $m_g \geq 0$ fan-out outputs defined with its input wire(s).

$$
\begin{array}{rl}
\Gate &= (g: \GateType) \times \Wire^{n_g} \\
\end{array}
$$
$$
\begin{array}{ccc}
\begin{array}{rl}
n &: \Gate + \GateType \to \Nb \\
m &: \Gate + \GateType \to \Nb
\end{array}
&
\begin{array}{rl}
\ty &: \Gate \to \GateType \\
\ty(t, \_) &= t
\end{array}
&
\begin{array}{rl}
\tin &: (g: \Gate) \to \Wire^{n_g} \\
\tin(\_, \abst{\vec{x}}) &= \abst{\vec{x}} \\
\end{array}
\end{array}
$$

Gate constructors type check $\abst{\vec{x}}$. e.g. $\text{Add}(\abst{a},\abst{b}) = (\text{Add}, (\abst{a},\abst{b}))$ type checks $\abst{a}, \abst{b}$ for the $\text{Add}$ gate type.

$$
\begin{array}{cc}
\begin{array}{rl}
\text{inty} &: \GateType \to \WireType^{n_g} \\
\text{outty} &: \GateType \to \WireType^{m_g}
\end{array}
&
\begin{array}{rl}
- ( - ) &: (g: \GateType) \to \Wire^{n_g} \to \Gate \\
g(\abst{\vec{x}}) &= \maybe{(g,\abst{\vec{x}})}{\forall i \in [n_g]. \text{inty}(g)_{i} = \ty(\abst{x}_i)}
\end{array}
\end{array}
$$

Arithmetize turns a program $f$ into an abstract circuit $\abst{f}$, which is a one-to-many-or-none relation between gates $g$ and output wire(s) $\abst{y}$ or $\bot$ for none. e.g. $(\text{Add}(\abst{a},\abst{b}), \abst{c}) \in \abst{f}$ corresponds to $\build{a+b=c}{}{}$. $\abst{f}$ are also acyclic.

TODO maybe make command for tikz gates? then u can output_wires.tikz() to produce tikz code; change to top down direction to align with IVC diagram?

\begin{center}
\begin{tabular}{ c c c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)},
  wire/.style={->, thick},
  label/.style={font=\small},
  port/.style={draw, minimum width=0.5cm, minimum height=1cm, inner sep=1pt}
]

% Main Add box
\node[draw, minimum width=1.5cm, minimum height=2cm] (G) at (0,0) {};

% Add label centered
\node at ($(G.center)+(0.25,0)$) {Add};

% Embedded input ports (inside left side of Add box)
\node[port, anchor=west] (a) at ($(G.west)+(0,0.5)$) {$\abst{a}$};
\node[port, anchor=west] (b) at ($(G.west)+(0,-0.5)$) {$\abst{b}$};

% Input wires into embedded ports
\draw[wire] ($(a.west)-(0.4,0)$) -- (a.west);
\draw[wire] ($(b.west)-(0.4,0)$) -- (b.west);

% Output wire and label
\node[label, anchor=west] (c) at ($(G.east)+(0.5,0)$) {$\abst{c}$};
\draw[wire] (G.east) -- (c);

\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)},
  wire/.style={->, thick},
  label/.style={font=\small},
  port/.style={draw, minimum width=0.6cm, minimum height=0.5cm, inner sep=1pt}
]

% Main gate box
\node[draw, minimum width=1.8cm, minimum height=2cm] (G) at (0,0) {};
\node at ($(G.center)+(0.25,0)$) {$\ty(g)$};

% Embedded input ports (left side)
\node[port, anchor=west] (x1) at ($(G.west)+(0, 0.75)$) {$\abst{x}_1$};
\node[port, anchor=west] (x2) at ($(G.west)+(0, 0.25)$) {$\abst{x}_2$};
\node[label, anchor=west] at ($(G.west)+(0, -0.25)$) {$\vdots$};
\node[port, anchor=west] (xn) at ($(G.west)+(0, -0.75)$) {$\abst{x}_{n_g}$};

% Input wires
\draw[wire] ($(x1.west)-(.4,0)$) -- (x1.west);
\draw[wire] ($(x2.west)-(.4,0)$) -- (x2.west);
\draw[wire] ($(xn.west)-(.4,0)$) -- (xn.west);

% Output wires
\node[label, anchor=west] (y1) at ($(G.east)+(0.5, 0.75)$) {$\abst{y}_1$};
\node[label, anchor=west] (y2) at ($(G.east)+(0.5, 0.25)$) {$\abst{y}_2$};
\node at ($(G.east)+(0.5, -0.25)$) {$\vdots$};
\node[label, anchor=west] (ym) at ($(G.east)+(0.5, -0.75)$) {$\abst{y}_{m_g}$};

\draw[wire] (G.east |- y1) -- (y1);
\draw[wire] (G.east |- y2) -- (y2);
\draw[wire] (G.east |- ym) -- (ym);

\end{tikzpicture}
&
$\Longleftrightarrow$
&
\begin{math}
\begin{array}{rl}
(\abst{x}_1, \abst{x}_2, \ldots, \abst{x}_{n_g}) &= \tin(g) \\
\set{(g, \abst{y}_1), (g, \abst{y}_2), (g, \abst{y}_{m_g})} &\subseteq \abst{f}
\end{array}
\end{math}
\end{tabular}
\end{center}

We notate inserting a gate or gadget $f$ to the abstract circuit with predicates $\build{f = \vec{y}}{s}{s'}$, $\build{f = y}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state; $s=(u, \abst{f})$ where $u$ is the current uuid, from $s$ to $s'$. Composition via $\bigwedge \build{f}{}{}$ denotes gadgets.

Wires annotated with $*$, i.e. $\build{f = y^*}{}{}$, are the final output and are appended to $\abst{\vec{Y}}$. They, may be omitted notationally.

These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \abst{f}$, then $\abst{\vec{y}}$ in $\build{g=\vec{y}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

$$
\begin{aligned}
\AbsCirc &= \set{
  \abst{f} \subset \Gate \times \Option(\Nb) \middle\vert
  \begin{array}{l}
  \forall (g,\abst{y}),(h,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
  \forall (g,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies \max(\id[\tin(g)]) < \min \left(\set{\id(\abst{y}) \middle\vert (g, \abst{y}) \in \abst{f}} \right)
  \end{array}
} \\
\Gate^{\abst{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
\\
\AState &= \Nb \times \AbsCirc
\end{aligned}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\out &: (\Option(\Nb) + \AbsCirc) \to (g: \Gate) \to \Nb^{m_g} \\
\out(\bot, \_) &= () \\
\out(u,g) &= (u..u+m_g) \\
\out(\abst{f}, g)
&= \out(\min\left(
  \set{\abst{y} \middle\vert (g,\abst{y}) \in \abst{f}}
\right), g) \\
\\
\text{entries}  &: \Nb \to \Gate \to \AbsCirc \\
\text{entries}(u,g) &= \begin{cases}
  \set{(g,\abst{y}) \middle\vert \abst{y} \in \out(u,g)}
  & m_g > 0 \\
  \set{(g,\bot)}
  & m_g = 0
\end{cases} \\
\\
\text{put} &: \Gate \to \AState \to \AState \\
\text{put}(g, u, \abst{f}) &= (
  u + m_g, \abst{f} \cup \text{entries}(u, g)
)
\end{array}
&
\begin{array}{rl}
\text{get} &: \AState \to (g: \Gate) \to \AState \times \Nb^{m_g} \\
\text{get}(u, \abst{f}, g)
&= \begin{cases}
  (u, \abst{f}, \out(\abst{f}, h)) & h \in \Gate^{\abst{f}}_g \\
  (\text{put}(g, u, \abst{f}), \out(u,g)) & \otherwise
\end{cases} \\
\\
\build{g = \vec{y}}{s}{s'}
&= \left(\text{get}(s,g) \overset{?}{=} (s', \abst{\vec{y}})\right)  \\
\build{f=y^*}{s}{s'} &= \build{f=y}{(s,\abst{\vec{Y}})}{(s', \abst{\vec{Y}} \cat \abst{y})} \\
\build{f}{s_1}{s_{k+1}}
&= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}} \\
\\
\text{arithmetize} &: (\Fb^n_q \to \Fb^m_q) \to \AbsCirc \times \Nb^{m'} \\
\text{arithmetize}(f) &= \maybe{(\abst{f}, \abst{\vec{Y}})}{
  \build{f}{\left(\circ_{i \in [0..n]}\text{put}(\text{Input}_i)(0,\emptyset), () \right)}{(\_, \abst{f}, \abst{\vec{Y}})}
}
\end{array}
\end{array}
$$

Note: $\text{Input}_i$ is a family of gates with no inputs and one output wire corresponding to an input of the final circuit. The list of gates available are defined at the end of the following subsection.

TODO update for types; out uses idx, Input has type tag

### Correctness Example

Example of the arithmetization of $\build{x^2 + y}{}{}$
$$
\begin{aligned}
&\text{arithmetize}((x,y): \Fb_q^2 \mapsto (x^2 + y): \Fb_q^1)
\\
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \build{x^2 + y = z^*}
    {(u, \abst{f}) = (\text{put}(\text{Input}_0) \circ \text{put}(\text{Input}_1)(0, \emptyset), \emptyset)}
    {(\_, \abst{f}'', (\abst{z}))}
  }
\\
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \abst{f}, \emptyset)}{(\_, \abst{f}'', (\abst{z}))}}
\\
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \abst{f})}{(u', \abst{f}')} \\
  \build{t + y = z^*}{(u', \abst{f}', \emptyset)}{(\_, \abst{f}'', (\abst{z}))}
\end{array}}
\\
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  \text{get}(u, \abst{f}, \text{Mul}(\abst{x},\abst{x})) &= (u', \abst{f}', (\abst{t})) \\
  \text{get}(u', \abst{f}', \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}))
\end{array}}
\\ 
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  (u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), u)}, (u)) &= (u', \abst{f}', (\abst{t})) \\
  \text{get}(u', \abst{f}', \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}))
\end{array}}
\\
&= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \text{get}(u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), u)}, \text{Add}(u,\abst{y})) = (\_, \abst{f}'', (\abst{z}))
}
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(\abst{x},\abst{x}) & u \\
    \text{Add}(u,\abst{y}) & u+1
  \end{array}}, (u+1)\right)}{
  (u, \abst{f}) = \text{put}(\text{Input}_0) \circ \text{put}(\text{Input}_1)(0, \emptyset)
}
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,\abst{y}) & u+1
  \end{array}}, (u+1)\right)}{
    (u, \abst{f}) = \text{put}(\text{Input}_1, 1, \set{(\text{Input}_0, 0)})
  }
\\
&= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,1) & u+1
  \end{array}}, (u+1) \right)}
  {(u, \abst{f}) = \left(2, \set{\begin{array}{rl}
    \text{Input}_0 & 0 \\
    \text{Input}_1 & 1
  \end{array}}\right)}
\\
&= \left(\set{\begin{array}{rl}
  \text{Input}_0 & 0 \\
  \text{Input}_1 & 1 \\
  \text{Mul}(0,0) & 2 \\
  \text{Add}(2,1) & 3
\end{array}}, (3)\right)
\end{aligned}
$$

TODO use types for wires

## Trace

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

Note: for each monotonic function below, we notate $\dagger$ as a check if the state has saturated in which the fixpoint compute can terminate. Wheras $s$ is the initial state and $\iota$ a constructor of it.

### Resolve

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

### Gate Constraints

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

### Copy Constraints

$\Downarrow_C$ quotients an ordered set; coordinate loop, of slot positions of $\vec{C}$ by the wire id corresponding to the value there.

This is done by peeking $\vec{g}$ and joining $c$ with the coordinate loop of the gate using $\sqcup$. This corresponds to $\mathtt{ctrn}$.

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

### Full $\Surkal$ Trace

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

### Iterative Fixpoint Compute

Trace is defined as a composition of monotonic functions that has control over their continuations. Thus if the full composition is $f$, then the trace is $\mu x. f(x)$. Given an initial state, it is notated as the supremum. $\text{sup}_{n \in \Nb} f^n(s_0)$, where $n$ is the smallest $n$ such that $f^n(s_0) = f^{n+1}(s_0)$, i.e. the least fixedpoint of $f$. We have shown the recursive definition before. Now we present the iterative definition which will be useful in code implementations to circumvent the recursion limit or stack overflow errors.

\begin{algorithm}[H]
\caption*{
  \textbf{sup:} iterative kleene least fixedpoint protocol.
}
\textbf{Inputs} \\
  \Desc{$f: \text{State}^T \to \text{State}^T$}{Monotonic function.} \\
  \Desc{$s_0 : \text{State}^T$}{Initial state.} \\
  \Desc{$\text{eq}: \text{State}^T \to \text{State}^T \to \Bb$}{Equality predicate on states.} \\
\textbf{Output} \\
  \Desc{$s_n : \text{State}^T$}{The state corresponding to the least fixedpoint of $f$.}
\begin{algorithmic}[1]
  \State Initialize variables:
    \Statex \algind $s := \bot$
    \Statex \algind $s' := s_0$ 
  \State Recursive compute:
    \Statex \textbf{do:}
    \Statex \algind $s := s'$
    \Statex \algind $s' := f(s')$
    \Statex \textbf{while} $\text{eq}(s,s') = \bot$
  \State Return the least fixedpoint:
    \Statex \textbf{return} $x$
  \end{algorithmic}
\end{algorithm}


### Monotonicity Proof

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

### Correctness Example

TODO

## Circuit

- prereq explainer
  - $\omega$: roots of unity / fft
  - $k_s$: cosets as id for slots

$$
\begin{array}{rl}
\text{Term} &= \text{Slot} + \text{Selector} + \set{J} \\
\text{Lookup} &= \set{T, F, H_1, H_2} \\
\text{PolyType} &= \text{Slot} + \text{Selector} + \text{Lookup} \\
\\
\text{pow2} &: \Nb \to \Nb \\
\text{pow2}(n) &= 2^{\lceil \log_2 (n) \rceil} \\
\\
\text{unity} &: \Nb \to \Fb_q \\
\text{unity}(N) &= \maybe{\omega}{
\begin{array}{rl}
  \omega &\in \Fb_q \\
  \omega^N &= 1 \\
  \forall k \in [N]. \omega^k &\neq 1
\end{array}
}\\
\\
\text{circuit} &: \text{TraceResult} \to R \\
\text{circuit}(\vec{\sigma}, \vec{C}) &= \begin{cases}
a
& N = \text{pow2}(\max(|\vec{t}|, |\vec{C}|) + \text{blind})\\
& \omega = \text{unity}(N) \\
\end{cases}
\end{array}
$$

- compute $h$; $k_s : \Fb_q^{|\text{Slot}|}$
- lookup thunk
  - up to $N$ minus blind rows
  - table vector $\vec{t}$
  - query vector $\vec{f}$ using $\vec{C}$ in $A,B,C,j$
  - grand product $\vec{h_1}, \vec{h_2}$
- expand $\vec{C}$
- expand $\vec{\sigma}$
- fft + cache
- commits
  - look at code

notation ideas

- $w[A,i] = \vec{C}[A,i]$ cache
- $R[Q_l,i] = \vec{C}[Q_l,i]$ cache
- $R[Q_l] = \text{fft}(\vec{C}[Q_l])$ poly
- $x[A] = \bot$ does not exist
- $w_\zeta[T,i] = ?[T,i]$ thunk cache
- $w_\zeta[T] = \text{fft}(?[T])$ thunk poly
- $w[\mathcal{C}_A] = \PCCommit(\text{fft}(\vec{C}[A]), \ldots)$ commit
- $w_\zeta[\mathcal{C}_T] = \PCCommit(\text{fft}(?[T]), \ldots)$ commit thunk
- $w[t,A,i]$ typed indexing (or maybe we dont need to if we split into two runs of prover and verifier)

notation for finite type indexing of vectors / matrices / tensors

### Correctness Example

TODO

\appendix

# Notation

TODO make to a neat table, and include notation in plonk report

types and type formers

- universe/ type of all types $\mathcal{U}$
- naturals $\Nb$
- pointed type $T_\bot$, has an (additional) smallest element $\bot$
- finite fields $\Fb_q$
- vector type $T^n$
- matrix / tensor type $T^{n \times m}$
- tuple / product type $T \times U$
- function type $X \to Y$
- partial function type $X \pto Y$
- disjoint union / sum type $T + U$

term constructors

- empty vector / unit tuple $()$
- vector term / tuple term $\vec{x} = (x_1, x_2, \cdots , x_n)$
- vector append / cons $y \cat \vec{x} = (y, x_1, x_2, \cdots x_n), \vec{x} \cat y = (x_1, x_2, \cdots, x_n, y)$
- vector of enumeration of a finite ordered type $(X..) = (x_1, x_2, \ldots x_n)$
- matrix / tensors as vectors $\vec{m}: T^{w \times h}, \vec{m}[i,j] = m_{i + h(j-1)}$
- function term / lambda abstraction $\lambda x. f(x)$
- function term by evaluations $\lambda[x \mapsto f(x)]$, implying $f(x)$ is evaluated upon construction for all $x$
- empty partial function $\bot$
- partial function append $f[x \mapsto y]$
- disjoint union implictly has no constructors, however we can $\text{inl}(t), \text{inr}(u)$ to avoid ambiguity

util functions

- maybe notation $\maybe{x}{\phi(x)} = \begin{cases} x & \phi(x) \\ \bot & \otherwise \end{cases}$
- maybe with default $\maybe{x \lor y}{\phi(x)} = \begin{cases} x & \phi(x) \\ y & \otherwise \end{cases}$
- vector of naturals builder $(s..t) = \begin{cases} () & t \leq s \\ s \cat (s+1 .. t) \end{cases}$
- vector concat $\vec{x} \cat \vec{y} = \begin{cases} \vec{y} & \vec{x} = () \\ \vec{x}' \cat (x \cat \vec{y}) & \vec{x} = \vec{x'} \cat x \end{cases}$
- vector concat with set $X \cat \vec{x}$; any random ordering of $X$; recursive application of axiom of choice
- vector map $f[\vec{x}] = (f(x_1), f(x_2), \ldots, f(x_n))$
- vector minus set $\vec{x} \setminus X$ turns $\vec{x}$ to a set and removes all elements in $X$
- min of a set with total ordering $\min(X)$
- partial function append vector $f[\vec{x} \mapsto \vec{y}] = \begin{cases} & \vec{x} = x \cat \vec{x}' \\ f[x \mapsto y][\vec{x}' \mapsto \vec{y}'] & \vec{y} = y \cat \vec{y}' \\ f & \otherwise \end{cases}$

identities

- associative product and function types
- unit type as identity for product types $T \times () = T$ i.e. $(t,()) = (t)$
- currying $T \to U \to V = (T \times U) \to V$
- curried / associative tuples $((a,b),c) = (a,b,c) = (a,(b,c))$

set theoretic notations

- set of naturals from one $[n] = \set{1,2,\ldots,n-1}$
- set of naturals with lower bound $[n..m] = \set{n,n+1,\ldots,m-1}$
- flattened case notation, conditions are propagated to conditions below if they don't contradict.
- if a case has no term, the next termed case must satisfy it, but subsequent cases need not (note the $\land \phi_2(a))$
$$
\begin{array}{rl}
\begin{cases}
a & \phi_1(a) \\
 & \phi_2(a) \\
b & \phi_3(b) \\
c & \phi_4(c) \\
\vdots
\end{cases} &=
\begin{cases}
a & \phi_1(a) \\
b(a) & (\phi_3(b(a)) \lor \phi_1(a)) \land \phi_2(a) \\
c(b(a),a) & \phi_4(c(b(a),a)) \lor \phi_1(a) \lor \phi_2(a) \lor \phi_3(b(a)) \\
\vdots
\end{cases}
\end{array}
$$

conventions

- $\abst{x}$ is an abstract of a thing, e.g. $\abst{f}$ is an abstract circuit, $\abst{y}$ is an abstract value / wire


