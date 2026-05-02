# Round 202605021800 — substrate const-trait foundation lifts

**Date:** 2026-05-02
**Phase:** TOPIC
**Scope:** arvo-strategy / arvo-storage / arvo-bits-contracts / arvo (facade)
**Source topics:** post-202605021600 audit findings (Expert A architectural dogfooding + Expert B const-trait completeness, both 2026-05-02)

## Context

Three rounds closed today on PR #42 (`feat/usize-const-arith`) shipped substantial const-trait surface — strategy markers, container projections, UArith/IArith, BitPrim, BitAccess/BitSequence/BitLogic on Bits, Bounded/Identity, USize/Cap inherent ops, UFixed/IFixed/Float Identity, Mask Bounded/Identity, BitPrim::is_zero substrate bridge.

Two parallel domain-expert audits ran post-202605021600 and identified 58 substrate gaps. Reports persisted at `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md` (18 findings) and `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md` (40 findings). The findings decompose into six rounds; this round is the first.

This round covers ONLY foundational const-trait lifts and substrate-bridge introduction. No consumer-code dogfooding (deferred to later rounds in the same PR). No widening of existing public API surface. Goal: every downstream lift in subsequent rounds (Mask&lt;W&gt; over Bits, MultiContainer BitPrim, Bits&lt;const N: Width&gt;) has its const-trait predicates already in place.

## Decisions to record (single-topic, single-round)

### Decision 1: Lift sealed marker traits to `pub const trait`

Per Expert B Findings 1, 2, 11, 12, 15, 16. The traits below carry zero method bodies (or trivially-pure bodies); the body-purity test is met. Lifting to `pub const trait` enables `[const]` propagation through every where-clause that bounds on them. Without this, subsequent rounds cannot land their const surfaces.

- `arvo-strategy/src/lib.rs:148` — `Signedness` (sealed marker)
- `arvo-strategy/src/container.rs:71-95` — `BitsContainerFor<const N, Sign>`
- `arvo-strategy/src/axes.rs:41-44, 77-80, 114-117, 159-166` — `OverflowPolicy`, `ContainerWidth`, `StorageLayout`, `HasAxes`
- `arvo-strategy/src/cross_strategy.rs:49-54` — `CrossStrategyOp` (currently missing `const` vs sibling `Resolve`)
- `arvo/src/float.rs:26-45` — `Ieee` + supertrait-bound on `Identity` (drops `Ieee::ZERO` / `Ieee::ONE` as redundant — they project through `<F as Identity>::ZERO/ONE`)
- `arvo/src/markers.rs:23, 28, 36, 49, 56` — `IntegerLike`, `FractionLike`, `BitPresentation`, `FloatLike`, `BoolLike`
- `arvo-storage/src/platform.rs:267-277` — `AsBool`

All `impl X for Y` blocks for these traits are lifted to `impl const X for Y` in the same round (no orphan; substrate-owned traits, substrate-owned impls).

### Decision 2: Define `ConstEq` and `ConstDefault` substrate bridges in `arvo-strategy`

Per Expert B Findings 7, 9. Stdlib `core::cmp::PartialEq` and `core::default::Default` are not const-stable. Several substrate surfaces depend on `T == U` semantics in `const fn` bodies (UArith division-by-zero check, Predicate family, sorts in algorithm crates) or on `Default::default()` (Mask::empty, BitMatrix::empty, UFixed/IFixed default).

Bridge shape:

```rust
pub const trait ConstEq {
    fn const_eq(&self, other: &Self) -> Bool;
    fn const_ne(&self, other: &Self) -> Bool {
        Bool(!self.const_eq(other).0)
    }
}

pub const trait ConstOrd: ConstEq {
    fn const_cmp(&self, other: &Self) -> Ordering;
    fn const_lt(&self, other: &Self) -> Bool;
    fn const_le(&self, other: &Self) -> Bool;
    fn const_gt(&self, other: &Self) -> Bool;
    fn const_ge(&self, other: &Self) -> Bool;
}

pub const trait ConstDefault {
    fn const_default() -> Self;
}
```

Place: `arvo-strategy` (sibling of `Bounded` / `Identity`). `Ordering` reuses `core::cmp::Ordering` which is const-constructible on nightly per `feature(const_cmp)`; if not yet stable, ship `pub enum ConstOrdering { Less, Equal, Greater }` as the substrate-owned alternative. Decision: use `core::cmp::Ordering` if const-constructible on rustc 1.96.0-nightly; otherwise use substrate-owned `ConstOrdering` enum. Probe at SRC-PLAN time.

`ConstEq` impls land in this round on every primitive-wrapper substrate type (USize, Cap, Bool, Bits, UFixed, IFixed, FastFloat, StrictFloat, Mask64, Mask256, NodeId, MetaCarrier, IBits, FBits, Width). `ConstOrd` impls land in the same round on the totally-ordered subset.

