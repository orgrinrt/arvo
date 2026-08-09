#![no_std]
#![allow(dead_code)]
//! The identity contract as a signature, and four families interpreting it.
//! No family is an alias over another, so no change of basis sits in type
//! position, so nothing computes a const argument.
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
impl Container for u64 {
    const BITS: u32 = 64;
    const ZERO: u64 = 0;
}
pub trait Policy {}
pub trait Lowering {}
pub struct Warm;
impl Policy for Warm {}
impl Lowering for Warm {}

// ---- the exponent form, a kind rather than a number ------------------------
pub trait ExponentForm {
    const IS_RANGED: bool;
}
// The exponent form's TYPE carries the kind. The exponent's VALUE is a read,
// on EMIN and EMAX, because a value derived from a parameter has no type position.
pub struct Constant;
pub struct Ranged;
impl ExponentForm for Constant {
    const IS_RANGED: bool = false;
}
impl ExponentForm for Ranged {
    const IS_RANGED: bool = true;
}

// ---- the identity contract. Every mathematical coordinate is a READ. --------
pub trait Numeral {
    const RADIX: u32;
    const PRECISION: u32;
    const EMIN: i32;
    const EMAX: i32;
    const SIGNED: bool;
    type Exponent: ExponentForm;
    type Store: Container;
}

// ---- family one: unsigned binary fixed point, written in digit counts -------
pub struct UFixed<const I: u32, const F: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Numeral
    for UFixed<I, F, C, S>
{
    const RADIX: u32 = 2;
    const PRECISION: u32 = I + F;
    const EMIN: i32 = -(F as i32);
    const EMAX: i32 = -(F as i32);
    const SIGNED: bool = false;
    type Exponent = Constant;
    type Store = C;
}

// ---- family two: signed binary fixed point ---------------------------------
pub struct IFixed<const I: u32, const F: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Numeral
    for IFixed<I, F, C, S>
{
    const RADIX: u32 = 2;
    const PRECISION: u32 = 1 + I + F;
    const EMIN: i32 = -(F as i32);
    const EMAX: i32 = -(F as i32);
    const SIGNED: bool = true;
    type Exponent = Constant;
    type Store = C;
}

// ---- family three: an IEEE-shaped float, written in ITS natural basis -------
pub struct FastFloat<
    const P: u32,
    const EMIN: i32,
    const EMAX: i32,
    C: Container,
    S: Policy + Lowering,
> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, const EMIN: i32, const EMAX: i32, C: Container, S: Policy + Lowering> Numeral
    for FastFloat<P, EMIN, EMAX, C, S>
{
    const RADIX: u32 = 2;
    const PRECISION: u32 = P;
    const EMIN: i32 = EMIN;
    const EMAX: i32 = EMAX;
    const SIGNED: bool = true;
    type Exponent = Ranged;
    type Store = C;
}

// ---- family four: decimal, radix ten ---------------------------------------
pub struct Decimal<const P: u32, const E: i32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const P: u32, const E: i32, C: Container, S: Policy + Lowering> Numeral
    for Decimal<P, E, C, S>
{
    const RADIX: u32 = 10;
    const PRECISION: u32 = P;
    const EMIN: i32 = E;
    const EMAX: i32 = E;
    const SIGNED: bool = true;
    type Exponent = Constant;
    type Store = C;
}

// ---- one generic algorithm over the contract, never over a family ----------
pub const fn quantum_digits<N: Numeral>() -> i32 {
    N::EMIN
}
pub fn fits<N: Numeral>() -> bool {
    N::PRECISION <= <N::Store as Container>::BITS
}

const _: () = assert!(<UFixed<13, 3, u16, Warm> as Numeral>::PRECISION == 16);
const _: () = assert!(<UFixed<8, 8, u16, Warm> as Numeral>::PRECISION == 16);
const _: () = assert!(<UFixed<13, 3, u16, Warm> as Numeral>::EMIN == -3);
const _: () = assert!(<UFixed<8, 8, u16, Warm> as Numeral>::EMIN == -8);
const _: () = assert!(<IFixed<12, 3, u16, Warm> as Numeral>::PRECISION == 16);
const _: () = assert!(<FastFloat<24, -126, 127, u32, Warm> as Numeral>::PRECISION == 24);
const _: () = assert!(<Decimal<16, -398, u64, Warm> as Numeral>::RADIX == 10);
const _: () = assert!(quantum_digits::<UFixed<13, 3, u16, Warm>>() == -3);

// the four families agree on precision where they should, as VALUES
const _: () = assert!(
    <UFixed<13, 3, u16, Warm> as Numeral>::PRECISION
        == <IFixed<12, 3, u16, Warm> as Numeral>::PRECISION
);
