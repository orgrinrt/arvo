//! Probe 3: the exponent as a type, the `Ranged` numeral built, and `mulnum` compiled over
//! two model `Ranged` numerals. This is the compile the fourth consolidation's section 3
//! asks for as the second read on the exponent fork.
//!
//! rustc --edition 2021 --crate-type lib probe_3_exponent_as_type.rs
//!
//! Section 1.15 derives, reasoned and uncompiled, that the exponent bounds must become types
//! the moment the exact-widening family reaches a `Ranged` numeral, and that a signed
//! exponent lands on the constructor-sign shape `Bias` already uses rather than on `Int`.
//! Both halves are compiled here. The shape is `EZero | EPos<P> | ENeg<P>` over the sealed
//! `Pos`, sealed at birth per the carrier-at-birth rule rather than after three passes.
//!
//! Every trait in the chain that reaches `MulNum`'s signature pattern-matches on constructor
//! heads, per the projection-chain constraint of section 1.11. Nothing here names `Reduce`.

#![allow(dead_code)]

use core::marker::PhantomData;

#[path = "vu_bias_sealed_adj.rs"]
pub mod bias;

use bias::nat::{AsPos, Cmp, Eq3, Gt, Lt, NSub, Nat, Pos, Pz, H, I, O};
use bias::{Bias, PAdd, C0};

// ---------------------------------------------------------------------------
// the signed exponent, sealed at birth
// ---------------------------------------------------------------------------

mod exp_sealed {
    pub trait ExponentSealed {}
}

/// A signed exponent. Constructor-sign, exactly as `Bias` carries its sign, over the sealed
/// `Pos`. `Int` is not consumed, which is the argument section 1.15 owed the `Int` drop.
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

/// Difference of two positives, dispatched on their comparison. Keyed on the `Ord3` marker
/// so every impl is constructor-headed.
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

/// Negation. Three impls, constructor-headed.
///
/// This trait exists because the first version of the negative-plus-positive impl reused
/// `SignedDiff` with its arguments swapped, and the compiler refused it: `Cmp<7, 4> = Gt`
/// selects the branch computing `4 - 7`, which walks `NSub` off the bottom of `Nat` and
/// reports `the trait bound Z: Dec is not satisfied`. The natural subtraction refusing to go
/// negative is the tower working; the repair is to compute the magnitude difference once and
/// apply the sign afterward, which is the same separation `Bias` makes.
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

/// Signed addition on exponents. Nine impls, all constructor-headed.
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

// ---------------------------------------------------------------------------
// natural addition, for the precision chain
// ---------------------------------------------------------------------------

pub trait NAdd<Rhs> {
    type Out: Nat;
}
impl<B: Nat> NAdd<B> for bias::nat::Z {
    type Out = B;
}
impl<A: Pos> NAdd<bias::nat::Z> for Pz<A> {
    type Out = Pz<A>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> NAdd<Pz<B>> for Pz<A> {
    type Out = Pz<<A as PAdd<B, C0>>::Out>;
}

// ---------------------------------------------------------------------------
// the numeral, with the exponent form nested
// ---------------------------------------------------------------------------

pub trait Underflow {
    const NAME: &'static str;
}
pub struct Gradual;
pub struct FlushToZero;
pub struct Abrupt;
impl Underflow for Gradual {
    const NAME: &'static str = "gradual";
}
impl Underflow for FlushToZero {
    const NAME: &'static str = "flush-to-zero";
}
impl Underflow for Abrupt {
    const NAME: &'static str = "abrupt";
}

pub trait Specials {
    const PRESENT: bool;
}
pub struct NoSpecials;
pub struct IeeeSpecials;
impl Specials for NoSpecials {
    const PRESENT: bool = false;
}
impl Specials for IeeeSpecials {
    const PRESENT: bool = true;
}

pub trait ExponentForm {}

/// One grid. The exponent is a single constant, so the quantum is a constant.
pub struct Implicit<E, A, B>(PhantomData<(E, A, B)>);
/// A family of grids indexed by an exponent interval, with a policy for the bottom.
pub struct Ranged<EMIN, EMAX, U, S>(PhantomData<(EMIN, EMAX, U, S)>);

impl<E: Exponent, A, B: Bias> ExponentForm for Implicit<E, A, B> {}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
}

pub trait Numeral {
    type Precision: Nat;
    type Exponent: ExponentForm;
    const EMIN: i64;
    const EMAX: i64;
    const P: u64;
    const SPECIALS: bool;
}

pub struct Fl<P, EMIN, EMAX, U, S>(PhantomData<(P, EMIN, EMAX, U, S)>);

impl<P: Pos, EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> Numeral
    for Fl<P, EMIN, EMAX, U, S>
{
    type Precision = Pz<P>;
    type Exponent = Ranged<EMIN, EMAX, U, S>;
    const EMIN: i64 = EMIN::VAL;
    const EMAX: i64 = EMAX::VAL;
    const P: u64 = P::VAL;
    const SPECIALS: bool = S::PRESENT;
}

// ---------------------------------------------------------------------------
// mulnum over two Ranged numerals: the map the spine rule forces
// ---------------------------------------------------------------------------

/// The exact product numeral. Precision adds, exponent bounds add, and every one of those
/// three quantities is computed and then appears in a type, which is the spine rule's own
/// statement of why they cannot be consts.
///
/// Gated on `Specials = NoSpecials`, the gate `ExactWindow` already carries: an exact
/// widening family has no answer for infinity times zero.
pub trait MulNum<Rhs> {
    type Out: Numeral;
}

