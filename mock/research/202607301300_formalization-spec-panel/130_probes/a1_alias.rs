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

// the brief's claim: a generic type alias computing the sum is refused
pub type UFixed<const I: u32, const F: u32, C, S> = Fx<{ I + F }, C, S>;
