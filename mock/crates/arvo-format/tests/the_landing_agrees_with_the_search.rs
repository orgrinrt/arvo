//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `land` against the per-magnitude search the IEEE parity suites wrote.
//!
//! The search in `ieee754/mod.rs` finds the finest magnitude at which
//! `panic_on_overflow` holds the rounded position, from the public surface. `land`
//! is that composition moved into the crate, so on every point the search lands
//! below the top the two give the same magnitude and the same slot, and where the
//! search reports the overflow region `land` lands nothing. The points are drawn
//! the way the parity suites draw theirs, over each format's exponent range and
//! past both ends of it.

#[path = "ieee754/mod.rs"]
mod ieee754;

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Scaled, adapt, land};
use arvo_format::format::step_exponent;
use arvo_format::overflow::Saturate;
use arvo_format::quantum::Exponent;
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::standards::ieee754::{
    Attributed,
    Binary16,
    Binary32,
    Binary64,
    Decimal32,
    Decimal64,
};
use ieee754::{Draws, land as search};
use notko::Maybe;

/// How many points `land` and the search disagree on under `S`, and how many the
/// search landed below the top, so a sweep that lands nothing cannot pass.
fn disagreements<S: DeclaredSignature>(points: &[(i128, i32)]) -> (usize, usize) {
    let mut wrong = 0;
    let mut finite = 0;
    for &(num, k) in points {
        let got = land::<S>(Scaled::of(num, 1, Exponent::of(k)), Dither::UNUSED);
        let agrees = match (search::<S>(num, 1, k), got) {
            (Some(want), Maybe::Is(l)) if want.finite => {
                finite += 1;
                step_exponent::<S::Format>(l.magnitude()).power() == want.value.exponent
                    && adapt::<S>(l.exact(), Dither::UNUSED).index() == want.value.significand
            },
            (Some(want), Maybe::Isnt) => !want.finite,
            (None, Maybe::Isnt) => true,
            _ => false,
        };
        wrong += usize::from(!agrees);
    }
    (wrong, finite)
}

/// Numerators up to `2^52` and exponents from `lo` to `hi`, drawn from `seed`.
fn points(seed: u64, lo: i64, hi: i64) -> Vec<(i128, i32)> {
    let mut d = Draws::seeded(seed);
    (0 .. 4000)
        .map(|_| {
            (
                d.within(-(1 << 52), 1 << 52) as i128,
                d.within(lo, hi) as i32,
            )
        })
        .collect()
}

macro_rules! every_direction {
    ($format:ty, $points:expr) => {{
        let points = $points;
        let all = [
            disagreements::<Attributed<$format, HalfEven, Saturate>>(&points),
            disagreements::<Attributed<$format, HalfUp, Saturate>>(&points),
            disagreements::<Attributed<$format, Ceil, Saturate>>(&points),
            disagreements::<Attributed<$format, Floor, Saturate>>(&points),
            disagreements::<Attributed<$format, TowardZero, Saturate>>(&points),
        ];
        for (wrong, finite) in all {
            assert_eq!(wrong, 0, "{} of {} points", wrong, points.len());
            assert!(
                finite > points.len() / 2,
                "only {finite} landed below the top"
            );
        }
    }};
}

#[test]
fn binary16_lands_where_the_search_does() {
    every_direction!(Binary16, points(0x0016_1A4D, -80, 10));
}

#[test]
fn binary32_lands_where_the_search_does() {
    every_direction!(Binary32, points(0x0032_1A4D, -230, 80));
}

#[test]
fn binary64_lands_where_the_search_does() {
    every_direction!(Binary64, points(0x0064_1A4D, -1150, 980));
}

#[test]
fn decimal32_lands_where_the_search_does() {
    every_direction!(Decimal32, points(0xDE32_1A4D, -125, 90));
}

#[test]
fn decimal64_lands_where_the_search_does() {
    every_direction!(Decimal64, points(0xDE64_1A4D, -420, 370));
}
