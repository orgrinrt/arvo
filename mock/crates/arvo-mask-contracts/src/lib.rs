#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-mask-contracts. Mask trait declaration.
//!
//! `Mask<const W: u16>` is the abstract const trait over bitmask
//! storage. Concrete impls (`Mask64` at W=64, `Mask256` at W=256,
//! `BitMatrix` over rows of `Mask64`) live in `arvo-bitmask`.
//! Consumers that depend only on the abstract surface pull this
//! crate, not the concrete-storage one.
//!
//! See `DESIGN.md` for the full surface.

use arvo_storage::{Bool, USize};

/// Bitmask of width `W` bits.
///
/// `set` / `clear` / `test` toggle individual bits. `union` /
/// `intersection` / `difference` / `complement` are whole-word
/// logical ops. `count` returns popcount. `mask_for_width(n)`
/// produces a mask with the lowest `n` bits set; consumed by
/// `Narrow<T>` impls to truncate a wider raw to a typed container.
///
/// `W` is encoded as `const u8` (mask widths in this stack top
/// out at 256). Default-method bodies compose `BitLogic` from
/// `arvo-bits-contracts`; concrete impls in `arvo-bitmask` may
/// override with hardware-specific implementations.
pub const trait Mask<const W: u16>: Sized + Copy {
    /// Mask with no bits set.
    fn empty() -> Self;
    /// Mask with every bit (in `[0, W)`) set.
    fn full() -> Self;
    /// Set bit `idx`. Returns `self` unchanged for `idx >= W`.
    fn set(self, idx: USize) -> Self;
    /// Clear bit `idx`. Returns `self` unchanged for `idx >= W`.
    fn clear(self, idx: USize) -> Self;
    /// Test bit `idx`. Returns `Bool::FALSE` for `idx >= W`.
    fn test(self, idx: USize) -> Bool;
    /// Count of set bits.
    fn count(self) -> USize;
    /// Set-theoretic union.
    fn union(self, other: Self) -> Self;
    /// Set-theoretic intersection.
    fn intersection(self, other: Self) -> Self;
    /// Set-theoretic difference: `self & !other`.
    fn difference(self, other: Self) -> Self;
    /// Set-theoretic complement (within `W` bits).
    fn complement(self) -> Self;
    /// Mask with the lowest `n` bits set; the rest cleared. Used by
    /// `Narrow<T>` impls to truncate a wider raw to fit a typed
    /// container of width `n`.
    fn mask_for_width(n: USize) -> Self;
}
