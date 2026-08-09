// The downstream optimisation layer's row: read the typestate, at concrete
// numerals, with no feature gate and no flag.
#![no_std]
extern crate acore;
use acore::{Hot, Lowering, UFixed, Warm};

pub fn container_type(x: UFixed<13, 3, Hot>) -> u16 {
    x.to_raw()
}
pub fn wide_container(x: UFixed<200, 100, Hot>) -> [u8; 38] {
    x.to_raw().0
}

pub const W1: u32 = <UFixed<13, 3, Hot> as Lowering>::STORED_WIDTH;
pub const B1: usize = <UFixed<13, 3, Hot> as Lowering>::BYTES;
pub const W2: u32 = <UFixed<40, 30, Warm> as Lowering>::STORED_WIDTH;

// naming the projected container as a type, which is what a layout pass needs
pub fn takes_container(_: <UFixed<13, 3, Warm> as Lowering>::Container) {}

const _: () = assert!(W1 == 16 && B1 == 2 && W2 == 70);
