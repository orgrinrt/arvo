# Probe G: does the residual carry work for a PRODUCT chain

Built in phase two, to attack my own phase-one mechanism against `AGREEMENTS.md:497-506`, which records
three topics converging that multiplicative chains need width growing linearly in fold length.

```sh
rustc -O -o product product.rs           && ./product      > product.out
rustc -O -o product_long product_long.rs && ./product_long > product_long.out
```

No timing. Exact error against the exact rational held as an `i128` numerator.

## Short chains, F = 8, 400 chains per length, max |err| in LSBs

| len | per_step | deferred | carried(F) | carried == deferred |
|---|---|---|---|---|
| 1 | 0.000 | 0.000 | 0.000 | 400/400 |
| 3 | 2.691 | 1.000 | 1.000 | 400/400 |
| 5 | 8.355 | 0.994 | 1.001 | 399/400 |
| 8 | 23.301 | 0.999 | 1.003 | 399/400 |

NC14 (all three identical at length 1) and NC15 (deferred differs from per_step somewhere, so the probe
has resolution) both clean.

## Long chains, F = 4, where the mechanism actually stops

| len | per_step | deferred | carried(F) | carried(2F) |
|---|---|---|---|---|
| 2 | 0.938 | 0.938 | 0.938 | 0.938 |
| 8 | 27.657 | 0.996 | 1.390 | 1.012 |
| 16 | 121.422 | 0.994 | 8.616 | 1.386 |
| 24 | 2221.867 | 0.993 | 60.867 | 5.867 |

NC16 (per_step keeps growing over the range, so the flat rows are a bound and not a ceiling) clean.
NC17 is a per-chain `i128` against `f64` magnitude cross-check on the oracle, clean at every row.

## What this establishes, and it bounds my own phase-one claim

**The residual carry is exact and constant-state for an accumulate chain and neither for a product
chain.** Probe A showed `comp == widened` on every row because `acc * 2^F + carry = sum(p_i)` exactly.
For a product, each carried limb buys a constant factor and not a change of growth class: one limb is
36x better than per-step at length 24 and still 61x worse than deferred, two limbs are 379x better and
still 5.9x worse.

The reason is arithmetic and it is why no amount of carried state fixes it. A product of `k` factors at
`F` fraction bits needs `F*(k-1)` fraction bits to be exact. A constant `c` limbs hold `c*F`. So the
carried form tracks the exact answer up to `k = c + 1` and departs from it after, which is what the
`400/400` column shows for `c = 1` at `k <= 3`.

**This corroborates `AGREEMENTS.md:497-506` rather than refuting it**, and it is the honest bound on what
probe A contributes: the accumulate shape and the product shape are different chains with different
answers, and a canon sentence that says "a chain" without saying which is wrong for one of them.

`holds for: F in {4, 8}, factors in (0, 2], chain length in {1..8} at F = 8 and {2,4,8,12,16,20,24} at
F = 4, 400 chains per length, truncation = floor, signedness = signed, container = i128, operation =
mul, arity = 2, family = fixed point, threads = 1`
