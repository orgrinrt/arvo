//! arvo-mask-contracts. Mask trait declaration.
//!
//! `Mask<W>` const trait. Abstract surface over `Mask64`,
//! `Mask256`, `BitMatrix` from `arvo-bitmask`. Concrete impls live
//! in `arvo-bitmask`.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
