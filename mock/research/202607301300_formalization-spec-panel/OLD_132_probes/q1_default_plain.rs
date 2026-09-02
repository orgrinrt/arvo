//! Can a const-param default name an earlier const param? Zero features.
#![no_std]
pub struct Foo<const N: usize, const M: usize = N>([u8; M]);
pub type Bar<const N: usize> = Foo<N>;
pub fn takes(_: Bar<4>) {}
