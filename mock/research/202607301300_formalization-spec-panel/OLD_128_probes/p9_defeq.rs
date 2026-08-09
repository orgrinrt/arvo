//! P9: canonicity in a GENERIC context, where const eval cannot run.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub struct W<const N: u16>;
pub trait Add2<const I: u16, const F: u16> {
    type const SUM: u16;
}
pub struct Adder;
pub struct Adder2;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    type const SUM: u16 = const { I + F };
}
// same VALUE, different definition path
impl<const I: u16, const F: u16> Add2<I, F> for Adder2 {
    type const SUM: u16 = const { F + I };
}

pub type ViaA<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;
pub type ViaB<const I: u16, const F: u16> = W<{ <Adder2 as Add2<I, F>>::SUM }>;

// concrete: both should be W<16>
pub fn concrete(x: ViaA<13, 3>) -> ViaB<8, 8> {
    x
}
