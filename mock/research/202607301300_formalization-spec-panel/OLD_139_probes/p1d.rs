#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct Idx<const N: u32>;
pub trait Sum2 {
    type const S: u32;
}
pub struct Pair<const A: u32, const B: u32>;
// form 1: bare parameter as RHS
impl<const A: u32, const B: u32> Sum2 for Pair<A, B> {
    type const S: u32 = A;
}
// form 2: literal RHS on a concrete impl
pub struct Lit;
impl Sum2 for Lit {
    type const S: u32 = 16;
}
pub fn a<const A: u32, const B: u32>() -> Idx<{ <Pair<A, B> as Sum2>::S }> {
    Idx
}
pub fn b() -> Idx<{ <Lit as Sum2>::S }> {
    Idx
}
