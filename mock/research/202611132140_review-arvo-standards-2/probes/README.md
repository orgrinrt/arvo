# Review 2 probes for `work/arvo-standards` at `9e270619`

Ad-hoc quick spikes, not benches. One crate, its own workspace, with a path
dependency on `mock/crates/arvo-format` of the clone at `9e270619`; it includes
the suite's own `tests/ieee754/mod.rs` by `#[path]` so the planted arms run the
exact rule and landing the suite runs. Run from this directory with
`CARGO_TARGET_DIR=/home/user/target CARGO_INCREMENTAL=0 cargo test -- --nocapture`,
and each feature arm with `--features <name>`. nightly-2026-05-28.

Raw results:

- default, `the_format_verdict_admits_the_unsigned_interchange`: passes, so
  `is_admissible_format::<Interchange<UnsignedBinaryRationals, 24, 127>>()` is
  `true`.
- default, `the_missing_decimal_planted_arms_are_writable_and_miss`: passes,
  printing `d32 +inf as zero 2820, d32 zero as -inf 2995, d64 +inf as zero 2912,
  d64 -inf as +inf 5913, d64 short p 5652`.
- `--features slots_verdict`: does not compile. `error[E0080]: evaluation
  panicked: an IEEE 754 significand is sign and magnitude, so its ambient domain
  must carry values below zero; this ambient is unsigned`, evaluation of
  `Significand::<UnsignedBinaryRationals, 24>::SIGN_AND_MAGNITUDE`, reached from
  `slots::is_admissible`.
- `--features emax0`: does not compile. `evaluation panicked: a quantum law over
  no magnitudes describes no values, so it is not a step law`, from
  `<ExponentRange<24, 0> as Quantum>::ADMITTED`. Refused, as it should be.

At the job's fix for `770d4d67` the crate lost its `allow(dead_code)`: it builds only under test, imports what its decimal test names (the committed copy did not), declares the two decimal formats itself since the shared module no longer carries them, and adds one planted arm, ties to even checked against the ties-to-away rule, so every variant of the shared module is used. `cargo test` passes its two default tests.
