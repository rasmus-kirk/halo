## Full Plonk Protocol

Combining all the previously discussed subprotocols we get the full plonk
protocol. Soundness and completeness should follow from the subprotocols, so
we won't discuss those here, but we will do a short analysis and justification
for the worst-case runtimes of both algorithms.

But first, let's more precisely define the inputs to the protocols, the
circuit, witness and public inputs:

- PlonkCircuit:
  - $n \in \Fb$: The number of rows in the trace table, must be a power of two.
  - $d = n - 1$: The degree bound for all committed polynomials.
  - $\ell_1 \in \Fb$: The number of input passes, from _the other circuit_ to _this circuit_.
  - $\ell_2 \in \Fb$: The number of public inputs in the circuit.
  - $\o \in \Fb$: The base element for the set of roots of unity $H = \{ 1, \o, \o^2, \dots, \o^{n-1} \}$
  - Commitments:
    - $\vec{C_q} \in \Eb(\Fb)^{10}$: Commitments to the selector polynomials
    - $\vec{C_r} \in \Eb(\Fb)^{15}$: Commitments to the coefficient polynomials.
    - $\vec{C_{id}} \in \Eb(\Fb)^{4}$: Commitments to the identity polynomials.
    - $\vec{C_\s} \in \Eb(\Fb)^{4}$: Commitments to the $\sigma$ polynomials.

  All of these are static for the circuit, meaning that they do not depend on
  the prover's private input or public input.
- PlonkWitness:
  - $\vec{q^{(e)}} \in (\Fb^n)^{10}$
  - $\vec{r^{(e)}} \in (\Fb^n)^{15}$
  - $\vec{\id^{(e)}} \in (\Fb^n)^4$
  - $\vec{\s^{(e)}} \in (\Fb^n)^4$
  - $\vec{w^{(e)}} \in (\Fb^n)^{16}$

  Here, we use $e$ to denote that these are evaluations of the polynomials,
  which are computed by the arithmetizer. To get the actual polynomials,
  the prover runs $\ifft$ on each of these in round 0.
- PlonkPublicInputs:
  - $\vec{x} \in \Fb^{\ell_1 + 2 + \ell_2}$: The public inputs for the trace
    table, containing both the vanilla public inputs ($\ell_2$), the inputs
    passed from the other circuit ($\ell_1$) a commitment to the passed inputs
    which has size 2.
- PlonkProof:
  - Evaluation Proofs:
    - $\pi_s \in \EvalProof$
    - $\pi_{s_\o} \in \EvalProof$
  - Evaluations:
    - $\vec{q^{(\xi)}} \in \Fb^{10}$
    - $\vec{r^{(\xi)}} \in \Fb^{15}$
    - $\vec{\id^{(\xi)}} \in \Fb^4$
    - $\vec{\s^{(\xi)}} \in \Fb^4$
    - $\vec{w^{(\xi)}} \in \Fb^{16}$
    - $\vec{w^{(\xi\o)}} \in \Fb^{3}$
    - $\vec{t^{(\xi)}} \in \Fb^{16}$
    - $z^{(\xi)} \in \Fb$
    - $z^{(\xi\o)} \in \Fb$
  - Commitments:
    - $\vec{C_w} \in \Eb(\Fb)^{16}$
    - $\vec{C_t} \in \Eb(\Fb)^{16}$
    - $C_z \in \Eb(\Fb)$

  All of this PlonkProof is constant size, except the two evaluation proofs which
  have size $\Oc(\lg(n))$. Thus the Plonk proof size is also $\Oc(\lg(n))$.

### Prover

