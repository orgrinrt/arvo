//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The four-limb integer against the host's 128-bit arithmetic, and at the edge
//! of its own four limbs.
//!
//! Wherever the host's integer holds the answer it is the oracle: every pair of
//! a list running through both signs, the limb boundary at `2^64` and the ends of
//! the index. Past it, the powers of two and ten that sit on either side of
//! `2^256` decide whether the limbs are left.

use notko::Maybe;

use super::Wide;

/// Values through both signs, the limb boundary and the ends of the index.
fn values() -> [i128; 17] {
    [
        0,
        1,
        -1,
        2,
        -3,
        7,
        -10,
        i64::MAX as i128,
        i64::MIN as i128,
        u64::MAX as i128,
        -(u64::MAX as i128),
        1 << 64,
        -(1 << 64),
        (1 << 64) + 1,
        1 << 100,
        i128::MAX,
        i128::MIN,
    ]
}

/// Positive divisors, small and up to the largest the step uses.
fn divisors() -> [u64; 7] {
    [1, 2, 3, 10, 1 << 32, i64::MAX as u64 - 6, i64::MAX as u64]
}

/// The value narrowed back, which every value of the host's integer has.
fn index(w: Wide) -> i128 {
    match w.index() {
        Maybe::Is(v) => v,
        Maybe::Isnt => panic!("{w:?} was formed from an index value and has to narrow back"),
    }
}

#[test]
fn a_value_of_the_index_survives_the_round_trip() {
    for v in values() {
        assert_eq!(index(Wide::of(v)), v);
    }
    assert!(Wide::of(0).is_zero().get());
    assert!(!Wide::of(0).is_negative().get());
    assert!(Wide::of(-1).is_negative().get());
}

#[test]
fn the_product_is_the_host_product_where_the_host_holds_it() {
    let mut compared = 0;
    for a in values() {
        for b in values() {
            let Some(host) = a.checked_mul(b) else { continue };
            let Maybe::Is(p) = Wide::of(a).times(Wide::of(b)) else {
                panic!("{a} * {b} is inside the index and so inside the limbs");
            };
            assert_eq!(index(p), host, "{a} * {b}");
            compared += 1;
        }
    }
    assert!(compared > 100, "{compared}");
}

#[test]
fn a_product_past_the_index_is_held_and_leaves_the_index_on_narrowing() {
    // `i128::MIN` squared is `2^254`, inside the limbs and outside the index.
    let Maybe::Is(p) = Wide::of(i128::MIN).times(Wide::of(i128::MIN)) else {
        panic!("2^254 is inside the limbs");
    };
    assert!(!p.is_negative().get());
    assert!(matches!(p.index(), Maybe::Isnt));
    let Maybe::Is(q) = Wide::of(i128::MIN).times(Wide::of(i128::MAX)) else {
        panic!("-2^127 (2^127 - 1) is inside the limbs");
    };
    assert!(q.is_negative().get());
}

#[test]
fn the_sum_is_the_host_sum_where_the_host_holds_it() {
    for a in values() {
        for b in values() {
            let Some(host) = a.checked_add(b) else { continue };
            let Maybe::Is(s) = Wide::of(a).plus(Wide::of(b)) else {
                panic!("{a} + {b} is inside the limbs");
            };
            assert_eq!(index(s), host, "{a} + {b}");
        }
    }
}

#[test]
fn the_floor_division_is_the_host_euclidean_division_at_a_positive_divisor() {
    for v in values() {
        for d in divisors() {
            let got = Wide::of(v).floor_divided(d);
            let host = v.div_euclid(d as i128);
            assert_eq!(index(got.quotient), host, "{v} / {d}");
            assert_eq!(got.remainder as i128, v.rem_euclid(d as i128), "{v} % {d}");
        }
    }
}

#[test]
fn a_power_is_the_host_power_where_the_host_holds_it() {
    for base in [0u64, 1, 2, 3, 10] {
        for e in 0u32 .. 40 {
            let Some(host) = (base as i128).checked_pow(e) else { continue };
            let Maybe::Is(p) = Wide::power(base, e) else {
                panic!("{base}^{e} is inside the index");
            };
            assert_eq!(index(p), host, "{base}^{e}");
        }
    }
    // A base of zero or one never grows, at an exponent whose loop would not end.
    assert!(matches!(Wide::power(1, u32::MAX), Maybe::Is(p) if index(p) == 1));
    assert!(matches!(Wide::power(0, u32::MAX), Maybe::Is(p) if p.is_zero().get()));
}

#[test]
fn the_limbs_are_left_exactly_at_two_to_the_256() {
    assert!(matches!(Wide::power(2, 255), Maybe::Is(_)));
    assert!(matches!(Wide::power(2, 256), Maybe::Isnt));
    // `10^77` is about `1.0e77`, under `2^256`, about `1.16e77`; `10^78` is past it.
    assert!(matches!(Wide::power(10, 77), Maybe::Is(_)));
    assert!(matches!(Wide::power(10, 78), Maybe::Isnt));
    let Maybe::Is(half) = Wide::power(2, 255) else { unreachable!() };
    let Maybe::Is(below) = half.plus(Wide::of(-1)) else { unreachable!() };
    let Maybe::Is(top) = half.plus(below) else {
        panic!("2^256 - 1 is the top of the limbs");
    };
    assert!(matches!(top.plus(Wide::of(1)), Maybe::Isnt));
    assert!(matches!(top.plus(Wide::of(-1)), Maybe::Is(_)));
    assert!(matches!(half.times(Wide::of(2)), Maybe::Isnt));
    assert!(matches!(half.times(Wide::of(-1)), Maybe::Is(_)));
}

#[test]
fn a_negative_value_floors_past_the_index_and_keeps_its_remainder() {
    // `-(2^200) - 1` over two: quotient `-(2^199) - 1`, remainder one.
    let Maybe::Is(p) = Wide::power(2, 200) else { unreachable!() };
    let Maybe::Is(v) = p.times(Wide::of(-1)) else { unreachable!() };
    let Maybe::Is(v) = v.plus(Wide::of(-1)) else { unreachable!() };
    let got = v.floor_divided(2);
    assert_eq!(got.remainder, 1);
    let Maybe::Is(half) = Wide::power(2, 199) else { unreachable!() };
    let Maybe::Is(expected) = half.times(Wide::of(-1)) else { unreachable!() };
    let Maybe::Is(expected) = expected.plus(Wide::of(-1)) else { unreachable!() };
    assert_eq!(got.quotient, expected);
}
