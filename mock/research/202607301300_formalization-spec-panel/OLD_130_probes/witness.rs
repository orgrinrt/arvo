#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");

pub fn square_wrong<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<I, F, C, S> {
    mul::<I, F, I, F, I, F, C, C, S>(x, x)
}

// One line per generic wrapper, in the library's own crate. Does naming the
// function item at a representative instantiation force its consts?
const _: () = {
    let _ = square_wrong::<13, 3, u16, Warm>;
};
