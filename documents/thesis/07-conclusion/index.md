# Conclusion

The core goal for this thesis was to implement, benchmark, analyze and
understand Incrementally Verifiable Computation in its entirety, using the
ideas put forward in the Halo paper[@halo]. We also wished to show whether an
IVC chain of signatures could be practically useful in the blockchain industry
with currently known recursive SNARK technology. The benchmarks show that
IVC may be a decent solution, but they do not definitively show that the IVC
solution is markedly better than the naive solution. Given that our results
indicate that it's viable using our simplified recursive SNARK, then it should
definitely be feasible for the more optimized Kimchi and Halo2 protocols.

There are plenty of remaining optimizations and improvements. This SNARK is not
quantum-safe, but if instantiated with FRI and a corresponding accumulation
scheme it should be able to be adapted with only minor modifications. Of
course, the omitted optimizations, like the Maller optimization, could be
added back for smaller proof sizes and a faster verifier. Zero-knowledge
might not be particularly useful for IVC, but adding it would be useful if
the Plonk construction is also used as a general-purpose ZK-SNARK. Lookups
can also be very useful, for certain operations like XOR, which would be
especially important if the SNARK should be able to model SHA3 efficiently. We
investigated Plonkup[@plonkup] for this, but ultimately deemed it unnecessary
to acheive primary goal of IVC.
