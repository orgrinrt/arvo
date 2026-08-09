// The tower, reproduced from 110:3312-3382's own declarations, trimmed to what the
// surface question needs: the sealed carrier, Nat/Pos, type-level addition, Number.
#![no_std]
#![allow(dead_code, incomplete_features)]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
pub use sealed::Sealed;

// ---- the sealed bottom carrier ------------------------------------------
pub trait Pos: Sealed {
    const VAL: u64;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);

impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat: Sealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);

impl Sealed for Z {}
impl<P: Pos> Sealed for Pz<P> {}
impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

// ---- type-level addition on Nat, via Pos with carry ---------------------
pub trait PosAdd<Rhs: Pos>: Pos {
    type Out: Pos;
}
pub trait PosAdd1<Rhs: Pos>: Pos {
    type Out: Pos;
} // rhs + self + 1

// succ on Pos
pub trait PosSucc: Pos {
    type Out: Pos;
}
impl PosSucc for H {
    type Out = O<H>;
}
impl<P: Pos> PosSucc for O<P> {
    type Out = I<P>;
}
impl<P: Pos + PosSucc> PosSucc for I<P> {
    type Out = O<<P as PosSucc>::Out>;
}

// a + b
impl PosAdd<H> for H {
    type Out = O<H>;
}
impl<B: Pos> PosAdd<O<B>> for H {
    type Out = I<B>;
}
impl<B: Pos + PosSucc> PosAdd<I<B>> for H {
    type Out = O<<B as PosSucc>::Out>;
}
impl<A: Pos + PosSucc> PosAdd<H> for O<A> {
    type Out = I<A>;
}
impl<A: Pos + PosSucc> PosAdd<H> for I<A> {
    type Out = O<<A as PosSucc>::Out>;
}
impl<A: Pos + PosAdd<B>, B: Pos> PosAdd<O<B>> for O<A> {
    type Out = O<<A as PosAdd<B>>::Out>;
}
impl<A: Pos + PosAdd<B>, B: Pos> PosAdd<I<B>> for O<A> {
    type Out = I<<A as PosAdd<B>>::Out>;
}
impl<A: Pos + PosAdd<B>, B: Pos> PosAdd<O<B>> for I<A> {
    type Out = I<<A as PosAdd<B>>::Out>;
}
impl<A: Pos + PosAdd1<B>, B: Pos> PosAdd<I<B>> for I<A> {
    type Out = O<<A as PosAdd1<B>>::Out>;
}

// a + b + 1
impl PosAdd1<H> for H {
    type Out = I<H>;
}
impl<B: Pos + PosSucc> PosAdd1<O<B>> for H {
    type Out = O<<B as PosSucc>::Out>;
}
impl<B: Pos + PosSucc> PosAdd1<I<B>> for H {
    type Out = I<<B as PosSucc>::Out>;
}
impl<A: Pos + PosSucc> PosAdd1<H> for O<A> {
    type Out = O<<A as PosSucc>::Out>;
}
impl<A: Pos + PosSucc> PosAdd1<H> for I<A> {
    type Out = I<<A as PosSucc>::Out>;
}
impl<A: Pos + PosAdd<B>, B: Pos> PosAdd1<O<B>> for O<A> {
    type Out = I<<A as PosAdd<B>>::Out>;
}
impl<A: Pos + PosAdd1<B>, B: Pos> PosAdd1<I<B>> for O<A> {
    type Out = O<<A as PosAdd1<B>>::Out>;
}
impl<A: Pos + PosAdd1<B>, B: Pos> PosAdd1<O<B>> for I<A> {
    type Out = O<<A as PosAdd1<B>>::Out>;
}
impl<A: Pos + PosAdd1<B>, B: Pos> PosAdd1<I<B>> for I<A> {
    type Out = I<<A as PosAdd1<B>>::Out>;
}

pub trait NatAdd<Rhs: Nat>: Nat {
    type Out: Nat;
}
impl NatAdd<Z> for Z {
    type Out = Z;
}
impl<B: Pos> NatAdd<Pz<B>> for Z {
    type Out = Pz<B>;
}
impl<A: Pos> NatAdd<Z> for Pz<A> {
    type Out = Pz<A>;
}
impl<A: Pos + PosAdd<B>, B: Pos> NatAdd<Pz<B>> for Pz<A> {
    type Out = Pz<<A as PosAdd<B>>::Out>;
}

pub type Sum<A, B> = <A as NatAdd<B>>::Out;

// ---- the contracts, minimal ---------------------------------------------
pub trait Radix: Sealed {}
pub struct Rad<P: Pos>(PhantomData<P>);
impl<P: Pos> Sealed for Rad<P> {}
impl<P: Pos> Radix for Rad<P> {}
pub type Binary = Rad<O<H>>;

pub trait ExponentForm: Sealed {}
pub struct Implicit;
pub struct Ranged;
impl Sealed for Implicit {}
impl Sealed for Ranged {}
impl ExponentForm for Implicit {}
impl ExponentForm for Ranged {}

pub trait SignDomain: Sealed {}
pub struct NonNegative;
pub struct Symmetric;
impl Sealed for NonNegative {}
impl Sealed for Symmetric {}
impl SignDomain for NonNegative {}
impl SignDomain for Symmetric {}

pub trait Precision: Nat {}
impl<T: Nat> Precision for T {}

pub trait Numeral {
    type Radix: Radix;
    type Precision: Precision;
    type Exponent: ExponentForm;
    type Domain: SignDomain;
}

pub trait Policy {}
pub trait Lowering {
    type Container;
}
pub struct Warm;
pub struct Hot;
impl Policy for Warm {}
impl Lowering for Warm {
    type Container = u64;
}
impl Policy for Hot {}
impl Lowering for Hot {
    type Container = u32;
}

pub struct Number<N: Numeral, S: Policy + Lowering> {
    datum: <S as Lowering>::Container,
    _numeral: PhantomData<N>,
}

// The fixed-point numeral: precision is the sum of the two written widths.
pub struct FixedNumeral<P: Precision, D: SignDomain>(PhantomData<(P, D)>);
impl<P: Precision, D: SignDomain> Numeral for FixedNumeral<P, D> {
    type Radix = Binary;
    type Precision = P;
    type Exponent = Implicit;
    type Domain = D;
}
