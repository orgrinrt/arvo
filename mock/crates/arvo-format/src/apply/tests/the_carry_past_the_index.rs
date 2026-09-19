//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The translation law, fed positions past the ends of the index.
//!
//! No slot names a position past the index, so these are reached the one way a
//! caller can reach them, through `Exact::between` carrying a remainder past the
//! end. Every range at an end is fed from its end slot, and its reference well
//! inside the index is fed the same ratio from its own end slot, where the carry
//! lands as an ordinary slot. A map answering for the position named answers the
//! two the same, shifted; one answering for a position pinned at the end of the
//! index does not, under wrapping in the slot and under saturation in the
//! verdict, at both ends.

use notko::Maybe;

use super::the_broken_maps::{no_lo_guard_onto_the_lowest, shipped as shipped_map};
use super::the_far_end_of_the_index::{BottomOf200, TopOf200};
use super::the_translation_law::{
    NegativeReference,
    NegativeReference200,
    Reference,
    Reference200,
    WideBottom,
    WideNegativeReference,
    WideReference,
    WideTop,
};
use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow, round_slot};
use crate::format::Format;
use crate::overflow::{Clamp, Policy, Saturate, Wrap};
use crate::points::Integer;
use crate::quantum::Constant;
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};

/// A plain integer format over a slot range.
type Over<S> = Grid<BinaryRationals, Constant<0>, S, 0, 1>;

/// How many whole slots past the end the positions reach. A function rather than
/// an item constant, for the reason `the_ratio_coordinate` gives.
fn reach() -> i64 {
    300
}

/// No dither, and one either side of the quarter the stochastic mode reads.
fn dithers() -> [Dither; 3] {
    [Dither::UNUSED, Dither::at(Fraction::of(1, 8)), Dither::at(Fraction::of(5, 8))]
}

/// The shipped constructor, which keeps the distance past the index.
fn shipped(slot: Slot, part: Fraction) -> Exact {
    Exact::between(slot, part)
}

/// A constructor that pins the slot at the index's end and drops the distance,
/// which is what `between` did before it kept one.
fn pinned(slot: Slot, part: Fraction) -> Exact {
    let exact = Exact::between(slot, part);
    Exact {
        past: 0,
        ..exact
    }
}

/// The law with the shipped constructor.
fn carries<R: DeclaredSignature, M: DeclaredSignature>() -> Maybe<(i64, Fraction, Dither)> {
    carries_with::<R, M>(shipped)
}

/// The first distance, remainder and dither at which `M` does not answer as `R`
/// shifted, for positions past the end of the index `M`'s range sits at, with
/// the moved side built by `make`.
///
/// A range ending at `i128::MAX` is fed from its highest slot upward, and one
/// starting at `i128::MIN` from its lowest slot downward. Remainders are the
/// four quarters, so a tie is among them.
fn carries_with<R: DeclaredSignature, M: DeclaredSignature>(
    make: fn(Slot, Fraction) -> Exact,
) -> Maybe<(i64, Fraction, Dither)> {
    let (r_lo, r_hi) = (
        <<R::Format as Format>::Slots as Slots>::MIN.index(),
        <<R::Format as Format>::Slots as Slots>::MAX.index(),
    );
    let (m_lo, m_hi) = (
        <<M::Format as Format>::Slots as Slots>::MIN.index(),
        <<M::Format as Format>::Slots as Slots>::MAX.index(),
    );
    let at_the_top = m_hi == i128::MAX;
    assert!(
        at_the_top || m_lo == i128::MIN,
        "the range is not at an end of the index"
    );
    let shift = m_lo - r_lo;
    let (r_from, m_from, sign) = if at_the_top { (r_hi, m_hi, 1) } else { (r_lo, m_lo, -1) };
    for k in 1 ..= reach() {
        for quarter in 0 .. 4 {
            let part = Fraction::of(sign * 4 * k + quarter, 4);
            for dither in dithers() {
                let r = Exact::between(Slot::at(r_from), part);
                let m = make(Slot::at(m_from), part);
                if adapt::<M>(m, dither).index() - shift != adapt::<R>(r, dither).index()
                    || panic_on_overflow::<M>(m, dither) != panic_on_overflow::<R>(r, dither)
                {
                    return Maybe::Is((sign * k, part, dither));
                }
            }
        }
    }
    Maybe::Isnt
}

/// The law over all eighteen declared signatures for one pair of ranges.
macro_rules! every_signature {
    ($reference:ty, $moved:ty) => {
        every_signature!(@modes $reference, $moved;
            Floor, Ceil, TowardZero, HalfUp, HalfEven, Stochastic);
    };
    (@modes $reference:ty, $moved:ty; $($mode:ty),+) => {
        $( every_signature!(@policies $reference, $moved, $mode; Wrap, Saturate, Clamp); )+
    };
    (@policies $reference:ty, $moved:ty, $mode:ty; $($policy:ty),+) => {
        $(
            assert_eq!(
                carries::<
                    Signature<Over<$reference>, Adapt<$mode, $policy>>,
                    Signature<Over<$moved>, Adapt<$mode, $policy>>,
                >(),
                Maybe::Isnt,
                "{} {} over {} against {}",
                stringify!($mode),
                stringify!($policy),
                stringify!($moved),
                stringify!($reference)
            );
        )+
    };
}

