//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The packed word's laws, over a 32-bit handle of a 28-bit id, a 3-bit kind
//! and a one-bit flag, over its wide-tag twin of 26, 5 and 1, over two 64-bit
//! words, one field the whole width and a one-bit field at each edge of the
//! carrier, and over every container width from one to sixty-four.

#![feature(const_trait_impl)]

use core::fmt::Debug;

use arvo_bits::{Below, Bits, Field, Lane, Layout, Word};
use arvo_format::{Bool, Width};
use notko_hlist::{Cons, Empty, Place, PlaceOf};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Kind;
#[derive(Clone, Copy)]
struct Flag;

impl Field for Id {
    type Value = Bits<28>;
}

impl Field for Kind {
    type Value = Bits<3>;
}

impl Field for Flag {
    type Value = Bool;
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Kind, Cons<Flag, Empty>>>;
}

#[derive(Clone, Copy)]
struct WideId;
#[derive(Clone, Copy)]
struct WideKind;

impl Field for WideId {
    type Value = Bits<26>;
}

impl Field for WideKind {
    type Value = Bits<5>;
}

/// The wide-tag shape, sharing the handle's flag at the same top bit.
struct Wide;

impl Layout for Wide {
    type Container = Bits<32>;
    type Fields = Cons<WideId, Cons<WideKind, Cons<Flag, Empty>>>;
}

#[derive(Clone, Copy)]
struct Tag;

impl Field for Tag {
    type Value = Bits<7>;
}

/// The handle's flag again, at bit zero this time.
struct Small;

impl Layout for Small {
    type Container = Bits<8>;
    type Fields = Cons<Flag, Cons<Tag, Empty>>;
}

/// The patterns a field is written with and a word is started from: zero, every
/// bit, and the two alternations, which a field moved by one bit cannot read back.
const PATTERNS: [u64; 4] = [0, u64::MAX, 0xaaaa_aaaa_aaaa_aaaa, 0x5555_5555_5555_5555];

/// A word of any width `N` as one field the whole width.
#[derive(Clone, Copy)]
struct All<const N: u32>;

impl<const N: u32> Field for All<N> {
    type Value = Bits<N>;
}

struct Whole<const N: u32>;

impl<const N: u32> Layout for Whole<N> {
    type Container = Bits<N>;
    type Fields = Cons<All<N>, Empty>;
}

/// A word of width `N` as an `M`-bit field under a one-bit field at the top;
/// `M` is `N - 1`, written out by the caller.
#[derive(Clone, Copy)]
struct Low<const M: u32>;
#[derive(Clone, Copy)]
struct Top;

impl<const M: u32> Field for Low<M> {
    type Value = Bits<M>;
}

impl Field for Top {
    type Value = Bool;
}

struct Split<const N: u32, const M: u32>;

impl<const N: u32, const M: u32> Layout for Split<N, M> {
    type Container = Bits<N>;
    type Fields = Cons<Low<M>, Cons<Top, Empty>>;
}

/// A 64-bit word with a one-bit field at each edge of the carrier and a 62-bit
/// field between them.
#[derive(Clone, Copy)]
struct Bottom;
#[derive(Clone, Copy)]
struct Middle;

impl Field for Bottom {
    type Value = Bool;
}

impl Field for Middle {
    type Value = Bits<62>;
}

struct Edges;

impl Layout for Edges {
    type Container = Bits<64>;
    type Fields = Cons<Bottom, Cons<Middle, Cons<Top, Empty>>>;
}

/// The bits field `F` at place `P` takes in a word of `L`.
fn span<L: Layout, F: Field, P: Place>() -> u64
where
    L::Fields: Below<P>,
{
    let w = <F::Value as Lane>::WIDTH.count();
    (u64::MAX >> (64 - w)) << <L::Fields as Below<P>>::BELOW.count()
}

/// Writes every pattern to `field` over every starting word, and checks that
/// it reads back and that nothing outside it moved.
fn round_trips<L, F, P>(field: F)
where
    L: Layout,
    F: Field,
    P: Place,
    F::Value: PartialEq + Debug,
    L::Fields: PlaceOf<F, P> + Below<P>,
{
    for start in PATTERNS {
        let word = Word::<L>::of(L::Container::from_low(Bits::masked(start)));
        for pattern in PATTERNS {
            let to = F::Value::from_low(Bits::masked(pattern));
            let written = word.with(field, to);
            assert_eq!(written.get(field), to, "start {start:#x}, pattern {pattern:#x}");
            assert_eq!(
                written.bits().low().raw() & !span::<L, F, P>(),
                word.bits().low().raw() & !span::<L, F, P>(),
                "writing a field moved a bit outside it, start {start:#x}, pattern {pattern:#x}"
            );
        }
    }
}

