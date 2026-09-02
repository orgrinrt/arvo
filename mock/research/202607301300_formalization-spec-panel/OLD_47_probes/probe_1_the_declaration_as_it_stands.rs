//! Probe 1: what a consumer types to declare the numerals for one real
//! workload, in the ratified encoding, with nothing added.
//!
//! The workload is the one this design exists for: a contiguous column of
//! 3,000,000 telemetry samples in Q0.15 (fifteen fractional digits, quantum
//! 1/32768), folded to a total, then narrowed back. That needs three
//! numerals declared by hand: the sample's precision and quantum, and the
//! accumulator's precision.
//!
//! Nothing here is a criticism of the encoding, whose uniqueness and seal
//! are settled and correct. It is a measurement of one thing the review has
//! never measured: the number of characters, and the number of opportunities
//! to be silently wrong, between a consumer and a declared numeral.
//!
//! EXPECTED: COMPILES CLEAN, with every value const-asserted. The finding is
//! not the outcome, it is the text.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_1_the_declaration_as_it_stands.rs

#![allow(dead_code)]

use tower::nat::{Nat, Pos, Pz, Ratio, Reduce, H, I, O};

// ---------------------------------------------------------------------------
// 1. The sample numeral's precision: fifteen.
//
// 15 = 1111b. Written outside-in, each constructor doubles: I<x> is 2x+1,
// O<x> is 2x, H is 1. So the consumer decomposes 15 by hand into 2*(2*(2*1+1)+1)+1
// and types the result inside-out.
// ---------------------------------------------------------------------------

pub type SamplePrecision = Pz<I<I<I<H>>>>;
const _: () = assert!(<SamplePrecision as Nat>::VAL == 15);

// ---------------------------------------------------------------------------
// 2. The sample numeral's quantum: 1/32768.
//
// 32768 = 2^15, so the denominator is fifteen O constructors over H. There is
// no shorter spelling, because the encoding is the number's binary expansion
// and 2^15 has fifteen digits.
// ---------------------------------------------------------------------------

pub type Pow2_15 = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>;
const _: () = assert!(<Pow2_15 as Pos>::VAL == 32768);

pub type SampleQuantum = Ratio<H, Pow2_15>;

// ---------------------------------------------------------------------------
// 3. The accumulator's precision.
//
// Total safety for arity n over a destination of precision p wants
// p + ceil(log2 n) digits (`40:328-344`). n = 3_000_000, ceil(log2 n) = 22,
// so the accumulator precision is 37.
//
// 37 = 100101b = 2*(2*(2*(2*(2*1)+1)+0)+0)+1, so: I<O<I<O<O<H>>>>>.
// Getting this wrong by one constructor is a well-formed Nat denoting a
// different number, and probe 1b is that.
// ---------------------------------------------------------------------------

pub type AccumPrecision = Pz<I<O<I<O<O<H>>>>>>;
const _: () = assert!(<AccumPrecision as Nat>::VAL == 37);

// ---------------------------------------------------------------------------
// 4. For scale: IEEE binary64's precision, 53.
//
// 53 = 110101b. Six constructors deep.
// ---------------------------------------------------------------------------

pub type Binary64Precision = Pz<I<O<I<O<I<H>>>>>>;
const _: () = assert!(<Binary64Precision as Nat>::VAL == 53);

// ---------------------------------------------------------------------------
// 5. And a decimal64 significand digit count, 16, for the radix-ten case the
// ratified contract exists to cover.
// ---------------------------------------------------------------------------

pub type Decimal64Digits = Pz<O<O<O<O<H>>>>>;
const _: () = assert!(<Decimal64Digits as Nat>::VAL == 16);

// ---------------------------------------------------------------------------
// 6. A reduced quantum through the normalising alias, which is the shape the
// tower recommends for a consumer-written ratio (`vu_nat.rs:450`). The
// consumer still spells both magnitudes in binary.
//
// 3/12 reduces to 1/4: 3 = I<H>, 12 = O<O<I<H>>>.
// ---------------------------------------------------------------------------

pub type ThreeTwelfths = <Ratio<I<H>, O<O<I<H>>>> as Reduce>::N;
pub type ThreeTwelfthsD = <Ratio<I<H>, O<O<I<H>>>> as Reduce>::D;
const _: () = assert!(<ThreeTwelfths as Pos>::VAL == 1);
const _: () = assert!(<ThreeTwelfthsD as Pos>::VAL == 4);
