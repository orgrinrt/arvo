**Date:** 2026-05-03T04:00Z
**Phase:** TOPIC
**Scope:** arvo Round 2 — typed Width plumbing through the substrate, Ieee relocation, LOGICAL_WIDTH internal arithmetic.
**Source topics:** Audit follow-through from Round 202605021800 (H6 deferred to Round 2 cleanup; H3 LOGICAL_WIDTH probe annotated with Round 2 follow-up). Resume memory `project_resume_post_compact_2026_05_02_arvo_round_1_doc_phase.md`.

# Round 2 — typed Width + Ieee relocation + LOGICAL_WIDTH internal arithmetic

Round 1 (202605021800) closed the substrate const-trait foundation and the
predicate-bridge family. Round 2 picks up the deferred audit items that did
not make Round 1's scope: H6 (relocate `Ieee` from arvo facade into
`arvo-strategy` to co-locate with its `Identity` / `Bounded` supertraits),
the LOGICAL_WIDTH internal-arithmetic cleanup the H3 probe flagged, and the
mechanical lift of substrate const-generic positions from bare `u16` to the
typed `Width` newtype that already ships in `arvo-storage::meta_bits`.

The round is mechanical and follows from prior decisions. No new shape is
proposed. `Width`, `IBits`, `FBits` already exist as `ConstParamTy` typed
meta-bit newtypes carrying a `MetaCarrier` payload (added in round
202604280806 / round 202605021200 to bypass the rustc trait-solver cycle
that arises when a struct field literally writes the generic
`Bits<9, Hot, Unsigned>` projection). Round 2 propagates the typed surface
into the const-generic positions that the substrate currently expresses as
bare `u16`.

## Background

Three audit items from Round 1's archived design are picked up here:

- **H6** (Round 1 audit, deferred): the `Ieee` const trait lives at
  `arvo/src/float.rs` lines 19 to 54. Its supertrait commitment
  (`Identity + Bounded`) ships from `arvo-strategy::arith` to satisfy the
  orphan rule when impls land on bare `f32` / `f64`. Co-locating the trait
  declaration in `arvo-strategy` removes a layer-inversion smell where the
  trait sits one layer above its supertraits. The substrate gains nothing
  from holding `Ieee` at the facade; the relocation is hygiene.

- **H3** (Round 1 probe, annotated for Round 2): `BitPresentation::
  LOGICAL_WIDTH` resolves at the const surface as `USize`. The probe at
  `mock/crates/arvo/tests/logical_width_const_probe.rs` validates const-
  callability through arvo's typed-arithmetic surface. The trait surface is
  already typed; the impl bodies route through `as usize` casts to compose
  the inner u16-carried meta-bit values. Round 2 replaces the cast-through-
  bare-primitives pattern with typed Width / USize arithmetic.

- **Const-generic surface u16 sites**: 77 occurrences of `const N: u16` /
  `const BITS: u16` / `const W: u16` across the substrate sit at type-
  parameter positions where the typed meta-newtype `Width` is what the
  consumer actually carries. The bare-primitive sites span:
  - `arvo-storage::Bits<const N: u16, S, Sign>` (the foundational primitive)
  - `arvo-strategy::{UContainerFor, IContainerFor, BitsContainerFor}<const N: u16, ...>`
  - `arvo-bits-contracts::{UBitContainer, IBitContainer}<const BITS: u16>`
  - `arvo-bits-contracts::Narrow / Widen / Narrowed<const N: u16, T>`
  - `arvo-mask-contracts::Mask<const W: u16>`
  - `arvo-hash::{Hasher, Fnv1a}<const N: Width>` (already lifted; reference)

  The lift is mechanical. The trait-solver cycle that drove MetaCarrier is
  specific to a struct field literally projecting `Bits<9, Hot, Unsigned>`
  while the struct is itself used as `const I: IBits` (a where-clause
  position cycle). Const-generic-parameter-position use of `Width` is
  governed by `ConstParamTy + adt_const_params` and does not rebroadcast
  the cycle. Where the lift hits new well-formedness gates the round
  documents the case and either lands the lift with the gate or holds that
  position open.

