//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `land`, against a walk over every magnitunknown = Maybe::Is(unknown.map_or(step, |u| u.min(step)));de.
//!
//! The oracle places the point at every magnitude in the host's 128-bit integer,
//! keeps the magnitudes whose rounded position `panic_on_overflow` holds in range,
//! and takes the one with the smallest step, the first such index at a tie. It
//! skips nothing and orders nothing by the law's slope, so it checks both the walk
//! and the magnitudes `land` does not walk. Where the host integer cannot form a
//! position, the position is past the slot index of every format swept here.

use notko::Maybe;

use super::*;
use crate::adapt::{Adapt, Signature};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Fraction, Remainder, adapt};
use crate::format::Phase;
use crate::overflow::Saturate;
use crate::points::{Biased, Floating, Integer};
use crate::quantum::{Indexed, MagnitudeCount};
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use crate::slots::{Signed, Slot};
use crate::wide::Wide;

/// A law whose step falls with magnitude, so its finest magnitude is its last.
struct Falling;

impl Quantum for Falling {
    const BASE: Exponent = Exponent::of(1);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(6);
    const SLOPE: Exponent = Exponent::of(-1);
}

macro_rules! format {
    ($name:ident, $ambient:ty, $quantum:ty, $bits:literal, $phase:expr) => {
        struct $name;

        impl Format for $name {
            type Ambient = $ambient;
            type Quantum = $quantum;
            type Slots = Signed<$bits>;

            const PHASE: Phase = $phase;
        }
    };
}

format!(FallingFour, BinaryRationals, Falling, 4, Phase::ZERO);
format!(
    PhasedFour,
    BinaryRationals,
    Indexed<-4, 6>,
    4,
    Phase::halves(1)
);
format!(
    DecimalFive,
    DecimalRationals,
    Indexed<-3, 5>,
    5,
    Phase::ZERO
);
format!(
    WideBinary,
    BinaryRationals,
    Indexed<-300, 400>,
    64,
    Phase::ZERO
);
format!(
    WideDecimal,
    DecimalRationals,
    Indexed<-100, 200>,
    64,
    Phase::of(-1, 3)
);

type Under<F, R> = Signature<F, Adapt<R, Saturate>>;

/// `r^e` in the host integer, or nothing past it.
fn power(r: i128, e: u32) -> Maybe<i128> {
    let mut p: i128 = 1;
    for _ in 0 .. e {
        match p.checked_mul(r) {
            Some(next) => p = next,
            None => return Maybe::Isnt,
        }
    }
    Maybe::Is(p)
}

/// What the host integer makes of the point at one magnitude.
enum Formed {
    /// The position.
    At(Exact),
    /// Past the slot index: a shift that rises and a numerator past the host
    /// integer, which over the denominators and phases swept here is a position
    /// past `2^64`.
    Past,
    /// A position the host integer cannot form and this oracle cannot place.
    Unknown,
}

/// The point's position at `magnitude`, formed in the host integer.
fn placed<F: Format>(num: i128, den: u128, k: i32, magnitude: Magnitude) -> Formed {
    let r = radix::<F>().base() as i128;
    let p = F::PHASE.numerator() as i128;
    let q = F::PHASE.denominator() as i128;
    let Ok(den) = i128::try_from(den) else {
        return Formed::Unknown;
    };
    let shift = k as i64 - step_exponent::<F>(magnitude).power() as i64;
    let formed = (|| {
        let (up, down) = if shift < 0 { (0, -shift) } else { (shift, 0) };
        let Maybe::Is(up) = power(r, u32::try_from(up).ok()?) else {
            return None;
        };
        let Maybe::Is(down) = power(r, u32::try_from(down).ok()?) else {
            return None;
        };
        let n = num
            .checked_mul(q)?
            .checked_mul(up)?
            .checked_sub(p.checked_mul(den)?.checked_mul(down)?)?;
        let d = den.checked_mul(q)?.checked_mul(down)?;
        Some(Exact::at(
            Slot::at(n.div_euclid(d)),
            Remainder::of(n.rem_euclid(d) as u128, d as u128),
        ))
    })();
    match formed {
        Some(exact) => Formed::At(exact),
        None if shift >= 0 && num != 0 => Formed::Past,
        None => Formed::Unknown,
    }
}

/// The magnitude the oracle lands the point on and its position there, nothing
/// where no magnitude holds it, or `Isnt` where a magnitude it cannot place is
/// finer than its pick.
fn walked<S: DeclaredSignature>(
    num: i128,
    den: u128,
    k: i32,
    dither: Dither,
) -> Maybe<Maybe<(Magnitude, Exact)>> {
    let count = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut best: Maybe<(i32, Magnitude, Exact)> = Maybe::Isnt;
    let mut unknown: Maybe<i32> = Maybe::Isnt;
    for m in 0 .. count {
        let magnitude = Magnitude::at(m);
        let step = step_exponent::<S::Format>(magnitude).power();
        let exact = match placed::<S::Format>(num, den, k, magnitude) {
            Formed::At(exact) => exact,
            Formed::Past => continue,
            Formed::Unknown => {
                unknown = Maybe::Is(unknown.map_or(step, |u| u.min(step)));
                continue;
            },
        };
        if panic_on_overflow::<S>(exact, dither).get() {
            continue;
        }
        if best.is_none_or(|(s, ..)| step < s) {
            best = Maybe::Is((step, magnitude, exact));
        }
    }
    // A magnitude the oracle cannot place matters only where it is finer than
    // the one it picked, since `land` takes the finest.
    match (best, unknown) {
        (Maybe::Is((s, ..)), Maybe::Is(u)) if u < s => Maybe::Isnt,
        (Maybe::Isnt, Maybe::Is(_)) => Maybe::Isnt,
        _ => Maybe::Is(best.map(|(_, m, e)| (m, e))),
    }
}

