## Outline

We now define the $\Surkal$ protocol using the above arguments.

\begin{algorithm}[H]
\caption*{
  \textbf{Surkål:} The Ultra-\plonk-ish NARK protocol.
}
\textbf{Inputs} \\
  \Desc{$f: W[\tin{}] \to W[\tout{}]$}{NP problem / program.} \\
  \Desc{$\vec{x} \in W[\tin{}]$}{The possibly private input to the program $f$} \\
\textbf{Output} \\
  \Desc{$\Result(\top, \bot)$}{Either the verifier accepts with $\top$ or rejects with $\bot$}
\begin{algorithmic}[1]
  \State $(R: \Circuit, x: \PublicInputs, w : \Witness) = \mathrm{relation} \circ \mathrm{trace}(\mathrm{arithmetize}(f), \vec{x})$ 
  \State $\pi = \SurkalProver(R,x,w)$
  \State \textbf{return} $\SurkalVerifier(R,x,\pi)$
  \end{algorithmic}
\end{algorithm}

### $\SurkalProver$

- handwave describe notation in concrete protocol
- describe use of arguments
- construct polys for vanishing argument
  - F_GC
  - grand products: F_CC1, F_CC2, F_PL1, F_PL2
- fiat shamir transformation of vanishing argument

TODO

### $\SurkalVerifier$

TODO

