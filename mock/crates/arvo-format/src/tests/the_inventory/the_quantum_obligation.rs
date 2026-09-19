//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a quantum law owes, and the constructions that do not meet it.
//!
//! The same shape as the slot range's obligation, one contract over. Both
//! conditions were stated where nothing held them, the first in a doc comment and
//! the second nowhere at all, so these are the first constructions ever written
//! against either.

use super::DoubleStepped;
use crate::quantum::{Constant, Exponent, Indexed, MagnitudeCount, Quantum};

/// A law over no magnitudes, which describes no values.
///
/// It compiles, which is the point: nothing about the three coordinates stops it
/// being written. Using it does not build, and the `trybuild` case records that.
struct NoMagnitudes;

impl Quantum for NoMagnitudes {
    const BASE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(0);
    const SLOPE: Exponent = Exponent::ONE;
}

/// A law whose exponent runs past what an exponent carries before its last
/// magnitude.
///
/// The sum overflows in the exponent's own width, which panics under
/// `overflow-checks` and wraps without it, and either way the crate would have
/// answered with a step law it does not have.
struct ReachRunsOff;

impl Quantum for ReachRunsOff {
    const BASE: Exponent = Exponent::of(i32::MAX - 2);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(8);
    const SLOPE: Exponent = Exponent::ONE;
}

/// A law with more magnitudes than a magnitude index can hold.
///
/// The other narrowing in the same arithmetic: the index is cast down into the
/// exponent's width before the multiply, so a count above what a signed 32-bit
/// integer carries wraps the index itself rather than the sum.
struct MagnitudesBeyondTheIndex;

impl Quantum for MagnitudesBeyondTheIndex {
    const BASE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(u32::MAX);
    const SLOPE: Exponent = Exponent::ONE;
}

#[test]
fn the_law_rejects_a_quantum_law_that_does_not_meet_the_contract() {
    assert!(
        !crate::quantum::is_admissible_quantum::<NoMagnitudes>().get(),
        "a law over no magnitudes was admitted, and it describes no values"
    );
    assert!(
        !crate::quantum::is_admissible_quantum::<ReachRunsOff>().get(),
        "a law whose exponent runs off the end was admitted"
    );
    assert!(
        !crate::quantum::is_admissible_quantum::<MagnitudesBeyondTheIndex>().get(),
        "a law with more magnitudes than an index carries was admitted"
    );
}

#[test]
fn the_law_admits_every_quantum_law_this_crate_ships() {
    // The control. A law refusing everything would pass the test above and
    // establish nothing, so it has to accept both shipped families across their
    // parameters rather than at the one instantiation somebody remembered.
    macro_rules! admits_constant {
        ($($e:literal),+ $(,)?) => {
            $(
                assert!(
                    crate::quantum::is_admissible_quantum::<Constant<$e>>().get(),
                    "the constant family at exponent {} was refused", $e
                );
            )+
        };
    }
    admits_constant!(
        -30,
        -24,
        -17,
        -8,
        -4,
        -1,
        0,
        1,
        4,
        8,
        17,
        24,
        30,
        2147483647,
        -2147483648
    );

    macro_rules! admits_indexed {
        ($(($e:literal, $c:literal)),+ $(,)?) => {
            $(
                assert!(
                    crate::quantum::is_admissible_quantum::<Indexed<$e, $c>>().get(),
                    "the indexed family at ({}, {}) was refused", $e, $c
                );
            )+
        };
    }
    admits_indexed!(
        (-14, 30),
        (-126, 254),
        (-1022, 2046),
        (0, 1),
        (-3, 7),
        (2147483646, 1),
    );

    // And the foreign law in the parent, which is neither shipped shape, so the
    // verdict is about the obligations rather than about the two families this
    // crate happens to write.
    assert!(crate::quantum::is_admissible_quantum::<DoubleStepped>().get());
}

#[test]
fn the_quantum_law_separates_the_two_constructions_rather_than_answering_one_way() {
    // Both directions in one place, so a verdict stuck at `true` or at `false`
    // fails here rather than passing one of the two tests above.
    let shipped = crate::quantum::is_admissible_quantum::<Indexed<-14, 30>>().get();
    let refused = crate::quantum::is_admissible_quantum::<NoMagnitudes>().get();
    assert_ne!(
        shipped, refused,
        "the verdict gives the same answer to a shipped law and one over no magnitudes"
    );

    // And the two conditions are separable: each refused construction trips one
    // of them and passes the other, so neither condition is doing all the work.
    assert_eq!(
        <NoMagnitudes as Quantum>::BASE,
        <Constant<0> as Quantum>::BASE,
        "the no-magnitude law would be admissible but for its count"
    );
    assert!(
        <ReachRunsOff as Quantum>::MAGNITUDES.count() >= 1,
        "the runaway law trips the count condition too, so it separates nothing"
    );
    assert!(
        <MagnitudesBeyondTheIndex as Quantum>::MAGNITUDES.count() >= 1,
        "the wide-count law trips the count condition too, so it separates nothing"
    );
}