/// A remainder in lowest terms.
///
/// The oracle forms its positions in the host's 128-bit integer, so a remainder
/// that does not read back in it is one the oracle never named.
fn lowest(r: Remainder) -> (u128, u128) {
    let (Maybe::Is(num), Maybe::Is(den)) = (r.numerator(), r.denominator()) else {
        panic!("{r:?}: a remainder past 128 bits, which this oracle does not form");
    };
    let (mut a, mut b) = (num, den);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (num / a, den / a)
}

/// The same position, compared as a slot and a ratio rather than as a pair.
fn same_position(a: Exact, b: Exact) -> bool {
    a.slot() == b.slot() && a.past == b.past && lowest(a.part) == lowest(b.part)
}

/// A set of points `num / den * radix^k`, visited one at a time.
trait Points {
    fn each(visit: impl FnMut(i128, u128, i32));
}

/// Every point over small numerators and denominators.
struct Small;

impl Points for Small {
    fn each(mut visit: impl FnMut(i128, u128, i32)) {
        for num in -300 ..= 300 {
            for den in [1, 2, 3, 7, 10] {
                for k in -12 ..= 6 {
                    visit(num, den, k);
                }
            }
        }
    }
}

/// Points whose landing sits a few magnitudes from where the walk starts on a
/// format whose slots span 64 bits, over shifts the small sweep never reaches.
struct Far;

impl Points for Far {
    fn each(mut visit: impl FnMut(i128, u128, i32)) {
        for num in [1, -1, 3, 1 << 40, -(1 << 62) - 5, i64::MAX as i128, (1 << 100) + 7] {
            for den in [1, 3, 1 << 20, (1 << 61) + 1] {
                for k in (-250 ..= 250).step_by(7) {
                    visit(num, den, k);
                }
            }
        }
    }
}

/// How many points land elsewhere than the oracle says, under `S`. At least half
/// of the points are ones the oracle places, so a sweep it cannot read fails.
fn misses<S: DeclaredSignature, P: Points>(dither: Dither) -> usize {
    let (mut wrong, mut placed, mut all) = (0, 0, 0);
    P::each(|num, den, k| {
        all += 1;
        let Maybe::Is(want) = walked::<S>(num, den, k, dither) else {
            return;
        };
        placed += 1;
        let got = land::<S>(Scaled::of(num, den, Exponent::of(k)), dither);
        let agrees = match (got, want) {
            (Maybe::Isnt, Maybe::Isnt) => true,
            (Maybe::Is(l), Maybe::Is((m, e))) => {
                l.magnitude() == m
                    && same_position(l.exact(), e)
                    && adapt::<S>(l.exact(), dither) == adapt::<S>(e, dither)
            },
            _ => false,
        };
        wrong += usize::from(!agrees);
    });
    assert!(
        placed * 2 >= all,
        "the oracle placed {placed} of {all} points"
    );
    wrong
}

const DITHERS: [Dither; 3] =
    [Dither::UNUSED, Dither::at(Fraction::of(1, 3)), Dither::at(Fraction::of(2, 3))];

macro_rules! every_mode {
    ($format:ty, $points:ty) => {{
        let mut total = 0;
        total += misses::<Under<$format, TowardZero>, $points>(Dither::UNUSED);
        total += misses::<Under<$format, Floor>, $points>(Dither::UNUSED);
        total += misses::<Under<$format, Ceil>, $points>(Dither::UNUSED);
        total += misses::<Under<$format, HalfUp>, $points>(Dither::UNUSED);
        total += misses::<Under<$format, HalfEven>, $points>(Dither::UNUSED);
        for dither in DITHERS {
            total += misses::<Under<$format, Stochastic>, $points>(dither);
        }
        total
    }};
}

#[test]
fn a_rising_binary_law_lands_on_the_finest_magnitude_in_range() {
    assert_eq!(every_mode!(Floating<4, -6, 8>, Small), 0);
}

#[test]
fn a_falling_law_lands_on_its_last_magnitude_first() {
    assert_eq!(every_mode!(FallingFour, Small), 0);
    // A third at radix two is finest on the step `2^-4`, magnitude five.
    let got =
        land::<Under<FallingFour, HalfEven>>(Scaled::of(1, 3, Exponent::ZERO), Dither::UNUSED);
    assert!(matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::at(5)));
}

