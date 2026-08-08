// p2: do the fold-layer's accumulator derivation (35_probes/p7, p8) and the
// law-layer's property bound (40_probes/p3) compose into ONE signature, or do
// they collide?
//
// 35 built the accumulator-from-capacity derivation and left the accumulator
// a plain CAdd numeral with no law-bearing bound on it. 40 built the
// property-bound mechanism (S::Overflow: AbsorbingTop + MonotoneAdd) and
// tested it only on a numeral with no derived width. Nobody combined them,
// and the real algorithm-crate shape (35 section 3.8, arvo-graph's shortest
// path) needs BOTH at once: an accumulator wide enough for the capacity, AND
// an accumulator whose overflow policy absorbs and is monotone. This probe
// is that composition.
//
// The mechanism risk being tested: 35's derivation names the accumulator
// through an ASSOCIATED TYPE (`type Acc`), never through a const-generic
// width expression, precisely so the derivation stays outside
// generic_const_exprs territory. 40's property bound is stated on
// `S::Overflow` where S is a type parameter, also never a const expression.
// Both mechanisms are trait-and-associated-type shaped rather than
// const-arithmetic shaped, so there is no a priori reason they should
// collide, but nobody had built the file that shows it.
//
// Arms:
//   base       : the composed bound, on a capacity and a law-satisfying
//                accumulator strategy.                        expect: compiles
//   bad_width  : same accumulator strategy, capacity with no Log2Ceil row.
//                                                              expect: refused, no accumulator
//   bad_law    : sufficient capacity, accumulator strategy that fails the
//                law bound (wrapping: no absorbing top).       expect: refused, E0277
//   observable_swap : the SAME consumer source, generic over which
//                accumulator strategy is picked, compiled twice: once with
//                an accumulator differing only on an UNOBSERVABLE axis
//                (headroom) from the satisfying one, once differing on the
//                OBSERVABLE axis (overflow). Tests whether 40's
//                observable/unobservable split, established for a bare
//                binary operation, still separates cleanly at this
//                composition point, which nobody has tested.
//
// No feature gates.
//
//   rustc +nightly-2026-05-28 --edition 2021 p2_composed_accumulator_and_law_bound.rs \
//         --crate-type lib -o /dev/null
//   ... --cfg bad_width       (expected: E0277, SumAccum not satisfied)
//   ... --cfg bad_law         (expected: E0277, AbsorbingTop/MonotoneAdd not satisfied)

#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Widths, unary ladder, and type-level addition. Identical shape to
// 35_probes/p7_accumulator_from_capacity.rs, reused rather than re-derived,
// because re-deriving it here would be re-testing 35's result instead of
// testing the composition.
// ---------------------------------------------------------------------------

pub struct Z;
pub struct Su<N>(PhantomData<N>);

pub type N3 = Su<Su<Su<Z>>>;
pub type N4 = Su<N3>;
pub type N5 = Su<N4>;
pub type N6 = Su<N5>;
pub type N8 = Su<Su<Su<Su<N4>>>>;

pub trait NatVal {
    const VAL: u32;
}
impl NatVal for Z {
    const VAL: u32 = 0;
}
impl<N: NatVal> NatVal for Su<N> {
    const VAL: u32 = N::VAL + 1;
}

pub trait Add<Rhs> {
    type Out;
}
impl<B> Add<B> for Z {
    type Out = B;
}
impl<A, B> Add<B> for Su<A>
where
    A: Add<B>,
{
    type Out = Su<<A as Add<B>>::Out>;
}

pub struct Cap<const K: usize>;

pub trait Log2Ceil {
    type Out;
}
impl Log2Ceil for Cap<3> {
    type Out = Su<Su<Z>>; // ceil(log2 3) = 2
}
impl Log2Ceil for Cap<4> {
    type Out = Su<Su<Z>>; // ceil(log2 4) = 2
}
// deliberately no impl for Cap<7>: bad_width instantiates against it, and the
// missing row is what makes the refusal a design fact rather than an
// oversight, exactly as 35_probes/p7's arm5.

