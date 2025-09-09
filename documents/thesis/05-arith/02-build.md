## Build

The build function describes the construction of an "abstract circuit" from a program $f$. Naively it can be thought of as a directed acyclic graph where gadgets are vertices and wires are edges, modelling the structure of the circuit without its concrete values.

\begin{definition}[UUID]
The set of unique identifiers for wires.
\end{definition}
$$
UUID = \Nb
$$

\begin{definition}[Wire]
A wire is an abstract representation of a value in the circuit.
\end{definition}
$$
\abst{w}: \Wire = \wire{i}{t}
$$

- *projections*:
  - $\id(\abst{w}): UUID$ - the unique identifier of the wire.
  - $\ty(\abst{w}): \WireType$ - the color of the wire.
- *notation*:
  - the hat $\abst{w}$ indicates that it is an abstract value, not a concrete value.
  - $\abst{w} = \wire{i}{t}$ denotes that $\id(\abst{w}) = i$ and $\ty(\abst{w}) = t$.
- *motivation*: rather than eagerly constructing a concrete circuit, with wires we can construct an abstract circuit which allows reasoning about the circuit structure algebraically with properads.

\begin{definition}[Colored Properad]
A colored properad (hereafter simply "properad") defines a class of gadgets.
\end{definition}
\newcommand{\Prpd}{\text{PrPd}}
$$
\abst{g}: \Prpd
$$

- *projections*:
  - $n(\abst{g}): \Nb$ - the number of inputs.
  - $m(\abst{g}): \Nb$ - the number of outputs.
  - $\pin(\abst{g}): \Color^{n(\abst{g})}$ - the profile for the inputs.
  - $\pout(\abst{g}): \Color^{m(\abst{g})}$ - the profile for the outputs.
  - there are more projections defined later.
- *notation*: the hat $\abst{g}$ indicates that it is an abstract gadget.
- *motivation*: Properads serve as a single source of truth for contributions to the gate constraint polynomial and class of sub-tables / gates to the concrete circuit. Moreover, using properads makes circuit construction an expression of an algebraic theory which allows us to reason about circuits algebraically[^properad], facilitating opportunities for optimizations via rewriting / caching of existing wires.

[^properad]: https://ncatlab.org/nlab/show/gebra+theory


\begin{definition}[Gadget]
A gadget is an instantiation of a properad with input wires.
\end{definition}
$$
g: \Ggt = \abst{g}(\avec{x})
$$

- *projections*:
  - $\ty(g): \Prpd$ - the properad of the gadget.
  - $\gin(g): \Wire^{n \circ \ty(g)}$ - the input wires of the gadget.
- *notation*:
  - $\abst{g}(\avec{x})$ denotes that $\ty[\avec{x}] = \pin(\abst{g})$ and other assertions defined later.
  - $\gpair{g}{\abst{y}} = (g, \abst{y})$ denotes that $\abst{y}$ is one of the output wires of gadget $g$.
  - $\abst{g}()$ denotes that the gadget has no input wires.
  - $\gpair{g}{\bot} = (g,\bot)$ denotes that the gadget has no output wires.

\begin{notation}[Guarded value]
It yields the value $a$ if $\phi(a)$ holds otherwise $\bot$.
\end{notation}
$$
\left(\maybe{a}{\phi(a)}\right): \Option(A) 
$$

\begin{notation}[Singleton vector]
A value can be coerced to a vector of length 1 and vice versa.
\end{notation}
$$
a:A = (a) : A^1
$$

\begin{definition}[Abstract Circuit]
An abstract circuit is a one-to-many-or-none relation between gadgets and output wires or none, that is guaranteed acyclic.
\end{definition}
$$
\begin{array}{rl}
\AbsCirc &= \set{\abst{f} \subset \Ggt \times \Option(\Wire) \middle\vert \text{OneToManyOrNone}(\abst{f}) \land \text{Acyclic}(\abst{f}) } \\
\text{OneToManyOrNone}(\abst{f}) &= \forall \gpair{g}{\abst{y}},\gpair{h}{\abst{y}} \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
\text{Acyclic}(\abst{f}) &= \forall \gpair{g}{\abst{y}} \in \abst{f}. \abst{y} \neq \bot \land |\text{in}(g)| > 0 \implies \max(\id[\gin(g)]) < \min \left(\id[\out(\abst{f},g)] \right)
\end{array}
$$

