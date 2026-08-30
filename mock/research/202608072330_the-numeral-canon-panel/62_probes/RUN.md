# 62_probes build line

Toolchain: `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), passed
explicitly on every invocation. No probe carries a feature gate:
`grep -c '^#!\[feature' *.rs` returns 0 on every file.

```
rustc +nightly-2026-05-28 -O --edition 2021 -o p1  p1_signed_grid_multiplication.rs        && ./p1  > p1_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o p2  p2_signed_cell_at_nonzero_fraction.rs   && ./p2  > p2_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o p2b p2b_failure_sets_at_deep_fraction.rs    && ./p2b > p2b_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o p3  p3_wrap_section_becomes_observable.rs   && ./p3  > p3_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o p4  p4_signed_multiplicative_accumulator.rs && ./p4  > p4_output.txt
```

Every probe prints its own instrument-validation lines and exits nonzero on failure.

`p3_output.v1_bad_mutant.txt` is p3's first run, kept per the panel's discipline: its
mutant (`rep(c) = c - m` with a shifted threshold) is congruent to `c` mod m and therefore
not a mutant, the check reported zero where it demanded nonzero, and the run printed
`P3 FAILS`. The corrected mutant misaligns the class by one and fires at 176.

## Reruns of inherited instruments (`rerun/`)

Rebuilt on the pin and diffed against their committed outputs before anything from them
was argued with. All byte-identical:

```
rustc +nightly-2026-05-28 -O --edition 2021 -o p3_57 ../../57_probes/p3_semiring_across_widths_and_scales.rs && ./p3_57 > p3_57.out
diff p3_57.out ../../57_probes/p3_output.txt          # empty
rustc +nightly-2026-05-28 -O --edition 2021 -o q2_61 ../../61_probes/q2_wrap_ring_at_nonzero_fraction.rs && ./q2_61 > q2_61.out
diff q2_61.out ../../61_probes/q2_output.txt          # empty
rustc +nightly-2026-05-28 -O --edition 2021 -o p3_35 ../../35_probes/p3_reduction_order.rs && ./p3_35 > p3_35.out 2> p3_35.err
diff p3_35.out ../../35_probes/p3.out                 # empty (stdout; stderr is witness lines)
```
