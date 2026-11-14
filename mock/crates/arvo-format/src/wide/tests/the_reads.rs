//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The four reads the applied map takes from the four-limb integer, `of_sum`,
//! `end`, `residue` and `is_even`, against values written as literal limbs.
//!
//! Each expected value is worked by hand and written as its sign and its limbs,
//! lowest first, so nothing of the integer's own arithmetic forms it.

use super::super::Wide;

#[test]
fn a_sum_of_two_index_values_carries_into_the_third_limb() {
    // `2 (2^127 - 1) = 2^128 - 2`, and `2 (-2^127) = -2^128`.
    assert_eq!(
        Wide::of_sum(i128::MAX, i128::MAX),
        Wide::signed(false, [u64::MAX - 1, u64::MAX, 0, 0])
    );
    assert_eq!(
        Wide::of_sum(i128::MIN, i128::MIN),
        Wide::signed(true, [0, 0, 1, 0])
    );
    // One past either end of the index: `2^127` and `-(2^127 + 1)`.
    assert_eq!(
        Wide::of_sum(i128::MAX, 1),
        Wide::signed(false, [0, 1 << 63, 0, 0])
    );
    assert_eq!(
        Wide::of_sum(i128::MIN, -1),
        Wide::signed(true, [1, 1 << 63, 0, 0])
    );
    // Across the sign nothing carries, and zero is not negative.
    assert_eq!(
        Wide::of_sum(i128::MIN, i128::MAX),
        Wide::signed(true, [1, 0, 0, 0])
    );
    assert_eq!(Wide::of_sum(0, 0), Wide::signed(false, [0; 4]));
    assert!(!Wide::of_sum(5, -5).is_negative().get());
}

#[test]
fn the_ends_of_the_four_limbs_are_every_bit_set() {
    assert_eq!(Wide::end(false), Wide::signed(false, [u64::MAX; 4]));
    assert_eq!(Wide::end(true), Wide::signed(true, [u64::MAX; 4]));
}

#[test]
fn a_residue_is_the_least_non_negative_one() {
    let end = |negative| Wide::signed(negative, [u64::MAX; 4]);
    let below_by_2_to_the_128 = Wide::signed(true, [0, 0, 1, 0]);
    // `2^256 - 1` is `2^64 - 1` modulo `2^64`, so its negation is 1.
    assert_eq!(end(false).residue(1 << 64), i128::from(u64::MAX));
    assert_eq!(end(true).residue(1 << 64), 1);
    // Worked by hand: `2^256` is 136 modulo 200, so `2^256 - 1` is 135 and its
    // negation 65.
    assert_eq!(end(false).residue(200), 135);
    assert_eq!(end(true).residue(200), 65);
    // `2^128` is 1 modulo 3, since 2 is -1 and the power is even, so `-2^128` is 2.
    assert_eq!(below_by_2_to_the_128.residue(3), 2);
    // `-2^127` is -1 modulo `2^127 - 1`, read back from both low limbs.
    assert_eq!(
        Wide::signed(true, [0, 1 << 63, 0, 0]).residue(i128::MAX),
        i128::MAX - 1
    );
    // Everything is zero modulo one.
    for v in [end(false), end(true), below_by_2_to_the_128, Wide::signed(false, [0; 4])] {
        assert_eq!(v.residue(1), 0);
    }
}

#[test]
fn evenness_is_the_magnitude_s_and_not_the_sign_s() {
    assert!(!Wide::signed(true, [3, 0, 0, 0]).is_even().get());
    assert!(!Wide::signed(true, [u64::MAX; 4]).is_even().get());
    assert!(Wide::signed(true, [0, 0, 1, 0]).is_even().get());
    assert!(Wide::signed(true, [0, 1, 0, 0]).is_even().get());
    assert!(Wide::signed(false, [0; 4]).is_even().get());
    assert!(
        !Wide::signed(false, [1, u64::MAX, u64::MAX, u64::MAX])
            .is_even()
            .get()
    );
}
