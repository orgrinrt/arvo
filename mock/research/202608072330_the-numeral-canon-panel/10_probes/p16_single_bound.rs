//! P16. `137`'s construction makes every generic site repeat four bounds
//! (`Idx<I>: ToNat<M>`, `Idx<F>: ToNat<M>`, `..: Add<..>`, `Sum<..>: Container`),
//! because a struct's where-clause is not implied for its users. `142c:329-333`
//! puts ergonomics at the alias tier above the plumbing, so this is a real cost
//! and it is not named anywhere in the record.
//! Collapse: one named trait `Rep<I, F>` on the marker, blanket-implemented
//! where the four hold, with the container as its associated type. The struct
//! then carries ONE bound and so does every generic site.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;
include!("ladder.rs");

pub struct Hot;
pub struct Idx<const N: u32>;
pub struct Arvo;

#[diagnostic::on_unimplemented(
    message = "arvo does not ship this width: {Self}",
    label = "this numeral names a width arvo does not ship",
    note = "widths are opt-in per program. Add `impl ToNat<MyWidths> for {Self}` and spell the numeral against `MyWidths`"
)]
pub trait ToNat<M> {
    type N;
}

macro_rules! bridge { ($($n:literal => $t:ty),* $(,)?) => { $( #[diagnostic::do_not_recommend] impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
bridge! { 0 => T0, 3 => T3, 6 => T6, 8 => T8, 13 => T13, 16 => T16, 24 => T24,
26 => T26, 30 => T30, 40 => T40, 64 => T64, 100 => T100, 200 => T200, }

pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

// --- the single named bound --------------------------------------------------
#[diagnostic::on_unimplemented(
    message = "there is no arvo numeral with {A} integer bits and {B} fraction bits",
    label = "no numeral of this shape"
)]
pub trait Rep<const A: u32, const B: u32> {
    type C;
    type W;
}

#[diagnostic::do_not_recommend]
impl<M, const A: u32, const B: u32> Rep<A, B> for M
where
    Idx<A>: ToNat<M>,
    Idx<B>: ToNat<M>,
    <Idx<A> as ToNat<M>>::N: Add<<Idx<B> as ToNat<M>>::N>,
    Sum<<Idx<A> as ToNat<M>>::N, <Idx<B> as ToNat<M>>::N>: Container,
{
    type C = Cont<Sum<<Idx<A> as ToNat<M>>::N, <Idx<B> as ToNat<M>>::N>>;
    type W = Sum<<Idx<A> as ToNat<M>>::N, <Idx<B> as ToNat<M>>::N>;
}

#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>
where
    M: Rep<I, F>,
{
    raw: <M as Rep<I, F>>::C,
    _m: PhantomData<(S, M)>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;

impl<const I: u32, const F: u32, S, M: Rep<I, F>> Clone for Fixed<I, F, S, M>
where
    <M as Rep<I, F>>::C: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, S, M: Rep<I, F>> Copy for Fixed<I, F, S, M> where
    <M as Rep<I, F>>::C: Copy
{
}

impl<const I: u32, const F: u32, S, M: Rep<I, F>> Fixed<I, F, S, M>
where
    <M as Rep<I, F>>::C: Wrapping + Copy,
{
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Fixed {
            raw: self.raw.wadd(o.raw),
            _m: PhantomData,
        }
    }
}

const _: () = assert!(core::mem::size_of::<UFixed<13, 3, Hot>>() == 2);
const _: () = assert!(core::mem::size_of::<UFixed<40, 24, Hot>>() == 8);
const _: () = assert!(core::mem::size_of::<UFixed<100, 100, Hot>>() == 32);

#[unsafe(no_mangle)]
pub fn arvo16(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn arvo64(a: UFixed<40, 24, Hot>, b: UFixed<40, 24, Hot>) -> UFixed<40, 24, Hot> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

// a generic site: ONE bound, against four in `137`
pub fn twice<const I: u32, const F: u32, S, M: Rep<I, F>>(a: Fixed<I, F, S, M>) -> Fixed<I, F, S, M>
where
    <M as Rep<I, F>>::C: Wrapping + Copy,
{
    a.add(a)
}
