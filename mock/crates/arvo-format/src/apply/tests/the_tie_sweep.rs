//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every signed tie, under every mode and policy, against two oracles.
//!
//! The other sweeps feed ties among everything else, at quarter remainders or at
//! half a denominator, so a tie rule is checked wherever a tie happens to land.
//! This one feeds nothing else. Every position is `s + 1/2`, written from the
//! slot below as `s + h/2h` and from the slot above as `(s + 1) - h/2h`, over the
//! denominators 2, 4 and `i64::MAX - 1`, so a rule reading the numerator, the
//! denominator or which neighbour the position was written from shows up as a
//! disagreement between two writings of one tie.
//!
//! Two oracles answer, and neither reads `round_slot`. The first is
//! `the_oracle::rounded`, which computes each mode from its formula over the
//! exact rational. The second is the tie table below, which states each mode's
//! answer at a tie and nothing else: `floor` goes to `s`, `ceil` and `half_up` to
//! `s + 1`, `toward_zero` to `s + 1` when `s + 1/2` is negative and to `s`
//! otherwise, `half_even` to whichever of the two is even, and `stochastic` to
//! `s + 1` at a dither below one half and to `s` from one half up. The two are
//! checked against each other first, so the sweep does not rest on one reading.
//!
//! The walk is exhaustive at every admitted width of both slot families whose
//! range spans at most `2^16` slots, which is widths one through sixteen, from
//! one tie under the range to one over it. Past that it walks the ties within
//! sixteen slots of either end of the range and of zero. The map is also run
//! directly at ranges pinned to both ends of the index, where the tie under
//! `i128::MIN` and the tie over `i128::MAX` are positions no format reaches, and
//! every planted tie rule `the_broken_maps.rs` keeps is run there and reported.

use notko::Maybe;

use super::the_broken_maps::{Map, planted_tie_rules, shipped};
use super::the_oracle::{Point, Wide, answer, completed, rounded};
use crate::adapt::DeclaredSignature;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow};
use crate::format::Format;
use crate::overflow::{Policy, SHIPPED_POLICIES};
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Slot, Slots};
use crate::tests::dispatch::{self, PerFormat, PerSignature};

/// The largest span, in slots, a range may have and still be walked whole.
///
/// A function rather than an item constant, for the reason the ratio
/// coordinate's suite gives: a const here is a coordinate spelled in the host's
/// own type, which the contract lint refuses in this crate.
fn exhaustive_span() -> i128 {
    1 << 16
}

/// The halves a tie is written with: `h / 2h` over the denominators 2, 4 and
/// `i64::MAX - 1`.
fn halves() -> [i64; 3] {
    [1, 2, (i64::MAX - 1) / 2]
}

/// The dithers a mode is asked at, in eighths.
///
/// Only the stochastic mode reads one, and it is asked below one half, at one
/// half exactly, and above it. The other five are asked at one dither, and
/// `five_modes_ignore_the_dither_and_one_does_not` is what says that suffices.
fn eighths(mode: Mode) -> &'static [i64] {
    match mode {
        Mode::Stochastic => &[0, 4, 7],
        _ => &[0],
    }
}

/// The slot a mode puts the tie `s + 1/2` on, stated at the tie and nowhere else.
fn tie_table(mode: Mode, s: Wide, eighth: i64) -> Wide {
    let up = s.plus(Wide::of(1));
    match mode {
        Mode::Floor => s,
        Mode::Ceil | Mode::HalfUp => up,
        Mode::TowardZero => {
            if s.is_negative() {
                up
            } else {
                s
            }
        },
        Mode::HalfEven => {
            if s.is_even() {
                s
            } else {
                up
            }
        },
        Mode::Stochastic => {
            if eighth < 4 {
                up
            } else {
                s
            }
        },
    }
}

/// One tie, as the map is asked it: the anchor slot and the signed half written
/// from it. The tie is `anchor + num / den`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Writing {
    anchor: i128,
    num:    i64,
    den:    i64,
}

impl Writing {
    /// The position the map reads.
    fn exact(self) -> Exact {
        Exact::between(Slot::at(self.anchor), Fraction::of(self.num, self.den))
    }

    /// The slot below the tie, as the oracles read it.
    fn below(self) -> Wide {
        if self.num < 0 {
            Wide::of(self.anchor).minus(Wide::of(1))
        } else {
            Wide::of(self.anchor)
        }
    }

    /// The tie as the formula oracle reads it: `below + h / 2h`.
    fn point(self) -> Point {
        Point {
            whole: self.below(),
            num:   self.num.abs(),
            den:   self.den,
        }
    }
}

/// Every writing of every tie next to `anchor`, from it and toward both sides.
fn writings_at(anchor: i128) -> impl Iterator<Item = Writing> {
    halves().into_iter().flat_map(move |h| {
        [h, -h].map(|num| {
            Writing {
                anchor,
                num,
                den: 2 * h,
            }
        })
    })
}

