//! Probe 2: the naturals, value-unique by construction rather than by
//! discipline. Nothing normalises, because there is nothing to normalise:
//! the non-canonical spellings probe 1 exhibits have no type at all under
//! this encoding, and every operation below is closed on it structurally.
//!
//! The encoding is the standard constructive-mathematics one (Coq's
//! `positive` and `N`, Barras et al.; also GHC's `Numeric.Natural` binary
//! form and Agda's `Bin`), which is prior art in type theory but not what
//! `typenum` or the design's current width chain uses:
//!
//!     Pos ::= H          -- 1
//!           | O<P>       -- 2p,   P: Pos, so >= 2
//!           | I<P>       -- 2p+1, P: Pos, so >= 3
//!     Nat ::= Z          -- 0
//!           | Pz<P>      -- P: Pos, so >= 1
//!
//! The leading digit is the terminator `H`, and it is a one. Under
//! `UTerm`/`UInt` the terminator is a zero-length chain and any number of
//! zero digits may precede the leading one, which is exactly the freedom
//! probe 1 measures.
//!
//! UNIQUENESS, by induction on the value. 1 is `H` and nothing else, since
//! `O<P>` needs `P: Pos` hence `val >= 2`, and `I<P>` likewise `val >= 3`.
//! An even `n >= 2` is `O<P>` with `val(P) = n/2 >= 1`, and by induction `P`
//! is unique. An odd `n >= 3` is `I<P>` with `val(P) = (n-1)/2 >= 1`,
//! likewise. Zero is `Z` and nothing else, since `Pz<P>` needs `P: Pos`.
//! So the map from `Pos`-inhabiting types to positive integers is a
//! bijection, and there is no `Trim` to write.
//!
//! The perimeter, stated as `what-you-can-observe-is-what-you-guaranteed.md`
//! asks it: the observation surface is the trait `Pos`, which is what every
//! generic position bounds on. `O<Z>` is a well-formed *type* and is not a
//! `Pos`, so it cannot reach any position the design cares about. That
//! refusal is `probe_2b`, committed refusing.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2_value_unique_naturals_by_construction.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

// --- the encoding ---

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

// --- the smart constructors: `2n` and `2n+1` on Nat, which is where the ---
// --- normalisation typenum spends a `Trim` on is absorbed, at O(1) per   ---
// --- digit and by impl selection rather than by a traversal.             ---

/// `Dbl<N> = 2N`. The whole content of typenum's `TrimTrailingZeros` is the
/// first line: doubling zero is zero, not a zero digit on an empty chain.
pub trait Dbl {
    type Out: Nat;
}
impl Dbl for Z {
    type Out = Z;
}
impl<P: Pos> Dbl for Pz<P> {
    type Out = Pz<O<P>>;
}

/// `DblInc<N> = 2N + 1`.
pub trait DblInc {
    type Out: Nat;
}
impl DblInc for Z {
    type Out = Pz<H>;
}
impl<P: Pos> DblInc for Pz<P> {
    type Out = Pz<I<P>>;
}

// --- successor on Pos (Coq's `Pos.succ`) ---

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

// --- addition on Pos with a carry bit. Eighteen impls, one per ---
// --- (constructor, constructor, carry). No output is ever zero, so no ---
// --- case needs a smart constructor and none needs a repair pass. ---

pub struct C0;
pub struct C1;

pub trait PAdd<Rhs, C> {
    type Out: Pos;
}

