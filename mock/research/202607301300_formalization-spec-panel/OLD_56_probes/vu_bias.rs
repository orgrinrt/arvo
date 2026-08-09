//! vu_bias_sealed.rs, retargeted at vu_nat_sealed_adj.rs (the Adjustment-sealed
//! tower). No other change.

//! The signed rational bias, built on `vu_nat.rs` rather than beside it.
//!
//! Op's checkpoint (`39b`) holds the value-unique encoding pending the bias
//! repair, and the third consolidation's section 1.11 states the repair as
//! costing no new mechanism: `Bias` becomes a signed, gcd-normalised
//! rational, the same normal form as `Adjustment`, composed rather than
//! invented. This file is that composition.
//!
//! One design fork happened only after it was run rather than argued.
//! `repro_isolate.rs` and `repro_unbounded.rs` (this directory) pin it: a
//! GENERIC TRAIT whose associated type, or whose where-clause, mentions
//! `Reduce` for an unconstrained-shape type parameter does not compile,
//! `E0275: overflow evaluating the requirement Pz<O<_>>: ExactDivOdd<_>`,
//! regardless of whether the associated type is bound, unbound, a raw
//! projection, or a named alias, and regardless of whether anything ever
//! calls it. `Gcd` (the bound `Adjustment` already uses, `N: Gcd<D, Out =
//! H>`) does not have this problem; `PMul` (probe 6's magnitude
//! multiplication, composed generically inside `IMul`'s own associated
//! types) does not either. The distinguishing fact, run down with
//! `repro_bare_alias_struct.rs`: a BARE top-level type alias referencing
//! `Reduce` for a fully generic `N, D` compiles and stays lazy, checked only
//! where instantiated; the same reference inside a trait impl's own
//! associated-type body or where-clause is checked eagerly, at the impl's
//! own definition, independent of any call site. `Reduce`'s own machinery
//! (`Strip2`, `ExactDivOdd`) is defined over `Nat`'s `Pz<P>` wrapper, and an
//! abstract `Pz<X>` unifies against the `Pz<O<P>>` pattern by inventing a
//! fresh `P` for `X`, which the solver can repeat without a base case;
//! `Gcd`/`PMul` are defined directly on `Pos`'s three exhaustive
//! constructors with no such wrapper position to unify a fresh variable
//! into, so they do not admit the same speculative descent.
//!
//! The consequence for the design, not only for this file: a generic
//! trait-level composition of `Reduce` with anything else (a hypothetical
//! generic `Adjustment` multiplication, not only `Bias`'s) hits the same
//! wall. `Reduce` composes safely only as a bare alias, evaluated at a
//! concrete numeral pair, which is exactly how every other file in this
//! review already uses it (probe 4's own CLAIM D/E: "a type-level multiply
//! ... is named in the file as the one piece this probe assumes rather than
//! builds"; file 31's closure formula and every WorkUnit-level composition
//! in the design names concrete operand numerals, never an abstract pair).
//! So the fix is not a workaround local to `Bias`; it is the shape the whole
//! design already uses, made explicit here because Bias's multiplication is
//! the first place anyone tried to compose `Reduce` generically and found
//! out it refuses.
//!
//! Magnitude multiplication (`PMul`/`PAdd`/`Succ`) does not exist in
//! `vu_nat.rs`; file 36 never needed it there because reduction only
//! divides. Ported here from `36_probes/probe_6...rs`'s own copy, retargeted
//! at `nat::H`/`nat::O`/`nat::I` (file 36's sealed encoding) rather than at
//! probe 6's own unsealed local copy of the same three constructors, so the
//! magnitude side of a `Bias` inherits the one seal that actually matters.
//!
//! Included with `#[path = "vu_bias.rs"] mod bias;`.

#![allow(dead_code)]

use core::marker::PhantomData;

#[path = "vu_nat.rs"]
pub mod nat;

use nat::{Gcd, Pos, Ratio, Reduce};
use nat::{H, I, O};

// --- magnitude multiplication, ported from 36_probes/probe_6, retargeted at
// --- nat's sealed H/O/I rather than probe 6's own local unsealed copy. ---

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

// --- the bias itself: zero, or a sign applied to a reduced positive ---
// --- rational magnitude. Sealed, exactly as Pos and Nat are sealed, and ---
// --- for the same reason: the induction that makes the three variants ---
// --- exhaustive has a hypothesis (that those are the only impls) which a ---
// --- formula cannot supply and only a closed perimeter can. ---
//
// The magnitude bound is written directly against the reduction condition
// (`N: Pos + Gcd<D, Out = H>, D: Pos`) rather than against the abstract
// `Adjustment` trait. Probe 3 is why: `Adjustment` is a public trait with no
// seal of its own, so a foreign type could implement it with an unreduced or
// outright fabricated (NUM, DEN) pair, and a `Bias` bound on `Adjustment`
// would inherit that hole. Bounding on the concrete condition instead means
// `Bias`'s guarantee rests only on `Pos`/`Nat`'s own seal, which probe 4 and
// probe 4b confirm holds against a genuinely separate downstream crate.