/// Whether `[lo, hi]` is walked whole.
fn walked_whole(lo: i128, hi: i128) -> bool {
    hi.checked_sub(lo).is_some_and(|d| d < exhaustive_span())
}

/// The anchors a range `[lo, hi]` is walked from.
///
/// Every slot from one under the range to one over it where the range is walked
/// whole, and otherwise the slots within sixteen of either end and of zero. An
/// anchor past the index is skipped rather than formed; the ties beyond the
/// index's ends are reached from the end slot itself, whose writing toward the
/// outside names them.
fn anchors(lo: i128, hi: i128) -> impl Iterator<Item = i128> {
    let bands: [(i128, i128, i128); 3] = if walked_whole(lo, hi) {
        [(lo, -1, hi - lo + 1), (0, 0, -1), (0, 0, -1)]
    } else {
        [(lo, -1, 17), (0, -17, 17), (hi, -16, 1)]
    };
    bands
        .into_iter()
        .flat_map(|(base, from, to)| (from ..= to).filter_map(move |off| base.checked_add(off)))
}

/// One tie fed into one range, and what came back against what the oracles
/// want.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Case {
    mode:    Mode,
    policy:  Policy,
    range:   (i128, i128),
    writing: Writing,
    eighth:  i64,
    got:     (i128, bool),
    want:    (i128, bool),
}

/// The first tie in `[lo, hi]` at which `ask` disagrees with the oracles.
///
/// Where the two oracles disagree with each other the sweep panics rather than
/// reporting, because a disagreement between them is a fault in this file.
fn first_disagreement(
    modes: &[Mode],
    policies: &[Policy],
    (lo, hi): (i128, i128),
    ask: impl Fn(Mode, Policy, Exact, Dither) -> (i128, bool),
) -> Maybe<Case> {
    for anchor in anchors(lo, hi) {
        for writing in writings_at(anchor) {
            let exact = writing.exact();
            assert!(exact.is_tie().get(), "{writing:?} is not a tie");
            for &mode in modes {
                for &eighth in eighths(mode) {
                    let at = tie_table(mode, writing.below(), eighth);
                    assert_eq!(
                        rounded(mode, writing.point(), (eighth, 8)),
                        at,
                        "the two oracles disagree at {writing:?} under {mode:?}"
                    );
                    let dither = Dither::at(Fraction::of(eighth, 8));
                    for &policy in policies {
                        let want = completed(policy, at, lo, hi);
                        assert_eq!(
                            answer(mode, policy, writing.point(), (eighth, 8), lo, hi),
                            want
                        );
                        let got = ask(mode, policy, exact, dither);
                        if got != want {
                            return Maybe::Is(Case {
                                mode,
                                policy,
                                range: (lo, hi),
                                writing,
                                eighth,
                                got,
                                want,
                            });
                        }
                    }
                }
            }
        }
    }
    Maybe::Isnt
}

// --- every admitted width, through the surface a consumer calls ---------------

/// The tie sweep at one declared signature, through `adapt` and
/// `panic_on_overflow`.
struct Surface {
    mode:   Mode,
    policy: Policy,
}

impl PerSignature for Surface {
    type Out = Maybe<Case>;

    fn run<S: DeclaredSignature>(&self) -> Maybe<Case> {
        let range = (
            <<S::Format as Format>::Slots as Slots>::MIN.index(),
            <<S::Format as Format>::Slots as Slots>::MAX.index(),
        );
        first_disagreement(&[self.mode], &[self.policy], range, |_, _, e, d| {
            (adapt::<S>(e, d).index(), panic_on_overflow::<S>(e, d).get())
        })
    }
}

/// Every format the width walk reaches, run at all eighteen signatures.
struct EveryWidth {
    found:   Maybe<Case>,
    whole:   usize,
    formats: usize,
}

impl PerFormat for EveryWidth {
    fn run<F: Format>(&mut self) {
        let (lo, hi) = (
            <F::Slots as Slots>::MIN.index(),
            <F::Slots as Slots>::MAX.index(),
        );
        self.formats += 1;
        self.whole += usize::from(walked_whole(lo, hi));
        if self.found.is() {
            return;
        }
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                let found = dispatch::at::<F, Surface>(mode, policy, &Surface {
                    mode,
                    policy,
                });
                if found.is() {
                    self.found = found;
                    return;
                }
            }
        }
    }
}

#[test]
fn every_admitted_width_answers_every_tie_as_the_oracles_do() {
    let mut walk = EveryWidth {
        found:   Maybe::Isnt,
        whole:   0,
        formats: 0,
    };
    dispatch::every_width(&mut walk);
    assert_eq!(walk.found, Maybe::Isnt);
    // Both families at every admitted width, and the whole range of each at
    // widths one through sixteen.
    assert_eq!(walk.formats, 128);
    assert_eq!(walk.whole, 32);
}

// --- the map at both ends of the index -----------------------------------------

