# Probe D: does a per-operation accuracy contract compose

```sh
rustc -O -o doubleround doubleround.rs && ./doubleround > doubleround.out
```

Exhaustive over every operand pair at each `F`. No timing.

Take the strongest per-operation contract available: every operation returns the correctly rounded
result, nearest, ties to even. A Q(.F) product is exact at 2F fraction bits. A design storing the
intermediate at `M` fraction bits rounds twice; one keeping the exact product rounds once.

Disagreements between the two, out of all pairs:

| F | M = F | F+1 | F+2 | ... | 2F-1 | 2F |
|---|---|---|---|---|---|---|
| 6 | 0 | 832 | 480 | 224, 96 | 32 | 0 |
| 8 | 0 | 15360 | 8064 | 3968, 1920, 896, 384 | 128 | 0 |
| 10 | 0 | 257024 | 130560 | 65024 ... 1536 | 512 | 0 |

**There is no `M` strictly between `F` and `2F` with zero disagreements, at any `F` tested.** Each extra
intermediate bit roughly halves the disagreement count and never reaches zero until the intermediate is
exact. So a chain-level accuracy guarantee **cannot be bought by strengthening the per-operation
guarantee**; it is bought only by not rounding the intermediate at all.

The `M = F` column is zero because the first rounding is then the identity, which makes it a third
control rather than a finding: it is the row that would be nonzero if the harness were lossy.

NC6 (`M = 2F` must agree) and NC7 (some `M` in the open interval must disagree) both clean at every `F`.

`holds for: F in {6, 8, 10}, M in [F, 2F], rounding = nearest-ties-to-even, operation = fixed-point
multiply, unsigned, chain length 1 multiply with 2 roundings, threads = 1`
