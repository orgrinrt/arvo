#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![feature(try_trait_v2)]
#![allow(incomplete_features)]

//! arvo-storage. Minimal L0 storage primitives.
//!
//! `Bits<N, S>` storage primitive. `Bool`, `USize`, `Cap` platform
//! wrappers. `IBits`, `FBits`, `Width` typed meta-bit newtypes.
//! The only fielded primitive types in the substrate.
//!
//! See `DESIGN.md` for the full surface.

mod bits;
mod meta_bits;
mod platform;

pub use arvo_strategy::{BitPrim, MultiContainer};
pub use bits::Bits;
pub use meta_bits::{FBits, IBits, MetaCarrier, Width, fbits, ibits, width};
pub use platform::{AsBool, Bool, Cap, USize};
