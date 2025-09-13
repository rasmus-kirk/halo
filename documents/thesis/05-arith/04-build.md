## Build

The build function describes the construction of an abstract circuit from a program $f$. Intuitively it can be thought of as a directed acyclic graph where gadgets are vertices and wires are edges, modelling the structure of the circuit without its concrete values.

\begin{definition}[UUID]
The set of unique identifiers for wires.
\end{definition}
$$
UUID: \Uni = \Nb
$$

- *motivation*: the total ordering of the naturals is necessary to determine acyclicity.

\begin{definition}[Wire]
A wire is an abstract representation of a value in the circuit.
\end{definition}
$$
\abst{w}: \Wire = \wire{i}{t}
$$

- \projs
  - $\id(\abst{w}): UUID$ - the unique identifier of the wire.
  - $\ty(\abst{w}): \Color$ - the color of the wire.
- \subnotation{hat} the hat $\abst{w}$ indicates that it is an abstract value, not a concrete value. i.e. $\abst{w} = \wire{i}{t}$ denotes that $\id(\abst{w}) = i$ and $\ty(\abst{w}) = t$.

\motivdef rather than eagerly constructing a circuit, with wires we can construct an abstract circuit which allows reasoning about the circuit structure algebraically with colored properads defined below.

\begin{definition}[Colored Properad / Operations]
Hereafter simply "properad". Intuitively, defines a class of gates.
\end{definition}
\newcommand{\Prpd}{\text{PrPd}}
$$
\abst{g}: \Prpd
$$

- \projs
  - $n(\abst{g}): \Nb$ - the number of inputs.
  - $m(\abst{g}): \Nb$ - the number of outputs.
  - $\pin(\abst{g}): \Color^{n(\abst{g})}$ - the profile for the inputs.
  - $\pout(\abst{g}): \Color^{m(\abst{g})}$ - the profile for the outputs.
  - $\eval(\abst{g}): \Program(\pin(\abst{g}), \pout(\abst{g}))$ - the canonical program of the properad.
  - there are more projections defined later.
- \subnotation{hat} the hat $\abst{g}$ indicates that properads are abstract gates.

\motivdef it acts as single source of truth for contributions to the gate constraint polynomial and circuit for gates of its kind. Moreover, circuit construction with it is an expression of an algebraic theory[^properad] which allows us to reason about circuits algebraically, facilitating the following:

- Optimizations via rewriting / caching of existing wires
- Formal reasoning / writing proofs about the circuit structure of a program.

[^properad]: https://ncatlab.org/nlab/show/gebra+theory


\begin{definition}[Gate]
A gate is an instantiation of a properad with input wires.
\end{definition}
$$
g: \Ggt = \abst{g}(\avec{x})
$$

- \projs
  - $\ty(g): \Prpd$ - the properad of the gate.
  - $\gin(g): \Wire^{n \circ \ty(g)}$ - the input wires of the gate.
- **Notation**:
  - $\abst{g}(\avec{x})$ denotes that $\ty[\avec{x}] = \pin(\abst{g})$ and other assertions defined later.
  - $\gpair{g}{\abst{y}} = (g, \abst{y})$ denotes that $\abst{y}$ is one of the output wires of gate $g$.
  - $\abst{g}()$ denotes that the gate has no input wires.
  - $\gpair{g}{\bot} = (g,\bot)$ denotes that the gate has no output wires.
  - underline may be omitted if the context is clear.

\motivdef it allows the circuit to be defined as a relation with entries in the form of $\gpair{g}{\abst{y}}$ or $\gpair{g}{\bot}$.

\begin{notation}[Guarded value]
It yields the value only if a predicate on it holds.
\end{notation}
$$
\begin{array}{rl}
\maybe{-}{-}&: A \to (A \to \Bb) \to \Option(A)  \\
\maybe{a}{\phi(a)} &= \begin{cases}
a & \phi(a) \\
\bot & \otherwise
\end{cases}
\end{array}
$$

If $\phi$ is a tautology; always true, we can coerce $(\maybe{a}{\phi(a)}): A$ without the option type.

\motivnot besides guarding values, we can declare variables in $\phi$ as tautological quantified formula(s) that computes the guarded value; naively, a model of $\mathtt{let}\ \phi\ \mathtt{in}\ a$

