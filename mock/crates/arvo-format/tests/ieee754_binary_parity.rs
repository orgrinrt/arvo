//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against IEEE 754's binary interchange formats, under the four
//! rounding-direction attributes the binary formats carry.
//!
//! The expected side never reads arvo code. Binary32 and binary64 are the host's:
//! Rust defines a conversion to either as round to nearest with ties to even, so
//! `f64 as f32` of an exact `f64` and `i64 as f64` are each one rounding of the
//! exact value, and the directed modes step from that answer to the neighbour on
//! the other side of the exact value, by its bits. Binary16 has no host type, so
//! its arms read the standard's published encodings, decoded from their bits,
//! and the standard's digit rule in integers at radix two, `ieee754::rule`.
//!
//! The arvo side is `standards::ieee754` landed by the composition in `ieee754`,
//! the finest magnitude whose rounded slot is in range. A value is compared as a
//! number, so the standard's negative zero meets this crate's one zero, which is
//! the signed-zero refusal stated where it is used.

mod ieee754;

use core::cmp::Ordering;

use arvo_format::format::{Format, step_exponent};
use arvo_format::overflow::Saturate;
use arvo_format::points::Floating;
use arvo_format::quantum::Quantum;
use arvo_format::rounding::{HalfUp, Mode, Rounding};
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Binary16, Binary32, Binary64};
use ieee754::{Direction, Draws, Standard, Value, agrees, land, misses, power, same};

/// Binary16 as the standard's table gives it.
const BINARY16: Standard = Standard {
    radix: 2,
    p:     11,
    qmin:  -24,
    qmax:  5,
};

// --- the host's answers, decoded from their bits ----------------------------------

/// A binary32 encoding as a value, or nothing for an infinity.
fn decode32(bits: u32) -> Option<Value> {
    let sign = if bits >> 31 == 1 { -1 } else { 1 };
    let field = ((bits >> 23) & 0xFF) as i32;
    let frac = (bits & 0x7F_FFFF) as i128;
    match field {
        0xFF => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -149,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 23),
                exponent:    field - 150,
            })
        },
    }
}

/// A binary64 encoding as a value, or nothing for an infinity.
fn decode64(bits: u64) -> Option<Value> {
    let sign = if bits >> 63 == 1 { -1 } else { 1 };
    let field = ((bits >> 52) & 0x7FF) as i32;
    let frac = (bits & 0xF_FFFF_FFFF_FFFF) as i128;
    match field {
        0x7FF => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -1074,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 52),
                exponent:    field - 1075,
            })
        },
    }
}

/// A binary16 encoding as a value, or nothing for an infinity.
fn decode16(bits: u16) -> Option<Value> {
    let sign = if bits >> 15 == 1 { -1 } else { 1 };
    let field = ((bits >> 10) & 0x1F) as i32;
    let frac = (bits & 0x3FF) as i128;
    match field {
        0x1F => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -24,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 10),
                exponent:    field - 25,
            })
        },
    }
}

