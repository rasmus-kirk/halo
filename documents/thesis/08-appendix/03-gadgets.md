## Gadgets

We present the gadgets needed to create the IVC circuit for completeness. We omit the $\PlonkVerifier$ as it's exactly identical to the one defined in the Plonk section, but with verification failure and success modelled with Booleans.

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
enough values has been absorbed and can thus be modelled outside the circuit.

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Absorb Gadget:} Absorbs a list of field elements into the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}$}{The field elements that the sponge should absorb.} \\
\textbf{Output} \\
  \Desc{$\vec{s}: \Fb^3$}{
    The sponge inner state after absorption.
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
  \State \Return $\vec{s}$
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Inner Sponge Squeeze Gadget:} Squeezes a field element from the the poseidon sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(s, x): (\Fb^3, \Fb)$}{
    The sponge inner state after absorption and the squeezed element.
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
    \State \Return $(\vec{s}, x = s_0)$
  \EndIf
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Affine Gadget:} Absorbs affine points into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{Ps}: \Eb(\Fb_\Bc)^k$}{The affine points to absorb} \\
\textbf{Output} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{
    The sponge inner state after absorption.
  }
\begin{algorithmic}[1]
  \For{$P$ in $\vec{Ps}$}
    \If{$P \meq \Oc$}
      \State $\text{InnerAbsorb}(\vec{s}, 0)$
      \State $\text{InnerAbsorb}(\vec{s}, 0)$
    \Else
      \State $\text{InnerAbsorb}(\vec{s}, P.x)$
      \State $\text{InnerAbsorb}(\vec{s}, P.y)$
    \EndIf
  \EndFor
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Field Element Gadget:} Absorbs field elements into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs}: \Fb^k_\Sc$}{The field elements to absorb} \\
\textbf{Output} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{
    The sponge inner state after absorption.
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
      \State $\text{InnerAbsorb}(\vec{s}, x)$
  \EndFor
  \State \Return $s$
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Absorb Scalar Gadget:} Absorbs scalars into the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{The inner state of the sponge (3 field elements).} \\
  \Desc{$\vec{xs} \in \Fb_\Sc^k$}{The scalars to absorb} \\
\textbf{Output} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{
    The sponge inner state after absorption.
  }
\begin{algorithmic}[1]
  \For{$x$ in $\vec{xs}$}
    \State Input pass $x$.
    \If{$|\text{Scalar-Field}| < |\text{Base-Field}|$}
      \State $\text{InnerAbsorb}(\vec{s}, x)$
    \Else
      \State Decompose $x$ into $h, l$ where $h$ represents the high-bits of $x$ and $l$ represents the low-bit.
      \State $\text{InnerAbsorb}(\vec{s}, h)$
      \State $\text{InnerAbsorb}(\vec{s}, l)$
    \EndIf
  \EndFor
  \State \Return $\vec{s}$
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption*{
  \textbf{Outer Sponge Squeeze Scalar Gadget:} Squeezes a scalar from the inner sponge.
}
\textbf{Inputs} \\
  \Desc{$\vec{s}: \Fb_\Bc^3$}{The inner state of the sponge (3 field elements).} \\
\textbf{Output} \\
  \Desc{$(s, x): (\Fb_\Bc^3, \Fb_\Sc)$}{
        The sponge inner state after squeezing and the squeezed scalar. 
  }
\begin{algorithmic}[1]
    \State $x = \text{InnerSqueeze}(\vec{s})$
    \If{$x < |\text{Base-Field}|$}
      \State \Return $\vec{s}$ and $x$ input passed.
    \Else
      \State \Return $\vec{s}$ and $h$ input passed, where $h$ is the high 254 bits of $x$. 
    \EndIf
\end{algorithmic}
\end{algorithm}

We lose a single bit of security if $x \geq |\text{Base-Field}|$, but this
only increases the odds of an attack by a small constant amount, which is still
negligible.

### $\PCDL$

The below algorithm is the non-ZK version of the algorithm specified in the
previous accumulation scheme project[@halo-accumulation].

\begin{algorithm}[H]
\caption{$\PCDLSuccinctCheck^{\rho_0}$}
\textbf{Inputs} \\
  \Desc{$C: \Eb(\Fb_\Bc)$}{A commitment to the coefficients of $p$.} \\
  \Desc{$d: \Nb$}{A degree bound on $p$.} \\
  \Desc{$z: \Fb_\Sc$}{The element that $p$ is evaluated on.} \\
  \Desc{$v: \Fb_\Sc$}{The claimed element $v = p(z)$.} \\
  \Desc{$\pi: \EvalProof$}{The evaluation proof produced by $\PCDLOpen$.} \\
