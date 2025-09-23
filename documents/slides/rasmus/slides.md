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
header-includes: \usepackage{amsmath}
---

# Introduction

## Motivation

\begin{block}{IVC is designed to solve the following problem:}
  \vspace{0.6em}
  \begin{quote}
    \color{GbGreyNt}
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

## Halo Overview

- **$\PCDL$**: A Polynomial Commitment Scheme.
- **$\ASDL$**: An Accumulation Scheme for Evaluation Proof instances.
- **Plonk**: A general-purpose, potentially zero-knowledge, SNARK.
- **Pasta**: A cycle of elliptic curves, Pallas and Vesta.

## The Gate Constraints

\begin{columns} 
% Column 1
    \begin{column}{.5\textwidth}
      \centering
      \scalebox{0.9}{
        \begin{tikzpicture}
        % First Layer
        \node (input1) at (3, 7) {$x_1$};
        \node (input2) at (5, 7) {$x_2$};
        \node (A) at (1, 7) {$3$};
        \node (B) at (7, 7) {$5$};
        % Second Layer
        \node[draw, rectangle] (mul21) at (3, 6) {$\times$};
        \node[above left=0.01cm of mul21] {$a_1$};
        \node[above right=0.01cm of mul21] {$b_1$};
        \node[below right=0.01cm of mul21] {$c_1$};

        \node[draw, rectangle] (mul22) at (6, 6) {$\times$};
        \node[above left=0.01cm of mul22] {$a_2$};
        \node[above right=0.01cm of mul22] {$b_2$};
        \node[below right=0.01cm of mul22] {$c_2$};

        \draw[->] (input1) -- (2, 7) |- (mul21);
        \draw[->] (input1) -- (4, 7) |- (mul21);

        \draw[->] (input2) -- (5, 6.5) |- (mul22);
        \draw[->] (B) -- (7, 6.5) |- (mul22);

        % Third Layer
        \node[draw, rectangle] (mul31) at (2, 5) {$\times$};
        \node[above left=0.01cm of mul31] {$a_3$};
        \node[above right=0.01cm of mul31] {$b_3$};
        \node[below right=0.01cm of mul31] {$c_3$};

        \draw[->] (mul21) -- (3, 5) |- (mul31);
        \draw[->] (A) -- (1, 5) |- (mul31);

        % Fourth Layer
        \node[draw, rectangle] (add41) at (4, 4) {$+$};
        \node[above left=0.01cm of add41] {$a_4$};
        \node[above right=0.01cm of add41] {$b_4$};
        \node[below right=0.01cm of add41] {$c_4$};

        \draw[->] (mul31) -- (2, 4) |- (add41);
        \draw[->] (mul22) -- (6, 4) |- (add41);

        % Fifth Layer
        \node (output) at (4, 3) { 47 };

        \draw[->] (add41) -- (output);

        \end{tikzpicture}
      }
    \end{column}
% Column 2    
    \begin{column}{.5\textwidth}
        \small
        $$
          \begin{aligned}
            a(X) &= \text{ifft}(\vec{a})\\
            b(X) &= \text{ifft}(\vec{b})\\
            c(X) &= \text{ifft}(\vec{c})
          \end{aligned}
        $$
        $$
          \begin{alignedat}{4}
            a(\o^1) &= a_1, \; &&a(\o^2) &&= a_2, \; &&\dots \\
            b(\o^1) &= b_1, \; &&b(\o^2) &&= b_2, \; &&\dots \\
            c(\o^1) &= c_1, \; &&c(\o^2) &&= c_2, \; &&\dots
          \end{alignedat}
        $$
    \end{column}%
\end{columns}

\small
$$
\begin{aligned}
  f_{GC}(X) &= a(X) q_l(X) + b(X) q_r(X) + c(X) q_o(X) + a(X) b(X) q_m(X) + q_c(X), \\
  f_{GC}(X) &\meq 0
\end{aligned}
$$

## The Gate Constraints

