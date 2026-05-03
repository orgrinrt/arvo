#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
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

pub mod aligned_widebits;
mod bits;
mod bridges;
mod layout_assertions;
mod meta_bits;
mod platform;
pub mod widebits;

pub use aligned_widebits::AlignedWideBits16;
pub use arvo_strategy::{Bounded, Identity, MultiContainer, MultiContainerHalf, SignedIdentity};
pub use bits::Bits;
pub use bridges::{
    ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq,
};
pub use meta_bits::{FBits, IBits, MetaCarrier, Width, fbits, ibits, width};
pub use platform::{AsBool, Bool, Cap, USize};
pub use widebits::WideBits;
