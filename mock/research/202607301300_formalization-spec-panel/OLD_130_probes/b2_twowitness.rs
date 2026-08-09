#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");
// A wrapper whose output coordinates are LITERALS: right at 13.3, wrong elsewhere.
pub fn square_literal<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<26, 6, C, S> {
    mul::<I, F, I, F, 26, 6, C, C, S>(x, x)
}
// witness one, at 13.3: passes, and proves nothing
pub fn w1(x: UFixed<13, 3, u16, Warm>) -> UFixed<26, 6, u16, Warm> {
    square_literal(x)
}
// witness two, at a different width: catches it
pub fn w2(x: UFixed<7, 2, u16, Warm>) -> UFixed<26, 6, u16, Warm> {
    square_literal(x)
}
