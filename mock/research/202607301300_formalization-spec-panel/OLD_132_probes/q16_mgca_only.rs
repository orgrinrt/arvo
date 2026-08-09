//! Step A (derive the byte count from the widths) under min_generic_const_args ALONE.
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub const fn bytes_for(w: u32) -> usize {
    (w as usize).div_ceil(8)
}
pub struct Rung<const I: u32, const F: u32>(PhantomData<([(); I as usize], [(); F as usize])>);
pub trait Tagged {
    type const BYTES: usize;
}
impl<const I: u32, const F: u32> Tagged for Rung<I, F> {
    type const BYTES: usize = const { bytes_for(I + F) };
}
pub struct Bytes<const B: usize>([u8; B]);
pub struct Fixed<const I: u32, const F: u32>(Bytes<{ <Rung<I, F> as Tagged>::BYTES }>);
