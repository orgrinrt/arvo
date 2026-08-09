// Probe 1, file 82. Does the exact fold width (file 80, probe_1) survive the precisions
// the design has already committed to supporting?
//
// File 80 built `foldexact(P, A) = bitlen(A * (2^P - 1))` at the type level and priced it
// at under a millisecond per cell, checked over p in {1, 2, 3, 8, 11, 16}. It named one
// bound and dismissed it: "`AllOnes` recurses on the *value* of P, so a pathological
// precision in the thousands would meet the default recursion limit; real precisions are
// two orders of magnitude below it" (80:192-194).
//
// Two facts make that dismissal worth testing rather than accepting.
//
// 1. rustc's default recursion limit is 128, not "thousands". A precision "two orders of
//    magnitude below" 128 would be about 1. The design commits to IEEE interchange
//    formats (78:682), which include binary128 (p = 113) and binary256 (p = 237).
// 2. `Nat::VAL` is `u128` in every probe in this corpus. `AllOnes<P>` denotes 2^P - 1,
//    which exceeds u128 at P = 128 exactly, and the fold's own intermediate
//    `A * (2^P - 1)` exceeds it sooner.
//
// So this probe instantiates the identical machinery at the four IEEE binary interchange
// precisions and records which ceiling fires first at each.
//
// Compile: rustc --edition 2021 --crate-type=lib --emit=metadata (pinned toolchain, run
// from inside the repo tree). Outcome in OUTCOMES.md.
#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---- the sealed grammar, verbatim from 80_probes/probe_1 ----

pub trait Nat {
    const VAL: u128;
}
pub struct Z;
impl Nat for Z {
    const VAL: u128 = 0;
}
pub trait Pos: Nat {}

pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);

impl Nat for H {
    const VAL: u128 = 1;
}
impl Pos for H {}
impl<P: Pos> Nat for O<P> {
    const VAL: u128 = 2 * P::VAL;
}
impl<P: Pos> Pos for O<P> {}
impl<P: Pos> Nat for I<P> {
    const VAL: u128 = 2 * P::VAL + 1;
}
impl<P: Pos> Pos for I<P> {}

// ---- predecessor, verbatim ----

pub trait DecP: Pos {
    type Out: Pos;
}
impl<Q: Pos> DecP for I<Q> {
    type Out = O<Q>;
}
impl DecP for O<H> {
    type Out = H;
}
impl<Q: Pos> DecP for O<O<Q>>
where
    O<Q>: DecP,
{
    type Out = I<<O<Q> as DecP>::Out>;
}
impl<Q: Pos> DecP for O<I<Q>> {
    type Out = I<O<Q>>;
}

// ---- 2^P - 1, verbatim ----

pub trait AllOnes: Pos {
    type Out: Pos;
}
impl AllOnes for H {
    type Out = H;
}
impl<Q: Pos> AllOnes for O<Q>
where
    O<Q>: DecP,
    <O<Q> as DecP>::Out: AllOnes,
{
    type Out = I<<<O<Q> as DecP>::Out as AllOnes>::Out>;
}
impl<Q: Pos> AllOnes for I<Q>
where
    O<Q>: AllOnes,
{
    type Out = I<<O<Q> as AllOnes>::Out>;
}

// ---- bit length, verbatim ----

pub trait Succ: Pos {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
}
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
}
impl<P: Pos> Succ for I<P>
where
    P: Succ,
{
    type Out = O<<P as Succ>::Out>;
}

pub trait BitLen: Pos {
    type Out: Pos;
}
impl BitLen for H {
    type Out = H;
}
impl<P: BitLen> BitLen for O<P>
where
    <P as BitLen>::Out: Succ,
{
    type Out = <<P as BitLen>::Out as Succ>::Out;
}
impl<P: BitLen> BitLen for I<P>
where
    <P as BitLen>::Out: Succ,
{
    type Out = <<P as BitLen>::Out as Succ>::Out;
}

// ---- the four IEEE binary interchange precisions, as grammar literals ----

// binary16: p = 11 = 0b1011
pub type P11 = I<I<O<H>>>;
// binary32: p = 24 = 0b11000
pub type P24 = O<O<O<I<H>>>>;
// binary64: p = 53 = 0b110101
pub type P53 = I<O<I<O<I<H>>>>>;
// binary128: p = 113 = 0b1110001
pub type P113 = I<O<O<O<I<I<H>>>>>>;
// binary256: p = 237 = 0b11101101
pub type P237 = I<O<I<I<O<I<I<H>>>>>>>;

// Sanity: the literals denote what they claim, checked at compile time.
const _: () = assert!(<P11 as Nat>::VAL == 11);
const _: () = assert!(<P24 as Nat>::VAL == 24);
const _: () = assert!(<P53 as Nat>::VAL == 53);
const _: () = assert!(<P113 as Nat>::VAL == 113);
const _: () = assert!(<P237 as Nat>::VAL == 237);

// ---- the test: instantiate AllOnes at each precision ----
//
// Each of these five lines is independently switchable. The probe is run five times,
// enabling one more each run, and OUTCOMES.md records which one first refuses and with
// which diagnostic.

pub type Ones11 = <P11 as AllOnes>::Out;
const _: () = assert!(<Ones11 as Nat>::VAL == (1u128 << 11) - 1);

pub type Ones24 = <P24 as AllOnes>::Out;
const _: () = assert!(<Ones24 as Nat>::VAL == (1u128 << 24) - 1);

pub type Ones53 = <P53 as AllOnes>::Out;
const _: () = assert!(<Ones53 as Nat>::VAL == (1u128 << 53) - 1);

#[cfg(feature = "p113")]
pub type Ones113 = <P113 as AllOnes>::Out;
#[cfg(feature = "p113")]
const _: () = assert!(<Ones113 as Nat>::VAL == (1u128 << 113) - 1);

#[cfg(feature = "p237")]
pub type Ones237 = <P237 as AllOnes>::Out;
#[cfg(feature = "p237")]
const _: () = assert!(<Ones237 as Nat>::VAL == 0); // value is unrepresentable in u128