\begin{columns} 
% Column 1
    \begin{column}{.5\textwidth}
      \centering
      \scalebox{0.75}{
        \begin{tikzpicture}
        % First Layer
        \node (input1) at (3, 7) {$x_1 = 2$};
        \node (input2) at (5, 7) {$x_2 = 7$};
        \node (A) at (1, 7) {$3$};
        \node (B) at (7, 7) {$5$};
        % Second Layer
        \node[draw, rectangle] (mul21) at (3, 6) {$\times$};
        \node[above left=0.01cm of mul21] {$2$};
        \node[above right=0.01cm of mul21] {$2$};

        \node[draw, rectangle] (mul22) at (6, 6) {$\times$};
        \node[above left=0.01cm of mul22] {$7$};
        \node[above right=0.01cm of mul22] {$5$};

        \draw[->] (input1) -- (2, 7) |- (mul21);
        \draw[->] (input1) -- (4, 7) |- (mul21);

        \draw[->] (input2) -- (5, 6.5) |- (mul22);
        \draw[->] (B) -- (7, 6.5) |- (mul22);

        % Third Layer
        \node[draw, rectangle] (mul31) at (2, 5) {$\times$};
        \node[above left=0.01cm of mul31] {$3$};
        \node[above right=0.01cm of mul31] {$4$};

        \draw[->] (mul21) -- (3, 5) |- (mul31);
        \draw[->] (A) -- (1, 5) |- (mul31);

        % Fourth Layer
        \node[draw, rectangle] (add41) at (4, 4) {$+$};
        \node[above left=0.01cm of add41] {$12$};
        \node[above right=0.01cm of add41] {$35$};

        \draw[->] (mul31) -- (2, 4) |- (add41);
        \draw[->] (mul22) -- (6, 4) |- (add41);

        % Fifth Layer
        \node (output) at (4, 3) { $47$ };

        \draw[->] (add41) -- (output);

        \end{tikzpicture}
      }
    \end{column}
% Column 2    
    \begin{column}{.5\textwidth}
        \small
        $$
          \begin{aligned}
            f_{GC}(X) &= a(X) q_l(X) \\
                      &+ b(X) q_r(X) \\
                      &+ c(X) q_o(X) \\
                      &+ a(X) b(X) q_m(X) \\
                      &+ q_c(X) \\
            \forall s \in S &= \{ \o^1, \o^2, \dots, \o^n \} : \\
            f_{GC}(s) &\meq 0
          \end{aligned}
        $$
    \end{column}%
\end{columns}

\small

\begin{center}
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|}
    \hline
    $\o^i$ & $a(\o^i)$ & $b(\o^i)$ & $c(\o^i)$ & $q_l(\o^i)$ & $q_r(\o^i)$ & $q_o(\o^i)$ & $q_m(\o^i)$ & $q_c(\o^i)$ \\\tabucline[1pt]{-}
    $\o^1$ & 3         & 0         & 0         & 1           & 0           & 0           & 0           & -3          \\\hline
    $\o^2$ & 5         & 0         & 0         & 1           & 0           & 0           & 0           & -5          \\\hline
    $\o^3$ & 47        & 0         & 0         & 1           & 0           & 0           & 0           & -47         \\\hline
    $\o^4$ & 2         & 2         & 4         & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^5$ & 5         & 7         & 35        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^6$ & 4         & 3         & 12        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^7$ & 35        & 12        & 47        & 1           & 1           & -1          & 0           & 0           \\\hline
    $\o^8$ & 0         & 0         & 0         & 0           & 0           & 0           & 0           & 0           \\\hline
  \end{tabu}
\end{center}

## Vanishing Argument: $\forall s \in S : f(s) \meq 0$

\small

$$
\renewcommand{\arraystretch}{1.75}
\begin{array}{>{\displaystyle}l >{\displaystyle}c >{\displaystyle}l}
\textbf{Prover}(f \in \Fb_{\leq d}[X])    &                                 & \textbf{Verifier}                           \\
C_f = \PCCommit(f(X), d, \bot)            &                                 &                                             \\
z_S(X) = \prod_{s \in S}(X - s)           &                                 &                                             \\
t(X) = \frac{f(X)}{z_S}                   &                                 &                                             \\
C_t = \PCCommit(t(X), d, \bot)            & \rarr{C_f, C_t}                 & \xi \in_R \Fb                               \\
v_f = f(\xi)                              & \larr{\xi}                      &                                             \\
\pi_f = \PCOpen(f(X), C_f, d, \xi, \bot)  &                                 &                                             \\
v_t = t(\xi)                              &                                 &                                             \\
\pi_t = \PCOpen(t(X), C_f, d, \xi, \bot)  & \rarr{v_f, \pi_f, v_t, \pi_t}   & v_f \meq v_t \cdot z_S(\xi)                 \\
                                          &                                 & \PCCheck(C_f, d, \xi, v_f, \pi_f)           \\
                                          &                                 & \PCCheck(C_t, d, \xi, v_t, \pi_t)           \\
\end{array}
$$

## Vanishing Argument: $\forall s \in S : f(s) \meq 0$

### Correctness

$$
\begin{aligned}
p(\xi) &= f(\xi) - t(\xi) z_S(\xi) \\
       &= f(\xi) - \left( \frac{f(\xi)}{z_S(\xi)} \right) z_S(\xi) \\
       &= 0
\end{aligned}
$$

### Soundness

- $z_S \; | \; f$ only if all of $s \in S : f(s) = 0$ (Factor Theorem)
- Schwartz-Zippel Lemma: $\xi \in_R \Fb : Pr[p(\xi) = 0 \; | \; p \neq 0] \leq \frac{\deg(p)}{|\Fb|}$
- $|\Fb| \gg \deg(p) \implies Pr[p(\xi) \; | \; p \neq 0] \leq \e$
- $\deg(p) \leq d \leq n$

