## Preprocessing

We now define the preprocessing pipeline where $f : \text{Program}$ and $\text{Program} = W[\vec{t_{in}}] \to W[\vec{t_{out}}]$

$$
(R,x,w) = \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})
$$

Note: we use alot of custom notations in this section, which are defined in the appendix.

### Arithmetizer

Wires $\abst{x}$ are abstract representations of values $x$, defined as a triple of unique identifier; uuid, output index of its gate and a wire type tag. $W$ maps the tag to the value's type e.g. $W(p) = \Fb_p, W(q) = \Fb_q$.

$$
\begin{array}{cccc}
\begin{array}{rl}
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
\ty &= \lambda(t, \_). t
\end{array}
&
\begin{array}{rl}
\tin &: (g: \Gate) \to \Wire^{n_g} \\
\tin &= \lambda (\_, \abst{\vec{x}}). \abst{\vec{x}} \\
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

$$
\AbsCirc = \set{
  \abst{f} \subset \Gate \times \Option(\Nb) \middle\vert
  \begin{array}{l}
  \forall (g,\abst{y}),(h,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
  \forall (g,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \land |\text{in}(g)| > 0 \implies \max(\id[\tin(g)]) < \min \left(\id[\out_{\abst{f}}(g)] \right)
  \end{array}
}
$$
$$
\begin{array}{rl}
\out &: \AbsCirc \to \Gate \to \Wire^{m_g} \\
\out_{\abst{f}}(g) &= \maybe{\abst{\vec{y}}}{\abst{y}_i \in \set{\abst{y} \middle\vert (g,\abst{y}) \in \abst{f}} \land i = \idx(\abst{y}_i) + 1}
\end{array}
$$

We can visualize a gate with an abstract circuit diagram:

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

We notate arithmetizing a program $f$ to an abstract circuit $\abst{f}$ with predicates $\build{f = \vec{y}}{s}{s'}$, $\build{f = y}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state; $s=(u, \abst{f})$ where $u$ is the current uuid, from $s$ to $s'$. Gadgets are compositions of $\bigwedge \build{f}{}{}$. Wires annotated with $*$, i.e. $\build{f = y^*}{}{}$, are the final output and are appended to $\abst{\vec{Y}}$. They, may be omitted notationally.

$$
\begin{array}{cc}
\AState = \Nb \times \AbsCirc \times \Option(\Wire^k)
&
\begin{array}{ll}
\build{-}{}{} &: \text{Program} \to \AState \to \AState \to \Bb \\
\build{-}{s}{s'} &: \text{Program} \to \Bb
\end{array}
\end{array}
$$
$$
\begin{array}{cccc}
\build{g = \vec{y}}{s}{s'}
= \left(\text{get}(s,g) = (s', \abst{\vec{y}})\right)
&
\build{f=y^*}{s}{s'} = \build{f=y}{(s,\abst{\vec{Y}})}{(s', \abst{\vec{Y}} \cat \abst{y})}
&
\build{f}{s_1}{s_{k+1}}
= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}}
\end{array}
$$

Gates have a canonical program that it corresponds to, e.g $\build{x + y=z}{s}{s'} = \left(\text{get}(s,\text{Add}(\abst{x},\abst{y})) = (s', \abst{z})\right)$, thus a program can be arithmetized iff it can be decomposed into these canonical programs.

These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \abst{f}$, then $\abst{\vec{y}}$ in $\build{g=\vec{y}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

TODO: fix below and ur done for typed arith; remove build notation, remove out, entries construct wires not just naturals vector, arith use proper general type

$$
\begin{aligned}
\Gate^{\abst{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
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

Note: $\text{Input}^t_i$ is a family of gates with no inputs and one output wire corresponding to an input of the final circuit. The list of gates available are defined at the end of the following subsection.

**Arithmetize Correctness Example**

Let $W(q)=\Fb_q$, in the following example of the arithmetization of $f(x,y) = x^2 + y$:

\begin{longtable}{@{}l@{}}
$\text{arithmetize}(f: \Fb_q^2 \to \Fb_q^1)
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \build{x^2 + y = z^*}
    {(u, \abst{f}) = (\text{put}(\text{Input}^q_0) \circ \text{put}(\text{Input}^q_1)(0, \emptyset), \emptyset)}
    {(\_, \abst{f}'', (\abst{z}))}
}$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \abst{f}, \emptyset)}{(\_, \abst{f}'', (\abst{z}))}}
= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \abst{f})}{(u', \abst{f}')} \\
  \build{t + y = z^*}{(u', \abst{f}', \emptyset)}{(\_, \abst{f}'', (\abst{z}))}
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  \text{get}(u, \abst{f}, \text{Mul}(\abst{x},\abst{x})) &= (u', \abst{f}', (\abst{t})) \\
  \text{get}(u', \abst{f}', \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  (u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), ((u,0,q)))}, (u)) &= (u', \abst{f}', (\abst{t})) \\
  \text{get}((u',0,q), \abst{f}', \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \text{get}(u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), (u,0,q))}, \text{Add}((u,0,q),\abst{y})) = (\_, \abst{f}'', (\abst{z}))
}
$ \\
$= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(\abst{x},\abst{x}) & (u,0,q) \\
    \text{Add}((u,0,q),\abst{y}) & (u+1,0,q)
  \end{array}}, ((u+1,0,q))\right)}{
  (u, \abst{f}) = \text{put}(\text{Input}^q_0) \circ \text{put}(\text{Input}^q_1)(0, \emptyset)
}
$ \\
$= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
  \text{Mul}((0,0,q),(0,0,q)) & (u,0,q) \\
  \text{Add}((u,0,q),\abst{y}) & (u+1,0,q)
