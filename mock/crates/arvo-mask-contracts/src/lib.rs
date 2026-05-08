#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-mask-contracts. Mask operations contract.
//!
//! `MaskOps` is the abstract const trait over a bitmask. Concrete
//! impls (the chassis `Mask<W>` from `arvo-bitmask` at any underlying
//! word `W`) live in `arvo-bitmask`. Consumers that depend only on
//! the abstract surface pull this crate, not the concrete-storage
//! one.
//!
//! Round 202605031748 (#313) renamed the trait `Mask` -> `MaskOps`
//! to disambiguate from the concrete chassis struct
//! `arvo_bitmask::Mask<W>`.
//!
//! Round 202605040602 (#315) dropped the trait's `const W: Width`
//! parameter. The parameter was dead in every method body (the
//! signature reads `Self` plus `USize` arguments, never the const
//! `W`). With the parameter gone, a single blanket impl on `Mask<W>`
//! for any `W` satisfying the underlying bit-trait bounds replaces
//! the prior per-W concrete-impl plan. The chassis's `W` type
//! parameter carries the storage-shape information; the trait stays
//! a uniform predicate. Mask-width-specific routing (the prior
//! `mask_for_width(n)` method) was never called by any consumer; the
//! substrate's `BitPrim::mask_low(n)` and `IBitPrim::mask_low(n)`
//! cover that role at the bare-primitive layer where `Narrow<T>`
//! impls actually compose. The method dropped with the reshape.
//!
//! See `DESIGN.md` for the full surface.

use arvo_storage::{Bool, USize};

/// Bitmask predicate.
///
/// `set` / `clear` / `test` toggle individual bits. `union` /
/// `intersection` / `difference` / `complement` are whole-word
/// logical ops. `count` returns popcount. Width follows from the
/// implementor's underlying word type; the trait is uniform across
/// the width axis.
///
/// Default-method bodies compose `BitLogic` and `BitAccess` from
/// `arvo-bits-contracts`; concrete impls in `arvo-bitmask` may
/// override with hardware-specific implementations.
pub const trait MaskOps: Sized + Copy {
    /// Mask with no bits set.
    fn empty() -> Self;
    /// Mask with every bit set.
    fn full() -> Self;
    /// Set bit `idx`. Returns `self` unchanged for `idx >= width`.
    fn set(self, idx: USize) -> Self;
    /// Clear bit `idx`. Returns `self` unchanged for `idx >= width`.
    fn clear(self, idx: USize) -> Self;
    /// Test bit `idx`. Returns `Bool::FALSE` for `idx >= width`.
    fn test(self, idx: USize) -> Bool;
    /// Count of set bits.
    fn count(self) -> USize;
    /// Set-theoretic union.
    fn union(self, other: Self) -> Self;
    /// Set-theoretic intersection.
    fn intersection(self, other: Self) -> Self;
    /// Set-theoretic difference: `self & !other`.
    fn difference(self, other: Self) -> Self;
    /// Set-theoretic complement.
    fn complement(self) -> Self;
}
