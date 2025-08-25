## Interpolate

At a high level, the interpolation algorithm performs copy constraint and $\plookup$ prep for each wire type.

For copy constraints, it will compute the root of unity $\omega$ with the minimum order for the number of rows. Then it computes the cosets $\vec{H}$, where there is a coset per copy constraint column. The public columns $\sigma_i$ for copy constraint columns are created with $\sigma$ computing their evaluations.

The trace table then is split into $x, R, w$ where public input column goes into $x$, the rest of the public columns in $R$, and the private columns in $w$. Every column then is mapped with fast fourier interpolate to compute them into polynomials.

$$
\begin{array}{ccc}
\begin{array}{rl}
\text{pow2} &: \Nb \to \Nb \\
\text{pow2}(n) &= 2^{\lceil \log_2 (n) \rceil}
\end{array} &
\begin{array}{rl}
\text{unity}_{\Fb_q} &: \to \Nb \to \Fb_q \\
\text{unity}_{\Fb_q}(n) &= \maybe{\omega}{
\begin{array}{rl}
  \omega &\in \Fb_q \setminus 1 \\
  \omega^n &= 1 \Leftrightarrow |\omega| = n
\end{array}
}
\end{array} &
\begin{array}{rl}
\text{cosets} &: (\omega: \Fb_q) \to \Fb_q^{|\omega| \times |CC|}\\
\text{cosets}(\omega) &= \maybe{\vec{H}}{
\begin{array}{l}
  \vec{H}_1 = \langle \omega \rangle \\
  k_{i>1} \in \Fb_q \setminus \bigcup\limits_{j<i} \vec{H}_j \cup \{1\} \\
  \vec{H}_{i>1} = k_i \langle \omega^i \rangle \\
\end{array}
}
\end{array}
\end{array}
$$
$$
\begin{array}{rl}
\text{interpolate} &: \text{TraceResult} \to \text{Pub} \times \text{Circuit} \times \Option(\text{Priv}) \\
\text{interpolate}(\sigma, C) &= \begin{cases}
& N_t = \text{pow2} \circ \max\limits_{s \in \Column} |C^t(s)|\\
& \vec{H^t} = \text{cosets} \circ \text{unity}_{W(t)}(N_t) \\
\cdots & \cdots
\end{cases}
\end{array}
$$
