#![no_std]
extern crate core2;
use core2::*;
pub struct Mine; // consumer's own marker
impl ToNat<Mine> for Idx<777> {
    const VAL: u32 = 777;
}
impl ToNat<Mine> for Idx<41> {
    const VAL: u32 = 41;
}
pub fn arvo_width(x: &UFixed<13, 3, Warm>) -> (u32, u32) {
    widths(x)
} // op's surface, no marker
pub fn my_width(x: &Fixed<777, 41, Warm, Mine>) -> (u32, u32) {
    widths(x)
} // a width arvo never listed