// ---------------------------------------------------------------------------
// Strategy axes and the law-bearing properties, mirroring 40_probes/p3's
// arrangement: properties implemented on the AXIS VALUE, so a strategy
// inherits them from the coordinates it names rather than restating them.
// ---------------------------------------------------------------------------

pub trait Headroom {}
pub struct Minimum;
pub struct Doubled;
impl Headroom for Minimum {}
impl Headroom for Doubled {}

pub trait Overflow {}
pub struct Wrap;
pub struct Saturate;
impl Overflow for Wrap {}
impl Overflow for Saturate {}

/// 35 section 3.4: the top absorbs at 63 of 63 (W,F) cells under saturation,
/// 0 of 63 under wrapping.
pub trait AbsorbingTop {}
impl AbsorbingTop for Saturate {}

/// 35 section 3.5: holds at 33 of 33 cells under saturation, fails at 33 of
/// 33 under wrapping.
pub trait MonotoneAdd {}
impl MonotoneAdd for Saturate {}

pub trait Strategy {
    type Headroom: Headroom;
    type Overflow: Overflow;
}
pub struct Strat<H, O>(PhantomData<(H, O)>);
impl<H: Headroom, O: Overflow> Strategy for Strat<H, O> {
    type Headroom = H;
    type Overflow = O;
}

pub type AccSat = Strat<Minimum, Saturate>; // satisfies the law bound
pub type AccSatDoubled = Strat<Doubled, Saturate>; // same law, unobservable axis moved
pub type AccWrap = Strat<Minimum, Wrap>; // fails the law bound

// ---------------------------------------------------------------------------
// The numeral, the fold-layer's accumulator relation, and the composed bound.
// ---------------------------------------------------------------------------

pub struct Num<W, S>(pub u64, PhantomData<(W, S)>);
impl<W, S> Num<W, S> {
    pub const fn new(v: u64) -> Self {
        Num(v, PhantomData)
    }
}
impl<W, S> Clone for Num<W, S> {
    fn clone(&self) -> Self {
        Num(self.0, PhantomData)
    }
}
impl<W, S> Copy for Num<W, S> {}

pub trait CAdd {
    fn cadd(self, rhs: Self) -> Self;
    fn zero() -> Self;
}
impl<W, S> CAdd for Num<W, S> {
    fn cadd(self, rhs: Self) -> Self {
        Num::new(self.0.wrapping_add(rhs.0))
    }
    fn zero() -> Self {
        Num::new(0)
    }
}

/// The fold-layer contract, from 35 section 3.2, EXTENDED with the law-layer
/// contract from 40 section 6.1: given an element numeral, a capacity, and a
/// CHOSEN accumulator strategy Sa, name the accumulator numeral (width
/// derived, per 35) and require it to satisfy the composed algebraic
/// requirement (per 40), by naming the requirement's own associated bound
/// rather than restating it inline at every call site.
pub trait TropicalSumAccum<C, Sa: Strategy> {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}

impl<We, Se, const K: usize, Sa> TropicalSumAccum<Cap<K>, Sa> for Num<We, Se>
where
    Cap<K>: Log2Ceil,
    We: Add<<Cap<K> as Log2Ceil>::Out>,
    Sa: Strategy,
    Sa::Overflow: AbsorbingTop + MonotoneAdd,
{
    type Acc = Num<<We as Add<<Cap<K> as Log2Ceil>::Out>>::Out, Sa>;
    fn lift(self) -> Self::Acc {
        Num::new(self.0)
    }
}

pub struct Bounded<T, const K: usize> {
    pub items: [T; K],
    pub live: usize,
}

