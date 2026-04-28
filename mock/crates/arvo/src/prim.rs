//! Facade re-export of the bit-primitive bridge surface.
//!
//! Per round 202604271346 D-12, `BitPrim`, `IBitPrim`, `UBitContainer`,
//! and `IBitContainer` moved to the `arvo-bits-contracts` crate. The
//! impls on `u8`/`u16`/`u32`/`u64` (`BitPrim`) and `i8`/`i16`/`i32`/`i64`
//! (`IBitPrim`) live there too (orphan rules require trait + foreign-
//! type impls to share a crate). The blanket impls of `UBitContainer`
//! / `IBitContainer` over `(Strategy, BITS)` also live there.
//!
//! This module re-exports the four traits so `arvo::BitPrim`,
//! `arvo::IBitPrim`, `arvo::UBitContainer`, `arvo::IBitContainer`
//! import paths remain valid for downstream consumers.

pub use arvo_bits_contracts::{BitPrim, IBitContainer, IBitPrim, UBitContainer};
