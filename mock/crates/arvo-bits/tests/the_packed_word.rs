//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The packed word's laws, over a 32-bit handle of a 28-bit id, a 3-bit kind
//! and a one-bit flag, and over its wide-tag twin of 26, 5 and 1, each layout
//! listing its fields out of their bit order.

#![feature(const_trait_impl)]

use core::fmt::Debug;

use arvo_bits::{Bits, Field, Holds, Lane, Layout, Word};
use arvo_format::{Bool, Width};

struct Handle;
struct Id;
struct Kind;
struct Flag;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Flag, (Id, (Kind, ())));
}

impl Field for Id {
    type Layout = Handle;
    type Value = Bits<28>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Kind {
    type Layout = Handle;
    type Value = Bits<3>;
    const BELOW: Width = Width::bits(28);
}

impl Field for Flag {
    type Layout = Handle;
    type Value = Bool;
    const BELOW: Width = Width::bits(31);
}

struct Wide;
struct WideId;
struct WideKind;
struct WideFlag;

impl Layout for Wide {
    type Container = Bits<32>;
    type Fields = (WideKind, (WideFlag, (WideId, ())));
}

impl Field for WideId {
    type Layout = Wide;
    type Value = Bits<26>;
    const BELOW: Width = Width::bits(0);
}

impl Field for WideKind {
    type Layout = Wide;
    type Value = Bits<5>;
    const BELOW: Width = Width::bits(26);
}

impl Field for WideFlag {
    type Layout = Wide;
    type Value = Bool;
    const BELOW: Width = Width::bits(31);
}

/// The patterns a field is written with and a word is started from: zero, every
/// bit, and the two alternations, which a field moved by one bit cannot read back.
const PATTERNS: [u64; 4] = [0, u64::MAX, 0xaaaa_aaaa_aaaa_aaaa, 0x5555_5555_5555_5555];

/// The bits field `F` takes in its 32-bit word.
fn span<F: Field>() -> u64 {
    let w = <F::Value as Lane>::WIDTH.count();
    (u64::MAX >> (64 - w)) << F::BELOW.count()
}

/// Writes every pattern to `F` over every starting word, and checks that it
/// reads back and that nothing outside it moved.
fn round_trips<L, F>()
where
    L: Layout<Container = Bits<32>>,
    F: Field<Layout = L>,
    F::Value: PartialEq + Debug,
    L::Fields: Holds<F>,
{
    for start in PATTERNS {
        let word = Word::<L>::of(Bits::masked(start));
        for pattern in PATTERNS {
            let to = F::Value::from_low(Bits::masked(pattern));
            let written = word.with::<F>(to);
            assert_eq!(written.get::<F>(), to, "start {start:#x}, pattern {pattern:#x}");
            assert_eq!(
                written.bits().raw() & !span::<F>(),
                word.bits().raw() & !span::<F>(),
                "writing a field moved a bit outside it, start {start:#x}, pattern {pattern:#x}"
            );
        }
    }
}

#[test]
fn each_field_of_the_handle_reads_back_what_was_written_and_moves_nothing_else() {
    round_trips::<Handle, Id>();
    round_trips::<Handle, Kind>();
    round_trips::<Handle, Flag>();
}

#[test]
fn each_field_of_the_wide_handle_reads_back_what_was_written_and_moves_nothing_else() {
    round_trips::<Wide, WideId>();
    round_trips::<Wide, WideKind>();
    round_trips::<Wide, WideFlag>();
}

#[test]
fn writing_one_field_leaves_the_others_reading_as_they_did() {
    for start in PATTERNS {
        let word = Word::<Handle>::of(Bits::masked(start));
        for pattern in PATTERNS {
            let kind = word.with::<Kind>(Bits::masked(pattern));
            assert_eq!(kind.get::<Id>(), word.get::<Id>());
            assert_eq!(kind.get::<Flag>(), word.get::<Flag>());
            let id = word.with::<Id>(Bits::masked(pattern));
            assert_eq!(id.get::<Kind>(), word.get::<Kind>());
            assert_eq!(id.get::<Flag>(), word.get::<Flag>());
            let flag = word.with::<Flag>(Bool::of(pattern & 1 == 1));
            assert_eq!(flag.get::<Id>(), word.get::<Id>());
            assert_eq!(flag.get::<Kind>(), word.get::<Kind>());
        }
    }
}

#[test]
fn of_and_bits_invert_each_other() {
    for raw in PATTERNS.into_iter().chain([1, 0x8000_0000, 0x7fff_ffff, 0x1234_5678]) {
        let bits = Bits::<32>::masked(raw);
        assert_eq!(Word::<Handle>::of(bits).bits(), bits);
        let word = Word::<Handle>::of(bits);
        assert_eq!(Word::<Handle>::of(word.bits()), word);
    }
}

#[test]
fn the_handle_holds_each_field_where_its_layout_puts_it() {
    let word = Word::<Handle>::of(Bits::masked(0))
        .with::<Id>(Bits::masked(0x0abc_def1))
        .with::<Kind>(Bits::masked(0b101))
        .with::<Flag>(Bool::TRUE);
    assert_eq!(word.bits().raw(), 0xdabc_def1);
    let parts = Word::<Handle>::of(Bits::masked(0xdabc_def1));
    assert_eq!(parts.get::<Id>().raw(), 0x0abc_def1);
    assert_eq!(parts.get::<Kind>().raw(), 0b101);
    assert!(parts.get::<Flag>().get());
    let wide = Word::<Wide>::of(Bits::masked(0))
        .with::<WideId>(Bits::masked(0x02bc_def1))
        .with::<WideKind>(Bits::masked(0b10110))
        .with::<WideFlag>(Bool::FALSE);
    assert_eq!(wide.bits().raw(), 0x5abc_def1);
}

const HANDLE: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with::<Id>(Bits::masked(0x0abc_def1))
    .with::<Kind>(Bits::masked(5))
    .with::<Flag>(Bool::TRUE);
const ID: Bits<28> = HANDLE.get::<Id>();
const KIND: Bits<3> = HANDLE.get::<Kind>();
const FLAG: Bool = HANDLE.get::<Flag>();
const RAW: Bits<32> = HANDLE.bits();

#[test]
fn every_read_and_write_evaluates_in_a_constant() {
    assert_eq!(ID.raw(), 0x0abc_def1);
    assert_eq!(KIND.raw(), 5);
    assert!(FLAG.get());
    assert_eq!(RAW.raw(), 0xdabc_def1);
}

#[test]
fn a_lane_agrees_with_cast_and_with_bit_zero() {
    for raw in PATTERNS.into_iter().chain([1, 2, 0x8000_0000_0000_0000]) {
        let low = Bits::<64>::masked(raw);
        assert_eq!(<Bits<28> as Lane>::from_low(low), low.cast::<28>());
        assert_eq!(<Bits<28> as Lane>::from_low(low).low(), low.cast::<28>().cast::<64>());
        assert_eq!(<Bits<64> as Lane>::from_low(low), low);
        assert_eq!(<Bool as Lane>::from_low(low), low.bit::<0>());
        let bit = low.bit::<0>();
        assert_eq!(bit.low(), Bits::<64>::masked(0).with_bit::<0>(bit));
    }
    assert_eq!(<Bool as Lane>::WIDTH, Width::bits(1));
    assert_eq!(<Bits<28> as Lane>::WIDTH, Width::bits(28));
}
