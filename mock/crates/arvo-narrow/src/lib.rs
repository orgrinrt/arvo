//! arvo-narrow. Concrete `Narrow<T>` impls.
//!
//! Per source / target primitive pair, using concrete masks from
//! `arvo-bitmask` and `BitLogic` from `arvo-bits-contracts`.
//! Generic-over-Narrow consumers depend on `arvo-narrow-contracts`.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]
