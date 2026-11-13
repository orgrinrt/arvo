# Src changelist: landing a point on a format's magnitudes

**Date:** 2026-11-13
**Round:** `202611132120-sknb`, job `arvo-format`.
**Doc changelist:** `202611132120-sknb_changelist.doc.lock.md`.
**Scope:** `mock/crates/arvo-format/src/apply/landing.rs` (new) and its `tests.rs`, `src/apply.rs`,
`src/lib.rs`, `src/wide.rs`, and `tests/the_landing_agrees_with_the_search.rs` (new).

## CHANGE: the landing

`apply::landing` holds `Scaled` with `Scaled::of(num: i128, den: u128, power: Exponent)`, a zero
denominator reading as zero, `Landing` with `magnitude` and `exact`, and
`land<S: DeclaredSignature>(Scaled, Dither) -> Maybe<Landing>`. `land` forces the slot range's
obligation, reads the phase in lowest terms through `placing::reduced_phase`, starts at the first
magnitude the bit-length bound leaves, and walks from the finest, magnitude zero for a rising law and the
largest for a falling one. At each it forms `n q U - p d V` over `D = d q V` in the four limbs, places it
with `placing::over` and `positioned`, and returns the first whose position `panic_on_overflow` holds in
range. Past the top it returns `Isnt` under
`FIXME(arvo_landing_a_value_on_a_magnitude_indexed_format_is_a_call)`. Every function is `const fn` and
`#[inline]`. `apply` and the crate root re-export the three names and `land`.

## CHANGE: a denominator in the four limbs

`Wide::of_unsigned(u128)`, beside `Wide::of`, since the point's denominator is unsigned.

## CHANGE: the tests

`src/apply/landing/tests.rs`: an oracle that places the point at every magnitude in the host integer and
takes the finest held in range, against `land`, every mode and three dithers, on `Floating<4, -6, 8>`, a
falling law, a phased law, a decimal law, `Integer<4>`, `Biased<4, -2, 1>`, and two 64-bit slot formats
over far shifts; a zero denominator; the top magnitude's edge; a point past what the host integer
positions. `tests/the_landing_agrees_with_the_search.rs`: the IEEE suites' search in `ieee754/mod.rs`
against `land` on points drawn across binary16, binary32, binary64, decimal32 and decimal64, every
direction, each sweep landing most of its points below the top.

## Verification

`cargo test -p arvo-format` with the lib tests, `compile_fail` and the four parity suites, the new
integration test, `cargo clippy -p arvo-format --all-targets -- -D warnings`, `cargo mock --lint-only`, and
mutants shown red: the walk started one magnitude coarser than the bound gives, and the order of a falling
law reversed.
