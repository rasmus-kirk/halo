---
title: Master's Thesis - Investigating feasibility of Halo2 for IVC in Rust
author:
  - Rasmus Kirk Jakobsen - 201907084
  - Abdul Haliq Abdul Latiff - 202303466
geometry: margin=2cm
bibliography: bibliography.bib
---

\newcommand{\maybe}[2]{ \left[ #1 \middle\vert #2 \right]}
\newcommand{\wave}[1]{ \bar{#1} }
\newcommand{\set}[1]{ \left\{ #1 \right\}}
\newcommand{\build}[3]{\left\llbracket #1 \right\rrbracket^{#2}_{#3}}
\newcommand{\AbsCirc}{\text{Circ}}
\newcommand{\Gate}{\text{Gate}}
\newcommand{\AState}{\text{State}}
\newcommand{\Mono}[1]{\text{Mono}^{#1}}
\newcommand{\VMap}{\text{VMap}}

\tableofcontents
\newpage


# Abstract

# Security Proofs

# High Level Protocol

\begin{algorithm}[H]
\caption*{
  \textbf{Surkål:} a plonkish NARK protocol.
}
\textbf{Inputs} \\
  \Desc{$f: \Fb^n_q \to \Fb^m_q$}{The program being proved.} \\
  \Desc{$\vec{x} \in \Fb^n_q$}{The possibly private input to the program $f$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State Precompute the corresponding entry of the circuit relation; $x\ R\ w$:
    \Statex \algind \textbf{let} $(x,w) = \mathrm{circuit}(\mathrm{trace}(\mathrm{arithmetize}(f), \vec{x}))$ 
  \State The prover $P$ computes the proof:
    \Statex \algind $\pi \gets P(x,w)$
  \State The verifier $V$ then checks:
    \Statex \algind \textbf{return} $V(x, \pi)$
  \end{algorithmic}
\end{algorithm}

TODO - general IVC

# General Protocols




## Vanishing Argument

- Rasmus

## Batched Evaluation Proofs

- Rasmus

## Grand Product Argument

- Haliq

### Copy Constraints

- Haliq

### Lookup Arguments

- Haliq

\newpage

# General Arithmetization Scheme

We define the functions in the following pipeline:
$$
(x,w) = \mathrm{circuit} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})
$$

## Arithmetize

Arithmetize turns a program $f$ into an abstract circuit $\wave{f}$, which is a one-to-many-or-none relation between gates $g$ and output wire id(s) $\wave{y}$ or $\bot$ which denotes no output wires. e.g. $(\text{Add}(a,b), c) \in \wave{f}$ corresponds to $\build{a+b=c}{}{}$.

We notate inserting a gate or gadget $f$ to the circuit with $\build{f = \wave{\vec{y}}}{s}{s'}$, $\build{f = \wave{y}}{s}{s'}$ or $\build{f}{s}{s'}$ which transits the state from $s$ to $s'$. State has the form $(u, \wave{f})$ where $u$ is the current uuid for wires.

Wires annotated as the final output will be appended to $\wave{\vec{Y}}$, i.e. $\build{f=\wave{y}^*}{(\_,\wave{\vec{Y}})}{(\_, \wave{\vec{Y}} \cat \wave{y})}$, which may be omitted notationally.

Gates are primitive operations with $n_g \geq 0$ fan in inputs and $m_g \geq 0$ fan out outputs. A circuit is a composition of gadget(s) and/or gate(s).

These inserts yield new wires. However, wires are reused by an equivalence class on gates. If $g \equiv h$ where $(h,\_) \in \wave{f}$, then $\wave{\vec{y}}$ in $\build{g=\wave{\vec{y}}}{s}{s}$ corresponds to the output wire(s) of $h$, leaving the state unchanged. Equivalence of gates are computed using the equivalence saturation library 'egglog', by @egglog.

$$
\begin{aligned}
\AbsCirc &= \set{
  \wave{f} \subset \Gate \times \Nb_\bot \middle\vert
  \forall (g,\wave{y}),(h,\wave{y}) \in \wave{f}. \wave{y} \neq \bot \implies g = h
} \\
\Gate^{\wave{f}}_g &= \set{h \in \Gate \middle\vert
  (h, \_) \in \wave{f} \land h \equiv g
}
\\
\AState &= \Nb \times \AbsCirc
\end{aligned}
$$
$$
\begin{array}{rlrl}
\text{out} &: (\Nb_\bot + \AbsCirc) \to \Gate \to \Nb^m &
\text{get} &: \AState \to \Gate \to \AState \times \Nb^m
\\
\text{out}(\bot, \_) &= () &
\multirow{3}{*}{$\text{get}(u, \wave{f}, g)$} &
\multirow{3}{*}{$=\begin{cases}
    (u, \wave{f}, \text{out}(\wave{f}, h)) & h \in \Gate^{\wave{f}}_g \\
    (\text{put}(g, u, \wave{f}), \text{out}(u,g)) & \text{otherwise}
  \end{cases}
$}
\\
\text{out}(u,g) &= (u..u+m_g) \\
\text{out}(\wave{f}, g)
&= \text{out}(\min\left(
  \set{\wave{y} \middle\vert (g,\wave{y}) \in \wave{f}}
\right), g)
\\ \\
\text{entries}  &: \Nb \to \Gate \to \AbsCirc &
\build{g = \wave{\vec{y}}}{s}{s'}
&= \left(\text{get}(s,g) \overset{?}{=} (s', \wave{\vec{y}})\right) 
\\
\text{entries}(u,g) &= \begin{cases}
  \set{(g,\wave{y}) \middle\vert \wave{y} \in \text{out}(u,g)}
  & m_g > 0 \\
  \set{(g,\bot)}
  & m_g = 0
\end{cases} &
\build{f}{s_1}{s_{k+1}}
&= \bigwedge\limits_{i \in [k]} \build{f_i}{s_i}{s_{i+1}} 
\\ \\
\text{put} &: \Gate \to \AState \to \AState &
\text{arithmetize} &: (\Fb^n_q \to \Fb^m_q) \to \AbsCirc \times \Nb^{m'}
\\
\text{put}(g, u, \wave{f}) &= (
  u + m, \wave{f} \cup \text{entries}(u, g)
) &
\text{arithmetize}(f) &= \maybe{(\wave{f}, \wave{\vec{Y}})}{
  \build{f}{(\text{put}(\text{Input})^n(0,\emptyset), \emptyset)}{(\_, \wave{f}, \wave{\vec{Y}})}
}
\end{array}
$$

get might just be egglog add calls. then saturate and find best for output wires.
but u need to also search relations / zero output wires involved.

you need to figure out if you can use egglog as the relation / abstract circuit instead.

## Trace

- composition of monotonic functions; kleene fixedpoint theorem
- continuations
- resolve
- stack of wire ids
- vmap

$$
\begin{array}{rl}
\Mono{T} &= T \times \Fb^k_q \to T \times \Fb^{k'}_q
\\
\text{MonoC}^T &= \Mono{T} \to \Mono{T}
\\
\VMap &= \Nb \rightharpoonup \Fb_q
\\ \\
\text{lift} &: \Mono{T} \to \Mono{U \times T}
\\
\text{lift}(f) &= (u,s) \mapsto (u,f(s))
\\ \\
\text{peek} &: \Fb^{k}_q \to \Fb_q + \bot
\\
\text{pop} &: \Fb^k_q \to \Fb^{k'}_q
\\
\text{push} &: \Mono{\VMap} \\
\text{resolve} &: \text{MonoC}^{T \times \VMap}
\\ \\
\text{trace} &: T \to \Mono{T\times \VMap} \to \Nb^k \times \AbsCirc \to \Fb^k_q \to T \\
\text{trace}^t_g(a,\vec{x}) &= \left[
t'
\middle\vert
\text{sup}_{n\in\Nb} \text{resolve}(g)^n (t,\text{init}(a, \vec{x})) = (t',\_)
\right]
\end{array}
$$

- define init as kleene fixedpoint too
- define peek, pop, push
- define resolve

### Asserts

- if stack not empty, just apply continuation
- else if stack empty
  - get no output gates
  - if input exists in domain of vmap
  - then push inputs
  - if stack still empty, apply continuation

### Gate Constraints

- append matrix

### Copy Constraints

- tabulate sigma

### Lookup Argument Constraints

- $t$ poly eval thunk
- $f$: get eval corresponding to $(x,y,z)$ when resolve lookup else get 

### Full Surkål Trace

... construct $t$ and $e$ and define $\text{trace} = \text{trace}^t_e$

## Circuit

- fft
- commits? pcdl
- lookup thunk

# Surkål Circuits

# Gates and Gadgets

| $\Gate = (\vec{x} : \Nb^n, f: \Fb^n_q$ | $\to \Fb^m_q, \_)$         | remarks                 |
|:-------------------------:|:-----------------------------:|:------------------------|
| Input$_i()$              | $(x_i)$                       | from trace              |
| Const$_{s,p}()$           | $(s)$                         |                         |
| Add$(x,y)$                | $(x+y)$                       |                         |
| Mul$(x,y)$                | $(x \times y)$                |                         |
| Inv$(x)$                  | $(x^{-1})$                    |                         |
| Pow7$(x)$                 | $(x^7)$                       |                         |
| If$(b,x,y)$               | $(b ? x : y)$                 |                         |
| Lookup$_T(x,y)$          | $\maybe{(z)}{(x,y,z) \in T}$  |                         |
| PtAdd$(x_P,y_P,x_Q,y_Q)$  | $(x_R, y_R)$                  | Arkworks point add      |
| Poseidon$(a,b,c)$         | $(a',b',c')$                  | Mina poseidon 5 rounds  |
| Public$(x)$               | $()$                          |                         |
| Bit$(b)$                  | $()$                          |                         |
| IsAdd$(x,y,z)$            | $()$                          |                         |
| IsMul$(x,y,z)$            | $()$                          |                         |
| IsLookup$_T(x,y,z)$      | $()$                          |                         |

## XOR

## Poseidon

## Range Check

## Foreign Field stuff

# Signatures

# IVC Verifier from Gadgets

## Surkål Verifier

## Accumulation Verifier

## SuccinctCheck

## Signatures

# Appendix

## Notation

TODO

## Arithmetize Example

Example of the arithmetization of $x^2 + y$ with gates Input, Mul$(a,b)$ and Add$(a,b)$ all with $m=1$:
$$
\begin{aligned}
&\text{arithmetize}((x,y) \mapsto (x^2 + y))
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{
  \build{x^2 + y = z^*}
    {((u, \wave{f}) = \text{put}(\text{Input})^2(0, \emptyset), \emptyset)}
    {(\_, \wave{f}'', (z))}
  }
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\build{\begin{array}{l}
  x \times x = t \\
  t + y = z^*
\end{array}}{(u, \wave{f}, \emptyset)}{(\_, \wave{f}'', (z))}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{l}
  \build{x \times x = t}{(u, \wave{f})}{(u', \wave{f}')} \\
  \build{t + y = z^*}{(u', \wave{f}', \emptyset)}{(\_, \wave{f}'', (z))}
\end{array}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{rl}
  \text{get}(u, \wave{f}, \text{Mul}(x,x)) &= (u', \wave{f}', (t)) \\
  \text{get}(u', \wave{f}', \text{Add}(t,y)) &= (\_, \wave{f}'', (z))
\end{array}}
\\ 
&= \maybe{\left(\wave{f}'', (z)\right)}{\begin{array}{rl}
  (u+1, \wave{f} \cup \set{(\text{Mul}(x,x), u)}, (u)) &= (u', \wave{f}', (t)) \\
  \text{get}(u', \wave{f}', \text{Add}(t,y)) &= (\_, \wave{f}'', (z))
\end{array}}
\\
&= \maybe{\left(\wave{f}'', (z)\right)}{
  \text{get}(u+1, \wave{f} \cup \set{(\text{Mul}(x,x))}, \text{Add}(u,y)) = (\_, \wave{f}'', (z))
}
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(x,x) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
  (u, \wave{f}) = \text{put}(\text{Input})^2(0, \emptyset)
}
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,y) & u+1
  \end{array}}, (u+1)\right)}{
    (u, \wave{f}) = \text{put}(\text{Input}, 1, \set{(\text{Input}, 0)}, \emptyset)
  }
\\
&= \maybe{\left(\wave{f} \cup \set{\begin{array}{rl}
    \text{Mul}(0,0) & u \\
    \text{Add}(u,1) & u+1
  \end{array}}, (u+1) \right)}
  {(u, \wave{f}) = \left(2, \set{\begin{array}{rl}
    \text{Input} & 0 \\
    \text{Input} & 1
  \end{array}}\right)}
\\
&= \left(\set{\begin{array}{rl}
  \text{Input} & 0 \\
  \text{Input} & 1 \\
  \text{Mul}(0,0) & 2 \\
  \text{Add}(2,1) & 3
\end{array}}, (3)\right)
\end{aligned}
$$

## Defining Equivalence of Gates with Egglog

TODO

## Kleene Fixedpoint Theorem in Trace

Trace is defined as a composition of monotonic functions that has control over their continuations. Thus if the full composition is $f$, then the trace is $\mu x. f(x)$. Given an initial state, it is notated as the supremum. $\text{sup}_{n \in \Nb} f^n(s_0)$, where $n$ is the smallest $n$ such that $f^n(s_0) = f^{n+1}(s_0)$, i.e. the least fixedpoint of $f$. We can compute it recursively or as a stack-based loop.

\begin{algorithm}[H]
\caption*{
  \textbf{sup:} kleene least fixedpoint protocol.
}
\textbf{Inputs} \\
  \Desc{$f: T \to T$}{Monotonic function.} \\
  \Desc{$s_0 : T$}{Initial state.} \\
\textbf{Output} \\
  \Desc{$s_n : T$}{The state corresponding to the least fixedpoint of $f$.}
\begin{algorithmic}[1]
  \State Initialize variables:
    \Statex \algind $x := \bot$
    \Statex \algind $x' := s_0$ 
  \State Recursive compute:
    \Statex \textbf{do:}
    \Statex \algind $x := x'$
    \Statex \algind $x' := f(x)$
    \Statex \textbf{while} $x \neq x'$
  \State Return the least fixedpoint:
    \Statex \textbf{return} $x$
  \end{algorithmic}
\end{algorithm}

We can show that the function is monotonic by defining the order on the state, and showing that the function preserves the order. The order is defined as follows:

$$
(t,v,\vec{s}) \sqsubseteq (t',v',\vec{s'}) \iff
\begin{aligned}
  &t \not\sqsubseteq t' \Rightarrow \text{dom}(v) \not\subseteq \text{dom}(v') \Rightarrow |s| < |s'|
\end{aligned}
$$

We never remove the mapping in $v$ thus the order is preserved for $v$ despite the fact that the stack $s$ can grow and shrink. To show $t \sqsubseteq t'$ then is to investigate the remaining monotonic continuations for Surkål.

# Bibliography

