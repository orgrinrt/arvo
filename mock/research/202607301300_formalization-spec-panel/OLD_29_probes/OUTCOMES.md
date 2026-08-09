# Probe outcomes, file 29

Both probes compiled with the workspace's pinned nightly (`--edition 2021 --crate-type lib`,
`#![no_std]`, no allocation). Every claim in each file is a `const` assertion, so compiling IS the
check: a false claim fails the build. A negative control was run on each, flipping one assertion to
the value the design's current vocabulary would predict, and both fail with `E0080`, confirming the
machinery is live rather than decorative.

**`probe_1_dither_breaks_the_residue_correlation.rs`: COMPILES.** A memoryless `Direction` rule
delivers the identical error (-3) at two call sites whose exact values share a residue class modulo
the quantum (3 and 23, quantum 10), which is the mechanism of banding stated as a fact about the map
rather than shown as an image. Adding two different externally supplied noise samples before
rounding, on the same rounder, with no new axis, produces different errors (7 and -3) at the two
sites. Negative control: flipping the final inequality to an equality fails to compile
(`error[E0080]: evaluation panicked: assertion failed: ERRORD_I1 == ERRORD_I2`). The mechanism this
demonstrates is narrow and stated as narrow in the file's own doc comment: same undithered error,
different dithered error, from an extra input requiring zero new arvo-side state. It does not claim
statistical independence between error and signal, which is a stronger, ensemble-level claim argued
in the deliverable and not checked here.

**`probe_2_error_shaper_bounds_a_narrow_accumulator.rs`: COMPILES.** An `ErrorShaper` const trait
with a plain associated `State` type (no GAT needed for the first-order case, though the design
already ships one elsewhere in `Quantisation::Fallibility<T>`) compiles under `const_trait_impl`,
`#![no_std]`, no allocation, sizes const. Threaded through a `const fn` fold over five copies of a
sub-half-quantum input (quantum 10, input 2), plain per-step rounding delivers a constant-bias drift
of a full quantum at N = 5 (`PLAIN_TOTAL_ERROR == -10`, the consolidation's own DC-ramp finding at
`26_consolidation_two.md:289-292`, reproduced independently at a five-step model rather than cited).
The shaped fold, same rounder, same inputs, carries the residual forward and returns total error to
zero with the state itself returning to zero (`SHAPED_TOTAL_ERROR == 0`, `SHAPED_FINAL_STATE == 0`).
Negative control: asserting the shaped total error equals the plain total error (-10) fails to
compile. The workload this probe uses (five steps, quantum 10) is a narrow accumulator by
construction, the regime the interior-safety fix does not reach because interior-safety's answer is
to widen the accumulator until no intermediate node can leave the numeral's range at all
(`26_consolidation_two.md:149-164`); shaping is the complementary answer for the case where the
accumulator stays narrow on purpose.

Next steps unblocked: the deliverable's proposal that dither and shaping are distinct mechanisms
(extra pure input vs. carried state) needing distinct homes (a call-site parameter riding the
round-first amendment's extended-grid input; a combinator-owned trait sitting beside, not inside,
`Quantisation`), and the claim that a shaped fold forfeits the parallel-regrouping machinery of
section 1.4 because the feedback is a strict sequential dependency, which this probe's own
implementation exhibits directly (`shaped_fold` cannot be split into independent partial folds and
recombined without the recombination itself becoming a design question, unlike `plain_fold`, which
splits trivially).
