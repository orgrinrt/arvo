// The shipped Pattern C projection (arvo-strategy/src/container.rs:254-259)
// with generic_const_exprs replaced by generic_const_args.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

pub trait Sign: Copy {}
#[derive(Clone, Copy)]
pub struct Unsigned;
#[derive(Clone, Copy)]
pub struct Signed;
impl Sign for Unsigned {}
impl Sign for Signed {}

pub struct Hot;
pub struct Warm;

pub trait Project<const TAG: usize, G: Sign> {
    type T: Copy;
}
pub struct Picker;
impl<G: Sign> Project<0, G> for Picker {
    type T = u8;
}
impl<G: Sign> Project<1, G> for Picker {
    type T = u16;
}
impl<G: Sign> Project<2, G> for Picker {
    type T = u32;
}
impl<G: Sign> Project<3, G> for Picker {
    type T = u64;
}

pub const fn tag_hot(n: u32) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else {
        3
    }
}

pub trait StoreFor<const N: u32, G: Sign> {
    type T: Copy;
}
impl<const N: u32, G: Sign> StoreFor<N, G> for Hot {
    type T = <Picker as Project<const { tag_hot(N) }, G>>::T;
}

// does it resolve at a concrete site?
pub fn concrete(x: <Hot as StoreFor<13, Unsigned>>::T) -> u16 {
    x
}
