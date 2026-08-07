#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
extern crate gcalib2;
pub use gcalib2::{takes16, PrecisionOf, W};
pub struct A<const N: usize>(pub [u8; N]);
pub fn widen<const N: usize>(_: A<N>) -> A<{ N + 1 }> {
    A([0; N + 1])
}
// consume the GCA crate's normalized surface from a GCE crate, no flag here
pub fn cross() {
    takes16(gcalib2::PrecisionOf::<13, 3> {});
}