\textbf{Output} \\
  \Desc{$(\Bb, (\Fb_{\Sc,d}[X], \Eb(\Fb_\Bc)))$}{
    The algorithm will either succeed and output $(\top, (h: \Fb_{\Sc,d}[X],
    U: \Eb(\Fb_\Bc))$ if $\pi$ is a valid proof and otherwise fail $(\bot,
    (h: \Fb_{\Sc,d}[X], U: \Eb(\Fb_\Bc))$.
  }
\begin{algorithmic}[1]
  \Require $d \leq D$
  \Require $(d+1)$ is a power of 2.
  \State Parse $\pi$ as $(\vec{L},\vec{R}, U := G^{(0)}, c := c^{(0)})$ and let $n = d + 1$.
  \State Compute the 0-th challenge: $\xi_0 := \rho_0(C, z, v)$, and set $H' := \xi_0 H \in \Eb(\Fb_\Bc)$.
  \State Compute the group element $C_0 := C + vH' \in \Eb(\Fb_\Bc)$.
  \For{$i \in [\lg(n)]$}
    \State Generate the i-th challenge: $\xi_i := \rho_0(\xi_{i-1}, L_i, R_i) \in \Fb_\Sc$.
    \State Compute the i-th commitment: $C_i := \xi^{-1}_i L_i + C_{i-1} + \xi_i R_i \in \Eb(\Fb_\Bc)$.
  \EndFor
\State Define the univariate polynomial $h(X) := \prod^{\lg(n)-1}_{i=0} (1 + \xi_{\lg(n) - i} X^{2^i}) \in \Fb_{\Sc,d}[X]$.
\State Compute the evaluation $v' := c \cdot h(z) \in \Fb_\Sc$.
\State $b = C_{lg(n)} \meq cU + v'H'$
\State Output $(b, (h(X), U))$.
\end{algorithmic}
\end{algorithm}

### $\ASDL$

The below algorithms are the non-ZK versions of the algorithms specified in the
previous accumulation scheme project[@halo-accumulation].

\begin{algorithm}[H]
\caption{$\ASDLCommonSubroutine$}
\textbf{Inputs} \\
  \Desc{$\vec{q}: \Instance^m$}{New instances \textit{and accumulators} to be accumulated.} \\
\textbf{Output} \\
  \Desc{$(\Bb, (\Eb(\Fb_\Bc), \Nb, \Fb_\Sc, \Fb_{\Sc,d}[X]))$}{
    The algorithm will either succeed $(\top, (\Eb(\Fb_\Bc), \Nb, \Fb, \Fb_{\Sc,d}[X]))$
    if the instances has consistent degree and hiding parameters and will
    otherwise fail ($\bot, (\Eb(\Fb_\Bc), \Nb, \Fb, \Fb_{\Sc,d}[X])$).
  }
\begin{algorithmic}[1]
  \Require $(D+1) = 2^k$, where $k \in \Nb$
  \State Parse $d$ from $q_1$.
  \State Let $b = \top$
  \For{$j \in [0, m]$}
    \State Parse $q_j$ as a tuple $(C_j, d_j, z_j, v_j, \pi_j)$.
    \State Compute $(b_i, (h_j(X), U_j)) := \PCDLSuccinctCheck^{\rho_0}(C_j, d_j, z_j, v_j, \pi_j)$.
    \State $b = b \land b_i$
    \State Check that $b_i = d_j \meq d$
    \State $b = b \land b_i$
  \EndFor
  \State Compute the challenge $\a := \rho_1(\vec{h}, \vec{U}) \in \Fb_\Sc$
  \State Let the polynomial $h(X) := \sum^m_{j=1} \a^j h_j(X) \in \Fb_{\Sc,d}[X]$
  \State Compute the accumulated commitment $C := \sum^m_{j=1} \a^j U_j$
  \State Compute the challenge $z := \rho_1(C, h(X)) \in \Fb_\Sc$.
  \State Randomize $C$: $\bar{C} := C \in \Eb(\Fb_\Bc)$.
  \State Output $(b, (\bar{C}, D, z, h(X)))$.
\end{algorithmic}
\end{algorithm}

\begin{algorithm}[H]
\caption{$\ASDLVerifier$}
\textbf{Inputs} \\
  \Desc{$\vec{q}: \Instance^m$}{New instances \textit{and possible accumulator} to be accumulated.} \\
  \Desc{$\acc_i: \Acc$}{The accumulator that accumulates $\vec{q}$. \textit{Not} the previous accumulator $\acc_{i-1}$.} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{
    The algorithm will either succeed $(\top)$ if $\acc_i$ correctly accumulates
    $\vec{q}$ and otherwise fail ($\bot$).
  }
  \begin{algorithmic}[1]
  \Require $(D+1) = 2^k$, where $k \in \Nb$ 
    \State Parse $\acc_i$ as $(\bar{C}, d, z, v, \_)$
    \State The accumulation verifier computes $(b_v, (\bar{C}', d', z', h(X))) := \ASDLCommonSubroutine(\vec{q})$
    \State $b_{(=)} = \bar{C}' \meq \bar{C} \land d' \meq d \land z' \meq z \land h(z) \meq v$.
    \State \Return $b_{(=)} \land b_v$
\end{algorithmic}
\end{algorithm}

### $\ASDL$

### Schnorr Signatures

\begin{algorithm}[H]
\caption{$\SchnorrVerifier$}
\textbf{Inputs} \\
  \Desc{$pk: \Eb(\Fb_\Bc)$}{The public key.} \\
  \Desc{$\s: \textbf{Signature}$}{The signature.} \\
  \Desc{$m: \Fb_\Bc^k$}{The signed message.} \\
\textbf{Output} \\
  \Desc{$\Bb_\Bc$}{
    A Boolean representing whether the verification succeeded.
  }
\begin{algorithmic}[1]
  \State Parse $\s$ as $(s, r)$
  \State $e = \Hc(pk, r, m)$.
  \State \Return $s G \meq R + e P$.
\end{algorithmic}
\end{algorithm}
