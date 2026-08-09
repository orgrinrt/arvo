// Can the container be PROJECTED from (strategy, width, sign) with no gate?
// This is the shipped mechanism (arvo-strategy/src/container.rs:254-259) with
// the forbidden generic_const_exprs removed.
#![no_std]

pub trait Sign: Copy {}
#[derive(Clone, Copy)]
pub struct Unsigned;
impl Sign for Unsigned {}

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

pub const fn tag(n: u32) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else {
        2
    }
}

pub trait StoreFor<const N: u32, G: Sign> {
    type T: Copy;
}

impl<const N: u32, G: Sign> StoreFor<N, G> for Warm {
    type T = <Picker as Project<{ tag(N) }, G>>::T;
}