#[test]
fn past_the_top_of_the_index_a_range_adapts_as_its_reference_does() {
    every_signature!(Reference, AtTheTop);
    every_signature!(WideReference, WideTop);
    every_signature!(Reference200, TopOf200);
}

#[test]
fn past_the_bottom_of_the_index_a_range_adapts_as_its_reference_does() {
    every_signature!(NegativeReference, AtTheBottom);
    every_signature!(WideNegativeReference, WideBottom);
    every_signature!(NegativeReference200, BottomOf200);
}

#[test]
fn the_law_reports_a_constructor_that_drops_the_distance() {
    // The negative control, kept as a constructor the suite holds rather than an
    // edit made once and reverted. Pinned without its distance, a position past
    // the top wraps as though it were a quarter past the top, and one past the
    // bottom floors onto the bottom and is called in range.
    type TopWrap<S> = Signature<Over<S>, Adapt<Ceil, Wrap>>;
    type BottomWrap<S> = Signature<Over<S>, Adapt<Floor, Wrap>>;
    assert!(carries_with::<TopWrap<Reference>, TopWrap<AtTheTop>>(pinned).is());
    assert!(carries_with::<TopWrap<Reference200>, TopWrap<TopOf200>>(pinned).is());
    assert!(carries_with::<BottomWrap<NegativeReference>, BottomWrap<AtTheBottom>>(pinned).is());
    assert!(carries_with::<BottomWrap<WideNegativeReference>, BottomWrap<WideBottom>>(pinned).is());

    // Under saturation both land on the same end slot, so only the verdict can
    // tell them apart, and it does: a whole slot past the top pins onto the top
    // itself, in range, and `Floor` puts one past the bottom on the bottom.
    type TopSat<S> = Signature<Over<S>, Adapt<Ceil, Saturate>>;
    assert_eq!(
        carries_with::<TopSat<Reference>, TopSat<AtTheTop>>(pinned),
        Maybe::Is((1, Fraction::of(4, 4), Dither::UNUSED))
    );
    type BottomSat<S> = Signature<Over<S>, Adapt<Floor, Saturate>>;
    assert!(carries_with::<BottomSat<NegativeReference>, BottomSat<AtTheBottom>>(pinned).is());
}

#[test]
fn the_carry_reaches_past_the_index_at_every_distance_it_is_fed() {
    // The law above is only about the index's ends if its positions are past
    // them. Every one of them is, and the reference's are not.
    for k in 1 ..= reach() {
        let up = Exact::between(Slot::at(i128::MAX), Fraction::of(4 * k, 4));
        assert_eq!((up.slot(), up.past), (Slot::at(i128::MAX), k));
        let down = Exact::between(Slot::at(i128::MIN), Fraction::of(-4 * k, 4));
        assert_eq!((down.slot(), down.past), (Slot::at(i128::MIN), -k));
        let inside = Exact::between(Slot::at(1255), Fraction::of(4 * k, 4));
        assert_eq!(inside.past, 0);
    }
}

#[test]
fn a_step_past_the_bottom_is_out_of_range_where_the_fed_range_does_not_start_there() {
    // `Exact::between(Slot::at(i128::MIN), Fraction::of(-3, 4))` names `MIN -
    // 3/4`, and `Ceil` takes it to `MIN`. Fed to `Integer<3>` (`[-4, 3]`) under
    // `Wrap`, the range's own bottom is not `i128::MIN`, so the shipped verdict
    // is out of range and the wrap reduces `i128::MIN` modulo eight to `0`,
    // worked by hand. `no_lo_guard_onto_the_lowest`, the verdict without the
    // `lo == i128::MIN` conjunct, calls the same step in range. The oracle sweep
    // reports that map over every range; this is one point of it, reduced by
    // hand.
    type Wrapped = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    let position = Exact::between(Slot::at(i128::MIN), Fraction::of(-3, 4));
    assert_eq!(adapt::<Wrapped>(position, Dither::UNUSED), Slot::at(0));
    assert!(panic_on_overflow::<Wrapped>(position, Dither::UNUSED).get());

    let stepped = round_slot(Mode::Ceil, position, Dither::UNUSED);
    let (min, max) = (Slot::at(-4), Slot::at(3));
    assert!((shipped_map().leaves)(stepped, min, max));
    assert_eq!(
        (shipped_map().complete)(Policy::Wrap, stepped, min, max),
        Slot::at(0)
    );
    assert!(!(no_lo_guard_onto_the_lowest().leaves)(stepped, min, max));
}
