//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every fimath rounding method and overflow action, against MathWorks'
//! definitions written as integer rules.
//!
//! `matlab_fi_parity.rs` holds the published worked examples, which reach no tie
//! and no overflow. This file holds the rest of the declaration set: each method
//! name `standards::matlab` exports, the Round alias, and both overflow actions,
//! over positions in quarters of a step from well below the range to well above
//! it, so every arm passes ties of both signs and both ends.
//!
//! The expected side is each method as MathWorks defines it, in exact integers
//! over the position counted in quarters, and reads no arvo code: Floor and
//! Ceiling toward the infinities, Zero toward zero, Nearest to the nearest with a
//! tie toward positive infinity, Convergent with a tie to the even neighbour, and
//! Round with a tie away from zero. Saturate pins to the range and Wrap is two's
//! complement modulo two to the word length.

mod ties_away;

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::slots::Slot;
use arvo_format::standards::matlab::overflow_action::{Saturate, Wrap};
use arvo_format::standards::matlab::rounding_method::{Ceiling, Convergent, Floor, Nearest, Zero};
use arvo_format::standards::matlab::{Fi, FiMath};

/// The word length every arm runs at.
const W: u32 = 8;

/// The declaration every arm runs over: a signed `fi` of eight bits at fraction
/// length zero, so a slot is the stored integer.
type F = Fi<8, 0>;

/// Positions from far below the range to far above it, counted in quarters.
const QUARTERS: core::ops::RangeInclusive<i128> = -1300 ..= 1300;

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

fn saturate(r: i128) -> i128 {
    r.clamp(-(1 << (W - 1)), (1 << (W - 1)) - 1)
}

fn wrap(r: i128) -> i128 {
    (r + (1 << (W - 1))).rem_euclid(1 << W) - (1 << (W - 1))
}

// --- the arm ----------------------------------------------------------------------

fn position(q: i128) -> Exact {
    Exact::between(
        Slot::at(q.div_euclid(4)),
        Fraction::of(q.rem_euclid(4) as i64, 4),
    )
}

/// How many positions a declared signature answers differently from the rule and
/// the action.
fn misses<S: DeclaredSignature>(rule: fn(i128) -> i128, action: fn(i128) -> i128) -> usize {
    QUARTERS
        .filter(|&q| adapt::<S>(position(q), Dither::UNUSED).index() != action(rule(q)))
        .count()
}

/// The Round alias, under an overflow action, against the same rule.
fn round_misses<O: arvo_format::overflow::Overflow>(action: fn(i128) -> i128) -> usize {
    QUARTERS
        .filter(|&q| {
            ties_away::select::<F, O>(q.div_euclid(4), q.rem_euclid(4) as i64, 4)
                != action(round(q))
        })
        .count()
}

#[test]
fn every_rounding_method_under_saturate_is_mathworks() {
    assert_eq!(misses::<FiMath<8, 0, Floor, Saturate>>(floor, saturate), 0);
    assert_eq!(
        misses::<FiMath<8, 0, Ceiling, Saturate>>(ceiling, saturate),
        0
    );
    assert_eq!(misses::<FiMath<8, 0, Zero, Saturate>>(zero, saturate), 0);
    assert_eq!(
        misses::<FiMath<8, 0, Nearest, Saturate>>(nearest, saturate),
        0
    );
    assert_eq!(
        misses::<FiMath<8, 0, Convergent, Saturate>>(convergent, saturate),
        0
    );
    assert_eq!(round_misses::<Saturate>(saturate), 0);
}

#[test]
fn every_rounding_method_under_wrap_is_mathworks() {
    assert_eq!(misses::<FiMath<8, 0, Floor, Wrap>>(floor, wrap), 0);
    assert_eq!(misses::<FiMath<8, 0, Ceiling, Wrap>>(ceiling, wrap), 0);
    assert_eq!(misses::<FiMath<8, 0, Zero, Wrap>>(zero, wrap), 0);
    assert_eq!(misses::<FiMath<8, 0, Nearest, Wrap>>(nearest, wrap), 0);
    assert_eq!(
        misses::<FiMath<8, 0, Convergent, Wrap>>(convergent, wrap),
        0
    );
    assert_eq!(round_misses::<Wrap>(wrap), 0);
}

// --- the planted arms -------------------------------------------------------------
//
// Each arm with its method or action swapped for a wrong one. Every planted arm
// is required to miss, so the sweep is shown able to tell each method from its
// nearest neighbour and each action from the other.

#[test]
fn a_planted_method_is_reported() {
    // The nearest rules differ only at ties, and each directed rule from its
    // mirror only off the grid, so these are the hardest swaps to see.
    assert!(misses::<FiMath<8, 0, Nearest, Saturate>>(convergent, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Convergent, Saturate>>(nearest, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Zero, Saturate>>(floor, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Floor, Saturate>>(ceiling, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Ceiling, Saturate>>(zero, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Nearest, Saturate>>(round, saturate) > 0);
}

#[test]
fn a_planted_action_is_reported() {
    assert!(misses::<FiMath<8, 0, Floor, Wrap>>(floor, saturate) > 0);
    assert!(misses::<FiMath<8, 0, Floor, Saturate>>(floor, wrap) > 0);
    assert!(round_misses::<Wrap>(saturate) > 0);
}
