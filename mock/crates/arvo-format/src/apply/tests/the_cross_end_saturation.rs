//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Cross-end coverage for the Saturate/Clamp arm of `complete_slot`.
//!
//! Every carried position fed into every range, cross-end. The carry-past-the-
//! index suite covers a position carried past one end fed into a range at that
//! SAME end. What it leaves unguarded is the far side: a position carried past
//! the bottom of the index, fed into a range sitting at the top, and the
//! mirror. `complete_slot`'s Saturate/Clamp arm reads `rounded.past < 0` alone
//! to decide the near-bottom case and `rounded.past > 0` by elimination for the
//! near-top one, with no read of where the fed range itself sits, which is
//! what makes that arm correct cross-end in the first place: the decision does
//! not depend on the range's own position. A mutant reintroducing such a
//! dependency (`the_broken_maps.rs`'s `m1_no_far_lo_guard`, `m2_no_far_hi_guard`)
//! passes the whole suite without this file, because every existing hand test
//! that feeds a carried position also happens to feed a range at the same end.
//!
//! Every position here is fed both on the grid, a whole number of slots past
//! the end, and off it, at a quarter, a half and three quarters of a slot past
//! it, over every rounding mode and every policy this arm covers: an on-grid
//! position alone leaves `round_slot`'s remainder-zero branch the only one this
//! file exercises, which is one of six.

use notko::Maybe;

use super::the_broken_maps::{
    Map,
    m1_no_far_lo_guard,
    m2_no_far_hi_guard,
    no_far_lo_guard_when_stepped_up,
    shipped,
};
use super::the_far_end_of_the_index::{BottomOf200, TopOf200};
use super::the_translation_law::{WideBottom, WideTop, range_of};
use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, Fraction, adapt, round_slot};
use crate::overflow::{Policy, Saturate, Wrap};
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

/// A position carried `k` whole slots plus `quarters` quarter-slots past the
/// bottom of the index, on the grid at `quarters = 0`, per `Exact::between`'s
/// own carry rule. 1, 2 and 3 are off the grid, a quarter, a half and three
/// quarters further down.
fn past_the_bottom_off_grid(k: i64, quarters: i64) -> Exact {
    Exact::between(Slot::at(i128::MIN), Fraction::of(-(4 * k) - quarters, 4))
}

