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
#![feature(const_ops)]
// WATCH-tier unstable feature, soundness-vetted in the stack sweep (task #626).
// `generic_const_exprs` is used here only for const-expression bounds and const-
// generic array lengths (`[(); cap_size(N)]:` array sizing, `[(); EXPR]:` compile-
// time assertions, width arithmetic in const-generic position). Its one known
// unsoundness (#97156, const `TypeId` resolved into types with higher-ranked-trait-
// bound subtyping) is unreachable: the stack bans `TypeId`. Builds clean on the
// pinned nightly. Migration to `generic_const_args` is tracked: #628.
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

pub mod aliases;
pub mod bitfield;
pub(crate) mod fixed_scale;
pub mod float;
pub mod ifixed;
pub mod ifixed_impl;
mod layout_assertions;
pub mod markers;
pub mod predicate;
pub mod prim;
pub mod strategy;
pub mod traits;
pub mod transparent;
pub mod ufixed;
pub mod ufixed_impl;

pub use aliases::{Fixed, Int, Signed, Uint};
pub use arvo_bits::{
    Bit, BitsRefitCtor, Byte, DWord, Narrow, Narrowed, Nibble, QWord, Widen, Widened, Word,
};
pub use arvo_numeric_contracts::{
    IsNonNegative, IsNonZero, IsPositive, IsZero, IsZeroOrPositive, Predicate,
};
pub use arvo_storage::{
    fbits, ibits, width, AsBool, Bits, Bool, Cap, FBits, IBits, NUSize, USize, Width,
};
pub use float::{FastFloat, Float, StrictFloat};
pub use ifixed::IFixed;
pub use markers::{BitPresentation, BoolLike, FloatLike, FractionLike, IntegerLike};
pub use notko::{Just, Maybe, Outcome};
pub use predicate::{Pred, Pred2, Pred3};
pub use prim::{BitPrim, IBitContainer, IBitPrim, UBitContainer};
pub use strategy::{
    tag_one_representable, width_le_64, Additive, Bounded, Cold, ConstBitEq, ConstDefault, ConstEq,
    ConstOrd, ConstOrdering, ConstPartialEq, FromU8Ieee, Hot, Identity, Ieee, Multiplicative,
    OneRepresentable, Picker, Precise, SignedIdentity, Signedness, Strategy, Unsigned, Warm,
};
pub use traits::{
    Abs, EuclidDiv, EvenShares, EvenSplittable, FromConstant, Recip, ScalarEuclid, Sqrt, TotalOrd,
};
pub use transparent::{raw, NumericPrimitive, Transparent};
pub use ufixed::UFixed;
