#![no_std]
#![allow(dead_code)]
//! The consumer writes the two numbers, in plain type syntax, no macro, no gate.
//! Nothing anywhere computes a const argument. Precision is a projection.
use core::marker::PhantomData;

// ---- strategy axis ---------------------------------------------------------
pub trait Policy {}
pub trait Lowering {}
pub struct Warm;
impl Policy for Warm {}
impl Lowering for Warm {}

// ---- the container ladder: carried and read, five rungs the hardware has ----
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

// ---- the numeral. The two numbers a consumer writes ARE the parameters. -----
pub struct UFixed<const I: u32, const F: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Clone for UFixed<I, F, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Copy for UFixed<I, F, C, S> {}

// ---- the mathematical coordinates, as projections in VALUE position ---------
pub trait Format {
    const PRECISION: u32;
    const EXPONENT: i32;
    const INTEGER_DIGITS: u32;
    type Store: Container;
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Format for UFixed<I, F, C, S> {
    const PRECISION: u32 = I + F; // legal: an associated const body is value position
    const EXPONENT: i32 = -(F as i32); // likewise
    const INTEGER_DIGITS: u32 = I;
    type Store = C;
}

impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> UFixed<I, F, C, S> {
    const FITS: () = assert!(
        I + F <= C::BITS,
        "arvo: the format's precision exceeds its container. \
         UFixed<I, F, C, S> stores I + F significant bits in C; \
         pick a wider C, or fewer bits."
    );
    pub const fn new(raw: C) -> Self {
        let () = Self::FITS;
        UFixed {
            raw,
            _s: PhantomData,
        }
    }
    pub const fn raw(self) -> C {
        self.raw
    }
}

// ---- what a consumer writes ------------------------------------------------
pub fn consumer() {
    let _a: UFixed<13, 3, u16, Warm> = UFixed::new(0);
    let _b: UFixed<8, 8, u16, Warm> = UFixed::new(0);
    let _w: UFixed<40, 30, u128, Warm> = UFixed::new(0);
    let _x: UFixed<3, 0, u8, Warm> = UFixed::new(0);
    let _y: UFixed<47, 0, u64, Warm> = UFixed::new(0);
    let _z: UFixed<0, 8, u8, Warm> = UFixed::new(0); // purely fractional
}

// ---- canonicity of the numeral: one type per format, reached however -------
pub fn wants_q13_3(_: UFixed<13, 3, u16, Warm>) {}
pub type Q13_3 = UFixed<13, 3, u16, Warm>;
pub type Sample = Q13_3;
pub fn canonical(x: UFixed<13, 3, u16, Warm>, y: Q13_3, z: Sample) {
    wants_q13_3(x);
    wants_q13_3(y);
    wants_q13_3(z);
}

// ---- canonicity of the precision: a const value, so 13+3 and 8+8 agree -----
const _: () = assert!(
    <UFixed<13, 3, u16, Warm> as Format>::PRECISION
        == <UFixed<8, 8, u16, Warm> as Format>::PRECISION
);
const _: () = assert!(<UFixed<13, 3, u16, Warm> as Format>::PRECISION == 16);
const _: () = assert!(<UFixed<40, 30, u128, Warm> as Format>::PRECISION == 70);
const _: () = assert!(<UFixed<13, 3, u16, Warm> as Format>::EXPONENT == -3);
const _: () = assert!(<UFixed<8, 8, u16, Warm> as Format>::EXPONENT == -8);

// ---- and their containers are the SAME TYPE, which is where op wanted the agreement
pub fn same_store<A: Format<Store = u16>, B: Format<Store = u16>>(_: A, _: B) {}
pub fn agree(a: UFixed<13, 3, u16, Warm>, b: UFixed<8, 8, u16, Warm>) {
    same_store(a, b);
}

// ---- the laws. Coordinate equalities are BOUNDS; sums are checks. ----------

/// Alignment is a parameter equality, so it is checked where the call is written.
pub fn add<
    const I: u32,
    const J: u32,
    const F: u32,
    const M: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    a: UFixed<I, F, C, S>,
    b: UFixed<J, F, C, S>,
) -> UFixed<M, F, D, S> {
    const {
        assert!(
            M == if I > J { I } else { J } + 1,
            "arvo: add widens the integer part by one above the wider input."
        )
    }
    let _ = (a, b);
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

/// The product's coordinates are sums, so the relation is a check.
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    a: UFixed<I, F, C, S>,
    b: UFixed<J, K, C, S>,
) -> UFixed<M, N, D, S> {
    const {
        assert!(
            M == I + J,
            "arvo: mul adds the integer digit counts of its inputs."
        )
    }
    const {
        assert!(
            N == F + K,
            "arvo: mul adds the fraction digit counts of its inputs."
        )
    }
    let _ = (a, b);
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

pub fn widen_int<
    const I: u32,
    const F: u32,
    const A: u32,
    const M: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    x: UFixed<I, F, C, S>,
) -> UFixed<M, F, D, S> {
    const { assert!(M == I + A, "arvo: widen_int adds A integer digits.") }
    let _ = x;
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

// ---- the laws used at a concrete site, output coordinates inferred ---------
pub fn laws() {
    let a: UFixed<13, 3, u16, Warm> = UFixed::new(0);
    let b: UFixed<13, 3, u16, Warm> = UFixed::new(0);
    let p: UFixed<26, 6, u32, Warm> = mul(a, b); // spellable, and inferred
    let s: UFixed<14, 3, u16, Warm> = add(a, b);
    let _ = (p, s);
}
pub fn wants_q26_6(_: UFixed<26, 6, u32, Warm>) {}
pub fn inferred_from_callee(a: UFixed<13, 3, u16, Warm>, b: UFixed<13, 3, u16, Warm>) {
    wants_q26_6(mul(a, b)); // no annotation at all
}

// ---- canonicity under a generic parameter, which is where GCA fails --------
pub fn widen_twice<
    const I: u32,
    const F: u32,
    const A: u32,
    const B: u32,
    const T: u32,
    const M: u32,
    C: Container,
    S: Policy + Lowering,
>(
    x: UFixed<I, F, C, S>,
) -> UFixed<M, F, C, S> {
    const { assert!(T == I + A && M == T + B, "arvo: two widenings compose.") }
    widen_int::<T, F, B, M, C, C, S>(widen_int::<I, F, A, T, C, C, S>(x))
}
pub fn widen_once<
    const I: u32,
    const F: u32,
    const A: u32,
    const B: u32,
    const AB: u32,
    const M: u32,
    C: Container,
    S: Policy + Lowering,
>(
    x: UFixed<I, F, C, S>,
) -> UFixed<M, F, C, S> {
    const { assert!(AB == A + B && M == I + AB, "arvo: one widening by the sum.") }
    widen_int::<I, F, AB, M, C, C, S>(x)
}
/// Both routes land in ONE type under a generic parameter. No definitional
/// equality is consulted, because M is a parameter rather than an expression.
pub fn interchange<
    const I: u32,
    const F: u32,
    const A: u32,
    const B: u32,
    const T: u32,
    const AB: u32,
    const M: u32,
    C: Container,
    S: Policy + Lowering,
>(
    x: UFixed<I, F, C, S>,
) -> UFixed<M, F, C, S> {
    let p: UFixed<M, F, C, S> = widen_twice::<I, F, A, B, T, M, C, S>(x);
    let q: UFixed<M, F, C, S> = widen_once::<I, F, A, B, AB, M, C, S>(x);
    let _ = q;
    p
}