mod bias_sealed {
    pub trait BiasSealed {}
}

pub trait Bias: bias_sealed::BiasSealed {
    /// Signed numerator. Zero exactly when the type is `BZero`; `BPos`/
    /// `BNeg` can never produce it, because their magnitude is `N: Pos`,
    /// which excludes zero by the same induction as everywhere else in this
    /// encoding.
    const NUM: i64;
    /// Always positive; the sign lives entirely in `NUM` and in which of the
    /// three variants a value inhabits.
    const DEN: u64;
}

pub struct BZero;
pub struct BPos<N, D>(PhantomData<(N, D)>);
pub struct BNeg<N, D>(PhantomData<(N, D)>);

impl bias_sealed::BiasSealed for BZero {}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> bias_sealed::BiasSealed for BPos<N, D> {}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> bias_sealed::BiasSealed for BNeg<N, D> {}

impl Bias for BZero {
    const NUM: i64 = 0;
    const DEN: u64 = 1;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BPos<N, D> {
    const NUM: i64 = N::VAL as i64;
    const DEN: u64 = D::VAL;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BNeg<N, D> {
    const NUM: i64 = -(N::VAL as i64);
    const DEN: u64 = D::VAL;
}

/// The consumer-facing spelling: normalise at the naming site, the same
/// discipline `Reduced<N, D>` uses for `Adjustment`, so two spellings of one
/// signed rational unify before anything asks whether they do. A bare type
/// alias, per the header note: this is the shape that stays lazy.
pub type ReducedBiasPos<N, D> = BPos<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;
pub type ReducedBiasNeg<N, D> = BNeg<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;

// --- `bias = B1 * B2` (`31:399-400`), lifted from integer to signed- ---
// --- rational algebra. Magnitude: multiply numerator by numerator and ---
// --- denominator by denominator (`PMul`), then reduce, because the ---
// --- product of two coprime pairs need not stay coprime (2/3 * 3/4 = ---
// --- 6/12, not already reduced by inspection). Sign: the ordinary four- ---
// --- combination rule, structural, no `Reduce` touched. Both are bare ---
// --- aliases, per the header note: this is what lets them stay generic. ---

/// The reduced numerator and denominator of `(N1/D1) * (N2/D2)`, magnitude
/// only, sign-agnostic. Composed of two mechanisms already built (`PMul`,
/// `Reduce`); no new arithmetic.
pub type BiasMagN<N1, D1, N2, D2> =
    <Ratio<<N1 as PMul<N2>>::Out, <D1 as PMul<D2>>::Out> as Reduce>::N;
pub type BiasMagD<N1, D1, N2, D2> =
    <Ratio<<N1 as PMul<N2>>::Out, <D1 as PMul<D2>>::Out> as Reduce>::D;

/// Positive times positive: positive.
pub type BiasMulPP<N1, D1, N2, D2> = BPos<BiasMagN<N1, D1, N2, D2>, BiasMagD<N1, D1, N2, D2>>;
/// Positive times negative, or negative times positive: negative.
pub type BiasMulPN<N1, D1, N2, D2> = BNeg<BiasMagN<N1, D1, N2, D2>, BiasMagD<N1, D1, N2, D2>>;
/// Negative times negative: positive.
pub type BiasMulNN<N1, D1, N2, D2> = BPos<BiasMagN<N1, D1, N2, D2>, BiasMagD<N1, D1, N2, D2>>;
// (BiasMulNP is the same type as BiasMulPN; sign multiplication is
// commutative and the type alias is named once, used from both call
// shapes. Multiplication by BZero is not a magnitude case at all: it is
// BZero, unconditionally, and is spelled directly rather than through an
// alias, because there is no magnitude to compute.)

/// The dispatch a consumer actually writes: pick the alias by which two
/// concrete `Bias` variants are being multiplied. A trait would be the
/// nicer surface (`b1.mul(b2)`, uniform across sign combinations), and
/// this file's header explains why one is not built: the trait shape is
/// exactly the one the toolchain refuses, and the refusal is definitional,
/// not something a different bound escapes.
pub trait BiasProduct<Rhs> {
    type Out: Bias;
}
impl<R: Bias> BiasProduct<R> for BZero {
    type Out = BZero;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> BiasProduct<BZero> for BPos<N, D> {
    type Out = BZero;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> BiasProduct<BZero> for BNeg<N, D> {
    type Out = BZero;
}
// The four sign-combination impls are deliberately NOT written here as a
// generic composition (that is exactly what does not compile, per the
// header note and probes 1b/2b). A consumer who needs the product of two
// concrete, non-zero-sign `Bias` values names the alias directly:
// `BiasMulPP<N1, D1, N2, D2>` and so on. Probe 2 does this.
