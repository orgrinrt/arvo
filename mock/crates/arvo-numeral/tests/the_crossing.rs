//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value under a sibling signature, and a preset that names an adaptation
//! per format.
//!
//! The crossing is checked against the three public doors it stands for:
//! every member of every signature at widths 1 to 8 crosses to every sibling,
//! keeps its slot, equals the value `adapted` builds under the target from that
//! slot, and crosses back to the value it left. The expected value never comes
//! from `under`, so a crossing that moved a member has nothing to agree with.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer, UFixed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{Adapt, DeclaredSignature, Format, Signature, WithOverflow, WithRounding};
use arvo_numeral::{Abort, Int, NamesAdaptation, Num, Quiet, Sole, Uint};
use common::{range, value};

/// Every member of `S` across to `T` and back.
fn cross<S, T>(crossed: &mut u64)
where
    S: Sole,
    T: Sole + DeclaredSignature<Format = S::Format>,
{
    let (min, max) = range::<S>();
    for a in min ..= max {
        let here = value::<S>(a);
        let there: Num<T, Quiet> = here.under::<T>();
        assert_eq!(
            there.slot().index(),
            a,
            "the crossing moved the member at {a}"
        );
        assert_eq!(
            there,
            value::<T>(a),
            "the crossing is not the three doors at {a}"
        );
        assert_eq!(
            there.under::<S>(),
            here,
            "the crossing back is not the value it left at {a}"
        );
        *crossed += 1;
    }
}

/// Every ordered pair of the adaptations listed, over one point.
macro_rules! every_ordered_pair {
    ($point:ty, $n:expr; [$($a:ty),+]) => {
        every_ordered_pair!(@from $point, $n; [$($a),+]; [$($a),+])
    };
    (@from $point:ty, $n:expr; [$($a:ty),+]; $all:tt) => {
        $( every_ordered_pair!(@to $point, $n; $a; $all); )+
    };
    (@to $point:ty, $n:expr; $a:ty; [$($b:ty),+]) => {
        $( cross::<Signature<$point, $a>, Signature<$point, $b>>($n); )+
    };
}

/// The five modes under both policies, every ordered pair of the ten.
macro_rules! every_sibling {
    ($point:ty, $n:expr) => {
        every_ordered_pair!($point, $n; [
            Adapt<TowardZero, Wrap>, Adapt<Floor, Wrap>, Adapt<Ceil, Wrap>,
            Adapt<HalfUp, Wrap>, Adapt<HalfEven, Wrap>,
            Adapt<TowardZero, Saturate>, Adapt<Floor, Saturate>, Adapt<Ceil, Saturate>,
            Adapt<HalfUp, Saturate>, Adapt<HalfEven, Saturate>
        ])
    };
}

#[test]
fn every_member_crosses_to_every_sibling_and_back_at_widths_one_to_eight() {
    let mut crossed = 0u64;
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                every_sibling!(Integer<$w>, &mut crossed);
                every_sibling!(UFixed<$w, 0>, &mut crossed);
            )+
        };
    }
    widths!(1, 2, 3, 4, 5, 6, 7, 8);
    // 2 + 4 + ... + 256 members a family, two families, a hundred ordered pairs.
    assert_eq!(crossed, 510 * 2 * 100);
}

#[test]
fn a_fixed_point_member_crosses_as_an_integer_does() {
    let mut crossed = 0u64;
    every_sibling!(Fixed<6, -3>, &mut crossed);
    every_sibling!(UFixed<6, 3>, &mut crossed);
    assert_eq!(crossed, 64 * 2 * 100);
}

type U8Wrap = Signature<UFixed<8, 0>, Adapt<TowardZero, Wrap>>;
type U8Saturate = Signature<UFixed<8, 0>, Adapt<TowardZero, Saturate>>;
type U8HalfEven = Signature<UFixed<8, 0>, Adapt<HalfEven, Wrap>>;

