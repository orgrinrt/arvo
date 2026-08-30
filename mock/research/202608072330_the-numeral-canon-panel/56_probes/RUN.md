# 56 probes, run record

Toolchain, passed explicitly on every command per the brief:
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

## Rerun of 55's probes (brief check, done before any new instrument)

Each of `55_probes/{p1,p2,p3}*.rs` compiled with `rustc +nightly-2026-05-28 -O` and run; each
exited 0. The captured outputs are committed here as `p1_one_membership_predicate.rerun.txt`,
`p2_saturate_is_adaptation_wrap_is_domain.rerun.txt`, `p3_encoding_is_a_separate_axis.rerun.txt`.
`diff` against `55_probes/p1_output.txt`, `p2_output.txt`, `p3_output.txt`: all three empty
(byte-identical). So `55`'s committed outputs reproduce on the pin.

## q1_two_law_families.rs

    rustc +nightly-2026-05-28 -O q1_two_law_families.rs -o q1 && ./q1
    exit 0, prints Q1 WORKS

Output committed as `q1_output.txt`. Establishes the 2x2 of adaptation laws (monotone,
distance-minimising) against the coherence law (rho(a op b) == rho(rho a op rho b)), all four
cells inhabited: signed clamp (A yes, C no, 476 chain-divergent triples of 4096), signed wrap
(A no, C yes, 0), unsigned add-only clamp over a nonnegative window (both, 0), the
opposite-bound mutant (neither, 897). Wrap coherent for multiplication too (ring hom); signed
clamp not. The unsigned clamp loses coherence the moment the window admits negative operands,
which is the reachable-floor condition.

Instrument-can-fail: every checker reports false on at least one row (monotone and nearest fail
on wrap and the mutant; coherence fails on signed clamp and the mutant), so no checker is
structurally green.

## q2_affine_membership.rs

    rustc +nightly-2026-05-28 -O q2_affine_membership.rs -o q2 && ./q2
    exit 0, prints Q2 WORKS

Output committed as `q2_output.txt`. The affine membership predicate (step, bias, bounds)
matches direct enumerations at bias zero and bias half-step (16 and 16 values); the
bias-dropped mutant predicate is detected; round-to-nearest onto the biased grid keeps all
four adaptation laws (total, retraction, monotone, distance-minimising) exhaustively over a
window past both bounds; the wrong-target mutant rounder fails retraction; exact sums of two
biased-grid points land on the biased grid 0 of 256 times and sit exactly half a step from it
every time (systematic tie); the biased grid contains neither zero nor one.

## q3_signed_encoding_trade.rs

    rustc +nightly-2026-05-28 -O q3_signed_encoding_trade.rs -o q3 && ./q3
    exit 0, prints Q3 WORKS

Output committed as `q3_output.txt`. Raw order agrees for offset binary only (of twos, offset,
scrambled); the raw 4-bit adder is correct on 256 of 256 pairs for two's complement, 0 of 256
for offset binary with the defect a constant 8 mod 16, 26 of 256 for the scrambled control;
the sorted correspondence (the unique monotone bijection between two finite total orders) is
exactly offset binary, and the K=7 mutant breaks it; the unsigned identity encoding holds both
properties at 256 of 256.

## Honesty notes

All three probes passed on their first complete run; no instrument defects were found and
repaired during this file's work, so unlike `08_probes/RUN.md` there is no kept-defect trail
here. The can-fail demonstrations are the mutants and the known-failing rows named above, not
a repair history. All counts are at 4 bits (q1, q3) or one grid geometry (q2, step 1/4, bias
1/8, scale 2^5); width transfer is argued, not probed.
