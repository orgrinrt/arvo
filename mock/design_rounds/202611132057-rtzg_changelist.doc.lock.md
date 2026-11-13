# Doc changelist: the fixed-point aliases and the exact accumulator

**Date:** 2026-11-13
**Round:** 202611132057-rtzg, branch `work/arvo-values`, PLAN step 8.
**Topics consumed:** `202611132057-rtzg_topic.the-fixed-point-aliases-and-the-exact-accumulator.md`,
the only flat topic on this branch or on `origin/dev`.
**Sketches:** none new. The probes the design rests on are registry rows and are named below.
**Scope:** `mock/crates/arvo-numeral/DESIGN.md.tmpl`, `mock/crates/arvo-numeral/README.md.tmpl`. No
change to `arvo-format` and no registry file.

## Canon

`obligation::a_deterministic_fixed_point_lane_with_declared_rounding`: the aliases, with the rounding
the consumer's to declare.
`proposal::a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`, at one expert,
and `retirement::dl_growing_an_accumulator_type_per_iteration`: one accumulator type, its sufficiency
a compile-time bound.
`probe::a_runtime_length_fold_compiles_at_every_length_and_computes_at_one`: the failure the capacity
makes unwritable.
`question::which_chain_carrier_ships`, open: the accumulator is one instance and closes nothing.
`ruling::never_a_runtime_check_and_one_lowered_path`: no count is validated.
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`, on a provisional stamp:
built on as provisional and not promoted.

## CHANGE: `arvo-numeral`, the fixed-point aliases

File: `mock/crates/arvo-numeral/DESIGN.md.tmpl`.

The canon list at the top gains the obligation, the proposal on folds and the open question, each
with what it is cited for.

`## Aliases` gains `Fix<BITS, FRAC, A, C>` over `Fixed<BITS, FRAC>` and `UFix<BITS, FRAC, A, C>` over
`UFixed<BITS, FRAC>`, reading the adaptation as the integer aliases do. The text says the rounding is
the consumer's to declare and no alias fixes one, that the product, quotient and conversion already
round by the declared mode, that every operation is integer arithmetic on the declared width, and
what a consumer splitting a sum has to know: associative under wrapping, not under saturation over a
signed range.

## CHANGE: `arvo-numeral`, the accumulator

A new section, `## The exact accumulator`, after `## A value under a sibling signature`. It states
`Accumulator<S, T, CAP, C>`: the exact sum of at most `CAP` members of `S`, held in a value of `T`.
It states the admission, `T` on `S`'s grid and wide enough for `CAP` times each end of `S`'s range,
refused as a failed constant otherwise, and why `T` is named and checked and not derived. It states
the functions and how each keeps the capacity: `empty`, `of::<N>`, `take`, `plus`, `minus`, `total`,
`finish` and `fits`. It states that `finish` is `S`'s adaptation applied once and carries the debug
check, that `T`'s own adaptation never acts, that the result does not depend on the order of the
terms, and the limit at a 64-bit carrier. It says the accumulator is one carrier a chain may use and
does not answer which one ships.

`## The debug check` names `finish` among the sites.

`## What is proved, and how` gains the aliases' tests, the accumulator against the exact sum
exhaustively at small widths and lengths and seeded above, `take` around the capacity, `plus` and
`minus`, the order-independence case on the signed saturating cell, the probe's numbers through the
real type, the mutant `mutant-accumulator`, the four compile-fail pairs, the debug check in `finish`
and the consumer.

`## What is not here` drops the accumulator and the lanes, and gains a capacity type for a slice's
length, an accumulator wider than a value and an accumulator of products.

`## The old families this is the base for`: the sentence saying only the integer points are here is
corrected, since the fixed-point ones are as well.

File: `mock/crates/arvo-numeral/README.md.tmpl`. The aliases sentence names `Fix` and `UFix`, and one
sentence names the accumulator.

## What the source changelist will carry

In `arvo-numeral`: the aliases `Fix` and `UFix`, a module `accumulator` with `Accumulator` and its
eight functions, the feature `mutant-accumulator`, the tests named above, the mutant's red run under
`mock/research/sketches/202611132057-rtzg_value-mutants/`, and the consumer crate reaching each new
item.
