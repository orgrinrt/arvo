#![no_std]
#![allow(dead_code)]
//! File 129's numeral, unmodified, plus one thing 129 did not write: a decode.
//! The point is that the format's scale is not in the type, so the decode is
//! not a function of the type, and the type system cannot say the two disagree.
use core::marker::PhantomData;
pub struct Warm;
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u16 {
    const BITS: u32 = 16;
}

pub struct Fx<const P: u32, C: Container, S> {
    pub raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, C: Container, S> Clone for Fx<P, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, C: Container, S> Copy for Fx<P, C, S> {}
impl<const P: u32, C: Container, S> Fx<P, C, S> {
    pub const fn new(raw: C) -> Self {
        Fx {
            raw,
            _s: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! UFixed {
    ($i:literal, $f:literal, $c:ty, $s:ty) => { $crate::Fx<{ $i + $f }, $c, $s> };
}

// Q13.3 one is raw 1<<3 == 8.  Q8.8 one is raw 1<<8 == 256.
pub const ONE_Q13_3: UFixed!(13, 3, u16, Warm) = Fx::new(1u16 << 3);
pub const ONE_Q8_8: UFixed!(8, 8, u16, Warm) = Fx::new(1u16 << 8);

// The scale is NOT a function of the type, so a decode has to be told it.
pub const fn decode_q13_3(x: Fx<16, u16, Warm>) -> u32 {
    x.raw as u32 * 1000 / 8
}
pub const fn decode_q8_8(x: Fx<16, u16, Warm>) -> u32 {
    x.raw as u32 * 1000 / 256
}

// Both constants have ONE type, so every decode accepts every value.
// Each of the four calls below type-checks. Two of them are wrong.
pub const A: u32 = decode_q13_3(ONE_Q13_3); // 1000, right
pub const B: u32 = decode_q8_8(ONE_Q8_8); // 1000, right
pub const C: u32 = decode_q13_3(ONE_Q8_8); // 32000, wrong, and accepted
pub const D: u32 = decode_q8_8(ONE_Q13_3); //    31, wrong, and accepted

const _: () = assert!(A == 1000);
const _: () = assert!(B == 1000);
const _: () = assert!(C == 32000); // one, decoded as thirty-two
const _: () = assert!(D == 31); // one, decoded as three hundredths

// And the assignment 129's own capstone writes, line 60 of q13_capstone.rs:
pub const REINTERPRETED: UFixed!(8, 8, u16, Warm) = ONE_Q13_3;
const _: () = assert!(REINTERPRETED.raw == 8); // a Q8.8 datum holding 8, which is 0.03125