/// The composed fold: the shape arvo-graph's min-plus relaxation actually
/// needs (35 section 3.4, 3.8), stated as one bound rather than two separate
/// preconditions the caller has to remember to both discharge.
pub fn min_plus_fold<We, Se, const K: usize, Sa>(
    xs: &Bounded<Num<We, Se>, K>,
) -> <Num<We, Se> as TropicalSumAccum<Cap<K>, Sa>>::Acc
where
    Sa: Strategy,
    Num<We, Se>: TropicalSumAccum<Cap<K>, Sa>,
{
    let mut acc = <<Num<We, Se> as TropicalSumAccum<Cap<K>, Sa>>::Acc as CAdd>::zero();
    let mut i = 0usize;
    while i < xs.live {
        acc = acc.cadd(xs.items[i].lift());
        i += 1;
    }
    acc
}

// ---------------------------------------------------------------------------
// arm base: capacity has a row, accumulator strategy satisfies the law.
// ---------------------------------------------------------------------------

pub fn positive(b: &Bounded<Num<N4, Wrap>, 3>) -> Num<N6, AccSat> {
    // element numeral wraps (Se = Wrap); the accumulator (Sa = AccSat) is a
    // separately chosen strategy that saturates, per 35 Q11's observation
    // that a fold is a widening LIFT into a closed add in the accumulator's
    // OWN numeral, so the element and accumulator strategies need not agree.
    min_plus_fold(b)
}

// ---------------------------------------------------------------------------
// arm bad_width: capacity 7 has no Log2Ceil row, so no accumulator relation
// exists, regardless of the accumulator strategy's law-worthiness.
// ---------------------------------------------------------------------------

#[cfg(bad_width)]
pub fn negative_width(
    b: &Bounded<Num<N4, Wrap>, 7>,
) -> <Num<N4, Wrap> as TropicalSumAccum<Cap<7>, AccSat>>::Acc {
    min_plus_fold(b)
}

// ---------------------------------------------------------------------------
// arm bad_law: capacity 3 has a row, but AccWrap's overflow policy fails the
// law bound, so the accumulator relation itself does not exist for this
// (element, capacity, accumulator-strategy) triple, independent of width.
// ---------------------------------------------------------------------------

#[cfg(bad_law)]
pub fn negative_law(
    b: &Bounded<Num<N4, Wrap>, 3>,
) -> <Num<N4, Wrap> as TropicalSumAccum<Cap<3>, AccWrap>>::Acc {
    min_plus_fold(b)
}

// ---------------------------------------------------------------------------
// arm observable_swap: one generic consumer, instantiated at two accumulator
// strategies that differ from AccSat on different axes. AccSatDoubled moves
// only the UNOBSERVABLE headroom axis and must still satisfy the bound.
// AccWrap moves the OBSERVABLE overflow axis and must not. Both calls are in
// this file; the second is gated so the first proves the positive half
// unconditionally and the whole file's default build (no cfg) demonstrates
// the unobservable-axis swap compiling clean, which is the arm this file
// ships uncommented.
// ---------------------------------------------------------------------------

pub fn generic_consumer<We, Se, const K: usize, Sa>(
    b: &Bounded<Num<We, Se>, K>,
) -> <Num<We, Se> as TropicalSumAccum<Cap<K>, Sa>>::Acc
where
    Sa: Strategy,
    Num<We, Se>: TropicalSumAccum<Cap<K>, Sa>,
{
    min_plus_fold(b)
}

pub fn observable_swap_unobservable_axis(b: &Bounded<Num<N4, Wrap>, 3>) -> Num<N6, AccSatDoubled> {
    // AccSatDoubled differs from AccSat only in headroom (Minimum vs
    // Doubled), which 40 section 5.1 classifies unobservable. Compiles.
    generic_consumer::<_, _, 3, AccSatDoubled>(b)
}

#[cfg(observable_axis)]
pub fn observable_swap_observable_axis(b: &Bounded<Num<N4, Wrap>, 3>) -> Num<N6, AccWrap> {
    // AccWrap differs from AccSat in overflow, which 40 classifies
    // observable, and it is exactly the coordinate the law bound is
    // quantified over. Must refuse, and it is the same generic_consumer
    // function instantiated differently, not a different bound.
    generic_consumer::<_, _, 3, AccWrap>(b)
}
