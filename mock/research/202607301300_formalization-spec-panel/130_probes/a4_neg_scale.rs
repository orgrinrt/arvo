#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");

// The exact line file 129's capstone writes at q13_capstone.rs:60, which
// compiles there. Here it is a type error, at the line that wrote it.
pub fn reinterpret(a: UFixed<13, 3, u16, Warm>) {
    let _b: UFixed<8, 8, u16, Warm> = a;
}

// And adding two differently scaled numerals, which 129's add accepts.
pub fn misaligned(a: UFixed<13, 3, u16, Warm>, b: UFixed<8, 8, u16, Warm>) {
    let _s: UFixed<14, 3, u16, Warm> = add(a, b);
}