- *projections*:
  - the output wires of gadget $g$ in abstract circuit $\abst{f}$ where their ids are sorted ascending.
    - $\out(\abst{f}, g): \Wire^{m \circ \ty(g)} = \maybe{\avec{y}}{\gpair{g}{\abst{y}_i}\in \abst{f} \land \pout \circ \ty(g) = \ty[\avec{y}] \land \id(\abst{y}_{i>1}) > \id(\abst{y}_{i-1})}$
- *motivation*: an abstract circuit as a relation contains less information than a directed acyclic graph (target vertex not known) yet is minimally sufficient to compute the concrete circuit; a simpler structure for proofs.
- *notation*: if $\forall i. \gpair{\abst{g}(\abst{x}_1, \ldots, \abst{x}_{n(\abst{g})})}{\abst{y}_i} \in \abst{f}$ and $\gpair{\text{Add}(\abst{a}, \abst{b})}{\abst{c}} \in \abst{f}$, then we can visualize the gadgets as an abstract circuit diagram as follows:

\begin{center}
\begin{tabular}{ c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{id}{(0,0)}{$\abst{x}_1$,$\cdots$,$\abst{x}_{n(\abst{g})}$}{$\abst{g}$}{3}
\draw[-,thick] ($(id-in-1)+(0,0.25)$) -- (id-in-1);
\draw[-,thick] ($(id-in-3)+(0,0.25)$) -- (id-in-3);
\draw[->,thick] (id-out-1) -- ($(id-out-1)+(0,-0.4)$);
\node[anchor=north east] at (id-out-1) {$\abst{y}_1$};
\node[anchor=north] at ($(id-out-2)+(0,-0.1)$) {$\cdots$};
\draw[->,thick] (id-out-3) -- ($(id-out-3)+(0,-0.4)$);
\node[anchor=north west] at (id-out-3) {$\abst{y}_{m(\abst{g})}$};
\end{tikzpicture}
&
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

\begin{definition}[Build State]
A triple of current UUID, abstract circuit and vector of global output wires.
\end{definition}
$$
s: \AState = \astate{u}{\abst{f}}{\avec{Y}}
$$

- *projections*:
  - $u(s): UUID$ - the current UUID.
  - $\abst{f}(s): \AbsCirc$ - the current abstract circuit.
  - $\avec{Y}(s): \Wire^k$ - the current global output wires.
- *notation*: $s = \astate{u}{\abst{f}}{\avec{Y}}$ denotes that $u(s) = u$, $\abst{f}(s) = \abst{f}$ and $\avec{Y}(s) = \avec{Y}$.
- *operations*:
  - $(\astate{u}{\abst{f}}{\avec{Y}} \cat \astate{u'}{\abst{f}'}{\avec{Y}'}) = \astate{u + u'}{\abst{f} \cup \abst{f}'}{\avec{Y} \cat \avec{Y}'}$ - extending the state
  - $(s \cat \abst{y}) = (s \cat \astate{0}{\emptyset}{\abst{y}})$ - appending output wires to the state
- *motivation*: keep track of stateful data whilst the user constructs the abstract circuits / declares gadgets.

\begin{definition}[Build Predicate]
A predicate that models the declaration of a gadget or sub-circuit being extended from the current state's abstract circuit.
\end{definition}
$$
\begin{array}{rl}
\build{-}{-}{-}&: \Program \to \AState \to \AState \to \Bb \\
\\
\build{g = \vec{y}}{s}{s'} &= \left(\text{get}(s,g) = (s', \avec{y})\right) \\
\build{f=y^*}{s}{s' \cat \abst{y}} &= \build{f=y}{s}{s'} \\
\build{f}{s_1}{s_{k+1}} &= \bigwedge\limits_{i \in (1..k+1)} \build{f_i}{s_i}{s_{i+1}}
\end{array}
$$

- *notation*:
  - states can be omitted if they are not relevant to the discussion, e.g. $\build{f}{}{}$.
  - $\build{\ldots = \vec{y}}{}{}$ denotes the expected output values of the program, when used in another predicate, they bound the same wire. e.g. $\build{f=y}{}{s} \land \build{g(y)}{s}{} = \build{g(f(\ldots))}{}{}$.
  - the final output wires can be declared by annotating values that they correspond to with $*$, e.g. $\build{f=y^*}{}{}$.
  - output values can be omitted if they are not used in other predicates / the rest of the proof, e.g. $\build{f}{}{}$.
  - abstract circuit composition can be expressed succinctly as build predicate conjunctions, e.g. $\build{f_1}{s_1}{s_2} \land \build{f_2}{s_2}{s_3} \land \build{f_3}{s_3}{s_4} = \build{f_1\ f_2\ f_3}{s_1}{s_4}$.
- *motivation*: extending an abstract circuit when expressed as a predicate, can be used to express proofs about abstract circuit construction concisely.

\begin{notation}[Hadamard Product / Zip vectors]
Element wise product of two vectors of the same length.
\end{notation}
$$
\begin{array}{rl}
- \odot - &: A^k \to B^k \to (A \times B)^k \\
\vec{x} \odot \vec{y} &= ((x_1,y_1), \ldots, (x_k,y_k))
\end{array}
$$

\begin{definition}[New Wires]
A function that yields new output wires for a gadget from the current state's UUID.
\end{definition}
$$
\begin{array}{rl}
\new &: UUID \to \Ggt \to \Wire^{m \circ \ty(g)} \\
\new(u,g) &= \wire{-}{-}[(u..u+m \circ \ty(g)) \odot (\pout \circ \ty(g))]
\end{array}
$$

\begin{definition}[New Entries]
Create new entries / pairs for an abstract circuit from the current state's UUID and a gadget.
\end{definition}
$$
\begin{array}{rl}
\entries &: UUID \to \Ggt \to \AbsCirc \\
\entries(u,g) &= \begin{cases}
\set{\gpair{g}{\abst{y}} \middle\vert \abst{y} \in \new(u,g)}
& m \circ \ty(g) > 0 \\
\set{\gpair{g}{\bot}} & \otherwise
\end{cases}
\end{array}
$$

TODO

- gadget quotient
- get; canonical program explainer, move build gadget here
- init
- build

### Circuit Building

Operations have a *canonical program* that it corresponds to, e.g $\build{x + y=z}{s}{s'} = \left(\text{get}(s,\ggt{Add}{x,y}) = (s', \abst{z})\right)$, thus a program can be arithmetized iff it can be decomposed into these canonical programs. These get calls yield new wires. However, if $g \equiv h$ where $\gpair{h}{\_} \in \abst{f}_s$, then $\avec{y}= \out(\abst{f},h)$ in $\build{g=\vec{y}}{s}{s}$ leaving the state unchanged.[^egglog-eq] 

