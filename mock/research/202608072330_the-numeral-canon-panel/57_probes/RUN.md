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
