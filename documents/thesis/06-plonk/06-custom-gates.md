# Gates

For each gate, we have a *Witness Row*, *Selector Row* and a *Coefficient
Row*. These rows describe the form of the constraints. Take the addition
gate as an example:

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:example-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $a$   & $b$   & $c$   & 0      & 0      & 0      & 0      & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

Here the first row describes the 16 witness inputs associated for the gate. In
this case only three witnesses are needed, so the other columns are set to 0.
The Second row describes the copy constraints. In this case that means that
slot 1 (corresponding to $w_1$), is copy constrained to the left input wire,
slot 2 (corresponding to $w_2$) is copy constrained to the right input wire
and slot 3 (corresponding to $w_3$) is copy constrained to the first, and
for this gate; only, output wire.

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:example-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    1     & 1     & -1    & 0     & 0     & 0       & 0     & 0       & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

This means that the $q_l = 1, q_r = 1, q_o = -1$ is set for this row. So
that for this row $f_{GC}$ becomes:
$$f_{GC} = q_l w_1 + q_r w_2 + q_o w_3 + \dots = 0 \implies w_1 + w_2 - w_3 = 0$$

We also have a coefficient row for each gate, that can store constants
for each gate. This is used in the scalar multiplication, poseidon and
range-check gates. For all other gates they are set to zero. If they are
not listed in a gate specification below, then all row-values are set to
zero. This is also the case for our example add gate.

