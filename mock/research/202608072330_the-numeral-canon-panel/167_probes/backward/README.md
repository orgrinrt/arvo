# Probe B: is an intermediate's width a function of its producers only

```sh
rustc -O -o backward backward.rs && ./backward > backward.out
```

Exhaustive over 16,777,216 triples per operator. No timing taken and none claimed.

A three-step chain `t1 = op(a,b); t2 = op(t1,c); r = op(t2,a)`, working width `W = 12`, consumer keeps
the low `K = 6` bits. The narrowed arm reduces every operand to `K` bits up front and evaluates the whole
chain at `K`.

**Licensed, zero disagreements out of 16,777,216:** wrapping add, wrapping sub, wrapping mul, bitwise and.

**Not licensed:** `x >> 1 then + y` (14,680,064 disagreements), `/ (y|1)` (8,128), **saturating add at the
working width (2,476,720)**, min (10,812,862).

The negative control is the second group: four operators disagree, so the first group's agreement is a
result rather than an artifact of a comparison that could not fail.

**The saturating row is the one that matters for a strategy axis.** A chain of wrapping additions may be
evaluated at the consumer's width; the same chain of saturating additions may not, because saturation is
not a congruence modulo 2^K and the clamp depends on bits the narrowed arm has thrown away.

`holds for: W = 12, K = 6, chain length 3, unsigned, threads = 1, the operator set listed`
