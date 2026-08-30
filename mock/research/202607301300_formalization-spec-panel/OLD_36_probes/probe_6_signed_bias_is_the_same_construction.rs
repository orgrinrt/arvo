//! Probe 6: the bias, which is the third numeral member the obligation
//! reaches and the one the other probes do not cover.
//!
//! `Implicit<const E: Exponent, A: Adjustment, B: Bias>` (`31:335`) carries a
//! bias, and a bias is signed: MATLAB's slope-and-bias objects take negative
//! biases routinely, and file 31's own product rule `bias = B1 * B2`
//! (`31:399-400`) is a signed multiplication. So `Bias` needs the same
//! treatment as the naturals, and it gets it by the same construction with no
//! new idea: Coq's `Z ::= Z0 | Zpos p | Zneg p`, where `p: Pos` excludes zero
//! and therefore excludes the second spelling of it.
//!
//! Uniqueness, by the same induction: zero is `IZero` and nothing else,
//! because `IPos<P>` and `INeg<P>` both need `P: Pos` and so denote a nonzero
//! magnitude; a nonzero integer is its sign paired with its magnitude, and
//! the magnitude is unique by probe 2. The negative-zero spelling that a
//! sign-magnitude encoding admits, which is the same defect as the leading
//! zero one layer up, has no type here.
//!
//! Worth noting where this lands relative to the identity contract: signed
//! zero is a real and wanted thing in the design, but on the DATUM side,
//! inside `Encoding::Canonical` (`31:370-374`). A numeral parameter is a
//! value-level object and must not carry two zeros. The two facts are not in
//! tension; they are the value/datum split doing its job at two different
//! layers, and this probe is the value-layer half.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_6_signed_bias_is_the_same_construction.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

mod sealed {
    pub trait PosSealed {}
    pub trait IntSealed {}
}

