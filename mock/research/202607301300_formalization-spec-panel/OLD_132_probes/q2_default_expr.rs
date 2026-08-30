//! Can a const-param default be an EXPRESSION over earlier params? Zero features.
#![no_std]
pub const fn bytes_for(w: usize) -> usize {
    w.div_ceil(8)
}
pub struct Foo<const N: usize, const B: usize = { bytes_for(N) }>([u8; B]);
pub type Bar<const N: usize> = Foo<N>;
pub fn takes(_: Bar<20>) {}
