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
use crate::tests::big::Big;

mod the_wide_divisor;

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

/// Positive divisors, one limb and two, up to the top of the host's signed
/// 128-bit integer, so the host's own division is the oracle at every one.
fn divisors() -> [u128; 12] {
    [
        1,
        2,
        3,
        10,
        1 << 32,
        i64::MAX as u128 - 6,
        i64::MAX as u128,
        u64::MAX as u128,
        1 << 64,
        (1 << 64) + 1,
        (3 << 70) + 5,
        i128::MAX as u128,
    ]
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
            let got = floor(Wide::of(v), d);
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
    let got = floor(v, 2);
    assert_eq!(got.remainder, 1);
    let Maybe::Is(half) = Wide::power(2, 199) else { unreachable!() };
    let Maybe::Is(expected) = half.times(Wide::of(-1)) else { unreachable!() };
    let Maybe::Is(expected) = expected.plus(Wide::of(-1)) else { unreachable!() };
    assert_eq!(got.quotient, expected);
}

#[test]
fn a_division_across_three_and_four_limbs_multiplies_back_at_divisors_not_powers_of_two() {
    // Numerators of three and four limbs, each moved off the divisors' grid,
    // on both signs, over divisors of one limb and of two up to the top of the
    // host's unsigned 128-bit integer. Every limb's remainder carries into the
    // one below it, so a carry dropped anywhere shows in the product read back,
    // and the remainder is checked against the shared wide oracle, whose
    // division is another algorithm on each side of `2^64`.
    let mut checked = 0;
    for (radix, power) in [(10, 45), (3, 120), (3, 150), (7, 90)] {
        let Maybe::Is(p) = Wide::power(radix, power) else { unreachable!() };
        for sign in [1, -1] {
            let Maybe::Is(n) = p.times(Wide::of(sign)) else { unreachable!() };
            let Maybe::Is(n) = n.plus(Wide::of(12_345)) else { unreachable!() };
            assert!(
                matches!(n.index(), Maybe::Isnt),
                "{radix}^{power} is past the index"
            );
            let oracle = Big::power(radix as i128, power)
                .times(Big::of(sign))
                .plus(Big::of(12_345));
            for d in [
                3,
                7,
                1_000_000_000_000_000_000,
                i64::MAX as u128,
                (1 << 64) + 1,
                (3 << 70) + 5,
                i128::MAX as u128 - 2,
                u128::MAX,
            ] {
                let got = floor(n, d);
                assert!(got.remainder < d, "{radix}^{power} over {d}");
                assert_eq!(
                    got.remainder,
                    oracle.floor_over(d).1,
                    "{radix}^{power} over {d}"
                );
                let Maybe::Is(back) = got.quotient.times(unsigned(d)) else {
                    panic!(
                        "{radix}^{power} over {d}: the quotient times the divisor left the limbs"
                    );
                };
                let Maybe::Is(back) = back.plus(unsigned(got.remainder)) else {
                    panic!("{radix}^{power} over {d}: the remainder added left the limbs");
                };
                assert_eq!(back, n, "{radix}^{power} over {d}");
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 4 * 2 * 8);
}

#[test]
fn a_two_limb_division_floors_a_negative_value_and_counts_its_remainder_from_above() {
    // `-(2^200) - 1` over `2^128 - 1`, whose quotient and remainder are found
    // by hand: `2^200 = 2^72 (2^128 - 1) + 2^72`, so `-(2^200) - 1` floors to
    // `-(2^72) - 1` with remainder `2^128 - 1 - 2^72 - 1`.
    let Maybe::Is(p) = Wide::power(2, 200) else { unreachable!() };
    let Maybe::Is(v) = p.times(Wide::of(-1)) else { unreachable!() };
    let Maybe::Is(v) = v.plus(Wide::of(-1)) else { unreachable!() };
    let got = floor(v, u128::MAX);
    assert_eq!(got.remainder, u128::MAX - (1 << 72) - 1);
    assert_eq!(index(got.quotient), -(1i128 << 72) - 1);
    // The positive twin: `2^200 + 1` over the same divisor floors to `2^72`.
    let Maybe::Is(w) = p.plus(Wide::of(1)) else { unreachable!() };
    let got = floor(w, u128::MAX);
    assert_eq!(got.remainder, (1 << 72) + 1);
    assert_eq!(index(got.quotient), 1i128 << 72);
}

/// A value of the host's unsigned 128-bit integer, built through the signed
/// entry so the suite reads nothing the step does not.
fn unsigned(v: u128) -> Wide {
    let Maybe::Is(half) = Wide::of((v >> 1) as i128).times(Wide::of(2)) else {
        unreachable!()
    };
    let Maybe::Is(whole) = half.plus(Wide::of((v & 1) as i128)) else {
        unreachable!()
    };
    whole
}

#[test]
fn a_value_narrows_to_a_denominator_exactly_when_it_is_one() {
    // The narrowing `D` is read through: every non-negative value under `2^128`
    // narrows to itself, and below zero or from `2^128` up nothing does.
    for v in [0u128, 1, u64::MAX as u128, 1 << 64, i128::MAX as u128, 1 << 127, u128::MAX] {
        assert!(
            matches!(unsigned(v).unsigned(), Maybe::Is(got) if got == v),
            "{v}"
        );
    }
    let Maybe::Is(top) = Wide::power(2, 128) else { unreachable!() };
    assert!(matches!(top.unsigned(), Maybe::Isnt));
    assert!(matches!(Wide::of(-1).unsigned(), Maybe::Isnt));
    assert!(matches!(
        unsigned(u128::MAX).negated().unsigned(),
        Maybe::Isnt
    ));
    assert_eq!(unsigned(u128::MAX).negated().negated(), unsigned(u128::MAX));
    assert_eq!(Wide::of(0).negated(), Wide::of(0));
}

/// The floor of `n` over a divisor the host's unsigned 128-bit integer holds,
/// with the remainder narrowed back to it, which it always is, being below `d`.
struct Floor {
    quotient:  Wide,
    remainder: u128,
}

fn floor(n: Wide, d: u128) -> Floor {
    let got = n.floor_divided(unsigned(d));
    let Maybe::Is(remainder) = got.remainder.unsigned() else {
        panic!("{n:?} over {d}: a remainder below the divisor left 128 bits");
    };
    Floor {
        quotient: got.quotient,
        remainder,
    }
}

/// A deterministic stream of 64-bit draws, SplitMix64.
struct Draws(u64);

impl Draws {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// The oracle's value of four limbs with a sign.
fn big_of(negative: bool, m: [u64; 4]) -> Big {
    let low = Big::of_unsigned((m[0] as u128) | ((m[1] as u128) << 64));
    let high = Big::of_unsigned((m[2] as u128) | ((m[3] as u128) << 64));
    let v = high.times(Big::power(2, 128)).plus(low);
    if negative { v.times(Big::of(-1)) } else { v }
}

/// `n` over `d` against the oracle's remainder, and multiplied back.
fn divides_back(negative: bool, m: [u64; 4], d: u128) {
    let n = Wide::signed(negative, m);
    let got = floor(n, d);
    assert!(
        got.remainder < d,
        "{m:?} over {d}: remainder {}",
        got.remainder
    );
    assert_eq!(
        got.remainder,
        big_of(negative, m).floor_over(d).1,
        "{negative} {m:?} over {d}"
    );
    // On magnitudes, since a floored negative quotient times the divisor can pass
    // `-2^256` where the dividend does not: `|n| = |q| d - (d - r)` below zero with a
    // remainder, and `|q| d + r` otherwise.
    let size = Wide::signed(false, got.quotient.magnitude);
    let (size, rest) = if negative && got.remainder != 0 {
        let Maybe::Is(less) = size.plus(Wide::of(-1)) else { unreachable!() };
        (less, d - got.remainder)
    } else {
        (size, got.remainder)
    };
    let Maybe::Is(back) = size.times(unsigned(d)) else {
        panic!("{negative} {m:?} over {d}: the quotient times the divisor left the limbs");
    };
    let Maybe::Is(back) = back.plus(unsigned(rest)) else {
        panic!("{negative} {m:?} over {d}: the remainder added left the limbs");
    };
    assert_eq!(back, Wide::signed(false, m), "{negative} {m:?} over {d}");
    assert_eq!(
        got.quotient.negative,
        negative && !got.quotient.is_zero().get()
    );
}

#[test]
fn a_two_limb_division_is_the_oracle_s_at_every_normalisation_and_at_the_estimate_s_edges() {
    // Divisors with their top bit at every position from 127 down to 64, so every
    // normalisation shift from 0 to 63, beside the two ends of the two-limb range
    // and a divisor whose low limb is zero.
    let mut draws = Draws(0x0D1F_0002);
    let mut divisors = [0u128; 64 + 5];
    for (shift, slot) in divisors[.. 64].iter_mut().enumerate() {
        let d = ((draws.next() as u128) << 64 | draws.next() as u128) | 1 << 127;
        *slot = d >> shift;
    }
    divisors[64 ..].copy_from_slice(&[
        1 << 64,
        (1 << 64) + 1,
        1 << 127,
        u128::MAX,
        u128::MAX << 64,
    ]);
    let mut checked = 0;
    for d in divisors {
        assert!(d >> 64 != 0);
        for _ in 0 .. 16 {
            let m = [draws.next(), draws.next(), draws.next(), draws.next()];
            divides_back(draws.next() & 1 == 1, m, d);
            checked += 1;
        }
        // The largest dividend, where each estimate starts at its highest.
        for negative in [false, true] {
            divides_back(negative, [u64::MAX; 4], d);
            divides_back(negative, [0, 0, 0, u64::MAX], d);
            checked += 2;
        }
        // A quotient limb of `B - 1` at each place, plus the largest remainder,
        // where the running remainder's top limb meets the divisor's and the
        // estimate is corrected down.
        for place in 0 .. 3 {
            let mut q = [0u64; 4];
            q[place] = u64::MAX;
            let Maybe::Is(product) = Wide::signed(false, q).times(unsigned(d)) else {
                continue;
            };
            let Maybe::Is(n) = product.plus(unsigned(d - 1)) else {
                continue;
            };
            divides_back(false, n.magnitude, d);
            divides_back(true, n.magnitude, d);
            checked += 2;
        }
    }
    assert!(checked >= 69 * 20, "{checked}");
}
