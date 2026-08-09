# Probe outcomes, file 30

Five probes, all `#![no_std]`, all compiled with the workspace's pinned nightly
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2021 --crate-type lib`).
**No unstable feature is used by any of them**, which is itself part of probe 1's
result. Every claim is a `const` assertion or a `const fn` exhaustion, so compiling
is the check; each probe has a negative control that was run and fails with `E0080`.

**`probe_1_partial_override_of_a_preset.rs`: WORKS, and answers a question nobody
in the review had asked.** A generic parameter default of the form
`<Warm as Policy>::Quantisation` resolves. So a preset can be a partially applied
generic whose untouched members project out of the preset it derives from, and a
consumer wanting `Warm` with a different tie direction writes `Like<Warm, ToOdd>`
rather than declaring a marker and restating all five associated types. Zero
overrides reproduces the parent's five-member fingerprint exactly; one override
reproduces the hand-written copy's. The limit is recorded in the probe: generic
arguments are positional, so the first divergence is free and a later-positioned
one costs spelling the earlier positions. Negative control: asserting the override
equals the parent fingerprint fails with
`error[E0080]: evaluation panicked: assertion failed: BY_OVERRIDE == WARM`.

**`probe_2_ieee_overflow_falls_out_of_round_first.rs`: WORKS, and shows files 27
and 28 compose into IEEE's actual overflow rule.** Model: radix 2, precision 3,
emax 2, scaled by 16. File 28's round-first amendment plus file 27's `Specials`
member reproduce IEEE 754-2019 (7.4) exactly for three rounding attributes,
checked exhaustively over real 1 through 9 against three oracles written from the
standard rather than from the pipeline: roundTiesToEven (overflow at the boundary
`2^emax(2 - 2^-p)`, derived from the format rather than written as a literal),
roundTowardZero (largest finite, never infinity), roundTowardPositive (infinity
for anything above the largest finite, no boundary). The tie at the boundary
resolves upward because the largest finite is an odd multiple and the first value
past emax is an even one, so the ties-to-even rule delivers infinity at exactly
the magnitude IEEE says it does, with no rule of its own. Removing specials from
the identity leaves the identical pipeline delivering the largest finite, which is
SystemC's and MATLAB's behaviour: one identity member decides it and no `Policy`
member changes. It also shows IEEE's mode coupling carries information: pairing
ties-to-even in range with the largest-finite over-range resolution disagrees with
roundTiesToEven, so the `conv-ieee754` alias rows are not restating a default.
Correction to file 27 recorded in the probe: "past the top is unreachable" once
infinity is representable (`27:188-193`) is false; the over-range position stays
inhabited and the midpoint that decides it is on the unbounded grid, not between
the largest finite and infinity, where no midpoint exists. Negative control: the
mismatched coupling assertion fails with `E0080`.

**`probe_3_sign_domain_against_sign_indexing.rs`: WORKS, and finds an internal
conflict between two sections of file 28.** Model: magnitude precision 3, sixteen
data, four indexings, three domains. Two results.

First, the sign axis is two independent axes. The value domain (`NonNegative`,
`Symmetric`, `AsymmetricLow`) and the indexing (`Unsigned`, `TwosComplement`,
`SignMagnitude`, `OnesComplement`) are not in bijection: the symmetric domain is
served by all three signed indexings and two's complement serves two different
domains, so naming one does not name the other, which is the test for whether
bundling them loses information. File 28 §3 bundles them into one three-instance
`Sign` axis on the identity side (`28:186-192`), which puts the datum-level fact
(two zeros) on the value-level side that file 28 §1 and file 27 had just cleared
of encoding parameters.

Second, file 28 §1's crossing contract is false as stated. It says "decode after
encode is the identity on data, and encode after decode is the identity on bit
patterns" (`28:84-86`); file 28 §2 then names three entrances to non-injectivity
(`28:119-138`). Checked: the first holds always, the second fails for
sign-magnitude and for ones' complement, with the named witness that
`encode(decode(0b1000))` is `0b0000`. What does hold is that encode-after-decode
is **idempotent**, so the correct crossing contract is a section-retraction pair
with a canonicalisation, which is what IEEE calls a canonical encoding and what
decimal calls a preferred exponent. Negative control on the failing round trip:
`E0080`.

A third result fell out: SystemC's `SC_SAT_SYM` is not a saturation mode. With the
axes split, the same `TowardNegative` clamp delivers -8 over an asymmetric domain
and -7 over a symmetric one, so the symmetric-saturation mode is a numeral choice
and one fewer thing in `Policy`.

**`probe_4_dither_manufactures_refusals.rs`: WORKS, and finds that file 29's
dither entry point and the `Refuse` resolution do not compose.** Model: unsigned,
quantum 2, top at 30 and exactly representable, `Precise` (nearest-ties-even in
range, refuse out of range). The undithered path returns the top unchanged; one
quantum of positive dither on that same exactly representable input refuses. File
29 says only that at the ends "the ordinary `OverRange` resolution takes over
exactly as it would for any other value" (`29:96-97`), which for `Precise` means a
caller's choice to decorrelate silently makes the computation fallible. Every
exact value within one dither amplitude of either end is affected. The candidate
fix is checked and works: confining the perturbed value to the numeral's range
before quantising restores totality over the whole range and every admissible
noise draw at one and two quanta of amplitude, while leaving the interior
mechanism intact.

A correction to file 29 fell out of the same run. Its claim that "two positions
with the same value modulo the quantum receive the identical error, always, from
every `Direction` the contract can express" (`29:69-72`) is false for `ToEven` and
`ToOdd`, where the tie is broken on the quotient's parity, so the error is
periodic with period `2Q` rather than `Q`. Its own probe used nearest-ties-away,
where the period is `Q`. The conclusion survives (a memoryless rule cannot
decorrelate, whatever its period) but the stated mechanism is off by a factor of
two, and the two directions it is wrong about are the ones `Warm`, `Cold` and
`Precise` all use.

**`probe_5_biased_multiplication_is_closed.rs`: WORKS, and closes an item the
consolidation lists as open.** The consolidation records that neither `Adjustment`
nor `Bias` is closed under multiplication and offers two candidate fixes
(`26_consolidation_two.md:326-331`); file 28 proposes the rational-pair adjustment
(`28:319-328`), which addresses the adjustment half only. The bias half has a
closed form. For `v1 = A1*k1 + B1` and `v2 = A2*k2 + B2` the cross terms are real
and all lie in one lattice, so the product numeral is

    adjustment  L = gcd(A1*A2, A1*B2, A2*B1)
    bias        B1*B2

Checked exhaustively over the full window product for six operand pairs including
a MATLAB-shaped slope-and-bias pair at scale 1000. With both biases zero the
formula returns `A1*A2` and bias zero, so the multiplicative half's verified width
adder is the special case rather than a second rule that has to agree with this
one. Negative control: replacing the formula with the naive `A1*A2` (dropping the
cross terms) fails on the biased case with `E0080`, so the cross terms are
load-bearing rather than an artefact of the numbers chosen.

What it does not claim: that `L` is the finest numeral containing the product set
(containment is what closure needs, and a finer-than-necessary quantum is the safe
direction), nor the product's width, nor the `FullRange` case, which needs file
28's rational pair before `A1*A2` and the gcd are expressible at the type level at
all.
