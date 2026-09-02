//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ratio coordinate's own contract, which is a different subject from the
//! laws of the map that reads it.
//!
//! `Fraction::of` is an adaptation: a ratio in the ambient rationals onto the
//! pairs the coordinate carries with a positive denominator. So the arms here are
//! about a map, the same way the ones beside them are, and they are stated as the
//! four properties that map holds rather than as a list of inputs somebody
//! happened to write down.
//!
//! **The matrix is what makes them mean anything.** The property separating the
//! coordinate's two regions is the parity of the operand that is not `i64::MIN`,
//! and a matrix carrying only odd ones asserts the saturating half over an empty
//! set while every arm present looks reasonable. There is a control below whose
//! only job is to fail when that happens.

use crate::apply::{round_slot, Dither, Exact, Fraction};
use crate::rounding::Mode;
use crate::slots::Slot;

// --- 12. the fraction's own contract, which is why it is a type ------------

/// Every pair the matrix below reaches, so the four properties are asserted over
/// one set rather than over whichever set each arm happened to write.
///
/// Both parities on both axes, because parity is what decides whether a pair
/// with `i64::MIN` in it has an exact form at all, and a matrix carrying only
/// odd operands asserts the saturating half over nothing while every arm present
/// looks reasonable.
fn every_ratio() -> impl Iterator<Item = (i64, i64)> {
    // A binding rather than an item constant: a const here would be a coordinate
    // spelled in the host's own type, which the contract lint refuses in the one
    // crate that is otherwise allowed to name it.
    let values = [
        i64::MIN,
        i64::MIN + 1,
        i64::MIN + 2,
        -1_000_000_006,
        -1_000_000_007,
        -8,
        -7,
        -2,
        -1,
        0,
        1,
        2,
        7,
        i64::MAX - 1,
        i64::MAX,
    ];
    values
        .into_iter()
        .flat_map(move |n| values.into_iter().map(move |d| (n, d)))
}

/// Whether `n/d` has an exact form the coordinate can hold.
///
/// Derived rather than searched, and the derivation is what the exhaustive search
/// in `mock/research/sketches/202609021019_the-fraction-decision/` agrees with at
/// every width from three bits to eight: a pair has no exact form exactly when one
/// operand is `i64::MIN`, the denominator is negative, and the other operand is
/// odd, so nothing cancels.
fn has_an_exact_form(n: i64, d: i64) -> bool {
    if d >= 0 || n == 0 {
        return true;
    }
    if (n == i64::MIN) == (d == i64::MIN) {
        // neither is `i64::MIN`, so the sign simply moves; or both are, and the
        // ratio is one
        return true;
    }
    let other = if d == i64::MIN { n } else { d };
    other.trailing_zeros() > 0
}

/// The signum of the ratio `n/d`, with a zero denominator reading as no ratio.
fn ratio_sign(n: i64, d: i64) -> i64 {
    n.signum() * d.signum()
}

/// `|rn*d - n*rd|` as a magnitude, exactly.
///
/// Every term is at most `2^126` and the difference of two of them reaches
/// `2^127`, which is one past `i128`, so the sign comes off before the subtract.
fn error_numerator(r: Fraction, n: i64, d: i64) -> u128 {
    let a = (r.numerator().unsigned_abs() as u128) * (d.unsigned_abs() as u128);
    let b = (n.unsigned_abs() as u128) * (r.denominator().unsigned_abs() as u128);
    if ratio_sign(r.numerator(), d) == ratio_sign(n, r.denominator()) {
        if a > b {
            a - b
        } else {
            b - a
        }
    } else {
        a + b
    }
}

