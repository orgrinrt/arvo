//! arvo-storage. Minimal L0 storage primitives.
//!
//! `Bits<N, S>` storage primitive. `Bool`, `USize` platform
//! wrappers. `IBits`, `FBits`, `Width` typed meta-bit newtypes.
//! The only fielded primitive types in the substrate.
//!
//! See `DESIGN.md` for the full surface.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![feature(macro_metavar_expr_concat)]
#![feature(try_trait_v2)]
#![allow(incomplete_features)]
