//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Translation by a whole number of slots moves every answer by that number.
//!
//! Under every mode and every policy, a range at an end of the index adapts a
//! position the way the same range placed well inside the index adapts the same
//! position relative to it. The references sit off zero, far enough that every
//! position fed to them keeps one sign on both sides, which is what lets the
//! modes reading a sign be fed positions below the lowest slot and past the
//! highest. Every shift is even, so half-even's parity survives it. And each
//! reference's lowest slot is not a multiple of its span, while the ranges at the
//! ends of the index are, so a wrap reduced from zero rather than from the
//! lowest slot answers differently on the two sides.
//!
//! The law is written twice. Once through `adapt` and `panic_on_overflow`, the
//! surface, over every declared signature. And once as a function of the map it
//! is asked about, so the broken maps in `the_broken_maps.rs` are reported by a
//! test rather than by edits somebody made once and reverted.

use notko::Maybe;

use super::the_broken_maps::{
    Map,
    anchored_at_zero,
    no_step_onto_the_lowest,
    reduced_modulo_256,
    shipped,
    subtracts_first,
};
use super::the_far_end_of_the_index::{BottomOf200, TopOf200};
use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow, round_slot};
use crate::format::Format;
use crate::overflow::{Clamp, Policy, SHIPPED_POLICIES, Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};
use crate::width::Width;

/// `2^64`, the widest span an admitted range has.
const fn span_of_64_bits() -> i128 {
    1 << 64
}

/// The reference for the range at the top: 256 slots from 1000.
pub(super) struct Reference;

impl Slots for Reference {
    const MAX: Slot = Slot::at(1255);
    const MIN: Slot = Slot::at(1000);
    const WIDTH: Width = Width::bits(8);
}

/// The reference for the range at the bottom: 256 slots from -2000.
pub(super) struct NegativeReference;

impl Slots for NegativeReference {
    const MAX: Slot = Slot::at(-1745);
    const MIN: Slot = Slot::at(-2000);
    const WIDTH: Width = Width::bits(8);
}

/// A range at the top of the index as wide as an admitted range gets.
pub(super) struct WideTop;

impl Slots for WideTop {
    const MAX: Slot = Slot::at(i128::MAX);
    const MIN: Slot = Slot::at(i128::MAX - span_of_64_bits() + 1);
    const WIDTH: Width = Width::bits(64);
}

/// Its reference, `2^64` slots from `2^64 + 1000`.
pub(super) struct WideReference;

impl Slots for WideReference {
    const MAX: Slot = Slot::at(2 * span_of_64_bits() + 999);
    const MIN: Slot = Slot::at(span_of_64_bits() + 1000);
    const WIDTH: Width = Width::bits(64);
}

/// A range at the bottom of the index as wide as an admitted range gets.
pub(super) struct WideBottom;

impl Slots for WideBottom {
    const MAX: Slot = Slot::at(i128::MIN + span_of_64_bits() - 1);
    const MIN: Slot = Slot::at(i128::MIN);
    const WIDTH: Width = Width::bits(64);
}

/// Its reference, `2^64` slots ending at `-2^64 - 1001`.
pub(super) struct WideNegativeReference;

impl Slots for WideNegativeReference {
    const MAX: Slot = Slot::at(-span_of_64_bits() - 1001);
    const MIN: Slot = Slot::at(-2 * span_of_64_bits() - 1000);
    const WIDTH: Width = Width::bits(64);
}

/// The reference for `TopOf200`: 200 slots from 1002, which is 2 modulo 200
/// where `TopOf200`'s lowest slot is 128.
pub(super) struct Reference200;

impl Slots for Reference200 {
    const MAX: Slot = Slot::at(1201);
    const MIN: Slot = Slot::at(1002);
    const WIDTH: Width = Width::bits(8);
}

/// The reference for `BottomOf200`: 200 slots from -1998, which is 2 modulo 200
/// where `BottomOf200`'s lowest slot is 72.
pub(super) struct NegativeReference200;

