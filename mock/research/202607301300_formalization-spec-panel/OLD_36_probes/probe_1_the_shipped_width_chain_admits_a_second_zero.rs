//! Probe 1: the width chain the multiplicative half already uses does NOT
//! satisfy the value-uniqueness obligation. File 34 states it does, for free
//! ("The shipped width chain already satisfies it (typenum-style binary, no
//! leading zeros)", `34:328-329`). That is a statement about the values the
//! design's *operations* happen to produce, not about the encoding, and the
//! obligation is about the encoding.
//!
//! The encoding is `25_probes/03_typelevel_binary_addwidth.rs` verbatim
//! (`UTerm` / `UInt<Hi, Lo>`, `Width` implemented for both). `Width` is the
//! whole observation surface: it is the bound every generic width position
//! carries. So the question the perimeter rule asks
//! (`what-you-can-observe-is-what-you-guaranteed.md`: "is there any way to
//! reach a value of this type for which it does not hold") is: does any type
//! other than `UTerm` inhabit `Width` with `VALUE == 0`.
//!
//! It does. `UInt<UTerm, B0>` inhabits `Width` (the blanket impl asks only
//! `Hi: Width, Lo: Bit`, both of which it satisfies) and its `VALUE` is 0.
//! So does `UInt<UInt<UTerm, B0>, B0>`, and so on: the zero value has
//! countably many spellings, and every width `n` has countably many, one per
//! count of leading zero digits. This file asserts that, which is the finding;
//! the type-level consequence is `probe_1b`, committed refusing.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_the_shipped_width_chain_admits_a_second_zero.rs
//! Outcome: WORKS (the assertions hold), and what it establishes is a defect.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

// --- 25_probes/03 encoding, unmodified in substance ---

pub trait Bit {
    const VAL: u16;
}
pub struct B0;
pub struct B1;
impl Bit for B0 {
    const VAL: u16 = 0;
}
impl Bit for B1 {
    const VAL: u16 = 1;
}

pub trait Width {
    const VALUE: u16;
}
pub struct UTerm;
pub struct UInt<Hi, Lo>(PhantomData<(Hi, Lo)>);
impl Width for UTerm {
    const VALUE: u16 = 0;
}
impl<Hi: Width, Lo: Bit> Width for UInt<Hi, Lo> {
    const VALUE: u16 = Hi::VALUE * 2 + Lo::VAL;
}

pub trait IncBy<C: Bit> {
    type Output: Width;
}
impl IncBy<B0> for UTerm {
    type Output = UTerm;
}
impl IncBy<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
}
impl<Hi: Width, Lo: Bit> IncBy<B0> for UInt<Hi, Lo> {
    type Output = UInt<Hi, Lo>;
}
impl<Hi, Lo> IncBy<B1> for UInt<Hi, Lo>
where
    Hi: Width,
    Lo: Bit + FullAdd<B1, B0>,
    Hi: IncBy<<Lo as FullAdd<B1, B0>>::Cout>,
{
    type Output =
        UInt<<Hi as IncBy<<Lo as FullAdd<B1, B0>>::Cout>>::Output, <Lo as FullAdd<B1, B0>>::Sum>;
}

pub trait FullAdd<Rhs: Bit, Cin: Bit> {
    type Sum: Bit;
    type Cout: Bit;
}
impl FullAdd<B0, B0> for B0 {
    type Sum = B0;
    type Cout = B0;
}
impl FullAdd<B1, B0> for B0 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B0, B1> for B0 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B1, B1> for B0 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B0, B0> for B1 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B1, B0> for B1 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B0, B1> for B1 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B1, B1> for B1 {
    type Sum = B1;
    type Cout = B1;
}

pub trait AddC<Rhs: Width, Cin: Bit> {
    type Output: Width;
}
impl<C: Bit> AddC<UTerm, C> for UTerm
where
    UTerm: IncBy<C>,
{
    type Output = <UTerm as IncBy<C>>::Output;
}
impl<Hi: Width, Lo: Bit, C: Bit> AddC<UTerm, C> for UInt<Hi, Lo>
where
    Self: IncBy<C>,
{
    type Output = <Self as IncBy<C>>::Output;
}
impl<Hi: Width, Lo: Bit, C: Bit> AddC<UInt<Hi, Lo>, C> for UTerm
where
    UInt<Hi, Lo>: IncBy<C>,
{
    type Output = <UInt<Hi, Lo> as IncBy<C>>::Output;
}
impl<Hi1, Lo1, Hi2, Lo2, C: Bit> AddC<UInt<Hi2, Lo2>, C> for UInt<Hi1, Lo1>
where
    Hi1: Width,
    Lo1: Bit + FullAdd<Lo2, C>,
    Hi2: Width,
    Lo2: Bit,
    Hi1: AddC<Hi2, <Lo1 as FullAdd<Lo2, C>>::Cout>,
{
    type Output = UInt<
        <Hi1 as AddC<Hi2, <Lo1 as FullAdd<Lo2, C>>::Cout>>::Output,
        <Lo1 as FullAdd<Lo2, C>>::Sum,
    >;
}

