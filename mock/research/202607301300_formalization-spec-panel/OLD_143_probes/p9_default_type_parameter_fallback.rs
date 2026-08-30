//! P9. Rust once had the mechanism op describes, as a feature gate. Does it
//! still exist on the pinned toolchain?
//!
//! `default_type_parameter_fallback` (rust-lang/rust#27336) made a type
//! parameter's default act as an inference fallback rather than a syntactic
//! elision, which is exactly "a generic to describe the output whose default
//! is taken when the consumer says nothing".
//!
//! Expected: E0635, unknown feature. Recorded because it settles that the
//! wall P1 and P4 hit is the language's, not the spelling's, and that no
//! nightly gate buys past it.

#![no_std]
#![feature(default_type_parameter_fallback)]

pub struct Keep;
pub struct Erase;

pub trait Dot<Out = Keep> {
    fn dot(self) -> Out;
}
