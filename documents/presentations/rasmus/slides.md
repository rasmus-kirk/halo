---
title: Investigating IVC with Accumulation Schemes
author:
  - Rasmus Kirk Jakobsen
theme: Berlin
institute: Computer Science Aarhus
fontsize: 9pt
lang: en-US
section-titles: true
toc: true
---

# Introduction

## Motivation

\begin{block}{IVC is designed to solve the following problem:}
  \vspace{0.6em}
  \begin{quote}
    \color{Gray}
    If a computation runs for hundreds of years and ultimately outputs 42,
    how can we check its correctness without re-executing the entire process?
  \end{quote}
  \vspace{0.4em}
\end{block}

\begin{block}{We define the transition function $F$ run on an initial state $s_0$:}
\vspace{0.5em}
\centering
\begin{tikzpicture}[node distance=2cm]

  % Nodes
  \node (s0) [node] {$s_0$};
  \node (s1) [node, right=of s0] {$s_1$};
  \node (dots) [right=2cm of s1] {$\dots$};
  \node (sn) [node, right=2cm of dots] {$s_n$};

  % Arrows with labels
  \draw[thick-arrow] (s0) -- node[above] {$F(s_0)$} (s1);
  \draw[thick-arrow] (s1) -- node[above] {$F(s_1)$} (dots);
  \draw[thick-arrow] (dots) -- node[above] {$F(s_{n-1})$} (sn);

\end{tikzpicture}
\vspace{0.5em}
\end{block}

- _How can we verify $s_n = F^n(s_0)$ without re-executing the computation?_

## IVC chain

\begin{block}{We can use a SNARK to prove each computation step:}
  \vspace{0.5em}
  \centering
  \begin{tikzpicture}[node distance=1.75cm]
    % Nodes
    \node (s0) [node] {$s_0$};
    \node (s1) [node, right=of s0] {$(s_1, \pi_1)$};
    \node (dots) [right=1.75cm of s1] {$\dots$};
    \node (sn) [node, right=2.25cm of dots] {$(s_n, \pi_n)$};
    % Arrows with labels
    \draw[thick-arrow] (s0) -- node[above] {$\Pc(s_0, \bot)$} (s1);
    \draw[thick-arrow] (s1) -- node[above] {$\Pc(s_1, \pi_1)$} (dots);
    \draw[thick-arrow] (dots) -- node[above] {$\Pc(s_{n-1}, \pi_{n-1})$} (sn);
  \end{tikzpicture}
  \vspace{0.5em}
\end{block}

### $\Pc(s_{i-1}, \pi_{i-1})$ represents:
  - $s_i = F(s_{i-1})$
  - $\pi_i = \SNARKProver(R, x = \{ s_0, s_i \}, w = \{ s_{i-1}, \pi_{i-1} \})$
  - $R := \text{I.K.} \; w = \{ \pi_{i-1}, s_{i-1} \} \; \text{ s.t. }$
    - $s_i \meq F(s_{i-1}) \; \land \; (s_{i-1} \meq s_0 \lor \Vc(R, x = \{ s_0, s_i \}, \pi_{i-1}))$

## Proof

- $R$ gives us a series of proofs of the claims:
$$
\begin{alignedat}{7}
  &\text{I.K.} \; w \; &&\text{ s.t. } \; &&s_n     &&= F(s_{n-1}) \; &&\land \; (s_{n-1} = s_0  &&\lor \Vc(R, x, \pi_{n-1}) = \top), \\
  &\text{I.K.} \; w \; &&\text{ s.t. } \; &&s_{n-1} &&= F(s_{n-2}) \; &&\land \; (s_{n-2} = s_0  &&\lor \Vc(R, x, \pi_{n-2}) = \top), \; \dots \\
  &\text{I.K.} \; w \; &&\text{ s.t. } \; &&s_1     &&= F(s_0)     \; &&\land \; (s_0 = s_0      &&\lor \Vc(R, x, \pi_0) = \top)
\end{alignedat}
$$
- Which, if all verify means that:
$$
\begin{alignedat}{4}
  &\SNARKVerifier(R, x, \pi_n) = \top \implies \\
  &s_n = F(s_{n-1}) \; \land \; \\
  &\SNARKVerifier(R, x, \pi_{n-1}) = \top \implies \\
  &s_{n-1} = F(s_{n-2}) \; \land \\
  &\SNARKVerifier(R, x, \pi_{n-2}) = \top \implies \dots \\
  &s_1 = F(s_0)
\end{alignedat}
$$

# Plonk

## 

## Conclusion

### The project:
  - Gained a deeper understanding of advanced cryptographic theory.
  - Learned to better carry theory into practice.
  - Implementing full IVC is _hard_.
  - Benchmarks looks good, excited to see degree bound increase.
  - Future work...
