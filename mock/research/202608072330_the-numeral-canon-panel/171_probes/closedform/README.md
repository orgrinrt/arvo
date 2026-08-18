# P5: R-10 answered with the mechanism rather than the count

```sh
python3 closedform.py    # first hypothesis, REFUTED by its own control
python3 closedform2.py   # locate the determining modulus
python3 closedform3.py   # the formula, tested across widths
```

`169` measured `167` 4.1's `M = 2F-1` column as exactly `2^(F-1)` at F = 4, 6, 8, 10 and called it a
closed form. R-10 asks me to put it in the table. Before doing that I wanted the characterisation,
because a count matching a formula at four points is still a trend.

## Two wrong hypotheses, both caught, both kept

**P5** guessed `a*b mod 2^F == 2^(F-1) - 1`. The **count matched exactly at every width and the set did
not**, which is the most dangerous near-miss available: a wrong characterisation with the right
cardinality. Kept in `closedform.out` rather than repaired away.

**P5b** then located the determining modulus and its cross-width check failed, because its residue
scaling rule was ad hoc. The characterisation was right and the scaling was wrong.

## The result

```
H:  a*b mod 2^(F+1)  in  { 2^(F-1) + 1 ,  3*2^(F-1) - 1 }
```

| F | disagreeing | 2^(F-1) | H exact | r1 alone | r2 alone | residues shifted by 2 |
|---|---|---|---|---|---|---|
| 4 | 8 | 8 | **yes** | no | no | no |
| 5 | 16 | 16 | **yes** | no | no | no |
| 6 | 32 | 32 | **yes** | no | no | no |
| 7 | 64 | 64 | **yes** | no | no | no |
| 8 | 128 | 128 | **yes** | no | no | no |
| 9 | 256 | 256 | **yes** | no | no | no |
| 10 | 512 | 512 | **yes** | no | no | no |

C-R, C-S and C-T all clean: H is exact at every width, neither residue alone suffices, and shifting
the residues by two breaks it.

**The reading.** `2^(F-1)` and `3*2^(F-1)` are the two odd multiples of `2^(F-1)` modulo `2^(F+1)`,
which are the tie points of the F-level rounding. The disagreeing products sit exactly one unit from a
tie, on the side the single discarded bit rounds **onto** it, where ties-to-even then breaks the other
way from the direct rounding. The count is `2^(F-1)` because that is how many products land there.

`holds for: F in 4..=10, M = 2F-1, rounding = nearest-ties-to-even at both roundings, operation =
fixed-point multiply, signedness = unsigned, operands exhaustive over [0, 2^F), threads = 1`