\begin{notation}[Singleton vector]
A value can be coerced automatically to a vector of length 1 and vice versa.
\end{notation}
$$
a:A = (a) : A^1
$$

\motivnot it reduces the use of parentheses; decluttering formal notation.

\begin{definition}[Abstract Circuit]
An abstract circuit is a one-to-many-or-none relation between gadgets and output wires or none, that is guaranteed acyclic.
\end{definition}
$$
\begin{array}{rl}
\AbsCirc &= \set{\abst{f} \subset \Ggt \times \Option(\Wire) \middle\vert \text{OneToManyOrNone}(\abst{f}) \land \text{Acyclic}(\abst{f}) } \\
\text{OneToManyOrNone}(\abst{f}) &= \forall \gpair{g}{\abst{y}},\gpair{h}{\abst{y}} \in \abst{f}. \abst{y} \neq \bot \implies g = h \\
\text{Acyclic}(\abst{f}) &= \forall \gpair{g}{\abst{y}} \in \abst{f}. (\abst{y} \neq \bot \land |\text{in}(g)| > 0) \implies \max(\id[\gin(g)]) < \min \left(\id[\out(\abst{f},g)] \right)
\end{array}
$$

- \projs
  - $\out(\abst{f}, g): \Wire^{m \circ \ty(g)} = \maybe{\avec{y}}{\gpair{g}{\abst{y}_i}\in \abst{f} \land \pout \circ \ty(g) = \ty[\avec{y}] \land \id(\abst{y}_{i>1}) > \id(\abst{y}_{i-1})}$ - the output wires of gate $g$ in abstract circuit $\abst{f}$ sorted ascending by UUID.
  - $\text{wires}(\abst{f}, g) = \gin(g) \cat \out(\abst{f}, g)$ - all wires; inputs and outputs.
- \subnotation{abstract circuit diagram} if $\forall i. \gpair{\abst{g}(\abst{x}_1, \ldots, \abst{x}_{n(\abst{g})})}{\abst{y}_i} \in \abst{f}$ and $\gpair{\text{Add}(\abst{a}, \abst{b})}{\abst{c}} \in \abst{f}$, then we can visualize the gates as an abstract circuit diagram as follows:

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

\motivdef an abstract circuit is simpler than a directed acyclic graph; target vertex; gate, of a given wire is not immediately extractable from the relation. Yet it is minimally sufficient to compute the circuit; a simpler structure for proofs. In the theory of properads, an abstract circuit models a colored pasting scheme[@yau2015-ssec8.2].

It is worth clarifying the naming conventions. A gadget is a composition of gates (we will define notation for gate and gadget composition as the build predicate later). Thus an abstract circuit is a gadget as well. However when we say circuit, we may refer to the following:

- *abstract circuit* - The top level / global gadget; for program $f$, in the context of some discussion.
- *trace table* - The concatenation of all constraints as a table. (we will define this later).
- *arguments to core plonk* - The values $(R,X,W)$ or $(R,X,\bot)$.


\begin{definition}[Build State]
A triple of current UUID, abstract circuit and vector of global output wires.
\end{definition}
\newcommand{\BuildState}{\text{BuildState}}
$$
s: \BuildState = \astate{u}{\abst{f}}{\avec{Y}}
$$

- \projs
  - $u(s): UUID$ - the current UUID.
  - $\abst{f}(s): \AbsCirc$ - the current abstract circuit.
  - $\avec{Y}(s): \Wire^k$ - the current global output wires.
