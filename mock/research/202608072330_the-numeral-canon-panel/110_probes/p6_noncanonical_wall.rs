//! P6b. The cost of parameterising by an axis the realisation map does not read.
//!
//! This file is EXPECTED TO FAIL TO COMPILE. The failure is the result.
//!
//! `p6_expressibility.rs` parameterises a primitive by the grid step, which is
//! what P5's criterion says the value set and the realisation map actually
//! read. Under that parameterisation the two spellings of the degenerate
//! primitive are one type and unify silently.
//!
//! Here is the alternative: carry the radix and the fraction width as separate
//! parameters, which is the shape a reader gets from "a primitive is a named
//! composition of a format, a number system, a law set and a strategy". It
//! reads more informatively. It also makes `radix 2, F = 0` and `radix 10,
//! F = 0` two distinct types denoting one primitive, and Rust's nominal typing
//! means there is no later repair: no impl, no blanket, no const predicate
//! turns two type constructors applied to different arguments into one type
//! without `generic_const_exprs`, which is forbidden.
//!
//! So the naming decision is not cosmetic and is not recoverable downstream.
//! It is made once, when the parameter list is chosen, and it is load-bearing
//! for every consumer that later wants to write one function over both.
//!
//! Build: rustc --edition 2021 --crate-type lib p6_noncanonical_wall.rs
//! Expected: E0308 mismatched types.

pub trait Rounding {}
pub struct Near;
impl Rounding for Near {}

pub trait Policy {}
pub struct Sat;
impl Policy for Sat {}

/// The axis-carrying parameterisation: radix and fraction width are separate
/// const parameters, exactly as the four-part decomposition suggests.
pub struct FxAxes<const LO: i128, const HI: i128, const RADIX: u32, const F: u32, R, P>(
    core::marker::PhantomData<(R, P)>,
);

#[repr(transparent)]
pub struct Num<T>(pub i128, pub core::marker::PhantomData<T>);

/// "Eight-bit unsigned, binary, no fraction."
pub type U8Binary = FxAxes<0, 255, 2, 0, Near, Sat>;
/// "Eight-bit unsigned, decimal, no fraction."
///
/// P5 established that at F = 0 the step is radix^0 = 1 and NOTHING reads the
/// radix: 0 of 108 configuration points made it observable, under every one of
/// the three signatures swept, and the direct test over the whole rational
/// line confirmed the realisation map does not read it. These two are the same
/// primitive by the criterion.
pub type U8Decimal = FxAxes<0, 255, 10, 0, Near, Sat>;

pub fn takes_binary(x: Num<U8Binary>) -> i128 {
    x.0
}

/// The wall. These denote one primitive and are two types, so this call is a
/// type error, and no amount of later design repairs it.
pub fn the_wall(d: Num<U8Decimal>) -> i128 {
    takes_binary(d)
}
