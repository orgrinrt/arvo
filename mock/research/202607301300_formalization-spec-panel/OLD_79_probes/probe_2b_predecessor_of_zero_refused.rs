//! Probe 2b. NEGATIVE CONTROL for probe 2. An empty capacity has no valid
//! index and so no far point; the type system states this by refusing to
//! compile `last_index::<Z-rooted-capacity>()`, not by returning a sentinel.
//!
//! This is the same shape as the far-point rule's own NaN exclusion (`68:281-
//! 282`): the supremum is taken over the ordered representable values only,
//! and where that set is empty (a zero-capacity collection's index set), the
//! operation is not merely undefined at one input, it has no type through
//! which to be called at all. `last_index` is generic over `C: Pos + Dec`;
//! `Z` does not implement `Pos`, so this fails at the bound, before monomorphisation,
//! with the compiler naming the missing bound directly.
#![no_std]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}

pub trait Pos: seal::Sealed {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

pub trait PosPred: Pos {
    type Out: Pos;
}
impl<Q: Pos> PosPred for I<Q> {
    type Out = O<Q>;
}
impl PosPred for O<H> {
    type Out = H;
}
impl<Q: Pos> PosPred for O<O<Q>>
where
    O<Q>: PosPred,
{
    type Out = I<<O<Q> as PosPred>::Out>;
}
impl<Q: Pos> PosPred for O<I<Q>>
where
    I<Q>: PosPred,
{
    type Out = I<<I<Q> as PosPred>::Out>;
}

pub trait Dec: Pos {
    type Out: Nat;
}
impl Dec for H {
    type Out = Z;
}
impl<Q: Pos> Dec for I<Q> {
    type Out = Pz<O<Q>>;
}
impl Dec for O<H> {
    type Out = Pz<H>;
}
impl<Q: Pos> Dec for O<O<Q>>
where
    O<Q>: PosPred,
{
    type Out = Pz<<O<O<Q>> as PosPred>::Out>;
}
impl<Q: Pos> Dec for O<I<Q>>
where
    I<Q>: PosPred,
{
    type Out = Pz<<O<I<Q>> as PosPred>::Out>;
}

pub const fn last_index<C: Pos + Dec>() -> usize
where
    <C as Dec>::Out: Nat,
{
    <C as Dec>::Out::VAL
}

// The refusal under test: Z is a Nat but not a Pos, so it cannot name
// `last_index`'s bound at all.
const _: usize = last_index::<Z>();

fn main() {}