#[test]
fn each_field_of_the_handle_reads_back_what_was_written_and_moves_nothing_else() {
    round_trips::<Handle, _, _>(Id);
    round_trips::<Handle, _, _>(Kind);
    round_trips::<Handle, _, _>(Flag);
}

#[test]
fn each_field_of_the_wide_handle_reads_back_what_was_written_and_moves_nothing_else() {
    round_trips::<Wide, _, _>(WideId);
    round_trips::<Wide, _, _>(WideKind);
    round_trips::<Wide, _, _>(Flag);
}

#[test]
fn each_field_of_a_sixty_four_bit_word_reads_back_what_was_written_and_moves_nothing_else() {
    round_trips::<Whole<64>, _, _>(All::<64>);
    round_trips::<Edges, _, _>(Bottom);
    round_trips::<Edges, _, _>(Middle);
    round_trips::<Edges, _, _>(Top);
}

/// The laws and the raw bits of a word of width `N` as one field.
fn whole_at<const N: u32>() {
    let mask = u64::MAX >> (64 - N);
    round_trips::<Whole<N>, _, _>(All::<N>);
    for pattern in PATTERNS {
        let whole = Word::<Whole<N>>::of(Bits::masked(0)).with(All::<N>, Bits::masked(pattern));
        assert_eq!(whole.bits().raw(), pattern & mask, "width {N}, pattern {pattern:#x}");
        let back = Word::<Whole<N>>::of(Bits::masked(pattern));
        assert_eq!(back.get(All::<N>).raw(), pattern & mask, "width {N}, pattern {pattern:#x}");
    }
}

/// The laws and the raw bits of a word of width `N` as an `M`-bit field under
/// a one-bit field, `M` being `N - 1`.
fn split_at<const N: u32, const M: u32>() {
    round_trips::<Split<N, M>, _, _>(Low::<M>);
    round_trips::<Split<N, M>, _, _>(Top);
    let low = u64::MAX >> (64 - M);
    for pattern in PATTERNS {
        let word = Word::<Split<N, M>>::of(Bits::masked(0))
            .with(Low::<M>, Bits::masked(pattern))
            .with(Top, Bool::TRUE);
        assert_eq!(word.bits().raw(), (1 << M) | (pattern & low), "width {N}, pattern {pattern:#x}");
        let parts = Word::<Split<N, M>>::of(Bits::masked(pattern));
        assert_eq!(parts.get(Low::<M>).raw(), pattern & low, "width {N}, pattern {pattern:#x}");
        assert_eq!(parts.get(Top).get(), pattern >> M & 1 == 1, "width {N}, pattern {pattern:#x}");
    }
}

/// `whole_at::<1>()`, then `whole_at::<N>()` and `split_at::<N, N - 1>()` for
/// each `N` given, `N - 1` worked out here.
macro_rules! at_widths {
    ($($n:literal)*) => { whole_at::<1>(); $( whole_at::<$n>(); split_at::<$n, { $n - 1 }>(); )* };
}

#[test]
fn every_container_width_from_one_to_sixty_four_holds_its_fields_where_the_list_puts_them() {
    at_widths!(
        2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62
        63 64
    );
}

#[test]
fn the_edges_of_a_sixty_four_bit_word_hold_each_field_where_its_layout_puts_it() {
    let word = Word::<Edges>::of(Bits::masked(0))
        .with(Top, Bool::TRUE)
        .with(Middle, Bits::masked(0x1234_5678_9abc_def0));
    assert_eq!(word.bits().raw(), 0x8000_0000_0000_0000 | (0x1234_5678_9abc_def0 << 1));
    let word = word.with(Bottom, Bool::TRUE);
    assert_eq!(word.bits().raw() & 1, 1);
    let parts = Word::<Edges>::of(Bits::masked(0x8000_0000_0000_0001));
    assert!(parts.get(Top).get());
    assert!(parts.get(Bottom).get());
    assert_eq!(parts.get(Middle).raw(), 0);
    let middle = Word::<Edges>::of(Bits::masked(0x7fff_ffff_ffff_fffe));
    assert!(!middle.get(Top).get());
    assert!(!middle.get(Bottom).get());
    assert_eq!(middle.get(Middle).raw(), 0x3fff_ffff_ffff_ffff);
    let whole = Word::<Whole<64>>::of(Bits::masked(u64::MAX)).with(All::<64>, Bits::masked(0x5555));
    assert_eq!(whole.bits().raw(), 0x5555);
}

