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
