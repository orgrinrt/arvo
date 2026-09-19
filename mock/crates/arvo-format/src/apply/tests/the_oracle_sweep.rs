//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The applied map against the oracle, at both ends of the index.
//!
//! Every mode and every policy, over ranges at the top of the index, at its
//! bottom, a few slots in from either end, and at neither end, the 200-slot and
//! `2^64`-slot ranges among them. Each range is fed positions from each end of
//! the index and from each of its own ends, up to `depth()` whole slots either
//! way, zero included, at every quarter remainder and under every eighth of a
//! dither. The slot and the overflow verdict are both compared with what the
//! oracle computes. The oracle reads `Clamp` as `Saturate`, taken from the
//! implementation, so those cells check the map against that reading, as
//! `the_oracle.rs` says; every tie rule it applies is the canon's.
//!
//! The sweep runs twice. Through `adapt` and `panic_on_overflow`, at every
//! declared signature over those ranges, which is the surface. And through a
//! `Map`, so every broken map `the_broken_maps.rs` keeps, the planted tie rules
//! among them, is asked the same questions and has to be reported.

use notko::Maybe;

use super::the_broken_maps::{
    Map,
    a_step_up_past_the_bottom_pins_high,
    anchored_at_zero,
    no_lo_guard_onto_the_lowest,
    no_step_onto_the_lowest,
    past_the_bottom_pins_high_off_the_bottom,
    past_the_top_pins_low_off_the_top,
    planted_tie_rules,
    reduced_modulo_256,
    shipped,
    subtracts_first,
    the_step_onto_the_bottom_pins_high,
};
use super::the_far_end_of_the_index::{BottomOf200, TopOf200};
use super::the_oracle::{Point, Wide, answer};
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
use crate::adapt::DeclaredSignature;
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow, round_slot};
use crate::format::Format;
use crate::overflow::{Policy, SHIPPED_POLICIES};
use crate::points::Integer;
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Slot, Slots};
use crate::tests::dispatch::{self, PerSignature};
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};
use crate::width::Width;

/// A plain integer format over a slot range.
type Over<S> = Grid<BinaryRationals, Constant<0>, S, 0, 1>;

/// 200 slots ending four under the top of the index, so a position a few
/// slots from the top is above it while the index still holds it.
pub(super) struct ShyOfTheTop;

impl Slots for ShyOfTheTop {
    const MAX: Slot = Slot::at(i128::MAX - 4);
    const MIN: Slot = Slot::at(i128::MAX - 203);
    const WIDTH: Width = Width::bits(8);
}

/// 200 slots starting four over the bottom of the index.
pub(super) struct ShyOfTheBottom;

impl Slots for ShyOfTheBottom {
    const MAX: Slot = Slot::at(i128::MIN + 203);
    const MIN: Slot = Slot::at(i128::MIN + 4);
    const WIDTH: Width = Width::bits(8);
}

/// Every format the sweep feeds, handed to `$then`.
macro_rules! every_format {
    ($then:ident) => {
        $then!(
            Over<AtTheTop>,
            Over<WideTop>,
            Over<TopOf200>,
            Over<ShyOfTheTop>,
            Over<AtTheBottom>,
            Over<WideBottom>,
            Over<BottomOf200>,
            Over<ShyOfTheBottom>,
            Integer<3>,
            Over<Reference>,
            Over<NegativeReference>,
            Over<WideReference>,
            Over<WideNegativeReference>,
            Over<Reference200>,
            Over<NegativeReference200>,
        )
    };
}

/// How many whole slots either way of each end a position is fed from. A
/// function rather than an item constant, for the reason `the_ratio_coordinate`
/// gives.
fn depth() -> i64 {
    8
}

/// The range of a format.
fn range_of<F: Format>() -> (Slot, Slot) {
    (<F::Slots as Slots>::MIN, <F::Slots as Slots>::MAX)
}

/// The range of every format the sweep feeds.
fn ranges() -> [(Slot, Slot); 15] {
    macro_rules! of {
        ($($f:ty),+ $(,)?) => {
            [$(range_of::<$f>()),+]
        };
    }
    every_format!(of)
}

