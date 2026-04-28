//! arvo-narrow. Concrete `Narrow<T>` impls.
//!
//! Per source / target primitive pair, using concrete masks from
//! `arvo-bitmask` and `BitLogic` from `arvo-bits-contracts`. The
//! `Narrow<T>` trait declaration also lives in `arvo-bits-contracts`
//! post-round-202604280034 (merged in from the previously-separate
//! `arvo-narrow-contracts` crate).
//!
//! See `DESIGN.md` for the full surface.

#![no_std]

pub use arvo_bits_contracts::{Narrow, Narrowed};
