# 88_probes/OUTCOMES.md

All five files built and run fresh this session, inside the repo tree, on the pinned toolchain
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed by `rustc
--version` immediately before each compile). Working directory:
`mock/research/202607301300_formalization-spec-panel/88_probes/`. Every compile is a standalone
`rustc --edition 2021 -O` invocation; none touches `mock/crates`.

## probe_1_three_level_digest_chain.rs

```
$ rustc --edition 2021 -O -o /tmp/probe1 probe_1_three_level_digest_chain.rs
warning: constant `W_C` is never used
warning: 1 warning emitted
$ /tmp/probe1
hot_shape: reproduces 72_probes/probe_4 part (a) exactly; the dirtied region is [13,16), the
container tier, not statement P
warm_shape: both tiers real (13-bit statement P, 6-bit statement C); masking to W_F undoes both
in one step, independent of the intermediate carrier-level digest which only undoes statement C
exit: 0
```

Confirms, at a case where both statement P and statement C are simultaneously real (the ratified
`Warm`/`Precise` row, `83:135-141`, W_F=13, W_S=26, W_C=32), that a datum-keyed digest is a single
mask to `W_F`, immune to dirt in either or both intermediate tiers, and that an intermediate
"carrier-level" digest (masking only to `W_S`, undoing statement C alone) is immune to
container-tier dirt but NOT to statement-P dirt, which is the compiled distinction between the
container level and the carrier level that file 72's original probe (`hot_shape` here, W_F=W_S=13,
reproduced exactly) could not show, because at that width statement P is vacuous and the two
levels coincide.

## probe_2_tier1_free_theorem_vs_tier2_trusted.rs

```
$ rustc --edition 2021 -O -o /tmp/probe2 probe_2_tier1_free_theorem_vs_tier2_trusted.rs
warning: constant `W_S` is never used
warning: 1 warning emitted
$ /tmp/probe2
tier-1 theorem holds for an honest column: both the raw-buffer digest and the masked-fold digest
are sound functions of the datum sequence alone, the raw form at zero extra per-element cost
tier-2: a single raw-door misuse decorrelates the free shortcut while the masked fold stays
correct, reproducing 87_probes/probe_2's finding at the column level
exit: 0
```

A 32-element `Layout::Dense` column, every element embedded through the pure constructor. Two
independent honest constructions of the same datum sequence give the same raw-buffer digest
(the tier-1 theorem, checked by rebuilding rather than by definition), and the same holds for the
masked per-element fold independently. One element's padding is then dirtied through what an
ordinary safe `&mut` accessor would do (no unsafe transmute needed, the weakest form of the attack
per `87:384-386`): the raw-buffer digest changes, the masked fold does not, reproducing
`87_probes/probe_2`'s per-value finding at column scale with a real 32-element buffer rather than
a single carrier.

## probe_3_value_keyed_never_free.rs

```
$ rustc --edition 2021 -O -o /tmp/probe3 probe_3_value_keyed_never_free.rs
$ /tmp/probe3
value-keyed digest requires Encoding::Canonical per element; no masking shortcut exists, at any
construction discipline, confirming V -> D is a non-site for the const-position test
exit: 0
```

A model decimal cohort (trailing-zero collapse) and a model NaN-payload pair. A datum-keyed
digest correctly separates both pairs; a value-keyed digest correctly collapses both, and does so
only through the same canonicalisation step `Encoding::Canonical` performs, which reads the
datum's own content (trailing zeros, the NaN discriminant) rather than discarding a fixed bit
range. No mask reproduces this, at any construction discipline, confirming the categorical
(not merely tiered) cost difference between the two digest kinds.

## probe_4_column_grouping_invariance.rs

```
$ rustc --edition 2021 -O -o /tmp/probe4 probe_4_column_grouping_invariance.rs
$ /tmp/probe4
naive sequential fold: order-sensitive but NOT grouping-invariant (morsel-then-combine diverges
from the direct fold)
positional combine: order-sensitive AND grouping-invariant at every tested split, by the
identical exponent-offset shift argument the multiplicative half already compiled at 68 section 1.9
exit: 0
```

A 64-element column. The naive chained running hash's morsel-then-combine result differs from
its own direct sequential fold at a genuine split (asserted, witness at split=32). The positional
(polynomial) combine reproduces the sequential result at every tested split (0, 1, 16, 32, 48,
63, 64, including the two degenerate single-morsel splits), and both constructions remain
order-sensitive (a two-element swap changes both digests), confirming grouping invariance and
order sensitivity are independent properties and the positional construction has both.

## probe_5_bitpacked_column_digest.rs

```
$ rustc --edition 2021 -O -o /tmp/probe5 probe_5_bitpacked_column_digest.rs
$ /tmp/probe5
bitpacked column, W=13, N=65: one tail-padding region of 3 bits is the ENTIRE dirt surface for the
whole column, against one region per value in the dense case (probe 2)
tier-1 theorem holds for the bitpacked case with a strictly smaller dirt surface than dense:
interior groups have zero padding by the ratified single meaning of Layout::Bitpacked, so only
the column-level tail can ever decorrelate the free shortcut
exit: 0
```

A W=13, N=65 `Layout::Bitpacked` column (65 = 8 full groups of the period P=8 plus one leftover
value, forcing a genuine tail group). The group-is-whole-bytes theorem (`G*8 = W*P`, `83:216-218`)
is checked at this width before anything else. The honest round trip holds. Two independent
honest packings of the same value sequence give the same raw-buffer digest (the tier-1 theorem,
bitpacked case). Dirtying only the tail group's own padding bits (which the round trip confirms
carry no live value) still decorrelates the raw digest, confirming the tier split applies here
too, with a dirt surface of exactly one tail region for the whole column rather than one region
per value.