## Decision: scope the round around three cohesive deliverables

The substrate has the typed `Width` newtype ready and the const-trait surface
arrives at typed `USize` for `LOGICAL_WIDTH` already. Round 2's deliverables
package the remaining work into three cohesive blocks, each landing in a
single doc-and-source pass:

### Deliverable A: Ieee relocation to arvo-strategy

- Move `Ieee` trait declaration from `arvo/src/float.rs` to
  `arvo-strategy/src/ieee.rs` (new module).
- Move both impls (`impl const Ieee for f32` and `impl const Ieee for f64`)
  alongside the declaration. Orphan rule is satisfied since both trait and
  impls live in arvo-strategy and the impls target bare-primitive types
  authorised in arvo-strategy already.
- Lift `Ieee::WIDTH: u16` to `Ieee::WIDTH: Width` (Width is reachable from
  arvo-strategy after Deliverable C lands). Bare-primitive bodies cast at
  the construction boundary through the existing `width(n)` const-fn helper.
- Re-export `Ieee` from arvo facade so consumer code keeps `arvo::Ieee`
  reach intact. The re-export is the only change at the facade layer.

### Deliverable B: LOGICAL_WIDTH internal-arithmetic cleanup

- `UFixed`'s `BitPresentation` impl at `arvo/src/ufixed.rs:204` and
  `IFixed`'s impl at `arvo/src/ifixed.rs:198` compose `LOGICAL_WIDTH` as
  `USize(I.raw() as usize + F.raw() as usize)` and similar. The surface is
  typed; the body routes through bare arithmetic.
- Round 2 replaces the body with typed Width arithmetic: `Width + Width`
  resolves to `Width` through the `impl_unsigned_integer_newtype!` macro
  surface from Round 1, then a single boundary cast emits the `USize` the
  trait surface declares.
- Width gains const-trait `Add` / `Sub` (one-line per-op blanket impls
  routed through the inner `u16` arithmetic) sufficient to compose the two
  call sites without bare-primitive intermediates. Equivalent surface for
  IBits / FBits if needed by call sites.

### Deliverable C: Width / IBits / FBits relocation to arvo-strategy

- Move the three meta-bit newtypes (`Width`, `IBits`, `FBits`) plus the
  `MetaCarrier` companion plus the `meta_bits_wrapper!` macro from
  `arvo-storage/src/meta_bits.rs` to `arvo-strategy/src/meta_bits.rs`.
- arvo-storage re-exports them so existing consumers continue to import
  through `arvo_storage::{Width, IBits, FBits, MetaCarrier}`. The
  facade re-export at the arvo crate stays untouched.
- The relocation makes Width reachable from arvo-strategy, which is required
  for Deliverable A's lifting of `Ieee::WIDTH: Width` and for any future
  bridge-trait const-generic-parameter lift.
- The `as_bits(self) -> Bits<9, Hot, Unsigned>` accessor on MetaCarrier and
  on each meta-newtype goes one of two ways: (1) keep the accessor at
  arvo-storage as a free-function or accessor trait that lifts a meta-
  newtype to the Bits-typed view, leaving the meta-newtype itself in
  arvo-strategy primitive-only; (2) drop the `as_bits()` accessor from the
  primitive-level newtype since no current consumer reaches for it from
  arvo-strategy. Round 2 picks option (1), keeps the accessor in
  arvo-storage as a thin lift-trait, and removes the inherent method from
  the primitive declaration.
- The `meta_bits_wrapper!` macro itself moves with the newtypes. arvo-
  strategy gains the `Bounded` / `Identity` / `ConstPartialEq` /
  `ConstBitEq` / `ConstOrd` / `ConstDefault` impls already shipping today.
  The Bits-projection accessor (`as_bits()`) becomes the lift trait in
  arvo-storage.