`ConstDefault` impls land on every type currently impl'ing `Default` whose body is pure (`Self::ZERO` / `Self(0)` / `Self::new()` shapes). The non-const stdlib `Default` impls stay alongside for boundary coverage.

`ConstFrom` / `ConstTryFrom` / `ConstDeref` / `ConstAsRef` / `ConstTry` / `ConstHash` bridges (Expert B Findings 6, 8, 21, 24, 40) are deferred to Round 5 / Round 6; they cascade after this foundation is in place.

### Decision 3: Collapse `UPrimConst` / `IPrimConst` into `Bounded` + `Identity` + new `SignedIdentity`

Per Expert A Finding 3. Two parallel const-trait surfaces ship for the same axis: `<u8 as Bounded>::MAX` and `<u8 as UPrimConst>::MAX` are the same value reached two different ways. Pick one: `Bounded` + `Identity` are the canonical surfaces (already wired into Mask/UFixed/IFixed/Bits per round 202605021600).

Concrete:
- Define `pub const trait SignedIdentity: Identity { const NEG_ONE: Self; }` in `arvo-strategy`.
- Per-signed-primitive `impl const SignedIdentity` (i8..i128, isize) wrapping the existing `IPrimConst::NEG_ONE` body.
- Rewrite `arvo-strategy/src/arith.rs:120, 148, 175, 203` (the four internal `UPrimConst::ZERO` references) to use `<T as Identity>::ZERO` / `<T as Bounded>::MAX`.
- Delete `pub const trait UPrimConst` and `pub const trait IPrimConst`. (Or alias for one cycle then delete; decision: delete in this round, no aliasing, since this is internal substrate machinery and no external consumer reaches for these names.)
- Drop `pub use {UPrimConst, IPrimConst}` from `arvo/src/strategy.rs:19-23`.

### Decision 4: USize / Cap impl `Bounded` + `Identity`; extend macros to include `usize` / `isize`

Per Expert A Finding 4. USize / Cap predate Bounded/Identity; `usize::MAX` is hardcoded at `arvo-storage/src/platform.rs:34`.

Concrete:
- Extend `impl_bounded_identity_u!` macro (in `arvo-strategy/src/arith.rs`) to include `usize`.
- Extend `impl_bounded_identity_i!` to include `isize`.
- Add `impl const Bounded for USize` / `impl const Identity for USize` in `arvo-storage/src/platform.rs`. Bodies route through `<usize as Bounded>::MAX` etc.
- Add `impl const Bounded for Cap` / `impl const Identity for Cap`. Bodies route through `<USize as Bounded>::MAX` etc.
- Rewire inherent `USize::MAX` / `USize::ZERO` / `USize::ONE` to project through the trait: `pub const MAX: Self = <Self as Bounded>::MAX;` etc. (No public-API change; consumers continue to write `USize::MAX`.)
- Mirror on Cap.

### Decision 5: Out-of-scope for this round (deferred to Rounds 2-6 within PR #42)

Explicitly NOT in this round, even though some findings appear adjacent:

- MultiContainer BitPrim impl (Round 2 / #311)
- IBitPrim parity with BitPrim (Round 2 / #311)
- BitAccess/BitSequence/BitLogic on signed Bits (Round 2 / #311)
- Numeric-contracts impl const sweep (Round 2 / #311)
- UWidenFrom/INarrowFrom const lift (Round 2 / #311)
- Bits&lt;const N: Width&gt; lift (Round 3 / #312)
- MetaCarrier resolution (Round 3 / #312)
- Mask&lt;W&gt; over Bits (Round 4 / #313)
- BitMatrix generalisation (Round 4 / #313)
- BitPrim::WIDTH typed (Round 4 / #313)
- ConstFrom / ConstDeref bridges (Round 5 / #314)
- Algorithm crate USize sweep (Round 5 / #314)
- Smoke test coverage (Round 6 / #315)
- ConstHash bridge (Round 6 / #315)

These rounds depend on this round's foundation but each has independent scope.

## Cited Source Material

Audit reports (read in entirety before SRC-PLAN):
- `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md`
- `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md`

Workspace rules invoked:
- `~/Dev/clause-dev/.claude/rules/arvo-toolbox-not-policer.md` — substrate ships tools, never policies
- `~/Dev/clause-dev/.claude/rules/arvo-always-optimal-internals.md` — public API rules; internals do whatever's optimal
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md` — runtime first, compile time last (this round expands runtime/consumer-ergonomics surface, not compile-time-cost optimisation)
- `~/Dev/clause-dev/.claude/rules/no-bare-primitives.md`
- `~/Dev/clause-dev/.claude/rules/use-the-stack-not-reinvent.md`

Current closed rounds on PR #42 branch (foundation this round builds on):
- 202605021200 (USize/Cap const-arith + Layer C sweep)
- 202605021400 (UArith/BitPrim const-trait machinery)
- 202605021600 (Bounded/Identity bridges + Mask cleanup)
