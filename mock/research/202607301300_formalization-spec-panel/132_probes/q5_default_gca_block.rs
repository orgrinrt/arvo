//! Const-param default as a `const {}` block under mGCA + GCA.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub const fn bytes_for(w: usize) -> usize {
    w.div_ceil(8)
}
pub struct Foo<const N: usize, const B: usize = const { bytes_for(N) }>([u8; B]);
pub type Bar<const N: usize> = Foo<N>;
pub fn takes(_: Bar<20>) {}