#[test]
fn the_ratio_coordinate_holds_its_four_properties_over_the_whole_matrix() {
    let mut exact_seen = 0usize;
    let mut saturated_seen = 0usize;
    for (n, d) in every_ratio() {
        let f = Fraction::of(n, d);

        // 1. the denominator is positive, which is what `Exact::between` reads
        assert!(
            f.denominator() > 0,
            "of({n}, {d}) produced a denominator of {}",
            f.denominator()
        );

        // a zero denominator names no ratio, so the three below have nothing to
        // be about
        if d == 0 {
            assert!(f.is_zero().get(), "of({n}, 0) should read as no position");
            continue;
        }

        // 2. the sign is the sign of the ratio that was named
        assert_eq!(
            ratio_sign(f.numerator(), f.denominator()),
            ratio_sign(n, d),
            "of({n}, {d}) answered {}/{} and changed the sign",
            f.numerator(),
            f.denominator()
        );

        let err = error_numerator(f, n, d);

        if has_an_exact_form(n, d) {
            // 3. exact wherever the coordinate admits an exact form
            exact_seen += 1;
            assert_eq!(
                err,
                0,
                "of({n}, {d}) answered {}/{} where an exact form exists",
                f.numerator(),
                f.denominator()
            );
        } else {
            // 4. within a relative `1 / i64::MAX` where it does not
            saturated_seen += 1;
            assert_ne!(n, 0, "a zero numerator always has an exact form");
            let bound = i64::MAX as u128;
            let scale = (f.denominator().unsigned_abs() as u128) * (n.unsigned_abs() as u128);
            let widened = err
                .checked_mul(bound)
                .expect("the relative error left u128, which the bound cannot be compared against");
            assert!(
                widened <= scale,
                "of({n}, {d}) answered {}/{} and left the 1/i64::MAX bound",
                f.numerator(),
                f.denominator()
            );
        }
    }

    // The control. Both regions have to be entered, or a rule that never
    // saturated and a rule that never cancelled would each pass one half of the
    // assertions above by never reaching the other.
    assert!(exact_seen > 0, "the exact region was never entered");
    assert!(
        saturated_seen > 0,
        "the saturating region was never entered"
    );
}

#[test]
fn the_control_the_matrix_carries_both_parities_against_the_least_value() {
    // The property that separates the two regions is the parity of the operand
    // that is not `i64::MIN`, so a matrix with only odd ones would assert the
    // second property over an empty set while looking complete. So the matrix is
    // checked for the parities before anything is concluded from it.
    let mut even_against_min = 0usize;
    let mut odd_against_min = 0usize;
    for (n, d) in every_ratio() {
        if d == i64::MIN && n != 0 && n != i64::MIN {
            if n.trailing_zeros() > 0 {
                even_against_min += 1;
            } else {
                odd_against_min += 1;
            }
        }
        if n == i64::MIN && d < 0 && d != i64::MIN {
            if d.trailing_zeros() > 0 {
                even_against_min += 1;
            } else {
                odd_against_min += 1;
            }
        }
    }
    assert!(
        even_against_min > 0,
        "no even operand against i64::MIN, so the cancelling case is untested"
    );
    assert!(
        odd_against_min > 0,
        "no odd operand against i64::MIN, so the saturating case is untested"
    );
}

#[test]
fn a_positive_denominator_is_kept_rather_than_replaced() {
    // The control on everything above: the ordinary path is the identity, so the
    // normalisation is about the pairs that need it.
    assert_eq!(Fraction::of(3, 7).numerator(), 3);
    assert_eq!(Fraction::of(3, 7).denominator(), 7);
    // And the exactly representable negative, which moves the sign and nothing else.
    assert_eq!(
        (
            Fraction::of(3, -7).numerator(),
            Fraction::of(3, -7).denominator()
        ),
        (-3, 7)
    );
    // A zero denominator names no position, so it is the one input that reads as
    // the zero position rather than as a ratio.
    assert!(Fraction::of(3, 0).is_zero().get());
}

#[test]
fn the_two_pairs_the_old_rule_named_now_keep_their_sign() {
    // Both sit in the saturating region, and both were answered with a
    // denominator of one and the wrong sign. `of(3, i64::MIN)` names a tiny
    // negative; `of(i64::MIN, -7)` names a large positive.
    let tiny = Fraction::of(3, i64::MIN);
    assert_eq!((tiny.numerator(), tiny.denominator()), (-3, i64::MAX));

    let large = Fraction::of(i64::MIN, -7);
    assert_eq!((large.numerator(), large.denominator()), (i64::MAX, 7));
}

