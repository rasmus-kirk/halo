---
title: Master Thesis Contract
author:
  - Rasmus Kirk Jakobsen - 201907084
  - Abdul Haliq Abdul Latiff - 202303466
geometry: margin=2cm
---

## Thesis Title
**Incrementally Verifiable Computation (IVC) Over Signatures Using Halo2**

## Objective
The primary focus of this thesis is to implement Incrementally Verifiable
Computation (IVC) using Halo2, a recursive proof system designed. Our
implementation will build upon existing work from previous projects covering
PLONK and accumulation schemes.

## Scope
1. **Implementation of IVC in Halo2**
   - Develop a fully functional implementation of Halo2 using Rust and Arkworks.
   - Use this implementation to achieve IVC.
   
2. **Technical Features and Challenges**
   - **Custom Gate Creation**: Develop specialized gates tailored for IVC within Halo2.
   - **Extending PLONK with Plookup**: Implement and optimize Plookup tables to enhance the verification process.
   
3. **Novel Work**
   - Explore the feasibility of implementing IVC over cryptographic signatures.
   - Investigate applications in succinct blockchains, particularly in systems utilizing the HotStuff consensus protocol.

## Expected Outcomes
- A complete and functional Halo2-based IVC implementation.
- A performance analysis comparing the feasibility of using IVC in blockchains.
- An exploration of potential use cases, particularly in the domain of blockchain scalability.

## Methodology
- **Programming Languages & Tools**: Rust and Arkworks.
- **Research & Development**: Iterative design, testing, and validation of cryptographic primitives.
- **Evaluation Metrics**: Performance benchmarks, security proofs, and benchmarking.

## Conclusion
The thesis aims to deepen our practical understanding of IVC by implementing
it using Halo2. We will explore the feasibility of applying IVC over
cryptographic signatures. By benchmarking this approach, we aim to assess its
viability for blockchain companies seeking to develop more secure and efficient
light clients.
