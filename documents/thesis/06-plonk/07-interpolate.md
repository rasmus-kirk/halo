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

- compute $\omega$ and pad
- x: take public columns
- R: compute $\sigma$ per row per CC column
- w: compute $h_1, h_2$ from t and f
- w: take private columns
- map $C$ with fft (f collected doesnt need fft, its used in Z construction only, check how verifier does vanishing on grand product)
- map with commits pair?
  - look at code

you can move this description to its own plookup section, including plookup prep in interpolate

plookup prep (in interpolate)

- collect f_i non bot cells into one column f
- check f length < C length; pad
- if larger then pad C with default rows
- resolve bot cells
- [NEW FUNC] make this into its own function to abstract away

lookup gadget batching optimization (in build)

- build only has gadgets for single lookups for arbitrary tables of arbitrary number of columns
- these can be lookup producing output wires and lookup asserts
- we sort these gadgets into buckets of their table's number of columns
- each bucket then is chunked into size priv column amount divided by table column number
- the largest chunk size is the number of f_i columns we need; generate in spec
- each chunk is a dynamically created higher order gadget
  - its output wires are all the output wires of its chunk
  - its input wires are all the input wires of its chunk
  - its ctrn however does not have the argument of in ++ out
    - but rather for each gadget in its chunk in_1 ++ out_1 ++ in_2 ++ out_2 ++ ..
    - thus e.g. the bucket is of column 3, i.e. binary input with unary output
    - we can chunk the wires by 3, and let W_i, W_i+1, W_i+2 slots assign per chunk
    - we compute f_i for each wire chunk
    - if not all wire chunks are used, we need default somehow ??? TODO default entry / row per table num cols, instead of just last entry in table vector, but need to omit in f compute, but cannot be just bot, so bot_col ???
  - term then is q_col (sum (sum zeta^i-1 W_((i-1)col + j)) + zeta^col j - f_i)
- [NEW FUNC] this post build but before trace logic will be its own optimizing function, maybe nested in build

disable q_k for dupe base gadgets (in trace)

- when a relative gadget has a multi lookup as its base, and the base is duplicated
- you will consume unecessary f_i for grand product
- thus, u need to modify trace, such that when a multi lookup is duplicated (exists in Omega)
- you augment its ctrn to be qk = 0 i.e. its term when resolved is just zero / it doesnt check anything
- and its fine, because its original (non duplicate) already checks it, the dupe is simply there to provide relative wiring
- [NEW FUNC] i.e. phi=1 has a function wrapping ctrn_g and omega to do this augmentation

remember to update feature list of surkal if u pull this off

can u design the abstraction such that u dont have to handle it at all three places? build, trace and interpolate. e.g. can u combine it for trace and build at least.

**Interpolate Correctness Example**

TODO
