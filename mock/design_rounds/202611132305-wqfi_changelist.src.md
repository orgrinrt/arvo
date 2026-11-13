# Src changelist: the IEEE suites land through `land`

**Date:** 2026-11-13
**Round:** `202611132305-wqfi`, job `arvo-format`.
**Doc changelist:** `202611132305-wqfi_changelist.doc.lock.md`.
**Scope:** `arvo-format`'s `tests/ieee754/mod.rs`, and `tests/the_landing_agrees_with_the_search.rs`
(deleted).

## CHANGE: the harness's `land` is one call to the crate's

`tests/ieee754/mod.rs`'s `land` builds a `Scaled` from the value and calls `arvo_format::apply::land`,
reading the slot back through `adapt` and the exponent through `step_exponent` at the landing's
magnitude. Where `land` returns nothing, the value is positioned at the top magnitude alone (the largest
on a rising law, magnitude zero on a falling one) and adapted there, as the overflow region. The walk
over every magnitude and its `panic_on_overflow` go; `position` stays for the top magnitude. The module
doc says the parity suites check `land` itself.

## CHANGE: the agreement test goes

`tests/the_landing_agrees_with_the_search.rs` is deleted, since the search it agreed with is gone.

## Verification

`cargo test -p arvo-format`, lib and `compile_fail` and the four parity suites; clippy `-D warnings`;
`cargo mock --lint-only`. Mutant: `land`'s walk started one magnitude past `first_candidate`, which the
binary and decimal parity suites both show red.