impl<P1, E1N, E1X, U, P2, E2N, E2X> MulNum<Fl<P2, E2N, E2X, U, NoSpecials>>
    for Fl<P1, E1N, E1X, U, NoSpecials>
where
    P1: Pos + PAdd<P2, C0>,
    P2: Pos,
    E1N: Exponent + ESum<E2N>,
    E1X: Exponent + ESum<E2X>,
    E2N: Exponent,
    E2X: Exponent,
    U: Underflow,
{
    type Out = Fl<
        <P1 as PAdd<P2, C0>>::Out,
        <E1N as ESum<E2N>>::Out,
        <E1X as ESum<E2X>>::Out,
        U,
        NoSpecials,
    >;
}

// ---------------------------------------------------------------------------
// the checks, forced through const assertions
// ---------------------------------------------------------------------------

// Pos literals: H = 1, O<P> = 2P, I<P> = 2P+1.
type P2 = O<H>;
type P3 = I<H>;
type P4 = O<O<H>>;
type P5 = I<O<H>>;
type P7 = I<I<H>>;
type P8 = O<O<O<H>>>;

const _: () = assert!(<EPos<P3> as Exponent>::VAL == 3);
const _: () = assert!(<ENeg<P3> as Exponent>::VAL == -3);
const _: () = assert!(<EZero as Exponent>::VAL == 0);

// signed addition, every sign combination
const _: () = assert!(<<EPos<P3> as ESum<EPos<P4>>>::Out as Exponent>::VAL == 7);
const _: () = assert!(<<ENeg<P3> as ESum<ENeg<P4>>>::Out as Exponent>::VAL == -7);
const _: () = assert!(<<EPos<P7> as ESum<ENeg<P4>>>::Out as Exponent>::VAL == 3);
const _: () = assert!(<<EPos<P4> as ESum<ENeg<P7>>>::Out as Exponent>::VAL == -3);
const _: () = assert!(<<ENeg<P7> as ESum<EPos<P4>>>::Out as Exponent>::VAL == -3);
const _: () = assert!(<<ENeg<P4> as ESum<EPos<P7>>>::Out as Exponent>::VAL == 3);
const _: () = assert!(<<EPos<P5> as ESum<ENeg<P5>>>::Out as Exponent>::VAL == 0);
const _: () = assert!(<<EZero as ESum<ENeg<P5>>>::Out as Exponent>::VAL == -5);
const _: () = assert!(<<EPos<P5> as ESum<EZero>>::Out as Exponent>::VAL == 5);

// two model numerals and their exact product
type M1 = Fl<P4, ENeg<P3>, EPos<P4>, Gradual, NoSpecials>; // p=4, e in [-3, 4]
type M2 = Fl<P3, ENeg<P2>, EPos<P3>, Gradual, NoSpecials>; // p=3, e in [-2, 3]
type M12 = <M1 as MulNum<M2>>::Out;

const _: () = assert!(<M1 as Numeral>::P == 4);
const _: () = assert!(<M1 as Numeral>::EMIN == -3);
const _: () = assert!(<M1 as Numeral>::EMAX == 4);
const _: () = assert!(<M2 as Numeral>::P == 3);
const _: () = assert!(<M12 as Numeral>::P == 7);
const _: () = assert!(<M12 as Numeral>::EMIN == -5);
const _: () = assert!(<M12 as Numeral>::EMAX == 7);
const _: () = assert!(!<M12 as Numeral>::SPECIALS);

// a binary32-shaped instance, to show the exponent magnitudes are not toy-sized.
// 126 = 1111110b = O<I<I<I<I<I<H>>>>>> read low bit first: 126 = 2*63, 63 = 2*31+1, ...
type P63 = I<I<I<I<I<H>>>>>;
type P126 = O<P63>;
type P127 = I<P63>;
type P24 = O<O<O<P3>>>;
type B32 = Fl<P24, ENeg<P126>, EPos<P127>, Gradual, NoSpecials>;
const _: () = assert!(<B32 as Numeral>::P == 24);
const _: () = assert!(<B32 as Numeral>::EMIN == -126);
const _: () = assert!(<B32 as Numeral>::EMAX == 127);

// the exact product numeral of binary32 with itself: p=48, e in [-252, 254].
type B32SQ = <B32 as MulNum<B32>>::Out;
const _: () = assert!(<B32SQ as Numeral>::P == 48);
const _: () = assert!(<B32SQ as Numeral>::EMIN == -252);
const _: () = assert!(<B32SQ as Numeral>::EMAX == 254);

// the mixed case the spine rule is really about: the result numeral's exponent bounds are
// computed from the operands', and the computation lands in TYPE position.
type M21 = <M2 as MulNum<M1>>::Out;
const _: () = assert!(<M21 as Numeral>::EMIN == <M12 as Numeral>::EMIN);
const _: () = assert!(<M21 as Numeral>::EMAX == <M12 as Numeral>::EMAX);
const _: () = assert!(<M21 as Numeral>::P == <M12 as Numeral>::P);

/// Forced through a signature, not left as an inert alias. A bare type alias defers its bound
/// checks; file 46's own probe 3d was green while asserting nothing until it forced.
pub fn forced<N1, N2>() -> i64
where
    N1: Numeral + MulNum<N2>,
    N2: Numeral,
{
    <<N1 as MulNum<N2>>::Out as Numeral>::EMAX
}

pub fn call_forced() -> i64 {
    forced::<M1, M2>()
}
