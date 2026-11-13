//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against IEEE 754's decimal interchange formats.
//!
//! There is no host decimal, so the expected side is the standard's own digit
//! rule written in integers, reading no arvo code: a value `c * 10^q` keeps at most
//! `p` digits and no exponent below the smallest, so the digits past either are
//! dropped, the dropped part decides the last kept digit by the direction's rule,
//! and a carry to `10^p` drops one more. Past the largest exponent, a significand
//! that still has room is scaled up into it, and one that does not is past the
//! largest finite member.
//!
//! The arvo side is `Decimal32` and `Decimal64` landed by the composition in
//! `ieee754`, the same composition the binary arms use at the other radix.

mod ieee754;

use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::quantum::{Magnitude, Quantum};
use arvo_format::rounding::{HalfUp, Mode, Rounding};
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Decimal32, Decimal64};
use ieee754::{Draws, Value, land, power, same};

/// The number of decimal digits of `|c|`, zero for zero.
fn digits(c: i128) -> u32 {
    let mut n = c.unsigned_abs();
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

/// `c` divided by `10^drop` under a direction, the standard's way: the kept digits
/// and the dropped part against half of what was dropped.
fn round_off(mode: Mode, c: i128, drop: u32) -> i128 {
    let Some(unit) = power(10, drop) else {
        // Everything is dropped and what is dropped is less than half a unit,
        // since `|c|` is below `10^38`.
        return match mode {
            Mode::Ceil if c > 0 => 1,
            Mode::Floor if c < 0 => -1,
            _ => 0,
        };
    };
    let kept = c / unit;
    let rest = c - kept * unit;
    if rest == 0 {
        return kept;
    }
    let toward_zero = kept;
    let away = kept + c.signum();
    match mode {
        Mode::TowardZero => toward_zero,
        Mode::Ceil => {
            if c > 0 {
                away
            } else {
                toward_zero
            }
        },
        Mode::Floor => {
            if c < 0 {
                away
            } else {
                toward_zero
            }
        },
        Mode::HalfEven => {
            match (rest.abs() * 2).cmp(&unit) {
                core::cmp::Ordering::Less => toward_zero,
                core::cmp::Ordering::Greater => away,
                core::cmp::Ordering::Equal => {
                    if kept % 2 == 0 {
                        toward_zero
                    } else {
                        away
                    }
                },
            }
        },
        _ => unreachable!("the decimal arms run the four directions the rule is written for"),
    }
}

/// The standard's answer for `c * 10^q` at precision `p` and exponent range
/// `[qmin, qmax]` of the significand's last digit, or nothing past the largest
/// finite member.
fn rule(mode: Mode, p: u32, qmin: i32, qmax: i32, c: i128, q: i32) -> Option<Value> {
    let too_many = digits(c).saturating_sub(p) as i32;
    let drop = too_many.max(qmin - q).max(0) as u32;
    let mut kept = round_off(mode, c, drop);
    let mut at = q + drop as i32;
    if digits(kept) > p {
        kept /= 10;
        at += 1;
    }
    if at > qmax {
        let up = (at - qmax) as u32;
        match power(10, up).and_then(|s| kept.checked_mul(s)) {
            Some(scaled) if digits(scaled) <= p => {
                kept = scaled;
                at = qmax;
            },
            _ => {
                // Past the largest finite member: an infinity, except toward
                // zero and on a directed mode's own side, where the standard
                // answers the largest finite member.
                let largest = power(10, p).unwrap() - 1;
                return match mode {
                    Mode::TowardZero => {
                        Some(Value {
                            significand: c.signum() * largest,
                            exponent:    qmax,
                        })
                    },
                    Mode::Ceil if c < 0 => {
                        Some(Value {
                            significand: -largest,
                            exponent:    qmax,
                        })
                    },
                    Mode::Floor if c > 0 => {
                        Some(Value {
                            significand: largest,
                            exponent:    qmax,
                        })
                    },
                    _ => None,
                };
            },
        }
    }
    Some(Value {
        significand: kept,
        exponent:    at,
    })
}

/// The exponent range of a format's last significand digit, read off its
/// quantum law.
fn range<F: Format>() -> (i32, i32) {
    let top = <F::Quantum as Quantum>::MAGNITUDES.largest();
    (
        <F::Quantum as Quantum>::BASE.power(),
        arvo_format::format::step_exponent::<F>(top).power(),
    )
}

/// Inputs as `c * 10^q`: structured ties and carries, then a sweep from below
/// the smallest subnormal to past the largest finite member.
fn inputs(p: u32, qmin: i32, qmax: i32, seed: u64) -> Vec<(i128, i32)> {
    let ten_p = power(10, p).unwrap();
    let mut v = vec![
        (ten_p * 10 + 5, 0),
        (ten_p * 10 + 15, 0),
        (ten_p * 100 - 5, 0),
        (-(ten_p * 10 + 5), 0),
        (-(ten_p * 10 + 15), 0),
        (5, qmin - 1),
        (15, qmin - 1),
        (-25, qmin - 1),
        (ten_p * 10 - 1, qmax),
        (ten_p - 1, qmax),
        (ten_p * 10 - 5, qmax - 1),
    ];
    let mut d = Draws::seeded(seed);
    for _ in 0 .. 3000 {
        let c = d.within(-999_999_999_999_999_999, 999_999_999_999_999_999) as i128;
        let q = d.within((qmin - 20) as i64, (qmax + 2) as i64) as i32;
        v.push((c, q));
        // A tie: a full significand, one more digit, and a five.
        let s = d.within(power(10, p - 1).unwrap() as i64, (ten_p - 1) as i64) as i128;
        v.push((
            s * 10 + 5,
            d.within((qmin - 1) as i64, (qmax - 1) as i64) as i32,
        ));
    }
    v
}

fn misses<F: Format, R: Rounding>(mode: Mode, p: u32, seed: u64) -> usize
where
    Attributed<F, R, Saturate>: arvo_format::adapt::DeclaredSignature,
{
    let (qmin, qmax) = range::<F>();
    inputs(p, qmin, qmax, seed)
        .into_iter()
        .filter(|&(c, q)| {
            let want = rule(mode, p, qmin, qmax, c, q);
            let agrees = match (land::<Attributed<F, R, Saturate>>(c, 1, q), want) {
                (Some(l), Some(w)) => same(10, l.value, w),
                (Some(l), None) => !l.finite,
                (None, _) => false,
            };
            !agrees
        })
        .count()
}

#[test]
fn the_exponent_range_is_the_standards() {
    // decimal32's last digit runs from 10^-101 to 10^90 and decimal64's from
    // 10^-398 to 10^369, which is `emin - (p - 1)` to `emax - (p - 1)`.
    assert_eq!(range::<Decimal32>(), (-101, 90));
    assert_eq!(range::<Decimal64>(), (-398, 369));
    assert_eq!(
        arvo_format::format::step_exponent::<Decimal32>(Magnitude::SMALLEST).power(),
        -101
    );
}

#[test]
fn decimal32_under_every_direction_is_the_digit_rule() {
    assert_eq!(
        misses::<Decimal32, RoundTiesToEven>(Mode::HalfEven, 7, 0xD32),
        0
    );
    assert_eq!(
        misses::<Decimal32, RoundTowardPositive>(Mode::Ceil, 7, 0xD32),
        0
    );
    assert_eq!(
        misses::<Decimal32, RoundTowardNegative>(Mode::Floor, 7, 0xD32),
        0
    );
    assert_eq!(
        misses::<Decimal32, RoundTowardZero>(Mode::TowardZero, 7, 0xD32),
        0
    );
}

#[test]
fn decimal64_under_every_direction_is_the_digit_rule() {
    assert_eq!(
        misses::<Decimal64, RoundTiesToEven>(Mode::HalfEven, 16, 0xD64),
        0
    );
    assert_eq!(
        misses::<Decimal64, RoundTowardPositive>(Mode::Ceil, 16, 0xD64),
        0
    );
    assert_eq!(
        misses::<Decimal64, RoundTowardNegative>(Mode::Floor, 16, 0xD64),
        0
    );
    assert_eq!(
        misses::<Decimal64, RoundTowardZero>(Mode::TowardZero, 16, 0xD64),
        0
    );
}

#[test]
fn a_planted_decimal_direction_is_reported() {
    assert!(misses::<Decimal32, HalfUp>(Mode::HalfEven, 7, 0xD32) > 0);
    assert!(misses::<Decimal32, RoundTowardZero>(Mode::Floor, 7, 0xD32) > 0);
    assert!(misses::<Decimal64, HalfUp>(Mode::HalfEven, 16, 0xD64) > 0);
    assert!(misses::<Decimal64, RoundTowardPositive>(Mode::TowardZero, 16, 0xD64) > 0);
}
