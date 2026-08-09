// Minimal reproduction: a GAT whose body is a `type const` projection.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
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
pub struct Rung<const N: u32>;
pub trait Tagged {
    type const TAG: usize;
}
impl<const N: u32> Tagged for Rung<N> {
    type const TAG: usize = const { tag(N) };
}

pub trait Lowering {
    type Store<const N: u32>: Copy;
}
pub struct Hot;
impl Lowering for Hot {
    type Store<const N: u32> = <Picker as Project<{ <Rung<N> as Tagged>::TAG }>>::T;
}
