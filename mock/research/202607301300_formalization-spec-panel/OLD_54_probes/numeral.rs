//! The `Specials`-carrying, radix-parameterised type-level numeral.
//!
//! Included at a probe's crate root beside `bias`, both by `#[path]`, so that this
//! module's own `crate::bias::...` paths resolve without a directory dance:
//!
//! ```ignore
//! #[path = "vu_bias_sealed_adj.rs"] pub mod bias;
//! #[path = "numeral.rs"]            pub mod numeral;
//! ```
//!
//! Three things here are new against `50_probes/probe_3_exponent_as_type.rs`, and each is
//! built sealed and attacked at declaration time per the carrier-at-birth rule
//! (`49:74-87`) rather than after a later pass finds the hole.
//!
//! 1. `Radix` is one constructor over the sealed `Pos`, bounded on a sealed `AtLeastTwo`
//!    predicate. The ratified table spells `type Radix: Radix` with the trait open, which
//!    admits `R = 1` and `R = 0`, both of which falsify the union-of-grids statement the
//!    whole float model rests on. One constructor plus a two-impl predicate keeps every
//!    radix expressible (16 for IBM hex float, 3, 2^k) while making the two broken ones
//!    unspellable.
//! 2. `Specials` is a product of two independent facts, not a chain of three. `INF` and
//!    `NAN` vary independently and three of the four corners have shipping witnesses.
//! 3. `Underflow` loses `FlushToZero`, which file 50 section 5.2 showed is a `Quantisation`
//!    resolution rather than a fact about what is representable.
//!
//! The exponent machinery (`EZero | EPos<P> | ENeg<P>`, `ESum`, `NegE`, `SignedDiff`) is
//! file 50's, copied rather than reinvented, with `Implicit`'s own single exponent moved
//! from a const to a type (section 4 of file 54 and `probe_4b`).

#![allow(dead_code)]

use crate::bias::nat::{AsPos, Cmp, Eq3, Gt, Lt, NSub, Nat, Pos, Pz, H, I, O};
use crate::bias::{Bias, PAdd, C0};
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// the signed exponent, sealed at birth (file 50, probe 3, unmodified in substance)
// ---------------------------------------------------------------------------

mod exp_sealed {
    pub trait ExponentSealed {}
}

pub trait Exponent: exp_sealed::ExponentSealed {
    const VAL: i64;
}

pub struct EZero;
pub struct EPos<P>(PhantomData<P>);
pub struct ENeg<P>(PhantomData<P>);

impl exp_sealed::ExponentSealed for EZero {}
impl<P: Pos> exp_sealed::ExponentSealed for EPos<P> {}
impl<P: Pos> exp_sealed::ExponentSealed for ENeg<P> {}

impl Exponent for EZero {
    const VAL: i64 = 0;
}
impl<P: Pos> Exponent for EPos<P> {
    const VAL: i64 = P::VAL as i64;
}
impl<P: Pos> Exponent for ENeg<P> {
    const VAL: i64 = -(P::VAL as i64);
}

pub trait SignedDiff<A, B> {
    type Out: Exponent;
}
impl<A: Pos, B: Pos> SignedDiff<A, B> for Eq3 {
    type Out = EZero;
}
impl<A: Pos, B: Pos> SignedDiff<A, B> for Gt
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: AsPos,
{
    type Out = EPos<<<Pz<A> as NSub<Pz<B>>>::Out as AsPos>::Out>;
}
impl<A: Pos, B: Pos> SignedDiff<A, B> for Lt
where
    Pz<B>: NSub<Pz<A>>,
    <Pz<B> as NSub<Pz<A>>>::Out: AsPos,
{
    type Out = ENeg<<<Pz<B> as NSub<Pz<A>>>::Out as AsPos>::Out>;
}

pub trait NegE {
    type Out: Exponent;
}
impl NegE for EZero {
    type Out = EZero;
}
impl<P: Pos> NegE for EPos<P> {
    type Out = ENeg<P>;
}
impl<P: Pos> NegE for ENeg<P> {
    type Out = EPos<P>;
}

