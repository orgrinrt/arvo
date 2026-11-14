//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `land`, against a walk over every magnitude.
//!
//! The oracle is written from the membership predicate: a member at the magnitude
//! whose step is `radix^s` is `phase radix^BASE + slot radix^s`, so the position of
//! `x` there is `(x - phase radix^BASE) / radix^s`, formed in the host's 128-bit
//! integer over the least of the three exponents. It keeps the magnitudes whose
//! rounded position `panic_on_overflow` holds in range and takes the one with the
//! smallest step. Where none holds it, the point is past an end, and the oracle
//! names the top magnitude's position, which `adapt` pins. It skips nothing and
//! orders nothing by the law's slope, so it checks both the walk and the
//! magnitudes `land` does not walk. The member sweeps beside it read no position
//! formula at all.

use core::num::NonZeroU128;

use notko::Maybe;

use super::*;
use crate::adapt::{Adapt, Signature};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Fraction, Remainder, adapt};
use crate::format::Phase;
use crate::overflow::Saturate;
use crate::placing::saturated;
use crate::points::{Biased, Floating, Integer};
use crate::quantum::{Indexed, MagnitudeCount};
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use crate::slots::{Signed, Slot};
use crate::wide::{Divided, Wide};

mod past_the_ends;
mod the_members;
mod the_one_progression;
mod the_walk_start;

/// A law whose step falls with magnitude, so its finest magnitude is its last.
pub(super) struct Falling;

impl Quantum for Falling {
    const BASE: Exponent = Exponent::of(1);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(6);
    const SLOPE: Exponent = Exponent::of(-1);
}

macro_rules! format {
    ($name:ident, $ambient:ty, $quantum:ty, $bits:literal, $phase:expr) => {
        pub(super) struct $name;

        impl Format for $name {
            type Ambient = $ambient;
            type Quantum = $quantum;
            type Slots = Signed<$bits>;

            const PHASE: Phase = $phase;
        }
    };
}

format!(FallingFour, BinaryRationals, Falling, 4, Phase::ZERO);
format!(FallingPhased, BinaryRationals, Falling, 4, Phase::halves(1));
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
    PhasedDecimal,
    DecimalRationals,
    Indexed<-20, 30>,
    64,
    Phase::of(-1, 3)
);
format!(
    ZeroDecimal,
    DecimalRationals,
    Indexed<-100, 200>,
    64,
    Phase::ZERO
);

pub(super) type Under<F, R> = Signature<F, Adapt<R, Saturate>>;

/// A denominator the tests know is not zero.
pub(super) const fn nz(den: u128) -> NonZeroU128 {
    match NonZeroU128::new(den) {
        Some(d) => d,
        None => panic!("a test denominator is never zero"),
    }
}

/// `r^e` in the host integer, for the small laws whose powers fit it.
fn power(r: i128, e: i32) -> i128 {
    let Ok(e) = u32::try_from(e) else {
        panic!("a negative exponent has no integer power");
    };
    let mut p: i128 = 1;
    for _ in 0 .. e {
        let Some(next) = p.checked_mul(r) else {
            panic!("{r}^{e} is past the host integer");
        };
        p = next;
    }
    p
}

/// What the four limbs make of the point at one magnitude.
#[derive(Clone, Copy)]
enum Formed {
    /// The position.
    At(Exact),
    /// Past `2^127` slots on the side it says, negative where it says so: one term
    /// left the four limbs while the other is under `2^255`, over a denominator
    /// under `2^128`.
    Past(bool),
    /// A position the four limbs cannot form and this oracle cannot place.
    Unknown,
}

/// `num * a * r^e` in the four limbs, with `e` at least zero.
fn term(r: u64, num: Wide, a: Wide, e: i32) -> Maybe<Wide> {
    match (num.times(a), Wide::power(r, e as u32)) {
        (Maybe::Is(x), Maybe::Is(y)) => x.times(y),
        _ => Maybe::Isnt,
    }
}

/// Whether a formed value is under `2^255` in magnitude.
fn small(v: Wide) -> bool {
    let Maybe::Is(half) = Wide::power(2, 255) else {
        unreachable!("2^255 is formed in the four limbs")
    };
    let m = if v.is_negative().get() { v.negated() } else { v };
    m.ordering(half).is_lt()
}

