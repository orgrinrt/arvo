//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every fimath rounding method and overflow action, against MathWorks'
//! definitions written as integer rules, over a matrix of shapes.
//!
//! `matlab_fi_parity.rs` holds the published worked examples, which reach no tie
//! and no overflow. This file holds the rest of the declaration set: each method
//! name `standards::matlab` exports, the Round alias, and both overflow actions,
//! at signed and unsigned word lengths from one bit to sixty-four and at fraction
//! lengths positive, zero and negative. At each shape the positions run in
//! quarters of a step around both ends of the range and around zero, so every
//! arm passes ties of both signs and both ends.
//!
//! The expected side is each method as MathWorks defines it, in exact integers
//! over the position counted in quarters, and reads no arvo code: Floor and
//! Ceiling toward the infinities, Zero toward zero, Nearest to the nearest with a
//! tie toward positive infinity, Convergent with a tie to the even neighbour, and
//! Round with a tie away from zero. Saturate pins to the range and Wrap is
//! modulo two to the word length, both computed from the shape. A slot is the
//! stored integer at every fraction length, so the rules read the shape's range
//! and not its scaling.

mod ties_away;

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::Overflow;
use arvo_format::slots::Slot;
use arvo_format::standards::matlab::overflow_action::{Saturate, Wrap};
use arvo_format::standards::matlab::rounding_method::{Ceiling, Convergent, Floor, Nearest, Zero};
use arvo_format::standards::matlab::{Fi, FiMath, Ufi, UfiMath};

/// A word as MathWorks states it: the stored integer's range and its length.
#[derive(Clone, Copy)]
struct Shape {
    low:   i128,
    high:  i128,
    width: u32,
}

impl Shape {
    fn signed(width: u32) -> Self {
        Self {
            low: -(1 << (width - 1)),
            high: (1 << (width - 1)) - 1,
            width,
        }
    }

    fn unsigned(width: u32) -> Self {
        Self {
            low: 0,
            high: (1 << width) - 1,
            width,
        }
    }

    /// Positions in quarters, forty either side of each end and of zero.
    fn quarters(self) -> impl Iterator<Item = i128> {
        [self.low, 0, self.high]
            .into_iter()
            .flat_map(|c| (-40 ..= 40).map(move |d| 4 * c + d))
    }
}

// --- MathWorks' definitions, in integers -----------------------------------------

fn floor(q: i128) -> i128 {
    q.div_euclid(4)
}

fn ceiling(q: i128) -> i128 {
    -(-q).div_euclid(4)
}

fn zero(q: i128) -> i128 {
    q / 4
}

fn nearest(q: i128) -> i128 {
    (q + 2).div_euclid(4)
}

fn convergent(q: i128) -> i128 {
    let down = q.div_euclid(4);
    match (q.rem_euclid(4) * 2).cmp(&4) {
        core::cmp::Ordering::Less => down,
        core::cmp::Ordering::Greater => down + 1,
        core::cmp::Ordering::Equal => down + down.rem_euclid(2),
    }
}

fn round(q: i128) -> i128 {
    q.signum() * (q.abs() + 2).div_euclid(4)
}

fn saturate(s: Shape, r: i128) -> i128 {
    r.clamp(s.low, s.high)
}

fn wrap(s: Shape, r: i128) -> i128 {
    (r - s.low).rem_euclid(1 << s.width) + s.low
}

// --- the arm ----------------------------------------------------------------------

type Rule = fn(i128) -> i128;
type Action = fn(Shape, i128) -> i128;

fn position(q: i128) -> Exact {
    Exact::between(
        Slot::at(q.div_euclid(4)),
        Fraction::of(q.rem_euclid(4) as i64, 4),
    )
}

/// How many positions a declared signature answers differently from the rule and
/// the action.
fn misses<S: DeclaredSignature>(shape: Shape, rule: Rule, action: Action) -> usize {
    shape
        .quarters()
        .filter(|&q| adapt::<S>(position(q), Dither::UNUSED).index() != action(shape, rule(q)))
        .count()
}

/// The Round alias on a format, under an overflow action, against the same rule.
fn round_misses<F: Format, O: Overflow>(shape: Shape, action: Action) -> usize {
    shape
        .quarters()
        .filter(|&q| {
            ties_away::select::<F, O>(q.div_euclid(4), q.rem_euclid(4) as i64, 4)
                != action(shape, round(q))
        })
        .count()
}

