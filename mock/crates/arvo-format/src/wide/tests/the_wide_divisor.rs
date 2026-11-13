//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The four-limb integer over a divisor of three and four limbs, its order, the
//! comparison of two scaled values, and the common divisor, against the shared
//! wide oracle.
//!
//! The oracle divides one bit at a time and reads nothing of the long division
//! the step runs, so where the two agree on the quotient and the remainder the
//! step's estimate, its corrections and its add-back are right at that pair.

use core::cmp::Ordering;

use notko::Maybe;

use super::super::Wide;
use super::{Draws, big_of};
use crate::tests::big::Big;

/// `n` over `d`, four limbs each, against the oracle's quotient and remainder.
fn agrees(negative: bool, m: [u64; 4], d: [u64; 4]) {
    let got = Wide::signed(negative, m).floor_divided(Wide::signed(false, d));
    let (q, r) = big_of(negative, m).floor_over_big(big_of(false, d));
    assert_eq!(
        (got.quotient.negative, got.quotient.magnitude),
        q.limbs(),
        "the quotient of {negative} {m:?} over {d:?}"
    );
    assert_eq!(
        (false, got.remainder.magnitude),
        r.limbs(),
        "the remainder of {negative} {m:?} over {d:?}"
    );
    assert!(!got.remainder.negative);
}

/// Four draws, the top limb's top bit at `top` and nothing above it.
fn divisor_topped_at(draws: &mut Draws, top: u32) -> [u64; 4] {
    let mut d = [draws.next(), draws.next(), draws.next(), draws.next()];
    let limb = (top / 64) as usize;
    let bit = top % 64;
    d[limb] = (d[limb] & (u64::MAX >> (63 - bit))) | 1 << bit;
    for above in &mut d[limb + 1 ..] {
        *above = 0;
    }
    d
}

#[test]
fn a_division_by_three_and_four_limbs_is_the_oracle_s_at_every_normalisation() {
    // The divisor's top bit at every position from 255 down to 128, so every
    // normalisation shift at both widths, each against dividends through both
    // signs, the largest dividend, the divisor itself and one under it.
    let mut draws = Draws(0x0D1F_0003);
    let mut checked = 0;
    for top in (128 .. 256).rev() {
        let d = divisor_topped_at(&mut draws, top);
        for _ in 0 .. 6 {
            let m = [draws.next(), draws.next(), draws.next(), draws.next()];
            agrees(draws.next() & 1 == 1, m, d);
            checked += 1;
        }
        let mut under = d;
        let k = under.iter().position(|&l| l != 0).unwrap_or(0);
        under[k] -= 1;
        for negative in [false, true] {
            agrees(negative, [u64::MAX; 4], d);
            agrees(negative, d, d);
            agrees(negative, under, d);
            checked += 3;
        }
    }
    assert_eq!(checked, 128 * 12);
}

#[test]
fn the_add_back_is_reached_and_corrects_the_quotient_limb() {
    // Found by search against the oracle: at three limbs the estimate, corrected
    // twice against the top two limbs, still takes one too many divisors here,
    // so the subtraction borrows and one divisor is added back.
    let m = [0, 0, 1 << 63, (1 << 63) - 1];
    let d = [1, 0, 1 << 63, 0];
    for negative in [false, true] {
        agrees(negative, m, d);
    }
}

#[test]
fn a_quotient_limb_of_the_base_less_one_is_found_at_three_limbs() {
    // `(B - 1) d + d - 1`, where the running remainder's top limb meets the
    // divisor's and the estimate starts at its highest.
    let mut draws = Draws(0x0D1F_0004);
    for top in [128, 150, 191, 192] {
        let d = divisor_topped_at(&mut draws, top);
        let Maybe::Is(product) =
            Wide::signed(false, [u64::MAX, 0, 0, 0]).times(Wide::signed(false, d))
        else {
            continue;
        };
        let mut less = d;
        let k = less.iter().position(|&l| l != 0).unwrap_or(0);
        less[k] -= 1;
        let Maybe::Is(n) = product.plus(Wide::signed(false, less)) else {
            continue;
        };
        agrees(false, n.magnitude, d);
        agrees(true, n.magnitude, d);
    }
}

