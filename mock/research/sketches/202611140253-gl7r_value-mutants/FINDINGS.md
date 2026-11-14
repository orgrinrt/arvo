# The value mutants as compiler flags

Round `202611140253` moves the mutants of `arvo-numeral` from cargo features to compiler flags, so
that no crate of a build can turn one on for another through feature unification, and adds
`arvo_mutant_convert_policy`, which completes a conversion by saturation wherever the source
saturates. This sketch holds one red run of each flag, to show that every flag still turns its own
tests red under the new spelling, and that the new mutant is seen by the sweep it was written for and
by no other.

Outcome: WORKS. Every one of the fourteen flags fails the tests that name its operation.

## How it was run

`run.sh` runs each flag from `mock/` as `RUSTFLAGS="--cfg <flag>" cargo test -p arvo-numeral
--no-fail-fast` against the tests below, and writes the run less its passing lines into
`outputs/<name>.txt`. Names given to it run only those outputs. The count mutant runs twice, once on
each of its two paths, since `the_count` builds only with `hlist-const` and `the_count_without_const`
only with `hlist` alone.

## What each run shows

The four rung flags, `arvo_mutant_8`, `arvo_mutant_16`, `arvo_mutant_32` and `arvo_mutant_64`, fail
the library's rung tests: 4 of 12 under the first and 8 of 12 under each of the others. The three
operation flags fail the library too, `arvo_mutant_sub` and `arvo_mutant_mul` on 2 tests each and
`arvo_mutant_neg` on 1.

`arvo_mutant_under` fails 6 of the 8 tests of `the_crossing`, `arvo_mutant_div` 2 of the 5 of
`the_quotient`, `arvo_mutant_convert` 5 of the 10 of `the_conversion` and `arvo_mutant_index` 6 of the
10 of `the_host_door`. `arvo_mutant_count` fails 2 of the 4 tests of `the_count` and all 3 of
`the_count_without_const`.

`arvo_mutant_convert_policy` fails the three tests of
`the_conversion_under_every_source_signature_past_the_range`, one a radix, and nothing else that was
run: the two tests that pass in that binary are the shared self-tests of `common::big`. At radix two
the sweep disagrees with the oracle on 322700 of 3525400 checks. The step from exponent minus one onto
zero, `the_conversion_under_every_source_signature`, passes all 5 under it, because no member leaves
the target's range there and the policy is never consulted, and so does
`the_conversion_at_the_positive_edge_binary`, whose sources wrap. This is the gap the round closes: on
the step alone the source's policy could not be seen.

## The policy over every conversion sweep, and the mode

Round `202611140356` runs `arvo_mutant_convert_policy` again over all 19 conversion binaries, each
`tests/the_conversion*.rs`, so that its passing the rest is a record rather than a check by hand. It
turns the three tests of `the_conversion_under_every_source_signature_past_the_range` red and nothing
else: every other conversion binary passes whole under it.

The same round adds `arvo_mutant_convert_mode`, which rounds the conversion by the source's mode in
place of the target's. Over the two binaries under every source signature it turns the three tests of
the step from minus one onto zero red, `the_conversion_under_every_source_signature`, and passes the
step from zero onto minus one, where the target's grid is the finer and nothing is rounded. The policy
mutant is seen by its own step and by no other conversion sweep, the mode mutant by its own step of
the two it was run over.

## Predicate

holds for: the flags named above, `arvo-numeral` at `3ba8f280` for the first thirteen runs and in round
`202611140356` for the policy over every conversion binary and for the mode, the debug profile, `x86_64`
Linux, the nightly the repository pins; each flag against the tests its output names and no others.
