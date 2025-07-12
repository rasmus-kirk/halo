### Relation

The construction of subterms of $F_{GC}$ is done by $\GateGroup$ via $gc$:

$$
\begin{array}{rl}
\GateGroup &= \GateType^k \times (gc: X^l \to X^{|\Slot| + |\Selector|} \to X)
\end{array}
$$

$$
\begin{array}{rl}
\text{Term} &= \text{Slot} + \text{Selector} + \set{J} \\
\text{Lookup} &= \set{T, F, H_1, H_2} \\
\text{PolyType} &= \text{Slot} + \text{Selector} + \text{Lookup} \\
\\
\text{pow2} &: \Nb \to \Nb \\
\text{pow2}(n) &= 2^{\lceil \log_2 (n) \rceil} \\
\\
\text{unity} &: \Nb \to \Fb_q \\
\text{unity}(n) &= \maybe{\omega}{
\begin{array}{rl}
  \omega &\in \Fb_q \setminus 1 \\
  \omega^n &= 1
\end{array}
}\\
\\
\text{relation} &: \text{TraceResult} \to R \\
\text{relation}(\vec{\sigma}, \vec{C}) &= \begin{cases}
a
& N = \text{pow2}(\max(|\vec{t}|, |\vec{C}|) + \text{blind})\\
& \omega = \text{unity}(N) \\
\end{cases}
\end{array}
$$

TODO 

- compute $k : (t: \text{WireType}) \to \text{Slot} \to W(t)$; $k^q_s : \Fb_q$
- lookup thunk
  - up to $N$ minus blind rows
  - table vector $\vec{t}$
  - query vector $\vec{f}$ using $\vec{C}$ in $A,B,C,j$
  - grand product $\vec{h_1}, \vec{h_2}$
- expand $\vec{C}$
- expand $\vec{\sigma}$
- fft + cache
- commits
  - look at code

notation ideas

- $w[A,i] = \vec{C}[A,i]$ cache
- $R[Q_l,i] = \vec{C}[Q_l,i]$ cache
- $R[Q_l] = \text{fft}(\vec{C}[Q_l])$ poly
- $x[A] = \bot$ does not exist
- $w_\zeta[T,i] = ?[T,i]$ thunk cache
- $w_\zeta[T] = \text{fft}(?[T])$ thunk poly
- $w[\mathcal{C}_A] = \PCCommit(\text{fft}(\vec{C}[A]), \ldots)$ commit
- $w_\zeta[\mathcal{C}_T] = \PCCommit(\text{fft}(?[T]), \ldots)$ commit thunk
- $w[t,A,i]$ typed indexing (or maybe we dont need to if we split into two runs of prover and verifier)

**Relation Correctness Example**

TODO
