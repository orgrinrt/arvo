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
#![feature(generic_const_exprs)]
#![feature(try_trait_v2)]
#![allow(incomplete_features)]

pub mod float;
pub mod ifixed;
pub mod markers;
pub mod newtype;
pub mod predicate;
pub mod strategy;
pub mod traits;
pub mod ufixed;

pub use float::{FastFloat, Float, Ieee, StrictFloat};
pub use ifixed::IFixed;
pub use markers::{BitPresentation, BoolLike, FloatLike, FractionLike, IntegerLike};
pub use newtype::{AsBool, Bool, Cap, FBits, IBits, USize};
pub use predicate::{Pred, Pred2, Pred3};
pub use strategy::{Cold, Hot, Precise, Strategy, Warm};
pub use traits::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
pub use ufixed::UFixed;
