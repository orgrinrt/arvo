//! arvo-bits-contracts. Bit-level trait declarations.
//!
//! `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`,
//! `UBitContainer`, `IBitContainer`. All `pub const trait`. Concrete
//! impls live in `arvo-bits`.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]
