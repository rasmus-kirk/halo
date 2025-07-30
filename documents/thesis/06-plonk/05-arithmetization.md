## Preprocessing

We now define the preprocessing pipeline[^notation]:

$$
\begin{array}{rl}
(R,x,w) 
&= \SurkalPreprocess(f,\vec{x}) \\ 
&= \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})
\end{array}
$$

[^notation]: refer to the appendix for the definition of notations used in this section.


### Arithmetizer

We define the arithmetize computation as follows:
$$
(\abst{f}, \avec{Y}) = \mathrm{arithmetize}(f)
$$

**Abstract Circuit, Chips, Operations and Wires**

Arithmetize turns a program $f$ into an *abstract circuit* $\abst{f}$, which is a one-to-many-or-none relation between chips $g$ and output wire(s) $\abst{y}$ or $\bot$ for none, inducing an acyclic circuit. e.g. $(\chip{Add}{\abst{a},\abst{b}}, \abst{c}) \in \abst{f}$ corresponds to $\build{a+b=c}{}{}$.

$$
\begin{array}{rl}
\text{OneToManyOrNone}(\abst{f}) &= \forall (g,\abst{y}),(h,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
\text{Acyclic}(\abst{f}) &= \forall (g,\abst{y}) \in \abst{f}. \abst{y} \neq \bot \land |\text{in}(g)| > 0 \implies \max(\id[\gin(g)]) < \min \left(\id[\out^{\abst{f}}(g)] \right) \\
\AbsCirc &= \set{
  \abst{f} \middle\vert
  \abst{f} \subset \Chip \times \Option(\Wire) \land
  \text{OneToManyOrNone}(\abst{f}) \land
  \text{Acyclic}(\abst{f})
}
\end{array}
$$

*Operations*; amongst other data, defines $n_g \geq 0$ number of fan-in wires typed $\tin{g}$ and $m_g \geq 0$ number of fan-out wires typed $\tout{g}$ that a chip of its operation must have. Wires are checked when constructing a chip e.g. $\chip{Add}{\abst{a},\abst{b}}$ type checks $\abst{a}, \abst{b}$ for the $\text{Add}$ operation.

$$
\begin{array}{cc}
\begin{array}{ccc}
g: \Ops &
\begin{array}{rl}
n_g &: \Nb \\
m_g &: \Nb
\end{array} &
\begin{array}{rl}
\tin{g} &: \WireType^{n_g} \\
\tout{g} &: \WireType^{m_g}
\end{array}
\end{array} &
\begin{array}{rl}
- ( - ) &: (g: \Ops) \to \Wire^{n_g} \to \Chip \\
g(\avec{x}) &= \maybe{g'}{
\begin{array}{l}
\ty(g') = g \land \gin(g') = \avec{x} \\
\tin{g} = \ty[\avec{x}] \land \cdots
\end{array}} = \chipu{g}{\abst{x}_1, \cdots, \abst{x}_{n_g}}
\end{array}
\end{array}
$$

*Chips*[^short-hand-chip] $g$ are operations $\ty(g)$ initialized with input wires $\gin(g)$. *Wires* $\abst{x}$; a unique identifier $\id(\abst{x})$ and *wire type tag* $\ty(\abst{x})$, are abstract representations of values $x: W(\ty(\abst{x}))$. $W$ maps the tag to the value's type e.g. $W(p) = \Fb_p$.

[^short-hand-chip]: As a notational shorthand, if $g:\Chip$, we may omit $\ty$ e.g. $n_g := n_{\ty(g)}$. We notate $g$ as operation or chip interchangably.


$$
\begin{array}{cc}
\begin{array}{c}
\begin{array}{ccc}
g: \Chip &
\ty(g): \Ops &
\gin(g): \Wire^{n_g}
\end{array} \\
\out^{\abst{f}}(g): \Wire^{m_g} = \maybe{\avec{y}}{
\begin{array}{l}
\forall i \in [m_g+1]. \abst{y}_i \in \set{\abst{y} \middle\vert (g,\abst{y}) \in \abst{f}} \\
\id(\abst{y}_{i>1}) > \id(\abst{y}_{i-1})
\end{array}}
\end{array} &
\begin{array}{c}
\begin{array}{rl}
\abst{w}: \Wire &
W: \WireType \to \Uni \\
\id(\abst{w}) : \Nb &
\ty(\abst{w}) : \WireType
\end{array} \\
\wire{i}{t} = \new(i,t) = \maybe{\abst{w}}{\begin{array}{rl}
\id(\abst{w}) &= i \\
\ty(\abst{w}) &= t
\end{array}}
\end{array}
\end{array}
$$
Chips in $\abst{f}$ can be visualized as an *abstract circuit diagram*
\begin{center}
\begin{tabular}{ c c c c }
\begin{math}
\begin{array}{rl}
(\abst{x}_1, \ldots, \abst{x}_{n_g}) &= \gin(g) \\
\set{(g, \abst{y}_1), (g, \abst{y}_{m_g})} &\subseteq \abst{f}
\end{array}
\end{math}
&
$\Longleftrightarrow$
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{id}{(0,0)}{$\abst{x}_1$,$\cdots$,$\abst{x}_{n_g}$}{$g$}{3}
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

**Circuit Building**

We can also notate the abstract circuit $\abst{f}$ with *predicates* $\build{f = \vec{y}}{s}{s'}$, $\build{f = y}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the *state* $s$ to $s'$ where $u_s$ is the current uuid. Abstract circuit compositions are conjunctions of predicates $\bigwedge \build{f}{}{}$. Wires annotated with $*$, i.e. $\build{f = y^*}{}{}$, are the final output and are appended to $\avec{Y}$. They, may be omitted notationally.

$$
\begin{array}{cc}
\begin{array}{c}
\begin{array}{rl}
s: \AState &
u_s: \Nb \\
\abst{f}_s: \AbsCirc &
\avec{Y}_s: \Wire^k
\end{array} \\
\begin{array}{rl}
\new(u,\abst{f},\avec{Y}) &= \maybe{s'}{\begin{array}{rl}
u_{s'} &= u \\
\abst{f}_{s'} &= \abst{f} \\
\avec{Y}_{s'} &= \avec{Y}
\end{array}} \\
s \cat \abst{y} &= \new(u_s, \abst{f}_s, \abst{y} \cat \avec{Y}_s)
\end{array}
\end{array}
&
\begin{array}{rl}
\build{-}{-}{-} &: (W[\tin{}] \to W[\tout{}]) \to \AState \to \AState \to \Bb \\
\build{g = \vec{y}}{s}{s'}
&= \left(\text{get}(s,g) = (s', \avec{y})\right) \\
\build{f=y^*}{s}{s' \cat \abst{y}}
&= \build{f=y}{s}{s'} \\
\build{f}{s_1}{s_{k+1}}
&= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}}
\end{array}
\end{array}
$$

Operations have a *canonical program* that it corresponds to, e.g $\build{x + y=z}{s}{s'} = \left(\text{get}(s,\chip{Add}{\abst{x},\abst{y}}) = (s', \abst{z})\right)$, thus a program can be arithmetized iff it can be decomposed into these canonical programs. These inserts yield new wires. However, wires are reused by an equivalence class on chips.[^egglog-eq] If $g \equiv h$ where $(h,\_) \in \abst{f}$, then $\avec{y}$ in $\build{g=\vec{y}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged.

[^egglog-eq]: Determining equivalence between chips is a sophisticated problem, a candidate is to use equality saturation such as @egglog, however we implement simpler ad hoc solutions that doesnt cover the full equivalence structure. We leave this definition open.


$$
\Chip^{\abst{f}}_g = \set{h \in \Chip \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
\new &: \Nb \to \Chip \to \Wire^{m_g} \\
\new(u,g) &= \new[(u..u+m_g) \odot \tout{g}]
\end{array} &
\begin{array}{rl}
\entries  &: \Nb \to \Chip \to \AbsCirc \\
\entries(u,g) &= \begin{cases}
\set{(g,\abst{y}) \middle\vert \abst{y} \in \new(u,g)}
& m_g > 0 \\
\set{(g,\bot)} & \otherwise
\end{cases}
\end{array} \\ \\
\begin{array}{rl}
\aput &: \Chip \to \AState \to \AState \\
\aput(g, s) &= \new\left(\begin{array}{c}
u_s + m_g \\
\abst{f}_s \cup \entries(u_s, g) \\
\avec{Y}_s
\end{array}\right)
\end{array} &
\begin{array}{rl}
\aget &: \AState \to (g: \Chip) \to \AState \times \Wire^{m_g} \\
\aget(s, g)
&= \begin{cases}
  (s, \out^{\abst{f}_s}(h)) & h \in \Chip^{\abst{f}_s}_g \\
  (\aput(g, s), \new(u_s,g)) & \otherwise
\end{cases}
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\begin{array}{rl}
\text{init} &: \WireType^k \to \AState \\
\text{init}(\vec{t}) &= \opcirc\limits_{i \in [k+1]} \aput(\Input^{t_{i}}_{i-1}) (0, \emptyset, ()) \\
\end{array} &
\begin{array}{rl}
\text{arithmetize} &: (W[\tin{}] \to W[\tout{}]) \to \AbsCirc \times \Wire^{m'} \\
\text{arithmetize}(f) &= \maybe{(\abst{f}_s, \avec{Y}_s)}{
  \build{f}{\text{init}(\tin{})}{s}
}
\end{array}
\end{array}
$$

**Arithmetize Correctness Example**

Let $W(q)=\Fb_q$ and $f: \Fb_q^2 \to \Fb_q^1$ where $f(x,y) = x^2 + y$, then:

\begin{longtable}{@{}l@{}}
Let $(\abst{f}, \avec{Y}) = \text{arithmetize}(f)$
\\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{
  \build{x^2 + y = z}
    {s}
    {s''}
}
= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z
\end{array}}{s}{s''}}
= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\begin{array}{l}
  \build{x \times x = t}{s}{s'} \\
  \build{t + y = z^*}{s'}{s''}
\end{array}}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\begin{array}{rl}
  \text{get}(u_s, \abst{f}_s, (), \chip{Mul}{\abst{x},\abst{x}}) &= (u_{s'}, \abst{f}_{s'}, (), (\abst{t})) \\
  \text{get}(u_{s'}, \abst{f}_{s'}, (), \chip{Add}{\abst{t}, \abst{y}}) &= (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\begin{array}{rl}
  (u_s+1, \abst{f} \cup \set{\begin{array}{rl}\chip{Mul}{\abst{x},\abst{x}} & \wire{u_s}{q}\end{array}}, (), (\wire{u_s}{q})) &= (u_{s'}, \abst{f}_{s'}, (), (\abst{t})) \\
  \text{get}(u_{s'}, \abst{f}_{s'}, (), \chip{Add}{\abst{t},\abst{y}}) &= (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{
  \text{get}(u_s+1, \abst{f}_s \cup \set{\begin{array}{rl}\chip{Mul}{\abst{x},\abst{x}} & \wire{u_s}{q}\end{array}}, (), \chip{Add}{\wire{u_s}{q},\abst{y}}) = (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\left(u_s+2, \abst{f} \cup \set{\begin{array}{rl}
    \chip{Mul}{\abst{x},\abst{x}} & \wire{u_s}{q} \\
    \chip{Add}{\wire{u_s}{q},\abst{y}} & \wire{u_s+1}{q}
  \end{array}}, (\wire{u_s+1}{q}), (\wire{u_s+1}{q})\right) = (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))}
$ \\
$= \left(\abst{f}_s \cup \set{\begin{array}{rl}
    \chip{Mul}{\abst{x},\abst{x}} & \wire{u_s}{q} \\
    \chip{Add}{\wire{u_s}{q},\abst{y}} & \wire{u_s+1}{q}
  \end{array}}, (\wire{u_s+1}{q})\right)
$ \\
where $(u_s,\abst{f}_s,()) = \text{init}(\tin{})$
\\ 
$= \opcirc\limits_{i \in [3]}\aput(\Input^{t^{in}_{i}}_{i-1})(0,\emptyset,())
= \text{put}(\Input^q_1) \circ \text{put}(\Input^q_0)(0, \emptyset,())
= \text{put}(\Input^q_1)(1, \set{\begin{array}{rl}\chipu{\Input^q_0}{} & \wire{0}{q} \end{array}}, ())$
\\
$= \left(2, \set{\begin{array}{rl}
  \chipu{\Input^q_0}{} & \wire{0}{q} \\
  \chipu{\Input^q_1}{} & \wire{1}{q}
\end{array}}, ()\right)$
\\
$\therefore \ (\abst{f}, \avec{Y}) = \left(\set{\begin{array}{rl}
  \chipu{\Input^q_0}{} & \wire{0}{q} \\
  \chipu{\Input^q_1}{} & \wire{1}{q} \\
  \chip{Mul}{\wire{0}{q},\wire{0}{q}} & \wire{2}{q} \\
  \chip{Add}{\wire{2}{q},\wire{1}{q}} & \wire{3}{q}
\end{array}}, (\wire{3}{q})\right)
$
\end{longtable}

Thus $\abst{x} = \wire{0}{q}$, $\abst{y} = \wire{1}{q}$, $\abst{t} = \wire{2}{q}$ and $\abst{z} = \wire{3}{q}$. The resulting abstract circuit can be notated as follows:

\begin{tabularx}{\textwidth}{@{} r c c Y Y @{}}
\toprule
 & Variables & Predicate & One to Many or None Relation & Abstract Circuit Diagram
\\\hline \\
notation & $(\abst{f}, \avec{Y})$ &
$\build{x^2+y=z^*}{}{}$ & 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[draw, anchor=center] (in1) at (0,0) {$\Input^q_0$};
\node[draw, anchor=center] (in2) at ($(in1.south)-(0,0.4)$) {$\Input^q_1$};
\node[anchor=center] (mul) at ($(in2.south)-(0,0.4)$) {$\chip{Mul}{\abst{x},\abst{x}}$};
\node[anchor=center] (add) at ($(mul.south)-(0,0.4)$) {$\chip{Add}{\abst{t},\abst{y}}$};

\node[anchor=center] (x) at ($(in1.east)+(3.5,0)$) {$\abst{x}$};
\node[anchor=center] (y) at ($(x.south)-(0,0.4)$) {$\abst{y}$};
\node[anchor=center] (t) at ($(y.south)-(0,0.4)$) {$\abst{t}$};
\node[anchor=center] (z) at ($(t.south)-(0,0.4)$) {$\abst{z}$};
\node[anchor=west] (outs) at ($(z.east)+(-0.125,0.075)$) {$\in \avec{Y}$};

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
\gate{in0}{(0,0)}{}{$\Input^q_0$}{1}
\gate{in1}{($(in0.north east)+(0.1,0)$)}{}{$\Input^q_1$}{1}
\gate{mul}{($(in0.south west)+(0.1875,-0.5)$)}{$\abst{x}$,$\abst{x}$}{$\text{Mul}$}{1}
\draw[-,thick] (in0-out-1) -- ($(in0-out-1)+(0,-0.25)$);
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- ($(mul-in-2)+(0,0.25)$);
\draw[-,thick] ($(mul-in-1)+(0,0.25)$) -- (mul-in-1);
\draw[-,thick] ($(mul-in-2)+(0,0.25)$) -- (mul-in-2);
\gate{add}{($(mul.north east)+(0.5,0)$)}{$\abst{t}$,$\abst{y}$}{$\text{Add}$}{1}
\draw[-,thick] (mul-out-1) -- ($(mul-out-1)+(0,-0.25)$);
\draw[-,thick] ($(mul-out-1)+(0,-0.25)$) -- ($(mul.south east)+(0.25,-0.25)$);
\draw[-,thick] ($(mul.south east)+(0.25,-0.25)$) -- ($(mul.north east)+(0.25,0.25)$);
\draw[-,thick] ($(mul.north east)+(0.25,0.25)$) -- ($(add-in-1)+(0,0.25)$);
\draw[-,thick] ($(add-in-1)+(0,0.25)$) -- (add-in-1);
\draw[-,thick] (in1-out-1) -- ($(in1-out-1)+(0,-0.25)$);
\draw[-,thick] ($(in1-out-1)+(0,-0.25)$) -- ($(add-in-2)+(0,0.25)$);
\draw[-,thick] ($(add-in-2)+(0,0.25)$) -- (add-in-2);
\draw[-,thick] (add-out-1) -- ($(add-out-1)+(0,-0.4)$);
\node[draw, thick, circle, double, double distance=1pt, anchor=north] at ($(add-out-1)+(0,-0.4)$) {$\abst{z}$};
\end{tikzpicture}
\\ \hline
use & object & proofs & code & readability
\\\toprule
\end{tabularx}