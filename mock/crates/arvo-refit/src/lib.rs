//! arvo-refit. Re-export gateway for the bit-width refit family.
//!
//! Re-exports `Narrow<T>` / `Widen<T>` / `Narrowed` / `Widened`
//! from `arvo-bits-contracts`. The trait declarations and the
//! cross-primitive + cross-Bits + cross-domain impl tables all
//! live in `arvo-bits-contracts` (orphan-rule constraint: trait
//! plus foreign-primitive impls share a crate).
//!
//! Renamed from `arvo-narrow` in round 202604301700. The prior
//! `arvo-narrow-contracts` trait crate was already merged into
//! `arvo-bits-contracts` in round 202604280034.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]

pub use arvo_bits_contracts::{Narrow, Narrowed, Widen, Widened};