/// One shape's arms and planted arms. `$fi` is the format, `$math` the fimath
/// over it, and `$shape` the word the rules are computed from.
macro_rules! shape {
    ($name:ident, $fi:ident, $math:ident, $shape:ident, $w:literal, $fl:literal) => {
        mod $name {
            use super::*;

            type F = $fi<$w, $fl>;

            fn word() -> Shape {
                Shape::$shape($w)
            }

            #[test]
            fn every_rounding_method_is_mathworks() {
                let s = word();
                for (got, at) in [
                    (
                        misses::<$math<$w, $fl, Floor, Saturate>>(s, floor, saturate),
                        "floor saturate",
                    ),
                    (
                        misses::<$math<$w, $fl, Ceiling, Saturate>>(s, ceiling, saturate),
                        "ceiling saturate",
                    ),
                    (
                        misses::<$math<$w, $fl, Zero, Saturate>>(s, zero, saturate),
                        "zero saturate",
                    ),
                    (
                        misses::<$math<$w, $fl, Nearest, Saturate>>(s, nearest, saturate),
                        "nearest saturate",
                    ),
                    (
                        misses::<$math<$w, $fl, Convergent, Saturate>>(s, convergent, saturate),
                        "convergent saturate",
                    ),
                    (round_misses::<F, Saturate>(s, saturate), "round saturate"),
                    (
                        misses::<$math<$w, $fl, Floor, Wrap>>(s, floor, wrap),
                        "floor wrap",
                    ),
                    (
                        misses::<$math<$w, $fl, Ceiling, Wrap>>(s, ceiling, wrap),
                        "ceiling wrap",
                    ),
                    (
                        misses::<$math<$w, $fl, Zero, Wrap>>(s, zero, wrap),
                        "zero wrap",
                    ),
                    (
                        misses::<$math<$w, $fl, Nearest, Wrap>>(s, nearest, wrap),
                        "nearest wrap",
                    ),
                    (
                        misses::<$math<$w, $fl, Convergent, Wrap>>(s, convergent, wrap),
                        "convergent wrap",
                    ),
                    (round_misses::<F, Wrap>(s, wrap), "round wrap"),
                ] {
                    assert_eq!(got, 0, "{at}");
                }
            }

            // Each arm with its method or action swapped for a wrong one. Every
            // planted arm is required to miss, so the sweep is shown able to
            // tell each method from its nearest neighbour and each action from
            // the other, at this shape. The method swaps run under Wrap, which is
            // one to one modulo two to the word length, so two rules that differ
            // anywhere stay apart; saturation collapses them at the ends, and on
            // an unsigned word it makes Zero and Floor one rule.
            #[test]
            fn a_planted_method_or_action_is_reported() {
                let s = word();
                for (got, at) in [
                    (
                        misses::<$math<$w, $fl, Nearest, Wrap>>(s, convergent, wrap),
                        "nearest as convergent",
                    ),
                    (
                        misses::<$math<$w, $fl, Convergent, Wrap>>(s, nearest, wrap),
                        "convergent as nearest",
                    ),
                    (
                        misses::<$math<$w, $fl, Zero, Wrap>>(s, floor, wrap),
                        "zero as floor",
                    ),
                    (
                        misses::<$math<$w, $fl, Floor, Wrap>>(s, ceiling, wrap),
                        "floor as ceiling",
                    ),
                    (
                        misses::<$math<$w, $fl, Ceiling, Wrap>>(s, zero, wrap),
                        "ceiling as zero",
                    ),
                    (
                        misses::<$math<$w, $fl, Nearest, Wrap>>(s, round, wrap),
                        "nearest as round",
                    ),
                    (
                        misses::<$math<$w, $fl, Floor, Wrap>>(s, floor, saturate),
                        "wrap as saturate",
                    ),
                    (
                        misses::<$math<$w, $fl, Floor, Saturate>>(s, floor, wrap),
                        "saturate as wrap",
                    ),
                    (
                        round_misses::<F, Wrap>(s, saturate),
                        "round wrap as saturate",
                    ),
                ] {
                    assert!(got > 0, "{at}");
                }
            }
        }
    };
}

shape!(signed_1_0, Fi, FiMath, signed, 1, 0);
shape!(signed_2_0, Fi, FiMath, signed, 2, 0);
shape!(signed_7_3, Fi, FiMath, signed, 7, 3);
shape!(signed_8_0, Fi, FiMath, signed, 8, 0);
shape!(signed_16_15, Fi, FiMath, signed, 16, 15);
shape!(signed_33_neg4, Fi, FiMath, signed, 33, -4);
shape!(signed_63_0, Fi, FiMath, signed, 63, 0);
shape!(signed_64_10, Fi, FiMath, signed, 64, 10);
shape!(unsigned_1_0, Ufi, UfiMath, unsigned, 1, 0);
shape!(unsigned_8_0, Ufi, UfiMath, unsigned, 8, 0);
shape!(unsigned_16_14, Ufi, UfiMath, unsigned, 16, 14);
shape!(unsigned_63_2, Ufi, UfiMath, unsigned, 63, 2);
shape!(unsigned_64_0, Ufi, UfiMath, unsigned, 64, 0);
