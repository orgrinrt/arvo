//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient against the applied map's oracle, at every member pair with a
//! non-zero divisor up to eight bits.
//!
//! The position is the quotient of the two values over the quantum, less the
//! phase, in `Ratio`, and the oracle in `apply` rounds and completes it from the
//! canon's formulas. The adapted quotient and its overflow verdict have to equal
//! the oracle's at every pair, mode and shipped policy, with four dithers for the
//! mode that reads one, at exponents on both sides of zero over both families,
//! at a phase and at radix ten. A planted quotient that forgets the phase's term
//! is run through the same comparison and has to be reported.

use core::ops::RangeInclusive;

use super::{divisors, exact_quotient, members, quantum, value};
use crate::adapt::{DeclaredSignature, overflow_of, rounding_of};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::tests::the_oracle::{Point, Wide, answer};
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow};
use crate::division::{Divisor, div, is_divisible, quotient_position};
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Signed, Slots, Unsigned};
use crate::tests::dispatch::{self, PerFormat, PerSignature};
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

/// Every pair at one signature against the oracle. `planted` swaps the step for
/// the quotient of the values over the quantum with the phase's term left in,
/// and counts rather than stops.
struct EveryPair {
    planted: bool,
}

impl PerSignature for EveryPair {
    type Out = Tally;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> Tally {
        let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
        let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
        let mode = rounding_of::<S::Adaptation>();
        let policy = overflow_of::<S::Adaptation>();
        let mut tally = Tally::default();
        for a in members::<S::Format>() {
            for b in divisors::<S::Format>() {
                let at = point(exact_quotient::<S::Format>(a, b));
                let divisor = Divisor::<S::Format>::assumed(b);
                let exact: Exact = if self.planted {
                    value::<S::Format>(a)
                        .over(value::<S::Format>(b))
                        .over(quantum::<S::Format>())
                        .position()
                } else {
                    quotient_position::<S::Format>(a, divisor)
                };
                for quarter in quarters(mode) {
                    let dither = Dither::at(Fraction::of(quarter, 4));
                    let want = answer(mode, policy, at, (quarter, 4), min, max);
                    let slot = if self.planted {
                        adapt::<S>(exact, dither)
                    } else {
                        div::<S>(a, divisor, dither)
                    };
                    let got = (slot.index(), panic_on_overflow::<S>(exact, dither).get());
                    if !self.planted {
                        assert_eq!(
                            got, want,
                            "{a:?} / {b:?} under {mode:?}, {policy:?}, dither {quarter}/4, over [{min}, {max}]"
                        );
                    }
                    tally.checked += 1;
                    tally.wrong += u64::from(got != want);
                }
            }
        }
        tally
    }
}

/// A sweep over formats, every signature at each.
#[derive(Default)]
struct Sweep {
    planted: bool,
    formats: u64,
    checked: u64,
    wrong:   u64,
    owed:    u64,
}

impl PerFormat for Sweep {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        assert!(
            is_divisible::<F>().get(),
            "the sweep runs at admitted formats only"
        );
        self.formats += 1;
        let pairs = (members::<F>().count() * divisors::<F>().count()) as u64;
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                let every = EveryPair {
                    planted: self.planted,
                };
                let tally = dispatch::at::<F, EveryPair>(mode, policy, &every);
                self.checked += tally.checked;
                self.wrong += tally.wrong;
                self.owed += pairs * (*quarters(mode).end() as u64 + 1);
            }
        }
    }
}

/// A binary format over `S` at the quantum `2^E` and zero phase.
type At<S, const E: i32> = Grid<BinaryRationals, Constant<E>, S, 0, 1>;

#[test]
fn the_quotient_answers_as_the_oracle_does_at_every_member_pair_up_to_eight_bits() {
    let mut walk = Sweep::default();
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                walk.run::<At<Signed<$w>, 0>>();
                walk.run::<At<Unsigned<$w>, 0>>();
                walk.run::<At<Signed<$w>, -3>>();
                walk.run::<At<Unsigned<$w>, -3>>();
                walk.run::<At<Signed<$w>, 2>>();
                walk.run::<At<Unsigned<$w>, 2>>();
                walk.run::<Grid<BinaryRationals, Constant<-1>, Signed<$w>, 1, 2>>();
                walk.run::<Grid<BinaryRationals, Constant<1>, Unsigned<$w>, -1, 3>>();
                walk.run::<Grid<DecimalRationals, Constant<-1>, Signed<$w>, 0, 1>>();
                walk.run::<Grid<DecimalRationals, Constant<1>, Unsigned<$w>, 0, 1>>();
                walk.run::<Grid<DecimalRationals, Constant<-2>, Signed<$w>, 2, 3>>();
            )+
        };
    }
    widths!(1, 2, 3, 4, 5, 6, 7, 8);
    assert_eq!(walk.formats, 8 * 11);
    assert_eq!(walk.checked, walk.owed);
    assert!(walk.checked > 0);
    assert_eq!(walk.wrong, 0);
}

#[test]
fn the_sweep_reports_a_planted_quotient_that_forgets_the_phase() {
    // At a quantum of one, so only the phase's term is missing, and at a phase.
    let mut planted = Sweep {
        planted: true,
        ..Sweep::default()
    };
    planted.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 1, 2>>();
    planted.run::<Grid<DecimalRationals, Constant<0>, Unsigned<4>, 2, 3>>();
    assert_eq!(planted.checked, planted.owed);
    assert!(
        planted.wrong > 0,
        "the sweep missed a quotient without its phase's term"
    );
    // The control: at zero phase the planted step is the shipped one, so the
    // same machinery has to find nothing wrong with it there.
    let mut control = Sweep {
        planted: true,
        ..Sweep::default()
    };
    control.run::<At<Signed<4>, -2>>();
    assert_eq!(control.checked, control.owed);
    assert_eq!(control.wrong, 0);
}
