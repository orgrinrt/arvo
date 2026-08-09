//! Probe 4b (negative control): `Implicit`'s exponent as a const parameter does not exist.
//!
//! rustc --edition 2021 --crate-type lib probe_4b_implicit_exponent_as_const_refused.rs
//!
//! File 50's `probe_3b` closed the const route for `Ranged`'s two bounds. This closes it for
//! `Implicit`'s single exponent, which file 50 explicitly did not test (`50:602-604`). The
//! shape is smaller (one exponent rather than two), and that matters: a reader might expect
//! a single const to survive where a pair did not, since the arithmetic is one addition
//! rather than two. It does not. The wall is about a generic parameter appearing in a const
//! operation at all, not about how many of them there are.

#![allow(dead_code)]

pub struct Fx<const P: u32, const E: i32>;

pub trait MulNumConst<Rhs> {
    type Out;
}

impl<const P1: u32, const E1: i32, const P2: u32, const E2: i32> MulNumConst<Fx<P2, E2>>
    for Fx<P1, E1>
{
    type Out = Fx<{ P1 + P2 }, { E1 + E2 }>;
}
