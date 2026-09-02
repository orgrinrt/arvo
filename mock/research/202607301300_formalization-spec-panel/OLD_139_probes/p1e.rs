#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct Idx<const N: u32>;
pub trait Sum2 {
    type const S: u32;
}
pub struct Pair<const A: u32, const B: u32>;
impl<const A: u32, const B: u32> Pair<A, B> {
    const HELPER: u32 = A + B;
}
impl<const A: u32, const B: u32> Sum2 for Pair<A, B> {
    type const S: u32 = Self::HELPER;
}
pub fn a<const A: u32, const B: u32>() -> Idx<{ <Pair<A, B> as Sum2>::S }> {
    Idx
}
