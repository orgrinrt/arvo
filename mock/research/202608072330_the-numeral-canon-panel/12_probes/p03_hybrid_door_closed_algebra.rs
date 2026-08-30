//! p03. The hybrid: const at the door, nat underneath. Does the CEILING lift?
//!
//! `11`'s b01 and b02 show the bridge table caps `mul`: two widths both in the
//! table produce an output width that is not, and rustc refuses. The cause was
//! localised to the const surface by b03, which dropped the const surface
//! entirely.
//!
//! This file keeps the const surface AND drops the ceiling, by making the
//! const->nat map a DOOR rather than a KEYING. The numeral is keyed on nats.
//! The consts appear only in a type alias a human writes. A derived width never
//! passes through the door, because by the time it is derived it is already a
//! nat.
//!
//! The door table below deliberately contains ONLY the widths a human writes:
//!   0, 3, 8, 13, 16, 24
//! It does NOT contain 26, 32, 48, 96, 192 or anything else the multiplies
//! produce. b01 could not get past the first of those. If this file compiles,
//! the ceiling was the keying, not the table.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata,asm -o out/p03.meta p03_hybrid_door_closed_algebra.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;
pub struct Warm;
pub struct Arvo;

pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

#[repr(transparent)]
pub struct Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

impl<WI, WF, S> Clone for Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Wrapping,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<WI, WF, S> Copy for Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Wrapping,
{
}

// --- the door. SIX rows, the widths a human types in this program ------------
pub struct Idx<const N: u32>;
#[diagnostic::on_unimplemented(
    message = "no width literal `{Self}` is declared in this program",
    label = "this literal width is not declared",
    note = "literal widths are declared per program, one line each. Declare it, or name a width you already have"
)]
pub trait ToNat<M> {
    type N;
}
macro_rules! door { ($($n:literal => $t:ty),* $(,)?) => { $(
    #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
door! { 0 => T0, 3 => T3, 8 => T8, 13 => T13, 16 => T16, 24 => T24 }

pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;
pub type UInt<const N: u32> = Fixed<NatOf<N>, T0, Warm>;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<NatOf<I>, NatOf<F>, S>;

// --- the law, on nats. Same as b03's mul_t. No Idx, no ToNat, no table. -----
pub fn mul<WI, WF, WJ, WK, S>(
    _a: Fixed<WI, WF, S>,
    _b: Fixed<WJ, WK, S>,
) -> Fixed<Sum<WI, WJ>, Sum<WF, WK>, S>
where
    WI: Add<WF> + Add<WJ>,
    WJ: Add<WK>,
    WF: Add<WK>,
    Sum<WI, WF>: Container,
    Sum<WJ, WK>: Container,
    Sum<WI, WJ>: Add<Sum<WF, WK>>,
    Sum<Sum<WI, WJ>, Sum<WF, WK>>: Container,
{
    todo!()
}

// --- the exact case b01 could not write --------------------------------------
// 24.8 times 24.8. Output is 48.16. There is NO Idx<48> row and NO Idx<16>
// row is needed for the output, because the output never touches the door.
pub type Coord = UFixed<24, 8, Hot>;
pub fn octave_1(a: Coord, b: Coord) -> Fixed<Sum<T24, T24>, Sum<T8, T8>, Hot> {
    mul(a, b)
}

// --- the octave b02 could not reach, and two past it -------------------------
type O1I = Sum<T24, T24>;
type O1F = Sum<T8, T8>;
pub fn octave_2(
    a: Fixed<O1I, O1F, Hot>,
    b: Fixed<O1I, O1F, Hot>,
) -> Fixed<Sum<O1I, O1I>, Sum<O1F, O1F>, Hot> {
    mul(a, b)
}
type O2I = Sum<O1I, O1I>;
type O2F = Sum<O1F, O1F>;
pub fn octave_3(
    a: Fixed<O2I, O2F, Hot>,
    b: Fixed<O2I, O2F, Hot>,
) -> Fixed<Sum<O2I, O2I>, Sum<O2F, O2F>, Hot> {
    mul(a, b)
}

// --- the container stays exact all the way up --------------------------------
const _: () = {
    assert!(core::mem::size_of::<Coord>() == 4); // 32 bits -> u32
    assert!(core::mem::size_of::<Fixed<O1I, O1F, Hot>>() == 8); // 64 bits -> u64
    assert!(core::mem::size_of::<Fixed<O2I, O2F, Hot>>() == 16); // 128 bits -> u128
                                                                 // 256 bits -> 4 words
    assert!(core::mem::size_of::<Fixed<Sum<O2I, O2I>, Sum<O2F, O2F>, Hot>>() == 32);
};

// --- and the door still does the one job it exists for -----------------------
pub type StrHandle = UInt<13>;
const _: () = assert!(core::mem::size_of::<StrHandle>() == 2);

// --- erasure and codegen through the door ------------------------------------
impl<WI, WF, S> Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Wrapping,
{
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Fixed {
            raw: Wrapping::wadd(self.raw, o.raw),
            _m: PhantomData,
        }
    }
}

#[unsafe(no_mangle)]
pub fn p03_arvo16(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn p03_native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn p03_arvo_vec(x: &mut [UFixed<13, 3, Hot>; 1024], y: &[UFixed<13, 3, Hot>; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub fn p03_native_vec(x: &mut [u16; 1024], y: &[u16; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
