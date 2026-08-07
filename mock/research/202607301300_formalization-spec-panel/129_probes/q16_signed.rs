#![no_std]
#![allow(dead_code)]
use core::marker::PhantomData;
pub struct Warm;
pub struct Signed;
pub struct Unsigned;
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u16 {
    const BITS: u32 = 16;
}
impl Container for i16 {
    const BITS: u32 = 16;
}
pub struct Fx<const P: u32, C: Container, Sign, S> {
    raw: C,
    _p: PhantomData<(Sign, S)>,
}
impl<const P: u32, C: Container, Sign, S> Clone for Fx<P, C, Sign, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, C: Container, Sign, S> Copy for Fx<P, C, Sign, S> {}
impl<const P: u32, C: Container, Sign, S> Fx<P, C, Sign, S> {
    const FITS: () = assert!(P <= C::BITS, "precision does not fit its container");
    pub fn new(raw: C) -> Self {
        let () = Self::FITS;
        Fx {
            raw,
            _p: PhantomData,
        }
    }
}
macro_rules! UFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ $i + $f }, $c, Unsigned, $s> }; }
macro_rules! IFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ 1 + $i + $f }, $c, Signed, $s> }; }

pub fn wants16u(_: Fx<16, u16, Unsigned, Warm>) {}
pub fn wants16i(_: Fx<16, i16, Signed, Warm>) {}
pub fn check(
    a: UFixed!(13, 3, u16, Warm),
    b: UFixed!(8, 8, u16, Warm),
    c: IFixed!(12, 3, i16, Warm),
    d: IFixed!(7, 8, i16, Warm),
) {
    wants16u(a);
    wants16u(b);
    wants16i(c);
    wants16i(d);
}
