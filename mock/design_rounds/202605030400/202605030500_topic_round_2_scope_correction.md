**Date:** 2026-05-03T05:00Z
**Phase:** TOPIC
**Scope:** arvo Round 2 — scope correction. Drops Deliverable C from the original topic and scopes the round to two cohesive deliverables.
**Source topics:** Supersedes the Deliverable C piece of `202605030400_topic_round_2_typed_width_and_ieee_relocation.md`. Deliverables A and B from the original topic carry forward unchanged.

# Round 2 — scope correction

The original topic (`202605030400_topic_round_2_typed_width_and_ieee_relocation.md`)
named three deliverables: A (Ieee relocation), B (LOGICAL_WIDTH internals), and
C (relocate Width / IBits / FBits / MetaCarrier from arvo-storage to
arvo-strategy). The doc CL that began executing those deliverables hit
realistic implementation cost on Deliverable C without commensurate value at
this stage of the substrate. This topic corrects the scope.

## What is dropped

**Deliverable C is removed from Round 2.** Width / IBits / FBits / MetaCarrier
stay in arvo-storage. The original topic's argument for relocation (co-locate
with strategy markers, prepare for Round 3's Bits-const-Width lift) does not
weigh against the immediate complexity:

- The const-trait predicate-bridge impls (`ConstPartialEq`, `ConstEq`,
  `ConstBitEq`, `ConstOrd`, `ConstDefault`) on the meta-newtypes target
  traits that live in arvo-storage (per the bridge-home rule, since they
  return `Bool` and Bool lives in arvo-storage). Splitting the
  `meta_bits_wrapper!` macro across crates so that arvo-strategy emits
  the type definition plus `Bounded` / `Identity` impls, and arvo-storage
  emits the predicate-bridge impls, doubles the macro surface area for a
  preparatory lift.
- The `as_bits()` accessor that projects each meta-newtype to
  `Bits<9, Hot, Unsigned>` cannot ride along to arvo-strategy because Bits
  lives in arvo-storage. Lifting the accessor to a `BitsView` trait in
  arvo-storage adds a new public-trait surface that no consumer needs
  outside the arvo crate today.
- Round 3 (#313), which lifts `Bits<const N: u16>` to `Bits<const N: Width>`,
  can proceed with Width remaining in arvo-storage. The consumer-facing
  bridge traits in arvo-strategy that need to mention Width as a typed
  const-generic param can either move down to arvo-storage themselves (a
  trade) or take a ConstParamTy bound that sits at the storage layer.
  That is a Round 3 decision point.

The relocation may still happen later. It is not in Round 2's scope.

## What ships in Round 2

The remaining scope is two cohesive deliverables, both auditable in a single
mechanical pass.

### Deliverable A: Ieee relocation to arvo-strategy

Move the `Ieee` const trait declaration plus its `f32` and `f64` impls from
`arvo/src/float.rs` to `arvo-strategy/src/ieee.rs` (new module). Re-export
from arvo facade so consumer imports through `arvo::Ieee` keep working.

- The supertrait commitment to `[const] Identity + [const] Bounded` lives
  with the trait declaration. Both supertraits already ship from
  `arvo-strategy::arith`; the relocation removes the layer-inversion
  smell without adding any cross-crate path.
- `Ieee::WIDTH: u16` stays bare-primitive-typed for this round. Round 3
  lifts it to `Width` once the broader bare-`u16` to typed-`Width` sweep
  cascades through the bridge family.
- The seal trait (`sealed::Sealed`) moves alongside the trait declaration.
  Future widenings (`f16` / `bf16` / `f128`) extend the seal in
  arvo-strategy plus the existing `impl_bounded_identity_f!` macro
  invocation.
- The `FromU8Ieee` companion trait travels with `Ieee` (same logical
  family, same orphan-rule constraints).

### Deliverable B: LOGICAL_WIDTH internal arithmetic cleanup

`UFixed`'s `BitPresentation` impl at `arvo/src/ufixed.rs:204` and `IFixed`'s
impl at `arvo/src/ifixed.rs:198` compose `LOGICAL_WIDTH` as
`USize(I.raw() as usize + F.raw() as usize)`. The trait surface is typed
`USize`; the body routes through bare arithmetic with `as usize` casts.

Round 2 replaces the body with typed arithmetic at the meta-bit layer. The
exact surface choice is implementation work and lands in the source CL:

- Either `IBits + FBits -> Width` const trait operator on the meta-newtypes
  (one or two impls in arvo-storage), with a single boundary cast at the
  trait emit point (`USize(width.raw() as usize)`).
- Or a const-fn helper `fn logical_width(i: IBits, f: FBits) -> USize`
  ships from arvo-storage and the impl bodies call it.

Either resolution keeps the trait surface unchanged at `USize` and removes
the bare `as usize` cast composition from the impl bodies.

## Locked decisions reaffirmed

The locked decisions from the original topic that survive scope correction:

1. **Ieee lives in arvo-strategy.** Decision 1 of the original topic.
2. **LOGICAL_WIDTH bodies route through typed arithmetic.** Decision 5 of
   the original topic.
3. **Bits<const N: u16> stays for Round 2.** Decision 7 of the original
   topic.

The dropped decisions are 2 (Width relocation), 3 (re-exports for the
relocated newtypes), 4 (BitsView lift trait), and 6 (Width const-trait
arithmetic via the macro). Some of these may resurface in Round 3.

## Round 3 setup

Round 3 (#313) inherits a substrate where:

- Ieee lives where its supertraits live (Deliverable A landed).
- LOGICAL_WIDTH bodies are typed (Deliverable B landed).
- Width / IBits / FBits / MetaCarrier still live in arvo-storage.

Round 3 carries the `Bits<const N: u16>` to `Bits<const N: Width>` lift plus
the bridge-trait cascade. The first Round 3 task is to verify the lift
compiles with the existing `<S as BitsContainerFor<{N.raw()}, Sign>>::T`
projection under `feature(generic_const_exprs)`. If the lift hits a
well-formedness cycle, Round 3 either:

- Moves the bridge traits down to arvo-storage so Width is reachable from
  the same crate the bridges live in, OR
- Introduces a Width-equivalent const-generic-position carrier (different
  shape, same effect) at arvo-strategy.

Round 2 ships the prerequisites that Round 3 needs: a clean Ieee location
and typed LOGICAL_WIDTH bodies.