## Copy Constraints

\begin{columns} 
% Column 1
    \begin{column}{.5\textwidth}
      \centering
      \scalebox{0.75}{
        \begin{tikzpicture}
        % First Layer
        \node (input1) at (3, 7) {$x_1 = 2$};
        \node (input2) at (5, 7) {$x_2 = 7$};
        \node (A) at (1, 7) {$3$};
        \node (B) at (7, 7) {$5$};
        % Second Layer
        \node[draw, rectangle] (mul21) at (3, 6) {$\times$};
        \node[above left=0.01cm of mul21] {$2$};
        \node[above right=0.01cm of mul21] {$2$};

        \node[draw, rectangle] (mul22) at (6, 6) {$\times$};
        \node[above left=0.01cm of mul22] {$7$};
        \node[above right=0.01cm of mul22] {$5$};

        \draw[->] (input1) -- (2, 7) |- (mul21);
        \draw[->] (input1) -- (4, 7) |- (mul21);

        \draw[->] (input2) -- (5, 6.5) |- (mul22);
        \draw[->] (B) -- (7, 6.5) |- (mul22);

        % Third Layer
        \node[draw, rectangle] (mul31) at (2, 5) {$\times$};
        \node[above left=0.01cm of mul31] {$3$};
        \node[above right=0.01cm of mul31] {$4$};

        \draw[->] (mul21) -- (3, 5) |- (mul31);
        \draw[->] (A) -- (1, 5) |- (mul31);

        % Fourth Layer
        \node[draw, rectangle] (add41) at (4, 4) {$+$};
        \node[above left=0.01cm of add41] {$12$};
        \node[above right=0.01cm of add41] {$35$};

        \draw[->] (mul31) -- (2, 4) |- (add41);
        \draw[->] (mul22) -- (6, 4) |- (add41);

        % Fifth Layer
        \node (output) at (4, 3) { $47$ };

        \draw[->] (add41) -- (output);

        \end{tikzpicture}
      }
    \end{column}
% Column 2    
    \begin{column}{.6\textwidth}
        \small
        $$
          \begin{alignedat}{2}
            a(\o^4) &= b(\o^4)               &&          \\
            b(\o^5) &= a(\o^2)               &&          \\
            a(\o^6) &= c(\o^4) \quad b(\o^6) &&= a(\o^1) \\
            a(\o^7) &= c(\o^5) \quad b(\o^7) &&= c(\o^6) \\
            c(\o^7) &= a(\o^3) \quad         &&            
          \end{alignedat}
        $$
    \end{column}%
\end{columns}

\small

\begin{center}
  \begin{tabu}{|c|[1pt]c|c|c|c|c|c|c|c|}
    \hline
    $\o^i$ & $a(\o^i)$ & $b(\o^i)$ & $c(\o^i)$ & $q_l(\o^i)$ & $q_r(\o^i)$ & $q_o(\o^i)$ & $q_m(\o^i)$ & $q_c(\o^i)$ \\\tabucline[1pt]{-}
    $\o^1$ & 3         & 0         & 0         & 1           & 0           & 0           & 0           & -3          \\\hline
    $\o^2$ & 5         & 0         & 0         & 1           & 0           & 0           & 0           & -5          \\\hline
    $\o^3$ & 47        & 0         & 0         & 1           & 0           & 0           & 0           & -47         \\\hline
    $\o^4$ & 2         & 2         & 4         & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^5$ & 5         & 7         & 35        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^6$ & 4         & 3         & 12        & 0           & 0           & -1          & 1           & 0           \\\hline
    $\o^7$ & 35        & 12        & 47        & 1           & 1           & -1          & 0           & 0           \\\hline
    $\o^8$ & 0         & 0         & 0         & 0           & 0           & 0           & 0           & 0           \\\hline
  \end{tabu}
\end{center}

## Copy Constraints

\small
$$
  \begin{alignedat}{2}
    a(\o^4) &= b(\o^4)               &&          \\
    b(\o^5) &= a(\o^2)               &&          \\
    a(\o^6) &= c(\o^4) \quad b(\o^6) &&= a(\o^1) \\
    a(\o^7) &= c(\o^5) \quad b(\o^7) &&= c(\o^6) \\
    c(\o^7) &= a(\o^3) \quad         &&            
  \end{alignedat}
$$

$$
\begin{aligned}
      \vec{f} &= \vec{a} \cat \vec{b} \cat \vec{c} \\
              &= [ a_1, a_2, a_3, a_4, a_5, a_6, a_7, a_8, b_1, b_2, b_3, b_4, b_5, b_6, b_7, b_8, c_1, c_2, c_3, c_4, c_5, c_6, c_7, c_8], \\
  \s(\vec{f}) &= [ b_6, b_5, a_3, b_4, a_5, c_4, c_5, a_8, b_1, b_2, b_3, a_4, a_2, a_1, c_6, b_8, c_1, c_2, c_3, a_6, a_7, b_7, a_3, c_8]
