#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
// const_convert is not stale: on the pinned nightly the feature is still named
// `const_convert` and points at the live const-From/Into umbrella (rustc #143773).
// The `const_from` successor name is not yet the gate name here. Reclassified by
// the soundness sweep (task #624); keep the gate.
#![feature(const_convert)]
#![feature(const_ops)]
#![feature(const_param_ty_trait)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![allow(incomplete_features)]

//! arvo-storage. Minimal L0 storage primitives.
//!
//! `Bits<N, S>` storage primitive. `Bool`, `USize`, `Cap` platform
//! wrappers. `IBits`, `FBits`, `Width` typed meta-bit newtypes.
//! The only fielded primitive types in the substrate.
//!
//! See `DESIGN.md` for the full surface.

mod bits;
mod bridges;
mod layout_assertions;
mod meta_bits;
mod platform;

pub use arvo_strategy::{A1, A16, A32, A64, Align, Bounded, Identity, SignedIdentity, WideBits};
pub use bits::Bits;
pub use bridges::{
    ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq,
};
pub use meta_bits::{FBits, IBits, MetaCarrier, Width, fbits, ibits, width};
pub use platform::{AsBool, Bool, BoolResidual, Cap, NUSize, USize};