/// The point's position at `magnitude`, from the membership predicate.
///
/// `x - phase radix^BASE = slot radix^s`, each term written over the least of the
/// three exponents `e`, so every power is a whole one: the position is
/// `num q r^(k - e) - p den r^(BASE - e)` over `den q r^(s - e)`.
fn placed<F: Format>(num: i128, den: u128, k: i32, magnitude: Magnitude) -> Formed {
    let r = radix::<F>().base() as u64;
    let (mut p, mut q) = (F::PHASE.numerator() as i128, F::PHASE.denominator() as i128);
    if q < 0 {
        (p, q) = (-p, -q);
    }
    let base = <F::Quantum as Quantum>::BASE.power();
    let s = step_exponent::<F>(magnitude).power();
    let e = if p == 0 { k.min(s) } else { k.min(s).min(base) };
    let Maybe::Is(d) = term(r, Wide::of_unsigned(den), Wide::of(q), s - e) else {
        return Formed::Unknown;
    };
    let value = term(r, Wide::of(num), Wide::of(q), k - e);
    let offset = if p == 0 {
        Maybe::Is(Wide::of(0))
    } else {
        term(r, Wide::of(p), Wide::of_unsigned(den), base - e)
    };
    let narrow = matches!(d.unsigned(), Maybe::Is(_));
    let n = match (value, offset) {
        (Maybe::Is(v), Maybe::Is(o)) => {
            match v.plus(o.negated()) {
                Maybe::Is(n) => n,
                Maybe::Isnt if narrow => return Formed::Past(v.is_negative().get()),
                Maybe::Isnt => return Formed::Unknown,
            }
        },
        (Maybe::Isnt, Maybe::Is(o)) if narrow && small(o) && num != 0 => {
            return Formed::Past(num < 0);
        },
        (Maybe::Is(v), Maybe::Isnt) if narrow && small(v) => return Formed::Past(p > 0),
        _ => return Formed::Unknown,
    };
    let Divided {
        quotient,
        remainder,
    } = n.floor_divided(d);
    match quotient.index() {
        Maybe::Is(slot) if remainder.is_zero().get() => Formed::At(Exact::on_grid(Slot::at(slot))),
        Maybe::Is(slot) => Formed::At(Exact::at(Slot::at(slot), Remainder::reduced(remainder, d))),
        Maybe::Isnt => Formed::Past(quotient.is_negative().get()),
    }
}

/// Where the oracle says the point lands.
#[derive(Clone, Copy)]
enum Want {
    /// On the finest magnitude whose rounded position is in range.
    At(Magnitude, Exact),
    /// Past an end: the top magnitude's position, which `adapt` pins.
    Top(Magnitude, Exact),
}

