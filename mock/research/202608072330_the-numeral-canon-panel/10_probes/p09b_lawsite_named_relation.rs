//! P09b. TREATMENT for the law-relation diagnostic, the one site class `137`
//! measured as degraded (`137:560-571`). The associated-type equality is moved
//! behind a named relation trait carrying `#[diagnostic::on_unimplemented]`,
//! with `#[diagnostic::do_not_recommend]` on the single blanket impl so rustc
//! reports the relation rather than drilling into its where-clause.
//! Same ladder, same bridge, same surface. No feature, no flag.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;
include!("ladder.rs");

pub struct Hot;
pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    type N;
}
pub struct Arvo;

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

// --- the named relation, which is what a consumer sees when a law fails -------
#[diagnostic::on_unimplemented(
    message = "width {C} is not the sum of widths {A} and {B}",
    label = "this output width does not follow from the input widths",
    note = "the result of this operation is {A} + {B} bits wide; write that width, or let it be inferred"
)]
pub trait WidthSum<M, const A: u32, const B: u32, const C: u32> {}

#[diagnostic::do_not_recommend]
impl<M, const A: u32, const B: u32, const C: u32> WidthSum<M, A, B, C> for Arvo
where
    Idx<A>: ToNat<M>,
    Idx<B>: ToNat<M>,
    Idx<C>: ToNat<M>,
    <Idx<A> as ToNat<M>>::N: Add<<Idx<B> as ToNat<M>>::N, O = <Idx<C> as ToNat<M>>::N>,
{
}

pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const OI: u32,
    const OF: u32,
    S,
    M,
>(
    _a: Fixed<I, F, S, M>,
    _b: Fixed<J, K, S, M>,
) -> Fixed<OI, OF, S, M>
where
    Arvo: WidthSum<M, I, J, OI> + WidthSum<M, F, K, OF>,
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
    Idx<J>: ToNat<M>,
    Idx<K>: ToNat<M>,
    Idx<OI>: ToNat<M>,
    Idx<OF>: ToNat<M>,
    <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
    <Idx<J> as ToNat<M>>::N: Add<<Idx<K> as ToNat<M>>::N>,
    <Idx<OI> as ToNat<M>>::N: Add<<Idx<OF> as ToNat<M>>::N>,
    Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    Sum<<Idx<J> as ToNat<M>>::N, <Idx<K> as ToNat<M>>::N>: Container,
    Sum<<Idx<OI> as ToNat<M>>::N, <Idx<OF> as ToNat<M>>::N>: Container,
{
    todo!()
}

// correct site, still infers nothing wrong
pub fn law_site(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<26, 6, Hot> {
    mul(a, b)
}
// deliberate mismatch, same as the control: 26 is right, 30 is asked for
pub fn law_site_wrong(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<30, 6, Hot> {
    mul(a, b)
}
