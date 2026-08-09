# Probe outcomes, file 31

Two new probes, both `#![no_std]`, both compiled with the workspace's pinned nightly
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2021 --crate-type lib`). Every claim is a
`const` assertion, so compiling is the check. This file also records four probes carried over from
`30_probes/` and recompiled here rather than trusted: `probe_2_ieee_overflow_falls_out_of_round_first.rs`,
`probe_3_sign_domain_against_sign_indexing.rs`, `probe_4_dither_manufactures_refusals.rs`, and
`probe_5_biased_multiplication_is_closed.rs`, all clean recompiles with no changes made, matching their
committed `30_probes/OUTCOMES.md` entries exactly.

**`probe_1_signed_overflow_is_asymmetric.rs`: WORKS, and converts a three-times-disclaimed gap into a
checked claim.** `30_probes/probe_2` states, in its own header, its own "what this does not show"
section, and file 30's summary, that it says nothing about the negative half of the range. This probe
extends the same model (radix 2, precision 3, emax 2, scale 16) to signed exact values via a
magnitude/sign split, with sign entering in exactly one function (`round_unbounded_signed`, shared with
ordinary in-range rounding) and nowhere in the resolution dispatch, which reads the spec's own separate
`OverRange`/`UnderRange` fields against the same absolute `Direction` markers probe 2 already verified.

The finding is a real asymmetry, not a symmetry check: `TowardPositive` and `TowardNegative` are
absolute directions on the value line, so roundTiesToEven and roundTowardZero (odd-symmetric) mirror
across zero, but roundTowardPositive does not. Deep positive overflow under roundTowardPositive delivers
`+infinity`; deep negative "underflow" under the identical attribute delivers the negative largest
finite, never `-infinity`, because rounding toward positive never selects the more negative of two
candidates. Checked exhaustively over real -9 through 9 for all three attributes (`agrees_signed`), and
pinned as its own assertion independent of the loop (`oracle_rp_signed(-1600) == -LARGEST_FINITE` and
`!= NEG_INF`), with the design's own pipeline reproducing the same result using no sign-conditional code
in the resolution logic itself.

Negative control: `design_pipeline_wrongly_mirrored`, which deliberately reuses the `over`-position
marker at the `under` position (mirroring the wrong end's resolution, the bug this probe exists to
catch), disagrees with the true roundTowardPositive oracle at deep negative overflow. The assertion that
it disagrees is live and compiled.

What it does not show: underflow and subnormals, NaN propagation, and roundTowardNegative /
roundTiesToAway, which were not tested (expected by the same absolute-direction argument to behave
correctly, not verified).

**`probe_2_biased_multiplication_negative_control.rs`: WORKS, and commits a negative control
`30_probes/OUTCOMES.md` narrates but never compiles.** Uses the exact operand pair `30_probes/probe_5`
uses for its primary witness (`A1=4, B1=2, A2=6, B2=4`). The naive adjustment (`A1*A2 = 24`, cross terms
dropped) fails the exhaustive on-grid check over the same window probe 5 checks the correct formula
against, and the first failing pair, found by exhaustive search rather than picked, is `k1=0, k2=1`:
`v1=2, v2=10`, product `20`, `(20 - 8) mod 24 == 12`, not zero. The true formula (`gcd(A1*A2, A1*B2,
A2*B1) = 4`) places the same product correctly (`(20 - 8) mod 4 == 0`), confirmed in the same file so it
stands alone without depending on `30_probes/probe_5` at runtime (the doc comment still cites it for the
formula's provenance).
