//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The applied map against the oracle, over a wider region than
//! `the_oracle_sweep.rs` feeds.
//!
//! That sweep feeds positions at most eight whole slots from an end, at quarter
//! remainders, into fifteen ranges. This one feeds distances up to `2^63` past
//! either end of the index, the furthest `Exact::between` can name, over the
//! denominators 1, 2, 3, 4, 7 and `i64::MAX`, into ranges one slot wide at zero
//! and at `[5, 5]` and at both ends of the index, the ranges `[0, 2^64 - 1]` and
//! `[-2^63, 2^63 - 1]`, the `2^64`-slot ranges at both ends of the index,
//! `[-4, 3]`, a 200-slot range off its own span, `[-100, 99]`, and a 256-slot
//! range, `[1000, 1255]`. Each position is written from both ends of the index,
//! from each end of the range, from one past each end of the range, and from
//! zero.
//!
//! Beside those it feeds the floors a formed step reaches past where
//! `Exact::between` can name: `±2^128`, `±2^192` and `±2^255` with the same
//! remainders over the same denominators, and `±(2^256 - 1)` on the grid, every
//! one into every range.
//!
//! It runs the shipped map and every broken map `the_broken_maps.rs` keeps over
//! that region, the planted tie rules among them, and runs `adapt` and
//! `panic_on_overflow` at `USize` and `ISize` under every mode and policy. The
//! oracle's one reading taken from the implementation, `Clamp` as `Saturate`,
//! holds here as it does in `the_oracle.rs`.

use core::cmp::Ordering;

use notko::Maybe;

use super::the_broken_maps::{
    Map,
    a_step_up_past_the_bottom_pins_high,
    anchored_at_zero,
    half_even_reads_a_narrowed_parity,
    narrows_the_floor,
    no_lo_guard_onto_the_lowest,
    no_step_onto_the_lowest,
    past_the_bottom_pins_high_off_the_bottom,
    past_the_top_pins_low_off_the_top,
    planted_tie_rules,
    reduced_modulo_256,
    saturates_past_the_index,
    shipped,
    subtracts_first,
    the_step_onto_the_bottom_pins_high,
};
use super::the_oracle::{Point, Wide, answer};
use crate::adapt::DeclaredSignature;
use crate::apply::{Dither, Exact, Fraction, Remainder, adapt, panic_on_overflow, round_slot};
use crate::format::Format;
use crate::overflow::{Policy, SHIPPED_POLICIES};
use crate::points::{ISize, USize};
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Slot, Slots};
use crate::tests::big::Big;
use crate::tests::dispatch::{self, PerSignature};
use crate::wide::Wide as Limbs;

/// `2^64`.
const fn span_64() -> i128 {
    1 << 64
}

/// Every range the map sweep feeds, as its lowest and highest slot.
fn ranges() -> [(i128, i128); 11] {
    [
        (0, 0),
        (5, 5),
        (i128::MIN, i128::MIN),
        (i128::MAX, i128::MAX),
        (0, span_64() - 1),
        (-(1 << 63), (1 << 63) - 1),
        (i128::MAX - span_64() + 1, i128::MAX),
        (i128::MIN, i128::MIN + span_64() - 1),
        (-4, 3),
        (-100, 99),
        (1000, 1255),
    ]
}

/// The slots a position is written from, for a range `[lo, hi]`.
fn anchors(lo: i128, hi: i128) -> [i128; 9] {
    [
        i128::MIN,
        i128::MIN + 1,
        i128::MAX,
        i128::MAX - 1,
        lo,
        hi,
        0,
        lo.wrapping_sub(1),
        hi.wrapping_add(1),
    ]
}

/// The denominators a remainder is written over.
fn denominators() -> [i64; 6] {
    [1, 2, 3, 4, 7, i64::MAX]
}

