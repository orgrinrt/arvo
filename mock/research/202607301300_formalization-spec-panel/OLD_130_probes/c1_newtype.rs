#![no_std]
#![allow(dead_code)]
use core::marker::PhantomData;
pub struct Warm;
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u16 {
    const BITS: u32 = 16;
}
pub struct Fx<const P: u32, C: Container, S> {
    raw: C,
    _s: PhantomData<S>,
}
// a newtype whose parameters are the consumer's two and whose field is the numeral
pub struct UFixed<const I: u32, const F: u32, C: Container, S>(Fx<{ I + F }, C, S>);