\begin{center}
  \captionof*{table}{Coefficient Row} \label{tab:example-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $r_1$ & $r_2$ & $r_3$ & $r_4$ & $r_5$ & $r_6$ & $r_7$ & $r_8$ & $r_9$ & $r_{10}$ & $r_{11}$ & $r_{12}$ & $r_{13}$ & $r_{14}$ & $r_{15}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0     & 0   & 0     & 0        & 0        & 0        & 0        & 0        & 0        \\\hline
  \end{tabu}
\end{center}

## Field

### Addition, Subtraction, Multiplication, Negation

For completeness, we include the witness tables for field addition, subtraction,
multiplication and negation even though they are part of vanilla-Plonk:

**Addition:**

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:field-add-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$ & $w_5$ & $w_6$ & $w_7$ & $w_8$ & $w_9$ & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$ \\\tabucline[1pt]{-}
    $a$   & $b$   & $c$   & 0      & 0      &  0     &  0     & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline 
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:field-add-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    1     & 1     & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0     \\\hline
  \end{tabu}
\end{center}

**Subtraction:**

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:field-sub-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $a$   & $b$   & $c$   & 0      & 0      & 0      & 0      & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:field-sub-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    1     & -1    & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

**Multiplication:**

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:field-mul-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $a$   & $b$   & $c$   & 0      & 0      & 0      & 0      & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:field-mul-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

**Negation:**

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:field-neg-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$  &  $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $a$   & 0      &  0     & 0      & 0      & 0      & 0      & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $I_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:field-neg-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    -1    & 0     & -1    & 0     & 0     & 0       & 0     & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

**Inverse:**

To model inverses, we can witness the inverse of $x$, $x^{-1}$, and constrain
that $x \cdot x^{-1} = 1$:

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:field-neg-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$    &  $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $x$   & $x^{-1}$ &  0     & 0      & 0      & 0      & 0      & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline
    $I_1$ & $O_1$    & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:field-neg-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 1     & 1     & 0       & 0     & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

## Booleans

### Witness Boolean

To witness a boolean, we need to constrain that the witnessed value indeed
is a bit. So we need that:

$$(b \cdot b) - b = 0$$

This can be modelled using the native plonk selector polynomials.

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-bool-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$  & $w_2$  & $w_3$  & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$  \\\tabucline[1pt]{-}
    $b$    & $b$    & 0      & 0      & 0      & 0      & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        & 0        & 0         \\\hline
    $O_1$  & $O_1$  & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-bool-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    -1    & 0     & 0     & 1     & 0     & 0       & 0   & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

### Equals

To check whether two values are equal, $b = x \meq y$, we need to witness $b$
and $\text{inv0}(x - y)$:

$$
\begin{aligned}
  b &= \begin{cases} 
    1 & \text{if } x = y, \\
    0 & \text{otherwise}.
  \end{cases} \\
  \text{inv0}(x) &= \begin{cases} 
    x^{-1} & \text{if } x \neq 0, \\
    0      & \text{otherwise}.
  \end{cases} \\
  \a &= \text{inv0}(x - y) \\
\end{aligned}
$$

Now, we want that $x = y \implies b = 1$ and $a \neq b \implies \a = 0$.

| Degree | | Constraint                                             | Meaning                                 |
|:------:|-|:-------------------------------------------------------|:----------------------------------------|
|      3 | | $q_{\text{(=)}} \cdot ((x - y) \cdot b) = 0$           | $x \neq y \implies b = 0$               |
|      3 | | $q_{\text{(=)}} \cdot ((x - y) \cdot \a + b - 1) = 0$  | $x = y \implies b = 1$                  |

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:equals-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$ \\\tabucline[1pt]{-}
    $x$   & $y$   & $b$   & $\a$   & 0      & 0      & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        & 0        & 0      \\\hline
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:equals-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0     & 0       & 0             & 1         & 0       \\\hline
  \end{tabu}
\end{center}

**Completeness:**

**Case $x \neq y \land b = 0$:**

$$(x - y) \cdot 0 = 0$$
$$(x - y) \cdot (x - y)^{-1} + 0 - 1 = 1 - 1 = 0$$

**Case $x = y \land b = 1$:**

$$(x - y) \cdot 1 = 0 \cdot 1 = 0$$
$$(x - y) \cdot 0 + 1 - 1 = 1 - 1 = 0$$

**Soundness:**

The first constraint is trivial. For the second constraint:

**Case $x \neq y$:**

The first constraint ensures that $b = 0$ in this case:

$$
\begin{aligned}
  (x - y) \cdot \a + 0 - 1 &= 0 \implies \\
  (x - y) \cdot \a &= 1
\end{aligned}
$$

**Case $x = y$:**
$$
\begin{aligned}
  (x - y) \cdot \a + b - 1 &= 0 \implies \\
  0 \cdot \a + b - 1 &= 0 \implies \\
  b &= 1
\end{aligned}
$$

### And, Or

To implement "And" for two booleans, $x, y$, we can simply multiply them,
costing a single row. Because $x, y$ are constrained to be bits, when they are
input (subsequent operations like "And" and "Or" are guaranteed to produce
bits, provided the inputs of these operations are bits). To implement "Or",
we can compose the following constraint that $c = x \lor y = x + y - (x \cdot
y)$. To see why it works, given that $x, y$ are already constrained to be bits:

\begin{center}
  \begin{tabu}{|c|c|c|}
    \hline
    $x$ & $y$ & Out                       \\\tabucline[1pt]{-}
    0   & 0   & $0 + 0 - (0 \cdot 0) = 0$ \\\hline
    0   & 1   & $0 + 1 - (0 \cdot 1) = 1$ \\\hline
    1   & 0   & $1 + 0 - (1 \cdot 0) = 1$ \\\hline
    1   & 1   & $1 + 1 - (1 \cdot 1) = 1$ \\\hline
  \end{tabu}
\end{center}

This can naively be done in three rows, but we can compress it to a single row as $0 = x + y - c - (x \cdot y)$:

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-point-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$ & $w_6$ & $w_7$ & $w_8$ & $w_9$ & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$    \\\tabucline[1pt]{-}
    $a$   & $b$   & $c$   & 0      & 0      & 0      & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        & 0        & 0      \\\hline
    $I_1$ & $I_2$ & $O_1$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$ \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-point-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    1     & 1     & -1    & -1    & 0     & 0       & 0     & 0         & 0           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

## Rangecheck

We want to constrain $x \in [0, 2^{254})$. We decompose $x$ into 254 bits and check that:

$$x = \sum_{i=0}^{253} b_i \cdot 2^i$$

The entire range-check then consists of $254 / 15 = 17$ rows:

\begin{center}
  \captionof*{table}{Witness Table} \label{tab:rangecheck-witness}
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$      & $w_2$     & $w_3$     & $w_4$     & $w_5$     & $w_6$     & $w_7$     & $w_8$     & $w_9$     & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$  \\\tabucline[1pt]{-}
    $acc_0$    & $b_0$     & $b_1$     & $b_2$     & $b_3$     & $b_4$     & $b_5$     & $b_6$     & $b_7$     & $b_8$     & $b_9$     & $b_{10}$  & $b_{11}$  & $b_{12}$  & $b_{13}$  & $b_{14}$  \\\hline
    $acc_1$    & $b_{15}$  & $b_{16}$  & $b_{17}$  & $b_{18}$  & $b_{19}$  & $b_{20}$  & $b_{21}$  & $b_{22}$  & $b_{23}$  & $b_{24}$  & $b_{25}$  & $b_{26}$  & $b_{27}$  & $b_{28}$  & $b_{29}$  \\\hline
    $acc_2$    & $b_{30}$  & $b_{31}$  & $b_{32}$  & $b_{33}$  & $b_{34}$  & $b_{35}$  & $b_{36}$  & $b_{37}$  & $b_{38}$  & $b_{39}$  & $b_{40}$  & $b_{41}$  & $b_{42}$  & $b_{43}$  & $b_{44}$  \\\hline
    $acc_3$    & $b_{45}$  & $b_{46}$  & $b_{47}$  & $b_{48}$  & $b_{49}$  & $b_{50}$  & $b_{51}$  & $b_{52}$  & $b_{53}$  & $b_{54}$  & $b_{55}$  & $b_{56}$  & $b_{57}$  & $b_{58}$  & $b_{59}$  \\\hline
    $acc_4$    & $b_{60}$  & $b_{61}$  & $b_{62}$  & $b_{63}$  & $b_{64}$  & $b_{65}$  & $b_{66}$  & $b_{67}$  & $b_{68}$  & $b_{69}$  & $b_{70}$  & $b_{71}$  & $b_{72}$  & $b_{73}$  & $b_{74}$  \\\hline
    $acc_5$    & $b_{75}$  & $b_{76}$  & $b_{77}$  & $b_{78}$  & $b_{79}$  & $b_{80}$  & $b_{81}$  & $b_{82}$  & $b_{83}$  & $b_{84}$  & $b_{85}$  & $b_{86}$  & $b_{87}$  & $b_{88}$  & $b_{89}$  \\\hline
    $acc_6$    & $b_{90}$  & $b_{91}$  & $b_{92}$  & $b_{93}$  & $b_{94}$  & $b_{95}$  & $b_{96}$  & $b_{97}$  & $b_{98}$  & $b_{99}$  & $b_{100}$ & $b_{101}$ & $b_{102}$ & $b_{103}$ & $b_{104}$ \\\hline
    $acc_7$    & $b_{105}$ & $b_{106}$ & $b_{107}$ & $b_{108}$ & $b_{109}$ & $b_{110}$ & $b_{111}$ & $b_{112}$ & $b_{113}$ & $b_{114}$ & $b_{115}$ & $b_{116}$ & $b_{117}$ & $b_{118}$ & $b_{119}$ \\\hline
    $acc_8$    & $b_{120}$ & $b_{121}$ & $b_{122}$ & $b_{123}$ & $b_{124}$ & $b_{125}$ & $b_{126}$ & $b_{127}$ & $b_{128}$ & $b_{129}$ & $b_{130}$ & $b_{131}$ & $b_{132}$ & $b_{133}$ & $b_{134}$ \\\hline
    $acc_9$    & $b_{135}$ & $b_{136}$ & $b_{137}$ & $b_{138}$ & $b_{139}$ & $b_{140}$ & $b_{141}$ & $b_{142}$ & $b_{143}$ & $b_{144}$ & $b_{145}$ & $b_{146}$ & $b_{147}$ & $b_{148}$ & $b_{149}$ \\\hline
    $acc_{10}$ & $b_{150}$ & $b_{151}$ & $b_{152}$ & $b_{153}$ & $b_{154}$ & $b_{155}$ & $b_{156}$ & $b_{157}$ & $b_{158}$ & $b_{159}$ & $b_{160}$ & $b_{161}$ & $b_{162}$ & $b_{163}$ & $b_{164}$ \\\hline
    $acc_{11}$ & $b_{165}$ & $b_{166}$ & $b_{167}$ & $b_{168}$ & $b_{169}$ & $b_{170}$ & $b_{171}$ & $b_{172}$ & $b_{173}$ & $b_{174}$ & $b_{175}$ & $b_{176}$ & $b_{177}$ & $b_{178}$ & $b_{179}$ \\\hline
    $acc_{12}$ & $b_{180}$ & $b_{181}$ & $b_{182}$ & $b_{183}$ & $b_{184}$ & $b_{185}$ & $b_{186}$ & $b_{187}$ & $b_{188}$ & $b_{189}$ & $b_{190}$ & $b_{191}$ & $b_{192}$ & $b_{193}$ & $b_{194}$ \\\hline
    $acc_{13}$ & $b_{195}$ & $b_{196}$ & $b_{197}$ & $b_{198}$ & $b_{199}$ & $b_{200}$ & $b_{201}$ & $b_{202}$ & $b_{203}$ & $b_{204}$ & $b_{205}$ & $b_{206}$ & $b_{207}$ & $b_{208}$ & $b_{209}$ \\\hline
    $acc_{14}$ & $b_{210}$ & $b_{211}$ & $b_{212}$ & $b_{213}$ & $b_{214}$ & $b_{215}$ & $b_{216}$ & $b_{217}$ & $b_{218}$ & $b_{219}$ & $b_{220}$ & $b_{221}$ & $b_{222}$ & $b_{223}$ & $b_{224}$ \\\hline
    $acc_{15}$ & $b_{225}$ & $b_{226}$ & $b_{227}$ & $b_{228}$ & $b_{229}$ & $b_{230}$ & $b_{231}$ & $b_{232}$ & $b_{233}$ & $b_{234}$ & $b_{235}$ & $b_{236}$ & $b_{237}$ & $b_{238}$ & $b_{239}$ \\\hline
    $acc_{16}$ & $b_{240}$ & $b_{241}$ & $b_{242}$ & $b_{243}$ & $b_{244}$ & $b_{245}$ & $b_{246}$ & $b_{247}$ & $b_{248}$ & $b_{249}$ & $b_{250}$ & $b_{251}$ & $b_{252}$ & $b_{253}$ & 0         \\\hline
    $\bot$     & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

The last row still indicates the copy constraints[^acc0]. Each $acc_i$ is the accumulation of all previously witnessed bits, so:

$$
\begin{aligned}
  acc_0 &= 0 \\
  acc_1 &= \sum_{i=0}^{14} b_i \cdot 2^i \\
  acc_2 &= \sum_{i=0}^{29} b_i \cdot 2^i \\
  acc_3 &= \dots \\
\end{aligned}
$$

However, for this, we still need to witness each power of two. Luckily, these are constant and fixed in the circuit specification, so we can use the coefficient table for this:

\begin{center}
  \captionof*{table}{Coefficient Table} \label{tab:scalar-mul-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $r_1$     & $r_2$     & $r_3$     & $r_4$     & $r_5$     & $r_6$     & $r_7$     & $r_8$     & $r_9$     & $r_{10}$  & $r_{11}$  & $r_{12}$  & $r_{13}$  & $r_{14}$  & $r_{15}$ \\\tabucline[1pt]{-}
    $2^0$     & $2^1$     & $2^2$     & $2^3$     & $2^4$     & $2^5$     & $2^6$     & $2^7$     & $2^8$     & $2^9$     & $2^{10}$  & $2^{11}$  & $2^{12}$  & $2^{13}$  & $2^{14}$ \\\hline
    $2^{15}$  & $2^{16}$  & $2^{17}$  & $2^{18}$  & $2^{19}$  & $2^{20}$  & $2^{21}$  & $2^{22}$  & $2^{23}$  & $2^{24}$  & $2^{25}$  & $2^{26}$  & $2^{27}$  & $2^{28}$  & $2^{29}$ \\\hline
    $2^{30}$  & $2^{31}$  & $2^{32}$  & $2^{33}$  & $2^{34}$  & $2^{35}$  & $2^{36}$  & $2^{37}$  & $2^{38}$  & $2^{39}$  & $2^{40}$  & $2^{41}$  & $2^{42}$  & $2^{43}$  & $2^{44}$ \\\hline
    $2^{45}$  & $2^{46}$  & $2^{47}$  & $2^{48}$  & $2^{49}$  & $2^{50}$  & $2^{51}$  & $2^{52}$  & $2^{53}$  & $2^{54}$  & $2^{55}$  & $2^{56}$  & $2^{57}$  & $2^{58}$  & $2^{59}$ \\\hline
    $2^{60}$  & $2^{61}$  & $2^{62}$  & $2^{63}$  & $2^{64}$  & $2^{65}$  & $2^{66}$  & $2^{67}$  & $2^{68}$  & $2^{69}$  & $2^{70}$  & $2^{71}$  & $2^{72}$  & $2^{73}$  & $2^{74}$ \\\hline
    $2^{75}$  & $2^{76}$  & $2^{77}$  & $2^{78}$  & $2^{79}$  & $2^{80}$  & $2^{81}$  & $2^{82}$  & $2^{83}$  & $2^{84}$  & $2^{85}$  & $2^{86}$  & $2^{87}$  & $2^{88}$  & $2^{89}$ \\\hline
    $2^{90}$  & $2^{91}$  & $2^{92}$  & $2^{93}$  & $2^{94}$  & $2^{95}$  & $2^{96}$  & $2^{97}$  & $2^{98}$  & $2^{99}$  & $2^{100}$ & $2^{101}$ & $2^{102}$ & $2^{103}$ & $2^{104}$ \\\hline
    $2^{105}$ & $2^{106}$ & $2^{107}$ & $2^{108}$ & $2^{109}$ & $2^{110}$ & $2^{111}$ & $2^{112}$ & $2^{113}$ & $2^{114}$ & $2^{115}$ & $2^{116}$ & $2^{117}$ & $2^{118}$ & $2^{119}$ \\\hline
    $2^{120}$ & $2^{121}$ & $2^{122}$ & $2^{123}$ & $2^{124}$ & $2^{125}$ & $2^{126}$ & $2^{127}$ & $2^{128}$ & $2^{129}$ & $2^{130}$ & $2^{131}$ & $2^{132}$ & $2^{133}$ & $2^{134}$ \\\hline
    $2^{135}$ & $2^{136}$ & $2^{137}$ & $2^{138}$ & $2^{139}$ & $2^{140}$ & $2^{141}$ & $2^{142}$ & $2^{143}$ & $2^{144}$ & $2^{145}$ & $2^{146}$ & $2^{147}$ & $2^{148}$ & $2^{149}$ \\\hline
    $2^{150}$ & $2^{151}$ & $2^{152}$ & $2^{153}$ & $2^{154}$ & $2^{155}$ & $2^{156}$ & $2^{157}$ & $2^{158}$ & $2^{159}$ & $2^{160}$ & $2^{161}$ & $2^{162}$ & $2^{163}$ & $2^{164}$ \\\hline
    $2^{165}$ & $2^{166}$ & $2^{167}$ & $2^{168}$ & $2^{169}$ & $2^{170}$ & $2^{171}$ & $2^{172}$ & $2^{173}$ & $2^{174}$ & $2^{175}$ & $2^{176}$ & $2^{177}$ & $2^{178}$ & $2^{179}$ \\\hline
    $2^{180}$ & $2^{181}$ & $2^{182}$ & $2^{183}$ & $2^{184}$ & $2^{185}$ & $2^{186}$ & $2^{187}$ & $2^{188}$ & $2^{189}$ & $2^{190}$ & $2^{191}$ & $2^{192}$ & $2^{193}$ & $2^{194}$ \\\hline
    $2^{195}$ & $2^{196}$ & $2^{197}$ & $2^{198}$ & $2^{199}$ & $2^{200}$ & $2^{201}$ & $2^{202}$ & $2^{203}$ & $2^{204}$ & $2^{205}$ & $2^{206}$ & $2^{207}$ & $2^{208}$ & $2^{209}$ \\\hline
    $2^{210}$ & $2^{211}$ & $2^{212}$ & $2^{213}$ & $2^{214}$ & $2^{215}$ & $2^{216}$ & $2^{217}$ & $2^{218}$ & $2^{219}$ & $2^{220}$ & $2^{221}$ & $2^{222}$ & $2^{223}$ & $2^{224}$ \\\hline
    $2^{225}$ & $2^{226}$ & $2^{227}$ & $2^{228}$ & $2^{229}$ & $2^{230}$ & $2^{231}$ & $2^{232}$ & $2^{233}$ & $2^{234}$ & $2^{235}$ & $2^{236}$ & $2^{237}$ & $2^{238}$ & $2^{239}$ \\\hline
    $2^{240}$ & $2^{241}$ & $2^{242}$ & $2^{243}$ & $2^{244}$ & $2^{245}$ & $2^{246}$ & $2^{247}$ & $2^{248}$ & $2^{249}$ & $2^{250}$ & $2^{251}$ & $2^{252}$ & $2^{253}$ & 0         \\\hline
  \end{tabu}
\end{center}

Now for the constraints, for each _row_ in the tables above:

\begin{center}
  \captionof*{table}{Range Check Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|cllll|}
    \hline
    Degree & & Constraint                                             & & Meaning                                                 \\\tabucline[1pt]{-}
    3      & & $b_{(i \cdot 15+0)} \cdot (b_{(i \cdot 15+0)} - 1)$    & & $b_{(i \cdot 15+0)}, \dots b_{(i \cdot 15+14)} \in \Bb$ \\
    3      & & $b_{(i \cdot 15+1)} \cdot (b_{(i \cdot 15+1)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+2)} \cdot (b_{(i \cdot 15+2)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+3)} \cdot (b_{(i \cdot 15+3)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+4)} \cdot (b_{(i \cdot 15+4)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+5)} \cdot (b_{(i \cdot 15+5)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+6)} \cdot (b_{(i \cdot 15+6)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+7)} \cdot (b_{(i \cdot 15+7)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+8)} \cdot (b_{(i \cdot 15+8)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+9)} \cdot (b_{(i \cdot 15+9)} - 1)$    & &                                                         \\
    3      & & $b_{(i \cdot 15+11)} \cdot (b_{(i \cdot 15+11)} - 1)$  & &                                                         \\
    3      & & $b_{(i \cdot 15+12)} \cdot (b_{(i \cdot 15+12)} - 1)$  & &                                                         \\
    3      & & $b_{(i \cdot 15+13)} \cdot (b_{(i \cdot 15+13)} - 1)$  & &                                                         \\
    3      & & $b_{(i \cdot 15+14)} \cdot (b_{(i \cdot 15+14)} - 1)$  & &                                                         \\\hline
    2      & & $acc_{i+1}$                                            & & $acc_{i+1} = acc_i + \sum_{j=0}^{14} b_{(i \cdot 15+j)}$ \\
    2      & & $-acc_i$                                               & &                                                         \\
    3      & & $-(b_{(i \cdot 15+0)} \cdot 2^{(i \cdot 15+0)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+1)} \cdot 2^{(i \cdot 15+1)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+2)} \cdot 2^{(i \cdot 15+2)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+3)} \cdot 2^{(i \cdot 15+3)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+4)} \cdot 2^{(i \cdot 15+4)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+5)} \cdot 2^{(i \cdot 15+5)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+6)} \cdot 2^{(i \cdot 15+6)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+7)} \cdot 2^{(i \cdot 15+7)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+8)} \cdot 2^{(i \cdot 15+8)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+9)} \cdot 2^{(i \cdot 15+9)})$       & &                                                         \\
    3      & & $-(b_{(i \cdot 15+10)} \cdot 2^{(i \cdot 15+10)})$     & &                                                         \\
    3      & & $-(b_{(i \cdot 15+11)} \cdot 2^{(i \cdot 15+11)})$     & &                                                         \\
    3      & & $-(b_{(i \cdot 15+12)} \cdot 2^{(i \cdot 15+12)})$     & &                                                         \\
    3      & & $-(b_{(i \cdot 15+13)} \cdot 2^{(i \cdot 15+13)})$     & &                                                         \\
    3      & & $-(b_{(i \cdot 15+14)} \cdot 2^{(i \cdot 15+14)})$     & &                                                         \\\hline
  \end{tabu}