- **Notation**: $s = \astate{u}{\abst{f}}{\avec{Y}}$ denotes that $u(s) = u$, $\abst{f}(s) = \abst{f}$ and $\avec{Y}(s) = \avec{Y}$.
- \opers
  - $(\astate{u}{\abst{f}}{\avec{Y}} \cat \astate{u'}{\abst{f}'}{\avec{Y}'}) = \astate{u + u'}{\abst{f} \cup \abst{f}'}{\avec{Y} \cat \avec{Y}'}$ - extending the state
  - $(s \cat \abst{y}) = (s \cat \astate{0}{\emptyset}{\abst{y}})$ - appending output wires to the state

\motivdef it represents stateful data at a point in time whilst the user constructs the abstract circuits by declaring gates.

\begin{notation}[Hadamard Product / Zip vectors]
Element wise product of two vectors of the same length.
\end{notation}
$$
\begin{array}{rl}
- \odot - &: A^k \to B^k \to (A \times B)^k \\
\vec{x} \odot \vec{y} &= ((x_1,y_1), \ldots, (x_k,y_k))
\end{array}
$$

\motivnot it is a concise notation for zipping two vectors.

\begin{definition}[New Wires]
A function that yields new output wires for a gate from the current state's UUID.
\end{definition}
$$
\begin{array}{rl}
\new &: UUID \to \Ggt \to \Wire^{m \circ \ty(g)} \\
\new(u,g) &= \wire{-}{-}[(u..u+m \circ \ty(g)) \odot (\pout \circ \ty(g))]
\end{array}
$$

\motivdef it is a helper function for build.

\begin{definition}[New Entries]
Create new entries for an abstract circuit relation from the current state's UUID and a gate.
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

\motivdef it is a helper function for build.

\begin{definition}[Gate Quotient / Cache]
An equivalence relation over gates induces a quotient over gates. If the user declares a gate that is equivalent to an existing gate in the abstract circuit, the output wires of the existing gate is reused. This is where algebraic optimizations are defined.
\end{definition}
$$
\Ggt^{\abst{f}}_g : \Uni = \set{h \in \Ggt \middle\vert
  (h, \_) \in \abst{f} \land h \equiv g
}
$$

\motivdef it facilitates reuse of output wires of equivalent gates to reduce the number of constraints in the circuit whilst preserving the same semantics. In future work, we could explore equality saturation techniques; [@egglog], as a candidate for defining gate equivalence.

\begin{definition}[Put gate]
Add a gate to the current abstract circuit.
\end{definition}
$$
\begin{array}{rl}
\aput &: \Ggt \to \BuildState \to \BuildState \\
\aput(g,s) &= s \cat \astate{(m \circ \ty(g))}{\entries(u(s),g)}{()}
\end{array}
$$

\motivdef it is a helper function for build.

\begin{definition}[Get output wires]
A function that retrieves the output wires of a gate from the current state's abstract circuit. If the gate is not in the abstract circuit, it extends the abstract circuit with new entries for the gate and yields new output wires.
\end{definition}
$$
\begin{array}{rl}
\aget &: \BuildState \to (g: \Ggt) \to \BuildState \times \Wire^{m \circ \ty(g)} \\
\aget(s, g)
&= \begin{cases}
  (s, \out(\abst{f}(s),h)) & \exists h \in \Ggt^{\abst{f}(s)}_g \\
  (\aput(g,s), \new(u(s),g)) & \otherwise
\end{cases}
\end{array}
$$

\motivdef it is a helper function for build.

\begin{definition}[Build Predicate]
A predicate that models the declaration of a gate or gadget being extended from the current state's abstract circuit.
\end{definition}
$$
\begin{array}{rl}
\build{-}{-}{-}&: \Program(\_,\_) \to \BuildState \to \BuildState \to \Bb
\end{array}
$$

- **Notation**:
  - $\build{f}{}{}$ states and output values can be omitted if they are not relevant to the discussion.
  - $\build{f}{s_1}{s_{k+1}} = \bigwedge\limits_{i \in (1..k+1)} \build{f_i}{s_i}{s_{i+1}}$ abstract circuit composition is build predicate conjunction.
  - $\build{f = \vec{y}}{}{}$ denotes that $\vec{y}$ are the expected output values of the program $f$. When used in another predicate, they bound the same wire. e.g. $\build{f=y}{}{s} \land \build{g(y)}{s}{} = \build{g(f(\ldots))}{}{}$
- \subnotation{Declare circuit output wire(s)} $\build{f=y^*}{s}{s' \cat \abst{y}} = \build{f=y}{s}{s'}$ the final output wires can be declared by annotating values with $*$.
- \subdefinition{Canonical Program in the build predicate} $\build{\eval(\abst{g}, \vec{x}) =\vec{y}}{s}{s'} = \left(\aget(s,\abst{g}(\avec{x})) \stackrel{?}{=} (s', \avec{y})\right)$
  - A program is arithmetizable if it can be decomposed into canonical programs; base cases.
  - e.g. $\build{x + y = z}{}{} =  \left(\aget(s,\text{Add}(\abst{x},\abst{y})) \stackrel{?}{=} (s', \abst{z})\right)$ is the canonical program for the $\text{Add}$ properad.

\motivdef extending an abstract circuit when expressed as a predicate, can be used to express proofs about abstract circuit construction concisely.

\begin{definition}[Input Properad]
$\Input^t_i$ is a properad whose gadget output is the wire for witness value $w_{i}$
\end{definition}

\begin{center}
\begin{tabular}{ c c }
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[minimum width=2cm, minimum height=1.5cm] (tab) {
\begin{tabular}{|c|c|c|c|c|}
\hline
\multicolumn{5}{|c|}{$\Input^t_i$} \\
\hline
$n$ & $m$ & $\pin$ & $\pout$ & $\eval()$ \\
\hline
$0$ & $1$ & $()$ & $t$ & $w_{i}$ \\
\hline
\end{tabular}
};
\end{tikzpicture}
&
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\gate{inp}{(0,0)}{}{$\text{Input}^t_{i}$}{1}
\draw[->,thick] (inp-out-1) -- ($(inp-out-1)+(0,-0.4)$);
\node[anchor=north east] at (inp-out-1) {$\abst{w_{i}}$};
\end{tikzpicture}
\end{tabular}
\end{center}

Only in the private case is $\eval$ well-defined, in the public case, eval will fail to yield a value. But we can still have a wire to represent it as shown in the abstract circuit diagram. This is because the type information of the witness is public even when the value is not.

\motivdef when treating the global input as a gadget, it allows us to unify all values of the circuit as a consequence of properads, whose benefits have been discussed earlier.

\begin{definition}[Initial Build State]
\end{definition}
$$
\begin{array}{rl}
\text{init} &: \Color^k \to \BuildState \\
\text{init}(\vec{t}) &= \left(
  \opcirc\limits_{i \in [k+1]} \aput(\Input^{t_{i}}_{i})
\right) (\astate{0}{\emptyset}{()})
\end{array}
$$

\motivdef it is a helper function for build.

\begin{definition}[Build]
Build models the user constructing an abstract circuit from the program $f$.
\end{definition}
$$
\begin{array}{rl}
\text{build} &: \Program(\pin, \pout) \to \AbsCirc \times \Wire^k \\
\text{build}(f) &= \maybe{(\abst{f}(s), \avec{Y}(s))}{
  \build{f}{\text{init}(\pin)}{s}
}
\end{array}
$$

\motivdef when modelling the user process of constructing an abstract circuit as an algorithm, it allows us to reason about the process formally.

\begin{tcolorbox}[breakable, enhanced, colback=GbBg00, title=Example, colframe=GbFg3, coltitle=GbBg00, fonttitle=\bfseries]

Build of $\build{x^2 + y = z^*}{}{}$

\begin{longtable}{@{}l@{}}
Let $f: \Fb_q^2 \to \Fb_q^1$ where $f(x,y) = x^2 + y$ \\
Thus $W[\pwit] = \Fb_q^2$, $\pwit = (q,q)$, $W[\pout] = \Fb_q$ and $\pout = q$ \\
Let $(\abst{f}, \avec{Y}) = \text{build}(f)= \maybe{\left(\abst{f}(s''), \avec{Y}(s'')\right)}{\build{x^2 + y = z^*}{s}{s''}}
$ \\
where  
$\build{x^2 + y = z^*}{s}{s''}
$\\
$= \build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{s}{s''}
$ \textit{rewriting the program to an equivalent form} \\
$= \build{x \times x = t}{s}{s'} \land \build{t + y = z^*}{s'}{s''}
$ \textit{by build predicate conjunction as gadget / gate composition}\\
$= \left(\begin{array}{rll}
  \aget(s, \ggt{Mul}{x,x}) &= (s', \abst{t}) &\land \\
  \aget(s', \ggt{Add}{t,y}) &= (s'', \abst{z})
\end{array}\right)
$ \textit{by build predicate on canonical programs of properads} \\
$= \left(\begin{array}{rll}
  (s \cat \astate{1}{\set{\begin{array}{rl}\ggt{Mul}{x,x} & \wire{u(s)}{q}\end{array}}}{()}, \wire{u(s)}{q}) &= (s', \abst{t}) & \land \\
  \aget(s', \ggt{Add}{t,y}) &= (s'', \abst{z})
\end{array}\right)
$ \textit{by definition of $\aget$, $\times$} \\
$= \left(\aget(s \cat \astate{1}{\set{\begin{array}{rl}\ggt{Mul}{x,x} & \wire{u(s)}{q}\end{array}}}{()}, \ggtw{Add}{\wire{u(s)}{q},\abst{y}}) = (s'', \abst{z})\right)
$ \textit{by substituting $s'$ and $\abst{t}$} \\
$= \left(\left(s \cat \astate{2}{\set{\begin{array}{rl}
    \ggt{Mul}{x,x} & \wire{u(s)}{q} \\
    \ggtw{Add}{\wire{u(s)}{q},\abst{y}} & \wire{u(s)+1}{q}
  \end{array}}}{\abst{z}}, \wire{u(s)+1}{q}\right) = (s'', \abst{z})\right)
$ \textit{by definition of $\aget$, $+$} \\
$\therefore (\abst{f}(s''), \avec{Y}(s'')) = \left(\abst{f}(s) \cup \set{\begin{array}{rl}
    \ggt{Mul}{x,x} & \wire{u(s)}{q} \\
    \ggtw{Add}{\wire{u(s)}{q},\abst{y}} & \wire{u(s)+1}{q}
  \end{array}}, \wire{u(s)+1}{q}\right)
$ \textit{by build state extension; $\cat$} \\
where
$s=\astate{u(s)}{\abst{f}(s)}{()} = \text{init}(\pwit)
$ \\ 
$= \opcirc\limits_{i \in (1..3)}\aput(\Input^{{t_{in}}_{i}}_{i}) (\astate{0}{\emptyset}{()})
$ \textit{by definition of initial build state} \\
$= \text{put}(\Input^q_2) \circ \text{put}(\Input^q_1)(\astate{0}{\emptyset}{()})
$ \textit{by unfolding the function composition} \\
$= \text{put}(\Input^q_2, \astate{1}{\set{\begin{array}{rl} \Input^q_1 & \wire{0}{q} \end{array}}}{()})
$ \textit{by definition of $\text{put}$} \\
$= \astate{2}{\set{\begin{array}{rl}
  \Input^q_1 & \wire{0}{q} \\
  \Input^q_2 & \wire{1}{q}
\end{array}}}{()}$
\textit{by definition of $\text{put}$}
\\
$\therefore \ (\abst{f}, \avec{Y}) = \left(\set{\begin{array}{rl}
  \Input^q_1 & \wire{0}{q} \\
  \Input^q_2 & \wire{1}{q} \\
  \ggtw{Mul}{\wire{0}{q},\wire{0}{q}} & \wire{2}{q} \\
  \ggtw{Add}{\wire{2}{q},\wire{1}{q}} & \wire{3}{q}
\end{array}}, \wire{3}{q}\right)
$ \textit{by substituting $s$ in $s''$ and simplifying}
\end{longtable}

Thus $\abst{x} = \wire{0}{q}$, $\abst{y} = \wire{1}{q}$, $\abst{t} = \wire{2}{q}$ and $\abst{z} = \wire{3}{q}$. The resulting abstract circuit can be notated as follows:

\begin{tabularx}{\textwidth}{@{} c c Y Y @{}}
\toprule
Variables & Predicate & One to Many or None Relation & Abstract Circuit Diagram
\\\hline \\
$(\abst{f}, \avec{Y})$ &
$\build{x^2+y=z^*}{}{}$ & 
\begin{tikzpicture}[
  baseline={(current bounding box.center)}
]
\node[anchor=center] (in1) at (0,0) {$\Input^q_1$};
\node[anchor=center] (in2) at ($(in1.south)-(0,0.4)$) {$\Input^q_2$};
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
\gate{in0}{(0,0)}{}{$\Input^q_1$}{1}
\gate{in1}{($(in0.north east)+(0.1,0)$)}{}{$\Input^q_2$}{1}
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
\\\toprule
\end{tabularx}

notational use cases:

\begin{itemize}
  \item \textbf{variables}: to represent an abstract circuit succinctly as variables.
  \item \textbf{predicate}: to model the composition of gadgets in proofs like the one above.
  \item \textbf{one to many or none relation}: as a clear data structure for implementation.
  \item \textbf{abstract circuit diagram}: to visualize the structure of the abstract circuit.
\end{itemize}

\end{tcolorbox}