#[test]
fn the_value_that_arrives_adds_under_the_policy_it_crossed_into() {
    let (top, one) = (value::<U8Wrap>(255), value::<U8Wrap>(1));
    assert_eq!(top.add(one).slot().index(), 0);
    let (top, one) = (top.under::<U8Saturate>(), one.under::<U8Saturate>());
    assert_eq!(top.add(one).slot().index(), 255);
    let (top, one) = (top.under::<U8Wrap>(), one.under::<U8Wrap>());
    assert_eq!(top.add(one).slot().index(), 0);
}

#[test]
fn the_shorthands_are_the_crossing_at_the_written_sibling() {
    let (min, max) = range::<U8Wrap>();
    for a in min ..= max {
        let v = value::<U8Wrap>(a);
        // The annotations are the written siblings, so each line is also the
        // claim that the alias is that type.
        let saturating: Num<U8Saturate, Quiet> = v.with_overflow::<Saturate>();
        let half_even: Num<U8HalfEven, Quiet> = v.with_rounding::<HalfEven>();
        assert_eq!(saturating, v.under::<U8Saturate>());
        assert_eq!(half_even, v.under::<U8HalfEven>());
        let aliased: Num<WithOverflow<U8Wrap, Saturate>, Quiet> = saturating;
        let twice: Num<WithRounding<WithOverflow<U8Wrap, Saturate>, HalfEven>, Quiet> =
            aliased.with_rounding::<HalfEven>();
        assert_eq!(twice.slot().index(), a);
    }
    // One site's override: the saturating sum of two values held as wrapping.
    let (a, b) = (value::<U8Wrap>(200), value::<U8Wrap>(100));
    assert_eq!(a.add(b).slot().index(), 44);
    assert_eq!(
        a.with_overflow::<Saturate>()
            .add(b.with_overflow::<Saturate>())
            .slot()
            .index(),
        255
    );
}

#[test]
fn the_crossing_keeps_the_check_and_asks_it_nothing() {
    // The top of the range crosses under `Abort` in a debug build: nothing of the
    // range is read, so there is nothing to abort on.
    let top: Num<U8Wrap, Abort> = Num::constant::<255>();
    let there: Num<U8Saturate, Abort> = top.under();
    assert_eq!(there.slot().index(), 255);
}

/// A preset that names one adaptation for unsigned integers and another for
/// signed ones: one type, an impl per region.
///
/// Each impl is over every width at which the point is a format, which is what
/// its where clause says.
struct Preset;

impl<const BITS: u32> NamesAdaptation<UFixed<BITS, 0>> for Preset
where
    UFixed<BITS, 0>: Format,
{
    type Adaptation = Adapt<TowardZero, Wrap>;
}

impl<const BITS: u32> NamesAdaptation<Integer<BITS>> for Preset
where
    Integer<BITS>: Format,
{
    type Adaptation = Adapt<HalfEven, Saturate>;
}

/// An alias whose adaptation carries a default, which is the whole of what
/// shipping one takes.
type Count<const BITS: u32, A = Preset, C = Quiet> = Uint<BITS, A, C>;
type Offset<const BITS: u32, A = Preset, C = Quiet> = Int<BITS, A, C>;

#[test]
fn a_preset_names_an_adaptation_per_format_and_an_alias_defaults_to_it() {
    let top: Count<8> = Num::constant::<255>();
    let one: Count<8> = Num::constant::<1>();
    assert_eq!(
        top.add(one).slot().index(),
        0,
        "the preset wraps unsigned integers"
    );

    let top: Offset<8> = Num::constant::<127>();
    let one: Offset<8> = Num::constant::<1>();
    assert_eq!(
        top.add(one).slot().index(),
        127,
        "the preset saturates signed integers"
    );

    // The default is overridden per case by naming the adaptation, and a value
    // by crossing.
    let top: Count<8, Adapt<TowardZero, Saturate>> = Num::constant::<255>();
    let one: Count<8, Adapt<TowardZero, Saturate>> = Num::constant::<1>();
    assert_eq!(top.add(one).slot().index(), 255);
    let top: Count<8> = Num::constant::<255>();
    let one: Count<8> = Num::constant::<1>();
    assert_eq!(
        top.with_overflow::<Saturate>()
            .add(one.with_overflow::<Saturate>())
            .slot()
            .index(),
        255
    );
}
