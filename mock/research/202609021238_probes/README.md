# Probe: what the `Slots::ADMITTED` obligation is actually guarded by

One question: if four of the five assertions in `Slots::ADMITTED`
(`mock/crates/arvo-format/src/slots.rs:210`) were deleted, would anything in the
repository go red?

## What was run

Baseline, at `7fed7b5932f31e77c8b288bd8aabbe93ec3b40aa`, unmodified tree:

```
cd mock && cargo test --workspace --all-targets   # output_baseline_all_targets.txt
cd mock && cargo test --workspace --doc           # output_baseline_doc.txt
```

156 + 9 = 165 passed, 0 failed, 2 ignored (both catalogue-reds).

Mutant, `slots.rs.mutant`, which is `slots.rs.baseline` with everything after the
first `assert!` removed from the `ADMITTED` block. `the_mutation.diff` is the
exact change. The four assertions removed are `WIDTH.count() >= 1`,
`WIDTH.count() <= 62`, the `i64::MAX` span bound and the `1i128 << WIDTH` bound.

```
cd mock && cargo test --workspace --all-targets && cargo test --workspace --doc
```

165 passed, 0 failed, 2 ignored. `output_mutant_suite.txt`.

## The result

**Identical.** Not one test, doctest or `trybuild` case in the workspace changes
colour when the obligation loses four of its five conditions, including both of
the ones that are about what an `i64` holds.

## Why, mechanically

The five conditions are written twice. Once in the const at `slots.rs:210` and
again, independently, in `is_admissible` at `slots.rs:249`, which is what every
test calls. `MIN <= MAX` is the only one with a build-refusal case, the
`compile_fail` doctest at `slots.rs:166`. `WidthTooNarrow` and `SpanTooWide` in
`mock/crates/arvo-format/src/tests/the_inventory.rs` exercise the other bounds
through the verdict copy and never force the const.

## The negative control

The mutation is not inert: `slots.rs.mutant` still compiles and the removed
assertions are the ones the tests name in their own messages, so a suite that
reached the const would have failed here. That it did not is the finding rather
than a property of the mutation. Restoring `slots.rs.baseline` and touching the
file returns the suite to the baseline figures above.
