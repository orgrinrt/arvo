//! Copy of 41_probes/vu_nat.rs (itself a copy of 36_probes/vu_nat.rs), the
//! UNSEALED tower everything downstream of file 36 actually composes with:
//! Adjustment's own blanket impl, the price sweeps, and file 41's Bias all
//! build on this exact module, not on the standalone, orphaned
//! probe_5_sealed_perimeter_lib.rs the consolidation cites at 40:446-448.
//! Kept unsealed here on purpose, as the base state the attack in probe 1/2
//! runs against. vu_nat_sealed.rs in this directory is the fix.

//! The value-unique natural tower, as one module, so probe 4 and the price
//! sweep include the same text rather than two copies that can decorrelate.
//! Probes 2 and 3 carry their own standalone copies on purpose, because each
//! is evidence about the encoding and the gcd respectively and should compile
//! without anything else present.
//!
//! Contents, in dependency order: the encoding (`Pos`, `Nat`), the smart
//! constructors (`Dbl`, `DblInc`) that absorb what `typenum` spends a `Trim`
//! on, comparison, decrement, partial subtraction, Stein's gcd, exact
//! division by an odd divisor (LSB-first, Jebelean/Hensel shape), and
//! reduction of a ratio to lowest terms.
//!
//! Included with `#[path = "vu_nat.rs"] mod nat;`.

#![allow(dead_code)]

use core::marker::PhantomData;

// --- encoding ---

