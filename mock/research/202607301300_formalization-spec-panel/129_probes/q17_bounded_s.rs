#![no_std]
#![allow(dead_code)]
//! The whole mechanism, no feature gates, no -Z flag.
use core::marker::PhantomData;

// ---- strategy axis, untouched ----------------------------------------------
pub trait Policy {
    const SATURATES: bool;
}
pub trait Lowering {
    const LANES: u32;
}
pub struct Warm;
impl Policy for Warm {
    const SATURATES: bool = false;
}
impl Lowering for Warm {
    const LANES: u32 = 1;
}

// ---- the container ladder: carry and read, never derive ---------------------
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u8 {
    const BITS: u32 = 8;
}
impl Container for u16 {
    const BITS: u32 = 16;
}
impl Container for u32 {
    const BITS: u32 = 32;
}
impl Container for u64 {
    const BITS: u32 = 64;
}
impl Container for u128 {
    const BITS: u32 = 128;
}

// ---- the numeral: precision is THE parameter, so canonicity is structural ---
pub struct Fx<const P: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, C: Container, S: Policy + Lowering> Clone for Fx<P, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, C: Container, S: Policy + Lowering> Copy for Fx<P, C, S> {}

impl<const P: u32, C: Container, S: Policy + Lowering> Fx<P, C, S> {
    const FITS: () = assert!(P <= C::BITS, "precision does not fit its container");
    pub fn new(raw: C) -> Self {
        let () = Self::FITS;
        Fx {
            raw,
            _s: PhantomData,
        }
    }
}

// ---- the two surface spellings, both concrete, both canonical ---------------
#[macro_export]
macro_rules! UFixed {
    ($i:literal, $f:literal, $c:ty, $s:ty) => { $crate::Fx<{ $i + $f }, $c, $s> };
}

// ---- the laws: the output precision is a parameter, the relation is a check -
pub fn mul<const P: u32, const Q: u32, const R: u32, C: Container, S: Policy + Lowering>(
    a: Fx<P, C, S>,
    b: Fx<Q, C, S>,
) -> Fx<R, C, S> {
    const {
        assert!(
            R == P + Q,
            "mul: output precision must equal the sum of the input precisions"
        )
    }
    let _ = b;
    Fx {
        raw: a.raw,
        _s: PhantomData,
    }
}

pub fn add<const P: u32, const Q: u32, const R: u32, C: Container, S: Policy + Lowering>(
    a: Fx<P, C, S>,
    b: Fx<Q, C, S>,
) -> Fx<R, C, S> {
    const {
        assert!(
            R == (if P > Q { P } else { Q }) + 1,
            "add: output precision must be one above the wider input"
        )
    }
    let _ = b;
    Fx {
        raw: a.raw,
        _s: PhantomData,
    }
}

// ---- what a consumer writes -------------------------------------------------
pub fn consumer() {
    // 13 integer bits, 3 fraction bits, in a u16.
    let a: UFixed!(13, 3, u16, Warm) = Fx::new(0);
    // 8 and 8: DIFFERENT scaling, SAME precision, SAME type as the above.
    let b: UFixed!(8, 8, u16, Warm) = a;
    // arbitrary widths, no table, no cap
    let _w: UFixed!(40, 30, u128, Warm) = Fx::new(0);
    let _x: UFixed!(3, 0, u8, Warm) = Fx::new(0);
    let _y: UFixed!(47, 0, u64, Warm) = Fx::new(0);

    // the product's precision is spellable and inferred from the annotation
    let c: Fx<32, u16, Warm> = mul(a, b);
    let d: Fx<17, u16, Warm> = add(a, b);
    let _ = (c, d);
}

// canonicity, asserted three ways at concrete instantiation
pub fn wants16(_: Fx<16, u16, Warm>) {}
pub fn canonical(x: UFixed!(13, 3, u16, Warm), y: UFixed!(8, 8, u16, Warm), z: Fx<16, u16, Warm>) {
    wants16(x);
    wants16(y);
    wants16(z);
}
