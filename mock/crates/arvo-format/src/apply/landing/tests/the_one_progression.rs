//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether the set is one progression, against the set written out.
//!
//! Each law here is small enough to list every member in finest steps, sort them
//! and look at the gaps, which is the definition rather than the walk
//! `is_one_progression` does. The degenerate laws with a slope are the cases the
//! slope test `is_constant_family` gets wrong, and they are the must-fail control.

use super::*;
use crate::format::is_one_progression;
use crate::quantum::is_constant_family;
use crate::slots::{Slots, Unsigned};
use crate::width::Width;

macro_rules! law {
    ($name:ident, $magnitudes:literal, $slope:literal) => {
        struct $name;

        impl Quantum for $name {
            const BASE: Exponent = Exponent::ZERO;
            const MAGNITUDES: MagnitudeCount = MagnitudeCount::of($magnitudes);
            const SLOPE: Exponent = Exponent::of($slope);
        }
    };
}

law!(Two, 2, 1);
law!(Three, 3, 1);
law!(TwoByFour, 2, 2);
law!(ThreeByFour, 3, 2);
law!(FallingTwo, 2, -1);
law!(FallingThree, 3, -1);

macro_rules! range {
    ($name:ident, $min:literal, $max:literal, $width:literal) => {
        struct $name;

        impl Slots for $name {
            const MAX: Slot = Slot::at($max);
            const MIN: Slot = Slot::at($min);
            const WIDTH: Width = Width::bits($width);
        }
    };
}

range!(Around, -1, 1, 2);
range!(One, 1, 1, 1);
range!(MinusOne, -1, -1, 1);
range!(BelowZero, -2, -1, 2);

macro_rules! set {
    ($name:ident, $quantum:ty, $slots:ty) => {
        struct $name;

        impl Format for $name {
            type Ambient = BinaryRationals;
            type Quantum = $quantum;
            type Slots = $slots;

            const PHASE: Phase = Phase::halves(1);
        }
    };
}

set!(Flat, crate::quantum::Constant<0>, Signed<4>);
set!(BitTwo, Two, Unsigned<1>);
set!(AroundTwo, Two, Around);
set!(OneTwoByFour, TwoByFour, One);
set!(OneThreeByFour, ThreeByFour, One);
set!(BitThree, Three, Unsigned<1>);
set!(TwoBitsThree, Three, Unsigned<2>);
set!(AroundThree, Three, Around);
set!(Floats, Indexed<-4, 6>, Signed<4>);
set!(BitFallingTwo, FallingTwo, Unsigned<1>);
set!(BitFallingThree, FallingThree, Unsigned<1>);
set!(MinusOneTwo, Two, MinusOne);
set!(MinusOneThree, Three, MinusOne);
set!(BelowZeroTwo, Two, BelowZero);

/// A decimal set: the same slots over the same law hold other members.
macro_rules! decimal {
    ($name:ident, $quantum:ty, $slots:ty) => {
        struct $name;

        impl Format for $name {
            type Ambient = DecimalRationals;
            type Quantum = $quantum;
            type Slots = $slots;

            const PHASE: Phase = Phase::ZERO;
        }
    };
}

decimal!(DecimalBitTwo, Two, Unsigned<1>);
decimal!(DecimalOneTwo, Two, One);

/// Whether the members of `F`, written out in finest steps, are equally spaced.
fn written_out<F: Format>() -> bool {
    let r = radix::<F>().base() as i128;
    let slope = <F::Quantum as Quantum>::SLOPE.power().unsigned_abs();
    let count = <F::Quantum as Quantum>::MAGNITUDES.count();
    let (lo, hi) = (
        <F::Slots as Slots>::MIN.index(),
        <F::Slots as Slots>::MAX.index(),
    );
    let mut members = [0_i128; 256];
    let mut n = 0;
    for m in 0 .. count {
        let scale = power(r, (slope * m) as i32);
        for k in lo ..= hi {
            let v = k * scale;
            if !members[.. n].contains(&v) {
                members[n] = v;
                n += 1;
            }
        }
    }
    members[.. n].sort_unstable();
    n <= 2
        || members[.. n]
            .windows(2)
            .all(|w| w[1] - w[0] == members[1] - members[0])
}

#[test]
fn a_constant_quantum_is_one_progression() {
    assert!(is_one_progression::<Flat>().get());
    assert!(written_out::<Flat>());
}

#[test]
fn a_degenerate_law_with_a_slope_is_one_progression_and_the_slope_test_misses_it() {
    for (set, slope_test) in [
        (
            is_one_progression::<BitTwo>().get(),
            is_constant_family::<Two>().get(),
        ),
        (
            is_one_progression::<AroundTwo>().get(),
            is_constant_family::<Two>().get(),
        ),
        (
            is_one_progression::<OneTwoByFour>().get(),
            is_constant_family::<TwoByFour>().get(),
        ),
    ] {
        assert!(set);
        assert!(!slope_test);
    }
    assert!(written_out::<BitTwo>());
    assert!(written_out::<AroundTwo>());
    assert!(written_out::<OneTwoByFour>());
}

#[test]
fn a_set_with_a_gap_is_not_one_progression() {
    assert!(!is_one_progression::<OneThreeByFour>().get());
    assert!(!is_one_progression::<BitThree>().get());
    assert!(!is_one_progression::<TwoBitsThree>().get());
    assert!(!is_one_progression::<AroundThree>().get());
    assert!(!is_one_progression::<Floats>().get());
    for written in [
        written_out::<OneThreeByFour>(),
        written_out::<BitThree>(),
        written_out::<TwoBitsThree>(),
        written_out::<AroundThree>(),
        written_out::<Floats>(),
    ] {
        assert!(!written);
    }
}

#[test]
fn a_falling_law_is_read_as_the_rising_one_it_mirrors() {
    // Two magnitudes a factor of two apart hold `{0, 1}` and `{0, 2}` in finest
    // steps, whichever way the step runs, and three add a gap at 3.
    assert!(is_one_progression::<BitFallingTwo>().get());
    assert!(!is_constant_family::<FallingTwo>().get());
    assert!(written_out::<BitFallingTwo>());
    assert!(!is_one_progression::<BitFallingThree>().get());
    assert!(!written_out::<BitFallingThree>());
}

#[test]
fn the_radix_decides_whether_the_steps_meet() {
    // In tenths the bit holds `{0, 1, 10}`, where in halves it held `{0, 1, 2}`.
    assert!(is_one_progression::<BitTwo>().get());
    assert!(!is_one_progression::<DecimalBitTwo>().get());
    assert!(!written_out::<DecimalBitTwo>());
    // Two members are one progression in any radix.
    assert!(is_one_progression::<DecimalOneTwo>().get());
    assert!(written_out::<DecimalOneTwo>());
}

#[test]
fn a_range_on_one_side_of_zero_is_one_progression_only_while_it_holds_two_members() {
    // `{-1, -2}` over two magnitudes, `{-1, -2, -4}` over three, and
    // `{-1, -2, -4}` again from the two slots `-2` and `-1` over two.
    assert!(is_one_progression::<MinusOneTwo>().get());
    assert!(written_out::<MinusOneTwo>());
    assert!(!is_one_progression::<MinusOneThree>().get());
    assert!(!written_out::<MinusOneThree>());
    assert!(!is_one_progression::<BelowZeroTwo>().get());
    assert!(!written_out::<BelowZeroTwo>());
}
