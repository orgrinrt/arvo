# Review probes for `work/arvo-standards` at `f0eeb2e3`

Ad-hoc quick spikes, not benches. Each `.rs` file is an integration test for
`arvo-format`, run by copying it into `mock/crates/arvo-format/tests/` of a tree
at `f0eeb2e3` (the `spike_ieee_formats.rs` file is lines 20 to 205 of
`tests/ieee754_binary_parity.rs` with the probe appended; the two
`spike_ties_away_*` files are lines 19 to 96 of `tests/ieee754_decimal_parity.rs`
with the probe appended) and running
`CARGO_INCREMENTAL=0 cargo test -p arvo-format --test <name>` from `mock/`.
Each probe reports by panicking with its counts, so "FAILED" is the expected
status and the message is the result.

Raw results at `f0eeb2e3`, nightly-2026-05-28:

- `spike_fimath_shapes`: 12 passed. The fimath sweep's rules hold at signed
  widths 1, 2, 7, 16, 33, 63, 64 and unsigned 1, 8, 16, 63, 64, at fraction
  lengths 0, 3, 15, -4, 10, 14, 2, under every method and both actions.
- `spike_ieee_formats::planted_formats`: `[("Binary32", 0), ("p23", 1883),
  ("p25", 5313), ("emax126", 329), ("emax128", 323), ("Floating<25,-149,254>", 1)]`.
  The binary32 sweep does separate wrong format parameters.
- `spike_ieee_formats::binary64_directed`: `[0, 0, 0, 0, 9]` for
  ties to even, toward positive, toward negative, toward zero, planted `HalfUp`.
- `spike_unsigned_ambient`: `negative member in unsigned ambient: true,
  admissible: true, p=0 format admits zero: true`.
- `spike_ties_away_decimal`: `d32 0 d64 0`; its planted control
  `spike_ties_away_planted` (the alias replaced by `HalfUp`): `d32 1474 d64 1559`.
- `clamp_is_refused.expected.txt` / `.actual.txt`: the trybuild expected and
  actual output of `arvo-numeral`'s `tests/ui/clamp_is_refused.rs` at `f0eeb2e3`;
  they differ at the instantiation note, `TowardZero` printed as `RoundTowardZero`.
  The same test passes at `faec24c6` (18 of 18 in `tests/compile_fail.rs`).