impl Slots for NegativeReference200 {
    const MAX: Slot = Slot::at(-1799);
    const MIN: Slot = Slot::at(-1998);
    const WIDTH: Width = Width::bits(8);
}

/// A plain integer format over a slot range.
type Over<S> = Grid<BinaryRationals, Constant<0>, S, 0, 1>;

/// Inclusive bands of offsets from the lowest slot, fed to both sides.
///
/// Below the lowest slot and up to the top of the index, for the range at the top.
fn narrow_top() -> [(i128, i128); 1] {
    [(-300, 255)]
}

/// From the lowest slot to 300 past the highest, for the range at the bottom.
fn narrow_bottom() -> [(i128, i128); 1] {
    [(0, 555)]
}

/// Below the lowest slot and up to the top of the index, over 200 slots.
fn top_of_200() -> [(i128, i128); 1] {
    [(-300, 199)]
}

/// From the lowest slot to 300 past the highest, over 200 slots.
fn bottom_of_200() -> [(i128, i128); 1] {
    [(0, 499)]
}

/// Around the lowest slot, and the 300 slots up to the top of the index.
fn wide_top() -> [(i128, i128); 2] {
    let top = span_of_64_bits() - 1;
    [(-300, 300), (top - 300, top)]
}

/// From the lowest slot, and 300 either side of the highest.
fn wide_bottom() -> [(i128, i128); 2] {
    let top = span_of_64_bits() - 1;
    [(0, 300), (top - 300, top + 300)]
}

/// Remainders on the grid, a quarter, a tie and three quarters.
fn parts() -> [Fraction; 4] {
    [Fraction::ZERO, Fraction::of(1, 4), Fraction::HALF, Fraction::of(3, 4)]
}

/// No dither, and one either side of the quarter the stochastic mode reads.
fn dithers() -> [Dither; 3] {
    [Dither::UNUSED, Dither::at(Fraction::of(1, 8)), Dither::at(Fraction::of(5, 8))]
}

