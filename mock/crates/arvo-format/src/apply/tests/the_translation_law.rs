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
//! is asked about, so the broken maps it has to report live here as tests rather
//! than as edits somebody made once and reverted.

use notko::Maybe;

use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{
    Dither,
    Exact,
    Fraction,
    Rounded,
    adapt,
    complete_slot,
    panic_on_overflow,
    round_slot,
};
use crate::format::Format;
use crate::overflow::{Clamp, Policy, SHIPPED_POLICIES, Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};
use crate::slots::{Slot, Slots, Unsigned};
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};
use crate::width::Width;

/// `2^64`, the widest span an admitted range has.
const fn span_of_64_bits() -> i128 {
    1 << 64
}

/// The reference for the range at the top: 256 slots from 1000.
struct Reference;

impl Slots for Reference {
    const MAX: Slot = Slot::at(1255);
    const MIN: Slot = Slot::at(1000);
    const WIDTH: Width = Width::bits(8);
}

/// The reference for the range at the bottom: 256 slots from -2000.
struct NegativeReference;

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
struct WideReference;

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
struct WideNegativeReference;

impl Slots for WideNegativeReference {
    const MAX: Slot = Slot::at(-span_of_64_bits() - 1001);
    const MIN: Slot = Slot::at(-2 * span_of_64_bits() - 1000);
    const WIDTH: Width = Width::bits(64);
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
fn every_shift_is_even_and_every_reference_is_off_the_span() {
    // What the law above leans on, checked rather than assumed: an odd shift
    // would move half-even's even neighbour, and a reference on a multiple of
    // its span could not tell a wrap from the lowest slot from one from zero.
    let pairs = [
        (range_of::<Reference>(), range_of::<AtTheTop>()),
        (range_of::<NegativeReference>(), range_of::<AtTheBottom>()),
        (range_of::<WideReference>(), range_of::<WideTop>()),
        (
            range_of::<WideNegativeReference>(),
            range_of::<WideBottom>(),
        ),
    ];
    for (reference, moved) in pairs {
        let span = moved.1.index() - moved.0.index() + 1;
        assert_eq!(span, reference.1.index() - reference.0.index() + 1);
        assert_eq!((moved.0.index() - reference.0.index()) % 2, 0);
        assert_ne!(reference.0.index().rem_euclid(span), 0);
        assert_eq!(moved.0.index().rem_euclid(span), 0);
    }
}

// --- the law as a function of the map, and the maps it has to refuse ---------

/// The two answers of one applied map the law compares.
#[derive(Clone, Copy)]
pub(super) struct Map {
    pub(super) complete: fn(Policy, Rounded, Slot, Slot) -> Slot,
    pub(super) leaves:   fn(Rounded, Slot, Slot) -> bool,
}

/// The shipped verdict, as `panic_on_overflow` asks it.
fn shipped_leaves(r: Rounded, min: Slot, max: Slot) -> bool {
    !r.lands_within(min.index(), max.index())
}

/// The map this crate ships.
pub(super) fn shipped() -> Map {
    Map {
        complete: complete_slot,
        leaves:   shipped_leaves,
    }
}

/// A wrap that subtracts the lowest slot before reducing, in the index's own
/// integer, wrapping where the difference leaves it, which is what the naive
/// subtraction does in a build without overflow checks.
pub(super) fn subtracts_first() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        Slot::at(
            lo + r
                .down()
                .wrapping_sub(lo)
                .wrapping_add(r.step())
                .rem_euclid(span),
        )
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// A wrap reduced from zero rather than from the lowest slot.
fn anchored_at_zero() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        Slot::at(lo + (r.down().rem_euclid(span) + r.step()).rem_euclid(span))
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// An overflow verdict whose in-range test has no step onto the lowest slot.
///
/// The slot half is the shipped one: out of range, every policy sends a step
/// onto the lowest slot to the lowest slot anyway, so only the verdict can show
/// the missing branch.
fn no_step_onto_the_lowest() -> Map {
    fn leaves(r: Rounded, min: Slot, max: Slot) -> bool {
        let (lo, hi) = (min.index(), max.index());
        !(r.down() >= lo && (r.down() < hi || (r.down() == hi && !r.up.get())))
    }
    Map {
        complete: complete_slot,
        leaves,
    }
}

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
}

/// The law as it was fed before: the range at zero, offsets 0 to 255.
fn old_law(map: Map) -> Break {
    first_break(map, range_of::<Unsigned<8>>(), range_of::<AtTheTop>(), &[(
        0, 255,
    )])
}

#[test]
fn the_law_holds_of_the_shipped_map_and_reports_each_broken_one() {
    // The positive control first: a law reporting the shipped map would be
    // reporting on its own instrument.
    assert_eq!(new_law(shipped()), Maybe::Isnt);
    assert_eq!(old_law(shipped()), Maybe::Isnt);

    // A wrap from zero passes the offsets the law used to be fed, where both
    // ranges sat on a multiple of their span, and the references off the span
    // report it.
    assert_eq!(old_law(anchored_at_zero()), Maybe::Isnt);
    assert!(new_law(anchored_at_zero()).is());

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
