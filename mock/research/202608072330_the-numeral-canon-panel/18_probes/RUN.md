# How to reproduce every number in `18_jhala_the_denotation_clause.md`

Toolchain pin for the Rust probe: `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
Host at the time of running: `aarch64-apple-darwin`. Python probes are stdlib only, no third-party
imports, and use exact integer or `fractions.Fraction` arithmetic throughout so no floating point
enters any count.

Every probe prints its own DOMAIN in its module docstring and in its first lines of output, because
`17` section 7 class C records that the panel's counts fail on unstated domains rather than on
arithmetic.

```
python3 p1_order_under_set_denotation.py            > p1.out
python3 p2_absorbing_top_operation_set.py           > p2.out
python3 p2b_absorbing_both_ends.py                  > p2b.out
python3 p3_interval_laws.py                         > p3.out    # defective, kept as trail
python3 p3b_interval_laws_fixed.py                  > p3b.out
python3 p3c_interval_law_directions.py              > p3c.out   # roughly two minutes
python3 p5_headroom_is_a_refinement_not_a_denotation.py > p5.out
python3 p6_precise_is_the_denotation_clause.py      > p6.out

rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
  p4_interval_erasure.rs --out-dir asm
grep '= _p4' asm/p4_interval_erasure.s        # the three folded symbol aliases
```

## What each probe is for

`p1` measures what happens to the value-level order when a datum stands for a set, across four
denotations, and finds that partitioning denotations keep the order total while overlapping ones do
not.

`p2` and `p2b` extend `07` section 4.2's fold-soundness measurement past the operation set it
covered. `07_probes/p5_postfixpoint_accumulator.py:126` uses `acc + p` only, so its zero-failure
result under the absorbing reading is quantified over a non-decreasing operation set. `p2` adds a
decreasing one; `p2b` reads both saturating endpoints as absorbing and classifies every failure by
which clamp caused it.

`p3` is **defective and is kept on disk for the trail**. Its `A - A == 0` row sliced the argument
list so the first argument was fixed at the one interval satisfying the law, and its point-numeral
control used outward rounding, which makes a point produce a two-element result. `p3b` fixes both
and `p3c` classifies each failure in both containment directions and reruns the inverse question on
a signed numeral where it has an answer.

`p4` asks whether a two-endpoint numeral erases, and separates that from the calling-convention
cost at width by giving the wide case both a typed and a raw arm.

`p5` tests the dispatch's claim that accumulator headroom is a set denotation, and finds it is a
refinement predicate on the type instead, with the reachable set computed by exact closure.

`p6` measures how often a numeral operation keeps a point denotation, which gives the record's open
`Precise` on `inexact` question a measured shape.

## Feature gates

```
grep -c '^#!\[feature' p4_interval_erasure.rs    ->  0
```

The check has to be anchored to the line start. The file's own header comments mention the
attribute, so an unanchored `grep -c '#!\[feature'` returns 2 and is a probe self-check reporting
its own prose. `17` section 7 records a stale header of the same shape in an earlier probe.

## What was not run

No bench harness. Every magnitude in the file is **unpriced**, including the calling-convention
observation in `p4`, which is the one that most looks like it wants a number.

Nothing above 8 bits of logical width. The interval order and law results are combinatorial and are
expected to hold at every width; the exactness shares in `p6` plainly move with width and the three
rows show the direction.