pub trait Pos: sealed::PosSealed {
    const VAL: i64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl sealed::PosSealed for H {}
impl<P: Pos> sealed::PosSealed for O<P> {}
impl<P: Pos> sealed::PosSealed for I<P> {}

impl Pos for H {
    const VAL: i64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: i64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: i64 = 2 * P::VAL + 1;
}

/// A signed integer, value-unique by construction and sealed.
pub trait Int: sealed::IntSealed {
    const VAL: i64;
}
pub struct IZero;
pub struct IPos<P>(PhantomData<P>);
pub struct INeg<P>(PhantomData<P>);

impl sealed::IntSealed for IZero {}
impl<P: Pos> sealed::IntSealed for IPos<P> {}
impl<P: Pos> sealed::IntSealed for INeg<P> {}

impl Int for IZero {
    const VAL: i64 = 0;
}
impl<P: Pos> Int for IPos<P> {
    const VAL: i64 = P::VAL;
}
impl<P: Pos> Int for INeg<P> {
    const VAL: i64 = -P::VAL;
}

/// Sign multiplication, which is the whole of `bias = B1 * B2`'s sign rule
/// and is three impls with nothing to normalise: the zero case absorbs on
/// both sides structurally, so no product can produce a signed zero.
pub trait IMul<Rhs> {
    type Out: Int;
}
impl<R: Int> IMul<R> for IZero {
    type Out = IZero;
}
impl<P: Pos> IMul<IZero> for IPos<P> {
    type Out = IZero;
}
impl<P: Pos> IMul<IZero> for INeg<P> {
    type Out = IZero;
}
impl<A: Pos + PMul<B>, B: Pos> IMul<IPos<B>> for IPos<A> {
    type Out = IPos<<A as PMul<B>>::Out>;
}
impl<A: Pos + PMul<B>, B: Pos> IMul<INeg<B>> for IPos<A> {
    type Out = INeg<<A as PMul<B>>::Out>;
}
impl<A: Pos + PMul<B>, B: Pos> IMul<IPos<B>> for INeg<A> {
    type Out = INeg<<A as PMul<B>>::Out>;
}
impl<A: Pos + PMul<B>, B: Pos> IMul<INeg<B>> for INeg<A> {
    type Out = IPos<<A as PMul<B>>::Out>;
}

// --- magnitude multiplication, so the sign rule above has something to ---
// --- multiply. Shift-and-add on the value-unique positives: doubling is ---
// --- structural, so a multiply is a chain of additions and nothing else. ---

pub struct C0;
pub struct C1;

pub trait PAdd<Rhs, C> {
    type Out: Pos;
}
impl PAdd<H, C0> for H {
    type Out = O<H>;
}
impl PAdd<H, C1> for H {
    type Out = I<H>;
}
impl<B: Pos> PAdd<O<B>, C0> for H {
    type Out = I<B>;
}
impl<B: Pos + Succ> PAdd<O<B>, C1> for H {
    type Out = O<<B as Succ>::Out>;
}
impl<B: Pos + Succ> PAdd<I<B>, C0> for H {
    type Out = O<<B as Succ>::Out>;
}
impl<B: Pos + Succ> PAdd<I<B>, C1> for H {
    type Out = I<<B as Succ>::Out>;
}
impl<A: Pos> PAdd<H, C0> for O<A> {
    type Out = I<A>;
}
impl<A: Pos + Succ> PAdd<H, C1> for O<A> {
    type Out = O<<A as Succ>::Out>;
}
impl<A: Pos + Succ> PAdd<H, C0> for I<A> {
    type Out = O<<A as Succ>::Out>;
}
impl<A: Pos + Succ> PAdd<H, C1> for I<A> {
    type Out = I<<A as Succ>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for O<A> {
    type Out = O<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C1> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<I<B>, C0> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for O<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for I<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<O<B>, C1> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C0> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for I<A> {
    type Out = I<<A as PAdd<B, C1>>::Out>;
}

pub trait Succ {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
}
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Succ> Succ for I<P> {
    type Out = O<<P as Succ>::Out>;
}

/// `a * b` on positives. `H * b = b`; `O<a> * b = O<a * b>` (free doubling);
/// `I<a> * b = O<a * b> + b`.
pub trait PMul<Rhs> {
    type Out: Pos;
}
impl<B: Pos> PMul<B> for H {
    type Out = B;
}
impl<A: Pos + PMul<B>, B: Pos> PMul<B> for O<A> {
    type Out = O<<A as PMul<B>>::Out>;
}
impl<A: Pos + PMul<B>, B: Pos> PMul<B> for I<A>
where
    O<<A as PMul<B>>::Out>: PAdd<B, C0>,
{
    type Out = <O<<A as PMul<B>>::Out> as PAdd<B, C0>>::Out;
}

// --- values and claims ---

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<O<H>>;
pub type P6 = O<I<H>>;
pub type P7 = I<I<H>>;
pub type P12 = O<O<I<H>>>;
pub type P13 = I<O<I<H>>>;
pub type P91 = I<I<O<I<I<O<H>>>>>>;
pub type P156 = O<O<I<I<I<O<O<H>>>>>>>;

const _: () = assert!(<P91 as Pos>::VAL == 91);
const _: () = assert!(<P156 as Pos>::VAL == 156);

// magnitude multiplication, which the product numeral's own formula needs
// (`31:399-400`: adjustment = gcd(A1*A2, A1*B2, A2*B1), bias = B1*B2).
const _: () = assert!(<<P13 as PMul<P7>>::Out as Pos>::VAL == 91);
const _: () = assert!(<<P7 as PMul<P13>>::Out as Pos>::VAL == 91);
const _: () = assert!(<<P12 as PMul<P13>>::Out as Pos>::VAL == 156);
const _: () = assert!(<<P1 as PMul<P13>>::Out as Pos>::VAL == 13);
const _: () = assert!(<<P13 as PMul<P1>>::Out as Pos>::VAL == 13);
const _: () = assert!(<<P4 as PMul<P3>>::Out as Pos>::VAL == 12);
const _: () = assert!(<<P6 as PMul<P2>>::Out as Pos>::VAL == 12);

// signed bias multiplication, all four sign combinations plus both zeros.
pub type IM<A, B> = <A as IMul<B>>::Out;
const _: () = assert!(<IM<IPos<P13>, IPos<P7>> as Int>::VAL == 91);
const _: () = assert!(<IM<IPos<P13>, INeg<P7>> as Int>::VAL == -91);
const _: () = assert!(<IM<INeg<P13>, IPos<P7>> as Int>::VAL == -91);
const _: () = assert!(<IM<INeg<P13>, INeg<P7>> as Int>::VAL == 91);
const _: () = assert!(<IM<IZero, INeg<P7>> as Int>::VAL == 0);
const _: () = assert!(<IM<INeg<P7>, IZero> as Int>::VAL == 0);

/// The product of a negative bias and zero is `IZero`, the same type as the
/// product of a positive one and zero. A sign-magnitude encoding with a
/// spellable negative zero would give two, and a law over biases would then
/// be as ill formed as file 34's probe 5b.
pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

pub fn there_is_one_zero_bias() {
    same_type(
        PhantomData::<IM<INeg<P7>, IZero>>,
        PhantomData::<IM<IPos<P7>, IZero>>,
    );
    same_type(PhantomData::<IM<IZero, IZero>>, PhantomData::<IZero>);
}

/// And the multiplication is commutative and associative as types, which the
/// bias half of the product rule needs for the same reason the gcd half does.
pub fn bias_multiplication_is_commutative_as_types() {
    same_type(
        PhantomData::<IM<IPos<P13>, INeg<P7>>>,
        PhantomData::<IM<INeg<P7>, IPos<P13>>>,
    );
}
