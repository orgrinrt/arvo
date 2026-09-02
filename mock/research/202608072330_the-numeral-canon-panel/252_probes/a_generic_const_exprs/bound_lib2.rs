// Arm A3's own control lib. A3 built clean in an ungated consumer, and that has
// two readings: the bound does not leak, or the bound is never checked in the
// consumer at all. A blanket impl cannot tell them apart.
//
// So `Small` is implemented for exactly three widths. A call whose const
// expression lands outside them must be refused in the consumer, or the
// consumer is not checking the bound and A3 measured nothing.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct Bits<const N: usize>;

pub trait Small {}
impl Small for Bits<0> {}
impl Small for Bits<7> {}
impl Small for Bits<13> {}

pub const fn joined<const A: usize, const B: usize>() -> usize
where
    Bits<{ A + B }>: Small,
{
    A + B
}