\end{center}

However, notice that we reference the next $acc$ ($acc_{i+1}$) in the
constraints, but this can be modelled as $w_1(\omega X)$. Each of the powers
of two are available to the prover and verifier in the coefficient table. Note
that this fact also means that in the constraints like $-(b_{(i \cdot 15+0)}
\cdot 2^{(i \cdot 15+0)})$, $2^{(i \cdot 15+0)}$ is from the coefficient
table, meaning that $2^{(i \cdot 15+0)} = r_1$, which also increases the
degree from two to three.

**Analysis:**

- Bit constraints:
  The bit constraints follow the previously defined bit constraint. So from
  the first series we get:  
  $b_{(i \cdot 15+0)}, \dots b_{(i \cdot 15+14)} \in \Bb$
- $acc_{i+1}$ constraints:
  We want to capture that $x = \sum_{i=0}^{253} b_i \cdot 2^i$. We can
  make this hold if $acc_0 = 0$ (We can just copy-constrain $acc_0$ to a
  zero-constant in the circuit) and $acc_{i+1} = acc_i + \sum_{j=0}^{14}
  b_{(i \cdot 15+j)}$. But taken together, this is exactly what these
  bottom-half constraints state, giving us:  
  $acc_{i+1} = acc_i + \sum_{j=0}^{14} b_{(i \cdot 15+j)} \implies acc_{17} = x = \sum_{i=0}^{253} b_i \cdot 2^i$  

