
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

- quantifiers $\forall x. P(x)$ instead of $\forall x: P(x)$ to disambiguate a typing judgement from the quantifier separator i.e. $\forall a:A$
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


