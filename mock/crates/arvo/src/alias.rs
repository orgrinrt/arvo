//! Semantic bit-width aliases for `Bits`.
//!
//! `Bit` / `Nibble` / `Byte` / `Word` / `DWord` / `QWord` are `Bits<N>`
//! aliases at the common power-of-two widths. Default strategy is
//! `Hot`: bit work typically wants the minimum container and wrapping
//! semantics.
//!
//! Because `Bits` is a newtype over `UFixed` (not a type alias), the
//! named aliases share `Bits`'s opaque-identity trait surface —
//! `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, but no
//! arithmetic. Consumers that want arithmetic on an N-bit value
//! reach for `UFixed<{IBits(N)}, {FBits::ZERO}, S>` directly.

use arvo_storage::Bits;
use crate::strategy::Hot;

/// 1-bit opaque bit-pattern.
///
/// Use for column-stored flag data. `Bool` is the control-flow
/// counterpart (see `arvo::newtype::Bool`).
pub type Bit<S = Hot> = Bits<1, S>;

/// 4-bit opaque bit-pattern (half-byte).
pub type Nibble<S = Hot> = Bits<4, S>;

/// 8-bit opaque bit-pattern.
pub type Byte<S = Hot> = Bits<8, S>;

/// 16-bit opaque bit-pattern (x86 "word").
pub type Word<S = Hot> = Bits<16, S>;

/// 32-bit opaque bit-pattern (x86 "dword").
pub type DWord<S = Hot> = Bits<32, S>;

/// 64-bit opaque bit-pattern (x86 "qword"; arvo's widest logical
/// value).
pub type QWord<S = Hot> = Bits<64, S>;
