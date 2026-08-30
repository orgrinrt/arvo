# Probe E: which operators may be reassociated, derived independently

```sh
rustc -O -o assoc assoc.rs && ./assoc > assoc.out
```

Exhaustive over the entire domain at `W = 4, 6, 8`: 16^3, 64^3 and 256^3 triples per operator. No
timing.

| operator | W=4 | W=6 | W=8 | verdict |
|---|---|---|---|---|
| wrapping add | 0 | 0 | 0 | associative |
| saturating add, unsigned | 0 | 0 | 0 | associative |
| **saturating sub, unsigned** | 3,040 | 212,352 | 13,882,880 | **not** |
| wrapping mul | 0 | 0 | 0 | associative |
| saturating mul, unsigned | 0 | 0 | 0 | associative |
| **fixed mul, truncate, F = W/2** | 878 | 53,032 | 2,097,706 | **not** |
| **fixed mul, round nearest, F = W/2** | 958 | 53,648 | 2,092,854 | **not** |
| min | 0 | 0 | 0 | associative |
| max | 0 | 0 | 0 | associative |
| **average, floor** | 3,152 | 246,080 | 16,516,352 | **not** |
| bitwise or | 0 | 0 | 0 | associative |
| bitwise xor | 0 | 0 | 0 | associative |

NC8: eight associative and four not, so the column carries information. NC9: a deliberately broken
reference comparing the left-associated form to itself reports 0 disagreements across all twelve
operators, which is what shows the real test compares two distinct expressions.

**The finding that bears on chains.** Fixed-point multiply is not associative, and **round-to-nearest
does not rescue it**: at W=4 the rounding arm is slightly worse (958 against 878) and at W=8 slightly
better (2,092,854 against 2,097,706), on the same order either way. So per-operation accuracy and
chain-level algebraic licence are independent axes, and improving the first buys nothing on the second.

`holds for: W in {4, 6, 8}, unsigned, F = W/2 for the fixed-multiply rows, arity 3, threads = 1, the
operator set listed`