\end{array}}, ((u+1),0,q)\right)}{
  (u, \abst{f}) = \text{put}(\text{Input}^q_1, 1, \set{(\text{Input}^q_0, (0,0,q))})
}
$\\
$= \maybe{\left(\abst{f} \cup \set{\begin{array}{rl}
  \text{Mul}((0,0,q),(0,0,q)) & (u,0,q) \\
  \text{Add}((u,0,q),(1,0,q)) & (u+1,0,q)
\end{array}}, ((u+1,0,q)) \right)}
{(u, \abst{f}) = \left(2, \set{\begin{array}{rl}
  \text{Input}_0 & (0,0,q) \\
  \text{Input}_1 & (1,0,q)
\end{array}}\right)}
$ \\
$= \left(\set{\begin{array}{rl}
  \text{Input}^q_0 & (0,0,q) \\
  \text{Input}^q_1 & (1,0,q) \\
  \text{Mul}((0,0,q),(0,0,q)) & (2,0,q) \\
  \text{Add}((2,0,q),(1,0,q)) & (3,0,q)
\end{array}}, ((3,0,q))\right)
$
\end{longtable}

This concludes with wires $\abst{x} = (0,0,q)$, $\abst{y} = (1,0,q)$, $\abst{t} = (2,0,q)$ and $\abst{z} = (3,0,q)$. $\abst{f}$ can be notated as:

\begin{tabularx}{\textwidth}{@{} c Y Y @{}}
\toprule
Predicate & Abstract Circuit Relation & Abstract Circuit Diagram
\\\hline \\
$\build{x^2+y=z^*}{}{}$ & 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, anchor=center] (in1) at (0,0) {$\text{Input}^q_0$};
\node[draw, anchor=center] (in2) at ($(in1.south)-(0,0.4)$) {$\text{Input}^q_1$};
\node[draw, anchor=center] (mul) at ($(in2.south)-(0,0.4)$) {$\text{Mul}((0,0,q),(0,0,q))$};
\node[draw, anchor=center] (add) at ($(mul.south)-(0,0.4)$) {$\text{Add}((2,0,q),(1,0,q))$};

\node[anchor=center] (x) at ($(in1.east)+(3.5,0)$) {$(0,0,q)$};
\node[anchor=center] (y) at ($(x.south)-(0,0.4)$) {$(1,0,q)$};
\node[anchor=center] (t) at ($(y.south)-(0,0.4)$) {$(2,0,q)$};
\node[anchor=center] (z) at ($(t.south)-(0,0.4)$) {$(3,0,q)$};
\node[anchor=west] (outs) at ($(z.east)+(-0.125,0.075)$) {$\in \abst{\vec{Y}}$};