pub trait Pos {
    const VAL: u64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

// --- smart constructors ---

pub trait Dbl {
    type Out: Nat;
}
impl Dbl for Z {
    type Out = Z;
}
impl<P: Pos> Dbl for Pz<P> {
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

pub trait AsPos {
    type Out: Pos;
}
impl<P: Pos> AsPos for Pz<P> {
    type Out = P;
}

/// Halving an even `Nat`. Structural: strip the outer `O`. No impl for odd,
/// and none is needed, because every call site has already matched on parity.
pub trait HalveEven {
    type Out: Nat;
}
impl HalveEven for Z {
    type Out = Z;
}
impl<P: Pos> HalveEven for Pz<O<P>> {
    type Out = Pz<P>;
}

// --- comparison ---

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

// --- decrement and partial subtraction ---

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

// --- Stein's binary gcd ---

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

// --- exact division by an odd divisor, LSB-first ---
//
// This is the 2-adic (Hensel) exact division Jebelean describes for
// multiprecision arithmetic, and it is the shape this encoding wants: the
// quotient's digits fall out least-significant first, which is the direction
// the outer constructor already reads. Each step is one parity match, one
// subtraction, and one structural halving. There is no comparison anywhere
// and no trial digit to retract, because exactness is the precondition.
//
// The contrast with the prior art is the direction, not the algorithm:
// `typenum`'s `Div` is MSB-first long division (`PrivateDiv` with a
// `PrivateDivIf` per step, `typenum-1.20.1/src/uint.rs`), which needs a
// comparison per digit and an `Invert` to walk the chain from the wrong end.
// Exact division is the only division a reduction needs, and it is the
// cheaper one.

pub trait ExactDivOdd<D> {
    type Out: Nat;
}
/// 0 / d = 0.
impl<D: Pos> ExactDivOdd<D> for Z {
    type Out = Z;
}
/// n even: the quotient's low digit is 0, recurse on n/2.
impl<P: Pos, D: Pos> ExactDivOdd<D> for Pz<O<P>>
where
    Pz<P>: ExactDivOdd<D>,
    <Pz<P> as ExactDivOdd<D>>::Out: Dbl,
{
    type Out = <<Pz<P> as ExactDivOdd<D>>::Out as Dbl>::Out;
}
/// n odd: the quotient's low digit is 1 (d is odd), recurse on (n - d)/2.
impl<D: Pos> ExactDivOdd<D> for Pz<H>
where
    Pz<H>: NSub<Pz<D>>,
    <Pz<H> as NSub<Pz<D>>>::Out: HalveEven,
    <<Pz<H> as NSub<Pz<D>>>::Out as HalveEven>::Out: ExactDivOdd<D>,
    <<<Pz<H> as NSub<Pz<D>>>::Out as HalveEven>::Out as ExactDivOdd<D>>::Out: DblInc,
{
    type Out =
        <<<<Pz<H> as NSub<Pz<D>>>::Out as HalveEven>::Out as ExactDivOdd<D>>::Out as DblInc>::Out;
}
impl<P: Pos, D: Pos> ExactDivOdd<D> for Pz<I<P>>
where
    Pz<I<P>>: NSub<Pz<D>>,
    <Pz<I<P>> as NSub<Pz<D>>>::Out: HalveEven,
    <<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven>::Out: ExactDivOdd<D>,
    <<<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven>::Out as ExactDivOdd<D>>::Out: DblInc,
{
    type Out = <<<<Pz<I<P>> as NSub<Pz<D>>>::Out as HalveEven>::Out as ExactDivOdd<D>>::Out as DblInc>::Out;
}

// --- reduction of a ratio to lowest terms ---

pub struct Ratio<N, D>(PhantomData<(N, D)>);

/// Strip the common factor of two from a pair, structurally. Free: it is
/// two constructor peels per step and nothing else. After it, at least one
/// side is odd, so the remaining gcd is odd and `ExactDivOdd` applies.
pub trait Strip2 {
    type N: Pos;
    type D: Pos;
}
impl<A: Pos, B: Pos> Strip2 for Ratio<O<A>, O<B>>
where
    Ratio<A, B>: Strip2,
{
    type N = <Ratio<A, B> as Strip2>::N;
    type D = <Ratio<A, B> as Strip2>::D;
}
impl<A: Pos, B: Pos> Strip2 for Ratio<O<A>, I<B>> {
    type N = O<A>;
    type D = I<B>;
}
impl<A: Pos, B: Pos> Strip2 for Ratio<I<A>, O<B>> {
    type N = I<A>;
    type D = O<B>;
}
impl<A: Pos, B: Pos> Strip2 for Ratio<I<A>, I<B>> {
    type N = I<A>;
    type D = I<B>;
}
impl<A: Pos> Strip2 for Ratio<O<A>, H> {
    type N = O<A>;
    type D = H;
}
impl<A: Pos> Strip2 for Ratio<I<A>, H> {
    type N = I<A>;
    type D = H;
}
impl<B: Pos> Strip2 for Ratio<H, O<B>> {
    type N = H;
    type D = O<B>;
}
impl<B: Pos> Strip2 for Ratio<H, I<B>> {
    type N = H;
    type D = I<B>;
}
impl Strip2 for Ratio<H, H> {
    type N = H;
    type D = H;
}

/// `Reduce` is the normal form: divide both sides by their gcd.
pub trait Reduce {
    type N: Pos;
    type D: Pos;
}
impl<N: Pos, D: Pos> Reduce for Ratio<N, D>
where
    Ratio<N, D>: Strip2,
    <Ratio<N, D> as Strip2>::N: Gcd<<Ratio<N, D> as Strip2>::D>,
    Pz<<Ratio<N, D> as Strip2>::N>:
        ExactDivOdd<<<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out>,
    Pz<<Ratio<N, D> as Strip2>::D>:
        ExactDivOdd<<<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out>,
    <Pz<<Ratio<N, D> as Strip2>::N> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
    <Pz<<Ratio<N, D> as Strip2>::D> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
{
    type N = <<Pz<<Ratio<N, D> as Strip2>::N> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out as AsPos>::Out;
    type D = <<Pz<<Ratio<N, D> as Strip2>::D> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out as AsPos>::Out;
}

/// The normalised ratio as a type, which is what a numeral's adjustment is.
pub type Reduced<N, D> = Ratio<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;

// --- the perimeter: only a coprime pair is an `Adjustment` ---

/// A rational adjustment. The impl is conditional on the pair being coprime,
/// stated as an associated-type equality on the gcd, so a non-reduced ratio
/// is a well-formed type that cannot reach any position bounded by this
/// trait. This is the same shape as `O<Z>` not being a `Pos`: the illegal
/// state is unrepresentable where it is observed, rather than absent by
/// convention.
pub trait Adjustment {
    const NUM: u64;
    const DEN: u64;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for Ratio<N, D> {
    const NUM: u64 = N::VAL;
    const DEN: u64 = D::VAL;
}