/// Whole slots from the anchor, out to a quarter of the numerator's range each
/// way, with the steps one past the span of the 200-slot and 256-slot ranges
/// `ranges()` feeds, so a wrap reduction that only breaks past such a span is
/// exercised.
fn wholes() -> [i64; 17] {
    [
        i64::MIN / 2,
        -(1 << 62),
        -1000,
        -257,
        -201,
        -40,
        -9,
        -1,
        0,
        1,
        9,
        40,
        201,
        257,
        1000,
        1 << 62,
        i64::MAX / 2,
    ]
}

/// Every ratio `num / den` the sweep writes a position with: each whole number
/// of slots above at the remainders zero, one, half the denominator and one
/// under it, wherever the numerator fits, and the numerator's own two ends over
/// each denominator, which reach furthest past the anchor.
fn ratios() -> impl Iterator<Item = (i64, i64)> {
    denominators().into_iter().flat_map(|den| {
        let remainders = [0, 1, den / 2, den - 1];
        let distinct =
            move |i: usize| remainders[i] < den && !remainders[.. i].contains(&remainders[i]);
        wholes()
            .into_iter()
            .flat_map(move |whole| {
                (0 .. remainders.len())
                    .filter(move |&i| distinct(i))
                    .map(move |i| (whole as i128) * (den as i128) + remainders[i] as i128)
            })
            .filter(|&num| num >= i64::MIN as i128 && num <= i64::MAX as i128)
            .map(move |num| (num as i64, den))
            .chain([(i64::MIN, den), (i64::MAX, den)])
    })
}

/// The position `anchor + num / den` as the oracle reads it.
fn point(anchor: i128, num: i64, den: i64) -> Point {
    Point {
        whole: Wide::of(anchor).plus(Wide::of(num.div_euclid(den) as i128)),
        num:   i128::from(num.rem_euclid(den)),
        den:   i128::from(den),
    }
}

/// The floors past the index a formed step reaches, named, and whether a
/// remainder is written beside them: `±2^128`, `±2^192` and `±2^255` with one,
/// and `±(2^256 - 1)` on the grid only, since a step whose remainder is not
/// zero has a floor of at most `2^255`.
fn far_floors() -> [(&'static str, Big, bool); 8] {
    let up = |k| Big::power(2, k);
    let end = up(255).plus(up(255).plus(Big::of(-1)));
    [
        ("2^128", up(128), true),
        ("-2^128", up(128).negated(), true),
        ("2^192", up(192), true),
        ("-2^192", up(192).negated(), true),
        ("2^255", up(255), true),
        ("-2^255", up(255).negated(), true),
        ("2^256 - 1", end, false),
        ("-(2^256 - 1)", end.negated(), false),
    ]
}

/// Where a case's position was written from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Origin {
    /// `anchor + num / den`.
    Anchor(i128, (i64, i64)),
    /// A named far floor and `num / den` past it, `num` below `den`.
    Far(&'static str, (i64, i64)),
}

/// Every position written from an anchor of `[lo, hi]`.
fn near(lo: i128, hi: i128) -> impl Iterator<Item = (Origin, Exact, Point)> {
    anchors(lo, hi).into_iter().flat_map(|anchor| {
        ratios().map(move |(num, den)| {
            (
                Origin::Anchor(anchor, (num, den)),
                Exact::between(Slot::at(anchor), Fraction::of(num, den)),
                point(anchor, num, den),
            )
        })
    })
}

/// Every position at a far floor: at the remainders zero, one, half the
/// denominator and one under it, over each denominator, where the floor takes
/// a remainder, and the floor alone where it does not.
fn far() -> impl Iterator<Item = (Origin, Exact, Point)> {
    far_floors()
        .into_iter()
        .flat_map(|(name, floor, off_grid)| {
            denominators().into_iter().flat_map(move |den| {
                let remainders = [0, 1, den / 2, den - 1];
                (0 .. remainders.len())
                    .filter(move |&i| {
                        remainders[i] < den && !remainders[.. i].contains(&remainders[i])
                    })
                    .map(move |i| remainders[i])
                    .filter(move |&num| off_grid || (num == 0 && den == 1))
                    .map(move |num| {
                        let part = Remainder::reduced(Limbs::of(num.into()), Limbs::of(den.into()));
                        (
                            Origin::Far(name, (num, den)),
                            Exact::floored(floor.wide(), part),
                            Point {
                                whole: Wide::of_big(floor),
                                num:   num.into(),
                                den:   den.into(),
                            },
                        )
                    })
            })
        })
}

/// The dithers, in eighths.
fn eighths() -> [i64; 4] {
    [0, 1, 3, 7]
}

/// One position fed into one range, and what came back against what the
/// oracle wants.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Case {
    mode:   Mode,
    policy: Policy,
    range:  (i128, i128),
    origin: Origin,
    eighth: i64,
    got:    (i128, bool),
    want:   (i128, bool),
}