#[test]
fn a_phase_moves_the_position_at_every_magnitude() {
    assert_eq!(every_mode!(PhasedFour, Small), 0);
}

#[test]
fn a_decimal_law_lands_on_the_finest_magnitude_in_range() {
    assert_eq!(every_mode!(DecimalFive, Small), 0);
}

#[test]
fn a_constant_quantum_lands_where_its_one_range_holds() {
    assert_eq!(every_mode!(Integer<4>, Small), 0);
    assert_eq!(every_mode!(Biased<4, -2, 1>, Small), 0);
}

#[test]
fn the_walk_starts_no_later_than_the_finest_magnitude_in_range() {
    assert_eq!(every_mode!(WideBinary, Far), 0);
    assert_eq!(every_mode!(WideDecimal, Far), 0);
}

#[test]
fn a_zero_denominator_reads_as_zero_the_way_a_remainder_s_does() {
    assert_eq!(
        Scaled::of(5, 0, Exponent::of(3)),
        Scaled::of(0, 1, Exponent::of(3))
    );
    assert_eq!(Remainder::of(5, 0), Remainder::ZERO);
    let got =
        land::<Under<Floating<4, -6, 8>, Floor>>(Scaled::of(5, 0, Exponent::of(3)), Dither::UNUSED);
    assert!(
        matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::SMALLEST
        && l.exact() == Exact::on_grid(Slot::at(0)))
    );
}

#[test]
fn past_the_top_magnitude_nothing_lands() {
    // The top magnitude's step is `2^1` and its slots run from -8 to 7, so toward
    // zero 15 and -17 are the last points a range holds, and 16 and -18 are past.
    let top = |num| {
        land::<Under<Floating<4, -6, 8>, TowardZero>>(
            Scaled::of(num, 1, Exponent::ZERO),
            Dither::UNUSED,
        )
    };
    assert!(matches!(top(15), Maybe::Is(l) if l.magnitude() == Magnitude::at(7)));
    assert!(matches!(top(-17), Maybe::Is(l) if l.magnitude() == Magnitude::at(7)));
    assert!(matches!(top(16), Maybe::Isnt));
    assert!(matches!(top(-18), Maybe::Isnt));
}

#[test]
fn a_point_past_the_host_integer_s_positions_still_lands() {
    // `(2^127 - 1) / (2^128 - 1)` is just under a half, so at the step `2^-4` it is
    // just under 8: in range toward zero, and rounded out of it to nearest.
    let point = Scaled::of(i128::MAX, u128::MAX, Exponent::ZERO);
    let toward = land::<Under<Floating<4, -6, 8>, TowardZero>>(point, Dither::UNUSED);
    let nearest = land::<Under<Floating<4, -6, 8>, HalfEven>>(point, Dither::UNUSED);
    assert!(
        matches!(toward, Maybe::Is(l) if l.magnitude() == Magnitude::at(2)
        && adapt::<Under<Floating<4, -6, 8>, TowardZero>>(l.exact(), Dither::UNUSED) == Slot::at(7))
    );
    assert!(
        matches!(nearest, Maybe::Is(l) if l.magnitude() == Magnitude::at(3)
        && adapt::<Under<Floating<4, -6, 8>, HalfEven>>(l.exact(), Dither::UNUSED) == Slot::at(4))
    );
}

#[test]
fn a_point_whose_denominator_passes_128_bits_still_lands() {
    // `7 / (2^128 - 1)` at `2^-20` on the integer grid is `D = (2^128 - 1) 2^20`,
    // and `7` shares no factor with it, so the remainder keeps a denominator past
    // the host's 128-bit integer: slot zero, a little above it.
    let point = Scaled::of(7, u128::MAX, Exponent::of(-20));
    let Maybe::Is(down) = land::<Under<Integer<8>, TowardZero>>(point, Dither::UNUSED) else {
        panic!("the point is in range at the one magnitude");
    };
    let Maybe::Is(up) = land::<Under<Integer<8>, Ceil>>(point, Dither::UNUSED) else {
        panic!("the point is in range at the one magnitude");
    };
    assert_eq!(down.exact().slot(), Slot::ZERO);
    assert!(!down.exact().is_on_grid().get());
    assert_eq!(
        adapt::<Under<Integer<8>, TowardZero>>(down.exact(), Dither::UNUSED),
        Slot::ZERO
    );
    assert_eq!(
        adapt::<Under<Integer<8>, Ceil>>(up.exact(), Dither::UNUSED),
        Slot::at(1)
    );
    // The pair is `7 / D` in lowest terms, and the accessors hand out neither
    // half, since the denominator has no host integer to be read back in.
    let Maybe::Is(scale) = Wide::power(2, 20) else {
        panic!("the power is formed in the four limbs");
    };
    let Maybe::Is(den) = Wide::of_unsigned(u128::MAX).times(scale) else {
        panic!("the denominator is formed in the four limbs");
    };
    let remainder = down.exact().part;
    assert_eq!(remainder, Remainder::reduced(Wide::of(7), den));
    assert!(matches!(remainder.numerator(), Maybe::Isnt));
    assert!(matches!(remainder.denominator(), Maybe::Isnt));
}
