// p3: is a segmented format expressible in the typestate, gate-free, and does
// it erase?
//
// `03` section 9, `06` section 9 and `07` section 8 each name this probe as
// owed and none of them wrote it. The question is narrow: the canonical
// exponent of a fixed-point numeral is a constant and of a float is a line, and
// the shapes that a join produces, and that a tapered format like a posit has,
// are neither. Can the typestate carry a canonical exponent that is neither?
//
// Everything here is scaffolding except the answer. The names, the arities, the
// direction the list is written in and the offset encoding of the exponent are
// chosen to reach the check and are not design decisions. What the file
// establishes is that the check compiles, that the ordering refuses at type
// check rather than at monomorphisation, and that the guarded conversion is the
// same instructions as the unguarded one.
//
// NO `#![feature(...)]` APPEARS IN THIS FILE. That is stated positively because
// it is checkable: if any of this needed a forbidden feature the file would not
// build on the pin.

#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------- type-level
// A grid exponent, offset so it is non-negative. The offset is scaffolding.

pub struct Z;
pub struct S<N>(PhantomData<N>);

// A canonical exponent function over a bounded window of binades, written
// coarsest-binade-last. Nil ends the window.
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);

// ---------------------------------------------------------------- ordering
// `A: AtMost<B>` reads: grid exponent A is no larger than B, so A's grid is at
// least as fine. Two impls, inductive, no enumeration of widths anywhere.

pub trait AtMost<Rhs> {}
impl<N> AtMost<N> for Z {}
impl<A, B> AtMost<S<B>> for S<A> where A: AtMost<B> {}

// `B: FinerThan<A>` reads: every value of the format A is a value of the format
// B, as far as the grid is concerned. The endpoint half of the inclusion test
// is deliberately absent; this probe is about the exponent coordinate only and
// says so rather than implying it covers the whole predicate.
pub trait FinerThan<A> {}
impl FinerThan<Nil> for Nil {}
impl<HB, TB, HA, TA> FinerThan<Cons<HA, TA>> for Cons<HB, TB>
where
    HB: AtMost<HA>,
    TB: FinerThan<TA>,
{
}

// ---------------------------------------------------------------- lattice
// Pointwise maximum and minimum of two canonical exponents, as associated
// types. The meet of two formats is the pointwise maximum (a coarser grid
// everywhere); the join is the pointwise minimum.

pub trait Larger<Rhs> {
    type Out;
}
impl Larger<Z> for Z {
    type Out = Z;
}
impl<B> Larger<S<B>> for Z {
    type Out = S<B>;
}
impl<A> Larger<Z> for S<A> {
    type Out = S<A>;
}
impl<A, B> Larger<S<B>> for S<A>
where
    A: Larger<B>,
{
    type Out = S<<A as Larger<B>>::Out>;
}

pub trait Smaller<Rhs> {
    type Out;
}
impl Smaller<Z> for Z {
    type Out = Z;
}
impl<B> Smaller<S<B>> for Z {
    type Out = Z;
}
impl<A> Smaller<Z> for S<A> {
    type Out = Z;
}
impl<A, B> Smaller<S<B>> for S<A>
where
    A: Smaller<B>,
{
    type Out = S<<A as Smaller<B>>::Out>;
}

pub trait Meet<Rhs> {
    type Out;
}
impl Meet<Nil> for Nil {
    type Out = Nil;
}
impl<HA, TA, HB, TB> Meet<Cons<HB, TB>> for Cons<HA, TA>
where
    HA: Larger<HB>,
    TA: Meet<TB>,
{
    type Out = Cons<<HA as Larger<HB>>::Out, <TA as Meet<TB>>::Out>;
}

pub trait Join<Rhs> {
    type Out;
}
impl Join<Nil> for Nil {
    type Out = Nil;
}
impl<HA, TA, HB, TB> Join<Cons<HB, TB>> for Cons<HA, TA>
where
    HA: Smaller<HB>,
    TA: Join<TB>,
{
    type Out = Cons<<HA as Smaller<HB>>::Out, <TA as Join<TB>>::Out>;
}

