//! arvo — numeric primitive substrate.
//!
//! `#![no_std]`, no alloc, no platform dependency. Every size is
//! const at type level. Every numeric type that has a precision /
//! throughput tradeoff carries a `Strategy` marker.
//!
//! L0 of the arvo stack. Consumers compose concrete types from the
//! primitives here; semantic domain aliases (angle, coord, ratio…)
//! are defined by downstream crates that know the `<I, F>` split
//! they want.
//!
//! See `DESIGN.md` for the full substrate layout.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![feature(try_trait_v2)]
#![allow(incomplete_features)]

pub mod aliases;
pub mod bitfield;
pub mod float;
pub mod ifixed;
pub mod ifixed_impl;
pub mod markers;
pub mod predicate;
pub mod prim;
pub mod strategy;
pub mod traits;
pub mod transparent;
pub mod ufixed;
pub mod ufixed_impl;

pub use arvo_bits::{Bit, Byte, DWord, Nibble, QWord, Word};
pub use aliases::{Fixed, Int, Signed, Uint};
pub use arvo_numeric_contracts::{
    IsNonNegative, IsNonZero, IsPositive, IsZero, IsZeroOrPositive, Predicate,
};
pub use arvo_storage::{
    AsBool, Bits, Bool, Cap, FBits, IBits, USize, Width, fbits, ibits, width,
};
pub use float::{FastFloat, Float, Ieee, StrictFloat};
pub use ifixed::IFixed;
pub use markers::{BitPresentation, BoolLike, FloatLike, FractionLike, IntegerLike};
pub use predicate::{Pred, Pred2, Pred3};
pub use prim::{BitPrim, IBitContainer, IBitPrim, UBitContainer};
pub use strategy::{Cold, Hot, Precise, Strategy, Warm, width_le_64};
pub use traits::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
pub use transparent::{NumericPrimitive, Transparent, raw};
pub use ufixed::UFixed;
