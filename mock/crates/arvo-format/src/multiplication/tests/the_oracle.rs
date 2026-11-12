//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product against the applied map's oracle, at every member pair up to
//! eight bits.
//!
//! The position is the slots' product times the quantum, in `Ratio`, and the
//! oracle in `apply` rounds and completes it from the canon's formulas. The
//! adapted product and its overflow verdict have to equal the oracle's at every
//! pair, mode and shipped policy, with four dithers for the mode that reads one,
//! at whole and fractional exponents. A planted product that forgets the
//! quantum is run through the same comparison and has to be reported.

use core::ops::RangeInclusive;

use super::{At, Ratio, exact_product, members};
use crate::adapt::{DeclaredSignature, overflow_of, rounding_of};
use crate::apply::tests::the_oracle::{Point, Wide, answer};
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow};
use crate::format::Format;
use crate::multiplication::{mul, product_position};
use crate::overflow::SHIPPED_POLICIES;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Signed, Slots, Unsigned};
use crate::tests::dispatch::{self, PerFormat, PerSignature};

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
        num:   rest.num as i64,
        den:   rest.den as i64,
    }
}

/// What a sweep found.
#[derive(Clone, Copy, Default)]
struct Tally {
    checked: u64,
    wrong:   u64,
}

/// Every member pair at one signature against the oracle. `planted` swaps the
/// step for the slots' product alone and counts rather than stops.
struct EveryPair {
    planted: bool,
}

impl PerSignature for EveryPair {
    type Out = Tally;

    fn run<S: DeclaredSignature>(&self) -> Tally {
        let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
        let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
        let mode = rounding_of::<S::Adaptation>();
        let policy = overflow_of::<S::Adaptation>();
        let mut tally = Tally::default();
        for a in members::<S::Format>() {
            for b in members::<S::Format>() {
                let at = point(exact_product::<S::Format>(a, b));
                let exact: Exact = if self.planted {
                    Ratio::whole(a.index() * b.index()).position()
                } else {
                    product_position::<S::Format>(a, b)
                };
                for quarter in quarters(mode) {
                    let dither = Dither::at(Fraction::of(quarter, 4));
                    let want = answer(mode, policy, at, (quarter, 4), min, max);
                    let slot = if self.planted {
                        adapt::<S>(exact, dither)
                    } else {
                        mul::<S>(a, b, dither)
                    };
                    let got = (slot.index(), panic_on_overflow::<S>(exact, dither).get());
                    if !self.planted {
                        assert_eq!(
                            got, want,
                            "{a:?} * {b:?} under {mode:?}, {policy:?}, dither {quarter}/4, over [{min}, {max}]"
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
    fn run<F: Format>(&mut self) {
        self.formats += 1;
        let pairs = members::<F>().count() as u64;
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                let every = EveryPair {
                    planted: self.planted,
                };
                let tally = dispatch::at::<F, EveryPair>(mode, policy, &every);
                self.checked += tally.checked;
                self.wrong += tally.wrong;
                self.owed += pairs * pairs * (*quarters(mode).end() as u64 + 1);
            }
        }
    }
}

#[test]
fn the_product_answers_as_the_oracle_does_at_every_member_pair_up_to_eight_bits() {
    let mut walk = Sweep::default();
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                walk.run::<At<Signed<$w>, 0>>();
                walk.run::<At<Unsigned<$w>, 0>>();
                walk.run::<At<Signed<$w>, -3>>();
                walk.run::<At<Unsigned<$w>, -3>>();
                walk.run::<At<Signed<$w>, 2>>();
                walk.run::<At<Unsigned<$w>, -1>>();
            )+
        };
    }
    widths!(1, 2, 3, 4, 5, 6, 7, 8);
    assert_eq!(walk.formats, 8 * 6);
    assert_eq!(walk.checked, walk.owed);
    assert_eq!(walk.wrong, 0);
}

#[test]
fn the_sweep_reports_a_product_that_forgets_the_quantum() {
    // At a quantum of one the planted product is the product, so the sweep has
    // to find nothing there and something at every other exponent.
    let mut at_one = Sweep {
        planted: true,
        ..Sweep::default()
    };
    at_one.run::<At<Signed<4>, 0>>();
    at_one.run::<At<Unsigned<4>, 0>>();
    assert_eq!(at_one.checked, at_one.owed);
    assert_eq!(at_one.wrong, 0);

    for planted in [
        {
            let mut w = Sweep {
                planted: true,
                ..Sweep::default()
            };
            w.run::<At<Signed<4>, -2>>();
            w
        },
        {
            let mut w = Sweep {
                planted: true,
                ..Sweep::default()
            };
            w.run::<At<Unsigned<4>, 1>>();
            w
        },
    ] {
        assert_eq!(planted.checked, planted.owed);
        assert!(
            planted.wrong > 0,
            "the sweep missed a product without its quantum"
        );
    }
}
