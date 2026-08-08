//! p08. Does p06's shape keep the ceiling?
//!
//! p06 recovers the headline diagnostic by putting the consts in printable
//! position and defaulting the nats off them. p07a..p07d show the two cannot be
//! tied by a bound. So the tie is the DEFAULT, which fires only where a site
//! omits the nat parameters, which is every ordinary site.
//!
//! The question this file answers: when `mul` produces a width, does that width
//! have to pass through the door?
//!
//! Two arms.
//!   A. one head constructor, output consts free.   Expected: no inference.
//!   B. two head constructors, derived values nat-keyed. Expected: closed.
//!
//! Arm A is commented out and its diagnostic captured in p08a; this file is
//! arm B and is expected to COMPILE. The door below has SIX rows and contains
//! neither 48 nor 96 nor 192.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p08.meta p08_does_p06_shape_keep_the_ceiling.rs
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
#[diagnostic::on_unimplemented(
    message = "no width literal `{Self}` is declared in this program",
    label = "this literal width is not declared",
    note = "a literal width is declared once, in one line. Declare it, or name a width you already have"
)]
pub trait ToNat<M> {
    type N;
}
macro_rules! door { ($($n:literal => $t:ty),* $(,)?) => { $(
    #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
door! { 0 => T0, 3 => T3, 5 => T5, 8 => T8, 13 => T13, 24 => T24 }
pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;

// --- the NAMED numeral: consts printable, nats defaulted off them ------------
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, WI = NatOf<I>, WF = NatOf<F>>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(S, WI, WF)>,
}
impl<const I: u32, const F: u32, S, WI, WF> Clone for Fixed<I, F, S, WI, WF>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, S, WI, WF> Copy for Fixed<I, F, S, WI, WF>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Copy,
{
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;
pub type UInt<const N: u32> = Fixed<N, 0, S0>;
pub struct S0;

// --- the DERIVED numeral: nat only. Nothing names it, so nothing prints it. ---
#[repr(transparent)]
pub struct Derived<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

// entering the derived world from a named numeral is free and total.
impl<const I: u32, const F: u32, S, WI, WF> Fixed<I, F, S, WI, WF>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    #[inline]
    pub fn derived(self) -> Derived<WI, WF, S> {
        Derived {
            raw: self.raw,
            _m: PhantomData,
        }
    }
}

pub fn mul<WI, WF, WJ, WK, S>(
    _a: Derived<WI, WF, S>,
    _b: Derived<WJ, WK, S>,
) -> Derived<Sum<WI, WJ>, Sum<WF, WK>, S>
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

// --- three octaves, and the door has no row for any of their widths ----------
pub type Coord = UFixed<24, 8, Hot>;
type O1I = Sum<T24, T24>;
type O1F = Sum<T8, T8>;
type O2I = Sum<O1I, O1I>;
type O2F = Sum<O1F, O1F>;

pub fn chain(a: Coord, b: Coord) -> Derived<Sum<O2I, O2I>, Sum<O2F, O2F>, Hot> {
    let o1 = mul(a.derived(), b.derived());
    let o2 = mul(o1, o1_again(a, b));
    mul(o2, o2_again(a, b))
}
fn o1_again(a: Coord, b: Coord) -> Derived<O1I, O1F, Hot> {
    mul(a.derived(), b.derived())
}
fn o2_again(a: Coord, b: Coord) -> Derived<O2I, O2F, Hot> {
    mul(o1_again(a, b), o1_again(a, b))
}

impl<WI, WF, S> Clone for Derived<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<WI, WF, S> Copy for Derived<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
    Cont<Sum<WI, WF>>: Copy,
{
}

const _: () = {
    assert!(core::mem::size_of::<Coord>() == 4);
    assert!(core::mem::size_of::<Derived<O1I, O1F, Hot>>() == 8);
    assert!(core::mem::size_of::<Derived<O2I, O2F, Hot>>() == 16);
    assert!(core::mem::size_of::<Derived<Sum<O2I, O2I>, Sum<O2F, O2F>, Hot>>() == 32);
};

// --- and coming BACK to a named numeral needs the door, which is the whole ---
// --- residue: a width a consumer NAMES must be declared; one it merely
// --- COMPUTES need not be.
impl<WI, WF, S> Derived<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    #[inline]
    pub fn named<const I: u32, const F: u32>(self) -> Fixed<I, F, S, WI, WF> {
        Fixed {
            raw: self.raw,
            _m: PhantomData,
        }
    }
}
