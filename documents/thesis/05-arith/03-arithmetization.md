## Formal Specification

We define the following and then describe the plonk protocol to highlight the role of the arithmetization pipeline.

\begin{notation}[Universe]
Naively, a set of all sets.
\end{notation}
\newcommand{\Uni}{\mathcal{U}}
$$
\Uni
$$

- *motivation*: to quantify over sets without defining them.

\begin{notation}[Type Annotation]
Naively, an assertion that an element belongs strictly to a set; $\exists! A. x \in A$.
\end{notation}
$$
x: A
$$

\begin{notation}[Function Type]
A set of all functions from set $A$ to set $B$.
\end{notation}
$$
A \to B
$$

- *associative*: $(A \to B) \to C = A \to (B \to C)$
- *partial application* Supplying some arguments returns a function that consumes the rest 
  - e.g. if $f: A \to B \to C$ then $f(a): B \to C$
- *currying*: A multi argument function can be partially applied. 
  - e.g. if $A \times B \to C = A \to B \to C$ then $f(a,b) = f(a)(b)$
- *composition*: If the domain of the first operand aligns with the codomain of the second operand, the composition is a pipeline between the two
  - e.g. if $- \circ - : (B \to C) \to (A \to B) \to (A \to C)$ then $g \circ f(a) = g(f(a))$

\begin{definition}[Color]
Naively, a color is the type tag for wires.
\end{definition}
\newcommand{\Color}{\text{Color}}
$$
\Color: \Uni = \set{p,q}
$$

- *projections*: $W: \Color \to \Uni$ - wire value type
  - $W(p) = \Fb_p$
  - $W(q) = \Fb_q$
- *motivation*: we need to account for value types that wires can represent, i.e. $\Fb_p$ and $\Fb_q$. In the theory of properads (which in our context will be defined later), this is defined as a color[@yau2015-ssec1.1.1].

\begin{definition}[Profile]
A vector of colors.
\end{definition}
$$
\vec{t}: \Color^k
$$

- *motivation*: a clean way to represent the types for a vector of multi color wires[@yau2015-ssec1.1.2].

\begin{notation}[Mapping over a vector]
Applies a function to each element of a vector.
\end{notation}
$$
\begin{array}{rl}
-[-] &: (A \to B) \to A^k \to B^k \\
f[\vec{x}] &= (f(x_1), \ldots, f(x_k))
\end{array}
$$

- *motivation*: concise notation whilst notationally distinct from a function that takes a vector as an argument.

\begin{definition}[Program]
A program is a function from a vector of values to another vector of some profiles.
\end{definition}
\newcommand{\pin}{\vec{t}_{in}}
\newcommand{\pout}{\vec{t}_{out}}
\newcommand{\Program}{\text{Program}}
$$
\Program(\pin, \pout) = W[\pin] \to W[\pout]
$$

\begin{definition}[Witness]
A vector of values corresponding to the input of a program
\end{definition}
\newcommand{\pwit}{\vec{t}_{wit}}
$$
\vec{w}: W[\pwit]
$$

\begin{definition}[Public Input]
A vector of values that is used by the plonk verifier as public inputs to the circuit.
\end{definition}
\newcommand{\ppub}{\vec{t}_{pub}}
$$
\vec{x}: W[\ppub]
$$

\begin{definition}[Language]
The language for a program $f$ is the set of public inputs $\vec{x}$ for which there exists a witness $\vec{w}$ such that the circuit structure $R$ as a relation $R_f$ holds.
\end{definition}
$$
(\vec{x}, \vec{w}) \in R_f
$$

\begin{definition}[PLONK protocol]
\end{definition}
\newcommand{\Arithmetizepub}{\Arithmetize_{\text{pub}}}
$$
\begin{array}{rll}
\mathrm{communication} & \mathrm{computation}\\
P \rightsquigarrow V& \PlonkProver \circ \Arithmetize(f, \vec{w}) &= \pi \\
V& \PlonkVerifier(\pi) \circ \Arithmetize_{\text{pub}}(f, \vec{x}) &\stackrel{?}{=} \top
\end{array}
$$

- *motivation*: We have seen the full plonk protocol before. Here, however, the role of arithmetization is clear.

\begin{definition}[Arithmetization Pipeline]
The arithmetization pipeline is a sequence of computations that transforms a program $f$ and its witness $\vec{w}$ or public input $\vec{x}$ into a circuit $(R,X,W)$ where $R$ is the public circuit structure, $X$ are public computed values and $W$ are witness computed values that the core plonk protocol operates over via the grand product argument and vanishing argument.
\end{definition}

$$
\begin{array}{rl}
\begin{array}{rl}
(R,X,W) 
&= \Arithmetize(f,\vec{w}) \\ 
&= \mathrm{interpolate} \circ \mathrm{trace}(\vec{w}) \circ \mathrm{build}(f)
\end{array} &
\begin{array}{rl}
(R,X,\bot)
&= \Arithmetizepub(f,\vec{x}) \\
&= \mathrm{interpolate} \circ \mathrm{trace}_{\text{pub}}(\vec{x}) \circ \mathrm{build}(f)
\end{array}
\end{array}
$$

- *structural integrity*: $R$ and $X$ are guaranteed to be the same for both pipelines given the same $f$ if $(\vec{x}, \vec{w}) \in R_f$.
- *motivation & features*:
  - Type safety across multiple field types (for cycle of curves)
  - Single source of truth (prevents arithmetizer implementation bugs)
  - User-extensible architecture (enables rapid prototyping of new gadgets)
  - Support for transcript dependent gadgets (enables lookup arguments)
  - Next row referencing capability (reduces gate count; used by poseidon gadget)
  - Declarative algebraic optimizations via gadget equivalence (reduces gate count)
  - Gadget declaration order invariant (prevents circuit composition bugs)

We now proceed to define $\text{interpolate}$, $\text{trace}$, $\text{trace}_{\text{pub}}$ and $\text{build}$.
