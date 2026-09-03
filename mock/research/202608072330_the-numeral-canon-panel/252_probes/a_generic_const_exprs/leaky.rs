// Arm A, the leaky side. A library that uses `generic_const_exprs` in a PUBLIC
// signature: the return type is a const expression over the caller's parameter.
//
// This is the exact shape `obligation::the_unstable_machinery_does_not_reach_a_consumer`
// says is unmeasured: "Whether a `generic_const_exprs` bound in a public
// signature can be hidden from a consumer at all is exactly the open question."
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct W<const N: usize>;

/// The const expression is in the return type, so it is part of the signature a
/// consumer resolves against rather than something the body hides.
pub const fn widen<const N: usize>(_: W<N>) -> W<{ N + 1 }> {
    W
}

/// The control item: same crate, same feature enabled, but this signature holds
/// no const expression at all. A consumer naming only this must build, or the
/// experiment is measuring "the crate is poisoned" rather than "the bound leaks".
pub const fn identity<const N: usize>(w: W<N>) -> W<N> {
    w
}
