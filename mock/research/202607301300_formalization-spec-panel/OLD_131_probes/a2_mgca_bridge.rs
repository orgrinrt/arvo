// Under min_generic_const_args alone: can a COMPUTED tag reach type position
// via a `type const` bridge?  g2 of 129 showed a BARE parameter round-trips.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub trait Project<const TAG: usize> {
    type T: Copy;
}
pub struct Picker;
impl Project<0> for Picker {
    type T = u8;
}
impl Project<1> for Picker {
    type T = u16;
}

pub const fn tag(n: u32) -> usize {
    if n <= 8 {
        0
    } else {
        1
    }
}

pub struct W<const N: u32>;
pub trait HasTag {
    type const TAG: usize;
}

// (a) bare parameter round trip: expected to work (129 g2)
pub trait HasN {
    type const V: u32;
}
impl<const N: u32> HasN for W<N> {
    type const V: u32 = N;
}
pub fn roundtrip<const N: u32>(_: W<N>) -> W<{ <W<N> as HasN>::V }> {
    W
}

// (b) the computed tag: the thing the projection actually needs
impl<const N: u32> HasTag for W<N> {
    type const TAG: usize = tag(N);
}