/// The first offset, remainder and dither at which `M` does not answer as `R`
/// shifted, in the slot or in the overflow verdict.
fn translates<R: DeclaredSignature, M: DeclaredSignature>(
    bands: &[(i128, i128)],
) -> Maybe<(i128, Fraction, Dither)> {
    let low_r = <<R::Format as Format>::Slots as Slots>::MIN.index();
    let low_m = <<M::Format as Format>::Slots as Slots>::MIN.index();
    let shift = low_m - low_r;
    for &(from, to) in bands {
        for offset in from ..= to {
            for part in parts() {
                for dither in dithers() {
                    let r = Exact::between(Slot::at(low_r + offset), part);
                    let m = Exact::between(Slot::at(low_m + offset), part);
                    if adapt::<M>(m, dither).index() - shift != adapt::<R>(r, dither).index()
                        || panic_on_overflow::<M>(m, dither) != panic_on_overflow::<R>(r, dither)
                    {
                        return Maybe::Is((offset, part, dither));
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

/// The law over all eighteen declared signatures for one pair of ranges.
macro_rules! every_signature {
    ($reference:ty, $moved:ty, $bands:expr) => {
        every_signature!(@modes $reference, $moved, $bands;
            Floor, Ceil, TowardZero, HalfUp, HalfEven, Stochastic);
    };
    (@modes $reference:ty, $moved:ty, $bands:expr; $($mode:ty),+) => {
        $( every_signature!(@policies $reference, $moved, $bands, $mode; Wrap, Saturate, Clamp); )+
    };
    (@policies $reference:ty, $moved:ty, $bands:expr, $mode:ty; $($policy:ty),+) => {
        $(
            assert_eq!(
                translates::<
                    Signature<Over<$reference>, Adapt<$mode, $policy>>,
                    Signature<Over<$moved>, Adapt<$mode, $policy>>,
                >(&$bands),
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
fn the_range_at_the_top_adapts_as_its_reference_does() {
    every_signature!(Reference, AtTheTop, narrow_top());
}

#[test]
fn the_range_at_the_bottom_adapts_as_its_reference_does() {
    every_signature!(NegativeReference, AtTheBottom, narrow_bottom());
}

#[test]
fn the_widest_range_at_the_top_adapts_as_its_reference_does() {
    every_signature!(WideReference, WideTop, wide_top());
}

#[test]
fn the_widest_range_at_the_bottom_adapts_as_its_reference_does() {
    every_signature!(WideNegativeReference, WideBottom, wide_bottom());
}

#[test]
fn the_range_of_200_at_the_top_adapts_as_its_reference_does() {
    // A span that is not a power of two, where a reduction that leans on the
    // span dividing `2^128` goes wrong and the spans above cannot show it.
    every_signature!(Reference200, TopOf200, top_of_200());
}

#[test]
fn the_range_of_200_at_the_bottom_adapts_as_its_reference_does() {
    every_signature!(NegativeReference200, BottomOf200, bottom_of_200());
}

/// Every pair the law is fed, each reference beside the range it is compared with.
pub(super) fn pairs() -> [((Slot, Slot), (Slot, Slot)); 6] {
    [
        (range_of::<Reference>(), range_of::<AtTheTop>()),
        (range_of::<NegativeReference>(), range_of::<AtTheBottom>()),
        (range_of::<WideReference>(), range_of::<WideTop>()),
        (
            range_of::<WideNegativeReference>(),
            range_of::<WideBottom>(),
        ),
        (range_of::<Reference200>(), range_of::<TopOf200>()),
        (
            range_of::<NegativeReference200>(),
            range_of::<BottomOf200>(),
        ),
    ]
}

#[test]
fn every_shift_is_even_and_every_reference_is_off_the_span() {
    // What the law above leans on, checked rather than assumed: an odd shift
    // would move half-even's even neighbour, and a reference whose lowest slot
    // has the moved range's residue modulo the span could not tell a wrap from
    // the lowest slot from one from zero. Each reference is also off a multiple
    // of its span, and keeps one sign with the range it is compared with.
    for (reference, moved) in pairs() {
        let span = moved.1.index() - moved.0.index() + 1;
        assert_eq!(span, reference.1.index() - reference.0.index() + 1);
        assert_eq!((moved.0.index() - reference.0.index()) % 2, 0);
        assert_ne!(reference.0.index().rem_euclid(span), 0);
        assert_ne!(
            reference.0.index().rem_euclid(span),
            moved.0.index().rem_euclid(span)
        );
        assert_eq!(reference.0.index() < 0, moved.0.index() < 0);
    }
}

// --- the law as a function of the map ----------------------------------------

/// Where a map first disagrees with itself shifted, if anywhere.
type Break = Maybe<(Mode, Policy, i128, Fraction, Dither)>;

/// A slot range's two ends.
pub(super) fn range_of<S: Slots>() -> (Slot, Slot) {
    (S::MIN, S::MAX)
}

/// The first mode, policy, offset, remainder and dither at which `map` over
/// `moved` does not answer as `map` over `reference` shifted.
fn first_break(
    map: Map,
    reference: (Slot, Slot),
    moved: (Slot, Slot),
    bands: &[(i128, i128)],
) -> Break {
    let shift = moved.0.index() - reference.0.index();
    for mode in ALL_MODES {
        for policy in SHIPPED_POLICIES {
            for &(from, to) in bands {
                for offset in from ..= to {
                    for part in parts() {
                        for dither in dithers() {
                            let r = round_slot(
                                mode,
                                Exact::between(Slot::at(reference.0.index() + offset), part),
                                dither,
                            );
                            let m = round_slot(
                                mode,
                                Exact::between(Slot::at(moved.0.index() + offset), part),
                                dither,
                            );
                            let slot_m = (map.complete)(policy, m, moved.0, moved.1).index();
                            let slot_r =
                                (map.complete)(policy, r, reference.0, reference.1).index();
                            let verdict_m = (map.leaves)(m, moved.0, moved.1);
                            let verdict_r = (map.leaves)(r, reference.0, reference.1);
                            if slot_m - shift != slot_r || verdict_m != verdict_r {
                                return Maybe::Is((mode, policy, offset, part, dither));
                            }
                        }
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

/// The law as the four pairs above feed it.
fn new_law(map: Map) -> Break {
    first_break(
        map,
        range_of::<Reference>(),
        range_of::<AtTheTop>(),
        &narrow_top(),
    )
    .or_else(|| {
        first_break(
            map,
            range_of::<NegativeReference>(),
            range_of::<AtTheBottom>(),
            &narrow_bottom(),
        )
    })
    .or_else(|| {
        first_break(
            map,
            range_of::<WideReference>(),
            range_of::<WideTop>(),
            &wide_top(),
        )
    })
    .or_else(|| {
        first_break(
            map,
            range_of::<WideNegativeReference>(),
            range_of::<WideBottom>(),
            &wide_bottom(),
        )
    })
    .or_else(|| {
        first_break(
            map,
            range_of::<Reference200>(),
            range_of::<TopOf200>(),
            &top_of_200(),
        )
    })
    .or_else(|| {
        first_break(
            map,
            range_of::<NegativeReference200>(),
            range_of::<BottomOf200>(),
            &bottom_of_200(),
        )
    })
}

#[test]
fn the_law_holds_of_the_shipped_map_and_reports_each_broken_one() {
    // The positive control first: a law reporting the shipped map would be
    // reporting on its own instrument.
    assert_eq!(new_law(shipped()), Maybe::Isnt);

    // A wrap from zero, which the references off the moved ranges' residues
    // report.
    assert!(new_law(anchored_at_zero()).is());

    // A wrap reducing the lowest slot modulo 256 is right at a span of 256 and
    // wrong everywhere else, so the ranges of 256 pass it and the ranges of 200
    // report it at both ends.
    let bad = reduced_modulo_256();
    assert!(new_law(bad).is());
    let narrow = [
        (
            range_of::<Reference>(),
            range_of::<AtTheTop>(),
            narrow_top(),
        ),
        (
            range_of::<NegativeReference>(),
            range_of::<AtTheBottom>(),
            narrow_bottom(),
        ),
    ];
    for (reference, moved, bands) in narrow {
        assert_eq!(first_break(bad, reference, moved, &bands), Maybe::Isnt);
    }
    let of_200 = [
        (
            range_of::<Reference200>(),
            range_of::<TopOf200>(),
            top_of_200(),
        ),
        (
            range_of::<NegativeReference200>(),
            range_of::<BottomOf200>(),
            bottom_of_200(),
        ),
    ];
    for (reference, moved, bands) in of_200 {
        assert!(first_break(bad, reference, moved, &bands).is());
    }

    // Subtracting first only goes wrong where the difference leaves the index,
    // which no translation within one end reaches, so this law cannot see it;
    // `the_far_end_of_the_index.rs` can. Stated here so nobody reads the law as
    // covering it.
    assert_eq!(new_law(subtracts_first()), Maybe::Isnt);
}

#[test]
fn a_verdict_missing_the_step_onto_the_lowest_slot_is_the_same_at_both_ends() {
    // A defect that is itself translation-invariant is invisible to the law
    // above however it is fed, and a verdict with no step onto the lowest slot is
    // one: it is wrong in the same place relative to every range. So the law
    // passes it, and what reports it is the position just under a range's
    // lowest slot stepping onto it, which the shipped verdict calls in range and
    // the broken one does not. Every range here with a slot under it.
    let (good, bad) = (shipped(), no_step_onto_the_lowest());
    assert_eq!(new_law(bad), Maybe::Isnt);
    for (min, max) in [
        range_of::<AtTheTop>(),
        range_of::<WideTop>(),
        range_of::<Reference>(),
        range_of::<NegativeReference>(),
        range_of::<WideReference>(),
        range_of::<WideNegativeReference>(),
    ] {
        let under = Exact::between(Slot::at(min.index() - 1), Fraction::of(1, 4));
        let stepped = round_slot(Mode::Ceil, under, Dither::UNUSED);
        assert!(!(good.leaves)(stepped, min, max));
        assert!((bad.leaves)(stepped, min, max));
        assert_eq!((good.complete)(Policy::Wrap, stepped, min, max), min);
    }
}
