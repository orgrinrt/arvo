//! E2c: take rustc's own suggested repair from E2b: `type const`.
//! The standing base (110:3620-3621) claims a `type const` body may not
//! compute from a generic parameter. Tested here rather than cited.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct W<const N: u16>;

pub trait Add2<const I: u16, const F: u16> {
    type const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    type const SUM: u16 = I + F;
}

pub type PrecisionOf<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;

fn takes16(_: W<16>) {}
fn probe() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}
