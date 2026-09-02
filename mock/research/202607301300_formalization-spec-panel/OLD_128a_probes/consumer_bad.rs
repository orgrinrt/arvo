#![no_std]
extern crate libarvo;
use libarvo::{takes16, PrecisionOf};
pub fn use_it() {
    takes16(PrecisionOf::<13, 4> {}); // 17, must be refused
}
