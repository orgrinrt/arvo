//! Probe 3b: the negative control for probe 3. The exponent as a const parameter, with
//! `mulnum` computing the result bounds, is refused by the compiler.
//!
//! rustc --edition 2021 --crate-type lib probe_3b_exponent_as_const_refused.rs
//!
//! This is the compiled half of section 1.15's reasoned derivation: it is not that the type
//! form is nicer, it is that the const form does not exist without a forbidden feature.

#![allow(dead_code)]

use core::marker::PhantomData;

pub struct Ranged<const EMIN: i32, const EMAX: i32>;
pub struct Fl<const P: u32, const EMIN: i32, const EMAX: i32>(PhantomData<()>);

pub trait MulNum<Rhs> {
    type Out;
}

impl<
        const P1: u32,
        const E1N: i32,
        const E1X: i32,
        const P2: u32,
        const E2N: i32,
        const E2X: i32,
    > MulNum<Fl<P2, E2N, E2X>> for Fl<P1, E1N, E1X>
{
    // Every one of these three is a quantity computed from the operands that has to appear
    // in the result's type. That is the spine rule's condition, and const position cannot
    // hold it.
    type Out = Fl<{ P1 + P2 }, { E1N + E2N }, { E1X + E2X }>;
}
