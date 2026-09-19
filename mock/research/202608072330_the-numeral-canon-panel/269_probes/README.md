# 269_probes

Seat 269's probes, numbered `270`, and its re-runs of the seats it audits.

- `p270a_each_reading_is_an_alias_over_the_other_names.rs`, output `p270a_output.txt`. Whether each
  reading of `half_up` is reachable from `floor` or `toward_zero` plus one exact addition of the half
  step, over every value of every container at `W` in 2..=16, `F` in 1..=W-1, both signednesses.
  Three negative controls, each asserted to fail exactly as predicted.
- `p270b_wrap_commutation_under_both_readings.rs`, output `p270b_output.txt`. Whether
  `probe::rounding_commutes_with_the_overflow_policies` holds under both readings. Three controls.
- `p270c_what_each_reading_lowers_to.rs` and the emitted `p270c_what_each_reading_lowers_to.s`. An
  ad-hoc quick spike, not a bench: one compiler, one target, instruction shapes read off the listing.
  It prices nothing.
- `sources.sh`, output `sources.out`. The passages quoted in `269`, extracted from the documents in the
  workspace store by hash, with the absence counts for the wording seat 267 attributes to MathWorks.
- `reproduction.txt`. The re-runs of `229_probes`, `267_probes` and `228_probes/p2`.

Host `aarch64-apple-darwin`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
