//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against IEEE 754's binary interchange formats.
//!
//! The expected side never reads arvo code. Binary32 and binary64 are the host's:
//! Rust defines a conversion to either as round to nearest with ties to even, so
//! `f64 as f32` of an exact `f64` and `i64 as f64` are each one rounding of the
//! exact value, and the directed modes step from that answer to the neighbour on
//! the other side of the exact value, by its bits. Binary16 has no host type, so
//! its arms read encodings and decode them from their bits.
//!
//! The arvo side is `standards::ieee754` landed by the composition in `ieee754`,
//! the finest magnitude whose rounded slot is in range. A value is compared as a
//! number, so the standard's negative zero meets this crate's one zero, which is
//! the signed-zero refusal stated where it is used.

mod ieee754;

use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::points::Floating;
use arvo_format::quantum::Quantum;
use arvo_format::rounding::{HalfUp, Mode, Rounding};
use arvo_format::slots::Slots;
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Binary16, Binary32, Binary64};
use ieee754::{Draws, Value, land, same};

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

// --- the arm -------------------------------------------------------------------------

/// Whether the landed value is the expected one: the same number where the
/// standard's answer is finite, and no magnitude holding it where it is not.
///
/// Where the standard's answer is finite past the range, the largest member on
/// the side the direction rounds away from, the completion's saturation is what
/// meets it, so a value no magnitude held still agrees when it is that member.
fn agrees(landed: Option<ieee754::Landed>, want: Option<Value>) -> bool {
    match (landed, want) {
        (Some(l), Some(w)) => same(2, l.value, w),
        (Some(l), None) => !l.finite,
        (None, _) => false,
    }
}

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

fn binary32_misses<R: Rounding>(mode: Mode) -> usize {
    binary32_inputs()
        .into_iter()
        .filter(|&(n, k)| {
            !agrees(
                land::<Attributed<Binary32, R, Saturate>>(n as i128, 1, k),
                host32(mode, n, k),
            )
        })
        .count()
}

#[test]
fn binary32_under_every_direction_is_the_host() {
    assert_eq!(binary32_misses::<RoundTiesToEven>(Mode::HalfEven), 0);
    assert_eq!(binary32_misses::<RoundTowardPositive>(Mode::Ceil), 0);
    assert_eq!(binary32_misses::<RoundTowardNegative>(Mode::Floor), 0);
    assert_eq!(binary32_misses::<RoundTowardZero>(Mode::TowardZero), 0);
}

#[test]
fn a_planted_binary32_direction_is_reported() {
    assert!(binary32_misses::<HalfUp>(Mode::HalfEven) > 0);
    assert!(binary32_misses::<RoundTowardZero>(Mode::Floor) > 0);
    assert!(binary32_misses::<RoundTowardNegative>(Mode::Ceil) > 0);
    assert!(binary32_misses::<RoundTowardPositive>(Mode::TowardZero) > 0);
}

/// The binary64 inputs. Wide integers convert in one rounding where the result is
/// normal, and integers below `2^53` are exact and round once in the scaling
/// where it is subnormal or past the largest finite member.
fn binary64_inputs() -> Vec<(i64, i32, f64)> {
    let mut v = Vec::new();
    let mut d = Draws::seeded(0x6464_0754);
    for _ in 0 .. 3000 {
        let n = d.next() as i64;
        let k = d.within(-1000, 900) as i32;
        v.push((n, k, (n as f64) * pow2(k)));
        let n = d.within(-(1 << 53) + 1, (1 << 53) - 1);
        let k = d.within(-1074, -1020) as i32;
        v.push((n, k, (n as f64) * pow2(k)));
        let k = d.within(960, 1023) as i32;
        v.push((n, k, (n as f64) * pow2(k)));
    }
    // A tie at 2^53 + 1, and its odd neighbour.
    v.push(((1 << 53) + 1, 0, ((1i64 << 53) + 1) as f64));
    v.push(((1 << 53) + 3, 0, ((1i64 << 53) + 3) as f64));
    v
}

fn binary64_misses<R: Rounding>() -> usize {
    binary64_inputs()
        .into_iter()
        .filter(|&(n, k, host)| {
            !agrees(
                land::<Attributed<Binary64, R, Saturate>>(n as i128, 1, k),
                decode64(host.to_bits()),
            )
        })
        .count()
}

#[test]
fn binary64_ties_to_even_is_the_host() {
    assert_eq!(binary64_misses::<RoundTiesToEven>(), 0);
}

#[test]
fn a_planted_binary64_mode_is_reported() {
    assert!(binary64_misses::<HalfUp>() > 0);
}

/// Binary16 encodings, as `num / den * 2^k` and the encoding it rounds to under
/// ties to even. The first seven are the format's documented landmarks; the
/// three after them are ties and an overflow, whose encodings follow from the
/// rule at that position.
const BINARY16: &[(i128, i128, i32, u16)] = &[
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

fn binary16_misses<R: Rounding>() -> usize {
    BINARY16
        .iter()
        .filter(|&&(num, den, k, bits)| {
            !agrees(
                land::<Attributed<Binary16, R, Saturate>>(num, den, k),
                decode16(bits),
            )
        })
        .count()
}

#[test]
fn binary16_is_its_encodings() {
    assert_eq!(binary16_misses::<RoundTiesToEven>(), 0);
}

#[test]
fn a_planted_binary16_mode_is_reported() {
    assert!(binary16_misses::<HalfUp>() > 0);
}

// --- the refusals, and the agreement with the shipped point ---------------------------

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
fn binary32_holds_the_values_of_the_shipped_floating_point_but_one() {
    type Shipped = Floating<25, -149, 254>;
    type Q = <Binary32 as Format>::Quantum;
    type S = <Binary32 as Format>::Slots;
    assert_eq!(
        <Q as Quantum>::BASE,
        <<Shipped as Format>::Quantum as Quantum>::BASE
    );
    assert_eq!(
        <Q as Quantum>::SLOPE,
        <<Shipped as Format>::Quantum as Quantum>::SLOPE
    );
    assert_eq!(
        <Q as Quantum>::MAGNITUDES,
        <<Shipped as Format>::Quantum as Quantum>::MAGNITUDES
    );
    assert_eq!(
        <S as Slots>::MAX,
        <<Shipped as Format>::Slots as Slots>::MAX
    );
    assert_eq!(
        <S as Slots>::WIDTH,
        <<Shipped as Format>::Slots as Slots>::WIDTH
    );
    // The one slot the two's complement range adds is `-2^24`. Below the top
    // magnitude it names `-2^23` a magnitude up, a binary32 value; at the top it
    // names `-2^128`, which binary32 does not hold.
    assert_eq!(
        <S as Slots>::MIN.index() - 1,
        <<Shipped as Format>::Slots as Slots>::MIN.index()
    );
    assert!(host32(Mode::HalfEven, -(1 << 24), 104).is_none());
    assert!(
        !land::<Attributed<Binary32, RoundTiesToEven, Saturate>>(-(1 << 24), 1, 104)
            .unwrap()
            .finite
    );
}
