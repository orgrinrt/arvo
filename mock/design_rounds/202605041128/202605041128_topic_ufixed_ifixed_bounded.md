**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** arvo (UFixed, IFixed)
**Source topics:** Round 6 (#324) PR #48 reviewer NIT 5; new task #325

# Round 7 Topic 1: UFixed and IFixed Bounded blanket forward

This topic covers a single substrate-completeness gap surfaced by the Round 6 senior review on PR #48. UFixed and IFixed do not impl `Bounded`. Round 6's `ufixed_const_arith.rs` and `ifixed_const_arith.rs` documented the gap inline (Apply-time deviation 3) and left MIN/MAX assertions skipped. The blanket forward is mechanical: same predicate-bundling pattern Round 5 (#306, #315) used for Identity / ConstPartialEq / ConstEq / ConstBitEq / ConstOrd / ConstDefault; the inner `Bits<W, S, Sign>` already impls `Bounded` (see `arvo-storage/src/bits.rs:101`).

The substrate stays toolbox-not-policer: consumers that previously had to reach for `Bits` to project MIN/MAX through transparent unwrap can now project through the typed UFixed / IFixed surface directly. No behaviour change. No public API removal. No new arithmetic.

## Context: why this round is small

Round 6 closed audit-specified F26-F30 / F35-F36 confirmation-only smoke tests. NIT 5 was the only finding the reviewer flagged as a genuine substrate observation (versus cosmetic NITs that were unactionable post-CLOSED). The follow-up was tracked unfiled at Round 6 close; this round files it as task #325 and ships it.

Per `no-legacy-shims-pre-1.0.md`: the missing impl is added clean. No deprecation alias, no shim. Per `arvo-toolbox-not-policer.md`: this exposes a substrate capability the inner Bits already has, restoring symmetry.

## Decisions

### Decision 1: UFixed gains const Bounded blanket

**File:** `mock/crates/arvo/src/ufixed.rs`

Add `impl<const I: IBits, const F: FBits, S: Strategy> const Bounded for UFixed<I, F, S>` with the same single-predicate cycle-avoidance shape Identity uses (see ufixed.rs:60-67). MIN and MAX project through the inner `Bits<{ ufixed_bits(I, F) }, S>` Bounded impl, then wrap into `Self`. Bound list mirrors the Identity blanket: `S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>` and `Bits<{ ufixed_bits(I, F) }, S>: [const] Bounded`. Add `Bounded` to the `use crate::strategy::{...}` import line.

### Decision 2: IFixed gains const Bounded blanket

**File:** `mock/crates/arvo/src/ifixed.rs`

Symmetric to Decision 1 with `Signed` sign axis: `impl<const I: IBits, const F: FBits, S: Strategy> const Bounded for IFixed<I, F, S>`. Project through `Bits<{ ifixed_bits(I, F) }, S, Signed>` Bounded. Add `Bounded` to the `use crate::strategy::{...}` import line.

### Decision 3: Round 6 ufixed and ifixed const tests gain MIN/MAX assertions

**Files:** `mock/crates/arvo/tests/ufixed_const_arith.rs`, `mock/crates/arvo/tests/ifixed_const_arith.rs`

Round 6 deviation 3 noted UFixed did not impl Bounded; the test files left MIN/MAX out. With Decisions 1 and 2 landing, append `const _U8_MIN: U8Hot = <U8Hot as Bounded>::MIN;` and the symmetric MAX, and likewise for I8Hot / I16Warm / I32Precise. Also a runtime assertion confirming `MIN.to_raw()` and `MAX.to_raw()` match the inner `Bits` projection. The intent is to verify the const-callable path lands; runtime parity is the regression sentinel.

## Sketches needed

None. The Identity blanket on UFixed (ufixed.rs:60-67) and IFixed (ifixed.rs:62-69) already compiles under the same predicate-bundling shape. Bounded uses the same inner-trait-on-Bits projection. Per `cl-claim-sketch-discipline.md`, sketches are required when there is "trait-solver-cycle risk, generic-const-expr risk, repr(transparent) layout risk, or any other 'does rustc actually accept this' question". This round triggers none: the predicate shape is exercised in Round 5 and Round 6 already.

## Lock criterion

This topic locks (frozen) when:

1. Decisions 1 and 2 have file paths and the predicate-bundling pattern named.
2. Decision 3 has the existing test files named and the surface to add enumerated.
3. The doc CL is opened referencing this topic.

## Out of scope

- Test infrastructure changes (the existing const tests stay; only assertions extend).
- Doc tmpl edits naming new traits (UFixed and IFixed already document Bounded availability through the canonical const-surface bullet list per Round 6 deviation 3 follow-up).
- Other Bounded gaps elsewhere in the substrate (none surfaced; if one does, it gets a separate round).

## Cross-references

- `mock/crates/arvo/src/ufixed.rs:60-67` (Identity blanket; pattern reference).
- `mock/crates/arvo/src/ifixed.rs:62-69` (Identity blanket; pattern reference for signed).
- `mock/crates/arvo-storage/src/bits.rs:101` (Bits Bounded impl that gets forwarded).
- `mock/crates/arvo-strategy/src/arith.rs:418` (Bounded trait declaration).
- `.claude/rules/cl-claim-sketch-discipline.md` (sketch-first criterion not triggered here).
- `.claude/rules/arvo-toolbox-not-policer.md` (restoring substrate symmetry consumers can already reach via Bits).
- `.claude/rules/no-legacy-shims-pre-1.0.md` (clean addition; no deprecation aliases).
- Task #325 (this round closes it).
- PR #48 reviewer report (Round 6 NIT 5).
