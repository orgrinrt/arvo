//! p02. Can a TYPE ALIAS WITH A CONST PARAMETER be the door?
//!
//! The bar governs what the alias writer TYPES. The ladder governs what the
//! compiler RECEIVES. Rust has exactly one construction that decouples those:
//! `pub type UInt<const N: u32> = <something built from N>;`
//!
//! If the alias body may carry a projection off a const parameter, then the
//! consumer writes `UInt<5>` (literally the bar) while the type underneath is
//! keyed on a nat. This file asks only whether the construction compiles and
//! resolves. Whether the ALGEBRA is then closed is p03.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p02.meta p02_const_door_alias.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;
pub struct Warm;
pub struct Arvo;

pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

// the numeral, keyed on NAT TYPES. This is b03's shape verbatim.
#[repr(transparent)]
pub struct Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

// --- the door: a const -> nat map, used ONLY where a human writes a number ---
pub struct Idx<const N: u32>;
#[diagnostic::on_unimplemented(
    message = "arvo has no width literal for {Self}",
    label = "this width is not spelled as a literal in this program",
    note = "widths written as literals are opt-in per program. Add the one line `arvo::width!(N);` \
            for the width you want, or spell the numeral against a width you already declared"
)]
pub trait ToNat<M> {
    type N;
}
macro_rules! door { ($($n:literal => $t:ty),* $(,)?) => { $(
    #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
door! { 0 => T0, 3 => T3, 5 => T5, 8 => T8, 13 => T13, 16 => T16, 24 => T24, 26 => T26, 48 => T48 }
pub type T5 = D1<D0<D1<Term>>>;
pub type T48 = D0<D0<D0<D0<D1<D1<Term>>>>>>;

pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;

// --- THE QUESTION: does this alias declaration compile? ----------------------
pub type UInt<const N: u32> = Fixed<NatOf<N>, T0, Warm>;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<NatOf<I>, NatOf<F>, S>;

// --- and does it RESOLVE at a use site? --------------------------------------
pub type StrHandle = UInt<5>;
pub type Coord = UFixed<13, 3, Hot>;

// erasure, asserted through the door
const _: () = {
    assert!(core::mem::size_of::<StrHandle>() == 1);
    assert!(core::mem::size_of::<Coord>() == 2);
    assert!(core::mem::align_of::<Coord>() == core::mem::align_of::<u16>());
};

// and does a value flow between the two spellings of the same type?
pub fn door_and_tower_agree(x: Fixed<T5, T0, Warm>) -> StrHandle {
    x
}
