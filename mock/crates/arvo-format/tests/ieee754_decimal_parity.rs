//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against IEEE 754's decimal interchange formats, under all five
//! rounding-direction attributes.
//!
//! There is no host decimal, so the expected side is the standard's digit rule
//! in integers, `ieee754::rule`, at each format's numbers as the standard's
//! table gives them, reading no arvo code. The arvo side is `Decimal32` and
//! `Decimal64` landed by the composition in `ieee754`, the same composition the
//! binary arms use at the other radix. `roundTiesToAway`, which the standard
//! requires of a decimal format, lands through the Round alias the MATLAB suite
//! writes, since a tie away from zero is not one of this crate's modes.

mod ieee754;
mod ties_away;

use arvo_format::apply::Exact;
use arvo_format::format::{Format, step_exponent};
use arvo_format::overflow::{Overflow, Saturate};
use arvo_format::quantum::Quantum;
use arvo_format::rounding::{HalfUp, Rounding};
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Decimal32, Decimal64};
use ieee754::{DECIMAL32, DECIMAL64, Direction, Landing, Standard, misses};

/// `roundTiesToAway` on a format, through the alias.
struct TiesToAway<F, O>(core::marker::PhantomData<(F, O)>);

impl<F: Format, O: Overflow> Landing for TiesToAway<F, O> {
    type Format = F;

    fn overflows(exact: Exact) -> bool {
        ties_away::overflows::<F, O>(exact)
    }

    fn read_off(exact: Exact) -> i128 {
        ties_away::read_off::<F, O>(exact)
    }
}

fn under<F: Format, R: Rounding>(dir: Direction, f: Standard, seed: u64) -> usize
where
    Attributed<F, R, Saturate>: arvo_format::adapt::DeclaredSignature,
{
    misses::<Attributed<F, R, Saturate>>(dir, f, seed)
}

/// Every direction on one format, as the count of misses per direction.
fn every_direction<F: Format>(f: Standard, seed: u64) -> [usize; 5] {
    [
        under::<F, RoundTiesToEven>(Direction::TiesToEven, f, seed),
        misses::<TiesToAway<F, Saturate>>(Direction::TiesToAway, f, seed),
        under::<F, RoundTowardPositive>(Direction::TowardPositive, f, seed),
        under::<F, RoundTowardNegative>(Direction::TowardNegative, f, seed),
        under::<F, RoundTowardZero>(Direction::TowardZero, f, seed),
    ]
}

#[test]
fn the_exponent_range_is_the_standards() {
    // The one arm that reads the format: its range against the standard's
    // numbers the rule is written at.
    for (low, high, f) in [
        (
            <<Decimal32 as Format>::Quantum as Quantum>::BASE.power(),
            step_exponent::<Decimal32>(
                <<Decimal32 as Format>::Quantum as Quantum>::MAGNITUDES.largest(),
            )
            .power(),
            DECIMAL32,
        ),
        (
            <<Decimal64 as Format>::Quantum as Quantum>::BASE.power(),
            step_exponent::<Decimal64>(
                <<Decimal64 as Format>::Quantum as Quantum>::MAGNITUDES.largest(),
            )
            .power(),
            DECIMAL64,
        ),
    ] {
        assert_eq!((low, high), (f.qmin, f.qmax));
    }
}

#[test]
fn decimal32_under_every_direction_is_the_digit_rule() {
    assert_eq!(every_direction::<Decimal32>(DECIMAL32, 0xD32), [0; 5]);
}

#[test]
fn decimal64_under_every_direction_is_the_digit_rule() {
    assert_eq!(every_direction::<Decimal64>(DECIMAL64, 0xD64), [0; 5]);
}

/// Every direction on one format with a wrong mode in its place: each tie rule
/// as `HalfUp`, each directed mode as its nearest wrong neighbour, toward zero
/// for the two directions and toward positive for toward zero.
fn every_direction_planted<F: Format>(f: Standard, seed: u64) -> [usize; 5] {
    [
        under::<F, HalfUp>(Direction::TiesToEven, f, seed),
        under::<F, HalfUp>(Direction::TiesToAway, f, seed),
        under::<F, RoundTowardZero>(Direction::TowardPositive, f, seed),
        under::<F, RoundTowardZero>(Direction::TowardNegative, f, seed),
        under::<F, RoundTowardPositive>(Direction::TowardZero, f, seed),
    ]
}

#[test]
fn a_planted_decimal_direction_is_reported() {
    for planted in [
        every_direction_planted::<Decimal32>(DECIMAL32, 0xD32),
        every_direction_planted::<Decimal64>(DECIMAL64, 0xD64),
    ] {
        assert!(planted.iter().all(|&m| m > 0), "{planted:?}");
    }
}

#[test]
fn a_planted_precision_is_reported() {
    // The rule at a precision one short of the format's, so the suite is shown
    // to read the format's numbers and not only its directions.
    let short = |f: Standard| {
        Standard {
            p: f.p - 1,
            ..f
        }
    };
    assert!(
        under::<Decimal32, RoundTiesToEven>(Direction::TiesToEven, short(DECIMAL32), 0xD32) > 0
    );
    assert!(
        under::<Decimal64, RoundTiesToEven>(Direction::TiesToEven, short(DECIMAL64), 0xD64) > 0
    );
}
