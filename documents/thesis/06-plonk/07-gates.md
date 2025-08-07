
### Defined Operations

TODO maybe move to appendix

TODO: all gates also does booleanity check, i.e. every (single output) gate has a flag to also assert if its bool

TODO assert WireType = {p,q} here onwards; bar notation $\bar{p} = q$

TODO: ctrn and loop too; term (in ctrn) includes $j$ lookup table index

| $g: \Ggt$                | $\text{eval}(g, \vec{x})$     | remarks                 |
|:-------------------------:|:-----------------------------:|:------------------------|
| Input$_i()$               | $(x_i)$                       | from trace              |
| Const$_{s,p}()$           | $(s)$                         |                         |
| Add$(x,y)$                | $(x+y)$                       |                         |
| Mul$(x,y)$                | $(x \times y)$                |                         |
| Inv$(x)$                  | $(x^{-1})$                    |                         |
| Pow7$(x)$                 | $(x^7)$                       |                         |
| If$(b,x,y)$               | $(b ? x : y)$                 |                         |
| Lookup$_T(x,y)$           | $\maybe{(z)}{(x,y,z) \in T}$  |                         |
| PtAdd$(x_P,y_P,x_Q,y_Q)$  | $(x_R, y_R)$                  | Arkworks point add      |
| Poseidon$(a,b,c)$         | $(a',b',c')$                  | Mina poseidon 5 rounds  |
| Public$(x)$               | $()$                          |                         |
| Bit$(b)$                  | $()$                          |                         |
| IsAdd$(x,y,z)$            | $()$                          |                         |
| IsMul$(x,y,z)$            | $()$                          |                         |
| IsLookup$_T(x,y,z)$       | $()$                          |                         |

TODO Concrete lookup table definitions here as well

**Trace Correctness Example**

TODO
