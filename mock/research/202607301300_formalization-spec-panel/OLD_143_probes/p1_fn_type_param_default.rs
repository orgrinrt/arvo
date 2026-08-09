//! P1. Does a function type parameter accept a default?
//!
//! Op's tier-one mechanism: "it will have a generic to describe the output,
//! and they can override its default whatever that might be". The obvious
//! spelling puts the default on the function's own type parameter.
//!
//! Expected: refused. Recorded because the refusal decides where the default
//! is allowed to live.

#![no_std]

pub trait Numeralish {}

/// The obvious spelling of "a generic describing the output, with a default".
pub fn dot<T: Numeralish, O = T>(_a: T, _b: T) -> O {
    unimplemented!()
}