\begin{algorithm}[H]
\caption*{
  \textbf{Plonk Non-Interactive Prover:}
}
\textbf{Inputs:} PlonkCircuit, PlonkPublicInput, PlonkWitness \\
\textbf{Output:} PlonkProof
\begin{algorithmic}[1]
  \Statex \hspace*{-20px} \textbf{Round 0:}
    $$
    \begin{aligned}
      &\vec{q} := [\ifft(q^{(e)}_i)]^{10}_{i=1}, \quad
      \vec{r} := [\ifft(r^{(e)}_i)]^{15}_{i=1}, \quad
      \vec{\id} := [\ifft(\id^{(e)}_i)]^{4}_{i=1}, \quad
      \vec{\s} := [\ifft(\s_i^{(e)})]^{4}_{i=1}, \\
      &\vec{w} := [\ifft(w_i^{(e)})]^{16}_{i=1}, \quad
      x(X) := \ifft(-\vec{x})
    \end{aligned}
    $$

  \Statex \hspace*{-20px} \textbf{Round 1:}
    \State $\vec{C_w} := [\PCDLCommit(w_1, d, \bot)]^{16}_{i=1}$
    \State $\Tc \from \vec{C_w}$

  \Statex \hspace*{-20px} \textbf{Round 2:}
    \State $\Tc \to \b, \g$
    \State $f'(X) := \prod_{i = 1}^{4} w_i(X) + \beta \id_i(X) + \g$
    \State $g'(X) := \prod_{i = 1}^{4} w_i(X) + \beta \s_i(X) + \g$
    \State Define $z$:
      \Statex \algind $z(\o^1) := 1, \quad z(\o^i) := \prod_{1 \leq j < i} \frac{f'(\o^j)}{g'(\o^j)}$
    \State $C_z := \PCDLCommit(z, d, \bot)$
    \State $\Tc \from C_z$
  \Statex \hspace*{-20px} \textbf{Round 3:}
  \State $\Tc \to \a, \zeta$
  \State Define $f_{CG}(X)$ to be all the constraints listed in the custom constraint section, using $[1, \zeta, \zeta^2, \dots]$ as the challenges required for the custom constraints. Define $f(X), t(X)$:
    \Statex \algind $f_{GC}(X) := w_1(X) q_1(X) + w_2(X) w_2(X) + w_3(X) q_3(X) + w_1(X) w_2(X) q_4(X) + q_5(X) + x(X) + f_{CG}(X)$
    \Statex \algind $f_{CC_1}(X) := L_1(X) \cdot z(X) - 1$
    \Statex \algind $f_{CC_2}(X) := z(X) \cdot f'(X) - z(X \cdot \o) \cdot g'(X)$
    \Statex \algind $f(X) := f_{GC}(X) + \a f_{CC_1}(X) + \a^2 f_{CC_2}(X)$
    \Statex \algind $t(X) := f(X) / z_H(X)$
  \State Split $t(X)$ into $\vec{t} \in \Fb_{\leq d}^{16}$, s.t:
    \Statex \algind $t(X) = \sum^{16}_{i=1} (X^n)^{i-1} \cdot t_i(X)$
  \State Commit to each of them $\vec{C_t} := [\PCDLCommit(t_1(X), d, \bot)]^{16}_{i=1}$
  \State $\Tc \from \vec{C_t}$

  \Statex \hspace*{-20px} \textbf{Round 4:}
  \State $\Tc \from \eta$
  \State $s(X) = \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i(X) \quad \text{where} \quad \vec{\tau} = \vec{q} \cat \vec{w} \cat \vec{t} \cat [ z(X) ]$
  \State $s_\o(X) = \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i(X) \quad \text{where} \quad \vec{\tau} = [w_1(X), w_2(X), w_3(X), z(X)]$

  \Statex \hspace*{-20px} \textbf{Round 5:}
  \State $\Tc \to \xi$
  \State $C_s := \PCDLCommit(s(X), d, \bot), \quad C_{s_\o} = \PCDLCommit(s_\o(X), d, \bot)$
  \State $\pi_s = \PCDLOpen(s(X), C_s, d, \xi, \bot)$
  \State $\pi_{s_\o} = \PCDLOpen(s_\o(X), C_{s_\o}, d, \xi \cdot \o, \bot)$
    $$
    \begin{aligned}
      &\vec{q^{(\xi)}} = [q_i(\xi)]^{10}_{i=1}, \quad
      \vec{r^{(\xi)}} = [r_i(\xi)]^{15}_{i=1}, \quad
      \vec{\id^{(\xi)}} = [\id_i(\xi)]^{4}_{i=1}, \quad
      \vec{\s^{(\xi)}} = [\s_i(\xi)]^{4}_{i=1}, \\
      &\vec{w^{(\xi)}} = [w_i(\xi)]^{16}_{i=1}, \quad
      \vec{w^{(\xi\o)}} = [w_i(\xi \cdot \o)]^{3}_{i=1}, \quad
      \vec{t^{(\xi)}} = [t_i(\xi)]^{16}_{i=1}, \quad
    \end{aligned}
    $$
  \Return $\pi_s, \pi_{s_\o}, \vec{q^{(\xi)}}, \vec{r^{(\xi)}}, \vec{\id^{(\xi)}}, \vec{\s^{(\xi)}}, \vec{w^{(\xi)}}, \vec{w^{(\xi\o)}}, \vec{t^{(\xi)}}, z(\xi), z(\xi \cdot \o), \vec{C_w}, \vec{C_t}, C_z$
  \end{algorithmic}
