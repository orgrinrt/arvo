//! The ablation gcd: the same Stein algorithm on the same value-unique
//! encoding, with typenum's odd/odd formulation instead of the tail form,
//! so the measured win can be decomposed rather than attributed wholesale
//! to the encoding. See `price/results.csv` and file 36 section 4.

use super::*;

// typenum's odd/odd impl (`typenum-1.20.1/src/uint.rs:1519-1528`) reads
//
//     Odd<Xp>: Max<Odd<Yp>> + Min<Odd<Yp>>,
//     Odd<Yp>: Max<Odd<Xp>> + Min<Odd<Xp>>,
//     Maximum<..>: Sub<Minimum<..>>,
//     Diff<Maximum<..>, Minimum<..>>: Gcd<Minimum<..>>,
//
// so it names Max and Min four times between the bounds and the output, each
// doing its own `Cmp`, subtracts at FULL width, and then falls into the
// even/odd rule to do the halving. `GcdMM` below is that, verbatim in shape,
// on the value-unique encoding: same four Max/Min bounds, same full-width
// difference, same fall-through. The only thing it does not reproduce is the
// `Trim`, which cannot exist here because there is nothing to trim.
//
// So: `Gcd` against `GcdMM` isolates the formulation, and `GcdMM` against
// typenum's own `Gcf` isolates what is left, which is the encoding.

pub trait PickMax<A, B> {
    type Out: Pos;
}
impl<A: Pos, B: Pos> PickMax<A, B> for Gt {
    type Out = A;
}
impl<A: Pos, B: Pos> PickMax<A, B> for Eq3 {
    type Out = A;
}
impl<A: Pos, B: Pos> PickMax<A, B> for Lt {
    type Out = B;
}

pub trait PickMin<A, B> {
    type Out: Pos;
}
impl<A: Pos, B: Pos> PickMin<A, B> for Gt {
    type Out = B;
}
impl<A: Pos, B: Pos> PickMin<A, B> for Eq3 {
    type Out = A;
}
impl<A: Pos, B: Pos> PickMin<A, B> for Lt {
    type Out = A;
}

pub trait MaxP<Rhs> {
    type Out: Pos;
}
impl<A: Pos + Cmp<B>, B: Pos> MaxP<B> for A
where
    <A as Cmp<B>>::Out: PickMax<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as PickMax<A, B>>::Out;
}

pub trait MinP<Rhs> {
    type Out: Pos;
}
impl<A: Pos + Cmp<B>, B: Pos> MinP<B> for A
where
    <A as Cmp<B>>::Out: PickMin<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as PickMin<A, B>>::Out;
}

/// Dispatch on whether the full-width difference vanished, mirroring
/// typenum's `Gcd<U0>` cases rather than dispatching on an ordering.
pub trait MMStep<Other> {
    type Out: Pos;
}
impl<Other: Pos> MMStep<Other> for Z {
    type Out = Other;
}
impl<P: Pos + GcdMM<Other>, Other: Pos> MMStep<Other> for Pz<P> {
    type Out = <P as GcdMM<Other>>::Out;
}

pub trait GcdMM<Rhs> {
    type Out: Pos;
}
impl<B: Pos> GcdMM<B> for H {
    type Out = H;
}
impl<A: Pos> GcdMM<H> for O<A> {
    type Out = H;
}
impl<A: Pos> GcdMM<H> for I<A> {
    type Out = H;
}
impl<A: Pos + GcdMM<B>, B: Pos> GcdMM<O<B>> for O<A> {
    type Out = O<<A as GcdMM<B>>::Out>;
}
impl<A: Pos + GcdMM<I<B>>, B: Pos> GcdMM<I<B>> for O<A> {
    type Out = <A as GcdMM<I<B>>>::Out;
}
impl<A: Pos, B: Pos> GcdMM<O<B>> for I<A>
where
    I<A>: GcdMM<B>,
{
    type Out = <I<A> as GcdMM<B>>::Out;
}
impl<A: Pos, B: Pos> GcdMM<I<B>> for I<A>
where
    I<A>: MaxP<I<B>> + MinP<I<B>>,
    I<B>: MaxP<I<A>> + MinP<I<A>>,
    Pz<<I<A> as MaxP<I<B>>>::Out>: NSub<Pz<<I<A> as MinP<I<B>>>::Out>>,
    <Pz<<I<A> as MaxP<I<B>>>::Out> as NSub<Pz<<I<A> as MinP<I<B>>>::Out>>>::Out:
        MMStep<<I<A> as MinP<I<B>>>::Out>,
{
    type Out =
        <<Pz<<I<A> as MaxP<I<B>>>::Out> as NSub<Pz<<I<A> as MinP<I<B>>>::Out>>>::Out as MMStep<
            <I<A> as MinP<I<B>>>::Out,
        >>::Out;
}
