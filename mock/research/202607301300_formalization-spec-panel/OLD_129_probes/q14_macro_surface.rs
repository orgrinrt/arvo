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
impl Container for u64 {
    const BITS: u32 = 64;
}
pub struct Fx<const P: u32, C: Container, S> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, C: Container, S> Clone for Fx<P, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, C: Container, S> Copy for Fx<P, C, S> {}
impl<const P: u32, C: Container, S> Fx<P, C, S> {
    const FITS: () = assert!(P <= C::BITS, "precision does not fit its container");
    pub fn new(raw: C) -> Self {
        let () = Self::FITS;
        Fx {
            raw,
            _s: PhantomData,
        }
    }
}

/// The surface spelling. The addition happens where the consumer writes it,
/// which is a concrete site, so it is ordinary const eval.
#[macro_export]
macro_rules! UFixed {
    ($i:literal, $f:literal, $c:ty, $s:ty) => { $crate::Fx<{ $i + $f }, $c, $s> };
}

pub fn wants16(_: Fx<16, u16, Warm>) {}
pub fn consumer(x: UFixed!(13, 3, u16, Warm), y: UFixed!(8, 8, u16, Warm)) {
    wants16(x);
    wants16(y);
    let _z: UFixed!(40, 24, u64, Warm) = Fx::new(0);
}
