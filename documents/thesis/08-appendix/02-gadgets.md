## Gadgets

**Draft Note:** I still haven't decided whether to expand this with Schnorr
Verifier, $\PCDLSuccinctCheck$, $\ASDLVerifier$, or whether it just bloats
the report. The nice thing about having it is that it actually tells you how
exactly the IVC-circuit is defined, which I feel is pretty important. The
$\PlonkVerifier$ can be omitted since it will be the same as in the plonk
section. Maybe appendix?

### Poseidon Sponges

The Poseidon State can be one of the following values:

$$
  \textbf{SpongeState} = \begin{cases}
    \texttt{Absorbed}(0) \\
    \texttt{Absorbed}(1) \\
    \texttt{Absorbed}(2) \\
    \texttt{Squeezed}(1) \\
    \texttt{Squeezed}(2) \\
  \end{cases}
$$
The $\textbf{SpongeState}$ shouldn't be part of the circuit, it just governs
what when the full poseidon gates should be added to the circuit, i.e. when
enough values has been absorbed.

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Absorb Gadget:} Absorbs a list of field elements into the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{
    The current state condition of the sponge.
  } \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}$}{The field elements that the sponge should absorb.} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb^3)$}{
    The sponge state condition and inner state after absorption.
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
    \If{$\text{sponge\_state} = \texttt{Absorbed}(n) \land n < 2$}
      \State $\text{sponge\_state} = \texttt{Absorbed}(n + 1)$
      \State $s_n = x$
    \ElsIf{$\text{sponge\_state} = \texttt{Absorbed}(2)$}
      \For{$i \in [0, 10]$} \Comment{Permute 55 times by using the Hades Gate 11 times}
        \State $\vec{s} = PoseidonBlockCipher(i, c, \vec{s})$
      \EndFor
      \State $\text{sponge\_state} = \texttt{Absorbed}(1)$
      \State $s_0 = s_0 + x$
    \Else
      \State $\text{sponge\_state} = \texttt{Absorbed}(1)$
      \State $s_0 = s_0 + x$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Squeeze Gadget:} Squeezes a field element from the the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(c, s, x): (\textbf{SpongeState}, \Fb^3, \Fb)$}{
    The sponge state condition and inner state after absorption and the squeezed element
  }
\begin{algorithmic}[1]
  \If{$\text{sponge\_state} = \texttt{Squeezed}(n) \land n < 2$}
    \State $\text{sponge\_state} = \texttt{Squeezed}(n + 1)$
    \State \textbf{Return} $x = s_n$
  \Else
    \For{$i \in [0, 10]$} \Comment{Permute 55 times by using the Hades Gate 11 times}
      \State $\vec{s} = \text{HadesGate}_i(c, \vec{s})$
    \EndFor
    \State $\text{sponge\_state} = \texttt{Squeezed}(1)$
    \State \textbf{Return} $x = s_0$
  \EndIf
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Affine Gadget:} Absorbs affine points into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{Ps}$}{The affine points to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb^3, \Fb)$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$P$ in $\vec{Ps}$}
    \If{$P \meq \Oc$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, 0)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, 0)$
    \Else
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, P.x)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, P.y)$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Field Element Gadget:} Absorbs field elements into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}: \Fb_{BF}$}{The field elements to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{BF})$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, x)$
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Scalar Gadget:} Absorbs scalars into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb_{BF}^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs} \in (\Fb_{S})$}{The scalars to absorb} \\
\textbf{Output} \\
  \Desc{$(c, s): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{BF})$}{
    The sponge state condition and inner state after absorption
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
    \If{$|\text{Scalar-Field}| < |\text{Base-Field}|$}
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, x)$
    \Else
      \State Decompose $x$ into $h, l$ where $h$ represents the high-bits of $x$ and $l$ represents the low-bit.
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, h)$
      \State $\text{InnerAbsorb}(\text{sponge\_state}, \vec{s}, l)$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Squeeze Scalar Gadget:} Squeezes a scalar from the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\text{sponge\_state}: \textbf{SpongeState}$}{The current state condition of the sponge.} \\
  \Desc{$\vec{s}: \Fb_{BF}^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(c, s, x): (\textbf{SpongeState}, \Fb_{BF}^3, \Fb_{S})$}{
    The sponge state condition and inner state after squeezing and the squeezed scalar. 
  }
\begin{algorithmic}[1]
    \If{$x < |\text{Base-Field}|$}
      \State $\text{InnerSqueeze}(\text{sponge\_state}, \vec{x}, x)$
    \Else
      \State $\text{InnerSqueeze}(\text{sponge\_state}, \vec{s}, 0)$
    \EndIf
\end{algorithmic}
\end{algorithm}

This gadget returns zero for values that are too large. This means that
there is a bias for the value zero (in one of the curves). An attacker
could try to target that seed, in order to predict the challenges produced
by the weaker sponge. This would allow the attacker to mess with the result
of the proofs. Previously the attacker's odds were $\frac{1}{q}$, now it's
$\frac{q-p}{q}$. Since $lg(q-p) \approx 86$ and $lg(q) \approx 254$ the odds
of a successful attack are still negligible, but we do lose some security.

$$\frac{q - p}{q} \approx \frac{2^{86}}{2^{254}} = 2^{86 - 254} = 2^{-168}$$

