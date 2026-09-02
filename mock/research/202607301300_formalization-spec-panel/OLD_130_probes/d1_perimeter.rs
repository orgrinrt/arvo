#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");
// Is an over-wide format merely NAMEABLE?
pub type Bad = UFixed<9, 0, u8, Warm>;
pub fn names_it(x: Bad) -> Bad {
    x
}
pub fn inhabits_it() -> Bad {
    UFixed::new(0)
}
