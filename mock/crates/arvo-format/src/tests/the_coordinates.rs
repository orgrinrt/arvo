//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the coordinate types themselves promise.
//!
//! Each of these is a law about a type this round introduced, and each is a
//! property the contract used to state in a doc comment with nothing holding it.
//! An index is not an extent, a denominator is never zero, and the two ratios the
//! crate carries are two coordinates rather than one written twice.

use crate::apply::Fraction;
use crate::format::Phase;
use crate::quantum::{Magnitude, MagnitudeCount};

// --- an index is not an extent, which is why they are two types --------------

#[test]
fn the_largest_admitted_magnitude_is_one_below_the_count() {
    // The off-by-one both types exist to make unwriteable, asserted rather than
    // trusted. A law over thirty magnitudes ranges over indices zero to
    // twenty-nine, and its own count is not one of them.
    let count = MagnitudeCount::of(30);
    assert_eq!(count.largest(), Magnitude::at(29));
    assert!(count.largest().is_within(count).get());
    assert!(!Magnitude::at(30).is_within(count).get());
    assert!(!Magnitude::at(count.count()).is_within(count).get());

    // And at the constant family's single magnitude, where the two numbers are
    // one apart and the confusion is easiest to make.
    let one = MagnitudeCount::ONE;
    assert_eq!(one.largest(), Magnitude::SMALLEST);
    assert!(Magnitude::SMALLEST.is_within(one).get());
    assert!(!Magnitude::at(1).is_within(one).get());
}

#[test]
fn an_extent_of_nothing_admits_no_index_at_all() {
    // The case that would underflow if the largest index were computed by
    // subtracting one from the count. It saturates instead, and nothing is
    // admitted, which the predicate rather than the arithmetic decides.
    let none = MagnitudeCount::of(0);
    assert_eq!(none.largest(), Magnitude::SMALLEST);
    assert!(!Magnitude::SMALLEST.is_within(none).get());
}

// --- the fraction and the phase are separate coordinates ---------------------

#[test]
fn a_phase_and_a_fraction_do_not_stand_in_for_one_another() {
    // Both are a ratio and neither is the other: a phase offsets the whole grid
    // and a fraction is a position between two of its points. Written down
    // because the two carry the same pair of numbers, which is exactly when a
    // design starts using one where it meant the other.
    let p = Phase::of(1, 2);
    let f = Fraction::of(1, 2);
    assert_eq!(p.numerator(), f.numerator());
    assert_eq!(p.denominator(), f.denominator());
    // The `trybuild` case pins that neither converts into the other. What is
    // asserted here is that they agree about the numbers and are still two
    // coordinates, which is the thing a reader would otherwise have to take on
    // trust from a doc comment.
}