\node[] (g) at ($(in1.north)+(0,0.4)$) {$g$};
\node[] (w) at ($(x.north)+(0,0.4)$) {$\abst{y}$};

\draw[-, dashed] (in1.east) -- (x.west);
\draw[-, dashed] (in2.east) -- (y.west);
\draw[-, dashed] (mul.east) -- (t.west);
\draw[-, dashed] (add.east) -- (z.west);
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
% input1 gate
\node[draw, anchor=north] (in1) at (0,0) {$\text{Input}^q_0$};

% input2 gate
\node[draw, anchor=north] (in2) at ($(in1.north)+(1.8,0)$) {$\text{Input}^q_1$};

% mul(x,x) = t gate
\node[draw, minimum width=2.4cm, minimum height=1.35cm, anchor=north west] (mul-g) at ($(in1.south)-(1.2,0.4)$) {};
\node at ($(mul-g.north)-(0,0.975)$) {$\text{Mul}$};
\node[anchor=center] (mul-1) at ($(mul-g.north west)+(0.6, -0.3)$) {$(0,0,q)$};
\node[anchor=center] (mul-2) at ($(mul-g.north west)+(1.8, -0.3)$) {$(0,0,q)$};
\draw[-] ($(mul-g.north west)+(0,-0.6)$) -- ($(mul-g.north west)+(2.4,-0.6)$);
\draw[-] ($(mul-g.north west)+(1.2,0)$) -- ($(mul-g.north west)+(1.2,-0.6)$);

% add(t,y) = z gate
\node[draw, minimum width=2.4cm, minimum height=1.35cm, anchor=north west] (add-g) at ($(mul-g.south)+(-0.6,-0.4)$) {};
\node at ($(add-g.north)-(0,0.975)$) {$\text{Add}$};
\node[anchor=center] (add-1) at ($(add-g.north west)+(0.6, -0.3)$) {$(2,0,q)$};
\node[anchor=center] (add-2) at ($(add-g.north west)+(1.8, -0.3)$) {$(1,0,q)$};
\draw[-] ($(add-g.north west)+(0,-0.6)$) -- ($(add-g.north west)+(2.4,-0.6)$);
\draw[-] ($(add-g.north west)+(1.2,0)$) -- ($(add-g.north west)+(1.2,-0.6)$);

% z output
\node[anchor=north] (z) at ($(add-g.south) + (0,-0.4)$) {$(3,0,q)$};
\node[anchor=west] (outs) at ($(z.east)+(-0.125,0.075)$) {$\in \abst{\vec{Y}}$};

% x wiring
\draw[-,thick] ($(mul-g.north)+(0,0.4)$) -- ($(mul-g.north)+(0,0.2)$);
\draw[-,thick] ($(mul-g.north west)+(0.6,0.2)$) -- ($(mul-g.north west)+(1.8,0.2)$);
\draw[->,thick] ($(mul-g.north west)+(0.6,0.2)$) -- ($(mul-g.north west)+(0.6,0)$);
\draw[->,thick] ($(mul-g.north west)+(1.8,0.2)$) -- ($(mul-g.north west)+(1.8,0)$);

% t wiring
\draw[->,thick] ($(mul-g.south)$) -- ($(mul-g.south)+(0,-0.4)$);

% y wiring
\draw[-,thick] ($(mul-g.north west)+(3.0,0.4)$) -- ($(mul-g.south west)+(3.0,-0.2)$);
\draw[-,thick] ($(mul-g.south west)+(3.0,-0.2)$) -- ($(mul-g.south west)+(2.4,-0.2)$);
\draw[->,thick] ($(mul-g.south west)+(2.4,-0.2)$) -- ($(add-g.north west)+(1.8,0)$);

% z wiring
\draw[->,thick] (add-g.south) -- (z.north);
\end{tikzpicture}
\\
\\\toprule
\end{tabularx}