\end{aligned}
$$

$$\forall i \in [n] : f(\o^i) \meq f(\o^{\s(i)})$$

## Copy Constraints - The Grand Product Argument

\small
\begin{columns}
  % Column 1
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
      \forall \o \in H : f(\o) &= g(\o), \\
      f'(X) &:= f(X) + \gamma, \\
      g'(X) &:= g(X) + \gamma, \\
      \prod_{i \in [n]} f'(\o^i) &\meq \prod_{i \in [n]} g'(\o^i)
    \end{aligned}
    $$
  \end{column}
  % Column 2    
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
      p(X) &= \prod_{i \in [n]} f(\o^i) + X \\
      q(X) &= \prod_{i \in [n]} g(\o^i) + X \\
    \end{aligned}
    $$
  \end{column}
\end{columns}

### Soundness $p(X) = q(X) \implies \{ a_1, \dots, a_n \} = \{ b_1, \dots, b_n \}$

\setlength{\columnsep}{0.5em}
\begin{columns}
  \centering
  % Column 1
  \begin{column}{.025\textwidth}
  \end{column}
  \begin{column}{.4\textwidth}
    \centering
    $$
    \begin{aligned}
      \text{roots}(p(X)) &= \{ -f(\o^1), \dots, -f(\o^n) \} \\
                         &= \{ -a_1, \dots, -a_n \} \\
    \end{aligned}
    $$
  \end{column}
  \begin{column}{.025\textwidth}
  \end{column}
  % Column 2    
  \begin{column}{.4\textwidth}
      \centering
      $$
      \begin{aligned}
        \text{roots}(q(X)) &= \{ -g(\o^1), \dots, -g(\o^n) \} \\
                           &= \{ -b_1, \dots, -b_n \}
      \end{aligned}
      $$
  \end{column}
  \begin{column}{.075\textwidth}
  \end{column}
\end{columns}

- Since the two polynomials are equal, they must have the same roots

$$
\begin{aligned}
  \text{roots}(p(X)) &= \text{roots}(q(X)) \implies \\
  \{ -a_1, \dots, -a_n \} &= \{ -b_1, \dots, -b_n \} \implies \\
  \{ a_1, \dots, a_n \} &= \{ b_1, \dots, b_n \}
\end{aligned}
$$

## Copy Constraints - The Grand Product Argument

\small
\begin{columns}
  % Column 1
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
      z(\o^1) &= 1 \\
      z(\o^i) &= \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} \\
    \end{aligned}
    $$
  \end{column}
  % Column 2    
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
      z(\o^i)              &= \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} \\
      z(\o^i)              &= z(\o^{i-1}) \frac{f'(\o^{i-1})}{g'(\o^{i-1})} \\
      z(\o^{i+1})          &= z(\o^i) \frac{f'(\o^i)}{g'(\o^i)} \\
      z(\o^{i+1}) g'(\o^i) &= z(\o^i) f'(\o^i) \\
      z(\o^i) f'(\o^i)     &= z(\o \cdot \o^i) g'(\o^i) \\
    \end{aligned}
    $$
  \end{column}
  \begin{column}{.1\textwidth}
  \end{column}
\end{columns}

### Verifier Checks

$$
\begin{aligned}
  f_{CC_1}(X) &= l_1(X)(z(X) - 1) \\
  f_{CC_2}(X) &= z(X) f'(X) - z(\o X) g'(X) \\
\end{aligned}
$$

## Copy Constraints - The Grand Product Argument

### Checking implies $\prod_{\o \in H} f'(\o) = \prod_{\o \in H} g'(\o)$

- For the $i = n$ case:

$$
\begin{aligned}
  z(\o^n) f'(\o^n)                                                 &= z(\o^{n+1}) g'(\o) \\
  \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)} f'(\o^n)          &= g'(\o) \\
  \prod_{1 \leq j < i} \frac{f'(\o^j) f'(\o^n)}{g'(\o^j) g'(\o^n)} &= 1 \\
  \frac{\prod_{i \in [n]} f'(\o^i)}{\prod_{i \in [n]} g'(\o^i)}    &= 1 \\
  \prod_{i \in [n]} f'(\o^i)                                       &= \prod_{i \in [n]} g'(\o^i) \\
\end{aligned}
$$

## Copy Constraints - The Grand Product Argument Protocol

