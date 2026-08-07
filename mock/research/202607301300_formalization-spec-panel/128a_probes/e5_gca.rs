//! E5: `generic_const_args`, the feature rustc named in E2d. Neither on the
//! allowed nor the forbidden list, so unvetted. Does it close the gap at all?
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

pub struct W<const N: u16>;

pub trait Add2<const I: u16, const F: u16> {
    type const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    type const SUM: u16 = const { I + F };
}

pub type PrecisionOf<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;

fn takes16(_: W<16>) {}
fn probe() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}
