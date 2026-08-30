#![no_std]
extern crate libarvo;
use libarvo::{takes16, PrecisionOf};
pub fn use_bad() {
    takes16(PrecisionOf::<13, 4> {});
}