/// Where the oracle lands the point, or `Isnt` where a magnitude it cannot place
/// could change its answer.
fn walked<S: DeclaredSignature>(num: i128, den: u128, k: i32, dither: Dither) -> Maybe<Want> {
    let count = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut best: Maybe<(i32, Magnitude, Exact)> = Maybe::Isnt;
    let mut unknown: Maybe<i32> = Maybe::Isnt;
    let mut top: Maybe<(i32, Magnitude, Formed)> = Maybe::Isnt;
    for m in 0 .. count {
        let magnitude = Magnitude::at(m);
        let step = step_exponent::<S::Format>(magnitude).power();
        let formed = placed::<S::Format>(num, den, k, magnitude);
        if let Formed::At(exact) = formed
            && !panic_on_overflow::<S>(exact, dither).get()
            && best.is_none_or(|(s, ..)| step < s)
        {
            best = Maybe::Is((step, magnitude, exact));
        }
        if matches!(formed, Formed::Unknown) {
            unknown = Maybe::Is(unknown.map_or(step, |u| u.min(step)));
        }
        if top.is_none_or(|(s, ..)| step > s) {
            top = Maybe::Is((step, magnitude, formed));
        }
    }
    match (best, unknown) {
        // A magnitude the oracle cannot place matters only where it is finer.
        (Maybe::Is((s, ..)), Maybe::Is(u)) if u < s => Maybe::Isnt,
        (Maybe::Is((_, m, e)), _) => Maybe::Is(Want::At(m, e)),
        (Maybe::Isnt, Maybe::Is(_)) => Maybe::Isnt,
        (Maybe::Isnt, Maybe::Isnt) => {
            match top {
                Maybe::Is((_, m, Formed::At(e))) => Maybe::Is(Want::Top(m, e)),
                Maybe::Is((_, m, Formed::Past(negative))) => {
                    Maybe::Is(Want::Top(m, saturated(negative)))
                },
                _ => Maybe::Isnt,
            }
        },
    }
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

/// Every point over numerators past 100 bits, where the decimal bound once
/// failed, the small ones beside them and denominators up to 62 bits, at each
/// shift `k` from `low` to `high` by `by`.
fn wide_points(low: i32, high: i32, by: usize, mut visit: impl FnMut(i128, u128, i32)) {
    let numerators: [i128; 10] = [
        1,
        -1,
        3,
        1 << 40,
        -(1 << 62) - 5,
        i64::MAX as i128,
        (1 << 100) + 7,
        (1 << 110) + 3,
        -(1 << 120) - 9,
        1 << 126,
    ];
    for num in numerators {
        for den in [1, 3, 1 << 20, (1 << 61) + 1] {
            for k in (low ..= high).step_by(by) {
                visit(num, den, k);
            }
        }
    }
}

/// Points whose landing sits a few magnitudes from where the walk starts on a
/// format whose slots span 64 bits, over shifts the small sweep never reaches.
struct Far;

impl Points for Far {
    fn each(visit: impl FnMut(i128, u128, i32)) {
        wide_points(-250, 250, 7, visit);
    }
}

/// The same numerators over the shifts a phased decimal law of 50 magnitudes
/// forms in the four limbs: its denominator carries `10^(s - BASE)`, so a span
/// of 300 magnitudes holds no position past the 77th.
struct Near;

impl Points for Near {
    fn each(visit: impl FnMut(i128, u128, i32)) {
        wide_points(-60, 60, 3, visit);
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
        let got = land::<S>(Scaled::of(num, nz(den), Exponent::of(k)), dither);
        let agrees = match (got, want) {
            (Maybe::Is(l), Want::At(m, e)) => {
                l.magnitude() == m
                    && l.exact() == e
                    && adapt::<S>(l.exact(), dither) == adapt::<S>(e, dither)
            },
            // Past the index the oracle and `land` may name different far
            // positions; the end `adapt` pins them to is what must agree.
            (Maybe::Is(l), Want::Top(m, e)) => {
                l.magnitude() == m && adapt::<S>(l.exact(), dither) == adapt::<S>(e, dither)
            },
            (Maybe::Isnt, _) => false,
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
        land::<Under<FallingFour, HalfEven>>(Scaled::of(1, nz(3), Exponent::ZERO), Dither::UNUSED);
    assert!(matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::at(5)));
}

#[test]
fn a_phase_moves_the_position_at_every_magnitude() {
    assert_eq!(every_mode!(PhasedFour, Small), 0);
    assert_eq!(every_mode!(FallingPhased, Small), 0);
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
    assert_eq!(every_mode!(PhasedDecimal, Near), 0);
    assert_eq!(every_mode!(ZeroDecimal, Far), 0);
}

#[test]
fn the_phase_is_read_in_units_of_the_magnitude_zero_quantum() {
    // The phase is a half of `2^-4`, so `21/32 = 1/32 + 5 (1/8)` is slot 5 on the
    // step `2^-3`, magnitude one, and at magnitude zero it is slot 10, past the
    // range. A phase read at the current step puts it at slot 4, off the grid.
    let got = land::<Under<PhasedFour, TowardZero>>(
        Scaled::of(21, nz(32), Exponent::ZERO),
        Dither::UNUSED,
    );
    assert!(
        matches!(got, Maybe::Is(l) if l.magnitude() == Magnitude::at(1)
        && l.exact() == Exact::on_grid(Slot::at(5)))
    );
}

#[test]
fn a_point_past_the_host_integer_s_positions_still_lands() {
    // `(2^127 - 1) / (2^128 - 1)` is just under a half, so at the step `2^-4` it is
    // just under 8: in range toward zero, and rounded out of it to nearest.
    let point = Scaled::of(i128::MAX, nz(u128::MAX), Exponent::ZERO);
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
    let point = Scaled::of(7, nz(u128::MAX), Exponent::of(-20));
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
