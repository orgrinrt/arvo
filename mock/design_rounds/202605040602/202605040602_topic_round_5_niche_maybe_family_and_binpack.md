**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** notko (cross-repo) + arvo-storage + arvo-comb
**Source topics:** Round 1 expanded P0 deferral list (Expert A F14 binpack Maybe sentinel)

# Round 5 Topic 2: niche-Maybe family. `Slot<T>` in notko, `NUSize` in arvo-storage, binpack F14 fix

This topic covers the substrate's transparent niche-Maybe primitives, plus the arvo-comb binpack return-type rework that motivated the family.

## Background

Expert A's audit Finding 14 named that `arvo-comb/src/binpack.rs` uses `USize(0)` as both "first valid bin" and "didn't fit" sentinel. The audit's literal proposal was to change the return type to `(USize, Array<Maybe<USize>, N>)`. That literal fix carries layout cost: `Maybe<USize>` is 16 bytes (no niche; USize has no zero-pattern niche) per slot. The substrate's identity is "no heap, transparent where possible"; doubling the array footprint to encode an absent flag is a smell.

notko already ships the building blocks: `Maybe<T>` enum (niche-optimised when `T: NicheFilled`), `NicheFilled` sealed marker trait, `NonZeroable` trait. `core::num::NonZero<usize>` is `NicheFilled`. So `Maybe<NonZeroUSize>` is 8 bytes via niche optimisation, the layout we want. The remaining gap is ergonomics: consumers want a 0-indexed logical view, not 1-indexed-with-NonZero plumbing in every call site.

Two layers are needed: a generic transparent niche-Maybe wrapper (the reusable primitive), and a USize-shaped wrapper that bakes the +1/-1 shift so consumers see logical 0-indexed semantics without remembering the encoding.

## Decisions

### Decision 1: notko gets `Slot<T: NonZeroable>` as a generic transparent niche-Maybe wrapper

```rust
#[repr(transparent)]
pub struct Slot<T: NonZeroable>(Maybe<T>);
```

`Slot<T>` is a type-level marker that this position is a niche-filled Maybe. Layout is identical to `T` because `Maybe<T>` niche-optimises when `T: NicheFilled` (which `T: NonZeroable` implies). No arithmetic; no shift; just a typed wrapper around `Maybe<T>` that signals niche-filling at the type level.

Surface:

```rust
impl<T: NonZeroable> Slot<T> {
    pub const NONE: Self;
    pub const fn some(value: T) -> Self;
    pub const fn as_maybe(&self) -> Maybe<T>;
    pub const fn is_some(&self) -> Bool;
    pub const fn is_none(&self) -> Bool;
}
```

Const-traits gate behind notko's new `const` feature (Topic 1 Decision 3). With feature off, the methods are non-const but otherwise equivalent.

### Decision 2: arvo-storage gets `NUSize`, niche-packed Maybe-of-USize with auto-shift

```rust
#[repr(transparent)]
pub struct NUSize(Slot<NonZeroUSize>);
```

`NUSize` lives next to `USize` in arvo-storage. Internally holds `Slot<NonZeroUSize>` (which is `Maybe<NonZeroUSize>` niche-optimised). Externally presents 0-indexed `USize` semantics: a logical value of `0` corresponds to inner `NonZeroUSize::new(1)`, a logical value of `n` corresponds to inner `NonZeroUSize::new(n+1)`, and "absent" maps to `Slot::NONE`.

Surface:

```rust
impl NUSize {
    pub const NONE: Self;
    pub const fn some(logical: USize) -> Self;            // shift +1, wrap
    pub const fn as_maybe(&self) -> Maybe<USize>;         // unwrap, shift -1
    pub const fn unwrap_or(self, default: USize) -> USize;
    pub const fn is_some(&self) -> Bool;
    pub const fn is_none(&self) -> Bool;
}
```

Layout: `size_of::<NUSize>() == size_of::<usize>()` (8 bytes on 64-bit). Same as raw `USize`.

The "+1 shift on construction, -1 on extraction" is the wrapper's only contract beyond Slot. Consumers see logical 0-indexed values throughout; the encoding is invisible.

### Decision 3: NU8 / NU16 / NU32 / NU64 are out of Round 5 scope

Round 5 ships only `NUSize`. The N-prefix family naming (`NUSize`, `NU8`, `NU16`, `NU32`, `NU64`) is established as a convention so future rounds can extend without bikeshedding, but the broader family adds source CL volume without unblocking Round 5 work. Add when a use case demands a specific width.

### Decision 4: arvo-comb binpack uses `Array<NUSize, N>`

```rust
pub fn bin_pack<...>(...) -> (USize, Array<NUSize, N>)
```

Consumers iterate the array and call `slot.as_maybe()` or `slot.is_some()` to distinguish placed-vs-not. Sentinel ambiguity from F14 disappears: `NUSize::NONE` is the sole "did not fit" representation; `NUSize::some(USize(0))` is the first valid bin.

Internal binpack body changes minimally: replace `Array<USize, N>` with `Array<NUSize, N>`, replace `Array::filled(USize(0))` with `Array::filled(NUSize::NONE)`, replace `bins_of_items.set(idx, USize(b))` with `bins_of_items.set(idx, NUSize::some(USize(b)))`.

### Decision 5: NBits<N> deferred

NBits<N> (a niche-filled Bits<N> variant for Maybe<NBits<N>> niche optimisation) is a parallel primitive at the bits-family layer, not a substitute for NUSize. It addresses different use cases (optional N-bit values without discriminant, e.g. bitfield slots, hash slots, packed enum fields). Ship NBits in a future arvo round when a use case demands it; design questions (where the niche lives, multi-container interaction, BitsContainerFor projection) warrant their own topic.

Tracked as task #323, gated behind #197 (viola TS ecosystem verified) + #231 (substrate-readiness gate) so it stays in the future-tasks pool.

## Out of scope

- Generic-with-arithmetic Slot variant (e.g. `Slot<T: NonZeroable + Arith>` that auto-shifts for any arithmetic NonZeroable). notko stays arithmetic-free.
- `Slot<T>` impls of `core::ops::Try` (the `?` desugaring concern is Topic 1's territory; Slot is a wrapper, not a fallibility carrier).
- Rewriting Maybe<USize> uses elsewhere in the substrate to NUSize. Targeted F14 fix only; other call sites stay as-is until a sweep round has bench evidence the layout cost matters.

## Cross-references

- `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md` Finding 14 (audit motivation).
- notko `src/maybe.rs` (`NicheFilled` sealed marker, `Maybe<T>` niche-optimisation).
- notko `src/nonzero.rs` (`NonZeroable` trait).
- arvo-storage `src/platform.rs` (USize lives here; NUSize joins).
- arvo-comb `src/binpack.rs:13, 45, 121-122` (the F14 sentinel sites).
- Topic 1 (this round): notko `const` cargo feature; Slot's const-trait surface gates behind it.
- Task #323 (NBits future round, gated behind #197 + #231).
