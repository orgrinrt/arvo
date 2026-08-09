# Probe outcomes, file 67

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` at the repo root. Every probe was compiled and run from inside the repo tree; a
bare `rustc` outside the tree resolves to stable and answers `E0554` rather than the question.

No probe enables any `#![feature(...)]` gate. That is deliberate for probe 7, where the point is that
the mechanism is permitted, and it is incidental elsewhere.

Reproduce: `rustc --edition 2021 -o /dev/null <probe>.rs` for the compile-fail probes,
`rustc --edition 2021 -O -o /tmp/x <probe>.rs && /tmp/x` for the rest. `model.rs` is copied verbatim
from `66_probes/model.rs` so probe 3's reproduction of file 66's count is against file 66's own model
rather than against a re-implementation that could differ.

| probe | what it asks | outcome |
|---|---|---|
| `probe_1_statement_two_is_illtyped.rs` | type the three crossing maps honestly and write statement 2 out | **FAILS TO COMPILE**, `E0308`, expected `Value` found `Val`. rustc's own suggested fix (`Value { inner: ... }`) is the unchecked coercion the design silently performs |
| `probe_2_the_escape_is_a_family.rs` | extend file 66's leak matrix along the `Specials` axis it held fixed | PASSES. 6 of 8 cells leak, not 1. Three of four `Specials` members leak under an IEEE layout; two of four leak under the OCP layout |
| `probe_3_no_encode_side_repair.rs` | is `encode . quantise . decode` total where `encode . decode` is not | PASSES. The quantiser refuses on **every** escaping datum, 1/9, 4/21, 9/297 and 108/2997. No encode-side repair exists; the datum-side one is forced. Reproduces file 66's 108/2997 independently |
| `probe_4_what_the_layers_key_on.rs` | do two faces for one literal project to one encoding, and does the projection erase the site | PASSES. Same encoding type, same law result, different `DISPLAY`, and nothing else observable |
| `probe_5a_encoding_keyed_refuses_two_values.rs` | what does an encoding-keyed refusal name | **FAILS TO COMPILE**, `E0308`, names `BPos<H, I<H>>` vs `BPos<H, O<H>>`. The expansion, not the face |
| `probe_5b_face_keyed_refuses_one_value.rs` | what does a face-keyed refusal cost | **FAILS TO COMPILE**, `E0308`, `Tagged<Third>` vs `Tagged<OneThird>`. Two names for one value, so the refusal is false about the numbers |
| `probe_5c_face_cannot_reach_numeral_position.rs` | can a face sit in a numeral position | **FAILS TO COMPILE**, `E0277`, `Third: Bias` is not satisfied. The seal forbids it, so the projection is the only route |
| `probe_6_adjustment_needs_its_own_door.rs` | does `Adjustment` share `Bias`'s door | PASSES. Under one shared door the two exchange silently and the value moves from 11 to 84.33. The reduction generator is shared by both arrangements |
| `probe_6b_role_swap_refuses.rs` | does the exchange refuse under two doors | **FAILS TO COMPILE**, `E0277` twice, `Bia<7,1>: Adjustment` and `Adj<1,3>: Bias` both unsatisfied |
| `probe_7_uniformity_fails_without_specialization.rs` | can a property's truth value move across widths with the bans in force | PASSES. TRUE at 8 bits, FALSE at 9, one parametric body, no `specialization`, no `TypeId`, no gate. The container projection is what moved |

## Negative controls, listed so the passes are not vacuous

- Probe 2: under `IeeeSpecials` with the IEEE layout, zero escape over a non-empty 128-datum set, so
  the escape is not an artifact of counting reserved patterns.
- Probe 3: the quantiser is the identity on all 2,701 values of `V(N)` at `r=10, p=3`, so "refuses on
  every escaping datum" is a statement about the data and not about a broken quantiser.
- Probe 4: `Half` denotes a genuinely different value and the encoding-keyed law refuses it (probe
  5a), so the unification in 4.1 is doing work rather than accepting anything.
- Probe 6: the correct arrangement gives the same number under both doors, so the two arrangements
  differ only in what they refuse.
- Probe 7: the same container class gives the same answer at 9 and 16 bits; and 40,000 doubled in a
  u16 container does wrap, so the u16 arm is doing arithmetic rather than being unreachable.

## What is measured versus reasoned

Every row above is a compile result or an assertion that ends a run. There is no wall-clock number in
this probe set at all, so nothing here needs the bench harness. The design proposals in the file
itself (the `Crossing` obligation's shape, the `container-class` transfer coordinate, the keying rule)
are reasoned from these results and are labelled as reasoned where they appear.
