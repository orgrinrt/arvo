#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
extern crate gcelib;
pub struct W<const N: u16>;
pub trait Add2<const I: u16, const F: u16> {
    type const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    type const SUM: u16 = const { I + F };
}
pub type PrecisionOf<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;
pub fn takes16(_: W<16>) {}
// touch the GCE crate's surface from a GCA crate
pub fn use_gce(a: gcelib::A<4>) -> gcelib::A<5> {
    gcelib::widen(a)
}
