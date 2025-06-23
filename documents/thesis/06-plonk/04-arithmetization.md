## Arithmetization

### Arithmetizer

We now define the preprocessing pipeline: $(R,x,w) = \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$

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

\begin{center}
\begin{tabular}{ c c c c }
\begin{math}
\begin{array}{rl}
(\abst{x}_1, \abst{x}_2, \ldots, \abst{x}_{n_g}) &= \tin(g) \\
\set{(g, \abst{y}_1), (g, \abst{y}_2), (g, \abst{y}_{m_g})} &\subseteq \abst{f}
\end{array}
\end{math}
&
$\Longleftrightarrow$
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=3.2cm, minimum height=1.35cm] (id-g) at (0,0) {};
\node at ($(id-g.north)-(0,0.975)$) {$\ty(g)$};
\node[anchor=center] (id-1) at ($(id-g.north west)+(0.4, -0.3)$) {$\abst{x}_1$};
\node[anchor=center] (id-2) at ($(id-g.north west)+(1.2, -0.3)$) {$\abst{x}_2$};
\node[anchor=center] (id-dots) at ($(id-g.north west)+(2.0, -0.3)$) {$\cdots$};
\node[anchor=center] (id-3) at ($(id-g.north west)+(2.8, -0.3)$) {$\abst{x}_{n_g}$};
\draw[-] ($(id-g.north west)+(0,-0.6)$) -- ($(id-g.north west)+(1.6,-0.6)$);
\draw[-] ($(id-g.north west)+(2.4,-0.6)$) -- ($(id-g.north east)+(0,-0.6)$);
\draw[-] ($(id-g.north west)+(0.8,0)$) -- ($(id-g.north west)+(0.8,-0.6)$);
\draw[-] ($(id-g.north west)+(1.6,0)$) -- ($(id-g.north west)+(1.6,-0.6)$);
\draw[-] ($(id-g.north west)+(2.4,0)$) -- ($(id-g.north west)+(2.4,-0.6)$);
\draw[->,thick] ($(id-g.north west)+(0.4,0.4)$) -- ($(id-g.north west)+(0.4,0)$);
\draw[->,thick] ($(id-g.north west)+(1.2,0.4)$) -- ($(id-g.north west)+(1.2,0)$);
\draw[->,thick] ($(id-g.north west)+(2.8,0.4)$) -- ($(id-g.north west)+(2.8,0)$);
\draw[->,thick] ($(id-g.south west)+(0.4,0)$) -- ($(id-g.south west)+(0.4,-0.4)$);
\draw[->,thick] ($(id-g.south west)+(1.2,0)$) -- ($(id-g.south west)+(1.2,-0.4)$);
\draw[->,thick] ($(id-g.south west)+(2.8,0)$) -- ($(id-g.south west)+(2.8,-0.4)$);
\node[anchor=center] (id-dots) at ($(id-g.south west)+(2.0, -0.3)$) {$\cdots$};
\node[anchor=center] (idy-1) at ($(id-g.south west)+(0.1, -0.3)$) {$\abst{y}_1$};
\node[anchor=center] (idy-2) at ($(id-g.south west)+(0.9, -0.3)$) {$\abst{y}_2$};
\node[anchor=center] (idy-3) at ($(id-g.south west)+(3.3, -0.3)$) {$\abst{y}_{m_g}$};
\end{tikzpicture}
&
e.g.
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, minimum width=1.6cm, minimum height=1.35cm] (id-g) at (0,0) {};
\node at ($(id-g.north)-(0,0.975)$) {$\text{Add}$};
\node[anchor=center] (id-1) at ($(id-g.north west)+(0.4, -0.3)$) {$\abst{a}$};
\node[anchor=center] (id-2) at ($(id-g.north west)+(1.2, -0.3)$) {$\abst{b}$};
\draw[-] ($(id-g.north west)+(0,-0.6)$) -- ($(id-g.north west)+(1.6,-0.6)$);
\draw[-] ($(id-g.north west)+(0.8,0)$) -- ($(id-g.north west)+(0.8,-0.6)$);
\draw[->,thick] ($(id-g.north west)+(0.4,0.4)$) -- ($(id-g.north west)+(0.4,0)$);
\draw[->,thick] ($(id-g.north west)+(1.2,0.4)$) -- ($(id-g.north west)+(1.2,0)$);
\draw[->,thick] ($(id-g.south)$) -- ($(id-g.south)+(0,-0.4)$);
\node[anchor=center] (idy-1) at ($(id-g.south)+(0.3, -0.3)$) {$\abst{c}$};
\end{tikzpicture}
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

**Arithmetize Correctness Example**

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


### Trace

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

### Relation

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
\text{relation} &: \text{TraceResult} \to R \\
\text{relation}(\vec{\sigma}, \vec{C}) &= \begin{cases}
a
& N = \text{pow2}(\max(|\vec{t}|, |\vec{C}|) + \text{blind})\\
& \omega = \text{unity}(N) \\
\end{cases}
\end{array}
$$

- compute $k : (t: \text{WireType}) \to \text{Slot} \to W(t)$; $k^q_s : \Fb_q$
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

**Relation Correctness Example**

TODO

### $\SurkalProver$

- destructure vanishing argument?
- split vanishing into prover and verifier via fiat shamir (poseidon hash for challenge)
- maybe define fiat shamir transformation of arguments above, so u can just make the calls here

- construct polys for vanishing argument
  - F_GC
  - grand products: F_CC1, F_CC2, F_PL1, F_PL2

TODO

### $\SurkalVerifier$

TODO

