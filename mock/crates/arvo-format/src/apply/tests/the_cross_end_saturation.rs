//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Cross-end coverage for the Saturate/Clamp arm of `complete_slot`.
//!
//! `146d`'s F1 asked for "every carried position into every range", cross-end.
//! The carry-past-the-index suite covers a position carried past one end fed
//! into a range at that SAME end. What it leaves unguarded is the far side: a
//! position carried past the bottom of the index, fed into a range sitting at
//! the top, and the mirror. `complete_slot`'s Saturate/Clamp arm reads
//! `rounded.past < 0` alone to decide the near-bottom case and
//! `rounded.past > 0` by elimination for the near-top one, with no read of
//! where the fed range itself sits, which is what makes that arm correct
//! cross-end in the first place: the decision does not depend on the range's
//! own position. A mutant reintroducing such a dependency (`the_broken_maps.rs`'s
//! `m1_no_far_lo_guard`, `m2_no_far_hi_guard`, named `M1` and `M2` in the fifth
//! review) passes the whole suite without this file, because every existing
//! hand test that feeds a carried position also happens to feed a range at the
//! same end.

use notko::Maybe;

use super::the_broken_maps::{Map, m1_no_far_lo_guard, m2_no_far_hi_guard, shipped};
use super::the_far_end_of_the_index::{BottomOf200, TopOf200};
use super::the_translation_law::{WideBottom, WideTop, range_of};
use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, Fraction, adapt, round_slot};
use crate::overflow::{Policy, Saturate};
use crate::points::Integer;
use crate::rounding::{ALL_MODES, Ceil, Floor, Mode};
use crate::slots::Slot;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};

/// The first policy, range and depth at which a break was found, or `Isnt`.
type Break = Maybe<(Policy, Slot, Slot, i64)>;

/// `Integer<3>`'s own range, `[-4, 3]`, at neither end of the index. Named
/// directly rather than through the translation law's reference ranges, which
/// are all wider than this one on purpose.
fn integer_3() -> (Slot, Slot) {
    (Slot::at(-4), Slot::at(3))
}

/// A position carried `k` whole slots past the bottom of the index, on the
/// grid, per `Exact::between`'s own carry rule.
fn past_the_bottom(k: i64) -> Exact {
    Exact::between(Slot::at(i128::MIN), Fraction::of(-4 * k, 4))
}

/// A position carried `k` whole slots past the top of the index.
fn past_the_top(k: i64) -> Exact {
    Exact::between(Slot::at(i128::MAX), Fraction::of(4 * k, 4))
}

/// Ranges at the top of the index, none starting at `i128::MIN`, plus a range
/// at neither end.
fn ranges_at_the_top() -> [(Slot, Slot); 4] {
    [
        range_of::<AtTheTop>(),
        range_of::<WideTop>(),
        range_of::<TopOf200>(),
        integer_3(),
    ]
}

/// Ranges at the bottom of the index, none ending at `i128::MAX`, plus a range
/// at neither end.
fn ranges_at_the_bottom() -> [(Slot, Slot); 4] {
    [
        range_of::<AtTheBottom>(),
        range_of::<WideBottom>(),
        range_of::<BottomOf200>(),
        integer_3(),
    ]
}

