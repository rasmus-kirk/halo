# Conclusion

The core goal for this thesis was to implement, benchmark, analyze and
understand Incrementally Verifiable Computation in its entirety, using the
ideas put forward in the Halo paper[@halo]. We also wished to show whether
an IVC chain of signatures could be practically useful in the blockchain
industry with currently known recursive SNARK technology, which the benchmarks
support. Given that our results indicate that it's viable using our simplified
recursive SNARK, then it should definitely be doable for the more optimized
Kimchi and Halo2 protocols.

There are plenty of remaining answers and work to be done. This SNARK is not
quantum-safe, but if instantiated with FRI and a corresponding accumulation
scheme it should be able to be adapted with only minor modifications. Of
course, the omitted optimizations, like the Maller's optimization, could be
added back for smaller proof sizes and a faster verifier. Our protocol is also
_not_ zero-knowledge, which is an obvious feature to add to the SNARK. Lookups
can also be very useful, for certain operations like XOR. We investigated
Plookup for this, but ultimately deemed it unnecessary to acheive IVC.
