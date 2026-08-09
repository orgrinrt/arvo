//! Does anything foreclose a type-level Nat for Precision later?
//! The mechanism already paid for by the container projection produces one.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

pub trait Sign: Copy {
    const EXTRA: u32;
}
#[derive(Clone, Copy)]
pub struct Unsigned;
#[derive(Clone, Copy)]
pub struct Signed;
impl Sign for Unsigned {
    const EXTRA: u32 = 0;
}
impl Sign for Signed {
    const EXTRA: u32 = 1;
}
pub struct Warm;
pub struct Fixed<const I: u32, const F: u32, G: Sign, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

/// The type-level Nat. One constructor, standalone argument.
pub struct Nat<const P: u32>;

/// Precision as a `type const`, so it reaches BOTH value position and type position.
pub trait Numeral {
    type const PRECISION: u32;
}
impl<const I: u32, const F: u32, G: Sign, S> Numeral for Fixed<I, F, G, S> {
    type const PRECISION: u32 = const { I + F };
}
/// The Nat is derived from the separated coordinates with no computed const argument
/// at any use site: the addition lives in the `type const` body, which is the one
/// place GCA admits it.
pub type PrecisionOf<X> = Nat<{ <X as Numeral>::PRECISION }>;

// 1. canonical at concrete sites: 13+3 and 8+8 are ONE type
pub fn wants16(_: Nat<16>) {}
pub fn canonical(
    a: PrecisionOf<UFixed<13, 3, Warm>>,
    b: PrecisionOf<UFixed<8, 8, Warm>>,
    c: PrecisionOf<IFixed<12, 3, Warm>>,
) {
    wants16(a); // 13 + 3
    wants16(b); // 8 + 8, same type
    let _: Nat<15> = c; // signed: precision is sign-free, 12 + 3
}
// 2. the Nat is usable as a BOUND: "these two numerals have the same precision"
pub fn same_precision<A, B, const P: u32>(_: A, _: B)
where
    A: Numeral<PRECISION = { P }>,
    B: Numeral<PRECISION = { P }>,
{
}
pub fn agree(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    same_precision(a, b);
}
// 3. and it refuses a genuine mismatch
// 4. and it refuses a genuine mismatch (this line is the negative control)
// pub fn disagree(a: UFixed<13, 3, Warm>, b: UFixed<9, 8, Warm>) { same_precision(a, b); }
// 5. a Nat reaches type position in a nested projection too
pub struct Column<N, const LEN: usize>(PhantomData<N>);
pub type ColumnOf<X, const LEN: usize> = Column<PrecisionOf<X>, LEN>;
pub fn column(_: ColumnOf<UFixed<13, 3, Warm>, 4096>) {}
pub fn column_agrees(x: ColumnOf<UFixed<8, 8, Warm>, 4096>) {
    column(x);
}