The full `Bits<const N: u16, ...>` to `Bits<const N: Width, ...>` lift,
including bridge-trait const-parameter retypings (UContainerFor /
IContainerFor / BitsContainerFor / UBitContainer / IBitContainer / Narrow
/ Widen / Mask), is **not** in Round 2's scope. That work spans 70 plus
sites across five crates with potential cycle-resolution at every
projection through the bridge tables. Round 3 (#313) carries the
mechanical lift forward; Round 2 ships the prerequisites Round 3 needs:
typed Width arithmetic at the meta-bit layer (Deliverable B) and Width
reachable from the dependency graph below `arvo-storage` (Deliverable C).

## What this round does NOT do

- Does not change the public API of `Bits<const N, S, Sign>`. Consumers
  that write `Bits<7, Hot, Unsigned>` continue to compile. The trait bound
  surface stays at `<const N: u16, ...>` until Round 3.
- Does not touch `UContainerFor` / `IContainerFor` / `BitsContainerFor` /
  `UBitContainer` / `IBitContainer` / `Narrow` / `Widen` const-generic
  shapes. Their bodies still drive 64-entry per-N projection tables; the
  table contents are unaffected by Width-typed surface lift.
- Does not touch `arvo-mask-contracts::Mask<const W: u16>`. Its const
  generic stays bare-primitive-typed; Round 3 lifts it together with the
  rest of the bridge family.
- Does not introduce any new trait. No new ConstFrom / ConstTryFrom /
  ConstDeref / ConstAsRef bridges (Round 4's scope).
- Does not change `BitPresentation::LOGICAL_WIDTH`'s typed surface. It
  remains `USize`. Only the impl-body internals change.

## Locked decisions (no further discussion required)

1. **Ieee lives in arvo-strategy.** Co-location with Identity / Bounded
   resolves the layer-inversion noise.
2. **Width / IBits / FBits / MetaCarrier live in arvo-strategy.** Reachable
   from every layer above. Required to lift Ieee::WIDTH and to unblock
   Round 3.
3. **arvo-storage re-exports the meta-bit newtypes.** No consumer-facing
   import path changes. Existing code keeps `arvo_storage::Width`.
4. **as_bits accessor lifts to a trait in arvo-storage.** The primitive-
   only meta-newtypes stay free of Bits projection at definition time.
   A `BitsView` (or similar) trait in arvo-storage carries the
   `as_bits(self) -> Bits<9, Hot, Unsigned>` method; impls on Width / IBits
   / FBits / MetaCarrier ship from arvo-storage.
5. **LOGICAL_WIDTH bodies route through typed Width arithmetic.** No bare
   `as usize` casts in the impl. One cast at the typed-emit boundary.
6. **Width gets const-trait `Add` / `Sub`.** Minimum surface to compose the
   LOGICAL_WIDTH bodies. Wider arithmetic surface waits for a real consumer
   need.
7. **Bits<const N: u16> stays for Round 2.** The mechanical lift cascades
   through 70 plus call sites and is naturally Round 3's scope. Round 2
   ships the prerequisites.

## Round 3 setup

Round 3 (#313) inherits a substrate where:

- `Width` is reachable from arvo-strategy and below.
- `Ieee` lives where its supertraits live.
- LOGICAL_WIDTH bodies are typed.
- The bridge-home rule (workspace .claude/rules/arvo-bridge-home-rule.md)
  applies to any new bridge added during the lift.

Round 3 carries the `Bits<const N: u16>` to `Bits<const N: Width>` lift
plus the bridge-trait cascade. The first Round 3 task is to verify that
`Bits<const N: Width, S, Sign>` compiles with the existing
`<S as BitsContainerFor<{N.raw()}, Sign>>::T` projection under
`feature(generic_const_exprs)`. If the lift triggers a new well-formedness
cycle, Round 3 documents the case and either lands the lift with the gate
or holds the position open.
