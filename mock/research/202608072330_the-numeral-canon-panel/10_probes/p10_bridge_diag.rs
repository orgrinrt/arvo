//! P10. The other degraded site: a width outside the shipped bridge. The
//! control (P09a with output width 27) reports
//!   the trait bound `Idx<27>: ToNat<Arvo>` is not satisfied
//! and then LISTS the shipped table lexicographically (Idx<0>, Idx<100>,
//! Idx<13>, ... and 5 others), leaking the enumeration into consumer output.
//! This is the same construction with `#[diagnostic::on_unimplemented]` on the
//! bridge trait, which `137:608-610` handed to op as a choice rather than built.
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

// the width is a parameter of the trait that carries the message, so it prints
#[diagnostic::on_unimplemented(
    message = "arvo does not ship a {W}-bit width",
    label = "no {W}-bit numeral exists",
    note = "widths are opt-in per program: add `impl ToNat<MyWidths> for Idx<{W}>` and spell the numeral against `MyWidths`"
)]
pub trait Ships<const W: u32> {}
#[diagnostic::do_not_recommend]
impl<const W: u32> Ships<W> for Arvo where Idx<W>: ToNat<Arvo> {}

macro_rules! bridge { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
bridge! { 0 => T0, 3 => T3, 6 => T6, 8 => T8, 13 => T13, 16 => T16, 24 => T24,
26 => T26, 30 => T30, 40 => T40, 64 => T64, 100 => T100, 200 => T200, }

pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
{
    raw: Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>,
    _m: PhantomData<(S, M)>,
}

pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;

pub fn numeral<const I: u32, const F: u32, S>(a: UFixed<I, F, S>) -> UFixed<I, F, S>
where
    Arvo: Ships<I> + Ships<F>,
    Idx<I>: ToNat<Arvo>,
    Idx<F>: ToNat<Arvo>,
    <Idx<I> as ToNat<Arvo>>::N: Add<<Idx<F> as ToNat<Arvo>>::N>,
    Sum<<Idx<I> as ToNat<Arvo>>::N, <Idx<F> as ToNat<Arvo>>::N>: Container,
{
    a
}

pub fn ok(a: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
    numeral(a)
}

// the unbridged call, routed through the named relation
pub fn unbridged_direct(a: Fixed<7, 1, Hot>) -> Fixed<7, 1, Hot> {
    a
}
