#![no_std]
// NOTE: no feature gates, no -Z flag. A plain downstream consumer.
extern crate libarvo;
use libarvo::{takes16, PrecisionOf};
pub fn use_it() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}
