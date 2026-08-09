#![no_std]
#![allow(dead_code)]
include!("cap_core.rs");
pub fn f(a: UFixed<13, 3, u16, Warm>, b: UFixed<13, 3, u16, Warm>) {
    let c = mul(a, b);
    let _ = c;
}