// ---------------------------------------------------------------- the shapes
// A three-binade window. The grid exponent is written offset by two, so 0 is
// the finest grid the window admits.

type N0 = Z;
type N1 = S<Z>;
type N2 = S<S<Z>>;
type N3 = S<S<S<Z>>>;
type N4 = S<S<S<S<Z>>>>;

/// A fixed-point format: the grid exponent is constant.
pub type Fixed = Cons<N1, Cons<N1, Cons<N1, Nil>>>;
/// A float: the grid exponent rises by one per binade.
pub type Float = Cons<N0, Cons<N1, Cons<N2, Nil>>>;
/// What the meet of those two must be: constant, then slope one. This is
/// gradual underflow, which the design already names.
pub type Underflow = Cons<N1, Cons<N1, Cons<N2, Nil>>>;
/// What the join of those two must be: slope one, then constant. The design
/// has no name for this one.
pub type Unnamed = Cons<N0, Cons<N1, Cons<N1, Nil>>>;
/// A tapered format: the grid exponent rises by two per binade. No maximum or
/// minimum of the two named shapes reaches it.
pub type Tapered = Cons<N0, Cons<N2, Cons<N4, Nil>>>;

// ---------------------------------------------------------------- the checks
// Type equality as a compile-time assertion.

pub trait Same<T> {}
impl<T> Same<T> for T {}

const fn assert_same<A, B>()
where
    A: Same<B>,
{
}

/// The meet of a fixed-point format and a float is gradual underflow.
pub const fn check_meet() {
    assert_same::<<Fixed as Meet<Float>>::Out, Underflow>();
}

/// The join of the same two is the shape with no name.
pub const fn check_join() {
    assert_same::<<Fixed as Join<Float>>::Out, Unnamed>();
}

/// Both named shapes include into their join, and their meet includes into
/// both. Stated as bounds, so the compiler checks them rather than a runtime
/// assertion.
pub fn check_order() {
    fn need_finer<B: FinerThan<A>, A>() {}
    need_finer::<Unnamed, Fixed>();
    need_finer::<Unnamed, Float>();
    need_finer::<Fixed, Underflow>();
    need_finer::<Float, Underflow>();
    // A false claim, kept as a comment because the compiler refused it and the
    // refusal is recorded in `p3_negctl.out`. Tapered is (0,2,4) and Underflow
    // is (1,1,2), so Tapered is coarser at the second binade and the ordering
    // must fail there, which is exactly what the diagnostic names.
    //   need_finer::<Tapered, Underflow>();
    // The true claims about the tapered shape:
    need_finer::<Finest, Tapered>();
    need_finer::<Float, Tapered>();
}

/// The finest format the window admits.
pub type Finest = Cons<N0, Cons<N0, Cons<N0, Nil>>>;

// ---------------------------------------------------------------- erasure
// A datum carrying its format in the type. The conversion is guarded by the
// order and is a move.

#[repr(transparent)]
pub struct Datum<F>(u64, PhantomData<F>);

impl<F> Datum<F> {
    pub const fn new(raw: u64) -> Self {
        Datum(raw, PhantomData)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}

/// A lossless widening, refused at type check when the target's grid is not at
/// least as fine at every binade.
#[inline(never)]
pub fn widen<A, B>(x: Datum<A>) -> Datum<B>
where
    B: FinerThan<A>,
{
    Datum(x.0, PhantomData)
}

/// The same move with no guard, as the baseline the guarded one is compared
/// against.
#[inline(never)]
pub fn widen_bare(x: u64) -> u64 {
    x
}

#[inline(never)]
pub fn call_fixed_to_unnamed(x: u64) -> u64 {
    widen::<Fixed, Unnamed>(Datum::new(x)).raw()
}

#[inline(never)]
pub fn call_float_to_unnamed(x: u64) -> u64 {
    widen::<Float, Unnamed>(Datum::new(x)).raw()
}

#[inline(never)]
pub fn call_tapered_to_finest(x: u64) -> u64 {
    widen::<Tapered, Finest>(Datum::new(x)).raw()
}
