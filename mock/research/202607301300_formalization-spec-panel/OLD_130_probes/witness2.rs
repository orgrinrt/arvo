#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");
pub fn square_wrong<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<I, F, C, S> {
    mul::<I, F, I, F, I, F, C, C, S>(x, x)
}
#[doc(hidden)]
pub fn __witness_square(x: UFixed<13, 3, u16, Warm>) -> UFixed<13, 3, u16, Warm> {
    square_wrong::<13, 3, u16, Warm>(x)
}
