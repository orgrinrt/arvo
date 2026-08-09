//! P10. The constructive answer to the wall P1, P4, P5 and P9 establish.
//!
//! Rust has no defaulting for inference variables, so an output that is
//! silently defaulted and explicitly overridable cannot be an unconstrained
//! type parameter. It can be a projection, because a projection is
//! determined. The consumer then chooses at the input rather than at the
//! output, which is where a consumer has something to say anyway.
//!
//! Two shapes are built and both are exercised with no annotation at the call
//! site.
//!
//! Shape A: the output projects off the input type. Doing nothing yields the
//! same T back, which is op's "perhaps the very same T: Add they piped in".
//!
//! Shape B: the consumer marks the input to ask for the simplified form. The
//! marker is a wrapper carrying no data, so the ask costs nothing at runtime
//! and is visible in the type.
//!
//! Expected: both compile, and neither call site names an output type.

#![no_std]

/// What the consumer piped in.
#[derive(Clone, Copy)]
pub struct Piped(pub u32);

/// The simplified form the design would hand back on request.
#[derive(Clone, Copy)]
pub struct Simplified(pub u32);

/// The ask, spelled at the input. Zero-sized wrapper, no runtime presence.
#[derive(Clone, Copy)]
pub struct Simplify<T>(pub T);

/// The projection. One impl per input type, so nothing is ambiguous.
pub trait Returns {
    type Out;
    fn build(raw: u32) -> Self::Out;
}

impl Returns for Piped {
    type Out = Piped;
    fn build(raw: u32) -> Piped {
        Piped(raw)
    }
}

impl<T> Returns for Simplify<T> {
    type Out = Simplified;
    fn build(raw: u32) -> Simplified {
        Simplified(raw)
    }
}

pub trait Raw {
    fn raw(&self) -> u32;
}
impl Raw for Piped {
    fn raw(&self) -> u32 {
        self.0
    }
}
impl<T: Raw> Raw for Simplify<T> {
    fn raw(&self) -> u32 {
        self.0.raw()
    }
}

/// The algorithm. Its output is a projection off its input, so the return
/// type is determined and no annotation is ever needed.
pub fn dot<T: Returns + Raw>(a: T, b: T) -> T::Out {
    T::build(a.raw().wrapping_mul(b.raw()))
}

/// Shape A. Nothing said, the same shape back.
pub fn plain(a: Piped, b: Piped) -> Piped {
    dot(a, b)
}

/// Shape B. The ask is at the input, and the output follows.
pub fn asked(a: Piped, b: Piped) -> Simplified {
    dot(Simplify(a), Simplify(b))
}

/// Neither call site annotated anything. This one has no return-position clue
/// at all, which is where P4 failed.
pub fn sink<X>(_x: X) {}
pub fn no_context(a: Piped, b: Piped) {
    sink(dot(a, b));
    sink(dot(Simplify(a), Simplify(b)));
}