#[test]
fn a_shared_factor_of_two_cancels_rather_than_saturating() {
    // Every one of these has an exact form inside the coordinate, reached by
    // cancelling the factor of two the two operands share.
    //
    // i64::MIN / -2 is 2^62 exactly.
    let halved = Fraction::of(i64::MIN, -2);
    assert_eq!((halved.numerator(), halved.denominator()), (1i64 << 62, 1));

    // i64::MIN / -8 is 2^60 exactly.
    let eighth = Fraction::of(i64::MIN, -8);
    assert_eq!((eighth.numerator(), eighth.denominator()), (1i64 << 60, 1));

    // 4 / i64::MIN is -1 over 2^61 exactly.
    let tiny = Fraction::of(4, i64::MIN);
    assert_eq!((tiny.numerator(), tiny.denominator()), (-1, 1i64 << 61));

    // i64::MIN over itself is one.
    let one = Fraction::of(i64::MIN, i64::MIN);
    assert_eq!((one.numerator(), one.denominator()), (1, 1));

    // Zero over the least value is zero, not a saturated approximation of it.
    assert!(Fraction::of(0, i64::MIN).is_zero().get());
}

#[test]
fn the_saturating_region_is_where_the_other_operand_is_odd() {
    // The negative control on the arm above: a rule that cancelled everything
    // would pass every exactness assertion by never entering this branch, so the
    // branch is named and its answer pinned.
    let odd_denominator = Fraction::of(i64::MIN, -3);
    assert_eq!(
        (odd_denominator.numerator(), odd_denominator.denominator()),
        (i64::MAX, 3)
    );

    let odd_numerator = Fraction::of(5, i64::MIN);
    assert_eq!(
        (odd_numerator.numerator(), odd_numerator.denominator()),
        (-5, i64::MAX)
    );

    // And the two are genuinely inexact, which is what makes them the second
    // region rather than a differently spelled exact answer.
    assert!(!has_an_exact_form(i64::MIN, -3));
    assert!(!has_an_exact_form(5, i64::MIN));
    assert!(error_numerator(odd_denominator, i64::MIN, -3) > 0);
    assert!(error_numerator(odd_numerator, 5, i64::MIN) > 0);
}

// --- 13. the tie comparison, at the top of the carrier ---------------------

#[test]
fn a_tie_is_decided_at_a_remainder_that_does_not_survive_doubling() {
    // The stored remainder reaches one below the denominator, so at a denominator
    // near the top of the type doubling it in that same carrier leaves the type,
    // which is why the comparison is cross-multiplied wide instead.
    let big = Exact::between(Slot::ZERO, Fraction::of(i64::MAX - 1, i64::MAX));
    assert!(
        !big.is_tie().get(),
        "a remainder just below one is not a tie"
    );

    // And the answer agrees with what `round_slot` decides at the same position,
    // which is the comparison the two were supposed to share.
    let down = big.slot().index() as i128;
    assert_eq!(
        round_slot(Mode::HalfEven, big, Dither::UNUSED),
        down + 1,
        "a remainder above the midpoint rounds up"
    );

    // The control: a real tie at a denominator whose doubling also leaves the
    // type is reported as one, so the widening did not answer no to everything.
    let half_of_max = Exact::between(Slot::ZERO, Fraction::of(i64::MAX - 1, 2));
    assert!(
        half_of_max.is_on_grid().get(),
        "an even numerator over two is whole"
    );

    let genuine = Exact::between(Slot::at(2), Fraction::HALF);
    assert!(genuine.is_tie().get(), "an ordinary tie stopped being one");
}

// --- 14. catalogued, and left red ------------------------------------------

#[test]
#[ignore = "catalogue: `Exact::between` adds the euclidean carry to the slot index in the \
            coordinate's own carrier, and a carry past the end of it has nowhere to go inside \
            `Slot`. `round_slot` widens on the way out, so the map admits a slot one past the \
            type after `Exact` is built and cannot admit one while it is being built. What a \
            carry past the coordinate means is a question about the slot coordinate rather than \
            the ratio, so it is stated here and left red until that coordinate answers it."]
fn a_carry_past_the_top_of_the_coordinate_still_lands_a_slot() {
    // The design says every position maps to a slot the format admits, for every
    // mode and every policy. This position does not reach the map at all.
    let e = Exact::between(Slot::at(i64::MAX), Fraction::of(9, 4));
    assert_eq!(
        e.slot().index(),
        i64::MAX,
        "the carry has to land somewhere the coordinate can hold"
    );
}
