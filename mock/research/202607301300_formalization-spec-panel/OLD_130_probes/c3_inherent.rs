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
pub struct Spec<const I: u32, const F: u32, C, S>(PhantomData<(C, S)>);
impl<const I: u32, const F: u32, C: Container, S> Spec<I, F, C, S> {
    pub type Of = Fx<{ I + F }, C, S>;
}