/// One position fed into one range, and what came back against what the
/// oracle wants.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Case {
    mode:    Mode,
    policy:  Policy,
    range:   (i128, i128),
    anchor:  i128,
    whole:   i64,
    quarter: i64,
    eighth:  i64,
    got:     (i128, bool),
    want:    (i128, bool),
}

/// Every position fed into `[lo, hi]`: the slot it is written from, how many
/// whole slots from there, and how many quarters past that.
fn feed(lo: i128, hi: i128) -> impl Iterator<Item = (i128, i64, i64)> {
    [i128::MIN, i128::MAX, lo, hi]
        .into_iter()
        .flat_map(|anchor| {
            (-depth() ..= depth())
                .flat_map(move |whole| (0 .. 4).map(move |quarter| (anchor, whole, quarter)))
        })
}

/// The first position, mode, policy and dither at which `ask` disagrees with the
/// oracle over one range, or `Isnt`. With `held_only`, positions whose slot the
/// index does not hold are skipped.
fn first_disagreement(
    modes: &[Mode],
    policies: &[Policy],
    (min, max): (Slot, Slot),
    held_only: bool,
    ask: impl Fn(Mode, Policy, Exact, Dither) -> (Slot, bool),
) -> Maybe<Case> {
    let (lo, hi) = (min.index(), max.index());
    for (anchor, whole, quarter) in feed(lo, hi) {
        let at = Wide::of(anchor).plus(Wide::of(whole as i128));
        if held_only && at.index().isnt() {
            continue;
        }
        let exact = Exact::between(Slot::at(anchor), Fraction::of(4 * whole + quarter, 4));
        let point = Point {
            whole: at,
            num:   quarter,
            den:   4,
        };
        for eighth in 0 .. 8 {
            let dither = Dither::at(Fraction::of(eighth, 8));
            for &mode in modes {
                for &policy in policies {
                    let want = answer(mode, policy, point, (eighth, 8), lo, hi);
                    let (slot, leaves) = ask(mode, policy, exact, dither);
                    if (slot.index(), leaves) != want {
                        return Maybe::Is(Case {
                            mode,
                            policy,
                            range: (lo, hi),
                            anchor,
                            whole,
                            quarter,
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

/// The first disagreement of `map` with the oracle, over every range, mode and
/// policy. The map's own rounding is asked, so a planted tie rule is what the
/// completion and the verdict see.
fn map_break(map: Map, held_only: bool) -> Maybe<Case> {
    for range in ranges() {
        let found = first_disagreement(
            &ALL_MODES,
            &SHIPPED_POLICIES,
            range,
            held_only,
            |mode, policy, e, d| {
                let rounded = (map.round)(mode, e, d);
                (
                    (map.complete)(policy, rounded, range.0, range.1),
                    (map.leaves)(rounded, range.0, range.1),
                )
            },
        );
        if found.is() {
            return found;
        }
    }
    Maybe::Isnt
}

/// The sweep at one declared signature, through the surface a consumer calls.
struct Surface {
    mode:   Mode,
    policy: Policy,
}

impl PerSignature for Surface {
    type Out = Maybe<Case>;

    fn run<S: DeclaredSignature>(&self) -> Maybe<Case> {
        first_disagreement(
            &[self.mode],
            &[self.policy],
            range_of::<S::Format>(),
            false,
            |_, _, e, d| (adapt::<S>(e, d), panic_on_overflow::<S>(e, d).get()),
        )
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
fn every_declared_signature_answers_as_the_oracle_does() {
    macro_rules! each {
        ($($f:ty),+ $(,)?) => {
            $( assert_eq!(surface_break::<$f>(), Maybe::Isnt, "{}", stringify!($f)); )+
        };
    }
    every_format!(each);
}

#[test]
fn the_shipped_map_answers_as_the_oracle_does() {
    // The positive control for the reports below: the sweep reporting the
    // shipped map would be reporting on its own instrument.
    assert_eq!(map_break(shipped(), false), Maybe::Isnt);
}

#[test]
fn the_sweep_reports_every_broken_map() {
    // Whether each map is also reported at a position the index holds. The
    // wraps, the missing step onto the lowest slot and every planted tie rule
    // are wrong inside the index; the other five change only a branch reading a
    // distance past it, so inside the index they are the shipped map, which is
    // what keeps each of them from being a map that is simply always wrong.
    let completions: [(&'static str, Map, bool); 9] = [
        ("subtracts_first", subtracts_first(), true),
        ("anchored_at_zero", anchored_at_zero(), true),
        ("reduced_modulo_256", reduced_modulo_256(), true),
        ("no_step_onto_the_lowest", no_step_onto_the_lowest(), true),
        (
            "past_the_bottom_pins_high_off_the_bottom",
            past_the_bottom_pins_high_off_the_bottom(),
            false,
        ),
        (
            "past_the_top_pins_low_off_the_top",
            past_the_top_pins_low_off_the_top(),
            false,
        ),
        (
            "a_step_up_past_the_bottom_pins_high",
            a_step_up_past_the_bottom_pins_high(),
            false,
        ),
        (
            "the_step_onto_the_bottom_pins_high",
            the_step_onto_the_bottom_pins_high(),
            false,
        ),
        (
            "no_lo_guard_onto_the_lowest",
            no_lo_guard_onto_the_lowest(),
            false,
        ),
    ];
    let ties = planted_tie_rules().map(|(name, map)| (name, map, true));
    let mut asked = 0;
    for (name, map, inside) in completions.into_iter().chain(ties) {
        asked += 1;
        assert!(map_break(map, false).is(), "{name} was not reported");
        assert_eq!(
            map_break(map, true).is(),
            inside,
            "{name}, inside the index"
        );
    }
    assert_eq!(asked, 14, "a broken map dropped out of the list");
}

#[test]
fn the_feed_reaches_every_band_around_both_ends() {
    // The sweep is only about the ends if its positions are at them. Counted
    // over `Integer<3>`, a range at neither end, and `ShyOfTheBottom`, whose
    // lowest slot has a slot under it inside the index. The ties are counted by
    // sign and by the parity of the slot below, because each planted tie rule
    // differs from the shipped map in one of those four cells only.
    let mut onto_the_bottom = 0;
    let mut further_under = 0;
    let mut one_over = 0;
    let mut stepping_over = 0;
    let mut onto_a_lowest_slot = 0;
    let mut ties = [[0usize; 2]; 2];
    for (lo, hi) in [(-4, 3), (i128::MIN + 4, i128::MIN + 203)] {
        for (anchor, whole, quarter) in feed(lo, hi) {
            let exact = Exact::between(Slot::at(anchor), Fraction::of(4 * whole + quarter, 4));
            if exact.is_tie().get() {
                ties[usize::from(exact.is_negative())][usize::from(exact.is_even())] += 1;
            }
            for mode in ALL_MODES {
                let r = round_slot(mode, exact, Dither::UNUSED);
                onto_the_bottom += usize::from(r.past == -1 && r.up.get());
                further_under += usize::from(r.past <= -2);
                one_over += usize::from(r.past == 1);
                stepping_over += usize::from(r.past == 0 && r.down() == i128::MAX && r.up.get());
                onto_a_lowest_slot += usize::from(
                    r.past == 0 && r.up.get() && r.down() == i128::MIN + 3 && lo == i128::MIN + 4,
                );
            }
        }
    }
    for (count, band) in [
        (
            onto_the_bottom,
            "the step from one under the index onto its bottom",
        ),
        (further_under, "two or more under the index"),
        (one_over, "one over the index"),
        (stepping_over, "the step from the top of the index past it"),
        (
            onto_a_lowest_slot,
            "the step onto a range's lowest slot inside the index",
        ),
        (ties[0][0], "a non-negative tie over an odd slot"),
        (ties[0][1], "a non-negative tie over an even slot"),
        (ties[1][0], "a negative tie over an odd slot"),
        (ties[1][1], "a negative tie over an even slot"),
    ] {
        assert!(count > 0, "the feed never reaches {band}");
    }
}
