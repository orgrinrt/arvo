# Probes behind the reading of the numeral-contracts shape

**Outcome:** WORKS as an instrument, and what it measures refutes parts of the shape. Each probe is listed
with its own result: WORKS, or FAILS WITH the error that is the finding. Every result is in `outputs/`.
`run.sh` rebuilds them all under the repository's toolchain (`outputs/toolchain.txt`,
`nightly-2026-05-28`). Each crate here is standalone, with its own `[workspace]` table, and builds into the
shared `/home/user/target`.

These are spikes. Each one checks one thing. Names, arities, field orders and trait shapes in them were
picked to reach the check and are not design proposals. The one construction offered as a design
direction (`monotone-policy-marker`) is labelled as that, and it is one instance.

The reading these serve is `mock/research/202611192100_numeral-contracts-shape/READING.md`.

## 1. The law regions on the landed value (`law-regions`)

**Outcome:** WORKS. `outputs/law-regions.txt` has 240 rows.

The sweep is exhaustive over every member triple of `Num` under `Quiet`, built through `adapted` on the slot
grid. It covers widths 1 to 8, fraction widths 0 to W (all of them at W up to 4, and the cells
{0, 1, mid, W} above that), both signednesses, `Wrap` and `Saturate`, and `TowardZero` and `HalfEven`. For
each cell it counts the failures of additive associativity, multiplicative associativity and monotonicity of
addition (`a <= b` implies `a + c <= b + c`), and prints the first witness.

Calibration against the registry, where the cells overlap. The instrument reproduces the panel's own counts
exactly:
- signed saturating addition: 952 failing triples at W = 4, and 4,177,792 at W = 8
  (`law::additive_associativity_under_saturation`);
- signed saturating multiplication: 28, 160, 780 and 3516 at W = 3, 4, 5, 6
  (`law::multiplicative_associativity_under_saturation`).

What it finds outside those cells:
- signed saturating addition is associative at W = 1, at both fraction widths;
- signed saturating multiplication is associative at W = 1, and also at (W, F) = (2, 2) and at (3, 3) under
  `TowardZero`;
- wrapping multiplication is associative at F > 0 in several cells: (1, 1) in every combination of sign and
  rounding, (2, 2) signed under both roundings, and (3, 3) signed under `TowardZero`. The last of these sits
  inside the fails region of `law::multiplicative_associativity_under_wrapping` (W in 3..=7, F > 0, both
  signednesses). That region carries no rounding axis, and under `TowardZero` that cell holds;
- monotonicity of addition holds in every saturating cell and fails in every wrapping cell, W = 1 included.
  No law row states this.

This is one instrument. The calibration makes it more than a one-off, because it agrees with the panel's
independent probes on every cell where both ran. The new cells (W = 1, F = W) stand on this instrument
alone.

## 2. Constness through the contracts, under arm three

Built against the first sketch's `arm-const`. `arm-const-copy` is that crate's `src/lib.rs` byte for byte
(`outputs/arm-const-copy-diff.txt`, diff exit 0), with its manifest made standalone, because cargo would not
resolve the original's inherited package fields two path hops away.

| Probe | Consumer gate | What it writes | Result |
|---|---|---|---|
| `concrete-const-call-no-gate` | none | `<I8 as Sum>::sum(..)` in a const item | FAILS WITH E0658 "cannot call conditionally-const method ... in constants" |
| `gated-const-eval` | `const_trait_impl` | the generic `sum_in_const` and the concrete call, both evaluated in const items at `Num`, read back by tests | WORKS: 2 tests pass, and 100 + 100 saturates to 127 at build time |
| `ungated-call-of-gated-generic` | none | the gated generic `const fn` from `gated-const-eval`, called at `Num` in a const item | FAILS WITH E0658 "cannot call conditionally-const function `sum_in_const::<..>` in constants" |
| `ungated-runtime-call-control` | none | the same call, outside a const context | WORKS: 1 test passes. The control: the refusal above is the const context and nothing else |
| `opt-on-gated-control` | `const_trait_impl` | arm two with its feature on, a `[const]` bound evaluated at `Num` | WORKS: 1 test passes |

What this establishes. Under arm three, a consumer with no gate can evaluate nothing at build time through
the contracts. That holds for a concrete trait call and for an arvo-side generic `const fn` alike. Only the
inherent `const fn`s give build-time evaluation, and they give it in every arm. A gated crate does get real
build-time evaluation through `const impl Sum for Num`. The first sketch never evaluated that; it only
declared a generic function.

## 3. The monotone marker's selection

| Probe | Result |
|---|---|
| `monotone-saturate-control` | WORKS. An outside policy `Pinned` with `POLICY = Policy::Saturate` is admitted by `Sole`, and its sums equal `Saturate`'s at all 65,536 pairs at W = 8, saturating ones included |
| `monotone-outside-policy` | FAILS WITH E0271 "type mismatch resolving `<Adapt<TowardZero, Pinned> as Adaptation>::Overflow == Saturate`". The shape's selection refuses a value that saturates |
| `monotone-policy-marker` | WORKS (design direction, one instance). A policy-side marker trait, implemented by `Saturate` and opted into by `Pinned`, gives both the numeral marker |
| `monotone-policy-marker-wrap` | FAILS WITH E0277 "the trait bound `arvo_format::overflow::Wrap: MonotoneCompletion` is not satisfied". The intended refusal |

## 4. `Constant<const V: i128>` (`constant-bound-check`)

**Outcome:** FAILS WITH E0080 under `cargo build`, and builds clean under `cargo check`
(`outputs/constant-bound-check.txt`). The impl has to cover every `V`, so `T: Constant<300>` is satisfied at
an 8-bit type for which 300 names no member. The refusal arrives only after monomorphisation, through
`Num::constant`'s own assertion. It is a build-time refusal and not a run-time one, and the bound carries no
information.

## 5. A const parameter of the stack's own type (`adt-param-lib`, `adt-param-consumer`)

**Outcome:** WORKS. `adt-param-lib` takes `adt_const_params` and declares `Constant<const V: Index>` over a
private-field struct. `adt-param-consumer` names no gate. It writes a bound `T: Constant<{ Index::at(3) }>`
and a const item over it, and its test reads back member 3. So, on this toolchain, the bare `i128` at the
const parameter is a choice made for ergonomics. No gate forces it.

## 6. The debug check under the contract (`abort-oracle`)

**Outcome:** WORKS, 3 tests pass. Through `arm-plain`'s `Sum`, `Num<_, Quiet>` gives the completion
(100 + 100 = 127). `Num<_, Abort>` panics in a debug build on the same pair, which `catch_unwind` observes,
and returns normally on an in-range pair. So "total" holds for `Quiet`, or for a release build. It does not
hold for every `C` the impl covers.

## 7. The whole workspace suite

`outputs/workspace-suite-summary.txt` is the `test result` lines of
`cargo test --manifest-path mock/Cargo.toml --workspace` at the branch head before this commit: 1072
passed, 0 failed, 1 ignored, exit 0.