#[test]
fn the_order_of_two_values_is_the_sign_of_their_difference() {
    let mut draws = Draws(0x0D1F_0005);
    for _ in 0 .. 400 {
        let (sa, sb) = (draws.next() & 1 == 1, draws.next() & 1 == 1);
        let a = [draws.next(), draws.next(), draws.next(), draws.next() >> 1];
        // Every third pair shares its top limbs, so the order is found low.
        let b = if draws.next().is_multiple_of(3) {
            [draws.next(), a[1], a[2], a[3]]
        } else {
            [draws.next(), draws.next(), draws.next(), draws.next() >> 1]
        };
        let difference = big_of(sa, a).plus(big_of(sb, b).times(Big::of(-1)));
        let expected = if difference.is_negative() {
            Ordering::Less
        } else if difference == Big::of(0) {
            Ordering::Equal
        } else {
            Ordering::Greater
        };
        let got = Wide::signed(sa, a).ordering(Wide::signed(sb, b));
        assert_eq!(got, expected, "{sa} {a:?} against {sb} {b:?}");
    }
    assert_eq!(Wide::of(0).ordering(Wide::of(0).negated()), Ordering::Equal);
}

#[test]
fn a_scaled_comparison_is_the_oracle_s_where_both_products_hold() {
    let mut draws = Draws(0x0D1F_0006);
    for _ in 0 .. 400 {
        let a = [draws.next(), draws.next(), draws.next() >> 2, 0];
        let b = [draws.next(), draws.next(), draws.next() >> 2, 0];
        let (x, y) = (draws.next(), draws.next());
        let left = big_of(false, a).times(Big::of_unsigned(x.into()));
        let right = big_of(false, b).times(Big::of_unsigned(y.into()));
        let difference = left.plus(right.times(Big::of(-1)));
        let expected = if difference.is_negative() {
            Ordering::Less
        } else if difference == Big::of(0) {
            Ordering::Equal
        } else {
            Ordering::Greater
        };
        let got = Wide::signed(false, a).scaled_against(x, Wide::signed(false, b), y);
        assert_eq!(got, expected, "{a:?} * {x} against {b:?} * {y}");
    }
}

#[test]
fn a_scaled_comparison_holds_its_fifth_limb() {
    // `3 * 2^255` against `2^255 + 1`: the left product passes `2^256`, and cut
    // to four limbs it would read `2^255`, below the right.
    let half = Wide::signed(false, [0, 0, 0, 1 << 63]);
    let above = Wide::signed(false, [1, 0, 0, 1 << 63]);
    assert_eq!(half.scaled_against(3, above, 1), Ordering::Greater);
    assert_eq!(above.scaled_against(1, half, 3), Ordering::Less);
    // `2^255 * 2` against `2^254 * 4`, both `2^256`, equal.
    let quarter = Wide::signed(false, [0, 0, 0, 1 << 62]);
    assert_eq!(half.scaled_against(2, quarter, 4), Ordering::Equal);
    // The largest value at the largest scale against itself one scale lower.
    let top = Wide::signed(false, [u64::MAX; 4]);
    assert_eq!(
        top.scaled_against(u64::MAX, top, u64::MAX - 1),
        Ordering::Greater
    );
}

#[test]
fn the_common_divisor_is_the_oracle_s_past_two_limbs() {
    let mut draws = Draws(0x0D1F_0007);
    for _ in 0 .. 60 {
        // A shared factor of up to two limbs, times cofactors of up to one.
        let g = [draws.next(), draws.next() >> (draws.next() % 64), 0, 0];
        let g = if g == [0; 4] { [1, 0, 0, 0] } else { g };
        let (x, y) = (draws.next() | 1, draws.next() >> (draws.next() % 64));
        let Maybe::Is(a) = Wide::signed(false, g).times(Wide::signed(false, [x, 0, 0, 0])) else {
            unreachable!()
        };
        let Maybe::Is(b) = Wide::signed(false, g).times(Wide::signed(false, [y, 0, 0, 0])) else {
            unreachable!()
        };
        let expected = big_of(false, a.magnitude).common_divisor(big_of(false, b.magnitude));
        let got = a.common_divisor(b);
        assert_eq!(
            (got.negative, got.magnitude),
            expected.limbs(),
            "{a:?} {b:?}"
        );
    }
}