// H + H
impl PAdd<H, C0> for H {
    type Out = O<H>;
}
impl PAdd<H, C1> for H {
    type Out = I<H>;
}
// H + O b
impl<B: Pos> PAdd<O<B>, C0> for H {
    type Out = I<B>;
}
impl<B: Pos + Succ> PAdd<O<B>, C1> for H {
    type Out = O<<B as Succ>::Out>;
}
// H + I b
impl<B: Pos + Succ> PAdd<I<B>, C0> for H {
    type Out = O<<B as Succ>::Out>;
}
impl<B: Pos + Succ> PAdd<I<B>, C1> for H {
    type Out = I<<B as Succ>::Out>;
}
// O a + H
impl<A: Pos> PAdd<H, C0> for O<A> {
    type Out = I<A>;
}
impl<A: Pos + Succ> PAdd<H, C1> for O<A> {
    type Out = O<<A as Succ>::Out>;
}
// I a + H
impl<A: Pos + Succ> PAdd<H, C0> for I<A> {
    type Out = O<<A as Succ>::Out>;
}
impl<A: Pos + Succ> PAdd<H, C1> for I<A> {
    type Out = I<<A as Succ>::Out>;
}
// O a + O b
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for O<A> {
    type Out = O<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C1> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
// O a + I b
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<I<B>, C0> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for O<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
// I a + O b
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for I<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<O<B>, C1> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
// I a + I b
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C0> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for I<A> {
    type Out = I<<A as PAdd<B, C1>>::Out>;
}

/// Addition on `Nat`, which is the width-adder position: `mul_full`'s
/// product width. `Z` is absorbing on both sides, structurally.
pub trait NAdd<Rhs> {
    type Out: Nat;
}
impl NAdd<Z> for Z {
    type Out = Z;
}
impl<B: Pos> NAdd<Pz<B>> for Z {
    type Out = Pz<B>;
}
impl<A: Pos> NAdd<Z> for Pz<A> {
    type Out = Pz<A>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> NAdd<Pz<B>> for Pz<A> {
    type Out = Pz<<A as PAdd<B, C0>>::Out>;
}

// --- CLAIM A: the values are right. ---

pub type N0 = Z;
pub type N1 = Pz<H>;
pub type N2 = Pz<O<H>>;
pub type N3 = Pz<I<H>>;
pub type N5 = Pz<I<O<H>>>;
pub type N7 = Pz<I<I<H>>>;
pub type N13 = Pz<I<O<I<H>>>>;
pub type N20 = Pz<O<O<I<O<H>>>>>;

const _: () = assert!(<N0 as Nat>::VAL == 0);
const _: () = assert!(<N1 as Nat>::VAL == 1);
const _: () = assert!(<N2 as Nat>::VAL == 2);
const _: () = assert!(<N3 as Nat>::VAL == 3);
const _: () = assert!(<N5 as Nat>::VAL == 5);
const _: () = assert!(<N7 as Nat>::VAL == 7);
const _: () = assert!(<N13 as Nat>::VAL == 13);
const _: () = assert!(<N20 as Nat>::VAL == 20);

// --- CLAIM B: the sums the multiplicative half actually needs. ---
// `26:252-256`: a `UFixed<13,3>` times `UFixed<7,2>` product needs 13+7=20
// and 3+2=5.

pub type Sum13_7 = <N13 as NAdd<N7>>::Out;
pub type Sum3_2 = <N3 as NAdd<N2>>::Out;
pub type Sum0_0 = <N0 as NAdd<N0>>::Out;
pub type Sum13_0 = <N13 as NAdd<N0>>::Out;

const _: () = assert!(<Sum13_7 as Nat>::VAL == 20);
const _: () = assert!(<Sum3_2 as Nat>::VAL == 5);
const _: () = assert!(<Sum0_0 as Nat>::VAL == 0);
const _: () = assert!(<Sum13_0 as Nat>::VAL == 13);

// --- CLAIM C: the sums are the SAME TYPES as the directly-written ---
// --- numerals, which is what probe 1b shows the old encoding cannot ---
// --- promise. A type-equality demand accepts them. ---

pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

pub fn sums_inhabit_their_own_numeral() {
    same_type(PhantomData::<Sum13_7>, PhantomData::<N20>);
    same_type(PhantomData::<Sum3_2>, PhantomData::<N5>);
    same_type(PhantomData::<Sum0_0>, PhantomData::<N0>);
    same_type(PhantomData::<Sum13_0>, PhantomData::<N13>);
}

// --- CLAIM D: associativity and commutativity of the width adder hold as ---
// --- TYPE identities, not only as value identities. This is the property ---
// --- file 34 section 2.6 needs and could only state for the rational ---
// --- adjustment; here it is for the widths themselves. ---

pub fn width_addition_is_associative_as_types() {
    same_type(
        PhantomData::<<<N13 as NAdd<N7>>::Out as NAdd<N5>>::Out>,
        PhantomData::<<N13 as NAdd<<N7 as NAdd<N5>>::Out>>::Out>,
    );
    same_type(
        PhantomData::<<<N3 as NAdd<N0>>::Out as NAdd<N2>>::Out>,
        PhantomData::<<N3 as NAdd<<N0 as NAdd<N2>>::Out>>::Out>,
    );
}

pub fn width_addition_is_commutative_as_types() {
    same_type(
        PhantomData::<<N13 as NAdd<N7>>::Out>,
        PhantomData::<<N7 as NAdd<N13>>::Out>,
    );
    same_type(
        PhantomData::<<N0 as NAdd<N13>>::Out>,
        PhantomData::<<N13 as NAdd<N0>>::Out>,
    );
}