Finally, due to the fact that we store the sum in the _next_ row, we need
a single zero row to capture the result of the sum:

\begin{center}
  \captionof*{table}{Range-check (zero-row) Witness Row} \label{tab:field-add-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$      & $w_2$  & $w_3$  & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$  & $w_{11}$  & $w_{12}$  & $w_{13}$  & $w_{14}$  & $w_{15}$  & $w_{16}$ \\\tabucline[1pt]{-}
    $acc_{17}$ & 0      & 0      & 0      & 0      &  0     &  0     & 0      & 0      & 0         & 0         & 0         & 0         & 0         & 0         & 0         \\\hline 
    $I_1$      & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Range-check (zero-row) Selector Row} \label{tab:field-add-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0   & 0         & 0             & 0         & 0       \\\hline
  \end{tabu}
\end{center}

We copy constrain $acc_{17}$ to $I_1$ to indicate that $x = acc_{17}$ must hold.

[^acc0]: Except, this table doesn't capture the fact that $acc_0$ needs to
         be constrained to a zero constant value in the circuit! All other
         rows does not have any copy constraints associated with it.


## Poseidon
<!-- TODO: reference -->

We also create a special gate type for poseidon hashing. This gate type is
inspired from an equivalent gate in Mina's kimchi proof system. At the heart
of the Poseidon hashing algorithm lies a cryptographic sponge construction,
like the one seen in SHA3. This is very convenient for fiat-shamir
transformations, since information sent to the verifier can cleanly be
modelled as sponge absorption, and queries made to the verifier can be
modelled as sponge squeezing. Squeezing and absorbing from the sponge a
certain number of times, triggers a permutation of the sponge state. The
original poseidon paper provide several small veriations on how this
permutation can be performed, with a variable number of partial and full
rounds of permutation. Kimchi's approach to this is to only perform the
expensive full rounds, but conversely make a highly specialized gate for
only these full rounds.

A complete permutation of the poseidon sponge state of size 3, then consists
of 55 full rounds of the following computation:
$$
\begin{aligned}
  s_i &= [s_{i,0}, s_{i,1}, s_{i,2}]^\top \\
  \text{sbox}(x) &= x^7 \\
  \vec{s_{i+1}} &= \vec{M} \cdot (\text{sbox}(\vec{s_i})) + [r_{i,0}, r_{i,1}, r_{i,2}]^\top \\
\end{aligned}
$$
$\vec{M} \in \Fb^{(3,3)}$ represents the constant MDE matrix, and $r_i$
represents the 55 round constants. Both of these were extracted from kimchi,
as we wanted our hash to have the same behaviour and security. If we split
this computation:

$$
\begin{aligned}
  s_0' &= \vec{M}{0,0} \cdot s_0^7 + \vec{M}{0,1} \cdot s_0^7 + \vec{M}{0,2} \cdot s_0^7 + r_{i,0} \\
  s_1' &= \vec{M}{1,0} \cdot s_1^7 + \vec{M}{1,1} \cdot s_1^7 + \vec{M}{1,2} \cdot s_1^7 + r_{i,1} \\
  s_2' &= \vec{M}{2,0} \cdot s_2^7 + \vec{M}{2,1} \cdot s_2^7 + \vec{M}{2,2} \cdot s_2^7 + r_{i,2} \\
\end{aligned}
$$

Leading us to the constraints:

\begin{center}
  \captionof*{table}{Poseidon Check Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|cllll|}
    \hline
    Degree & & Constraint                                                                                                   & & Meaning                                                                                  \\\tabucline[1pt]{-}
    3      & & $s_{1,0} - \vec{M}{0,0} \cdot s_{0,0}^7 + \vec{M}{0,1} \cdot s_{0,0}^7 + \vec{M}{0,2} \cdot s_{0,0}^7 + r_{0,0}$ & &                                                                                          \\
    3      & & $s_{1,1} - \vec{M}{1,0} \cdot s_{0,1}^7 + \vec{M}{1,1} \cdot s_{0,1}^7 + \vec{M}{1,2} \cdot s_{0,1}^7 + r_{0,1}$ & &  $\vec{s_1} = \vec{M} \cdot (\text{sbox}(\vec{s_0})) + [r_{0,0}, r_{0,1}, r_{0,2}]^\top$ \\
    3      & & $s_{1,2} - \vec{M}{2,0} \cdot s_{0,2}^7 + \vec{M}{2,1} \cdot s_{0,2}^7 + \vec{M}{2,2} \cdot s_{0,2}^7 + r_{0,2}$ & &                                                                                          \\\hline
    3      & & $s_{2,0} - \vec{M}{0,0} \cdot s_{1,0}^7 + \vec{M}{0,1} \cdot s_{1,0}^7 + \vec{M}{0,2} \cdot s_{1,0}^7 + r_{1,0}$ & &                                                                                          \\
    3      & & $s_{2,1} - \vec{M}{1,0} \cdot s_{1,1}^7 + \vec{M}{1,1} \cdot s_{1,1}^7 + \vec{M}{1,2} \cdot s_{1,1}^7 + r_{1,1}$ & &  $\vec{s_2} = \vec{M} \cdot (\text{sbox}(\vec{s_1})) + [r_{1,0}, r_{1,1}, r_{1,2}]^\top$ \\
    3      & & $s_{2,2} - \vec{M}{2,0} \cdot s_{1,2}^7 + \vec{M}{2,1} \cdot s_{1,2}^7 + \vec{M}{2,2} \cdot s_{1,2}^7 + r_{1,2}$ & &                                                                                          \\\hline
    3      & & $s_{3,0} - \vec{M}{0,0} \cdot s_{2,0}^7 + \vec{M}{0,1} \cdot s_{2,0}^7 + \vec{M}{0,2} \cdot s_{2,0}^7 + r_{2,0}$ & &                                                                                          \\
    3      & & $s_{3,1} - \vec{M}{1,0} \cdot s_{2,1}^7 + \vec{M}{1,1} \cdot s_{2,1}^7 + \vec{M}{1,2} \cdot s_{2,1}^7 + r_{2,1}$ & &  $\vec{s_3} = \vec{M} \cdot (\text{sbox}(\vec{s_2})) + [r_{2,0}, r_{2,1}, r_{2,2}]^\top$ \\
    3      & & $s_{3,2} - \vec{M}{2,0} \cdot s_{2,2}^7 + \vec{M}{2,1} \cdot s_{2,2}^7 + \vec{M}{2,2} \cdot s_{2,2}^7 + r_{2,2}$ & &                                                                                          \\\hline
    3      & & $s_{4,0} - \vec{M}{0,0} \cdot s_{3,0}^7 + \vec{M}{0,1} \cdot s_{3,0}^7 + \vec{M}{0,2} \cdot s_{3,0}^7 + r_{3,0}$ & &                                                                                          \\
    3      & & $s_{4,1} - \vec{M}{1,0} \cdot s_{3,1}^7 + \vec{M}{1,1} \cdot s_{3,1}^7 + \vec{M}{1,2} \cdot s_{3,1}^7 + r_{3,1}$ & &  $\vec{s_4} = \vec{M} \cdot (\text{sbox}(\vec{s_3})) + [r_{3,0}, r_{3,1}, r_{3,2}]^\top$ \\
    3      & & $s_{4,2} - \vec{M}{2,0} \cdot s_{3,2}^7 + \vec{M}{2,1} \cdot s_{3,2}^7 + \vec{M}{2,2} \cdot s_{3,2}^7 + r_{3,2}$ & &                                                                                          \\\hline
    3      & & $s_{5,0} - \vec{M}{0,0} \cdot s_{4,0}^7 + \vec{M}{0,1} \cdot s_{4,0}^7 + \vec{M}{0,2} \cdot s_{4,0}^7 + r_{4,0}$ & &                                                                                          \\
    3      & & $s_{5,1} - \vec{M}{1,0} \cdot s_{4,1}^7 + \vec{M}{1,1} \cdot s_{4,1}^7 + \vec{M}{1,2} \cdot s_{4,1}^7 + r_{4,1}$ & &  $\vec{s_5} = \vec{M} \cdot (\text{sbox}(\vec{s_4})) + [r_{4,0}, r_{4,1}, r_{4,2}]^\top$ \\
    3      & & $s_{5,2} - \vec{M}{2,0} \cdot s_{4,2}^7 + \vec{M}{2,1} \cdot s_{4,2}^7 + \vec{M}{2,2} \cdot s_{4,2}^7 + r_{4,2}$ & &                                                                                          \\\hline
  \end{tabu}
\end{center}

For the first row:

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-point-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$    \\\tabucline[1pt]{-}
    $s_0$ & $s_1$ & $s_2$ & $s_3$  & $s_4$  & $s_5$  & $s_6$  & $s_7$  & $s_8$  & $s_9$    & $s_{10}$ & $s_{11}$ & $s_{12}$ & $s_{13}$ & $s_{14}$ & 0      \\\hline
    $I_1$ & $I_2$ & $I_3$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$ \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-point-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 1       & 0     & 0         & 0           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

This is missing the last 3 states, but these are used in the next
five rounds of the permutation, so you can add these constraints,
witnesses and selector polynomials 10 more times to complete the
permutation[^poseidon-cc]. Finally, a zero-row can be added
to store the final state after 55 rounds (11 times the above gates):

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-point-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$    \\\tabucline[1pt]{-}
    $s_0''$ & $s_1$ & $s_2$ & $s_3$  & $s_4$  & $s_5$  & $s_6$  & $s_7$  & $s_8$  & $s_9$    & $s_{10}$ & $s_{11}$ & $s_{12}$ & $s_{13}$ & $s_{14}$ & 0      \\\hline
    $I_1$ & $I_2$ & $I_3$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$ \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-point-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 1       & 0     & 0         & 0           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

[^poseidon-cc]: Obviously, for the next 10 rounds there is no copy constraints.

## Elliptic Curves

### Witness Point

Points are represented in Affine Form, and the identity point is represented
as $\Oc = (0,0)$. 0 is not a valid $x$-coordinate of a valid point, because
we need the curve equation to hold ($y^2 = x^3 + 5$), this is not possible
since 5 is not square in $\Fb_p$ and 0 is not a $y$-coordinate in a valid
point since $-5$ is not a cube in $\Fb_q$.

To witness a point, we have to constrain that the witnessed
point is on the curve. For the Pallas/Vesta curves used we have the curve
equation. So we need constraints that encodes that $x \neq
0 \land y \neq 0 \implies y^2 - x^3 - 5 = 0$:

\begin{center}
  \captionof*{table}{Custom Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|cllll|}
    \hline
    Degree & & Constraint                                             & & Meaning                                 \\\tabucline[1pt]{-}
    5      & & $(q_{\text{point}} \cdot x) \cdot (y^2 - x^3 - 5) = 0$ & & $x \neq 0 \implies (y^2 - x^3 - 5) = 0$ \\\hline
    5      & & $(q_{\text{point}} \cdot y) \cdot (y^2 - x^3 - 5) = 0$ & & $y \neq 0 \implies (y^2 - x^3 - 5) = 0$ \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-point-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$  & $w_4$  & $w_5$ & $w_6$ & $w_7$ & $w_8$ & $w_9$ & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$    \\\tabucline[1pt]{-}
    $x$   & $y$   & 0      & 0      & 0      & 0      & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        & 0        & 0      \\\hline
    $O_1$ & $O_2$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$ \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-point-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 1     & 0         & 0           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

Soundness and completeness hold trivially.

### Addition

In the constraints, we use a trick similar to the one used in the equality
gate, where we model the condition $x = 0 \implies y = z$ by using the
constraint $(1 - x \cdot \text{inv0}(x)) \cdot y - z = 0$. When $x = 0$:

$$
\begin{aligned}
  0 &= (1 - x \cdot \text{inv0}(x)) \cdot y - z \\
  0 &= (1 - 0) \cdot y - z \\
  0 &= y - z \\
\end{aligned}
$$

The inverse is there for correctness, so we don't constrain the $y - z$
when $x \neq 0$.

We witness:

$$
\begin{aligned}
  \a &= \text{inv0}(x_q - x_p) \\
  \b &= \text{inv0}(x_p) \\
  \g &= \text{inv0}(x_q) \\
  \d &= \begin{cases}
    \text{inv0}(y_q + y_p), & \text{ if } x_q = x_p, \\
    0,                      & \text{ otherwise.}
  \end{cases} \\
  \l &= \begin{cases}
    \frac{y_q - y_p}{x_q - x_p}, & \text{ if } x_q \neq x_p, \\
    \frac{3x_p^2}{2y_p},         & \text{ if } x_q \neq x_p, \\
    0,                           & \text{ otherwise.}
  \end{cases} \\
\end{aligned}
$$

Where:

$$
  \text{inv0}(x) = \begin{cases}
    0,   & \text{ if } x = 0, \\
    1/x, & \text{ otherwise.}
  \end{cases} \\
$$

\begin{center}
  \captionof*{table}{Custom Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|c|l|l|}
    \hline
    Degree & Constraint                                                                                    & Meaning                                                                                    \\\tabucline[1pt]{-} \rule{0pt}{14pt} \rule[-8pt]{0pt}{0pt}
    4      & $q_{(+)} \cdot (x_q - x_p) \cdot ((x_q - x_p) \cdot \l - (y_q - y_p)) = 0$                    & $x_q \neq x_p \implies \l = \frac{y_q - y_p}{x_q - x_p}$                                   \\\hline \rule{0pt}{14pt}
    5      & $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = 0$                 & $x_q = x_p \land y_p \neq 0 \implies \l = \frac{3x_p^2}{2y_p}$                             \\\rule[-8pt]{0pt}{0pt}
           &                                                                                               & $x_q = x_p \land y_p = 0 \implies x_p = 0$                                                 \\\hline \rule{0pt}{14pt}
    6      & $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$           & $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies x_r = \l^2 - x_p - x_q$           \\
    6      & $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$ & $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies y_r = \l \cdot (x_p - x_q) - y_p$ \\
    6      & $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$           & $x_p \neq 0 \land x_q \neq 0 \land y_q \neq y_p \implies x_r = \l^2 - x_p - x_q$           \\\rule[-8pt]{0pt}{0pt}
    6      & $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$ & $x_p \neq 0 \land x_q \neq 0 \land y_q \neq y_p \implies y_r = \l \cdot (x_p - x_q) - y_p$ \\\hline \rule{0pt}{14pt}
    4      & $q_{(+)} \cdot (1 - x_p \cdot \b) \cdot (x_r - x_q) = 0$                                      & $x_p = 0 \implies x_r = x_q$                                                               \\\rule[-8pt]{0pt}{0pt}
    4      & $q_{(+)} \cdot (1 - x_p \cdot \b) \cdot (y_r - y_q) = 0$                                      & $x_p = 0 \implies y_r = y_q$                                                               \\\hline \rule{0pt}{14pt}
    4      & $q_{(+)} \cdot (1 - x_q \cdot \b) \cdot (x_r - x_p) = 0$                                      & $x_q = 0 \implies x_r = x_p$                                                               \\\rule[-8pt]{0pt}{0pt}
    4      & $q_{(+)} \cdot (1 - x_q \cdot \b) \cdot (y_r - y_p) = 0$                                      & $x_q = 0 \implies y_r = y_p$                                                               \\\hline \rule{0pt}{14pt}
    4      & $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a - (y_q + y_p) \cdot \d) \cdot x_r = 0$               & $x_q = x_p \land y_q = -y_p \implies x_r = 0$                                              \\\rule[-8pt]{0pt}{0pt}
    4      & $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a - (y_q + y_p) \cdot \d) \cdot y_r = 0$               & $x_q = x_p \land y_q = -y_p \implies y_r = 0$                                              \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:witness-point-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$ & $w_2$ & $w_3$ & $w_4$ & $w_5$ & $w_6$ & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$  \\\tabucline[1pt]{-}
    $x_p$ & $y_p$ & $x_q$ & $y_q$ & $x_r$ & $y_r$ & $\a$   & $\b$   & $\g$   & $\d$     & $\l$     & 0        & 0        & 0        & 0        & 0         \\\hline
    $I_1$ & $I_2$ & $I_3$ & $I_4$ & $O_1$ & $O_2$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$    \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:witness-point-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0     & 1         & 0           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

