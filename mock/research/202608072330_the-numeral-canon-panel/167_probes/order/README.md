# Probe F: is association order an accuracy lever in fixed point

```sh
rustc -O -o order order.rs && ./order > order.out
```

No timing. Exact error against an `i128` oracle for the fixed-point arms and an `f64` compensated oracle
for the `f32` arm.

## Fixed point, Q(.12) multiply-accumulate, error in LSBs, worst of 32 seeds

| n | sequential | pairwise tree | difference |
|---|---|---|---|
| 16 | 9.985 | 9.985 | **0.000** |
| 64 | 36.224 | 36.224 | **0.000** |
| 256 | 139.219 | 139.219 | **0.000** |
| 1024 | 536.403 | 536.403 | **0.000** |
| 4096 | 2082.494 | 2082.494 | **0.000** |
| 16384 | 8256.423 | 8256.423 | **0.000** |
| 65536 | 32922.755 | 32922.755 | **0.000** |

Zero sizes where either order is strictly better.

## NC12, the control: the same comparison on a relative-precision accumulator

| n | sequential err | tree err | seq/tree |
|---|---|---|---|
| 16 | 1.20e-6 | 6.11e-7 | 1.96 |
| 256 | 6.99e-5 | 1.05e-5 | 6.63 |
| 4096 | 4.17e-3 | 2.38e-4 | 17.56 |
| 65536 | 2.43e-1 | 2.56e-3 | **94.80** |

The tree wins at 7 of 7 sizes. So the instrument detects an ordering effect where one exists, which is
what makes the zeros above a finding rather than a broken comparison. NC13, that both orders perform the
identical number of truncations, is clean at every size.

## What this establishes

**The accuracy argument for reassociating a reduction is a floating-point argument and it does not
transfer to fixed point.** A float's absolute rounding error scales with the magnitude of the running
sum, so keeping partial magnitudes balanced keeps errors small. A fixed-point accumulator has a fixed
absolute LSB, the number of roundings is the same in both orders, and the error is bit-identical.

So in fixed point, association order is **purely a speed lever with zero accuracy content**, and in
relative precision it is both. A canon sentence about reassociation that does not name the numeral
family is wrong for one of them.

`holds for: fixed rows, F = 12, I = 3, n in {16 .. 65536}, operands uniform in [-4,4), truncation =
floor, signedness = signed, arity = 2 tree, threads = 1. f32 rows: operands uniform in [0,1), all
positive, so catastrophic cancellation is absent by construction and the f32 advantage is not claimed
for mixed-sign data.`
