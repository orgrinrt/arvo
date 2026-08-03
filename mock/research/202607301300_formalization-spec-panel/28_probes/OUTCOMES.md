# Probe outcomes, file 28

Both probes compiled with the workspace rustc, `--edition 2021 --crate-type lib`, no feature gates,
`#![no_std]`. Every claim in each file is a `const` assertion, so compiling IS the check: a false
claim fails the build. A negative control was run on probe 1 (flipping the SubstituteZero assertion
to the spec's claimed direction) and fails with
`error[E0080]: evaluation panicked: assertion failed: associative(2)`, confirming the machinery is
live rather than decorative.

**`probe_1_unsigned_blanket_refuted.rs`: COMPILES, and refutes the spec's unsigned blanket.** The
faithfulness derivation at `202607301200_topic.the-formalization-spec.md:210-216` grants `AddAssoc`
to unsigned addition under **every** `Resolution` pair, on the reasoning that one end is unreachable
"whatever it does there". Checked exhaustively at a 5-bit model: ReduceModulo, clamp and Refuse
(under Kleene equality) are associative; **SubstituteZero is not**, witness
`(25 + 10) + 5 = 5` against `25 + (10 + 5) = 0`. SubstituteZero is SystemC's `SC_SAT_ZERO`, so the
false cell is a mode the vocabulary exists to express. The reachable end's rule matters; the correct
condition is per-resolution, not per-signedness. A refinement fell out of the same run: unsigned
clamping IS associative (absorption, needs non-negative operands), so the consolidation's "a
retraction gives neither in general" is signedness-dependent, and the structural classification of
the recovery map alone cannot see that.

**`probe_2_classify_after_rounding.rs`: COMPILES, and shows the five positions are assigned in the
wrong order.** IEEE 754-2019 (7.4) rounds first, as though the exponent range were unbounded, and
declares overflow only of the rounded result; SystemC and MATLAB fi likewise quantise first and
apply the overflow mode to the quantiser's output. The spec positions the exact value. The two
orderings disagree on the band past the largest representable but within half a quantum of it:
there, classify-first refuses (or wraps) while every test standard returns the largest finite with
no overflow event. Confirmed at a model: `Refuse` diverges at the band (REFUSED against 28), clamp
agrees everywhere (exhaustively checked, which is why the shipped clamping presets never surface
the defect), and classify-first `ReduceModulo` is not even well-defined on the band (the reduction
of a band value lands back in the band). The band is empty for same-format addition (exact sums are
on-grid) and inhabited for multiplication, division, mixed-format addition and every float
operation.

Next steps unblocked: the deliverable's amendment that quantisation is "round on the unbounded
grid, then resolve the rounded result against the range", and the correction of the faithfulness
derivation to a per-resolution condition.
