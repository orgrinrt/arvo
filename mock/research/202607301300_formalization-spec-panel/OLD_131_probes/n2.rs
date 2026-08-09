#![no_std]
use arvocore::*;
pub fn wrong_product(a: UFixed<1000, 1000, Precise>, b: UFixed<1200, 1300, Precise>) {
    let _p: UFixed<2201, 2300, Precise> = mul(a, b);
}
