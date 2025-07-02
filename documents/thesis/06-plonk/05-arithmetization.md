## Preprocessing

We now define the preprocessing pipeline where $f : \text{Program}$ and $\text{Program} = W[\vec{t^{in}}] \to W[\vec{t^{out}}]$

$$
(R,x,w) = \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})
$$

Note: refer to the appendix for the definition of notations used in this section.

### Arithmetizer

*Wires* $\abst{x}$ are abstract representations of values $x$, defined as a pair of unique identifier; uuid, and a *wire type tag*. $W$ maps the tag to the value's type e.g. $W(p) = \Fb_p, W(q) = \Fb_q$.

$$
\begin{array}{cccc}
\begin{array}{rl}
\Wire &= \Nb \times \WireType \\
W &: \WireType \to \mathcal{U} \\
\end{array}
&
\begin{array}{rl}
\id &: \Wire \to \Nb \\
\id &= \lambda(i, \_). i \\
\end{array}
&
\begin{array}{rl}
\ty &: \Wire \to \WireType \\
\ty &= \lambda(\_, t). t \\
\end{array}
\end{array}
$$

*Gates* $g$ are primitive operations with $n_g \geq 0$ fan-in inputs and $m_g \geq 0$ fan-out outputs defined with its input wire(s).

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

Gate constructors type checks its inputs $\abst{\vec{x}}$. e.g. $\text{Add}(\abst{a},\abst{b}) = (\text{Add}, (\abst{a},\abst{b}))$ type checks $\abst{a}, \abst{b}$ for the $\text{Add}$ gate type.

$$
\begin{array}{cc}
\begin{array}{rl}
\text{inty} &: \GateType \to \WireType^{n_g} \\
\text{outty} &: \GateType \to \WireType^{m_g}
\end{array}
&
\begin{array}{rl}
- ( - ) &: (g: \GateType) \to \Wire^{n_g} \to \Gate \\
g(\abst{\vec{x}}) &= \maybe{(g,\abst{\vec{x}})}{\forall i. \text{inty}(g)_{i} = \ty(\abst{x}_i)}
\end{array}
\end{array}
$$

Arithmetize turns a program $f$ into an *Abstract Circuit* $\abst{f}$, which is a one-to-many-or-none relation between gates $g$ and output wire(s) $\abst{y}$ or $\bot$ for none. e.g. $(\text{Add}(\abst{a},\abst{b}), \abst{c}) \in \abst{f}$ corresponds to $\build{a+b=c}{}{}$. $\abst{f}$ are also acyclic.

