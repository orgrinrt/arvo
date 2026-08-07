// P1. Can an associated const sit in const-argument position under min_generic_const_args?
// If yes, width arithmetic never needs a structural magnitude and never needs a bridge.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]

pub struct Idx<const N: u32>;

pub trait Sum2 {
    const S: u32;
}

pub struct Pair<const A: u32, const B: u32>;
impl<const A: u32, const B: u32> Sum2 for Pair<A, B> {
    const S: u32 = A + B;
}

// The question: is `Idx<{ <Pair<A,B> as Sum2>::S }>` a legal type here?
pub fn total<const A: u32, const B: u32>() -> Idx<{ <Pair<A, B> as Sum2>::S }> {
    Idx
}
