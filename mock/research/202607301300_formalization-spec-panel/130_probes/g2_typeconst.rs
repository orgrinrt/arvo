#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
#![no_std]
#![allow(dead_code)]
use core::marker::PhantomData;
pub struct F<const I: u32, const K: u32>(PhantomData<()>);
pub trait Format {
    type const PRECISION: u32;
}
impl<const I: u32, const K: u32> Format for F<I, K> {
    type const PRECISION: u32 = I + K;
}
