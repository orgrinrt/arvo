#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

//! arvo-bits — bit-storage aliases.
//!
//! `Bit` / `Nibble` / `Byte` / `Word` / `DWord` / `QWord` are
//! `arvo_storage::Bits<N, S>` aliases at the common power-of-two
//! widths. Default strategy is `Hot`: bit work typically wants the
//! minimum container and wrapping semantics.
//!
//! The bit-level trait declarations (`HasBitWidth`, `BitAccess`,
//! `BitSequence`, `BitLogic`) live in `arvo-bits-contracts`. The
//! blanket impls on `Bits<N, S>` also live there (orphan rule:
//! trait + foreign-type impls share a crate). This crate is the
//! domain-alias surface, nothing else.

// Convenience re-exports so consumers can pull bit-level traits and
// the bit-storage primitive from one crate. Trait declarations live
// in `arvo-bits-contracts`; storage primitive lives in `arvo-storage`.
pub use arvo_bits_contracts::{
    BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth, IBitContainer, IBitPrim,
    UBitContainer,
};
pub use arvo_storage::Bits;
pub use arvo_strategy::{Hot, Strategy};

/// 1-bit opaque bit-pattern.
///
/// Use for column-stored flag data. `Bool` (in `arvo-storage`) is
/// the control-flow counterpart.
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
