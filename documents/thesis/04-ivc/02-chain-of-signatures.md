## Chain of Signatures

Blockchains with consensuses based on HotStuff[@hotstuff], such as Concordium
and Partisia, have elected committees sign blocks. The highest block signed
by the current committee is deemed the latest block. During catch-up, when a
node has to sync with the blockchain, it has to download all previous blocks
from the first block, the genesis block. This is also the case for light nodes
that require less resources, with slightly inferior security guarantees. We
want to enable near-instant catchup for light clients in blockchains based
on HotStuff with only minimal security slackening compared to traditional
light client catchup.

We specify a recursive SNARK construction and instantiate it over a chain of
signatures, which would allow safe catchup for light clients on blockchains
based on the HotStuff consensus. Taking Concordium as the main example; they
elect a committee once a day and that committee is responsible for signing
valid blocks. Concordium is a proof of stake blockchain so the committee is
elected according to the size of thier staked tokens. They could create a
parallel _IVC blockchain_, one where each block contains:
$$B_i = \{ \s^{(pk)}_i, j_i = i, pk_i, ptr_i \in \Bb^{32}, \s^{(ptr)}_i \}$$

- $\s^{(pk)}_i$: A signature on the public key of the current committee
  ($pk_i$), signed by the previous committee identified by the public key
  $pk_{i-1}$.
- $j_i$: A sequential block-id. This must be present for the soundness of
  the IVC circuit.
- $pk_i$: The public key of the current committee.
- $ptr_i$: A hash of the most recent block on the main blockchain.
- $\s^{(ptr)}_i$: A signature on $ptr_i$, signed by the current public key.

Traditionally, a blockchain would need the hash of the previous block to
tie together blocks. We can omit that since we already have the signature
$\s_i^{(pk)}$, linking together $pk_{i-1}$ and $pk_i$, thus also linking
together $B_{i-1}$ and $B_i$. To verify this IVC blockchain, one would need
all blocks from the genesis block $B_0$, until the most recent block $B_n$.
Then they may verify the relation:
$$\text{Verify}_{pk_{i-1}}(\s^{(pk)}, pk_i) \land \text{Verify}_{pk_i}(\s^{(ptr)}, ptr_i) \land j_i \meq j_{i-1} + 1$$
Now we have a chain of signatures from the first genesis committee, all the
way to the final committee at block $n$. Assuming that the first committee
is honest, it should only sign the next honestly elected committee, which
by the security of the blockchain should also be majority-honest. That
committee will then also only sign the next honest committee. We can continue
this argument until reaching committee $n$, which contains a pointer to the
most recent block on the main blockchain. We can now trust that block, and
trust the blockchain, given that we trust the genesis committee, and that
the subsequent committees have been honest.

This is of course not much of an improvement, to catch up on the main
blockchain you need to catch up on some _other_ blockchain. The second
blockchain is however constructed to be SNARK-friendly. There is only a
single public key, representing each committee, the signature scheme can be
Schnorr's using poseidon for the hashing of messages, which works well in
SNARK constructions.  Importantly, this secondary blockchain can use Poseidon
hashes, while the main blockchain may prefer Sha3 for security benefits,
and the secondary blockchain may use Schnorr signatures, while the main
blockchain doesn't have to change their signature scheme.

The main committee still needs to generate and sign using the Schnorr
signature scheme, but for this they can use a multisignature scheme like
FROST[@frost]. In the next section we define the IVC scheme that's able to
support this.
