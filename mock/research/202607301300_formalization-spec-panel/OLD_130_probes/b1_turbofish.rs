#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");
// Can a generic wrapper COMPUTE its output coordinates in a turbofish?
pub fn square_computed<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<I, F, C, S> {
    mul::<I, F, I, F, { I + I }, { F + F }, C, C, S>(x, x)
}