/// `2^k` as an `f64`, exactly, for `k` in `[-1074, 1023]`.
fn pow2(k: i32) -> f64 {
    if k >= -1022 {
        f64::from_bits(((k + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (k + 1074))
    }
}

/// The neighbour of a binary32 one step toward positive or negative infinity.
fn step32(r: f32, up: bool) -> f32 {
    if r == 0.0 {
        let tiny = f32::from_bits(1);
        return if up { tiny } else { -tiny };
    }
    let away = (r > 0.0) == up;
    let bits = r.to_bits();
    f32::from_bits(if away { bits + 1 } else { bits - 1 })
}

/// The host's binary32 answer for `n * 2^k` under a mode, where `n * 2^k` is an
/// exact `f64`.
fn host32(mode: Mode, n: i64, k: i32) -> Option<Value> {
    let x = (n as f64) * pow2(k);
    let r = x as f32;
    let r = if r.is_infinite() {
        match mode {
            Mode::HalfEven => r,
            Mode::Ceil if r < 0.0 => -f32::MAX,
            Mode::Floor if r > 0.0 => f32::MAX,
            Mode::TowardZero => r.signum() * f32::MAX,
            _ => r,
        }
    } else {
        match mode {
            Mode::Floor if (r as f64) > x => step32(r, false),
            Mode::Ceil if (r as f64) < x => step32(r, true),
            Mode::TowardZero if (r as f64).abs() > x.abs() => step32(r, r < 0.0),
            _ => r,
        }
    };
    decode32(r.to_bits())
}

/// Whether `v` is below, at or above the exact `n * 2^k`.
fn against_exact(v: Value, n: i64, k: i32) -> Ordering {
    // Both sides are within a few dozen bits of each other in magnitude, so the
    // finer exponent's scale keeps each in the index.
    let e = v.exponent.min(k);
    let scale = |s: i128, at: i32| {
        power(2, (at - e) as u32)
            .and_then(|p| s.checked_mul(p))
            .expect("the two sides of a comparison are near each other in magnitude")
    };
    scale(v.significand, v.exponent).cmp(&scale(n as i128, k))
}

/// The neighbour of a binary64 one step toward positive or negative infinity.
fn step64(r: f64, up: bool) -> f64 {
    if r == 0.0 {
        let tiny = f64::from_bits(1);
        return if up { tiny } else { -tiny };
    }
    let away = (r > 0.0) == up;
    let bits = r.to_bits();
    f64::from_bits(if away { bits + 1 } else { bits - 1 })
}

/// The host's binary64 answer for `n * 2^k` under a mode, where the host's own
/// product is one rounding of it.
fn host64(mode: Mode, n: i64, k: i32) -> Option<Value> {
    let r = (n as f64) * pow2(k);
    let r = if r.is_infinite() {
        match mode {
            Mode::Ceil if r < 0.0 => -f64::MAX,
            Mode::Floor if r > 0.0 => f64::MAX,
            Mode::TowardZero => r.signum() * f64::MAX,
            _ => r,
        }
    } else {
        let side = against_exact(decode64(r.to_bits()).unwrap(), n, k);
        match mode {
            Mode::Floor if side.is_gt() => step64(r, false),
            Mode::Ceil if side.is_lt() => step64(r, true),
            Mode::TowardZero if (n > 0 && side.is_gt()) || (n < 0 && side.is_lt()) => {
                step64(r, r < 0.0)
            },
            _ => r,
        }
    };
    decode64(r.to_bits())
}

// --- binary32 ------------------------------------------------------------------------

/// The binary32 inputs: structured ties and carries, then a sweep across the
/// subnormals, the normals and past the largest finite member.
fn binary32_inputs() -> Vec<(i64, i32)> {
    let mut v = vec![
        ((1 << 24) + 1, 0),
        ((1 << 24) + 3, 0),
        ((1 << 25) - 1, 0),
        (-((1 << 24) + 1), 0),
        (-((1 << 24) + 3), 0),
        (3, -150),
        (5, -150),
        (1, -150),
        (-3, -150),
        ((1 << 25) - 1, -150),
        ((1 << 25) - 1, 103),
        ((1 << 24) - 1, 104),
        (-((1 << 25) - 1), 103),
    ];
    let mut d = Draws::seeded(0x1EEE_0754);
    for _ in 0 .. 4000 {
        let n = d.within(-(1 << 52), 1 << 52);
        let k = d.within(-230, 80) as i32;
        v.push((n, k));
        // A tie: a 24-bit significand and half a step.
        let s = d.within(1 << 23, (1 << 24) - 1);
        v.push((s * 2 + 1, d.within(-200, 102) as i32));
    }
    v
}

fn binary32_misses<F: Format, R: Rounding>(mode: Mode) -> usize
where
    Attributed<F, R, Saturate>: arvo_format::adapt::DeclaredSignature,
{
    binary32_inputs()
        .into_iter()
        .filter(|&(n, k)| {
            !agrees(
                2,
                land::<Attributed<F, R, Saturate>>(n as i128, 1, k),
                host32(mode, n, k),
            )
        })
        .count()
}

#[test]
fn binary32_under_every_direction_is_the_host() {
    assert_eq!(
        binary32_misses::<Binary32, RoundTiesToEven>(Mode::HalfEven),
        0
    );
    assert_eq!(
        binary32_misses::<Binary32, RoundTowardPositive>(Mode::Ceil),
        0
    );
    assert_eq!(
        binary32_misses::<Binary32, RoundTowardNegative>(Mode::Floor),
        0
    );
    assert_eq!(
        binary32_misses::<Binary32, RoundTowardZero>(Mode::TowardZero),
        0
    );
}

#[test]
fn a_planted_binary32_direction_is_reported() {
    assert!(binary32_misses::<Binary32, HalfUp>(Mode::HalfEven) > 0);
    assert!(binary32_misses::<Binary32, RoundTowardZero>(Mode::Floor) > 0);
    assert!(binary32_misses::<Binary32, RoundTowardNegative>(Mode::Ceil) > 0);
    assert!(binary32_misses::<Binary32, RoundTowardPositive>(Mode::TowardZero) > 0);
}

#[test]
fn a_planted_binary32_format_is_reported() {
    // A precision one short, and a largest exponent one short, under the right
    // direction: the host still answers binary32, so each must miss.
    use arvo_format::standards::ieee754::Binary;
    assert!(binary32_misses::<Binary<23, 127>, RoundTiesToEven>(Mode::HalfEven) > 0);
    assert!(binary32_misses::<Binary<24, 126>, RoundTiesToEven>(Mode::HalfEven) > 0);
}

#[test]
fn the_shipped_floating_point_is_binary32_but_one_member() {
    // The same sweep over `Floating<25, -149, 254>` misses exactly where its one
    // extra member, the two's complement end `-2^24` at the top magnitude, names
    // `-2^128`: binary32 rounds that input to an infinity and the shipped point
    // holds it.
    type Shipped = Floating<25, -149, 254>;
    assert_eq!(
        binary32_misses::<Shipped, RoundTiesToEven>(Mode::HalfEven),
        1
    );
    assert!(host32(Mode::HalfEven, -((1 << 25) - 1), 103).is_none());
    let landed =
        land::<Attributed<Shipped, RoundTiesToEven, Saturate>>(-((1 << 25) - 1), 1, 103).unwrap();
    assert!(landed.finite);
    assert!(same(2, landed.value, Value {
        significand: -1,
        exponent:    128,
    }));
}

// --- binary64 ------------------------------------------------------------------------

/// The binary64 inputs. Wide integers convert in one rounding where the result is
/// normal, and integers below `2^53` are exact and round once in the scaling
/// where it is subnormal or past the largest finite member.
fn binary64_inputs() -> Vec<(i64, i32)> {
    let mut v = Vec::new();
    let mut d = Draws::seeded(0x6464_0754);
    for _ in 0 .. 3000 {
        v.push((d.next() as i64, d.within(-1000, 900) as i32));
        let n = d.within(-(1 << 53) + 1, (1 << 53) - 1);
        v.push((n, d.within(-1074, -1020) as i32));
        v.push((n, d.within(960, 1023) as i32));
    }
    // A tie at 2^53 + 1, and its odd neighbour.
    v.push(((1 << 53) + 1, 0));
    v.push(((1 << 53) + 3, 0));
    v
}

fn binary64_misses<R: Rounding>(mode: Mode) -> usize {
    binary64_inputs()
        .into_iter()
        .filter(|&(n, k)| {
            !agrees(
                2,
                land::<Attributed<Binary64, R, Saturate>>(n as i128, 1, k),
                host64(mode, n, k),
            )
        })
        .count()
}

#[test]
fn binary64_under_every_direction_is_the_host() {
    assert_eq!(binary64_misses::<RoundTiesToEven>(Mode::HalfEven), 0);
    assert_eq!(binary64_misses::<RoundTowardPositive>(Mode::Ceil), 0);
    assert_eq!(binary64_misses::<RoundTowardNegative>(Mode::Floor), 0);
    assert_eq!(binary64_misses::<RoundTowardZero>(Mode::TowardZero), 0);
}

#[test]
fn a_planted_binary64_direction_is_reported() {
    assert!(binary64_misses::<HalfUp>(Mode::HalfEven) > 0);
    assert!(binary64_misses::<RoundTowardZero>(Mode::Floor) > 0);
    assert!(binary64_misses::<RoundTowardNegative>(Mode::Ceil) > 0);
    assert!(binary64_misses::<RoundTowardPositive>(Mode::TowardZero) > 0);
}

// --- binary16 ------------------------------------------------------------------------

/// Binary16 encodings, as `num / den * 2^k` and the encoding it rounds to under
/// ties to even. The first seven are the format's documented landmarks; the
/// three after them are ties and an overflow, whose encodings follow from the
/// rule at that position.
const ENCODINGS: &[(i128, i128, i32, u16)] = &[
    (1, 3, 0, 0x3555),
    (65504, 1, 0, 0x7BFF),
    (1, 1, -24, 0x0001),
    (1, 1, -14, 0x0400),
    (1023, 1, -24, 0x03FF),
    (1025, 1, -10, 0x3C01),
    (-2, 1, 0, 0xC000),
    (2049, 1, -11, 0x3C00),
    (2051, 1, -11, 0x3C02),
    (65520, 1, 0, 0x7C00),
];

fn encoding_misses<R: Rounding>() -> usize {
    ENCODINGS
        .iter()
        .filter(|&&(num, den, k, bits)| {
            !agrees(
                2,
                land::<Attributed<Binary16, R, Saturate>>(num, den, k),
                decode16(bits),
            )
        })
        .count()
}

fn binary16_misses<R: Rounding>(dir: Direction) -> usize
where
    Attributed<Binary16, R, Saturate>: arvo_format::adapt::DeclaredSignature,
{
    misses::<Attributed<Binary16, R, Saturate>>(dir, BINARY16, 0x1616_0754)
}

#[test]
fn binary16_is_its_encodings() {
    assert_eq!(encoding_misses::<RoundTiesToEven>(), 0);
}

#[test]
fn binary16_under_every_direction_is_the_digit_rule() {
    assert_eq!(binary16_misses::<RoundTiesToEven>(Direction::TiesToEven), 0);
    assert_eq!(
        binary16_misses::<RoundTowardPositive>(Direction::TowardPositive),
        0
    );
    assert_eq!(
        binary16_misses::<RoundTowardNegative>(Direction::TowardNegative),
        0
    );
    assert_eq!(binary16_misses::<RoundTowardZero>(Direction::TowardZero), 0);
}

#[test]
fn the_binary16_exponent_range_is_the_standards() {
    type Q = <Binary16 as Format>::Quantum;
    assert_eq!(<Q as Quantum>::BASE.power(), BINARY16.qmin);
    assert_eq!(
        step_exponent::<Binary16>(<Q as Quantum>::MAGNITUDES.largest()).power(),
        BINARY16.qmax
    );
}

#[test]
fn a_planted_binary16_direction_is_reported() {
    assert!(encoding_misses::<HalfUp>() > 0);
    assert!(binary16_misses::<HalfUp>(Direction::TiesToEven) > 0);
    assert!(binary16_misses::<RoundTowardZero>(Direction::TowardNegative) > 0);
    assert!(binary16_misses::<RoundTowardNegative>(Direction::TowardPositive) > 0);
    assert!(binary16_misses::<RoundTowardPositive>(Direction::TowardZero) > 0);
    // The digit rule tells ties to away from ties to even, so the right landing
    // checked against the other tie rule must miss.
    assert!(binary16_misses::<RoundTiesToEven>(Direction::TiesToAway) > 0);
}

// --- the refusals ----------------------------------------------------------------------

#[test]
fn past_the_largest_finite_member_there_is_no_infinity() {
    // 65520 is where binary16 rounds to an infinity. No magnitude holds it, and
    // the completion pins it to the largest finite member, which is the
    // standard's answer toward zero and not its answer to nearest.
    let landed = land::<Attributed<Binary16, RoundTiesToEven, Saturate>>(65520, 1, 0).unwrap();
    assert!(!landed.finite);
    assert!(same(2, landed.value, decode16(0x7BFF).unwrap()));
}

#[test]
fn the_standards_negative_zero_lands_on_the_one_zero() {
    // A value below the smallest subnormal's half, from below: the host answers
    // negative zero, an encoding with its sign set, and the landing answers the
    // one zero this crate has, compared as a value.
    let host = ((-1i64) as f64) * pow2(-160);
    assert_eq!((host as f32).to_bits(), 0x8000_0000);
    let landed = land::<Attributed<Binary32, RoundTiesToEven, Saturate>>(-1, 1, -160).unwrap();
    assert!(landed.finite);
    assert_eq!(landed.value.significand, 0);
}

#[test]
fn a_significand_that_is_not_sign_and_magnitude_is_reported_by_its_verdict() {
    // The verdict beside the obligation answers without forcing it, so the wrong
    // constructions are reported here rather than refusing the build; the
    // compile-fail cases show the same constructions refused where used.
    use arvo_format::ambient::{BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
    use arvo_format::slots::is_admissible;
    use arvo_format::standards::ieee754::{Significand, is_sign_and_magnitude};
    assert!(is_sign_and_magnitude::<BinaryRationals, 24>().get());
    assert!(is_sign_and_magnitude::<DecimalRationals, 7>().get());
    assert!(!is_sign_and_magnitude::<UnsignedBinaryRationals, 24>().get());
    assert!(!is_sign_and_magnitude::<BinaryRationals, 0>().get());
    // The slot range's verdict reads the bounds without forcing anything, so it
    // reports a significand past the ladder. It does not see the sign and
    // magnitude override, ticket arvo_the_slots_verdict_cannot_see_an_overridden_admitted.
    assert!(!is_admissible::<Significand<BinaryRationals, 113>>().get());
}

#[test]
fn nothing_puts_a_value_on_an_interchange_format_yet() {
    // Conversion and the operations refuse a floating format, ticket
    // arvo_nothing_puts_a_value_onto_an_ieee_754_interchange_forma. This fails the
    // day one of them admits an interchange format, and the hole closes with it.
    // The fixed-point conversion is the control that the verdict can say yes.
    use arvo_format::addition::is_addable;
    use arvo_format::conversion::is_convertible;
    use arvo_format::division::is_divisible;
    use arvo_format::multiplication::is_multipliable;
    use arvo_format::standards::ieee754::Decimal32;
    use arvo_format::standards::matlab::Fi;
    assert!(is_convertible::<Fi<32, 0>, Fi<16, 0>>().get());
    assert!(!is_convertible::<Fi<32, 0>, Binary32>().get());
    assert!(!is_convertible::<Binary32, Fi<32, 0>>().get());
    assert!(!is_convertible::<Binary16, Binary32>().get());
    assert!(!is_convertible::<Decimal32, Decimal32>().get());
    assert!(!is_addable::<Binary32>().get());
    assert!(!is_addable::<Decimal32>().get());
    assert!(!is_multipliable::<Binary32>().get());
    assert!(!is_multipliable::<Decimal32>().get());
    assert!(!is_divisible::<Binary32>().get());
    assert!(!is_divisible::<Decimal32>().get());
}
