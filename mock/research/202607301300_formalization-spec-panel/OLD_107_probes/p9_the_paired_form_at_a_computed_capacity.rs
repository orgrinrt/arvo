//! Probe 9. The ratified form, at the one input nobody has fed it.
//!
//! The ratified shape is `76_probes/b2`'s `Slot<N, const K: usize>`: the count
//! keys on the shared numeral, the array length is a companion literal, and
//! the two are checked to agree at a construction door. Every test of it in
//! this panel supplies a capacity a HUMAN DECLARED, where the literal is
//! something the author writes next to the numeral they also wrote.
//!
//! This probe supplies a capacity nobody declared: the sum of two others,
//! produced by the tower's own type-level arithmetic. That is not an exotic
//! input. It is the shape of shape composition (`102:799-805`), of the
//! bivector extent derived from a rank (`102:904-913`, where the derived
//! extent is required to "pair with its literal"), and of any concatenation,
//! split, or reshape.
//!
//! Claims:
//!   A. at a CONCRETE computed capacity the paired form still works, provided
//!      the author computes the number by hand and writes it. The agreement
//!      check then guards a number a human did arithmetic to produce, which is
//!      the worst input a check of that kind can be given.
//!   B. at a GENERIC computed capacity there is no literal to write, and the
//!      form cannot be spelled at all. REFUSED.
//!   C. the derived form (probe 4) expresses claim B's signature with no
//!      literal anywhere.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

pub trait Pos {
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

pub trait Inc {
    type Out: Pos;
}
impl Inc for H {
    type Out = O<H>;
}
impl<P: Pos> Inc for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Inc> Inc for I<P> {
    type Out = O<<P as Inc>::Out>;
}
pub type Suc<A> = <A as Inc>::Out;

pub trait Add<R> {
    type Out: Pos;
}
pub type Sum<A, B> = <A as Add<B>>::Out;
pub trait AddC<R> {
    type Out: Pos;
}
pub type SumC<A, B> = <A as AddC<B>>::Out;

impl Add<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> Add<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Pos + Inc> Add<I<Q>> for H {
    type Out = O<Suc<Q>>;
}
impl<P: Pos + Inc> Add<H> for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<O<Q>> for O<P> {
    type Out = O<Sum<P, Q>>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<I<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + Inc> Add<H> for I<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<O<Q>> for I<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> Add<I<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}
impl AddC<H> for H {
    type Out = I<H>;
}
impl<Q: Pos + Inc> AddC<O<Q>> for H {
    type Out = O<Suc<Q>>;
}
impl<Q: Pos + Inc> AddC<I<Q>> for H {
    type Out = I<Suc<Q>>;
}
impl<P: Pos + Inc> AddC<H> for O<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + Add<Q>, Q: Pos> AddC<O<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<I<Q>> for O<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + Inc> AddC<H> for I<P> {
    type Out = I<Suc<P>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<O<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<I<Q>> for I<P> {
    type Out = I<SumC<P, Q>>;
}

pub type N5 = I<O<H>>;
pub type N7 = I<I<H>>;

// The ratified paired form, verbatim in shape from `76_probes/b2`.
pub struct Slot<P, const K: usize>(PhantomData<P>);

pub trait Capacity {
    const VAL: usize;
    type Array<T: Copy>: Copy;
}

impl<P: Pos, const K: usize> Capacity for Slot<P, K> {
    const VAL: usize = {
        assert!(
            P::VAL == K,
            "capacity's declared length disagrees with its value"
        );
        K
    };
    type Array<T: Copy> = [T; K];
}

// CLAIM A. A concrete computed capacity. The `12` is a number a human did
// arithmetic to produce. Nothing in the type system produced it, and the
// agreement check is the only thing standing between this line and a wrong
// answer that compiles.
pub type Cat57 = Slot<Sum<N5, N7>, 12>;
const _: () = assert!(<Cat57 as Capacity>::VAL == 12);
const _: () = assert!(core::mem::size_of::<<Cat57 as Capacity>::Array<u32>>() == 48);

// CLAIM B. The same operation, generic. A function returning the storage for
// the concatenation of two capacity-bounded domains. There is no author here,
// so there is no literal, and the only honest thing to write in the literal's
// position is the value the numeral already knows. Compile with `--cfg refuse`.
#[cfg(refuse)]
pub fn concat_storage<A, B, T>(
) -> <Slot<Sum<A, B>, { <Sum<A, B> as Pos>::VAL }> as Capacity>::Array<T>
where
    A: Pos + Add<B>,
    B: Pos,
    T: Copy,
{
    unimplemented!()
}

// The nearest legal spelling: take the literal as a parameter and hope the
// caller passes the right one. Compiles, and is the whole finding.
pub fn concat_storage_with_a_literal<A, B, const K: usize, T>(
) -> <Slot<Sum<A, B>, K> as Capacity>::Array<T>
where
    A: Pos + Add<B>,
    B: Pos,
    T: Copy,
    Slot<Sum<A, B>, K>: Capacity,
{
    unimplemented!()
}