**Analysis**

1. $q_{(+)} \cdot (x_q - x_p) \cdot ((x_q - x_p) \cdot \l - (y_q - y_p)) = 0$  

   Meaning that $x_q \neq x_p \implies \l = \frac{y_q - y_p}{x_q - x_p}$ Which
   is equivalent to stating $P \neq Q \implies \l = \frac{y_q - y_p}{x_q - x_p}$.

2. $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = 0$

   Meaning that $x_q = x_p \implies \l = \frac{3x_p^2}{2y_p}$, except if
   $y_p = 0$, then:

   $0 = (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = (2y_p \cdot \l - 3x_p^2) = -3x_p^2$

   Which is only satisfied if $x_p = 0$. So this means that the constraint
   ensures:  

   $x_q = x_p \land y_p \neq 0 \implies \l = \frac{3x_p^2}{2y_p}$  
   $x_q = x_p \land y_p = 0 \implies x_p = 0$

   Or:

   $(P = Q \lor Q = -P) \land P \neq \Oc \land Q \neq \Oc \implies \l = \frac{3x_p^2}{2y_p}$

3.
  a. $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  b. $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$
  c. $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  d. $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$

  It's clear that if $(x_p \cdot x_q \cdot (x_q - x_p) \neq 0 \implies x_p
  \neq 0 \land x_q \neq 0 x_q \neq x_p$. So 3.a states:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies x_r = \l^2 - x_p - x_q$.

  Constraint 3.b, 3.c, 3.d have similar meaning. Combining 3.a, 3.b, 3.c, 3.d yeilds:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  
  $x_p \neq 0 \land x_q \neq 0 \land y_q \neq y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$

  Or equivalently:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \land y_q \neq y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  

  From the curve we know that any point where $x$ or $y$ is 0, is invalid,
  except if it's the identity point. We can also combine the two implications:

  $x_p \neq 0 \land y_p \neq 0 \land x_q \neq 0 \land y_q \neq 0 \land x_q \neq x_p \land y_q \neq y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  

  Which simplifies to:

  $P \neq \Oc \land Q \neq \Oc \land P \neq Q \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$
4.
  a. $q_{(+)} \cdot (1 - x_p \cdot \b) \cdot (x_r - x_q) = 0$
  b. $q_{(+)} \cdot (1 - x_p \cdot \b) \cdot (y_r - y_q) = 0$

  Meaning that:  
  $x_p = 0 \land y_p = 0 \implies x_r = x_q \land y_r = y_q$
  $P = \Oc \implies R = Q$
5.
  a. $q_{(+)} \cdot (1 - x_q \cdot \b) \cdot (x_r - x_p) = 0$
  b. $q_{(+)} \cdot (1 - x_q \cdot \b) \cdot (y_r - y_p) = 0$

  Meaning that:  
  $x_q = 0 \land y_q = 0 \implies x_r = x_p \land y_r = y_p$
  $Q = \Oc \implies R = P$
6.
  a. $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a - (y_q + y_p) \cdot \d) \cdot x_r = 0$
  b. $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a - (y_q + y_p) \cdot \d) \cdot y_r = 0$

  Meaning that:  
  $x_q = x_p \land y_q = -y_p \implies x_r = 0 \land y_r = 0$

  Or:  
  $Q = -P \implies R = \Oc$


