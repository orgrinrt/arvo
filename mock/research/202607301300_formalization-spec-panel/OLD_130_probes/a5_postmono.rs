#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");

// the wrapper the author gets wrong. Wrong for every I, F. Never instantiated here.
pub fn square_wrong<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<I, F, C, S> {
    mul::<I, F, I, F, I, F, C, C, S>(x, x)
}
