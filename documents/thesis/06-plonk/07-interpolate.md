## Interpolate

$$
\begin{array}{rl}
\text{pow2} &: \Nb \to \Nb \\
\text{pow2}(n) &= 2^{\lceil \log_2 (n) \rceil} \\
\\
\text{unity}_{\Fb_q} &: \to \Nb \to \Fb_q \\
\text{unity}_{\Fb_q}(n) &= \maybe{\omega}{
\begin{array}{rl}
  \omega &\in \Fb_q \setminus 1 \\
  \omega^n &= 1 \Leftrightarrow |\omega| = n
\end{array}
}\\
\\
\text{cosets} &: (\omega: \Fb_q) \to \Fb_q^{|\omega| \times |CC|}\\
\text{cosets}(\omega) &= \maybe{\vec{H}}{
\begin{array}{l}
  \vec{H}_1 = \langle \omega \rangle \\
  k_{i>1} \in \Fb_q \setminus \bigcup\limits_{j<i} \vec{H}_j \cup \{1\} \\
  \vec{H}_{i>1} = k_i \langle \omega^i \rangle \\
\end{array}
}\\
\\
\text{interpolate} &: \text{TraceResult} \to \text{Pub} \times \text{Circuit} \times \Option(\text{Priv}) \\
\text{interpolate}(\sigma, C) &= \begin{cases}
a
& N_t = \text{pow2} \circ \max\limits_{s \in \Column} |C^t(s)|\\
& \vec{H^t} = \text{cosets} \circ \text{unity}_{W(t)}(N_t) \\
\end{cases}
\end{array}
$$

TODO 

- pad $C$ to equal row length per type
- x: take public columns
- R: compute $\sigma$ per row per CC column
- w: take private columns
- w: compute $h_1, h_2$ for plookup
- map $C$ with fft
- map with commits pair?
  - look at code

**Interpolate Correctness Example**

TODO
