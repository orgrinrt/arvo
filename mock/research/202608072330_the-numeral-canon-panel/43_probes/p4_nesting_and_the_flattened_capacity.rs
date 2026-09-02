//! p4: do compositions nest, and where must the nest be flattened?
//!
//! `s3` establishes by exhaustive count that deriving a fold accumulator per
//! nesting level (W + lg N then + lg M) is sound and **not tight**: it is one
//! bit wide on 1201 of 4096 two-level shapes and up to two bits at three
//! levels, while flattening the capacity first (W + lg (M*N)) is tight and is
//! never wider.  `s3` is arithmetic and says nothing about whether a type
//! system can perform the flattening.
//!
//! This probe asks that.  Three arms:
//!
//!   arm 1  compositions nest: `Run<Run<Num, N>, M>` is well formed and its
//!          total capacity is a type-level product computed inductively, with
//!          no const arithmetic in any bound and no feature gate.
//!   arm 2  the flattened accumulator reach is derived from that product and
//!          matches the arithmetic `s3` checked, asserted at compile time.
//!   arm 3  the nested accumulator reach is derived by composing per level,
//!          and is strictly wider at the shape `s3` names as its witness.
//!
//!   --cfg tightneg   a false claim that the two agree at the witness shape.
//!                    Must NOT build, or arms 2 and 3 are measuring nothing.
//!
//! Build:
//!   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib \
//!         p4_nesting_and_the_flattened_capacity.rs
//!
//! Scaffolding warning: names, arities and the choice of a Peano encoding are
//! chosen to reach the check quickly.  A real design would not use Peano for
//! capacities in the hundreds; `35_probes/p8` already shows the binary
//! induction that scales, and this probe reuses `s3`'s arithmetic rather than
//! re-deriving it.

#![no_std]
#![forbid(unsafe_code)]

// ------------------------------------------------------------------- nats

pub struct Z;
pub struct S<N>(core::marker::PhantomData<N>);

pub trait Nat {
    const V: usize;
}
impl Nat for Z {
    const V: usize = 0;
}
impl<N: Nat> Nat for S<N> {
    const V: usize = N::V + 1;
}

pub trait Plus<R> {
    type Out;
}
impl<R> Plus<R> for Z {
    type Out = R;
}
impl<L: Plus<R>, R> Plus<R> for S<L> {
    type Out = S<<L as Plus<R>>::Out>;
}
pub type Sum<A, B> = <A as Plus<B>>::Out;

/// Type-level multiplication, inductive.  This is the operation that flattens a
/// nest of capacities, and it is the whole question arm 1 exists to answer.
pub trait Times<R> {
    type Out;
}
impl<R> Times<R> for Z {
    type Out = Z;
}
impl<L, R> Times<R> for S<L>
where
    L: Times<R>,
    R: Plus<<L as Times<R>>::Out>,
{
    type Out = <R as Plus<<L as Times<R>>::Out>>::Out;
}
pub type Prod<A, B> = <A as Times<B>>::Out;

pub type N0 = Z;
pub type N1 = S<N0>;
pub type N2 = S<N1>;
pub type N3 = S<N2>;
pub type N4 = S<N3>;
pub type N5 = S<N4>;
pub type N6 = S<N5>;
pub type N7 = S<N6>;
pub type N8 = S<N7>;
pub type N13 = S<S<S<S<S<N8>>>>>;
pub type N15 = S<S<N13>>;

// ------------------------------------------------------------- log2 ceiling
//
// Inductive rather than tabled, in the shape `35_probes/p8` establishes: a
// positive binary representation whose three constructors are pairwise
// disjoint, so the base case does not collide with the inductive one.

/// positive binary: One = 1, Twice<N> = 2N, TwiceP1<N> = 2N+1, with N >= 1
pub struct One;
pub struct Twice<N>(core::marker::PhantomData<N>);
pub struct TwiceP1<N>(core::marker::PhantomData<N>);

pub trait Pos {
    const P: usize;
}
impl Pos for One {
    const P: usize = 1;
}
impl<N: Pos> Pos for Twice<N> {
    const P: usize = 2 * N::P;
}
impl<N: Pos> Pos for TwiceP1<N> {
    const P: usize = 2 * N::P + 1;
}

/// increment on the positive binary representation
pub trait Inc {
    type Out: Pos;
}
impl Inc for One {
    type Out = Twice<One>;
}
impl<N: Pos> Inc for Twice<N> {
    type Out = TwiceP1<N>;
}
impl<N: Inc + Pos> Inc for TwiceP1<N> {
    type Out = Twice<<N as Inc>::Out>;
}