pub trait AddWidth<Rhs: Width>: Width {
    type Output: Width;
}
impl<L: Width + AddC<R, B0>, R: Width> AddWidth<R> for L {
    type Output = <L as AddC<R, B0>>::Output;
}

// --- CLAIM A: the zero value has more than one inhabitant of `Width`. ---

/// The canonical spelling.
pub type Zero0 = UTerm;
/// One leading zero digit. Nothing in the encoding refuses it: the blanket
/// impl asks only `Hi: Width, Lo: Bit`.
pub type Zero1 = UInt<UTerm, B0>;
/// Two leading zero digits.
pub type Zero2 = UInt<UInt<UTerm, B0>, B0>;

const _: () = assert!(<Zero0 as Width>::VALUE == 0);
const _: () = assert!(<Zero1 as Width>::VALUE == 0);
const _: () = assert!(<Zero2 as Width>::VALUE == 0);

/// And every other width likewise. Three spellings of thirteen.
pub type Thirteen0 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
pub type Thirteen1 = UInt<UInt<UInt<UInt<UInt<UTerm, B0>, B1>, B1>, B0>, B1>;
pub type Thirteen2 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B0>, B0>, B1>, B1>, B0>, B1>;

const _: () = assert!(<Thirteen0 as Width>::VALUE == 13);
const _: () = assert!(<Thirteen1 as Width>::VALUE == 13);
const _: () = assert!(<Thirteen2 as Width>::VALUE == 13);

// --- CLAIM B: the width adder propagates the spelling rather than ---
// --- normalising it, so a product numeral's width type depends on how ---
// --- its operands were spelled, not only on what they denote. ---

pub type SumOfCanonicalZeros = <Zero0 as AddWidth<Zero0>>::Output;
pub type SumOfPaddedZeros = <Zero1 as AddWidth<Zero0>>::Output;

const _: () = assert!(<SumOfCanonicalZeros as Width>::VALUE == 0);
const _: () = assert!(<SumOfPaddedZeros as Width>::VALUE == 0);

/// Same for a nonzero pair: 13 + 0 keeps whichever spelling of 13 it was given.
pub type ThirteenPlusZeroCanonical = <Thirteen0 as AddWidth<Zero0>>::Output;
pub type ThirteenPlusZeroPadded = <Thirteen1 as AddWidth<Zero0>>::Output;

const _: () = assert!(<ThirteenPlusZeroCanonical as Width>::VALUE == 13);
const _: () = assert!(<ThirteenPlusZeroPadded as Width>::VALUE == 13);

// --- CLAIM C: the impls that would repair this do not exist, and the ---
// --- prior art shows what they cost when they do. ---
//
// typenum, the named prior art (`26:249-252`, `34:333-335`), carries the
// same defect in the same encoding and patches it with a dedicated operator:
// `Sub` is `PrivateSub` followed by `Trim`
// (typenum-1.20.1/src/uint.rs:558-564), and `Trim` is
// `Invert -> TrimTrailingZeros -> Invert`
// (typenum-1.20.1/src/private.rs:35-36, 304-310), three further traversals of
// the digit chain whose only job is to delete leading zeros the encoding
// permitted. `And` and `Xor` pay it too (private.rs:79, 87).
//
// The chain above has no subtraction yet, which is the only reason it has not
// needed a `Trim` of its own. Any width difference (a narrowing `quantize`, an
// accumulator's guard-bit headroom, the `ceil(log2 n)` fold bound) introduces
// one, and with it the same three-pass repair.

// --- CLAIM D: the leading-zero spellings are reachable through the ---
// --- literal bridge, not only by hand. ---
//
// `25_probes/04_literal_to_typewidth_bridge.rs` is a macro-generated table
// from `const N: u16` to a width type. Its committed rows are all canonical,
// but nothing checks that: the macro takes the spelling as an argument
// (`0 => UTerm, 1 => UInt<UTerm, B1>, ...`), so a generator that emitted
// fixed-length rows (the obvious way to write one for a large bound) would
// emit `0 => UInt<UInt<..., B0>, B0>` and every width would arrive padded.
// The table below is that generator's output for a 3-digit fixed width, and
// it type-checks, values and all.

pub trait LiteralWidth<const N: u16> {
    type W: Width;
}
pub struct FixedLenLit;

impl LiteralWidth<0> for FixedLenLit {
    type W = UInt<UInt<UInt<UTerm, B0>, B0>, B0>;
}
impl LiteralWidth<1> for FixedLenLit {
    type W = UInt<UInt<UInt<UTerm, B0>, B0>, B1>;
}
impl LiteralWidth<5> for FixedLenLit {
    type W = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
}

const _: () = assert!(<<FixedLenLit as LiteralWidth<0>>::W as Width>::VALUE == 0);
const _: () = assert!(<<FixedLenLit as LiteralWidth<1>>::W as Width>::VALUE == 1);
const _: () = assert!(<<FixedLenLit as LiteralWidth<5>>::W as Width>::VALUE == 5);
