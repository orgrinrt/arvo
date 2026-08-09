//! Probe 3. The target shape, built far enough to price it.
//!
//! Width is a type. `I` and `F` are types (probe 2 established they must be).
//! The question this answers: what does the CONTAINER DISPATCH become? Today
//! it is one `const fn` returning a bucket index
//! (`arvo-strategy/src/container.rs:60-91`). With the width as a type there is
//! no const to switch on, so the bucket has to be derived structurally.
#![no_std]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

// ---- the tower's binary Pos, as the review specifies it -------------------
pub struct H; // 1
pub struct O<P>(PhantomData<P>); // 2P
pub struct I<P>(PhantomData<P>); // 2P+1
pub trait Pos {
    const VAL: u128;
}
impl Pos for H {
    const VAL: u128 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u128 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u128 = 2 * P::VAL + 1;
}

pub struct Z;
pub struct Pz<P>(PhantomData<P>);
pub trait Nat {
    const VAL: u128;
}
impl Nat for Z {
    const VAL: u128 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u128 = P::VAL;
}

// ---- type-level addition (needed: width = I + F) --------------------------
// Half of what a real migration needs. Carry-propagating binary add on Pos.
pub trait Succ {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
} // 1 -> 2
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
} // 2P -> 2P+1
impl<P: Pos + Succ> Succ for I<P> {
    type Out = O<<P as Succ>::Out>;
} // 2P+1 -> 2(P+1)

pub trait AddP<R> {
    type Out: Pos;
}
impl AddP<H> for H {
    type Out = O<H>;
} // 1+1=2
impl<P: Pos> AddP<O<P>> for H
where
    O<P>: Succ,
{
    type Out = <O<P> as Succ>::Out;
}
impl<P: Pos> AddP<I<P>> for H
where
    I<P>: Succ,
{
    type Out = <I<P> as Succ>::Out;
}
impl<P: Pos> AddP<H> for O<P> {
    type Out = I<P>;
}
impl<P: Pos> AddP<H> for I<P>
where
    I<P>: Succ,
{
    type Out = <I<P> as Succ>::Out;
}
impl<A: Pos + AddP<B>, B: Pos> AddP<O<B>> for O<A> {
    type Out = O<<A as AddP<B>>::Out>;
}
impl<A: Pos + AddP<B>, B: Pos> AddP<I<B>> for O<A> {
    type Out = I<<A as AddP<B>>::Out>;
}
impl<A: Pos + AddP<B>, B: Pos> AddP<O<B>> for I<A> {
    type Out = I<<A as AddP<B>>::Out>;
}
// 2A+1 + 2B+1 = 2(A+B+1): needs A+B then succ. One carry case, and it is the
// one that makes a hand-rolled adder a real piece of work rather than a table.
impl<A: Pos + AddP<B>, B: Pos> AddP<I<B>> for I<A>
where
    <A as AddP<B>>::Out: Succ,
{
    type Out = O<<<A as AddP<B>>::Out as Succ>::Out>;
}

pub trait AddN<R> {
    type Out: Nat;
}
impl AddN<Z> for Z {
    type Out = Z;
}
impl<P: Pos> AddN<Pz<P>> for Z {
    type Out = Pz<P>;
}
impl<P: Pos> AddN<Z> for Pz<P> {
    type Out = Pz<P>;
}
impl<A: Pos + AddP<B>, B: Pos> AddN<Pz<B>> for Pz<A> {
    type Out = Pz<<A as AddP<B>>::Out>;
}

// ---- the container dispatch, rebuilt structurally -------------------------
// Today: `tag_hot_cold(n: u16) -> usize`, ten lines of `if`.
// Here: the bucket is the bit length of the width, which for the boundaries
// arvo uses (8/16/32/64/128) is exactly `len(W) - 1` clamped, so the tag is
// derived by structural recursion over the Pos spine rather than by comparison.
pub struct B0;
pub struct B1;
pub struct B2;
pub struct B3;
pub struct B4;
pub struct B5;
pub trait Tag {}
impl Tag for B0 {}
impl Tag for B1 {}
impl Tag for B2 {}
impl Tag for B3 {}
impl Tag for B4 {}
impl Tag for B5 {}

// Bucket by structural depth. `Depth` counts bits of the width minus one,
// which lands 1..=8 in B0, 9..=16 in B1 and so on ONLY because the boundaries
// are powers of two AND the width is decremented first. That decrement is the
// piece a const fn got for free.
pub trait Bucket {
    type Out: Tag;
}
impl Bucket for Z {
    type Out = B0;
}
impl Bucket for Pz<H> {
    type Out = B0;
} // 1
impl<P: Pos> Bucket for Pz<O<P>>
where
    Pz<P>: Bucket,
{
    type Out = <Pz<P> as Bucket>::Out;
}
impl<P: Pos> Bucket for Pz<I<P>>
where
    Pz<P>: Bucket,
{
    type Out = <Pz<P> as Bucket>::Out;
}

// ---- storage ---------------------------------------------------------------
pub trait Strategy {}
pub struct Hot;
impl Strategy for Hot {}
pub trait Project<T: Tag, S: Strategy> {
    type T: Copy;
}
pub struct Picker;
impl Project<B0, Hot> for Picker {
    type T = u8;
}
impl Project<B1, Hot> for Picker {
    type T = u16;
}
impl Project<B2, Hot> for Picker {
    type T = u32;
}
impl Project<B3, Hot> for Picker {
    type T = u64;
}
impl Project<B4, Hot> for Picker {
    type T = u128;
}

pub trait BitsContainerFor<W: Nat>: Strategy {
    type T: Copy;
}
impl<W: Nat + Bucket> BitsContainerFor<W> for Hot
where
    Picker: Project<<W as Bucket>::Out, Hot>,
{
    type T = <Picker as Project<<W as Bucket>::Out, Hot>>::T;
}

#[repr(transparent)]
pub struct Bits<W: Nat, S: Strategy>(<S as BitsContainerFor<W>>::T, PhantomData<(W, S)>)
where
    S: BitsContainerFor<W>;

#[repr(transparent)]
pub struct UFixed<Ib: Nat, Fb: Nat, S: Strategy>(Bits<<Ib as AddN<Fb>>::Out, S>)
where
    Ib: AddN<Fb>,
    S: BitsContainerFor<<Ib as AddN<Fb>>::Out>;

// ---- a call site ----------------------------------------------------------
// Today:  UFixed<{ ibits(8) }, { fbits(0) }, Hot>
// Here:   UFixed<N8, N0, Hot>
pub type N0 = Z;
pub type N8 = Pz<O<O<O<H>>>>; // 8
pub type N7 = Pz<I<I<H>>>; // 7
pub type Byte = UFixed<N8, N0, Hot>;
pub type U7 = UFixed<N7, N0, Hot>;

const _: () = {
    assert!(N8::VAL == 8);
    assert!(N7::VAL == 7);
};
