#![no_std]
#![allow(dead_code)]
include!("cap_core.rs");
pub fn f(a: UFixed<13, 3, u16, Warm>, b: UFixed<8, 8, u16, Warm>) {
    let _s: UFixed<14, 3, u32, Warm> = add(a, b);
}
