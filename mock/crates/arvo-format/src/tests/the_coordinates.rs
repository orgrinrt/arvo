//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the coordinate types themselves promise.
//!
//! Each of these is a law about a type this round introduced, and each is a
//! property the contract used to state in a doc comment with nothing holding it.
//! An index is not an extent, and an extent of nothing admits no index at all.
//!
//! That the phase and the fraction are two coordinates rather than one ratio
//! written twice is a refusal, and a refusal is a build failure, so it lives in
//! `tests/ui/a_phase_is_not_a_fraction.rs` where it can fail. An arm here asserted
//! instead that the two agree about the numbers they were handed, which is one
//! computation compared against the same computation and could not have failed for
//! any implementation either type could have had.

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
    //
    // A quantum law cannot carry this extent, because its obligation refuses one
    // that ranges over no magnitudes. The coordinate can still be written, which
    // is why the saturation is a property of the type rather than something the
    // obligation makes unreachable.
    let none = MagnitudeCount::of(0);
    assert_eq!(none.largest(), Magnitude::SMALLEST);
    assert!(!Magnitude::SMALLEST.is_within(none).get());
}
