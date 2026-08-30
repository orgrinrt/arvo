#![no_std]
use arvocore::*;
pub fn no_annotation(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _c = mul(a, b);
}
