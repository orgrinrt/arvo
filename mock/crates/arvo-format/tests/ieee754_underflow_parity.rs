//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity below the smallest subnormal, on the formats whose other suites read
//! the host.
//!
//! The host's conversion of an integer scaled that far down is two roundings, so
//! the expected side here is the standard's digit rule in integers, `ieee754::rule`,
//! as it is for binary16 and decimal everywhere. The inputs are the shared
//! `ieee754::underflow`: a far value, the significand bound, the first unit past
//! the slot index and one past that, the tie at half that unit where the index
//! holds it, and a sweep from four hundred exponents below the smallest
//! subnormal. The standard's answer there is the smallest subnormal on the side
//! the direction rounds away to, and zero otherwise.

mod ieee754;

use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::Rounding;
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Binary32, Binary64, Decimal64};
use ieee754::{Direction, Landed, Standard, Value, land, misses_over, underflow};

/// Binary32 as the standard's table gives it.
const BINARY32: Standard = Standard {
    radix: 2,
    p:     24,
    qmin:  -149,
    qmax:  104,
};

/// Binary64 as the standard's table gives it.
const BINARY64: Standard = Standard {
    radix: 2,
    p:     53,
    qmin:  -1074,
    qmax:  971,
};

/// Decimal64 as the standard's table gives it.
const DECIMAL64: Standard = Standard {
    radix: 10,
    p:     16,
    qmin:  -398,
    qmax:  369,
};

/// The underflow inputs a format misses under one direction landed by `R`.
fn below<F: Format, R: Rounding>(dir: Direction, f: Standard, seed: u64) -> usize {
    misses_over::<Attributed<F, R, Saturate>>(dir, f, underflow(f, seed))
}

/// The four directions the binary formats carry, each under its own mode.
fn every_direction<F: Format>(f: Standard, seed: u64) -> [usize; 4] {
    [
        below::<F, RoundTiesToEven>(Direction::TiesToEven, f, seed),
        below::<F, RoundTowardPositive>(Direction::TowardPositive, f, seed),
        below::<F, RoundTowardNegative>(Direction::TowardNegative, f, seed),
        below::<F, RoundTowardZero>(Direction::TowardZero, f, seed),
    ]
}

/// The value a landing names, with `finite` asserted.
fn landed_value(landed: Option<Landed>) -> Value {
    let landed = landed.expect("a value below the finest step lands there");
    assert!(
        landed.finite,
        "a value below the finest step is in range there"
    );
    landed.value
}

#[test]
fn binary32_below_the_smallest_subnormal_is_the_digit_rule() {
    assert_eq!(every_direction::<Binary32>(BINARY32, 0x3232_0000), [0; 4]);
}

#[test]
fn binary64_below_the_smallest_subnormal_is_the_digit_rule() {
    assert_eq!(every_direction::<Binary64>(BINARY64, 0x6464_0000), [0; 4]);
}

#[test]
fn decimal64_below_the_smallest_subnormal_is_the_digit_rule() {
    assert_eq!(every_direction::<Decimal64>(DECIMAL64, 0xD64_0000), [0; 4]);
}

#[test]
fn binary64_at_two_to_the_minus_1400_is_the_smallest_subnormal_or_zero() {
    // The exact product of two to the minus 700 with itself.
    let up = landed_value(land::<Attributed<Binary64, RoundTowardPositive, Saturate>>(
        1, 1, -1400,
    ));
    assert_eq!((up.significand, up.exponent), (1, -1074));
    let down = landed_value(land::<Attributed<Binary64, RoundTowardNegative, Saturate>>(
        -1, 1, -1400,
    ));
    assert_eq!((down.significand, down.exponent), (-1, -1074));
    let even = landed_value(land::<Attributed<Binary64, RoundTiesToEven, Saturate>>(
        1, 1, -1400,
    ));
    assert_eq!(even.significand, 0);
}

#[test]
fn decimal64_at_ten_to_the_minus_480_is_the_smallest_subnormal_or_zero() {
    let up = landed_value(land::<Attributed<Decimal64, RoundTowardPositive, Saturate>>(1, 1, -480));
    assert_eq!((up.significand, up.exponent), (1, -398));
    let zero = landed_value(land::<Attributed<Decimal64, RoundTowardZero, Saturate>>(
        1, 1, -480,
    ));
    assert_eq!(zero.significand, 0);
}

#[test]
fn a_half_unit_past_the_index_is_read_as_a_half() {
    // Two to the 126th over two to the 127th steps is a tie at the smallest
    // subnormal's half, and one more is past it, so ties to even reads zero for
    // the one and the smallest subnormal for the other.
    let half = 1_i128 << 126;
    let tie = landed_value(land::<Attributed<Binary64, RoundTiesToEven, Saturate>>(
        half,
        1,
        -1074 - 127,
    ));
    assert_eq!(tie.significand, 0);
    let past = landed_value(land::<Attributed<Binary64, RoundTiesToEven, Saturate>>(
        half + 1,
        1,
        -1074 - 127,
    ));
    assert_eq!((past.significand, past.exponent), (1, -1074));
}

#[test]
fn a_wrong_direction_below_the_smallest_subnormal_is_reported() {
    // The case that has to fail: each direction landed by a neighbouring mode
    // misses on the underflow inputs, so they tell the directions apart.
    assert!(
        below::<Binary64, RoundTowardZero>(Direction::TowardPositive, BINARY64, 0x6464_0000) > 0
    );
    assert!(
        below::<Binary64, RoundTowardZero>(Direction::TowardNegative, BINARY64, 0x6464_0000) > 0
    );
    assert!(
        below::<Binary64, RoundTowardPositive>(Direction::TowardZero, BINARY64, 0x6464_0000) > 0
    );
    assert!(
        below::<Binary64, RoundTowardPositive>(Direction::TiesToEven, BINARY64, 0x6464_0000) > 0
    );
    assert!(
        below::<Decimal64, RoundTowardZero>(Direction::TowardPositive, DECIMAL64, 0xD64_0000) > 0
    );
}