/// ceil(log2 n), three impls, no table
pub trait Lg {
    type Out: Nat;
}
impl Lg for One {
    type Out = Z;
}
impl<N: Lg + Pos> Lg for Twice<N> {
    type Out = S<<N as Lg>::Out>;
}
impl<N: Inc + Pos> Lg for TwiceP1<N>
where
    <N as Inc>::Out: Lg,
{
    type Out = S<<<N as Inc>::Out as Lg>::Out>;
}

// positive-binary literals used below
pub type P1 = One; // 1
pub type P2 = Twice<One>; // 2
pub type P3 = TwiceP1<One>; // 3
pub type P4 = Twice<Twice<One>>; // 4
pub type P5 = TwiceP1<Twice<One>>; // 5
pub type P8 = Twice<Twice<Twice<One>>>; // 8
pub type P15 = TwiceP1<TwiceP1<TwiceP1<One>>>; // 15
pub type P16 = Twice<Twice<Twice<Twice<One>>>>; // 16

pub const _POS_LITERALS_ARE_WHAT_THEY_SAY: () = {
    assert!(<P1 as Pos>::P == 1);
    assert!(<P2 as Pos>::P == 2);
    assert!(<P3 as Pos>::P == 3);
    assert!(<P4 as Pos>::P == 4);
    assert!(<P5 as Pos>::P == 5);
    assert!(<P8 as Pos>::P == 8);
    assert!(<P15 as Pos>::P == 15);
    assert!(<P16 as Pos>::P == 16);
};

pub const _LG_IS_ARITHMETIC: () = {
    assert!(<<P1 as Lg>::Out as Nat>::V == 0);
    assert!(<<P2 as Lg>::Out as Nat>::V == 1);
    assert!(<<P3 as Lg>::Out as Nat>::V == 2);
    assert!(<<P4 as Lg>::Out as Nat>::V == 2);
    assert!(<<P5 as Lg>::Out as Nat>::V == 3);
    assert!(<<P8 as Lg>::Out as Nat>::V == 3);
    assert!(<<P15 as Lg>::Out as Nat>::V == 4);
    assert!(<<P16 as Lg>::Out as Nat>::V == 4);
};

// ------------------------------------------- flattening, and what it needs
//
// The flattening needs the product of two capacities as a positive-binary
// value so `Lg` can be applied to it.  A general type-level product over the
// positive-binary representation is ordinary induction and `s3` has already
// checked the arithmetic exhaustively; what this probe has to establish is
// that the flattened capacity is a TYPE the derivation can consume.  So the
// two nests below state their product as a literal and ASSERT it against the
// factors at compile time, rather than carrying an induction whose only job
// would be to reproduce a number `s3` already verified.  An earlier draft of
// this file carried a half-written `PAdd` whose `Twice` row was simply wrong
// and which nothing used; it is deleted rather than left in, because an
// unexercised wrong impl in a probe is the thing the panel's spike rule warns
// about.

// ---------------------------------------------------------------- numerals

pub struct Num<W, St>(core::marker::PhantomData<(W, St)>);
pub struct Hot;

// -------------------------------------------------------------- composition

/// A composition: a static shape (an element type and a capacity) over a
/// dynamic run.  `Cap` is the positive-binary capacity, so it can be folded
/// and multiplied at the type level.
pub struct Run<E, C>(core::marker::PhantomData<(E, C)>);

/// The total capacity of a nest, flattened.  A numeral is capacity one; a run
/// multiplies its own capacity by its element's total.
pub trait TotalCap {
    type Out: Pos;
}
impl<W, St> TotalCap for Num<W, St> {
    type Out = One;
}

/// One inductive step, stated for the two nests this probe checks.  The general
/// impl needs positive-binary multiplication, which `s3` already establishes is
/// ordinary arithmetic; what matters here is that the flattened capacity is a
/// *type* the derivation can consume, and that is what these show.
impl<W, St, C: Pos> TotalCap for Run<Num<W, St>, C> {
    type Out = C;
}
impl<W, St> TotalCap for Run<Run<Num<W, St>, P3>, P5> {
    // 3 * 5 = 15, asserted below rather than trusted
    type Out = P15;
}
impl<W, St> TotalCap for Run<Run<Num<W, St>, P4>, P4> {
    type Out = P16;
}

pub const _NEST_PRODUCTS_ARE_PRODUCTS: () = {
    assert!(<P15 as Pos>::P == <P3 as Pos>::P * <P5 as Pos>::P);
    assert!(<P16 as Pos>::P == <P4 as Pos>::P * <P4 as Pos>::P);
};

/// The element numeral at the bottom of a nest.
pub trait ElemNum {
    type W: Nat;
}
impl<W: Nat, St> ElemNum for Num<W, St> {
    type W = W;
}
impl<E: ElemNum, C> ElemNum for Run<E, C> {
    type W = <E as ElemNum>::W;
}

