#![no_std]
#![allow(dead_code)]
//! D48's literal arity: the container rides on the strategy, which is where the
//! standing design already puts the Lowering projection.
use core::marker::PhantomData;
pub trait Container: Copy {
    const BITS: u32;
    const ZERO: Self;
}
impl Container for u8 {
    const BITS: u32 = 8;
    const ZERO: u8 = 0;
}
impl Container for u16 {
    const BITS: u32 = 16;
    const ZERO: u16 = 0;
}
impl Container for u32 {
    const BITS: u32 = 32;
    const ZERO: u32 = 0;
}

pub trait Policy {}
pub trait Lowering {
    type Container: Container;
}
pub struct Warm<C: Container>(PhantomData<C>);
impl<C: Container> Policy for Warm<C> {}
impl<C: Container> Lowering for Warm<C> {
    type Container = C;
}

pub struct UFixed<const I: u32, const F: u32, S: Policy + Lowering> {
    raw: <S as Lowering>::Container,
    _s: PhantomData<S>,
}
impl<const I: u32, const F: u32, S: Policy + Lowering> Clone for UFixed<I, F, S> {
    fn clone(&self) -> Self {
        UFixed {
            raw: self.raw,
            _s: PhantomData,
        }
    }
}
impl<const I: u32, const F: u32, S: Policy + Lowering> Copy for UFixed<I, F, S> {}
impl<const I: u32, const F: u32, S: Policy + Lowering> UFixed<I, F, S> {
    pub const FITS: () = assert!(
        I + F <= <S as Lowering>::Container::BITS,
        "arvo: the format does not fit the container its strategy names."
    );
    pub const fn zero() -> Self {
        let () = Self::FITS;
        UFixed {
            raw: <<S as Lowering>::Container as Container>::ZERO,
            _s: PhantomData,
        }
    }
}
pub trait Format {
    const PRECISION: u32;
    const EXPONENT: i32;
}
impl<const I: u32, const F: u32, S: Policy + Lowering> Format for UFixed<I, F, S> {
    const PRECISION: u32 = I + F;
    const EXPONENT: i32 = -(F as i32);
}
// D48's spelling, three parameters, plain type syntax
pub fn consumer() {
    let _a: UFixed<13, 3, Warm<u16>> = UFixed::zero();
    let _b: UFixed<8, 8, Warm<u16>> = UFixed::zero();
    let _c: UFixed<20, 10, Warm<u32>> = UFixed::zero();
}
const _: () = assert!(<UFixed<13, 3, Warm<u16>> as Format>::PRECISION == 16);
const _: () = assert!(<UFixed<13, 3, Warm<u16>> as Format>::EXPONENT == -3);