### Scalar Multiplication

We follow a standard double-and-add scalar multiplication algorithm. It's
specified below. It may seem a bit odd, but it's just to better relate to
the eventual constraints.

\begin{algorithm}[H]
\caption*{
  \textbf{Double-and-Add Scalar Multiplication}
}
\textbf{Inputs} \\
  \Desc{$x$}{The scalar.} \\
  \Desc{$P$}{The point to scale.} \\
\textbf{Output} \\
  \Desc{$A$}{
    $A = x \cdot P$
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

There is three points to compute, $Q, R, S$, we start by constraining the
doubling.

- $A = \Oc \implies Q = \Oc$:
- $A \neq \Oc \implies Q = 2A$:

Standard doubling dictates that to compute the doubling of $A = (x_a, y_a)$, $2A = Q =
(x_q, y_q)$:

$$
\begin{aligned}
  \l  &= \frac{3x_a^2}{2y_a} \\
  x_q &= \l_q^2 - 2 \cdot x_a \\
  y_q &= \l_q \cdot (x_a - x_q) - y_a \\
\end{aligned}
$$

Except when $A = \Oc$, then becomes $Q = \Oc$. From this we can derive the
constraints. Witness:

$$
\begin{aligned}
  \b_q &= \text{inv0}(x_a) \\
  \l_q &= \begin{cases}
    \frac{3x_a^2}{2y_a}, & \text{ if } A \neq \Oc, \\
    0,                   & \text{ otherwise.}
  \end{cases} \\
\end{aligned}
$$

And add the following constraints:

\begin{center}
  \captionof*{table}{Custom Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|c|ll|ll|}
    \hline
    Degree & & Constraint                                                        & & Meaning                              \\\tabucline[1pt]{-}
    4      & & $q_{(\cdot)} \cdot (1 - x_a \cdot \b_q) \cdot x_q = 0$            & & $x_a = 0 \implies x_q = 0$           \\
    4      & & $q_{(\cdot)} \cdot (1 - x_a \cdot \b_q) \cdot y_q = 0$            & & $x_a = 0 \implies y_q = 0$           \\\hline
    3      & & $q_{(\cdot)} \cdot (2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2) = 0$  & & $\l_q = \frac{3x_a^2}{2y_a}$         \\\hline
    3      & & $q_{(\cdot)} \cdot (\l_q^2 - 2 \cdot x_a - x_q) = 0$              & & $x_q = \l_q^2 - 2 \cdot x_a$         \\\hline
    3      & & $q_{(\cdot)} \cdot (\l_q \cdot (x_a - x_q) - y_a - y_q) = 0$      & & $y_q = \l_q \cdot (x_a - x_q) - y_a$ \\\hline
  \end{tabu}
\end{center}

**Propositions:**

1. $x_a = 0 \implies (x_q, y_q) = (0, 0)$
2. $\l_q = \frac{3x_a^2}{2y_a}$
3. $x_q = \l_q^2 - 2 \cdot x_a$
4. $y_q = \l_q \cdot (x_a - x_q) - y_a$

**Cases:**

- $A = (0, 0) = \Oc$:
  - Completeness:
    1. Holds because $(x_a, y_a) = (x_q, y_q) = (0, 0)$
    2. Holds because $0 = \l_q = x_a = y_a$
    3. Holds because $0 = x_q  = \l_q = x_a$
    4. Holds because $0 = y_q = \l_q = x_a = x_q = y_a$
  - Soundness: $(x_r, y_r) = (0, 0)$ is the only solution to 1.
- $A = (x_a, y_a) \neq \Oc$:
  - Completeness:
    (1) Holds because $x_a \neq 0$
    (2) Holds because $\l_q = 3x_p^2 / 2y_p$
    (3) Holds because $x_q = \l_q^2 - 2 \cdot x_a$
    (4) Holds because $y_q = \l_q \cdot (x_a - x_q) - y_a$
  - Soundness:

     Firstly, (2) states that $\l_q$ is computed correctly, (3) states that
     $x_q$ is computed correctly, (4) states that $y_q$ is computed correctly.
     Thus, $Q = 2A$

From here, we simply add the previous point addition constraints, with one
small change; we already have $\b_q$, which is used to check whether $A$
is the identity point. However, since $A = \Oc \implies Q = \Oc$, we can
replace the $\b$ from the point addition constraints with the already witnessed
$\b_q$. Soundness and completeness follow from the previous section. Finally,
we need to create constraints for $S = \textbf{ if } b_1 = 1 \textbf{ then }
R \textbf{ else } Q$.

\begin{center}
  \captionof*{table}{Custom Constraints} \label{tab:scalar-mul-bit-constraints} 
  \begin{tabu}{|c|ll|ll|}
    \hline
    Degree & & Constraint                                                          & & Meaning                                                               \\\tabucline[1pt]{-}
    3      & & $b_i \cdot (b_i - 1) = 0$                                           & & $b_i \in \Bb$                                                         \\\hline
    3      & & $q_{(\cdot)} \cdot x_s - (b_i \cdot x_r + (1 - b_i) \cdot x_q) = 0$ & & $x_s = \textbf{ if } b_i = 1 \textbf{ then } x_r \textbf{ else } x_q$ \\
    3      & & $q_{(\cdot)} \cdot y_s - (b_i \cdot y_r + (1 - b_i) \cdot y_q) = 0$ & & $y_s = \textbf{ if } b_i = 1 \textbf{ then } y_r \textbf{ else } y_q$ \\\hline
    3      & & $q_{(\cdot)} \cdot acc_{i+1} - (acc_i + b_i \cdot 2^i) = 0$         & & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$                                 \\\hline
  \end{tabu}
\end{center}

**Propositions:**

(1) $b_i \in \Bb$
    Standard bit constraint
(2) $S = \textbf{ if } b_i = 1 \textbf{ then } R \textbf{ else } Q$
    $x_s - (b_i \cdot x_r + (1 - b_i) \cdot x_q) \implies x_s = (b_i \cdot x_r + (1 - b_i) \cdot x_q)$  
    - $b_i = 0$: $x_s = (0 \cdot x_r + (1 - 0) \cdot x_q) \implies x_s = x_r$
    - $b_i = 1$: $x_s = (1 \cdot x_r + (1 - 1) \cdot x_q) \implies x_s = x_r$

    Since this also holds for $y_s, y_r, y_q$: $S = \textbf{ if } b_i = 1 \textbf{ then } R \textbf{ else } Q$
(3) $acc_{i+1} - (acc_i + b_i \cdot 2^i)$
    $acc_{i+1} = (acc_i + b_i \cdot 2^i)$

So, for each iteration of the loop, we witness:

$$
\begin{aligned}
  \g_q &= \text{inv0}(x_a) \\
  \l_q &= \begin{cases}
    \frac{3x_a^2}{2y_a}, & \text{ if } A \neq \Oc, \\
    0,                   & \text{ otherwise.}
  \end{cases} \\
  \a_r &= \text{inv0}(x_q - x_p) \\
  \b_r &= \text{inv0}(x_p) \\
  \d_r &= \begin{cases}
    \text{inv0}(y_q + y_p), & \text{ if } x_q = x_p, \\
    0,                      & \text{ otherwise.}
  \end{cases} \\
  \l_r &= \begin{cases}
    \frac{y_q - y_p}{x_q - x_p}, & \text{ if } x_q \neq x_p, \\
    \frac{3x_p^2}{2y_p},         & \text{ if } x_q \neq x_p, \\
    0,                           & \text{ otherwise.}
  \end{cases} \\
\end{aligned}
$$

\begin{center}
  \captionof*{table}{Custom Constraints} \label{tab:witness-point-constraints} 
  \begin{tabu}{|c|l|l|}
    \hline
    Degree & Constraint                                                                                      & Meaning                                                                                      \\\tabucline[1pt]{-}
    4      & $q_{(\cdot)} \cdot (1 - x_a \cdot \b_q) \cdot x_q$                                           & $x_a = 0 \implies x_q = 0$                                                                   \\
    4      & $q_{(\cdot)} \cdot (1 - x_a \cdot \b_q) \cdot y_q$                                           & $x_a = 0 \implies y_q = 0$                                                                   \\\hline
    3      & $q_{(\cdot)} \cdot (2 \cdot y_a \cdot \l_q - 3 \cdot x_a^2)$                                    & $\l_q = \frac{3x_a^2}{2y_a}$                                                                 \\\hline
    3      & $q_{(\cdot)} \cdot (\l_q^2 - 2 \cdot x_a - x_q)$                                               & $x_q = \l_q^2 - 2 \cdot x_a$                                                                 \\\hline
    3      & $q_{(\cdot)} \cdot (\l_q \cdot (x_a - x_q) - y_a - y_q)$                                       & $y_q = \l_q \cdot (x_a - x_q) - y_a$                                                         \\\hline
    4      & $q_{(\cdot)} \cdot (x_q - x_p) \cdot ((x_q - x_p) \cdot \l_r - (y_q - y_p))$                 & $x_q \neq x_p \implies \l_r = \frac{y_q - y_p}{x_q - x_p}$                                   \\\hline
    5      & $q_{(\cdot)} \cdot (1 - (x_q - x_p) \cdot \a_r) \cdot (2y_p \cdot \l_r - 3x_p^2)$            & $x_q = x_p \land y_p \neq 0 \implies \l_r = \frac{3x_p^2}{2y_p}$                             \\
           &                                                                                                 & $x_q = x_p \land y_p = 0 \implies x_p = 0$                                                  \\\hline
    6      & $q_{(\cdot)} \cdot x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l_r^2 - x_p - x_q - x_r)$           & $x_p, x_q \neq 0 \land x_q \neq x_p \implies x_r = \l_r^2 - x_p - x_q$                       \\
    6      & $q_{(\cdot)} \cdot x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l_r \cdot (x_p - x_r) - y_p - y_r)$ & $x_p, x_q \neq 0 \land x_q \neq x_p \implies y_r = \l_r \cdot (x_p - x_q) - y_p$             \\
    6      & $q_{(\cdot)} \cdot x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l_r^2 - x_p - x_q - x_r)$           & $x_p, x_q \neq 0 \land y_q \neq y_p \implies x_r = \l_r^2 - x_p - x_q$                       \\
    6      & $q_{(\cdot)} \cdot x_p \cdot x_q \cdot (y_q - y_p) \cdot (\l_r \cdot (x_p - x_r) - y_p - y_r)$ & $x_p, x_q \neq 0 \land y_q \neq y_p \implies y_r = \l_r \cdot (x_p - x_q) - y_p$             \\\hline
    4      & $q_{(\cdot)} \cdot (1 - x_p \cdot \b_r) \cdot (x_r - x_q))$                                   & $x_p = 0 \implies x_r = x_q$                                                                 \\
    4      & $q_{(\cdot)} \cdot (1 - x_p \cdot \b_r) \cdot (y_r - y_q))$                                   & $x_p = 0 \implies y_r = y_q$                                                                 \\\hline
    4      & $q_{(\cdot)} \cdot (1 - x_q \cdot \g_q) \cdot (x_r - x_p))$                                   & $x_q = 0 \implies x_r = x_p$                                                                 \\
    4      & $q_{(\cdot)} \cdot (1 - x_q \cdot \g_q) \cdot (y_r - y_p))$                                   & $x_q = 0 \implies y_r = y_p$                                                                 \\\hline
    4      & $q_{(\cdot)} \cdot (1 - (x_q - x_p) \cdot \a_r - (y_q + y_p) \cdot \d_r) \cdot x_r)$          & $x_q = x_p \land y_q = -y_p \implies x_r = 0$                                                \\
    4      & $q_{(\cdot)} \cdot (1 - (x_q - x_p) \cdot \a_r - (y_q + y_p) \cdot \d_r) \cdot y_r)$          & $x_q = x_p \land y_q = -y_p \implies y_r = 0$                                                \\\hline
    3      & $q_{(\cdot)} \cdot b_i \cdot (b_i - 1)$                                                       & $b_i \in \Bb$                                                                                \\\hline
    3      & $q_{(\cdot)} \cdot (x_s - (b_i \cdot x_r + (1 - b_i) \cdot x_q))$                               & $x_s = \textbf{ if } b_i = 1 \textbf{ then } x_r \textbf{ else } x_q$                        \\
    3      & $q_{(\cdot)} \cdot (y_s - (b_i \cdot y_r + (1 - b_i) \cdot y_q))$                               & $y_s = \textbf{ if } b_i = 1 \textbf{ then } y_r \textbf{ else } y_q$                        \\\hline
    3      & $q_{(\cdot)} \cdot (acc_{i+1} - (acc_i + b_i \cdot 2^i))$                                       & $acc_{i+1} - (acc_i + b_i \cdot 2^i)$                                                        \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:scalar-mul-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$  & $w_2$  & $w_3$  & $w_4$ & $w_5$ & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$  \\\tabucline[1pt]{-}
    $x_a$  & $y_a$  & $acc$  & $x_p$ & $y_p$ & $x_q$  & $y_q$  & $x_r$  & $y_r$  & $b_i$    & $\g_q$   & $\l_q$   & $\a_r$   & $\b_r$   & $\d_r$   & $\l_r$    \\\hline
    $\bot$ & $\bot$ & $\bot$ & $I_2$ & $I_3$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$    \\\hline
  \end{tabu}