#[test]
fn one_field_listed_in_two_layouts_reads_and_writes_at_its_place_in_each() {
    let handle = Word::<Handle>::of(Bits::masked(0)).with(Flag, Bool::TRUE);
    assert_eq!(handle.bits().raw(), 0x8000_0000);
    let small = Word::<Small>::of(Bits::masked(0)).with(Flag, Bool::TRUE);
    assert_eq!(small.bits().raw(), 1);
    assert!(Word::<Handle>::of(Bits::masked(0x8000_0000)).get(Flag).get());
    assert!(!Word::<Handle>::of(Bits::masked(1)).get(Flag).get());
    assert!(Word::<Small>::of(Bits::masked(1)).get(Flag).get());
    assert!(!Word::<Small>::of(Bits::masked(0x80)).get(Flag).get());
}

#[test]
fn writing_one_field_leaves_the_others_reading_as_they_did() {
    for start in PATTERNS {
        let word = Word::<Handle>::of(Bits::masked(start));
        for pattern in PATTERNS {
            let kind = word.with(Kind, Bits::masked(pattern));
            assert_eq!(kind.get(Id), word.get(Id));
            assert_eq!(kind.get(Flag), word.get(Flag));
            let id = word.with(Id, Bits::masked(pattern));
            assert_eq!(id.get(Kind), word.get(Kind));
            assert_eq!(id.get(Flag), word.get(Flag));
            let flag = word.with(Flag, Bool::of(pattern & 1 == 1));
            assert_eq!(flag.get(Id), word.get(Id));
            assert_eq!(flag.get(Kind), word.get(Kind));
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
        .with(Id, Bits::masked(0x0abc_def1))
        .with(Kind, Bits::masked(0b101))
        .with(Flag, Bool::TRUE);
    assert_eq!(word.bits().raw(), 0xdabc_def1);
    let parts = Word::<Handle>::of(Bits::masked(0xdabc_def1));
    assert_eq!(parts.get(Id).raw(), 0x0abc_def1);
    assert_eq!(parts.get(Kind).raw(), 0b101);
    assert!(parts.get(Flag).get());
    let wide = Word::<Wide>::of(Bits::masked(0))
        .with(WideId, Bits::masked(0x02bc_def1))
        .with(WideKind, Bits::masked(0b10110))
        .with(Flag, Bool::FALSE);
    assert_eq!(wide.bits().raw(), 0x5abc_def1);
}

const HANDLE: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with(Id, Bits::masked(0x0abc_def1))
    .with(Kind, Bits::masked(5))
    .with(Flag, Bool::TRUE);
const ID: Bits<28> = HANDLE.get(Id);
const KIND: Bits<3> = HANDLE.get(Kind);
const FLAG: Bool = HANDLE.get(Flag);
const RAW: Bits<32> = HANDLE.bits();
const EDGES: Word<Edges> = Word::<Edges>::of(Bits::masked(0))
    .with(Top, Bool::TRUE)
    .with(Bottom, Bool::TRUE)
    .with(Middle, Bits::masked(0x2aaa_aaaa_aaaa_aaaa));
const TOP: Bool = EDGES.get(Top);
const MIDDLE: Bits<62> = EDGES.get(Middle);
const ALL: Bits<64> = Word::<Whole<64>>::of(Bits::masked(0))
    .with(All::<64>, Bits::masked(u64::MAX))
    .get(All::<64>);

#[test]
fn every_read_and_write_evaluates_in_a_constant() {
    assert_eq!(ID.raw(), 0x0abc_def1);
    assert_eq!(KIND.raw(), 5);
    assert!(FLAG.get());
    assert_eq!(RAW.raw(), 0xdabc_def1);
    assert!(TOP.get());
    assert_eq!(MIDDLE.raw(), 0x2aaa_aaaa_aaaa_aaaa);
    assert_eq!(EDGES.bits().raw(), 0xd555_5555_5555_5555);
    assert_eq!(ALL.raw(), u64::MAX);
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
