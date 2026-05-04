# Sketch: `MaskOps` blanket-impl shape resolution

**Date**: 2026-05-04T06:02Z
**Round**: 202605040602 (Round 5, #315)
**Tracks**: src CL Topic 3 Decision 2 (MaskOps blanket impl).

## Question

The src CL ships a single blanket `impl const MaskOps<...> for Mask<W>` for any
`W` satisfying the underlying bit-trait bounds (`BitSequence + BitAccess +
BitLogic + Copy + Default`). The current `MaskOps<const W: Width>` declaration
takes a const-generic value parameter, while the chassis `Mask<W>` in
`arvo-bitmask` is generic over a *type* `W` (the underlying word storage). The
shapes do not compose for a blanket impl: a `const W: Width` value parameter
on the trait does not bind the chassis's type parameter.

Two paths considered:

- (a) Reshape `MaskOps` to drop the `const W: Width` parameter entirely. The
  trait becomes a plain `Sized + Copy` predicate over `Self`. Width is read
  from the storage type, not from a separate parameter.
- (b) Reshape `MaskOps` to take a *type* parameter (`pub const trait
  MaskOps<T: BitPrim>`) so the blanket reads `impl<T: [const] BitPrim> const
  MaskOps<T> for Mask<T>`. The type parameter is redundant with `Self` (since
  `Self = Mask<T>` determines `T`), but adds future flexibility for
  cross-chassis ops.

## Hypothesis

Path (a) is the cleaner shape. The current `MaskOps<const W: Width>::W` is
dead data: every method in the trait body takes/returns `Self` and `USize`,
none reference `W`. The only place width matters is `mask_for_width(n: USize)
-> Self`, where `n` is a runtime-shaped (const-callable) parameter, not the
trait's `W`. Removing `W` simplifies the trait and lets the blanket impl
satisfy the substrate's "tools, not policy" stance: consumers pick the chassis
flavor (`Mask<Bits<64, Hot, Unsigned>>` or `Mask<Bits<256, Hot, Unsigned>>`)
and the trait is uniform across them.

## Probe

`01_reshape_no_param.rs` declares the reshaped trait and the blanket impl in
isolation, against a stub `Mask<W>` mirroring the chassis layout. The probe
compiles iff path (a) holds under rustc 1.96.0-nightly's const-trait solver
with the existing `BitSequence + BitAccess + BitLogic + Copy + Default`
bounds.

## Outcome

`WORKS` (path a). The reshaped trait drops the const-Width parameter; the
blanket impl on `Mask<W>` compiles under rustc 1.96.0-nightly with `[const]`
relaxed bounds on the bit-trait family. Method bodies delegate to the chassis
inherent methods that already exist in `arvo-bitmask/src/ops.rs`.

The src CL applies path (a). `MaskOps`'s `const W: Width` parameter is dropped
in `arvo-mask-contracts/src/lib.rs`. The blanket impl lands in
`arvo-bitmask/src/ops.rs` next to the inherent block. Per-W concrete overrides
move to a future bench-driven round (BACKLOG entry).

## Notes

The `mask_for_width(n: USize)` method on the reshaped trait reads its operand
as a runtime `USize`, not a const generic. This is the same shape used by
`BitPrim::mask_low(n: USize) -> Self` (the substrate helper used by `Narrow`
impls). The chassis's `Mask<W>::mask_for_width` already exists as an inherent
method via the underlying `BitPrim::mask_low` route; the trait method
delegates.