/// Ranges at both ends of the index, around zero, and one slot wide at each end,
/// so a tie under `i128::MIN` and a tie over `i128::MAX` are both fed.
fn ranges() -> [(i128, i128); 8] {
    [
        (i128::MIN, i128::MIN + 15),
        (i128::MAX - 15, i128::MAX),
        (i128::MIN, i128::MIN),
        (i128::MAX, i128::MAX),
        (-8, 7),
        (0, 15),
        (-(1 << 63), (1 << 63) - 1),
        (0, (1 << 64) - 1),
    ]
}

/// The first tie at which `map` disagrees with the oracles, over every range.
fn map_break(map: Map) -> Maybe<Case> {
    for (lo, hi) in ranges() {
        let found = first_disagreement(
            &ALL_MODES,
            &SHIPPED_POLICIES,
            (lo, hi),
            |mode, policy, e, d| {
                let r = (map.round)(mode, e, d);
                (
                    (map.complete)(policy, r, Slot::at(lo), Slot::at(hi)).index(),
                    (map.leaves)(r, Slot::at(lo), Slot::at(hi)),
                )
            },
        );
        if found.is() {
            return found;
        }
    }
    Maybe::Isnt
}

#[test]
fn the_shipped_map_answers_every_tie_at_both_ends_of_the_index_as_the_oracles_do() {
    assert_eq!(map_break(shipped()), Maybe::Isnt);
}

#[test]
fn the_tie_sweep_reports_every_planted_tie_rule_at_a_tie_of_the_sign_it_breaks() {
    let mut asked = 0;
    for (name, map) in planted_tie_rules() {
        asked += 1;
        let Maybe::Is(case) = map_break(map) else {
            panic!("{name} was not reported");
        };
        let negative = case.writing.below().is_negative();
        match name {
            // The old rule agrees with the ruled one above zero.
            "half_up_ties_away_from_zero" => assert!(negative, "{name}: {case:?}"),
            // Toward zero agrees with it below zero.
            "half_up_ties_toward_zero" => assert!(!negative, "{name}: {case:?}"),
            _ => {},
        }
        let broken = if name.starts_with("half_even") { Mode::HalfEven } else { Mode::HalfUp };
        assert_eq!(case.mode, broken, "{name} reported under another mode");
    }
    assert_eq!(asked, 5, "a planted tie rule dropped out of the list");
}

// --- what the feed reaches ------------------------------------------------------

#[test]
fn the_tie_feed_reaches_every_sign_and_parity_and_both_sides_of_every_range() {
    for (lo, hi) in ranges() {
        // Sign by parity, then under the range and over it.
        let mut cells = [0usize; 4];
        let (mut under, mut over) = (0usize, 0usize);
        for anchor in anchors(lo, hi) {
            for w in writings_at(anchor) {
                let s = w.below();
                cells[usize::from(s.is_negative()) * 2 + usize::from(s.is_even())] += 1;
                under += usize::from(s.is_below(Wide::of(lo)));
                over += usize::from(!s.is_below(Wide::of(hi)));
            }
        }
        // A range pinned to one end of the index has no slot of the other sign
        // inside the walk, which the ranges around zero cover instead.
        if lo < 0 && hi >= 0 {
            assert!(cells.iter().all(|&c| c > 0), "[{lo}, {hi}]: {cells:?}");
        } else {
            let sign = usize::from(lo < 0) * 2;
            assert!(
                cells[sign] > 0 && cells[sign + 1] > 0,
                "[{lo}, {hi}]: {cells:?}"
            );
        }
        assert!(under > 0, "no tie under [{lo}, {hi}]");
        assert!(over > 0, "no tie over [{lo}, {hi}]");
    }
}

#[test]
fn the_tie_feed_names_the_tie_under_the_index_and_the_tie_over_it() {
    let bottom = Wide::of(i128::MIN).minus(Wide::of(1));
    let top = Wide::of(i128::MAX);
    let feeds = |s: Wide| {
        ranges()
            .into_iter()
            .any(|(lo, hi)| anchors(lo, hi).any(|a| writings_at(a).any(|w| w.below() == s)))
    };
    assert!(feeds(bottom), "the tie under i128::MIN is never fed");
    assert!(feeds(top), "the tie over i128::MAX is never fed");
}

#[test]
fn both_oracles_put_the_negative_half_up_tie_on_the_slot_above() {
    // The cell the ruling moved, answered by each oracle separately and as a
    // value, so the agreement the sweep asserts at every tie is also seen to be
    // the ruled answer rather than two readings agreeing on the old one.
    let s = Wide::of(-3);
    let point = Point {
        whole: s,
        num:   1,
        den:   2,
    };
    assert_eq!(rounded(Mode::HalfUp, point, (0, 8)), Wide::of(-2));
    assert_eq!(tie_table(Mode::HalfUp, s, 0), Wide::of(-2));
    assert_ne!(rounded(Mode::HalfUp, point, (0, 8)), Wide::of(-3));
}
