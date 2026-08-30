#![no_std]
use arvocore::*;
pub fn misaligned(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _s: UFixed<14, 3, Warm> = add(a, b);
}