[^egglog-eq]: Determining equivalence between gadgets is a sophisticated problem, a candidate is to use equality saturation such as @egglog, however we implement simpler ad hoc solutions that doesnt cover the full equivalence structure. We leave this definition open.


$$
\Ggt^{\abst{f}}_g = \set{h \in \Ggt \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
$$
$$
\begin{array}{cc}
\begin{array}{rl}
\new &: \Nb \to \Ggt \to \Wire^{m_g} \\
\new(u,g) &= \text{wire}[(u..u+m_g) \odot \tout{g}] \\
\\
\entries  &: \Nb \to \Ggt \to \AbsCirc \\
\entries(u,g) &= \begin{cases}
\set{\gpair{g}{\abst{y}} \middle\vert \abst{y} \in \new(u,g)}
& m_g > 0 \\
\set{\gpair{g}{\bot}} & \otherwise
\end{cases} \\
\\
\text{init} &: \WireType^k \to \AState \\
\text{init}(\vec{t}) &= \left(
  \opcirc\limits_{i \in [k+1]} \aput(\Input^{t_{i}}_{i-1})
\right) \astate{0}{\emptyset}{()}
\end{array} &
\begin{array}{rl}
\aput &: \Ggt \to \AState \to \AState \\
\aput(g, s) &= s \sqcup \astate{m_g}{\entries(u_s,g)}{\emptyset} \\
\\
\aget &: \AState \to (g: \Ggt) \to \AState \times \Wire^{m_g} \\
\aget(s, g)
&= \begin{cases}
  (s, \out(\abst{f}_s,h)) & h \in \Ggt^{\abst{f}_s}_g \\
  (\aput(g, s), \new(u_s,g)) & \otherwise
\end{cases} \\
\\
\text{build} &: (W[\tin{}] \to W[\tout{}]) \to \AbsCirc \times \Wire^k \\
\text{build}(f) &= \maybe{(\abst{f}_s, \avec{Y}_s)}{
  \build{f}{\text{init}(\tin{})}{s}
}
\end{array}
\end{array}
$$


**Build Correctness Example**

Let $W(q)=\Fb_q$ and $f: \Fb_q^2 \to \Fb_q^1$ where $f(x,y) = x^2 + y$, then:

\begin{longtable}{@{}l@{}}
Let $(\abst{f}, \avec{Y}) = \text{build}(f)$
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
  \text{get}(u_s, \abst{f}_s, (), \ggt{Mul}{x,x}) &= (u_{s'}, \abst{f}_{s'}, (), (\abst{t})) \\
  \text{get}(u_{s'}, \abst{f}_{s'}, (), \ggt{Add}{t,y}) &= (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\begin{array}{rl}
  (u_s+1, \abst{f}_s \cup \set{\begin{array}{rl}\ggt{Mul}{x,x} & \wire{u_s}{q}\end{array}}, (), (\wire{u_s}{q})) &= (u_{s'}, \abst{f}_{s'}, (), (\abst{t})) \\
  \text{get}(u_{s'}, \abst{f}_{s'}, (), \ggt{Add}{t,y}) &= (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
\end{array}}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{
  \text{get}(u_s+1, \abst{f}_s \cup \set{\begin{array}{rl}\ggt{Mul}{x,x} & \wire{u_s}{q}\end{array}}, (), \ggtw{Add}{\wire{u_s}{q},\abst{y}}) = (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))
}
$ \\
$= \maybe{\left(\abst{f}_{s''}, (\abst{z})\right)}{\left(u_s+2, \abst{f}_s \cup \set{\begin{array}{rl}
    \ggt{Mul}{x,x} & \wire{u_s}{q} \\
    \ggtw{Add}{\wire{u_s}{q},\abst{y}} & \wire{u_s+1}{q}
  \end{array}}, (\wire{u_s+1}{q}), (\wire{u_s+1}{q})\right) = (u_{s''}, \abst{f}_{s''}, (\abst{z}), (\abst{z}))}
$ \\
$= \left(\abst{f}_s \cup \set{\begin{array}{rl}
    \ggt{Mul}{x,x} & \wire{u_s}{q} \\
    \ggtw{Add}{\wire{u_s}{q},\abst{y}} & \wire{u_s+1}{q}
  \end{array}}, (\wire{u_s+1}{q})\right)
$ \\
where $\astate{u_s}{\abst{f}_s}{()} = \text{init}(\tin{})$
\\ 
$= \opcirc\limits_{i \in (1..3)}\aput(\Input^{t^{in}_{i}}_{i-1}) \astate{0}{\emptyset}{()}
= \text{put}(\Input^q_1) \circ \text{put}(\Input^q_0)\astate{0}{\emptyset}{()}
= \text{put}(\Input^q_1)\astate{1}{\set{\begin{array}{rl} \Input^q_0 & \wire{0}{q} \end{array}}}{()}$
\\
$= \astate{2}{\set{\begin{array}{rl}
  \Input^q_0 & \wire{0}{q} \\
  \Input^q_1 & \wire{1}{q}
\end{array}}}{()}$
\\
$\therefore \ (\abst{f}, \avec{Y}) = \left(\set{\begin{array}{rl}
  \Input^q_0 & \wire{0}{q} \\
  \Input^q_1 & \wire{1}{q} \\
  \ggtw{Mul}{\wire{0}{q},\wire{0}{q}} & \wire{2}{q} \\
  \ggtw{Add}{\wire{2}{q},\wire{1}{q}} & \wire{3}{q}
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
\node[anchor=center] (in1) at (0,0) {$\Input^q_0$};
\node[anchor=center] (in2) at ($(in1.south)-(0,0.4)$) {$\Input^q_1$};
\node[anchor=center] (mul) at ($(in2.south)-(0,0.4)$) {$\ggt{Mul}{x,x}$};
\node[anchor=center] (add) at ($(mul.south)-(0,0.4)$) {$\ggt{Add}{t,y}$};

\node[anchor=center] (x) at ($(in1.east)+(2,0)$) {$\abst{x}$};
\node[anchor=center] (y) at ($(x |- in2)$) {$\abst{y}$};
\node[anchor=center] (t) at ($(x |- mul)$) {$\abst{t}$};
\node[anchor=center] (z) at ($(x |- add)$) {$\abst{z}$};
\node[anchor=west] (outs) at ($(z.east)+(-0.125,0.075)$) {$\in \avec{Y}$};

\node[] (g) at ($(in1.north)+(0,0.4)$) {$g$};
\node[] (w) at ($(x |- g)$) {$\abst{w}$};
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
use & object & proofs & implementation & readability
\\\toprule
\end{tabularx}

