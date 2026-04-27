//! arvo-numeric-contracts. Numeric-op trait declarations.
//!
//! `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, `Predicate`
//! family. All `pub const trait`. Default impls in the arvo facade.
//! Concrete impls bind to `Bits<N, S>`, `UFixed`, `IFixed`.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
