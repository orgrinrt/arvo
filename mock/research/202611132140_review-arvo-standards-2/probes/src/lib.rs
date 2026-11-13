//! Review round 2 probes at `9e270619`. Ad-hoc quick spikes, not benches.
//! Raw output is in `README.md` beside this crate.
//!
//! The crate is its tests and nothing else, so it builds only under test.

#![cfg(test)]

#[path = "../../../../crates/arvo-format/tests/ieee754/mod.rs"]
mod ieee754;

use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::is_admissible_format;
use arvo_format::overflow::Saturate;
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Decimal32, Decimal64, Interchange};
use ieee754::{Direction, Standard, misses};

/// Decimal32 and decimal64 as the standard's table gives them.
const DECIMAL32: Standard = Standard {
    radix: 10,
    p:     7,
    qmin:  -101,
    qmax:  90,
};
const DECIMAL64: Standard = Standard {
    radix: 10,
    p:     16,
    qmin:  -398,
    qmax:  369,
};

type Unsigned = Interchange<UnsignedBinaryRationals, 24, 127>;

/// The format-level verdict calls an unsigned interchange admissible, though
/// every reader of its slot range refuses it at compile time.
#[test]
fn the_format_verdict_admits_the_unsigned_interchange() {
    assert!(is_admissible_format::<Unsigned>().get());
}

/// The planted arms the decimal suite does not carry, each required to miss.
#[test]
fn the_missing_decimal_planted_arms_are_writable_and_miss() {
    let d32_pos = misses::<Attributed<Decimal32, RoundTowardZero, Saturate>>(
        Direction::TowardPositive,
        DECIMAL32,
        0xD32,
    );
    let d32_zero = misses::<Attributed<Decimal32, RoundTowardNegative, Saturate>>(
        Direction::TowardZero,
        DECIMAL32,
        0xD32,
    );
    let d64_pos = misses::<Attributed<Decimal64, RoundTowardZero, Saturate>>(
        Direction::TowardPositive,
        DECIMAL64,
        0xD64,
    );
    let d64_neg = misses::<Attributed<Decimal64, RoundTowardPositive, Saturate>>(
        Direction::TowardNegative,
        DECIMAL64,
        0xD64,
    );
    let d64_even_short = misses::<Attributed<Decimal64, RoundTiesToEven, Saturate>>(
        Direction::TiesToEven,
        ieee754::Standard {
            p: DECIMAL64.p - 1,
            ..DECIMAL64
        },
        0xD64,
    );
    // Added at `770d4d67` so the shared module compiles here with every variant
    // used: ties to even checked against the ties-to-away rule must miss.
    let d32_away_as_even = misses::<Attributed<Decimal32, RoundTiesToEven, Saturate>>(
        Direction::TiesToAway,
        DECIMAL32,
        0xD32,
    );
    assert!(d32_away_as_even > 0);
    println!(
        "d32 +inf as zero {d32_pos}, d32 zero as -inf {d32_zero}, d64 +inf as zero {d64_pos}, d64 -inf as +inf {d64_neg}, d64 short p {d64_even_short}"
    );
    assert!(d32_pos > 0 && d32_zero > 0 && d64_pos > 0 && d64_neg > 0 && d64_even_short > 0);
}

/// `EMAX = 0` names a format with no magnitudes.
#[cfg(feature = "emax0")]
#[test]
fn an_emax_of_zero() {
    use arvo_format::format::contains;
    use arvo_format::quantum::Magnitude;
    use arvo_format::slots::Slot;
    use arvo_format::standards::ieee754::Binary;
    let c = contains::<Binary<24, 0>>(Slot::ZERO, Magnitude::SMALLEST).get();
    panic!("Binary<24, 0> contains zero at the smallest magnitude: {c}");
}

/// The slot-range verdict form on the unsigned significand.
#[cfg(feature = "slots_verdict")]
#[test]
fn the_slot_verdict_on_an_unsigned_significand() {
    use arvo_format::slots::is_admissible;
    use arvo_format::standards::ieee754::Significand;
    let v = is_admissible::<Significand<UnsignedBinaryRationals, 24>>().get();
    panic!("slots::is_admissible on the unsigned significand: {v}");
}
