//! Stand-in for the ratified encoding (58:850-880), trimmed to what this
//! sketch needs: `Pos`, `Nat`, `Gcd` (Stein's binary gcd) are reused
//! verbatim in shape from `42_probes/vu_nat_sealed.rs` (same constructor
//! names, same impl bodies), because this sketch is testing the notation
//! vehicle, not re-deriving arithmetic the review already compiled. `Bias`
//! is spelled exactly as 58:132-134/855 states it: `BZero | BPos<N, D> |
//! BNeg<N, D>`, `N, D: Pos`, coprime, sealed at the perimeter the same way
//! `Adjustment` already is in `42_probes`.
//!
//! Included via `#[path = "tower.rs"] mod tower;` from every consumer file
//! in this sketch, so all of them share one definition rather than drifting
//! copies.

#![allow(dead_code)]

use core::marker::PhantomData;

mod sealed {
    pub trait PosSealed {}
    pub trait NatSealed {}
    pub trait BiasSealed {}
}

pub trait Pos: sealed::PosSealed {
    const VAL: u64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl sealed::PosSealed for H {}
impl<P: Pos> sealed::PosSealed for O<P> {}
impl<P: Pos> sealed::PosSealed for I<P> {}

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat: sealed::NatSealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl sealed::NatSealed for Z {}
impl<P: Pos> sealed::NatSealed for Pz<P> {}

impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

// --- comparison, needed by Gcd's odd-step branch ---

pub trait Ord3 {}
pub struct Lt;
pub struct Eq3;
pub struct Gt;
impl Ord3 for Lt {}
impl Ord3 for Eq3 {}
impl Ord3 for Gt {}

pub trait Tie<T> {
    type Out: Ord3;
}
impl<T: Ord3> Tie<T> for Eq3 {
    type Out = T;
}
impl<T: Ord3> Tie<T> for Lt {
    type Out = Lt;
}
impl<T: Ord3> Tie<T> for Gt {
    type Out = Gt;
}

pub trait Cmp<Rhs> {
    type Out: Ord3;
}
impl Cmp<H> for H {
    type Out = Eq3;
}
impl<B: Pos> Cmp<O<B>> for H {
    type Out = Lt;
}
impl<B: Pos> Cmp<I<B>> for H {
    type Out = Lt;
}
impl<A: Pos> Cmp<H> for O<A> {
    type Out = Gt;
}
impl<A: Pos> Cmp<H> for I<A> {
    type Out = Gt;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<O<B>> for O<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<I<B>> for I<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<I<B>> for O<A>
where
    <A as Cmp<B>>::Out: Tie<Lt>,
{
    type Out = <<A as Cmp<B>>::Out as Tie<Lt>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<O<B>> for I<A>
where
    <A as Cmp<B>>::Out: Tie<Gt>,
{
    type Out = <<A as Cmp<B>>::Out as Tie<Gt>>::Out;
}

// --- decrement, subtraction (odd-step of Gcd needs both) ---

pub trait Dbl {
    type Out: Nat;
}
impl Dbl for Z {
    type Out = Z;
}
impl<P: Pos> Dbl for Pz<P> {
    type Out = Pz<O<P>>;
}

pub trait AsPos {
    type Out: Pos;
}
impl<P: Pos> AsPos for Pz<P> {
    type Out = P;
}

pub trait Dec {
    type Out: Nat;
}
impl Dec for Pz<H> {
    type Out = Z;
}
impl<P: Pos> Dec for Pz<O<P>>
where
    Pz<P>: Dec,
    <Pz<P> as Dec>::Out: DblInc,
{
    type Out = <<Pz<P> as Dec>::Out as DblInc>::Out;
}
impl<P: Pos> Dec for Pz<I<P>> {
    type Out = Pz<O<P>>;
}

pub trait DblInc {
    type Out: Nat;
}
impl DblInc for Z {
    type Out = Pz<H>;
}
impl<P: Pos> DblInc for Pz<P> {
    type Out = Pz<I<P>>;
}

pub trait NSub<Rhs> {
    type Out: Nat;
}
impl NSub<Z> for Z {
    type Out = Z;
}
impl<A: Pos> NSub<Z> for Pz<A> {
    type Out = Pz<A>;
}
impl NSub<Pz<H>> for Pz<H> {
    type Out = Z;
}
impl<A: Pos> NSub<Pz<H>> for Pz<O<A>>
where
    Pz<O<A>>: Dec,
{
    type Out = <Pz<O<A>> as Dec>::Out;
}
impl<A: Pos> NSub<Pz<H>> for Pz<I<A>> {
    type Out = Pz<O<A>>;
}
impl<A: Pos, B: Pos> NSub<Pz<O<B>>> for Pz<O<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<I<B>>> for Pz<I<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<O<B>>> for Pz<I<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: DblInc,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as DblInc>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<I<B>>> for Pz<O<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
    <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out: Dec,
{
    type Out = <<<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out as Dec>::Out;
}

pub type TailDiff<A, B> = <<Pz<A> as NSub<Pz<B>>>::Out as AsPos>::Out;

// --- Stein's binary gcd, verbatim shape from 42_probes/vu_nat_sealed.rs ---

pub trait Gcd<Rhs> {
    type Out: Pos;
}
impl<B: Pos> Gcd<B> for H {
    type Out = H;
}
impl<A: Pos> Gcd<H> for O<A> {
    type Out = H;
}
impl<A: Pos> Gcd<H> for I<A> {
    type Out = H;
}
impl<A: Pos + Gcd<B>, B: Pos> Gcd<O<B>> for O<A> {
    type Out = O<<A as Gcd<B>>::Out>;
}
impl<A: Pos + Gcd<I<B>>, B: Pos> Gcd<I<B>> for O<A> {
    type Out = <A as Gcd<I<B>>>::Out;
}
impl<A: Pos, B: Pos> Gcd<O<B>> for I<A>
where
    I<A>: Gcd<B>,
{
    type Out = <I<A> as Gcd<B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Gcd<I<B>> for I<A>
where
    <A as Cmp<B>>::Out: OddStep<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as OddStep<A, B>>::Out;
}

pub trait OddStep<A, B> {
    type Out: Pos;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Eq3 {
    type Out = I<A>;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Gt
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: AsPos,
    TailDiff<A, B>: Gcd<I<B>>,
{
    type Out = <TailDiff<A, B> as Gcd<I<B>>>::Out;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Lt
where
    Pz<B>: NSub<Pz<A>>,
    <Pz<B> as NSub<Pz<A>>>::Out: AsPos,
    TailDiff<B, A>: Gcd<I<A>>,
{
    type Out = <TailDiff<B, A> as Gcd<I<A>>>::Out;
}

// --- Bias, spelled exactly as the ratified table (58:132-134, 58:855) ---
//
// `BZero | BPos<N, D> | BNeg<N, D>`, coprime by the same perimeter shape
// `Adjustment` already uses in `42_probes/vu_nat_sealed.rs:452`: the impl is
// conditional on `N: Pos + Gcd<D, Out = H>`, so an unreduced pair is a
// well-formed TYPE that simply cannot reach any position bounded by
// `Bias`. This is the open, generically-constructible form; the sketch's
// "trusted face" variant (numeral_pm's second emission) sidesteps this
// perimeter entirely rather than widening it, per file 56 section 4.3.

mod bias_sealed {
    pub trait Sealed {}
}
pub struct BZero;
pub struct BPos<N, D>(PhantomData<(N, D)>);
pub struct BNeg<N, D>(PhantomData<(N, D)>);
impl bias_sealed::Sealed for BZero {}
impl<N: Pos, D: Pos> bias_sealed::Sealed for BPos<N, D> {}
impl<N: Pos, D: Pos> bias_sealed::Sealed for BNeg<N, D> {}

pub trait Bias: bias_sealed::Sealed {
    const NUM: i128;
    const DEN: u128;
}
impl Bias for BZero {
    const NUM: i128 = 0;
    const DEN: u128 = 1;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BPos<N, D> {
    const NUM: i128 = N::VAL as i128;
    const DEN: u128 = D::VAL as u128;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BNeg<N, D> {
    const NUM: i128 = -(N::VAL as i128);
    const DEN: u128 = D::VAL as u128;
}

// --- the notation macro's bridge trait ---
//
// NOT sealed, deliberately: this is an unbounded, per-literal vocabulary
// (one type per distinct numeral a consumer writes), the opposite of the
// four carriers above. Sealing it would defeat the point (a bounded table
// is exactly what the design already refused, 49:1004-1007). The bridge
// connects a macro-minted face to its raw `Bias` encoding without the face
// itself ever needing to satisfy `Bias`'s own private seal, which section 2
// of the writeup shows it structurally cannot from outside tower.rs's own
// module (61_probes/probe_3).
pub trait NumeralFace {
    type Encoding: Bias;
    const DISPLAY: &'static str;
}

// --- Reduce, ported verbatim from 42_probes/vu_nat_sealed.rs, added here
// ONLY for the staging-cost comparison in the writeup section on pricing:
// does the notation macro's host-side reduction actually save the
// type-level Reduce cost, or is Bias's own Gcd-bound check already paying
// an equivalent price? Unused by raw_bias!/numeral_face! themselves (both
// already reduce host-side); used only by price_unreduced_*.rs. ---

pub trait Dec2 {
    type Out: Nat;
}
impl Dec2 for Pz<H> {
    type Out = Z;
}
impl<P: Pos> Dec2 for Pz<O<P>>
where
    Pz<P>: Dec2,
    <Pz<P> as Dec2>::Out: DblInc,
{
    type Out = <<Pz<P> as Dec2>::Out as DblInc>::Out;
}
impl<P: Pos> Dec2 for Pz<I<P>> {
    type Out = Pz<O<P>>;
}

/// Strip the common factor of two from a pair, structurally.
pub trait Strip2 {
    type N: Pos;
    type D: Pos;
}
impl<A: Pos, B: Pos> Strip2 for BiasRatio<O<A>, O<B>>
where
    BiasRatio<A, B>: Strip2,
{
    type N = <BiasRatio<A, B> as Strip2>::N;
    type D = <BiasRatio<A, B> as Strip2>::D;
}
impl<A: Pos, B: Pos> Strip2 for BiasRatio<O<A>, I<B>> {
    type N = O<A>;
    type D = I<B>;
}
impl<A: Pos, B: Pos> Strip2 for BiasRatio<I<A>, O<B>> {
    type N = I<A>;
    type D = O<B>;
}
impl<A: Pos, B: Pos> Strip2 for BiasRatio<I<A>, I<B>> {
    type N = I<A>;
    type D = I<B>;
}
impl<A: Pos> Strip2 for BiasRatio<O<A>, H> {
    type N = O<A>;
    type D = H;
}
impl<A: Pos> Strip2 for BiasRatio<I<A>, H> {
    type N = I<A>;
    type D = H;
}
impl<B: Pos> Strip2 for BiasRatio<H, O<B>> {
    type N = H;
    type D = O<B>;
}
impl<B: Pos> Strip2 for BiasRatio<H, I<B>> {
    type N = H;
    type D = I<B>;
}
impl Strip2 for BiasRatio<H, H> {
    type N = H;
    type D = H;
}

pub struct BiasRatio<N, D>(PhantomData<(N, D)>);

pub trait ExactDivOdd<D> {
    type Out: Nat;
}
impl<D: Pos> ExactDivOdd<D> for Z {
    type Out = Z;
}
impl<P: Pos, D: Pos> ExactDivOdd<D> for Pz<O<P>>
where
    Pz<P>: ExactDivOdd<D>,
    <Pz<P> as ExactDivOdd<D>>::Out: Dbl,
{
    type Out = <<Pz<P> as ExactDivOdd<D>>::Out as Dbl>::Out;
}
impl<D: Pos> ExactDivOdd<D> for Pz<H>
where
    Pz<H>: NSub<Pz<D>>,
    <Pz<H> as NSub<Pz<D>>>::Out: HalveEven2,
    <<Pz<H> as NSub<Pz<D>>>::Out as HalveEven2>::Out: ExactDivOdd<D>,
    <<<Pz<H> as NSub<Pz<D>>>::Out as HalveEven2>::Out as ExactDivOdd<D>>::Out: DblInc,
{
    type Out =
        <<<<Pz<H> as NSub<Pz<D>>>::Out as HalveEven2>::Out as ExactDivOdd<D>>::Out as DblInc>::Out;
}
impl<P: Pos, D: Pos> ExactDivOdd<D> for Pz<I<P>>
where
    Pz<I<P>>: NSub<Pz<D>>,
    <Pz<I<P>> as NSub<Pz<D>>>::Out: HalveEven2,
    <<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven2>::Out: ExactDivOdd<D>,
    <<<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven2>::Out as ExactDivOdd<D>>::Out: DblInc,
{
    type Out = <<<<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven2>::Out as ExactDivOdd<D>>::Out as DblInc>::Out;
}

pub trait HalveEven2 {
    type Out: Nat;
}
impl HalveEven2 for Z {
    type Out = Z;
}
impl<P: Pos> HalveEven2 for Pz<O<P>> {
    type Out = Pz<P>;
}

pub trait Reduce {
    type N: Pos;
    type D: Pos;
}
impl<N: Pos, D: Pos> Reduce for BiasRatio<N, D>
where
    BiasRatio<N, D>: Strip2,
    <BiasRatio<N, D> as Strip2>::N: Gcd<<BiasRatio<N, D> as Strip2>::D>,
    Pz<<BiasRatio<N, D> as Strip2>::N>:
        ExactDivOdd<<<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out>,
    Pz<<BiasRatio<N, D> as Strip2>::D>:
        ExactDivOdd<<<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out>,
    <Pz<<BiasRatio<N, D> as Strip2>::N> as ExactDivOdd<
        <<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
    <Pz<<BiasRatio<N, D> as Strip2>::D> as ExactDivOdd<
        <<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
{
    type N = <<Pz<<BiasRatio<N, D> as Strip2>::N> as ExactDivOdd<
        <<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out,
    >>::Out as AsPos>::Out;
    type D = <<Pz<<BiasRatio<N, D> as Strip2>::D> as ExactDivOdd<
        <<BiasRatio<N, D> as Strip2>::N as Gcd<<BiasRatio<N, D> as Strip2>::D>>::Out,
    >>::Out as AsPos>::Out;
}

/// An unreduced pair, reduced BY THE TYPE CHECKER via `Reduce`, then read
/// through `Bias`'s own verification bound. This is what a notation macro
/// that did NOT reduce host-side would force every consumer to pay.
pub type ReducedBias<N, D> = BPos<<BiasRatio<N, D> as Reduce>::N, <BiasRatio<N, D> as Reduce>::D>;