/// The mirror of `past_the_bottom_off_grid`, past the top of the index.
fn past_the_top_off_grid(k: i64, quarters: i64) -> Exact {
    Exact::between(Slot::at(i128::MAX), Fraction::of(4 * k + quarters, 4))
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
                for quarters in 0 ..= 3 {
                    for mode in ALL_MODES {
                        let rounded =
                            round_slot(mode, past_the_bottom_off_grid(k, quarters), Dither::UNUSED);
                        if (map.complete)(policy, rounded, min, max) != min {
                            return Maybe::Is((policy, min, max, k));
                        }
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
                for quarters in 0 ..= 3 {
                    for mode in ALL_MODES {
                        let rounded =
                            round_slot(mode, past_the_top_off_grid(k, quarters), Dither::UNUSED);
                        if (map.complete)(policy, rounded, min, max) != max {
                            return Maybe::Is((policy, min, max, k));
                        }
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
        "the missing far lo guard was not reported"
    );

    // The positive control on the mutant itself: at `AtTheBottom`, whose own
    // lowest slot is `i128::MIN`, the extra conjunct never fires, so the
    // mutant is not simply always wrong.
    let (min, max) = range_of::<AtTheBottom>();
    for k in 1 ..= 5 {
        for quarters in 0 ..= 3 {
            for mode in ALL_MODES {
                let rounded =
                    round_slot(mode, past_the_bottom_off_grid(k, quarters), Dither::UNUSED);
                assert_eq!(
                    (shipped().complete)(Policy::Saturate, rounded, min, max),
                    (m1_no_far_lo_guard().complete)(Policy::Saturate, rounded, min, max),
                    "k={k}, quarters={quarters}, {mode:?}"
                );
            }
        }
    }

    // The mirror direction is untouched by this mutation.
    assert_eq!(first_break_from_the_top(m1_no_far_lo_guard()), Maybe::Isnt);
}

#[test]
fn the_law_reports_the_missing_far_hi_guard() {
    assert!(
        first_break_from_the_top(m2_no_far_hi_guard()).is(),
        "the missing far hi guard was not reported"
    );

    let (min, max) = range_of::<AtTheTop>();
    for k in 1 ..= 5 {
        for quarters in 0 ..= 3 {
            for mode in ALL_MODES {
                let rounded = round_slot(mode, past_the_top_off_grid(k, quarters), Dither::UNUSED);
                assert_eq!(
                    (shipped().complete)(Policy::Saturate, rounded, min, max),
                    (m2_no_far_hi_guard().complete)(Policy::Saturate, rounded, min, max),
                    "k={k}, quarters={quarters}, {mode:?}"
                );
            }
        }
    }

    assert_eq!(
        first_break_from_the_bottom(m2_no_far_hi_guard()),
        Maybe::Isnt
    );
}

#[test]
fn the_law_reports_the_missing_far_lo_guard_when_stepped_up() {
    // A second, independent way to drop the far-lo guard: conditional on
    // `rounded.up` rather than on the fed range's own bottom. An on-grid
    // position never sets `rounded.up` (`round_slot` returns `down` unchanged
    // when the remainder is zero), so this mutant is invisible to a matrix fed
    // only whole-slot positions; it needs the off-grid quarters this file's
    // matrix now feeds.
    assert!(
        first_break_from_the_bottom(no_far_lo_guard_when_stepped_up()).is(),
        "the missing far lo guard (stepped-up variant) was not reported"
    );

    // The positive control: at `AtTheBottom`, whose own lowest slot is
    // `i128::MIN`, the extra disjunct is always true, so the mutant is not
    // simply always wrong there.
    let (min, max) = range_of::<AtTheBottom>();
    for k in 1 ..= 5 {
        for quarters in 0 ..= 3 {
            for mode in ALL_MODES {
                let rounded =
                    round_slot(mode, past_the_bottom_off_grid(k, quarters), Dither::UNUSED);
                assert_eq!(
                    (shipped().complete)(Policy::Saturate, rounded, min, max),
                    (no_far_lo_guard_when_stepped_up().complete)(
                        Policy::Saturate,
                        rounded,
                        min,
                        max
                    ),
                    "k={k}, quarters={quarters}, {mode:?}"
                );
            }
        }
    }

    assert_eq!(
        first_break_from_the_top(no_far_lo_guard_when_stepped_up()),
        Maybe::Isnt
    );
}

#[test]
fn wrap_residues_hand_reduced_off_grid_into_integer_3() {
    // Hand-reduced Wrap answers for off-grid carried positions, over
    // `Integer<3>`'s own `[-4, 3]`, span 8, at neither end of the index.
    //
    // `Exact::between(Slot::at(i128::MIN), Fraction::of(-6, 4))` names a
    // position 1.5 slots below `i128::MIN`. Under `Floor` this rounds down to
    // 2 whole slots below (`i128::MIN` stays the pinned slot, `past = -2`,
    // `step = 0`). Reduced by hand: `i128::MIN` is exactly divisible by the
    // span 8, so `i128::MIN.rem_euclid(8) = 0`; `(-2i128).rem_euclid(8) = 6`;
    // `(-4i128).rem_euclid(8) = 4` (the range's own lowest slot, `-4`); the
    // offset is `0 + 6 - 4 + 0 = 2`, and the answer is `lo + 2 = -2`.
    type Bottom = Signature<Integer<3>, Adapt<Floor, Wrap>>;
    let bottom = Exact::between(Slot::at(i128::MIN), Fraction::of(-6, 4));
    assert_eq!(adapt::<Bottom>(bottom, Dither::UNUSED), Slot::at(-2));

    // The mirror: `Fraction::of(6, 4)` past `i128::MAX`, 1.5 slots above it.
    // Under `Ceil` this rounds up (`past = 1`, `step = 1`). Reduced by hand:
    // `i128::MAX.rem_euclid(8) = 7` (`i128::MAX = i128::MIN - 1` in two's
    // complement, one below a multiple of 8); `1i128.rem_euclid(8) = 1`; the
    // range's own lowest slot reduces the same as above, `4`; the offset is
    // `7 + 1 - 4 + 1 = 5`, and the answer is `lo + 5 = 1`.
    type Top = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    let top = Exact::between(Slot::at(i128::MAX), Fraction::of(6, 4));
    assert_eq!(adapt::<Top>(top, Dither::UNUSED), Slot::at(1));
}

#[test]
fn the_failure_scenario_the_missing_far_guard_produces() {
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
