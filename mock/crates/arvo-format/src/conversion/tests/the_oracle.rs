//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion against the applied map's oracle, at every member up to eight
//! bits, onto targets finer and coarser than the source, across families,
//! between a phase and none and between radix two and radix ten.
//!
//! The position is the source member's value over the target's quantum, less
//! the target's phase, in `Ratio`, and the oracle in `apply` rounds and
//! completes it. The adapted result and its overflow verdict have to equal the
//! oracle's at every member, mode and shipped policy. A conversion onto the
//! source's own grid is the identity. A planted step that forgets the target's
//! phase is run through the same comparison and has to be reported.

use core::marker::PhantomData;
use core::ops::RangeInclusive;

use super::{exact_conversion, members, quantum, value};
use crate::adapt::{DeclaredSignature, overflow_of, rounding_of};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::tests::the_oracle::{Point, Wide, answer};
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow};
use crate::conversion::{convert, converted_position, is_convertible};
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Signed, Slots, Unsigned};
use crate::tests::dispatch::{self, PerSignature};
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

/// The quarters the dither is drawn at: four for the mode that reads it, one
/// for every other.
fn quarters(mode: Mode) -> RangeInclusive<i64> {
    if mode == Mode::Stochastic { 0 ..= 3 } else { 0 ..= 0 }
}

/// A ratio of slots as the oracle's point.
fn point(at: Ratio) -> Point {
    let rest = Ratio::of(at.num.rem_euclid(at.den), at.den);
    Point {
        whole: Wide::of(at.floor()),
        num:   rest.num,
        den:   rest.den,
    }
}

/// What a sweep found.
#[derive(Clone, Copy, Default)]
struct Tally {
    checked: u64,
    wrong:   u64,
}

/// Every member of `F` onto one target signature against the oracle. `planted`
/// swaps the step for the value over the target's quantum with the target's
/// phase left in, and counts rather than stops.
struct EveryMember<F> {
    planted: bool,
    source:  PhantomData<F>,
}

impl<F: Format> PerSignature for EveryMember<F> {
    type Out = Tally;

    fn run<S: DeclaredSignature>(&self) -> Tally {
        let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
        let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
        let mode = rounding_of::<S::Adaptation>();
        let policy = overflow_of::<S::Adaptation>();
        let mut tally = Tally::default();
        for a in members::<F>() {
            let at = point(exact_conversion::<F, S::Format>(a));
            let exact: Exact = if self.planted {
                value::<F>(a).over(quantum::<S::Format>()).position()
            } else {
                converted_position::<F, S::Format>(a)
            };
            for quarter in quarters(mode) {
                let dither = Dither::at(Fraction::of(quarter, 4));
                let want = answer(mode, policy, at, (quarter, 4), min, max);
                let slot = if self.planted {
                    adapt::<S>(exact, dither)
                } else {
                    convert::<F, S>(a, dither)
                };
                let got = (slot.index(), panic_on_overflow::<S>(exact, dither).get());
                if !self.planted {
                    assert_eq!(
                        got, want,
                        "{a:?} under {mode:?}, {policy:?}, dither {quarter}/4, onto [{min}, {max}]"
                    );
                }
                tally.checked += 1;
                tally.wrong += u64::from(got != want);
            }
        }
        tally
    }
}

/// A sweep over pairs, every target signature at each.
#[derive(Default)]
struct Sweep {
    planted: bool,
    pairs:   u64,
    checked: u64,
    wrong:   u64,
    owed:    u64,
}

impl Sweep {
    fn run<F: Format, T: Format>(&mut self) {
        assert!(
            is_convertible::<F, T>().get(),
            "the sweep runs at admitted pairs only"
        );
        self.pairs += 1;
        let count = members::<F>().count() as u64;
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                let every = EveryMember::<F> {
                    planted: self.planted,
                    source:  PhantomData,
                };
                let tally = dispatch::at::<T, EveryMember<F>>(mode, policy, &every);
                self.checked += tally.checked;
                self.wrong += tally.wrong;
                self.owed += count * (*quarters(mode).end() as u64 + 1);
            }
        }
    }
}

/// A binary format over `S` at the quantum `2^E` and zero phase.
type At<S, const E: i32> = Grid<BinaryRationals, Constant<E>, S, 0, 1>;

#[test]
fn the_conversion_answers_as_the_oracle_does_at_every_member_up_to_eight_bits() {
    let mut walk = Sweep::default();
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                // Onto a finer grid, and onto a coarser one, which rounds.
                walk.run::<At<Signed<$w>, 0>, At<Signed<8>, -2>>();
                walk.run::<At<Unsigned<$w>, -3>, At<Signed<8>, 0>>();
                // Onto a narrower range, which the policy completes.
                walk.run::<At<Signed<$w>, 2>, At<Signed<6>, 0>>();
                // From a phase to none, and from none to a phase.
                walk.run::<Grid<BinaryRationals, Constant<-1>, Signed<$w>, 1, 2>, At<Unsigned<8>, -2>>();
                walk.run::<At<Unsigned<$w>, -2>, Grid<DecimalRationals, Constant<-1>, Unsigned<8>, 2, 3>>();
                // From radix ten to radix two.
                walk.run::<Grid<DecimalRationals, Constant<-1>, Signed<$w>, 0, 1>, At<Signed<8>, -3>>();
            )+
        };
    }
    widths!(1, 2, 3, 4, 5, 6, 7, 8);
    assert_eq!(walk.pairs, 8 * 6);
    assert_eq!(walk.checked, walk.owed);
    assert!(walk.checked > 0);
    assert_eq!(walk.wrong, 0);
}

#[test]
fn a_conversion_onto_the_source_s_own_grid_is_the_identity() {
    macro_rules! own {
        ($($f:ty),+ $(,)?) => {
            $(
                for a in members::<$f>() {
                    assert_eq!(converted_position::<$f, $f>(a), Exact::on_grid(a), "{a:?}");
                }
            )+
        };
    }
    own!(
        At<Signed<8>, 0>,
        At<Unsigned<8>, -5>,
        Grid<BinaryRationals, Constant<-1>, Signed<8>, 1, 2>,
        Grid<DecimalRationals, Constant<-2>, Signed<8>, 2, 3>,
    );
}

#[test]
fn the_sweep_reports_a_planted_conversion_that_forgets_the_target_s_phase() {
    let mut planted = Sweep {
        planted: true,
        ..Sweep::default()
    };
    planted.run::<At<Unsigned<4>, -2>, Grid<DecimalRationals, Constant<-1>, Unsigned<8>, 2, 3>>();
    assert_eq!(planted.checked, planted.owed);
    assert!(
        planted.wrong > 0,
        "the sweep missed a conversion without the target's phase"
    );
    // The control: onto a target with no phase the planted step is the shipped
    // one, and the same machinery finds nothing wrong.
    let mut control = Sweep {
        planted: true,
        ..Sweep::default()
    };
    control.run::<Grid<BinaryRationals, Constant<-1>, Signed<4>, 1, 2>, At<Unsigned<8>, -2>>();
    assert_eq!(control.checked, control.owed);
    assert_eq!(control.wrong, 0);
}
