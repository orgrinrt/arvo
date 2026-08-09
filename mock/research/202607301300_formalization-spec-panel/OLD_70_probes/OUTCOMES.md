# Outcomes, probe 1: the far-direction resolution, gated on `Specials`

**Hypothesis.** No new `Resolution` constructor is needed to express IEEE overflow-to-infinity.
The same closed vocabulary that already expresses `Clamp` (the near-direction reading: `TowardNegative`
at `OverRange`, `TowardPositive` at `UnderRange`, picking the existing finite neighbour) expresses
overflow-to-infinity under the opposite, far-direction reading, provided the numeral's `Specials` axis
makes the signed infinity representable. Where it does not, the far-direction reading must be
unrepresentable, because there is no point there to round toward.

**Toolchain.** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, matching `rust-toolchain.toml`,
`aarch64-apple-darwin`, no `#![feature(...)]` gates, `--edition 2024`, no other codegen flags. Confirmed
identical to the pinned toolchain the review has cited throughout.

**probe_1_far_direction_positive.rs.** WORKS. Compiles and runs clean. Two positive cases:
`HardwareBinary32` (`Specials = IeeeSpecials`) and `InfCarrying` (`Specials = InfOnly`) both satisfy
`OverflowsToFarPoint<N, TowardPositive>` through the `HasInfinity` marker, and the generic function
`accepts_ieee_overflow_to_infinity::<N>()` type-checks for both. Output:
`probe 1: far-direction resolution well-formed only where Specials: HasInfinity`.

**probe_1b_negative_control_nospecials.rs.** FAILS, as predicted. Identical file with the two positive
calls replaced by `accepts_ieee_overflow_to_infinity::<BoundedFixed>()`, where `BoundedFixed::Specials =
NoSpecials`. Refuses at the exact call site:

```
error[E0277]: the trait bound `NoSpecials: HasInfinity` is not satisfied
   --> probe_1b_negative_control.rs:127:41
    |
127 |     accepts_ieee_overflow_to_infinity::<BoundedFixed>();
    |                                         ^^^^^^^^^^^^ unsatisfied trait bound
```

**probe_1c_negative_control_nanonly.rs.** FAILS, as predicted, for the fourth `Specials` member not
covered by the first two probes. `NanCarryingButFinite::Specials = NanOnly` (carries NaN, no infinity).
Refuses identically:

```
error[E0277]: the trait bound `NanOnly: HasInfinity` is not satisfied
```

**Coverage.** All four members of the `Specials` product are exercised: `IeeeSpecials` and `InfOnly`
accept the far-direction reading (both make the infinity representable); `NoSpecials` and `NanOnly`
refuse it (neither does). This is the whole product, not a sample of it.

**What this does and does not establish.** It establishes that the well-formedness bound this dispatch
proposes (`OverRange = TowardPositive` or `UnderRange = TowardNegative` requires `N::Specials:
HasInfinity`) is expressible in the permitted feature set and enforces exactly the boundary it claims
to. It does not establish that this is the mechanism the review will adopt for the `Quantisation`
trait itself; that is a design call for the review, offered as a suggestion per the standing
instruction to suggest rather than legislate. The toy `Numeral`, `Specials`, `Resolution` and `Direction`
traits here are minimal restatements of the shapes documented in `11_current_shape_draft.md` and
`68_consolidation_seven.md` section 1.16, built to test the one bound in question; they are not a
sketch of the numeral tower itself and make no claim to be.
