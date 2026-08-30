#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct W<const N: u32>;
pub const fn add(a: u32, b: u32) -> u32 {
    a + b
}
pub trait S {
    type const V: u32;
}
impl<const P: u32, const Q: u32> S for (W<P>, W<Q>) {
    type const V: u32 = add(P, Q);
}
