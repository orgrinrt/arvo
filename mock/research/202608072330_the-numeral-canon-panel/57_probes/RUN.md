# 57_probes: how to run, and what each probe is for

Toolchain `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), passed explicitly.
A bare `rustc` outside the repository tree resolves to stable.

```
rustc +nightly-2026-05-28 -O --edition 2021 -o p1 p1_absorption_versus_bound_count.rs && ./p1
rustc +nightly-2026-05-28 -O --edition 2021 -o p2 p2_absorption_necessity_sweep.rs && ./p2
rustc +nightly-2026-05-28 -O --edition 2021 -o p2b p2b_necessity_violations_are_degenerate.rs && ./p2b
```

Each prints its own instrument-validation block and exits nonzero on FAILS.

## rerun/

`42_probes/p3`, `55_probes/p4`, `55_probes/p5` and `56_probes/q1` rebuilt on the pin and diffed
against their committed outputs before any of their counts were argued with. All four
byte-identical; the diffs are the empty files beside the regenerated outputs.

```
cd rerun
rustc +nightly-2026-05-28 -O --edition 2021 -o r42p3 ../../42_probes/p3_one_ended_clamp_versus_two.rs && ./r42p3 | diff - ../../42_probes/p3.out
rustc +nightly-2026-05-28 -O --edition 2021 -o r55p4 ../../55_probes/p4_induced_algebra_grades.rs && ./r55p4 | diff - ../../55_probes/p4_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o r55p5 ../../55_probes/p5_one_bound_divergence.rs && ./r55p5 | diff - ../../55_probes/p5_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o r56q1 ../../56_probes/q1_two_law_families.rs && ./r56q1 | diff - ../../56_probes/q1_output.txt
```

## What each establishes

- **p1** runs four candidate associativity criteria head to head on twelve configurations,
  including `42_probes/p3`'s own four blocks and `55_probes/p5`'s. Adds per-divergence face
  attribution, which neither predecessor took for `42`'s rows.
- **p2** sweeps 4248 configurations per ambient operation to test absorption as a biconditional
  rather than sampling it, reporting sufficiency and necessity violations separately.
- **p2b** characterises p2's 153 multiplication necessity violations.
- **p3** second-reads `55_probes/p4`'s unsigned-saturation semiring at nine widths, gives the
  congruence argument that covers all widths at once, and measures what fractional bits do to it.
- **p4** attributes the fractional collapse to a named factor by running the grid coarsening and the
  range clamping separately, and measures what coherence buys at fold lengths 2 through 6.
- **p5** separates a ladder of algebraic strength from a grading, and measures the precision grading.
- **p6** chases the one-bit gap p5 opened between the predicted and the measured accumulator grade.

`p4_output.v1_failed_assertion.txt` is p4's first run, kept. It reported FAILS because the probe
asserted a divergence at fold length two, where there is only one association order and nothing can
diverge. The measurement was right and the assertion was wrong; both are on disk.

## Added at the resumption (files 58 through 62 had landed)

```
rustc +nightly-2026-05-28 -O --edition 2021 -o p7 p7_the_congruence_condition_per_operation.rs && ./p7
rustc +nightly-2026-05-28 -O --edition 2021 -o p8 p8_the_150_are_the_same_class_as_the_153.rs && ./p8
```

- **p7** answers `62:361-365`'s named open question: the congruence condition, stated per operation,
  which turns `62`'s measured symmetric-range table into a quotient argument. Two earlier runs kept:
  `p7_output.v1_overquantified_ambient.txt` (quantified the congruence over all of the integers
  rather than the reachable ambient, and correctly reported FAILS) and
  `p7_output.v2_predictor_too_broad.txt` (predicted "sign confined" for multiplication, which is
  wrong because only the non-negative half-line is closed under it).
- **p8** checks whether `61`'s widened-sweep count of 150 absorption mispredictions is the same
  collapsed-operation class `p2b` characterised, reproducing `61`'s sweep parameters from its source.

`rerun2/` holds `61_probes/q1`, `61_probes/q2` and `62_probes/p1` rebuilt on the pin and diffed
against their committed outputs before any of their counts were argued with. All three
byte-identical.
