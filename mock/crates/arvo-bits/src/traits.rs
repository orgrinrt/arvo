//! Public bit-level contracts.
//!
//! Three traits, all local to arvo-bits:
//!
//! - `HasBitWidth` — logical bit count at the type level.
//! - `BitAccess` — read / set / clear / toggle individual bits.
//! - `BitSequence` — popcount and scan operations.
//!
//! Concrete impls live in `ufixed_impl.rs` / `ifixed_impl.rs`. See
//! DESIGN for the concrete-impls-not-blanket rationale.

use arvo::newtype::{Bool, USize};

/// Logical bit width at the type level.
///
/// For `UFixed<I, F, S>` this is `I + F`; for `IFixed<I, F, S>` it is
/// `1 + I + F` (the sign bit counts). The width is the logical bit
/// count, not the container size — the container may be wider under
/// `Warm` / `Precise` strategies.
pub trait HasBitWidth {
    /// Logical bit width.
    const WIDTH: USize;
}

/// Individual bit read / write.
///
/// All mutators take `self` and return `Self` — functional style, no
/// interior mutation. `idx` is LSB-first (bit 0 is least significant).
/// Indices `>= WIDTH` do not panic: `bit` returns `Bool::FALSE` and
/// the three `with_bit_*` mutators return `self` unchanged.
pub trait BitAccess: HasBitWidth + Copy {
    /// Read bit at position `idx`.
    fn bit(self, idx: USize) -> Bool;
    /// Produce a copy with bit `idx` set to 1.
    fn with_bit_set(self, idx: USize) -> Self;
    /// Produce a copy with bit `idx` cleared to 0.
    fn with_bit_cleared(self, idx: USize) -> Self;
    /// Produce a copy with bit `idx` flipped.
    fn with_bit_toggled(self, idx: USize) -> Self;
}

/// Bulk bit-scanning / popcount.
///
/// Maps to hardware intrinsics on common targets: `count_ones` to
/// `popcnt`, `trailing_zeros` to `cttz` / `tzcnt`, `leading_zeros`
/// to `ctlz` / `lzcnt`. Operates on the raw container bits; the
/// logical-width contract is implicit in the types' construction.
pub trait BitSequence: HasBitWidth + Copy {
    /// Count trailing (LSB) zero bits.
    fn trailing_zeros(self) -> USize;
    /// Count leading (MSB) zero bits.
    fn leading_zeros(self) -> USize;
    /// Count set bits.
    fn count_ones(self) -> USize;
    /// Count cleared bits.
    fn count_zeros(self) -> USize;
    /// `Bool::TRUE` when every bit is zero.
    fn is_zero(self) -> Bool;
}

/// Whole-word bitwise logic.
///
/// Single-instruction on the backing primitive (x86 `or` / `and` /
/// `xor` / `not`; identical on aarch64). Loop-free. The three
/// bit-level contracts partition: `BitAccess` covers SINGLE-BIT
/// read/write; `BitSequence` covers SCAN / POPCOUNT; `BitLogic`
/// covers WHOLE-WORD logical ops. Together they describe a
/// bit-bearing container.
///
/// Hot-only surface. Other strategies (`Warm`, `Precise`, `Cold`)
/// have wider containers that make whole-word ops meaningless at
/// the logical-width level (NOT would flip surplus container bits).
pub trait BitLogic: HasBitWidth + Copy {
    /// Whole-word OR.
    fn bitor(self, other: Self) -> Self;
    /// Whole-word AND.
    fn bitand(self, other: Self) -> Self;
    /// Whole-word NOT.
    fn bitnot(self) -> Self;
    /// Whole-word XOR.
    fn bitxor(self, other: Self) -> Self;

    /// Clear the lowest set bit.
    ///
    /// Default impl uses `BitSequence::trailing_zeros` +
    /// `BitAccess::with_bit_cleared`. Concrete types can override with
    /// `self.to_raw() & (self.to_raw() - 1)` for the single-instruction
    /// fast path (x86 `BLSR`). Returns `self` unchanged when the word
    /// is already zero.
    #[inline]
    fn clear_lowest_set_bit(self) -> Self
    where
        Self: BitAccess + BitSequence,
    {
        if <Self as BitSequence>::is_zero(self).0 {
            return self;
        }
        let idx = <Self as BitSequence>::trailing_zeros(self);
        <Self as BitAccess>::with_bit_cleared(self, idx)
    }
}
