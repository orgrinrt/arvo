#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct W<const N: u32>;
pub struct T3<const A: u32, const B: u32, const C: u32>;

pub trait L {
    type const V: u32;
} // (A + B) + C
pub trait R {
    type const V: u32;
} // A + (B + C)
impl<const A: u32, const B: u32, const C: u32> L for T3<A, B, C> {
    type const V: u32 = const { (A + B) + C };
}
impl<const A: u32, const B: u32, const C: u32> R for T3<A, B, C> {
    type const V: u32 = const { A + (B + C) };
}

pub fn generic<const A: u32, const B: u32, const C: u32>(
    x: W<{ <T3<A, B, C> as L>::V }>,
) -> W<{ <T3<A, B, C> as R>::V }> {
    x
}
