## Security of Elliptic Curve Addition Constraints

**Analysis**

1. $q_{(+)} \cdot (x_q - x_p) \cdot ((x_q - x_p) \cdot \l - (y_q - y_p)) = 0$  

   Which ensures:  
   $x_q \neq x_p \implies \l = \frac{y_q - y_p}{x_q - x_p}$  
   $(P \neq Q \land P \neq -Q) \implies \l = \frac{y_q - y_p}{x_q - x_p}$

2. $q_{(+)} \cdot (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = 0$

   Meaning that $x_q = x_p \implies \l = \frac{3x_p^2}{2y_p}$, except if
   $y_p = 0$, then:

   $0 = (1 - (x_q - x_p) \cdot \a) \cdot (2y_p \cdot \l - 3x_p^2) = (2y_p \cdot \l - 3x_p^2) = -3x_p^2$

   Which is only satisfied if $x_p = 0$. So this means that the constraint
   ensures:  

   $x_q = x_p \land y_p \neq 0 \implies \l = \frac{3x_p^2}{2y_p}$  
   $x_q = x_p \land y_p = 0 \implies x_p = 0$  
   $(P = Q \lor Q = -P) \land P \neq \Oc \land Q \neq \Oc \implies \l = \frac{3x_p^2}{2y_p}$

3.
  a. $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  b. $q_{(+)} \cdot (x_p \cdot x_q \cdot (x_q - x_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$
  c. $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q + y_p) \cdot (\l^2 - x_p - x_q - x_r) = 0$
  d. $q_{(+)} \cdot (x_p \cdot x_q \cdot (y_q + y_p) \cdot (\l \cdot (x_p - x_r) - y_p - y_r) = 0$

  It's clear that if $(x_p \cdot x_q \cdot (x_q - x_p) \neq 0 \implies x_p
  \neq 0 \land x_q \neq 0 \land x_q \neq x_p$. So 3.a states:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies x_r = \l^2 - x_p - x_q$.

  Constraint 3.b, 3.c, 3.d have similar meaning. Combining 3.a, 3.b, 3.c, 3.d yields:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  
  $x_p \neq 0 \land x_q \neq 0 \land y_q \neq -y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$

  Or equivalently:

  $x_p \neq 0 \land x_q \neq 0 \land x_q \neq x_p \land y_q \neq -y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  

  From the curve we know that any point where $x$ or $y$ is 0, is invalid,
  except if it's the identity point. We can also combine the two implications:

  $x_p \neq 0 \land y_p \neq 0 \land x_q \neq 0 \land y_q \neq 0 \land x_q \neq x_p \land y_q \neq -y_p \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$  

  Which simplifies to:

  $P \neq \Oc \land Q \neq \Oc \land -P \neq Q \implies x_r = \l^2 - x_p - x_q \land y_r = \l \cdot (x_p - x_r) - y_p$
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
  $Q = -P \implies R = \Oc$

**Security:**

Analysing the cases of $P + Q = R$:

- $\Oc + \Oc = \Oc$
  - Completeness:
    1. Holds because $P = Q$
    2. Holds because $P = Q = \Oc$
    3. Holds because $P = Q = \Oc$
    4. Holds because $P = \Oc \land R = Q = \Oc$
    5. Holds because $Q = \Oc \land R = P = \Oc$
    6. Holds because $Q = -P = \Oc \land R = \Oc$
  - Soundness: $R = \Oc$ is the only solution to 6.
- $P + \Oc = P : P \neq \Oc$
  - Completeness:
    1. $P \neq Q$, so $\l = \frac{y_q - y_p}{x_q - x_p}$ is a solution
    2. Holds because $Q = \Oc$
    3. Holds because $Q = \Oc$
    4. Holds because $P \neq \Oc$
    5. Holds because $Q = \Oc \land R = P$
    6. Holds because $Q \neq -P$
  - Soundness: $R = \Oc$ is the only solution to 5.
- $\Oc + Q = Q : Q \neq \Oc$
  - Completeness:
    1. $P \neq Q$, so $\l = \frac{y_q - y_p}{x_q - x_p}$ is a solution
    2. Holds because $P = \Oc$
    3. Holds because $P = \Oc$
    4. Holds because $P = \Oc \land R = Q$
    5. Holds because $Q \neq \Oc$
    6. Holds because $Q \neq -P$
  - Soundness: $R = \Oc$ is the only solution to 4.
- $P + Q = 2P : P = Q \neq \Oc$
  - Completeness:
    1. Holds because $P = Q$
    2. $P = Q$, so $\l = \frac{2x_p^2}{2y_p}$ is a solution
    3. Holds because $P \neq \Oc \land Q \neq 0 \land Q \neq -P$, so $R =
       (x_r = \l^2 - x_p - x_q, y_r = \l \cdot (x_p - x_r) - y_p)$. Which
       is consistent with point doubling, since $x_p = x_q$ and $\l =
       \frac{2x_p^2}{2y_p}$.
    4. Holds because $P \neq \Oc$
    5. Holds because $Q \neq \Oc$
    6. Holds because $Q \neq -P$
  - Soundness: $\l$ is computed correctly (2). $R = 2P$ is the only solution
    to 3.
- $P + Q = \Oc : P = -Q \neq \Oc$
  - Completeness:
    1. Holds because $P = -Q$
    2. $P = Q$, so $\l = \frac{2x_p^2}{2y_p}$ is a solution
    3. Holds because $-P = Q$
    4. Holds because $P \neq \Oc$
    5. Holds because $Q \neq \Oc$
    6. Holds because $Q = -P \land R = \Oc$
  - Soundness: $R = \Oc$ is the only solution to 6.
- $P + Q = \Oc : P \neq -Q, P \neq Q, P \neq \Oc, Q \neq \Oc$
  - Completeness:
    1. $P \neq Q \land P \neq -Q$, so $\l = \frac{y_q - y_p}{x_q - x_p}$ is a valid solution
    2. Holds because $P \neq Q \land Q \neq -P$
    3. Holds because $P \neq -Q$, so $R =
       (x_r = \l^2 - x_p - x_q, y_r = \l \cdot (x_p - x_r) - y_p)$. Which
       is consistent with affine point addition since $\l = \frac{y_q -
       y_p}{x_q - x_p}$.
    4. Holds because $P \neq \Oc$
    5. Holds because $Q \neq \Oc$
    6. Holds because $Q \neq -P$
  - Soundness: $\l$ is computed correctly (2). $R = P + Q$ is the only solution