/// The first policy, range and depth at which `map`'s completion disagrees
/// with pinning at the range's own lowest slot, over positions carried past
/// the bottom of the index and fed into ranges at the top.
fn first_break_from_the_bottom(map: Map) -> Break {
    for policy in [Policy::Saturate, Policy::Clamp] {
        for (min, max) in ranges_at_the_top() {
            for k in 1 ..= 5 {
                for mode in ALL_MODES {
                    let rounded = round_slot(mode, past_the_bottom(k), Dither::UNUSED);
                    if (map.complete)(policy, rounded, min, max) != min {
                        return Maybe::Is((policy, min, max, k));
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

/// The mirror: positions carried past the top, fed into ranges at the bottom,
/// against the range's own highest slot.
fn first_break_from_the_top(map: Map) -> Break {
    for policy in [Policy::Saturate, Policy::Clamp] {
        for (min, max) in ranges_at_the_bottom() {
            for k in 1 ..= 5 {
                for mode in ALL_MODES {
                    let rounded = round_slot(mode, past_the_top(k), Dither::UNUSED);
                    if (map.complete)(policy, rounded, min, max) != max {
                        return Maybe::Is((policy, min, max, k));
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

#[test]
fn the_shipped_map_pins_at_the_near_end_regardless_of_where_the_far_range_sits() {
    // The positive control: the law reporting the shipped map would be
    // reporting on its own instrument.
    assert_eq!(first_break_from_the_bottom(shipped()), Maybe::Isnt);
    assert_eq!(first_break_from_the_top(shipped()), Maybe::Isnt);
}

#[test]
fn the_law_reports_the_missing_far_lo_guard() {
    assert!(
        first_break_from_the_bottom(m1_no_far_lo_guard()).is(),
        "M1 was not reported"
    );

    // The positive control on the mutant itself: at `AtTheBottom`, whose own
    // lowest slot is `i128::MIN`, the extra conjunct never fires, so the
    // mutant is not simply always wrong.
    let (min, max) = range_of::<AtTheBottom>();
    for k in 1 ..= 5 {
        for mode in ALL_MODES {
            let rounded = round_slot(mode, past_the_bottom(k), Dither::UNUSED);
            assert_eq!(
                (shipped().complete)(Policy::Saturate, rounded, min, max),
                (m1_no_far_lo_guard().complete)(Policy::Saturate, rounded, min, max),
                "k={k}, {mode:?}"
            );
        }
    }

    // The mirror direction is untouched by this mutation.
    assert_eq!(first_break_from_the_top(m1_no_far_lo_guard()), Maybe::Isnt);
}

#[test]
fn the_law_reports_the_missing_far_hi_guard() {
    assert!(
        first_break_from_the_top(m2_no_far_hi_guard()).is(),
        "M2 was not reported"
    );

    let (min, max) = range_of::<AtTheTop>();
    for k in 1 ..= 5 {
        for mode in ALL_MODES {
            let rounded = round_slot(mode, past_the_top(k), Dither::UNUSED);
            assert_eq!(
                (shipped().complete)(Policy::Saturate, rounded, min, max),
                (m2_no_far_hi_guard().complete)(Policy::Saturate, rounded, min, max),
                "k={k}, {mode:?}"
            );
        }
    }

    assert_eq!(
        first_break_from_the_bottom(m2_no_far_hi_guard()),
        Maybe::Isnt
    );
}

#[test]
fn the_failure_scenario_the_review_named() {
    // `adapt::<Signature<Integer<3>, Adapt<Floor, Saturate>>>(Exact::between(
    // Slot::at(i128::MIN), Fraction::of(-9, 4)), ..)`: two whole slots past the
    // bottom and a quarter, into `Integer<3>`, `[-4, 3]`. The shipped map
    // answers `-4`; `m1_no_far_lo_guard` answers `3`.
    type S = Signature<Integer<3>, Adapt<Floor, Saturate>>;
    let exact = Exact::between(Slot::at(i128::MIN), Fraction::of(-9, 4));
    assert_eq!(adapt::<S>(exact, Dither::UNUSED), Slot::at(-4));

    let (min, max) = integer_3();
    let rounded = round_slot(Mode::Floor, exact, Dither::UNUSED);
    assert_eq!(
        (shipped().complete)(Policy::Saturate, rounded, min, max),
        Slot::at(-4)
    );
    assert_eq!(
        (m1_no_far_lo_guard().complete)(Policy::Saturate, rounded, min, max),
        Slot::at(3)
    );

    // And the mirror past the top.
    type Mirror = Signature<Integer<3>, Adapt<Ceil, Saturate>>;
    let up = Exact::between(Slot::at(i128::MAX), Fraction::of(9, 4));
    assert_eq!(adapt::<Mirror>(up, Dither::UNUSED), Slot::at(3));
    let rounded_up = round_slot(Mode::Ceil, up, Dither::UNUSED);
    assert_eq!(
        (shipped().complete)(Policy::Saturate, rounded_up, min, max),
        Slot::at(3)
    );
    assert_eq!(
        (m2_no_far_hi_guard().complete)(Policy::Saturate, rounded_up, min, max),
        Slot::at(-4)
    );
}
