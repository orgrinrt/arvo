//! Re-export of the bit-level trait declarations.
//!
//! Per round 202604271346 D-12, `HasBitWidth`, `BitAccess`,
//! `BitSequence`, `BitLogic` moved to the `arvo-bits-contracts`
//! crate as `pub const trait`. The concrete impls of these traits
//! on `arvo-storage::Bits<N, S>` (in `bits_impl.rs`),
//! `arvo::UFixed<I, F, S>` (in `ufixed_impl.rs`), and
//! `arvo::IFixed<I, F, S>` (in `ifixed_impl.rs`) stay in this crate.
//!
//! This module re-exports the four trait declarations so existing
//! `arvo_bits::traits::HasBitWidth` import paths remain valid.

pub use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence, HasBitWidth};