\small
$$
\renewcommand{\arraystretch}{1.75}
\begin{array}{>{\displaystyle}l >{\displaystyle}c >{\displaystyle}l}
\textbf{Prover}(f, g \in \Fb_{\leq d}[X])                        &                        & \textbf{Verifier}                           \\
C_f = \PCCommit(f(X), d, \bot)                                   &                        &                                             \\
C_g = \PCCommit(g(X), d, \bot)                                   & \rarr{C_f, C_g}        & \a, \b \in_R \Fb                            \\
z(\o^1) = 1                                                      &                        &                                             \\
z(\o^i) = \prod_{1 \leq j < i} \frac{f(\o^j) + \g}{g(\o^j) + \g} & \lrarr{\hspace{2.5em}} & \forall h \in H :                           \\
                                                                 & \lrarr{\hspace{2.5em}} & f_{CC_1}(h) \meq l_1(h) (z(h) - 1)          \\
                                                                 & \lrarr{\hspace{2.5em}} & f_{CC_2}(h) \meq z(h) f'(h) - z(\o h) g'(h) \\
\end{array}
$$

## Copy Constraints - Applying the Grand Product Argument

- $\Pc$ wants to prove that for $i \in [n] : f_i = f_{\s(i)}$:
$$
\begin{aligned}
  &\{ (f_i, i) \mid i \in [1,n] \} = \{ (f_i, \s(i)) \mid i \in [1,n] \} \implies \\
  &f_i = f_{\s_i} \implies \\
  &\vec{f} = \s(\vec{f})
\end{aligned}
$$
- Meaning for polynomials:
$$
\begin{aligned}
  &\{ (f(\o^i), \id(\o^i)) \mid i \in [1,n] \} = \{ (f(\o^i), \s(\o^i)) \mid i \in [1,n] \} \implies \\
  &f(\o^i) = f(\o^{\s(i)})
\end{aligned}
$$
- Which is exactly suited for the **Grand Product Argument**, with:
$$f'(X) = f(X) + \beta \id(X), \quad g'(X) = f(X) + \beta \s(X)$$

## Public Inputs

- Public inputs, $\vec{x} : |\vec{x}| = \ell_2$
- Leading to a public input polynomial, $x(X) = \ifft(\vec{x})$

\small
$$f_{GC}(X) = a(X) q_l(X) + b(X) q_r(X) + c(X) q_o(X) + a(X) b(X) q_m(X) + q_c(X) + x(X)$$

## Custom Gates

1. Add a new selector polynomial
2. Create a constraint table 
3. Convert constraint table to 

## Custom Gates - Double-and-Add

\small
\begin{algorithm}[H]
\caption*{
  \textbf{Double-and-Add Scalar Multiplication} $A = x \cdot P$
}
\begin{algorithmic}[1]
  \State Let $\vec{b}$ be the bits of $x$, from LSB to MSB.
  \State Let $A = \Oc$.
  \State Let $acc = 0$.
  \For{$i \in (255, 0]$}
    \State $acc \mathrel{+}= b_i \cdot 2^i$
    \State $Q := 2 A$
    \State $R := P + Q$
    \State $S := \textbf{if } b_i = 1 \textbf{ then } R \textbf{ else } Q$
    \State $A := S$
  \EndFor
  \State \textbf{assert} $acc \meq x$
  \State \Return A
\end{algorithmic}
\end{algorithm}

## Custom Gates - Double-and-Add - Double

- $A = \Oc \implies Q = \Oc$:
- $A \neq \Oc \implies Q = 2A$:

$$
\begin{aligned}
  \l  &= \frac{3x_a^2}{2y_a} \\
  x_q &= \l_q^2 - 2 \cdot x_a \\
  y_q &= \l_q \cdot (x_a - x_q) - y_a \\
\end{aligned}
$$

- Witness:

$$
\begin{aligned}
  \g_q &= \text{inv0}(x_a) \\
  \l_q &= \begin{cases}
    \frac{3x_a^2}{2y_a}, & \text{ if } A \neq \Oc, \\
    0,                   & \text{ otherwise.}
  \end{cases} \\
\end{aligned}
$$

## Custom Gates - Double-and-Add - Double Constraints

\small
\begin{table}[H]
  \centering
  \begin{tabu}{|c|ll|ll|}
    \hline
    Degree & & Constraint                                  & & Meaning                              \\\tabucline[1pt]{-}
    4      & & $(1 - x_a \cdot \g_q) \cdot x_q$            & & $x_a = 0 \implies x_q = 0$           \\
    4      & & $(1 - x_a \cdot \g_q) \cdot y_q$            & & $x_a = 0 \implies y_q = 0$           \\\hline
    3      & & $(2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2)$  & & $\l_q = \frac{3x_a^2}{2y_a}$         \\\hline
    3      & & $(\l_q^2 - 2 \cdot x_a - x_q)$              & & $x_q = \l_q^2 - 2 \cdot x_a$         \\\hline
    3      & & $(\l_q \cdot (x_a - x_q) - y_a - y_q)$      & & $y_q = \l_q \cdot (x_a - x_q) - y_a$ \\\hline
  \end{tabu}
\end{table}

## Custom Gates - Double-and-Add - Ternary Constraints

