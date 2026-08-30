//! p06. Attacking p05's finding, route 1: does rustc ELIDE a defaulted type
//! parameter when it prints a type in a diagnostic?
//!
//! p05 shows the nat-keyed candidates print digit towers on the most ordinary
//! consumer error. If rustc hides a type parameter that equals its default, a
//! numeral could carry the const coordinates in printable position and the nat
//! in a defaulted one, and the diagnostic would read like C0's while the
//! keying stayed C4's.
//!
//! Two questions, both compiled:
//!   (a) may a type parameter default be a projection off a const parameter?
//!   (b) does the diagnostic print the defaulted parameter?
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE, at the deliberate mismatch. What
//! is being read is WHAT IT PRINTS, not whether it errors.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p06.meta p06_default_param_elision.rs 2> out/p06.log
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;
pub struct Arvo;
pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;
pub type T5 = D1<D0<D1<Term>>>;

pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    type N;
}
macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
d! { 0 => T0, 3 => T3, 5 => T5, 6 => T6, 13 => T13, 26 => T26 }
pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;

// (a) the construction: consts in printable position, nat in a defaulted one.
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, WI = NatOf<I>, WF = NatOf<F>>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(S, WI, WF)>,
}

pub type Coord = Fixed<13, 3, Hot>;
pub type Product = Fixed<26, 6, Hot>;

const _: () = assert!(core::mem::size_of::<Coord>() == 2);

// (b) the deliberate mismatch. Read the printed type.
pub fn mistake(x: Product) -> Coord {
    x
}