$$
\AbsCirc = \set{
  \abst{f} \subset \Gate \times \Option(\Nb) \middle\vert
  \begin{array}{l}
  \forall (g,\abst{y}),(h,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
  \forall (g,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \land |\text{in}(g)| > 0 \implies \max(\id[\tin(g)]) < \min \left(\id[\out^{\abst{f}}(g)] \right)
  \end{array}
}
$$
A wire's output order relative to a gate and the output wires of a gate can be computed as follows:
$$
\begin{array}{rl}
\begin{array}{rl}
\idx &: \AbsCirc \to \Wire \to \Nb \\
\idx^{\abst{f}}(\abst{y}) &= \maybe{\id(\abst{y}) - \min\limits_{(g,\abst{w}) \in \abst{f}} \id(\abst{w})}{(g,\abst{y}) \in \abst{f}}
\end{array}
&
\begin{array}{rl}
\out &: \AbsCirc \to \Gate \to \Wire^{m_g} \\
\out^{\abst{f}}(g) &= \maybe{\abst{\vec{y}}}{\abst{y}_i \in \set{\abst{y} \middle\vert (g,\abst{y}) \in \abst{f}} \land \id(\abst{y}_{i>1}) = \id(\abst{y}_{i-1}) + 1}
\end{array}
\end{array}
$$

We can visualize a gate with an abstract circuit diagram:

\begin{center}
\begin{tabular}{ c c c c }
\begin{math}
\begin{array}{rl}
(\abst{x}_1, \ldots, \abst{x}_{n_g}) &= \tin(g) \\
\set{(g, \abst{y}_1), (g, \abst{y}_{m_g})} &\subseteq \abst{f}
\end{array}
\end{math}
&
$\Longleftrightarrow$
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{id}{(0,0)}{$\abst{x}_1$,$\cdots$,$\abst{x}_{n_g}$}{$\ty(g)$}{3}
\draw[-,thick] ($(id-in-1)+(0,0.25)$) -- (id-in-1);
\draw[-,thick] ($(id-in-3)+(0,0.25)$) -- (id-in-3);
\draw[->,thick] (id-out-1) -- ($(id-out-1)+(0,-0.4)$);
\node[anchor=north east] at (id-out-1) {$\abst{y}_1$};
\node[anchor=north] at ($(id-out-2)+(0,-0.1)$) {$\cdots$};
\draw[->,thick] (id-out-3) -- ($(id-out-3)+(0,-0.4)$);
\node[anchor=north west] at (id-out-3) {$\abst{y}_{m_g}$};
\end{tikzpicture}
&
e.g.
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{add}{(0,0)}{$\abst{a}$,$\abst{b}$}{$\text{Add}$}{1}
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] ($(add-in-2)+(0,0.25)$) -- (add-in-2);
\draw[->,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\node[anchor=north east] at (add-out-1) {$\abst{c}$};
\end{tikzpicture}
\end{tabular}
\end{center}

We notate arithmetizing a program $f$ with predicates $\build{f = \vec{y}}{s}{s'}$, $\build{f = y}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state; $s=(u, \abst{f})$ where $u$ is the current uuid, from $s$ to $s'$. Gadgets are compositions of $\bigwedge \build{f}{}{}$. Wires annotated with $*$, i.e. $\build{f = y^*}{}{}$, are the final output and are appended to $\abst{\vec{Y}}$. They, may be omitted notationally.

$$
\begin{array}{c}
\begin{array}{rl}
\AState = \Nb \times \AbsCirc \times \Wire^k &
\begin{array}{ll}
\build{-}{}{} &: \text{Program} \to \AState \to \AState \to \Bb \\
\build{-}{s}{s'} &: \text{Program} \to \Bb
\end{array}
\end{array} \\
\\
\begin{array}{rlrlrl}
u &: \AState \to \Nb &
\abst{f} &: \AState \to \AbsCirc &
\abst{\vec{Y}} &: \AState \to \Wire^k \\
u_{(r,\_)} &= r &
\abst{f}_{(\_,r,\_)} &= r &
\abst{\vec{Y}}_{(\_,r)} &= r
\end{array} \\
\\
\begin{array}{rlrlrl}
\build{g = \vec{y}}{s}{s'}
&= \left(\text{get}(s,g) = (s', \abst{\vec{y}})\right) &
\build{f=y^*}{s}{s'}
&= \build{f=y}{(s,\abst{\vec{Y}})}{(s', \abst{\vec{Y}} \cat \abst{y})} &
\build{f}{s_1}{s_{k+1}}
&= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}}
\end{array}
\end{array}
$$

Gates have a canonical program that it corresponds to, e.g $\build{x + y=z}{s}{s'} = \left(\text{get}(s,\text{Add}(\abst{x},\abst{y})) = (s', \abst{z})\right)$, thus a program can be arithmetized iff it can be decomposed into these canonical programs.

These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \abst{f}$, then $\abst{\vec{y}}$ in $\build{g=\vec{y}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

$$
\begin{array}{rl}
\begin{array}{rl}
\text{new} &: \Nb \to \Gate \to \Wire^{m_g} \\
\text{new}(u,g) &= (u..u+m_g) \odot \text{outty}(g) \\
\\
\entries  &: \Nb \to \Gate \to \AbsCirc \\
\entries(u,g) &= \begin{cases}
  \set{(g,\abst{y}) \middle\vert \abst{y} \in \text{new}(u,g)}
  & m_g > 0 \\
  \set{(g,\bot)}
  & m_g = 0
\end{cases} \\
\\
\aput &: \Gate \to \AState \to \AState \\
\aput(g, s) &= (
  u_s + m_g, \abst{f}_s \cup \entries(u_s, g), \abst{\vec{Y}}_s
)
\end{array}
&
\begin{array}{rl}
\Gate^{\abst{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
} \\
\aget &: \AState \to (g: \Gate) \to \AState \times \Wire^{m_g} \\
\aget(s, g)
&= \begin{cases}
  (s, \out(\abst{f}_s, h)) & h \in \Gate^{\abst{f}_s}_g \\
  (\aput(g, s), \text{new}(u_s,g)) & \otherwise
\end{cases} \\
\\
\text{arithmetize} &: (W[\vec{t^{in}}] \to W[\vec{t^{out}}]) \to \AbsCirc \times \Wire^{m'} \\
\text{arithmetize}(f) &= \maybe{(\abst{f}, \abst{\vec{Y}})}{
  \build{f}{\left(\opcirc\limits_{i \in \left[0..\left|\vec{t^{in}}\right|\right]}\aput(\text{Input}^{t^{in}_{i+1}}_i)\right)(0,\emptyset, ())}{(\_, \abst{f}, \abst{\vec{Y}})}
}
\end{array}
\end{array}
$$

**Arithmetize Correctness Example**

Let $W(q)=\Fb_q$ and $f: \Fb_q^2 \to \Fb_q^1$ where $f(x,y) = x^2 + y$, then:

\begin{longtable}{@{}l@{}}
Let $(\abst{f}, \abst{\vec{Y}})$
\\
$= \text{arithmetize}(f)$
\\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \build{x^2 + y = z^*}
    {(u, \abst{f},())}
    {(\_, \abst{f}'', (\abst{z}))}
}$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \abst{f}, ())}{(\_, \abst{f}'', (\abst{z}))}}$
\\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \abst{f},())}{(u', \abst{f}',())} \\
  \build{t + y = z^*}{(u', \abst{f}', ())}{(\_, \abst{f}'', (\abst{z}))}
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  \text{get}(u, \abst{f}, (), \text{Mul}(\abst{x},\abst{x})) &= (u', \abst{f}', (), (\abst{t})) \\
  \text{get}(u', \abst{f}', (), \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{\begin{array}{rl}
  (u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), (u,q))}, (), ((u,q))) &= (u', \abst{f}', (), (\abst{t})) \\
  \text{get}(u', \abst{f}', (), \text{Add}(\abst{t},\abst{y})) &= (\_, \abst{f}'', (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}'', (\abst{z})\right)}{
  \text{get}(u+1, \abst{f} \cup \set{(\text{Mul}(\abst{x},\abst{x}), (u,q))}, (), \text{Add}((u,q),\abst{y})) = (\_, \abst{f}'', (\abst{z}), (\abst{z}))
}
$ \\
$= \left(\abst{f} \cup \set{\begin{array}{rl}
    \text{Mul}(\abst{x},\abst{x}) & (u,q) \\
    \text{Add}((u,q),\abst{y}) & (u+1,q)
  \end{array}}, ((u+1,q))\right)
$ \\
where $(u,\abst{f},())$
\\ 
$= \opcirc\limits_{i \in [0..2]}\aput(\text{Input}^{t^{in}_{i+1}}_i)(0,\emptyset,())$
\\
$= \text{put}(\text{Input}^q_1) \circ \text{put}(\text{Input}^q_0, 0, \emptyset,())$
\\
$= \text{put}(\text{Input}^q_1, 1, \set{(\text{Input}^q_0, (0,q))}, ())$
\\
$= \left(2, \set{\begin{array}{rl}
  \text{Input}^q_0 & (0,q) \\
  \text{Input}^q_1 & (1,q)
\end{array}}, ()\right)$
\\
$\therefore \ (\abst{f}, \abst{\vec{Y}}) = \left(\set{\begin{array}{rl}
  \text{Input}^q_0 & (0,q) \\
  \text{Input}^q_1 & (1,q) \\
  \text{Mul}((0,q),(0,q)) & (2,q) \\
  \text{Add}((2,q),(1,q)) & (3,q)
\end{array}}, ((3,q))\right)
$
\end{longtable}

Thus $\abst{x} = (0,q)$, $\abst{y} = (1,q)$, $\abst{t} = (2,q)$ and $\abst{z} = (3,q)$. The resulting abstract circuit can be notated as follows:

\begin{tabularx}{\textwidth}{@{} c Y Y @{}}
\toprule
Predicate & One to Many or None Relation & Abstract Circuit Diagram
\\\hline \\
$\build{x^2+y=z^*}{}{}$ & 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, anchor=center] (in1) at (0,0) {$\text{Input}^q_0$};
\node[draw, anchor=center] (in2) at ($(in1.south)-(0,0.4)$) {$\text{Input}^q_1$};
\node[draw, anchor=center] (mul) at ($(in2.south)-(0,0.4)$) {$\text{Mul}(\abst{x},\abst{x})$};
\node[draw, anchor=center] (add) at ($(mul.south)-(0,0.4)$) {$\text{Add}(\abst{t},\abst{y})$};

\node[anchor=center] (x) at ($(in1.east)+(3.5,0)$) {$\abst{x}$};
\node[anchor=center] (y) at ($(x.south)-(0,0.4)$) {$\abst{y}$};
\node[anchor=center] (t) at ($(y.south)-(0,0.4)$) {$\abst{t}$};
\node[anchor=center] (z) at ($(t.south)-(0,0.4)$) {$\abst{z}$};
\node[anchor=west] (outs) at ($(z.east)+(-0.125,0.075)$) {$\in \abst{\vec{Y}}$};

\node[] (g) at ($(in1.north)+(0,0.4)$) {$g$};
\node[] (w) at ($(x.north)+(0,0.4)$) {$\abst{w}$};
\node[] (f) at ($($(g)!.5!(w)$)$) {$\abst{f}$};

\draw[-, dashed] (in1.east) -- (x.west);
\draw[-, dashed] (in2.east) -- (y.west);
\draw[-, dashed] (mul.east) -- (t.west);
\draw[-, dashed] (add.east) -- (z.west);
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{in0}{(0,0)}{}{$\text{Input}^q_0$}{1}
\gate{in1}{($(in0.north east)+(0.1,0)$)}{}{$\text{Input}^q_1$}{1}
\gate{mul}{($(in0.south west)+(0,-0.5)$)}{$\abst{x}$,$\abst{x}$}{$\text{Mul}$}{1}
\draw[-,thick] (in0-out-1) -- (mul-in-1);
\draw[-,thick] (in0-out-1) -- (mul-in-2);
\gate{add}{($(mul.north east)+(0.5,0)$)}{$\abst{t}$,$\abst{y}$}{$\text{Add}$}{1}
\draw[-,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.25)$);
\draw[-,thick] ($(mul-out-1)+(0,-0.25)$) -- ($(mul.south east)+(0.25,-0.25)$);
\draw[-,thick] ($(mul.south east)+(0.25,-0.25)$) -- ($(mul.north east)+(0.25,0.25)$);
\draw[-,thick] ($(mul.north east)+(0.25,0.25)$) -- ($(add-in-1)+(0,0.25)$);
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] (in1-out-1) -- (add-in-2);
\draw[-,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\node[draw, thick, circle, double, double distance=1pt, anchor=north] at ($(add-out-1)+(0,-0.4)$) {$\abst{z}$};
\end{tikzpicture}
\\
\\\toprule
\end{tabularx}