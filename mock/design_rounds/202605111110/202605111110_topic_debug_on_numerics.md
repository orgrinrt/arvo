**Date:** 2026-05-11
**Phase:** TOPIC
**Scope:** arvo facade + arvo-storage
**Source topics:** ad-hoc workspace decision (substrate gap surfaced by hilavitkutin runtime megaround Session 2A)

# Topic: Debug on arvo numerics

## Why this round exists

Standard Rust numerics (`u8`..`u128`, `i8`..`i128`, `f32`, `f64`, `usize`, `isize`, `bool`) all impl `core::fmt::Debug`. arvo's primitive analogues (`USize`, `Bool`, `UFixed`, `IFixed`, `Bits`, `FastFloat`, `StrictFloat`, `Cap`) do not. Consumer code that derives `Debug` on structs holding arvo primitives cannot compile, forcing localised `transmute_copy`-style workarounds at consumer call sites. The hilavitkutin runtime megaround surfaced this gap when its engine id types (FiberId / PhaseId / TrunkId / UnitId) needed `Debug` for their containing structs to derive cleanly.

The fix lives in arvo because arvo owns the numerics. Without a substrate-level `Debug` impl, every consumer reinvents the same workaround.

## Decision

Ship plain non-const `impl core::fmt::Debug` on:

- `arvo_storage::USize`
- `arvo_storage::Bool`
- `arvo_storage::Cap`
- `arvo_storage::Bits<const N: u16, S: Strategy, Sign: Signedness>` (where `Self::Inner: Debug`)
- `arvo::UFixed<const I: IBits, const F: FBits, S: Strategy>` (delegates to inner Bits)
- `arvo::IFixed<const I: IBits, const F: FBits, S: Strategy>` (delegates to inner Bits)
- `arvo::FastFloat<F: Ieee>` (delegates to inner Ieee primitive)
- `arvo::StrictFloat<F: Ieee>` (delegates to inner Ieee primitive)

Implementation strategy: each impl projects to the underlying primitive (via direct transmute through the `#[repr(transparent)]` chain, justified by the `Transparent` invariant), then delegates to that primitive's `Debug`. Format shape: `TypeName(value)` for newtypes (`USize(42)`, `Bool(true)`); the chained types (UFixed/IFixed/Bits/FastFloat/StrictFloat) format the underlying value directly (no extra wrapper noise; the type is named at the binding site).

## Non-decision: const Debug

`ConstDebug` as a `pub const trait` is the principled long-term shape (write-into-fixed-buffer pattern: `const fn const_debug(&self, buf: &mut [u8; DEBUG_MAX_LEN]) -> USize;` with blanket `impl<T: ConstDebug> core::fmt::Debug for T`). It is not in scope for this round; `core::fmt::Formatter` is not const-compatible, so the const trait needs its own writer abstraction, and per-numeric const-fmt-via-digit-extraction is substantial work.

Filed as workspace BACKLOG / arvo-side follow-up. This round ships plain non-const Debug only.

## Soundness

The `transmute_copy::<Self, T>` projection is sound because every numeric in scope here carries `#[repr(transparent)]` over a chain that terminates at a primitive of equal size. `Transparent::Inner` is the type the trait names; for chained newtypes (UFixed -> Bits -> primitive), the projection skips intermediate hops. Adding `#[repr(transparent)]` already requires the layout-equivalence invariant; this round leverages it for Debug. No new safety surface introduced.

## Closure criterion

Doc CL touches arvo + arvo-storage DESIGN.md.tmpl prose where Transparent / repr(transparent) chains are described, adding one sentence noting Debug-on-numerics.

Src CL adds the impls in their owning crates plus a trivial smoke test asserting Debug works for representative widths and strategies.