\end{algorithm}

**Runtime:**

First note that _all_ polynomial multiplications can be modelled to run in
$\Oc(n \lg(n))$ because they can be modelled as $c(X) = a(X) \cdot b(X) =
\ifft(\fft(a(X)) \cdot \fft(b(X)))$. The runtime of multiplication over
the evaluation domain is $\Oc(n)$ and the runtime of the $\fft$ and $\ifft$
is $\Oc(n \lg(n))$. Polynomial addition is $\Oc(n)$. The $\PCDL$ functions
$\PCDLCommit, \PCDLOpen$ have a runtime of $\Oc(n)$. Therefore the worst-case
runtime of the prover is $\Oc(n \lg(n))$.

### Verifier

\begin{algorithm}[H]
\caption*{
  \textbf{Plonk Non-Interactive Verifier:}
}
\textbf{Inputs:} PlonkCircuit, PlonkPublicInput, PlonkProof \\
\textbf{Output:} $\Result(\top, \bot)$
\begin{algorithmic}[1]
  \Statex \hspace*{-20px} \textbf{Round 1:}
  \State $\Tc \from \vec{C_w}$

  \Statex \hspace*{-20px} \textbf{Round 2:}
  \State $\Tc \to \beta, \gamma$
  \State $\Tc \from C_z$


  \Statex \hspace*{-20px} \textbf{Round 3:}
  \State $\Tc \to \alpha, \zeta$
  \State $\Tc \from \vec{C_t}$

  \Statex \hspace*{-20px} \textbf{Round 4:}
  \State $\Tc \to \eta$
  \State $\Tc \from C_s, C_{s_\o}$

  \Statex \hspace*{-20px} \textbf{Round 5:}
  \State $\Tc \to \xi$
  \State Compute:
    \Statex \algind $\xi^n$ using iterative squaring, since $n$ is a power of 2 ($\lg(n)$ multiplications).
    \Statex \algind $L_1(\xi) = \frac{\o \cdot (\xi^n - 1)}{n \cdot (\xi - \o)}$
    \Statex \algind $z_H(\xi) = \xi^n - 1$
    \Statex \algind $x(\xi) = \sum_{i=1}^{\ell_1 + 2 + \ell_2} L_i(\xi) \cdot (-x_i) \quad \text{where} \quad L_i(\xi) = \frac{\o^i \cdot (\xi^n - 1)}{n \cdot (\xi - \o^i)}$
  \State Define $f_{CG}(\xi)$ to be all the constraints listed in the custom constraint
  section, using $[1, \zeta, \zeta^2, \dots]$ as the challenges required for the
  custom constraints, and the evaluations ($\vec{q^{(v)}}, \vec{r^{(v)}},
  \vec{w^{(v)}}, \vec{w_{\o}^{(v)}}$) provided by the prover. Then, define $f(\xi), t(\xi)$:
    \Statex \algind $f'(\xi) = \prod_{i = 1}^4 w^{(v)}_i + \b \cdot \id^{(v)}_i + \g$
    \Statex \algind $g'(\xi) = \prod_{i = 1}^4 w^{(v)}_i + \b \cdot \s^{(v)}_i + \g$
    \Statex \algind $f_{GC}(\xi) = w^{(v)}_1 q^{(v)}_1 + w^{(v)}_2 q^{(v)}_2 + w^{(v)}_3 q^{(v)}_3 + w^{(v)}_1 w^{(v)}_2 q^{(v)}_4 + q^{(v)}_5 + x(\xi) + f_{CG}(\xi)$
    \Statex \algind $f_{CC_1}(\xi) = L_1(\xi) \cdot (z^{(v)} - 1)$
    \Statex \algind $f_{CC_2}(\xi) = z^{(v)} \cdot f'(\xi) - z_{\o}^{(v)} \cdot g'(\xi)$
    \Statex \algind $f(\xi) = f_{GC}(\xi) + \a \cdot f_{CC_1}(\xi) + \a^2 \cdot f_{CC_2}(\xi)$
    \Statex \algind $t(\xi) = \sum_{i=1}^{16} (\xi^n)^{i-1} \cdot t^{(v)}_i$
  \State Compute the evaluations and commitments of $s(X), s_\o(X)$:
    \Statex \algind $s(\xi) := \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i \quad \text{where} \quad \vec{\tau} = \vec{q^{(v)}} \cat \vec{w^{(v)}} \cat \vec{t^{(v)}} \cat [ z(\xi) ]$
    \Statex \algind $s_{\o}(\xi) := \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i \quad \text{where} \quad \vec{\tau} = [w_1(\xi \cdot \o), w_2(\xi \cdot \o), w_3(\xi \cdot \o), z(\xi \cdot \o)]$
    \Statex \algind $C_s := \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i \quad \text{where} \quad \vec{\tau} = \vec{C_q} \cat \vec{C_w} \cat \vec{C_t} \cat [ C_z ]$
    \Statex \algind $C_{s_\o} := \sum_{i=1}^{| \vec{\tau} |} \eta^{i-1} \cdot \tau_i \quad \text{where} \quad \vec{\tau} = [C_{w_1}, C_{w_2}, C_{w_3}, C_z]$
  \State Compute the commitment to the passed inputs:
    \Statex $C_{\text{IP}} := \sum_{i=1}^{\ell_1} x_i \cdot G_i$
    \Statex $C_{\text{IP}}' := (x_{\ell_1 + 1}, x_{\ell_1 + 2})$

  \Statex \hspace*{-20px} \textbf{Checks:}
  \State $C_{\text{IP}} \meq C_{\text{IP}}'$
  \State $f(\xi) \meq t(\xi) \cdot z_H(\xi)$
  \State $\PCDLCheck(C_s, d, \xi, s(\xi), \pi_s)$
  \State $\PCDLCheck(C_{s_\o}, d, \xi \cdot \o, s_\o(\xi), \pi_{s_\o})$
\end{algorithmic}
\end{algorithm}

**Runtime:**

- $\xi^n$ is $\Oc(\lg(n))$ multiplications, due to iterative squaring.
- $x(\xi)$ is $\Oc(\ell)$
- The computation of $f(\xi), t(\xi)$ doesn't depend on $n$, so that part is constant. Same is true for $s(X), s_\o(X)$.
- The computation of $C_{\text{IP}}$ is $\ell_1$ scalar multiplications.
- All transcript hash interactions are also constant.

Overall, the worst-case runtime with fixed $\ell_1, \ell_2$ and variable $n$
is $\Oc(\lg(n))$.