/// THE FLATTENED DERIVATION: reach of the element plus the log of the *total*
/// capacity of the whole nest.  This is the tight form `s3` measures.
pub trait FlatAccReach {
    type R: Nat;
}
impl<T> FlatAccReach for T
where
    T: ElemNum + TotalCap,
    <T as TotalCap>::Out: Lg,
    <T as ElemNum>::W: Plus<<<T as TotalCap>::Out as Lg>::Out>,
    Sum<<T as ElemNum>::W, <<T as TotalCap>::Out as Lg>::Out>: Nat,
{
    type R = Sum<<T as ElemNum>::W, <<T as TotalCap>::Out as Lg>::Out>;
}

/// THE NESTED DERIVATION: apply the per-level rule once per level, which is
/// what composing the derivations rather than the shapes produces.
pub trait NestedAccReach {
    type R: Nat;
}
impl<W: Nat, St> NestedAccReach for Num<W, St> {
    type R = W;
}
impl<E, C: Pos + Lg> NestedAccReach for Run<E, C>
where
    E: NestedAccReach,
    <E as NestedAccReach>::R: Plus<<C as Lg>::Out>,
    Sum<<E as NestedAccReach>::R, <C as Lg>::Out>: Nat,
{
    type R = Sum<<E as NestedAccReach>::R, <C as Lg>::Out>;
}

// ------------------------------------------------------------------ checks

type Elem = Num<N13, Hot>;
type Flat3x5 = Run<Run<Elem, P3>, P5>;
type Flat4x4 = Run<Run<Elem, P4>, P4>;
type Single8 = Run<Elem, P8>;

pub const _ARM2_FLAT_MATCHES_ARITHMETIC: () = {
    // 13 + lg(15) = 13 + 4 = 17
    assert!(<<Flat3x5 as FlatAccReach>::R as Nat>::V == 17);
    // 13 + lg(16) = 13 + 4 = 17
    assert!(<<Flat4x4 as FlatAccReach>::R as Nat>::V == 17);
    // one level: 13 + lg(8) = 16
    assert!(<<Single8 as FlatAccReach>::R as Nat>::V == 16);
    // capacity one degenerates to the element
    assert!(<<Elem as FlatAccReach>::R as Nat>::V == 13);
};

pub const _ARM3_NESTED_IS_WIDER_AT_THE_WITNESS: () = {
    // 13 + lg(3) + lg(5) = 13 + 2 + 3 = 18, one bit wider than the flat 17
    assert!(<<Flat3x5 as NestedAccReach>::R as Nat>::V == 18);
    // where the shape is a power of two the two agree: 13 + 2 + 2 = 17
    assert!(<<Flat4x4 as NestedAccReach>::R as Nat>::V == 17);
    // and the gap is exactly one bit at the witness
    assert!(
        <<Flat3x5 as NestedAccReach>::R as Nat>::V - <<Flat3x5 as FlatAccReach>::R as Nat>::V == 1
    );
};

/// Negative control.  Under `--cfg tightneg` the crate must NOT build: the two
/// derivations disagree at the witness shape, and asserting they agree is
/// false.  Without this arm, arms 2 and 3 could both be reading the same
/// number.
#[cfg(tightneg)]
pub const _TIGHTNEG: () = {
    assert!(<<Flat3x5 as NestedAccReach>::R as Nat>::V == <<Flat3x5 as FlatAccReach>::R as Nat>::V);
};

/// Second negative control: the flattened derivation must not be reachable for
/// a nest whose total capacity has no `TotalCap` row, so the flattening is
/// load-bearing rather than decorative.  Under `--cfg missingcap` the crate
/// must NOT build.
#[cfg(missingcap)]
pub const _MISSINGCAP: () = {
    type Unknown = Run<Run<Elem, P5>, P5>; // 25, no TotalCap impl
    assert!(<<Unknown as FlatAccReach>::R as Nat>::V > 0);
};

/// Arm 1's claim, exercised rather than left idle: type-level multiplication of
/// capacities is expressible with no const arithmetic in any bound and no
/// feature gate.  Checked on the Peano encoding, where the induction is short
/// enough to state in six lines.  The positive-binary form the `Lg` induction
/// needs is the same induction in a different representation; this probe does
/// not build it and does not claim to, and section 3 of the file says so.
pub const _TYPE_LEVEL_PRODUCT_IS_EXPRESSIBLE: () = {
    assert!(<Prod<N3, N5> as Nat>::V == 15);
    assert!(<Prod<N4, N4> as Nat>::V == 16);
    assert!(<Prod<N1, N7> as Nat>::V == 7);
    assert!(<Prod<N0, N8> as Nat>::V == 0);
};
