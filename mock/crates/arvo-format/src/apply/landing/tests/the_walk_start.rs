//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where the walk starts, at the points a bound reading too little skips past.

use super::*;

#[test]
fn a_negative_shift_on_a_decimal_law_starts_the_walk_early_enough() {
    // `2^126` is `8507059173023461586.58...` times `10^19`, in range on that step
    // and past it on `10^18`. A bound reading `10^-1` as at least `2^-3` starts the
    // walk at `10^20` and drops a digit.
    let got = land::<Under<ZeroDecimal, TowardZero>>(
        Scaled::of(1 << 126, nz(1), Exponent::ZERO),
        Dither::UNUSED,
    );
    assert!(
        matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::at(119)
        && l.exact().slot() == Slot::at(8_507_059_173_023_461_586))
    );
}

/// A falling law over 101 magnitudes, its steps from `2^100` down to `2^0`.
struct LongFalling;

impl Quantum for LongFalling {
    const BASE: Exponent = Exponent::of(100);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(101);
    const SLOPE: Exponent = Exponent::of(-1);
}

struct LongFallingPhased;

impl Format for LongFallingPhased {
    type Ambient = BinaryRationals;
    type Quantum = LongFalling;
    type Slots = Signed<4>;

    const PHASE: Phase = Phase::halves(1);
}

#[test]
fn a_falling_phased_law_starts_the_walk_at_its_finest_magnitude() {
    // The phase is a half of `2^100`, so `2^99 + 3` and `2^99 - 5` are slots 3 and
    // -5 on the finest step `2^0`, magnitude 100. A walk start that reads only how
    // far the point is from zero skips 33 magnitudes and lands them off the grid.
    for (num, slot) in [((1_i128 << 99) + 3, 3), ((1 << 99) - 5, -5)] {
        let got = land::<Under<LongFallingPhased, TowardZero>>(
            Scaled::of(num, nz(1), Exponent::ZERO),
            Dither::UNUSED,
        );
        assert!(
            matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::at(100)
            && l.exact() == Exact::on_grid(Slot::at(slot)))
        );
    }
}