/// The first case over `[lo, hi]` at which `ask` disagrees with the oracle,
/// or `Isnt`.
fn first_disagreement(
    modes: &[Mode],
    policies: &[Policy],
    (lo, hi): (i128, i128),
    ask: impl Fn(Mode, Policy, Exact, Dither) -> (Slot, bool),
) -> Maybe<Case> {
    for (origin, exact, p) in near(lo, hi).chain(far()) {
        for eighth in eighths() {
            let dither = Dither::at(Fraction::of(eighth, 8));
            for &mode in modes {
                for &policy in policies {
                    let want = answer(mode, policy, p, (eighth, 8), lo, hi);
                    let (slot, leaves) = ask(mode, policy, exact, dither);
                    if (slot.index(), leaves) != want {
                        return Maybe::Is(Case {
                            mode,
                            policy,
                            range: (lo, hi),
                            origin,
                            eighth,
                            got: (slot.index(), leaves),
                            want,
                        });
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

/// The first disagreement of `map` with the oracle over every range, mode and
/// policy, through the map's own rounding.
fn map_break(map: Map) -> Maybe<Case> {
    for (lo, hi) in ranges() {
        let found = first_disagreement(
            &ALL_MODES,
            &SHIPPED_POLICIES,
            (lo, hi),
            |mode, policy, e, d| {
                let rounded = (map.round)(mode, e, d);
                (
                    (map.complete)(policy, rounded, Slot::at(lo), Slot::at(hi)),
                    (map.leaves)(rounded, Slot::at(lo), Slot::at(hi)),
                )
            },
        );
        if found.is() {
            return found;
        }
    }
    Maybe::Isnt
}

/// The sweep at one declared signature, through `adapt` and
/// `panic_on_overflow`.
struct Surface {
    mode:   Mode,
    policy: Policy,
}

impl PerSignature for Surface {
    type Out = Maybe<Case>;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> Maybe<Case> {
        let range = (
            <<S::Format as Format>::Slots as Slots>::MIN.index(),
            <<S::Format as Format>::Slots as Slots>::MAX.index(),
        );
        first_disagreement(&[self.mode], &[self.policy], range, |_, _, e, d| {
            (adapt::<S>(e, d), panic_on_overflow::<S>(e, d).get())
        })
    }
}

/// The first disagreement at any of the eighteen signatures over `F`.
fn surface_break<F: Format>() -> Maybe<Case> {
    for mode in ALL_MODES {
        for policy in SHIPPED_POLICIES {
            let found = dispatch::at::<F, Surface>(mode, policy, &Surface {
                mode,
                policy,
            });
            if found.is() {
                return found;
            }
        }
    }
    Maybe::Isnt
}

#[test]
fn the_shipped_map_answers_as_the_oracle_does_over_the_wide_region() {
    assert_eq!(map_break(shipped()), Maybe::Isnt);
}

#[test]
fn the_platform_width_points_answer_as_the_oracle_does() {
    assert_eq!(surface_break::<USize>(), Maybe::Isnt, "USize");
    assert_eq!(surface_break::<ISize>(), Maybe::Isnt, "ISize");
}

#[test]
fn the_wide_sweep_reports_every_broken_map() {
    let completions: [(&'static str, Map); 12] = [
        ("subtracts_first", subtracts_first()),
        ("anchored_at_zero", anchored_at_zero()),
        ("reduced_modulo_256", reduced_modulo_256()),
        ("no_step_onto_the_lowest", no_step_onto_the_lowest()),
        (
            "past_the_bottom_pins_high_off_the_bottom",
            past_the_bottom_pins_high_off_the_bottom(),
        ),
        (
            "past_the_top_pins_low_off_the_top",
            past_the_top_pins_low_off_the_top(),
        ),
        (
            "a_step_up_past_the_bottom_pins_high",
            a_step_up_past_the_bottom_pins_high(),
        ),
        (
            "the_step_onto_the_bottom_pins_high",
            the_step_onto_the_bottom_pins_high(),
        ),
        ("no_lo_guard_onto_the_lowest", no_lo_guard_onto_the_lowest()),
        ("narrows_the_floor", narrows_the_floor()),
        ("saturates_past_the_index", saturates_past_the_index()),
        (
            "half_even_reads_a_narrowed_parity",
            half_even_reads_a_narrowed_parity(),
        ),
    ];
    let mut asked = 0;
    for (name, map) in completions.into_iter().chain(planted_tie_rules()) {
        asked += 1;
        assert!(map_break(map).is(), "{name} was not reported");
    }
    assert_eq!(asked, 17, "a broken map dropped out of the list");
}

#[test]
fn the_wide_feed_reaches_past_either_end_of_the_index_by_more_than_2_to_the_62() {
    // The sweep is only about the far region if its positions are in it.
    // Counted as the floor `round_slot` returns against the index's ends, over every
    // anchor of `[-4, 3]` and every ratio, under `Floor`.
    let far = 1i64 << 62;
    let (mut above, mut below, mut odd_denominator) = (0, 0, 0);
    for anchor in anchors(-4, 3) {
        for (num, den) in ratios() {
            let r = round_slot(
                Mode::Floor,
                Exact::between(Slot::at(anchor), Fraction::of(num, den)),
                Dither::UNUSED,
            );
            above += usize::from(
                r.down.ordering(Limbs::of_sum(i128::MAX, i128::from(far))) == Ordering::Greater,
            );
            below += usize::from(
                r.down.ordering(Limbs::of_sum(i128::MIN, -i128::from(far))) == Ordering::Less,
            );
            odd_denominator += usize::from(den == 7 && num.rem_euclid(den) != 0);
        }
    }
    assert!(above > 0, "the feed never reaches 2^62 past the top");
    assert!(below > 0, "the feed never reaches 2^62 past the bottom");
    assert!(
        odd_denominator > 0,
        "the feed never writes an off-grid seventh"
    );
}

#[test]
fn the_far_feed_reaches_the_floors_a_formed_step_reaches() {
    // Counted as the floor and the step `round_slot` returns under `Ceil`, which
    // steps up wherever a remainder was written, against the four-limb
    // integer's two ends and against `±2^255` formed as a power of two.
    let half = |negative| {
        let Maybe::Is(v) = Limbs::power(2, 255) else { unreachable!() };
        if negative { v.negated() } else { v }
    };
    let (mut ends, mut halves_off_grid) = (0, 0);
    for (_, exact, _) in far() {
        let r = round_slot(Mode::Ceil, exact, Dither::UNUSED);
        ends += usize::from(r.down == Limbs::end(false) || r.down == Limbs::end(true));
        halves_off_grid +=
            usize::from((r.down == half(false) || r.down == half(true)) && r.step() == 1);
    }
    assert_eq!(
        ends, 2,
        "the feed does not reach both ends of the four limbs"
    );
    assert!(
        halves_off_grid >= 2,
        "the feed writes no remainder past 2^255"
    );
}
