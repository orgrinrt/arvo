//! arvo-narrow-contracts. Narrow trait declaration.
//!
//! `Narrow<T>` const trait. `Narrowed<N, T>` typed alias. The
//! unified mask-and-cast bridge replacing per-size helpers.
//! Concrete impls live in `arvo-narrow` and `arvo-bitmask`.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