\end{center}

Each next $S$ is stored in the next row, so in the constraints, one would
define $x_s(X) = w_1(\o X), y_s = w_2(\o X), acc_{i+1} = w_3(\o X)$.

\begin{center}
  \captionof*{table}{Selector Row} \label{tab:scalar-mul-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $q_l$ & $q_r$ & $q_o$ & $q_m$ & $q_c$ & $q_H$ & $q_P$ & $q_{(+)}$ & $q_{(\cdot)}$ & $q_{(=)}$ & $q_{R}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0     & 0         & 1           & 0         & 0       \\\hline
  \end{tabu}
\end{center}

\begin{center}
  \captionof*{table}{Coefficient Row} \label{tab:scalar-mul-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $r_1$ & $r_2$ & $r_3$ & $r_4$ & $r_5$ & $r_6$ & $r_7$ & $r_8$ & $r_9$ & $r_{10}$ & $r_{11}$ & $r_{12}$ & $r_{13}$ & $r_{14}$ & $r_{15}$ \\\tabucline[1pt]{-}
    $2^i$ & 0     & 0     & 0     & 0     & 0       & 0     & 0   & 0     & 0        & 0        & 0        & 0        & 0        & 0        \\\hline
  \end{tabu}
\end{center}

Meaning that in the constraints $2^i = r_1$

**Last Row**

We need one last row since the next $A = \text{ if } b_i \meq 1 \text{ then }
R \text{ else } Q$ is stored in the next row in each iteration.

\begin{center}
  \captionof*{table}{Witness Row} \label{tab:scalar-mul-witness} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $w_1$  & $w_2$  & $w_3$  & $w_4$  & $w_5$  & $w_6$  & $w_7$  & $w_8$  & $w_9$  & $w_{10}$ & $w_{11}$ & $w_{12}$ & $w_{13}$ & $w_{14}$ & $w_{15}$ & $w_{16}$  \\\tabucline[1pt]{-}
    $x_a$  & $y_a$  & $acc$  & 0      & 0      & 0      & 0      & 0      & 0      & 0        & 0        & 0        & 0        & 0        & 0        & 0         \\\hline
    $O_1$  & $O_2$  & $I_1$  & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$ & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$   & $\bot$    \\\hline
  \end{tabu}
\end{center}

Note: Copy constraining $acc$ to input 1 ensures that $\sum_{i=0}^{254} b_i \cdot 2^i$.

Since this row is just to store the result and copy constrain $acc$, all
selector polynomials is set to zero.

\begin{center}
  \captionof*{table}{Coefficient Row} \label{tab:scalar-mul-selector} 
  \begin{tabu}{|c|c|c|c|c|c|c|c|c|c|c|c|c|c|c|}
    \hline
    $r_1$ & $r_2$ & $r_3$ & $r_4$ & $r_5$ & $r_6$ & $r_7$ & $r_8$ & $r_9$ & $r_{10}$ & $r_{11}$ & $r_{12}$ & $r_{13}$ & $r_{14}$ & $r_{15}$ \\\tabucline[1pt]{-}
    0     & 0     & 0     & 0     & 0     & 0       & 0     & 0   & 0     & 0        & 0        & 0        & 0        & 0        & 0        \\\hline
  \end{tabu}
\end{center}
