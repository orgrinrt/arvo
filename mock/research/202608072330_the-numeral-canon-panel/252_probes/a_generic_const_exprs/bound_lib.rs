// Arm A3. `obligation::the_unstable_machinery_does_not_reach_a_consumer` says
// "a `generic_const_exprs` bound in a public signature", and arm A measured a
// const expression in a RETURN TYPE, which is a different position. This is the
// bound shape: the const expression sits in a `where` clause on a public fn.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct Bits<const N: usize>;

pub trait Fits {}
impl<const N: usize> Fits for Bits<N> {}

/// The const expression is in the `where` clause rather than in a type the
/// caller writes.
pub const fn joined<const A: usize, const B: usize>() -> usize
where
    Bits<{ A + B }>: Fits,
{
    A + B
}
