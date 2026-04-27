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
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]

pub use arvo_bits_contracts::{Narrow, Narrowed};
