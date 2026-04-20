//! Semantic bit-width aliases for `UFixed`.
//!
//! `Bit` / `Nibble` / `Byte` / `Word` / `DWord` / `QWord` are integer
//! UFixed aliases at the common power-of-two widths. All are
//! `F = FBits::ZERO` (pure integer). Default strategy is `Hot`: bit
//! work typically wants the minimum container and wrapping semantics.
//!
//! The `IBITS_*` constants are module-private because arvo L0 only
//! exposes `IBits::ZERO` / `IBits::ONE` — extending L0 with more
//! named constants would reopen the closed L0 round. Module-local
//! constants are enough: the aliases below are the only sites that
//! need them, and consumers reference the aliases, not the
//! constants.

use arvo::newtype::{FBits, IBits};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;

// Private alias-width constants. See module docs for why these live
// here instead of on `IBits` in arvo L0.
pub(crate) const IBITS_ONE: IBits = IBits(1);
pub(crate) const IBITS_FOUR: IBits = IBits(4);
pub(crate) const IBITS_EIGHT: IBits = IBits(8);
pub(crate) const IBITS_SIXTEEN: IBits = IBits(16);
pub(crate) const IBITS_THIRTYTWO: IBits = IBits(32);
pub(crate) const IBITS_SIXTYFOUR: IBits = IBits(64);

/// 1-bit unsigned fixed-point value.
///
/// Use for column-stored flag data. `Bool` is the control-flow
/// counterpart (see arvo::newtype::Bool).
pub type Bit<S = Hot> = UFixed<IBITS_ONE, { FBits::ZERO }, S>;

/// 4-bit unsigned fixed-point value (half-byte).
pub type Nibble<S = Hot> = UFixed<IBITS_FOUR, { FBits::ZERO }, S>;

/// 8-bit unsigned fixed-point value.
pub type Byte<S = Hot> = UFixed<IBITS_EIGHT, { FBits::ZERO }, S>;

/// 16-bit unsigned fixed-point value (x86 "word").
pub type Word<S = Hot> = UFixed<IBITS_SIXTEEN, { FBits::ZERO }, S>;

/// 32-bit unsigned fixed-point value (x86 "dword").
pub type DWord<S = Hot> = UFixed<IBITS_THIRTYTWO, { FBits::ZERO }, S>;

/// 64-bit unsigned fixed-point value (x86 "qword"; arvo's widest
/// logical value).
pub type QWord<S = Hot> = UFixed<IBITS_SIXTYFOUR, { FBits::ZERO }, S>;
