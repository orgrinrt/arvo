# 58_probes: how to run, and what each probe is for

Toolchain `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), passed explicitly.
A bare `rustc` outside the repository tree resolves to stable.

```
rustc +nightly-2026-05-28 -O --edition 2021 -o p1 p1_rounding_mode_versus_truncation.rs && ./p1
rustc +nightly-2026-05-28 -O --edition 2021 -o p2 p2_multiplicative_grade_does_not_collapse.rs && ./p2
```

Each prints its own instrument-validation block and exits nonzero on FAILS. Compiled binaries are
not committed; only sources and outputs.

## What each establishes

- **p1** answers `57` section 7's flagged open probe directly: does round-to-nearest restore the
  semiring `57_probes/p3` found collapsing at F > 0. It does not. Violation counts move (mostly
  down, sometimes up, per row) but never to zero on any row measured.
- **p2** asks whether the multiplicative fold has a grading shaped like the additive one `57_probes/p6`
  measured (a small, n-independent constant). It does not: section 1 isolates the pure fractional
  axis (no intermediate range clamp) and finds the required guard grows linearly in fold length,
  saving exactly one rescale's worth of bits (constant in n) below full precision, never more.
  Section 2 keeps a bug from the first run of this probe (an intermediate range clamp at full guard
  width, which should be impossible to diverge under and did) and shows it as a second, independent,
  additive-shaped mechanism (range-clamp reachability, job one's absorption criterion again) that
  compounds with the fractional axis rather than substituting for it.

`p2_output.v1_bug.txt` is p2's first run, kept. It asserted `min_w <= full_w` and the assertion
failed, correctly: min_w exceeded full_w because the accumulator was being range-clamped at every
step, even at full fractional precision, which introduces a second lossy step the comment claimed
was absent. The measurement was right and the docstring's claim about what the code tested was
wrong; both are on disk, and section 2 of the corrected probe measures the thing the bug actually
found rather than discarding it.

## Re-run before argued with

Every count this file's parent (`58_wronski_the_fraction_boundary.md`) takes from `57_probes` is a
count regenerated on this pin, diffed against the committed output.

```
cd rerun
rustc +nightly-2026-05-28 -O --edition 2021 -o r57p3 ../../57_probes/p3_semiring_across_widths_and_scales.rs && ./r57p3 | diff - ../../57_probes/p3_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o r57p4 ../../57_probes/p4_which_factor_breaks_and_what_coherence_buys.rs && ./r57p4 | diff - ../../57_probes/p4_output.txt
rustc +nightly-2026-05-28 -O --edition 2021 -o r57p2 ../../57_probes/p2_absorption_necessity_sweep.rs && ./r57p2 | diff - ../../57_probes/p2_output.txt
```
