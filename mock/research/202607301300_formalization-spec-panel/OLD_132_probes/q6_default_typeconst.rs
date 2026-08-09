//! Default = a `type const` projection. Does the default route work at all under GCA?
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub const fn bytes_for(w: usize) -> usize {
    w.div_ceil(8)
}
pub struct Rung<const N: usize>(PhantomData<[(); N]>);
pub trait Tagged {
    type const BYTES: usize;
}
impl<const N: usize> Tagged for Rung<N> {
    type const BYTES: usize = const { bytes_for(N) };
}
pub struct Foo<const N: usize, const B: usize = { <Rung<N> as Tagged>::BYTES }>([u8; B]);
pub type Bar<const N: usize> = Foo<N>;
pub fn takes(_: Bar<20>) {}
