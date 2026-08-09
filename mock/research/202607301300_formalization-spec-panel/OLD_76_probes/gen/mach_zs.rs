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

// Successor and addition on the sealed encoding: the machinery route Z needs
// so that a stored width is a type. Peano would be linear in the VALUE; this
// is the binary encoding, so it is linear in the number of BITS.
pub trait SuccP {
    type Out: Pos;
}
impl SuccP for H {
    type Out = O<H>;
}
impl<P: Pos> SuccP for O<P> {
    type Out = I<P>;
}
impl<P: Pos> SuccP for I<P>
where
    P: SuccP,
{
    type Out = O<<P as SuccP>::Out>;
}

pub trait AddP<R> {
    type Out: Pos;
}
impl AddP<H> for H {
    type Out = O<H>;
}
impl<P: Pos> AddP<H> for O<P> {
    type Out = I<P>;
}
impl<P: Pos> AddP<H> for I<P>
where
    P: SuccP,
{
    type Out = O<<P as SuccP>::Out>;
}
impl<P: Pos> AddP<O<P>> for H
where
    P: SuccP,
{
    type Out = I<P>;
}
impl<P: Pos> AddP<I<P>> for H
where
    P: SuccP,
{
    type Out = O<<P as SuccP>::Out>;
}
impl<A: Pos, B: Pos> AddP<O<B>> for O<A>
where
    A: AddP<B>,
{
    type Out = O<<A as AddP<B>>::Out>;
}
impl<A: Pos, B: Pos> AddP<I<B>> for O<A>
where
    A: AddP<B>,
{
    type Out = I<<A as AddP<B>>::Out>;
}
impl<A: Pos, B: Pos> AddP<O<B>> for I<A>
where
    A: AddP<B>,
{
    type Out = I<<A as AddP<B>>::Out>;
}
impl<A: Pos, B: Pos> AddP<I<B>> for I<A>
where
    A: AddP<B>,
    <A as AddP<B>>::Out: SuccP,
{
    type Out = O<<<A as AddP<B>>::Out as SuccP>::Out>;
}

pub trait AddN<R> {
    type Out: Nat;
}
impl AddN<Z> for Z {
    type Out = Z;
}
impl<P: Pos> AddN<Z> for Pz<P> {
    type Out = Pz<P>;
}
impl<P: Pos> AddN<Pz<P>> for Z {
    type Out = Pz<P>;
}
impl<A: Pos, B: Pos> AddN<Pz<B>> for Pz<A>
where
    A: AddP<B>,
{
    type Out = Pz<<A as AddP<B>>::Out>;
}

// Obligation 2: one is representable exactly when the integer part is nonzero.
// A bound, not a bool: the absence at Z is what refuses.
pub trait OneRepresentable {}
impl<P: Pos> OneRepresentable for Pz<P> {}
// Obligation 3.
pub trait IsZero {}
impl IsZero for Z {}
pub trait NonZero {}
impl<P: Pos> NonZero for Pz<P> {}

pub struct Hot;
pub struct Warm;
pub struct Cold;
pub trait Strategy {}
impl Strategy for Hot {}
impl Strategy for Warm {}
impl Strategy for Cold {}

// Obligation 4: the capacity, split by layer (probe B2). The count is the
// shared carrier; the array grammar is the lowering-side literal.
pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
pub const fn agrees<N: Nat, const K: usize>() -> bool {
    N::VAL == K
}
pub trait Capacity: Nat {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    fn build<T: Copy>(v: T) -> Self::Array<T>;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
    fn build<T: Copy>(v: T) -> [T; K] {
        const { assert!(agrees::<N, K>(), "capacity length disagrees with its value") };
        [v; K]
    }
}

pub struct UFixed<Ib, Fb, S>(PhantomData<(Ib, Fb, S)>);
// Obligation 1: the stored width. Under this arm it is a type-level sum, and
// the solver folds it wherever a consumer reads it.
pub trait Stored {
    type Width: Nat;
    const W: usize;
}
impl<Ib: Nat + AddN<Fb>, Fb: Nat, S: Strategy> Stored for UFixed<Ib, Fb, S> {
    type Width = <Ib as AddN<Fb>>::Out;
    const W: usize = <<Ib as AddN<Fb>>::Out as Nat>::VAL;
}
// Obligation 2 at the numeral: reachable only when the integer part has one.
pub trait HasOne {}
impl<Ib: Nat + OneRepresentable, Fb: Nat, S: Strategy> HasOne for UFixed<Ib, Fb, S> {}
// Arithmetic: the sum of two numerals, which is where a width genuinely has to
// be computed rather than declared, in both arms.
pub trait AddNum<R> {
    type Out;
}
impl<Ia: Nat + AddN<Ib>, Fa: Nat + AddN<Fb>, Ib: Nat, Fb: Nat, S: Strategy>
    AddNum<UFixed<Ib, Fb, S>> for UFixed<Ia, Fa, S>
{
    type Out = UFixed<<Ia as AddN<Ib>>::Out, <Fa as AddN<Fb>>::Out, S>;
}
