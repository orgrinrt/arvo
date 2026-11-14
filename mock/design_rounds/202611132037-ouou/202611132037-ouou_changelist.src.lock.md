# Src changelist: the warnings arvo-format carries

**Date:** 2026-11-13
**Round:** `202611132037-ouou`, job `arvo-format`.
**Doc changelist:** `202611132037-ouou_changelist.doc.lock.md`.
**Scope:** `mock/crates/arvo-format/src/apply.rs`, `src/format.rs`,
`src/apply/tests/the_ratio_coordinate.rs` and `src/tests/the_identity/the_magnitude_range.rs`.

## CHANGE: `Rounded::at` is compiled for the tests

`#[cfg(test)]` on `Rounded::at`, which stays beside the type it builds. Its one caller is
`apply/tests/mod.rs`.

## CHANGE: the redundant attribute

`#[must_use]` comes off `cancelling_slot`, whose return type `Maybe` is must-use already.

## CHANGE: two tests say it the short way

The error numerator in `the_ratio_coordinate.rs` takes `a.abs_diff(b)`, and the count in
`the_magnitude_range.rs` is one `if let ... && ...` chain.

## Verification

`cargo clippy -p arvo-format --all-targets` with no warning, `cargo test -p arvo-format` and the four
parity suites as the brief names them.