\small
\begin{table}[H]
  \centering
  \begin{tabu}{|c|ll|ll|}
    \hline
    Degree & & Constraint                                    & & Meaning                                                               \\\tabucline[1pt]{-}
    3      & & $b_i \cdot (b_i - 1)$                         & & $b_i \in \Bb$                                                         \\\hline
    3      & & $x_s - (b_i \cdot x_r + (1 - b_i) \cdot x_q)$ & & $x_s = \textbf{ if } b_i = 1 \textbf{ then } x_r \textbf{ else } x_q$ \\
    3      & & $y_s - (b_i \cdot y_r + (1 - b_i) \cdot y_q)$ & & $y_s = \textbf{ if } b_i = 1 \textbf{ then } y_r \textbf{ else } y_q$ \\\hline
    3      & & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$         & & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$                                 \\\hline
  \end{tabu}
\end{table}

## Custom Gates - Double-and-Add

\small

\begin{table}[H]
  \centering
  \begin{tabu}{|c|ll|ll|}
    \hline
    Degree & & Constraint                                    & & Meaning                              \\\tabucline[1pt]{-}
    4      & & $(1 - x_a \cdot \g_q) \cdot x_q$              & & $x_a = 0 \implies x_q = 0$           \\
    4      & & $(1 - x_a \cdot \g_q) \cdot y_q$              & & $x_a = 0 \implies y_q = 0$           \\\hline
    3      & & $2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2$      & & $\l_q = \frac{3x_a^2}{2y_a}$         \\\hline
    3      & & $\l_q^2 - 2 \cdot x_a - x_q$                & & $x_q = \l_q^2 - 2 \cdot x_a$         \\\hline
    3      & & $(\l_q \cdot (x_a - x_q) - y_a - y_q)$        & & $y_q = \l_q \cdot (x_a - x_q) - y_a$ \\\tabucline[1pt]{-}
    3      & & $b_i \cdot (b_i - 1)$                         & & $b_i \in \Bb$                                                         \\\hline
    3      & & $x_s - (b_i \cdot x_r + (1 - b_i) \cdot x_q)$ & & $x_s = \textbf{ if } b_i = 1 \textbf{ then } x_r \textbf{ else } x_q$ \\
    3      & & $y_s - (b_i \cdot y_r + (1 - b_i) \cdot y_q)$ & & $y_s = \textbf{ if } b_i = 1 \textbf{ then } y_r \textbf{ else } y_q$ \\\hline
    3      & & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$         & & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$                                 \\\hline
  \end{tabu}
\end{table}

$$
\begin{alignedat}{1}
f_{GC}(X) &= \dots + q_{(\cdot)} \cdot ( \\
          &\quad \zeta^0 \cdot ((1 - x_a \cdot \g_q) \cdot x_q) \\
          &\quad \zeta^1 \cdot ((1 - x_a \cdot \g_q) \cdot y_q) \\
          &\quad \zeta^2 \cdot (2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2) \\
          &\quad \dots \\
          &)
\end{alignedat}
$$

## Custom Gates - Double-and-Add

\small

