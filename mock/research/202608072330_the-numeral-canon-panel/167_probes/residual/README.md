# Probe A: the residual is the chain-level fact

Two programs, built with `rustc -O` on the pinned nightly, run from this directory.

```sh
rustc -O -o residual residual.rs && ./residual        > residual.out
rustc -O -o carrier_bound carrier_bound.rs && ./carrier_bound > carrier_bound.out
```

**Not benchmarks.** No timing is taken and none is claimed. These produce exact error magnitudes and
counts against an `i128` oracle.

## What `residual.rs` establishes

Over a fixed-point multiply-accumulate chain at `F = 12`, error in LSBs of the Q(.12) result, worst of
32 seeds:

| n | naive (floor per step) | naive_round (nearest per step) | widened | comp (residual carried) |
|---|---|---|---|---|
| 16 | 10.94 | 2.87 | 0.998 | 0.998 |
| 256 | 136.2 | 8.43 | 0.995 | 0.995 |
| 4096 | 2084.7 | 37.95 | 0.989 | 0.989 |
| 65536 | 32831.3 | 231.6 | 0.984 | 0.984 |
| 1048576 | 524046.5 | 1418.2 | 0.929 | 0.929 |

`naive` grows linearly in `n`. `naive_round`, which is the best a per-operation design can do, grows as
`sqrt(n)`. `widened` and `comp` are bounded below one LSB and **do not grow at all**.

`comp` equals `widened` on every row, which is a theorem rather than a coincidence: `acc * 2^F +
carry_n = sum(p_i)` exactly, so the carried residual reconstructs the wide accumulation without ever
materialising a wide accumulator.

Negative controls, all clean: NC1 (`F = 0`, every arm exact, 0 violations), NC2 (`fake_comp` computes
the residual and discards it, must equal `naive` bit for bit, 0 violations across 11 sizes), NC3
(products exactly representable, `naive` must be exact, 0 violations).

## What `carrier_bound.rs` establishes, and how its first version failed

**v1 failed and the failure is the finding.** It drew operands from `[-4, 4)`, so the accumulated sum is
a random walk growing as `sqrt(n)`, the worst-case bit count `I + 2F + log2(n)` was never approached,
and **zero** geometries overflowed at any size tested. `carrier_bound_v1_FAILED.out` is that run, kept.

The mechanism: worst-case accumulator width is reached only by non-cancelling terms. v2 draws from
`[0, 4)` and the boundary appears exactly where the arithmetic predicts it.

At a 64-bit container, `I = 3`:

| F | n | widened bits needed | comp bits needed | outcome |
|---|---|---|---|---|
| 20 | 262144 | 61 | 41 | both exact |
| 24 | 16384 | 65 | 41 | **comp only** |
| 24 | 262144 | 69 | 45 | **comp only** |
| 26 | 1024 | 65 | 39 | **comp only** |
| 26 | 16384 | 69 | 43 | **comp only** |
| 26 | 262144 | 73 | 47 | **comp only** |

Five geometries where the one-rounding guarantee is reachable by carrying the residual and unreachable
by widening the accumulator, at a fixed container width. NC4 (a geometry where both fit: the arms must
agree, and do) is clean.

`holds for: container = 64 bits, I = 3 including sign, F in {8,16,20,24,26}, n in {2^10, 2^14, 2^18},
operands non-cancelling same-sign, rounding = floor, signedness = signed, threads = 1, profile = the
`rustc -O` build above`
