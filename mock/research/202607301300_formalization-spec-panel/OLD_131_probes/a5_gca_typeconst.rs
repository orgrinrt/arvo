// GCA, taking rustc's own repair: factor the tag into a `type const` item.
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

// the type-const carrier: one per strategy
pub struct TagOf<const N: u32, S>(core::marker::PhantomData<S>);
pub trait HasTag {
    type const TAG: usize;
}
impl<const N: u32> HasTag for TagOf<N, Hot> {
    type const TAG: usize = const { tag_hot(N) };
}

pub trait StoreFor<const N: u32, G: Sign> {
    type T: Copy;
}
impl<const N: u32, G: Sign> StoreFor<N, G> for Hot
where
    Picker: Project<{ <TagOf<N, Hot> as HasTag>::TAG }, G>,
{
    type T = <Picker as Project<{ <TagOf<N, Hot> as HasTag>::TAG }, G>>::T;
}

pub fn concrete13(x: <Hot as StoreFor<13, Unsigned>>::T) -> u16 {
    x
}
pub fn concrete3(x: <Hot as StoreFor<3, Unsigned>>::T) -> u8 {
    x
}
pub fn concrete47(x: <Hot as StoreFor<47, Unsigned>>::T) -> u64 {
    x
}
