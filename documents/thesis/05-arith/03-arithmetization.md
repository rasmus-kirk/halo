## Introduction to the Formal Specification

We define the following and then describe the \Plonk  protocol to highlight the role of the arithmetization pipeline.

\begin{notation}[Universe]
Informally, a set of all sets such that one can quantify over sets without defining them.
\end{notation}
\newcommand{\Uni}{\mathcal{U}}
$$
\Uni
$$

\begin{notation}[Type Annotation]
Informally, an assertion that an element belongs strictly to a set. 
\end{notation}
$$
x: A
$$

\motivnot  it concisely expresses that element $x$ strictly belongs to set $A$ where $A$ is unique; $\exists! A. x \in A$.

\begin{notation}[First order logic quantifiers]
Instead of quantifying with colons, we quantify with dots to clearly disambiguate the notation from type annotations.
\end{notation}

$$
\exists! A. x \in A \mapsto \exists! A: x \in A
$$

\begin{notation}[Function Type]
A set of all functions from set $A$ to set $B$.
\end{notation}
$$
A \to B
$$

- \subdefinition{associativity} $(A \to B) \to C = A \to (B \to C)$
- \subdefinition{partial application} Supplying some arguments returns a function that consumes the rest. e.g. $f: A \to B \to C \Leftrightarrow f(a): B \to C$
- \subnotation{placeholder} Dashes i.e. $-$, is a placeholder for an argument.
- \subdefinition{currying} A multi argument function can be partially applied. e.g. $f(a,-) = f(a)$ such that $f(a,b) = f(a)(b)$
- \subdefinition{composition} We can compose functions when the domain of the first operand aligns with the codomain of the second operand. Note the use of placeholder notation. i.e., $g \circ f = (- \circ -)(g,f)$, in the following:

$$
\begin{array}{rl}
- \circ - &: (B \to C) \to (A \to B) \to (A \to C) \\
(g \circ f) &: A \to C
\end{array}
$$

\begin{definition}[Color]
Informally, a color is the type tag for wires.
\end{definition}
\newcommand{\Color}{\text{Color}}
$$
\Color: \Uni = \set{p,q}
$$

- \projs $W: \Color \to \Uni$ - Wire Type
  - $W(p) = \Fb_p$
  - $W(q) = \Fb_q$

\motivdef it accounts for value types that wires can represent, i.e. $\Fb_p$ and $\Fb_q$. In the theory of colored properads (which in our context will be defined later), this is defined as a color[@yau2015-ssec1.1.1].

\begin{definition}[Profile]
A vector of colors.
\end{definition}
$$
\vec{t}: \Color^k
$$

\motivdef it is a succinct way to express the type tags for multiple wires. The terminology follows from [@yau2015-ssec1.1.2].

\begin{notation}[Mapping over a vector]
Applies a function to each element of a vector.
\end{notation}
$$
\begin{array}{rl}
-[-] &: (A \to B) \to A^k \to B^k \\
f[\vec{x}] &= (f(x_1), \ldots, f(x_k))
\end{array}
$$

\motivdef it gives a concise notation whilst distinct from a function that takes a vector as an argument. i.e. $f(\vec{x}) \neq f[\vec{x}]$.

\begin{notation}[Vector Product Isomorphism]
A vector is coercable into a product.
\end{notation}
$$
(t_1, \ldots, t_k): T^k = (t_1, \ldots, t_k): \underbrace{T \times \ldots \times T}_{k}
$$

\motivdef it is an intuitive way to reason about $W[\vec{t}]$ as multiple value types rather than a vector of sum type of the codomain of $W$.

\begin{definition}[Program]
A program is a function from multiple values to multiple values.
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

\motivdef we have seen the full plonk protocol before. Here, however, the role of arithmetization is clear.

\begin{definition}[Arithmetization Pipeline]
The arithmetization pipeline is a sequence of computations that transforms a program $f$ and its witness $\vec{w}$ or public input $\vec{x}$ into a circuit $(R,X,W)$ where $R$ is the public circuit structure, $X$ are public computed values and $W$ are witness computed values that the core plonk protocol operates over via the grand product argument and vanishing argument.

Intuitively, the arithmetization pipeline can be thought of as a compiler pipeline with the front end parsing the program $f$ into an abstract circuit $\abst{f}$. From this intermediate representation, depending on the private or public variant, it will compute the trace table (to be defined later) via the $\text{trace}$ algorithm. Finally, the last pass interpolates the trace table into polynomials and compute other relevant data to yield the circuit $(R,X,W)$ or $(R,X,\bot)$. This is then fed to the core $\Plonk$ protocol.
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

- \subdefinition{structural integrity} $R$ and $X$ are guaranteed to be the same for both pipelines given the same $f$ if $(\vec{x}, \vec{w}) \in R_f$.
- *features*:
  - Type safety across multiple field types (for cycle of curves)
  - Single source of truth (prevents arithmetizer implementation bugs)
  - User-extensible architecture (enables rapid prototyping of new gates)
  - Support for transcript dependent gates (enables $\plookup$ like gates)
  - Next row referencing capability (reduces constraint count; used by poseidon gate)
  - Declarative algebraic optimizations via gate equivalence (reduces constraint count)
  - Gate declaration order invariant (prevents circuit composition bugs)

We now proceed to define $\text{interpolate}$, $\text{trace}$, $\text{trace}_{\text{pub}}$ and $\text{build}$.