\begin{table}[H]
  \centering
  \begin{tabu}{|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$  & $w_2$    & $w_3$    & $w_4$    & $w_5$    & $w_6$    & $w_7$    & $w_8$    \\\tabucline[1pt]{-}
    $x_a$  & $y_a$    & $acc$    & $x_p$    & $y_p$    & $x_q$    & $y_q$    & $x_r$    \\\hline\hline
    $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$ \\\tabucline[1pt]{-}
    $y_r$  & $b_i$    & $\g_q$   & $\l_q$   & $\a_r$   & $\b_r$   & $\d_r$   & $\l_r$   \\\hline
  \end{tabu}
\end{table}

\scriptsize
\begin{columns}
  % Column 1
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
    f_{GC}(X) &= \dots + q_{(\cdot)} \cdot ( \\
              &\quad \zeta^0 \cdot ((1 - x_a \cdot \g_q) \cdot x_q) \\
              &\quad \zeta^1 \cdot ((1 - x_a \cdot \g_q) \cdot y_q) \\
              &\quad \zeta^2 \cdot (2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2) \\
              &\quad \dots \\
              &)
    \end{aligned}
    $$
  \end{column}
  % Column 2    
  \begin{column}{.4\textwidth}
    $$
    \begin{aligned}
    f_{GC}(X) &= \dots + q_{(\cdot)}(X) \cdot ( \\
              &\quad \zeta^0 \cdot ((1 - w_1(X) \cdot \g_q) \cdot w_6(X)) \\
              &\quad \zeta^1 \cdot ((1 - w_1(X) \cdot \g_q) \cdot w_6(X)) \\
              &\quad \zeta^2 \cdot (2 \cdot w_2(X) \cdot w_{12}(X) - 3 \cdot w_1(X)^2) \\
              &\quad \dots \\
              &)
    \end{aligned}
    $$
  \end{column}
  \begin{column}{.1\textwidth}
  \end{column}
\end{columns}

# IVC

## Chain of Signatures

$$B_i = \{ \s^{(pk)}_i, j_i = i, pk_i, ptr_i \in \Bb^{256}, \s^{(ptr)}_i \}$$

- $\s^{(pk)}_i$: A signature on the public key of the current committee
  ($pk_i$), signed by the previous committee identified by the public key
  $pk_{i-1}$.
- $j_i$: A sequential block-id. This must be present for the soundness of
  the IVC circuit.
- $pk_i$: The public key of the current committee.
- $ptr_i$: A hash of the most recent block on the main blockchain.
- $\s^{(ptr)}_i$: A signature on $ptr_i$, signed by the current public key.

$$\text{Verify}_{pk_{i-1}}(\s^{(pk)}, pk_i) \land \text{Verify}_{pk_i}(\s^{(ptr)}, ptr_i) \land j_i \meq j_{i-1} + 1$$

## IVC

\begin{center}
\scalebox{0.65}{
\begin{tikzpicture}
  % First Layer
  %% Nodes
  \node[draw, rectangle] (R_ivc_p) at (2.25, 6.5) {$R_{IVC}^{(p)}$};
  \node[draw, rectangle] (x_prev_p) at (3.5, 6.5) {$x_{i-1}^{(p)}$};
  \node[draw, rectangle] (pi_prev_p) at (4.75, 6.5) {$\pi_{i-1}^{(p)}$};

  \node[draw, rectangle] (q_p) at (6, 6.5) {$\vec{q}^{(p)}$};
  \node[draw, rectangle] (acc_prev_p) at (7.5, 6.5) {$\acc_{i-1}^{(p)}$};
  \node[draw, rectangle] (acc_next_p) at (9, 6.5) {$\acc_i^{(p)}$};

  \node[draw, rectangle] (R_ivc_q) at (10.75, 6.5) {$R_{IVC}^{(q)}$};
  \node[draw, rectangle] (x_prev_q) at (12, 6.5) {$x_{i-1}^{(q)}$};
  \node[draw, rectangle] (pi_prev_q) at (13.25, 6.5) {$\pi_{i-1}^{(q)}$};

  \node[draw, rectangle] (q_q) at (14.5, 6.5) {$\vec{q}^{(p)}$};
  \node[draw, rectangle] (acc_prev_q) at (16, 6.5) {$\acc_{i-1}^{(p)}$};
  \node[draw, rectangle] (acc_next_q) at (17.5, 6.5) {$\acc_i^{(p)}$};

  %% Arrows
  \draw[dashed-arrow] (R_ivc_p) -- (2.25, 7.1) -- (3.5, 7.1) -- (x_prev_p);
  \draw[dashed-arrow] (pi_prev_p) -- (4.75, 7.1) -- (6, 7.1) -- (q_p);
  \draw[dashed-arrow] (acc_prev_p) -- (7.5, 7.4) -- (3.5, 7.4) -- (x_prev_p);

  \draw[dashed-arrow] (R_ivc_q) -- (10.75, 7.1) -- (12, 7.1) -- (x_prev_q);
  \draw[dashed-arrow] (pi_prev_q) -- (13.25, 7.1) -- (14.5, 7.1) -- (q_q);
  \draw[dashed-arrow] (acc_prev_q) -- (16, 7.4) -- (12, 7.4) -- (x_prev_q);

  % Second Layer
  \node[draw, rectangle] (svf_p) at (3.5, 5) {$\PlonkVerifierFast$};
  \node[draw, rectangle] (asv_p) at (7.5, 5) {$\ASVerifier$};

  \node[draw, rectangle] (svf_q) at (12, 5) {$\PlonkVerifierFast$};
  \node[draw, rectangle] (asv_q) at (16, 5) {$\ASVerifier$};

  %% Arrows
  \draw[arrow] (R_ivc_p) -- (2.25, 6) -- (3.5, 5.75) -- (svf_p);
  \draw[arrow] (x_prev_p) -- (3.5, 6) -- (3.5, 5.75) -- (svf_p);
  \draw[arrow] (pi_prev_p) -- (4.75, 6) -- (3.5, 5.75) -- (svf_p);

  \draw[arrow] (q_p) -- (6, 6) -- (7.5, 5.75) -- (asv_p);
  \draw[arrow] (acc_prev_p) -- (7.5, 6) -- (7.5, 5.75) -- (asv_p);
  \draw[arrow] (acc_next_p) -- (9, 6) -- (7.5, 5.75) -- (asv_p);

  \draw[arrow] (R_ivc_q) -- (10.75, 6) -- (12, 5.75) -- (svf_q);
  \draw[arrow] (x_prev_q) -- (12, 6) -- (12, 5.75) -- (svf_q);
  \draw[arrow] (pi_prev_q) -- (13.25, 6) -- (12, 5.75) -- (svf_q);

  \draw[arrow] (q_q) -- (14.5, 6) -- (16, 5.75) -- (asv_q);
  \draw[arrow] (acc_prev_q) -- (16, 6) -- (16, 5.75) -- (asv_q);
  \draw[arrow] (acc_next_q) -- (17.5, 6) -- (16, 5.75) -- (asv_q);

  % Third Layer
  \node[draw, rectangle] (and) at (9.5, 3) {$\land$};

  %% Arrows
  \draw[arrow] (svf_p) -- (3.5, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (asv_p) -- (7.5, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (svf_q) -- (12, 4.5) -- (9.5, 3.75) -- (and);
  \draw[arrow] (asv_q) -- (16, 4.5) -- (9.5, 3.75) -- (and);

  % Fourth Layer
  \node[draw, rectangle] (s_next) at (3.25, 3.5) {$s_i$};
  \node[draw, rectangle] (s_prev) at (4.5, 3.5) {$s_{i-1}$};
  \node[draw, rectangle] (s_0) at (5.75, 3.5) {$s_0$};

  % Fifth Layer
  \node[draw, rectangle] (zero) at (5.75, 2) {$s_{i-1} \meq s_0$};
  \node[draw, rectangle] (F) at (3.25, 2) {$F'(s_{i-1}, s_i)$};

  \draw[arrow] (s_next) -- (3.25, 3) -- (3.25, 2.75) -- (F);
  \draw[arrow] (s_prev) -- (4.25, 3) -- (3.25, 2.75) -- (F);

  \draw[arrow] (s_prev) -- (4.75, 3) -- (5.75, 2.75) -- (zero);
  \draw[arrow] (s_0) -- (5.75, 3) -- (5.75, 2.75) -- (zero);

  % Sixth Layer
  \node[draw, rectangle] (or1) at (8, 0.5) {$\lor$};
  
  \draw[arrow] (zero) -- (5.75, 1.5) -- (8, 1.25) -- (or1);
  \draw[arrow] (and) -- (9.5, 1.5) -- (8, 1.25) -- (or1);

  % Sixth Layer
  \node[draw, rectangle] (and2) at (5.75, -1) {$\land$};
  \draw[arrow] (or1) -- (8, 0) -- (5.75, -0.25) -- (and2);
  \draw[arrow] (F) -- (3.25, 0) -- (5.75, -0.25) -- (and2);

\end{tikzpicture}
}
\end{center}

# Benchmarks and Conclusion

## Benchmarks

- **IVC-Prover:** Parallel: ~300 s. Single Threaded: ~900 s.
- **IVC-Verifier:** Parallel: ~3 s. Single Threaded: ~9 s.
- **Naive Signature Verification:** Parallel: ~1300 signatures per
  second. Single Threaded: ~310 signatures per second.

### When is it better?

- $1300 \cdot 3 \; / \; 2 = 1950$ days before the IVC verifier would be faster
- The ~10 kB Proof would be smaller after only 87 days

## Conclusion

### The project:
  - Fully implemented and understood a complex IVC-scheme
  - Showed IVC may be useful in the context of blockchain catch-up
  - Optimizations, lookups, quantum security...

## Plonk Arguments - Batched Evaluation Proofs

\small

$$
\renewcommand{\arraystretch}{1.75}
\begin{array}{>{\displaystyle}l >{\displaystyle}c >{\displaystyle}l}
\textbf{Prover}(\vec{f} \in \Fb_{\leq d}^k[X]) &                         & \textbf{Verifier}                           \\
C_{f_i} = \PCCommit(f_i(X), d, \bot)           & \rarr{\vec{C_f}}        & \a, \xi \in_R \Fb                           \\
w(X) = \sum_{i = 0}^{k-1} \a^i f_i(X)          & \larr{\a, \xi}          &                                             \\
C_w(X) = \PCCommit(w(X), d, \bot)              &                         &                                             \\
v_{f_i} = f_i(\xi)                             &                         &                                             \\
\pi_w = \PCOpen(w(X), C_w, d, \xi, \bot)       & \rarr{\pi_w, \vec{v_f}} & C_w = \sum_{i = 0}^{k-1} \a^i C_{f_i}       \\
                                               &                         & v_w = \sum_{i = 0}^{k-1} \a^i v_{f_i}       \\
                                               &                         & \PCCheck(C_w, d, \xi, v_w, \pi_w)           \\
\end{array}
$$

## Plonk Arguments - Batched Evaluation Proofs Security

### Correctness

- $\PCCheck(C_w, d, \xi, v_w, \pi_w)$
  - $C_w = \sum_{i = 0}^{k-1} \a^i C_{f_i} = \PCCommit(w(X), d, \bot)$
  - $v_w = \sum_{i = 0}^{k-1} \a^i v_{f_i} = w(\xi)$

### Soundness

$$p(\a) := \sum_{i=0}^{k-1} \a^i (f_i(\xi) - v_i)$$

- Schwartz-Zippel...

