#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub const fn tag(n: u32) -> usize {
    if n <= 8 {
        0
    } else {
        1
    }
}
pub struct W<const N: u32>;
pub trait HasTag {
    type const TAG: usize;
}
impl<const N: u32> HasTag for W<N> {
    type const TAG: usize = const { tag(N) };
}