pub trait ESum<Rhs> {
    type Out: Exponent;
}
impl ESum<EZero> for EZero {
    type Out = EZero;
}
impl<B: Pos> ESum<EPos<B>> for EZero {
    type Out = EPos<B>;
}
impl<B: Pos> ESum<ENeg<B>> for EZero {
    type Out = ENeg<B>;
}
impl<A: Pos> ESum<EZero> for EPos<A> {
    type Out = EPos<A>;
}
impl<A: Pos> ESum<EZero> for ENeg<A> {
    type Out = ENeg<A>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> ESum<EPos<B>> for EPos<A> {
    type Out = EPos<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> ESum<ENeg<B>> for ENeg<A> {
    type Out = ENeg<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + Cmp<B>, B: Pos> ESum<ENeg<B>> for EPos<A>
where
    <A as Cmp<B>>::Out: SignedDiff<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as SignedDiff<A, B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> ESum<EPos<B>> for ENeg<A>
where
    <A as Cmp<B>>::Out: SignedDiff<A, B>,
    <<A as Cmp<B>>::Out as SignedDiff<A, B>>::Out: NegE,
{
    type Out = <<<A as Cmp<B>>::Out as SignedDiff<A, B>>::Out as NegE>::Out;
}

pub trait NAdd<Rhs> {
    type Out: Nat;
}
impl<B: Nat> NAdd<B> for crate::bias::nat::Z {
    type Out = B;
}
impl<A: Pos> NAdd<crate::bias::nat::Z> for Pz<A> {
    type Out = Pz<A>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> NAdd<Pz<B>> for Pz<A> {
    type Out = Pz<<A as PAdd<B, C0>>::Out>;
}

// ---------------------------------------------------------------------------
// Radix: one constructor over the sealed `Pos`, bounded on a sealed `AtLeastTwo`
// ---------------------------------------------------------------------------
//
// The design wants two things that look opposed: every radix expressible (the ratified
// table's own "2 and 10 instantiated; any r expressible", `49:110`), and a guarantee that
// the exponent function `radix^e` generates a strictly refining chain of grids. A finite
// constructor seal cannot serve the first. An open trait cannot serve the second: `R = 1`
// makes every grid identical, so the union of grids is one grid and the exponent carries no
// information; `R = 0` makes the quantum zero. Both compile fine against an open trait with
// a `const R: u64`.
//
// The resolution is the shape `Bias` already uses. One constructor family over the sealed
// `Pos`, so the inhabitant set is infinite but generated, and a sealed predicate on the
// constructor head carrying the well-formedness the trait itself cannot state.

mod radix_sealed {
    pub trait RadixSealed {}
    pub trait AtLeastTwoSealed {}
}

/// A `Pos` of at least two. Two impls, constructor-headed, exhaustive by construction:
/// `Pos ::= H | O<P> | I<P>` with `H = 1`, `O<P> = 2P >= 2`, `I<P> = 2P+1 >= 3`. `H` has
/// no impl, so radix one is unspellable, and radix zero has no `Pos` spelling at all.
pub trait AtLeastTwo: Pos + radix_sealed::AtLeastTwoSealed {}
impl<P: Pos> radix_sealed::AtLeastTwoSealed for O<P> {}
impl<P: Pos> radix_sealed::AtLeastTwoSealed for I<P> {}
impl<P: Pos> AtLeastTwo for O<P> {}
impl<P: Pos> AtLeastTwo for I<P> {}

pub trait Radix: radix_sealed::RadixSealed {
    type Digits: AtLeastTwo;
    const R: u64;
}

/// The only `Radix` constructor.
pub struct Rad<P>(PhantomData<P>);

impl<P: AtLeastTwo> radix_sealed::RadixSealed for Rad<P> {}
impl<P: AtLeastTwo> Radix for Rad<P> {
    type Digits = P;
    const R: u64 = P::VAL;
}

// Pos literals used below. H = 1, O<P> = 2P, I<P> = 2P+1, read from the low bit up.
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<P2>;
pub type P5 = I<P2>;
pub type P7 = I<P3>;
pub type P8 = O<P4>;
pub type P10 = O<P5>;
pub type P16 = O<P8>;
pub type P24 = O<O<O<P3>>>;

pub type Two = Rad<P2>;
pub type Ten = Rad<P10>;
/// IBM hex float, in the design's own vocabulary, at no cost: a third `Radix` instance.
pub type Sixteen = Rad<P16>;

// ---------------------------------------------------------------------------
// Specials: a product of two facts, sealed, four instances
// ---------------------------------------------------------------------------
//
// File 50 section 5.2 proposes three instances as a chain: none, infinities-only, IEEE.
// The chain reading is wrong, and the middle rung is the one with no witness. Infinity
// presence and NaN presence vary independently in shipping formats, and the corner the
// chain cannot name is the one that is actually deployed: OCP OFP8's `E4M3` carries NaN
// and no infinity, spending the freed exponent code to gain a binade of range, while its
// sibling `E5M2` carries both.
//
// So `Specials` is the four-point product lattice `{INF} x {NAN}`, and the top three
// corners this file can name a witness for are `NoSpecials` (every fixed-point numeral),
// `NanOnly` (E4M3) and `IeeeSpecials` (binary32, binary64, decimal64, E5M2). `InfOnly` is
// declared, costs nothing, and is grounded `unknown` rather than justified by a plausible
// sentence, which is exactly what file 53 asked for and what the review has struck a claim
// for lacking before.
//
// Signalling NaN is deliberately not an axis here, following file 50: reading one is an
// operation and the design's grade already carries what an operation raises. The quiet
// and signalling data are an `Encoding::Fields` reserved-code distinction.

mod specials_sealed {
    pub trait SpecialsSealed {}
}

pub trait Specials: specials_sealed::SpecialsSealed {
    const INF: bool;
    const NAN: bool;
    /// Present iff `NAN`, and the count is a datum fact the encoding fixes. Declared here
    /// only so the crossing check can be driven from the numeral; the real home is
    /// `Encoding::Fields`.
    const NAN_DATA_MIN: u32;
}

pub struct NoSpecials;
pub struct InfOnly;
pub struct NanOnly;
pub struct IeeeSpecials;

impl specials_sealed::SpecialsSealed for NoSpecials {}
impl specials_sealed::SpecialsSealed for InfOnly {}
impl specials_sealed::SpecialsSealed for NanOnly {}
impl specials_sealed::SpecialsSealed for IeeeSpecials {}

impl Specials for NoSpecials {
    const INF: bool = false;
    const NAN: bool = false;
    const NAN_DATA_MIN: u32 = 0;
}
impl Specials for InfOnly {
    const INF: bool = true;
    const NAN: bool = false;
    const NAN_DATA_MIN: u32 = 0;
}
impl Specials for NanOnly {
    const INF: bool = false;
    const NAN: bool = true;
    const NAN_DATA_MIN: u32 = 1;
}
impl Specials for IeeeSpecials {
    const INF: bool = true;
    const NAN: bool = true;
    const NAN_DATA_MIN: u32 = 1;
}

// ---------------------------------------------------------------------------
// Underflow: two instances, sealed. Flush-to-zero has left (file 50, 5.2).
// ---------------------------------------------------------------------------

mod uf_sealed {
    pub trait UnderflowSealed {}
}

pub trait Underflow: uf_sealed::UnderflowSealed {
    const GRADUAL: bool;
}
pub struct Gradual;
pub struct Abrupt;
impl uf_sealed::UnderflowSealed for Gradual {}
impl uf_sealed::UnderflowSealed for Abrupt {}
impl Underflow for Gradual {
    const GRADUAL: bool = true;
}
impl Underflow for Abrupt {
    const GRADUAL: bool = false;
}

// ---------------------------------------------------------------------------
// SignDomain: a value fact (49:114), sealed.
// ---------------------------------------------------------------------------

mod dom_sealed {
    pub trait SignDomainSealed {}
}
pub trait SignDomain: dom_sealed::SignDomainSealed {
    const SIGNED: bool;
}
pub struct NonNegative;
pub struct Symmetric;
pub struct AsymmetricLow;
impl dom_sealed::SignDomainSealed for NonNegative {}
impl dom_sealed::SignDomainSealed for Symmetric {}
impl dom_sealed::SignDomainSealed for AsymmetricLow {}
impl SignDomain for NonNegative {
    const SIGNED: bool = false;
}
impl SignDomain for Symmetric {
    const SIGNED: bool = true;
}
impl SignDomain for AsymmetricLow {
    const SIGNED: bool = true;
}

// ---------------------------------------------------------------------------
// The exponent form, and the numeral
// ---------------------------------------------------------------------------

pub trait ExponentForm {
    const EMIN: i64;
    const EMAX: i64;
    const RANGED: bool;
    const GRADUAL: bool;
    const INF: bool;
    const NAN: bool;
}

/// One grid. `E` is a TYPE, not a const, for the reason `probe_4b` compiles: `mulnum` over
/// two `Implicit` numerals adds the exponents and the sum appears in the result type.
pub struct Implicit<E, A, B>(PhantomData<(E, A, B)>);

/// A family of grids indexed by an exponent interval, with a policy for the bottom and a
/// `Specials` set for what sits outside the finite range.
pub struct Ranged<EMIN, EMAX, U, S>(PhantomData<(EMIN, EMAX, U, S)>);

impl<E: Exponent, A, B: Bias> ExponentForm for Implicit<E, A, B> {
    const EMIN: i64 = E::VAL;
    const EMAX: i64 = E::VAL;
    const RANGED: bool = false;
    const GRADUAL: bool = false;
    const INF: bool = false;
    const NAN: bool = false;
}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
    const EMIN: i64 = EMIN::VAL;
    const EMAX: i64 = EMAX::VAL;
    const RANGED: bool = true;
    const GRADUAL: bool = U::GRADUAL;
    const INF: bool = S::INF;
    const NAN: bool = S::NAN;
}

pub trait Numeral {
    type Radix: Radix;
    type Precision: Nat;
    type Exponent: ExponentForm;
    type Domain: SignDomain;

    const R: u64;
    const P: u64;
    const EMIN: i64;
    const EMAX: i64;
    const INF: bool;
    const NAN: bool;
    const GRADUAL: bool;
    const SIGNED: bool;
}

/// A ranged numeral: the float shape, radix-parameterised.
pub struct Fl<R, P, EMIN, EMAX, U, S, D>(PhantomData<(R, P, EMIN, EMAX, U, S, D)>);

impl<R, P, EMIN, EMAX, U, S, D> Numeral for Fl<R, P, EMIN, EMAX, U, S, D>
where
    R: Radix,
    P: Pos,
    EMIN: Exponent,
    EMAX: Exponent,
    U: Underflow,
    S: Specials,
    D: SignDomain,
{
    type Radix = R;
    type Precision = Pz<P>;
    type Exponent = Ranged<EMIN, EMAX, U, S>;
    type Domain = D;

    const R: u64 = R::R;
    const P: u64 = P::VAL;
    const EMIN: i64 = EMIN::VAL;
    const EMAX: i64 = EMAX::VAL;
    const INF: bool = S::INF;
    const NAN: bool = S::NAN;
    const GRADUAL: bool = U::GRADUAL;
    const SIGNED: bool = D::SIGNED;
}

/// The adjustment half of `mulnum` over `Implicit` numerals, declared as a projection and
/// implemented at concrete adjustments only.
///
/// This is the projection-chain constraint (`49:306-324`) obeyed rather than rediscovered:
/// the product of two reduced rationals is `Reduce` applied to a product, and naming
/// `Reduce` as a bound in a chain that reaches `MulNum`'s signature is the compiled
/// divergence file 41 found and file 48 re-found in a consumer combinator. So the generic
/// impl names this trait, and the trait's impls are written where the operands are
/// concrete. The three-way gcd behind a real instance is file 42's and is not re-derived
/// here; probe 4's instances are the dyadic ones, where the product is immediate.
pub trait AdjProduct<Rhs> {
    type Out;
}

/// A fixed numeral: one grid, exponent as a type, adjustment and bias nested.
pub struct Fx<R, P, E, A, B, D>(PhantomData<(R, P, E, A, B, D)>);

impl<R, P, E, A, B, D> Numeral for Fx<R, P, E, A, B, D>
where
    R: Radix,
    P: Pos,
    E: Exponent,
    A: crate::bias::nat::Adjustment,
    B: Bias,
    D: SignDomain,
{
    type Radix = R;
    type Precision = Pz<P>;
    type Exponent = Implicit<E, A, B>;
    type Domain = D;

    const R: u64 = R::R;
    const P: u64 = P::VAL;
    const EMIN: i64 = E::VAL;
    const EMAX: i64 = E::VAL;
    const INF: bool = false;
    const NAN: bool = false;
    const GRADUAL: bool = false;
    const SIGNED: bool = D::SIGNED;
}

// ---------------------------------------------------------------------------
// mulnum: the exact-widening map, over both exponent forms, at equal radix
// ---------------------------------------------------------------------------
//
// The radix appears once, as one shared parameter across both operands and the result.
// Two numerals of different radix have no `MulNum` impl at all, which is the correct
// answer: the exact product of a binary and a decimal value is neither, it lands on a
// third numeral whose adjustment carries a 5-power denominator (probe 6).

pub trait MulNum<Rhs> {
    type Out: Numeral;
}

impl<R, P1, E1N, E1X, U, D, P2, E2N, E2X> MulNum<Fl<R, P2, E2N, E2X, U, NoSpecials, D>>
    for Fl<R, P1, E1N, E1X, U, NoSpecials, D>
where
    R: Radix,
    P1: Pos + PAdd<P2, C0>,
    P2: Pos,
    E1N: Exponent + ESum<E2N>,
    E1X: Exponent + ESum<E2X>,
    E2N: Exponent,
    E2X: Exponent,
    U: Underflow,
    D: SignDomain,
{
    type Out = Fl<
        R,
        <P1 as PAdd<P2, C0>>::Out,
        <E1N as ESum<E2N>>::Out,
        <E1X as ESum<E2X>>::Out,
        U,
        NoSpecials,
        D,
    >;
}

/// The `Implicit` half, which file 50 left as an honest carve-out: whether the single
/// exponent has to move to a type at the same time as the ranged bounds. It does, and for
/// the identical reason. `E1 + E2` is computed and appears in the result numeral's type.
impl<R, P1, E1, A1, B1, D, P2, E2, A2, B2> MulNum<Fx<R, P2, E2, A2, B2, D>>
    for Fx<R, P1, E1, A1, B1, D>
where
    R: Radix,
    P1: Pos + PAdd<P2, C0>,
    P2: Pos,
    E1: Exponent + ESum<E2>,
    E2: Exponent,
    A1: crate::bias::nat::Adjustment + AdjProduct<A2>,
    A2: crate::bias::nat::Adjustment,
    <A1 as AdjProduct<A2>>::Out: crate::bias::nat::Adjustment,
    B1: Bias + crate::bias::BiasProduct<B2>,
    B2: Bias,
    D: SignDomain,
{
    type Out = Fx<
        R,
        <P1 as PAdd<P2, C0>>::Out,
        <E1 as ESum<E2>>::Out,
        <A1 as AdjProduct<A2>>::Out,
        <B1 as crate::bias::BiasProduct<B2>>::Out,
        D,
    >;
}
