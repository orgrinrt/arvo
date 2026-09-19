//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An oracle for the applied map that shares none of its arithmetic.
//!
//! The map carries a position past either end of the index as a pinned slot, a
//! step and a distance, because the index's own integer is the widest it has.
//! The oracle does not: it holds the position as an integer two limbs wide, so a
//! slot eight past `i128::MAX` is an ordinary value to it, and it answers from
//! what each mode and each policy is documented to do. It never reads `Exact`,
//! `Rounded`, `round_slot` or `complete_slot`, which is what lets it disagree
//! with them.
//!
//! Each mode picks between the two neighbours of an off-grid position by the
//! words its doc gives: the lower or the higher neighbour, the one of smaller
//! magnitude, the nearer one with a tie going to the larger magnitude or to the
//! even one, and for the stochastic mode the higher neighbour exactly when the
//! dither lies below the position's remainder. Saturation and clamping pin to
//! the end of the range on the side the rounded position left by, and wrapping
//! reduces its distance from the lowest slot modulo the span.

use core::cmp::Ordering;

use notko::Maybe;

use crate::overflow::Policy;
use crate::rounding::Mode;

/// `2^64`, the weight of the high limb.
const fn limb() -> i128 {
    1 << 64
}

/// An integer two limbs wide, `hi * 2^64 + lo`, with `lo` in `[0, 2^64)`.
///
/// Wide enough that every position within `2^63` of either end of the index,
/// and every difference of two such, is exact in it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Wide {
    hi: i128,
    lo: i128,
}

impl Wide {
    /// The index value `v`, exactly.
    pub(super) fn of(v: i128) -> Self {
        Self {
            hi: v >> 64,
            lo: v & (limb() - 1),
        }
    }

    /// The sum, exactly.
    pub(super) fn plus(self, other: Self) -> Self {
        let lo = self.lo + other.lo;
        Self {
            hi: self.hi + other.hi + (lo >> 64),
            lo: lo & (limb() - 1),
        }
    }

    /// The negation, exactly.
    pub(super) fn negated(self) -> Self {
        if self.lo == 0 {
            Self {
                hi: -self.hi,
                lo: 0,
            }
        } else {
            Self {
                hi: -self.hi - 1,
                lo: limb() - self.lo,
            }
        }
    }

    /// The difference, exactly.
    pub(super) fn minus(self, other: Self) -> Self {
        self.plus(other.negated())
    }

    /// Whether it is below zero.
    pub(super) fn is_negative(self) -> bool {
        self.hi < 0
    }

    /// Whether it is even.
    pub(super) fn is_even(self) -> bool {
        self.lo % 2 == 0
    }

    /// The magnitude, exactly.
    pub(super) fn magnitude(self) -> Self {
        if self.is_negative() { self.negated() } else { self }
    }

    /// The value as an index, where the index holds it.
    pub(super) fn index(self) -> Maybe<i128> {
        Maybe::from(
            self.hi
                .checked_mul(limb())
                .and_then(|high| high.checked_add(self.lo)),
        )
    }

    /// The order of two values, exactly.
    pub(super) fn order(self, other: Self) -> Ordering {
        (self.hi, self.lo).cmp(&(other.hi, other.lo))
    }

    /// Whether it lies below `other`.
    pub(super) fn is_below(self, other: Self) -> bool {
        self.order(other) == Ordering::Less
    }

    /// The least non-negative residue modulo `span`, for a span in `[1, 2^64]`.
    ///
    /// Each limb is reduced on its own and the two recombined in an unsigned
    /// 128-bit integer, where a product of two residues below `2^64` fits.
    pub(super) fn rem_euclid(self, span: i128) -> i128 {
        assert!(
            0 < span && span <= limb(),
            "a span the oracle does not reduce by"
        );
        let s = span as u128;
        let high = self.hi.rem_euclid(span) as u128;
        let weight = (limb() as u128) % s;
        (((high * weight) % s + (self.lo as u128) % s) % s) as i128
    }
}

/// A position as the oracle reads it: `whole + num / den`, `0 <= num < den`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Point {
    pub(super) whole: Wide,
    pub(super) num:   i64,
    pub(super) den:   i64,
}

/// The slot a mode puts the position on, as the oracle reads the mode.
///
/// The dither is a ratio in `[0, 1)`, read by the stochastic mode alone.
pub(super) fn rounded(mode: Mode, p: Point, dither: (i64, i64)) -> Wide {
    if p.num == 0 {
        return p.whole;
    }
    let (lower, higher) = (p.whole, p.whole.plus(Wide::of(1)));
    // The distances to the two neighbours are `num / den` and `(den - num) / den`.
    let (to_lower, to_higher) = (p.num, p.den - p.num);
    let lower_is_smaller = lower.magnitude().is_below(higher.magnitude());
    let smaller_magnitude = if lower_is_smaller { lower } else { higher };
    let larger_magnitude = if lower_is_smaller { higher } else { lower };
    let nearer = match to_lower.cmp(&to_higher) {
        Ordering::Less => Maybe::Is(lower),
        Ordering::Greater => Maybe::Is(higher),
        Ordering::Equal => Maybe::Isnt,
    };
    match mode {
        Mode::Floor => lower,
        Mode::Ceil => higher,
        Mode::TowardZero => smaller_magnitude,
        Mode::HalfUp => nearer.unwrap_or(larger_magnitude),
        Mode::HalfEven => nearer.unwrap_or(if lower.is_even() { lower } else { higher }),
        Mode::Stochastic => {
            let (dn, dd) = (dither.0 as i128, dither.1 as i128);
            if dn * (p.den as i128) < (p.num as i128) * dd { higher } else { lower }
        },
    }
}

/// The slot the policy returns for a rounded position and a range `[lo, hi]`,
/// and whether the position left the range.
pub(super) fn completed(policy: Policy, at: Wide, lo: i128, hi: i128) -> (i128, bool) {
    let (low, high) = (Wide::of(lo), Wide::of(hi));
    if !at.is_below(low) && !high.is_below(at) {
        return (at.index().expect("inside a range the index holds"), false);
    }
    let slot = match policy {
        Policy::Wrap => lo + at.minus(low).rem_euclid(hi - lo + 1),
        Policy::Saturate | Policy::Clamp => {
            if at.is_below(low) {
                lo
            } else {
                hi
            }
        },
    };
    (slot, true)
}

/// The oracle's answer: the slot, and the overflow verdict.
pub(super) fn answer(
    mode: Mode,
    policy: Policy,
    p: Point,
    dither: (i64, i64),
    lo: i128,
    hi: i128,
) -> (i128, bool) {
    completed(policy, rounded(mode, p, dither), lo, hi)
}

// --- the oracle's own arithmetic, checked where the index can check it ----------

/// Values at and around both ends of the index, around zero, and around the
/// limb boundary.
fn samples() -> [i128; 17] {
    [
        i128::MIN,
        i128::MIN + 1,
        i128::MIN + 7,
        -limb() - 1,
        -limb(),
        -limb() + 1,
        -3,
        -1,
        0,
        1,
        2,
        limb() - 1,
        limb(),
        limb() + 1,
        i128::MAX - 7,
        i128::MAX - 1,
        i128::MAX,
    ]
}

#[test]
fn the_two_limbs_agree_with_the_index_wherever_the_index_holds_the_answer() {
    for a in samples() {
        assert_eq!(Wide::of(a).index(), Maybe::Is(a), "{a}");
        assert_eq!(Wide::of(a).is_negative(), a < 0, "{a}");
        assert_eq!(Wide::of(a).is_even(), a % 2 == 0, "{a}");
        for b in samples() {
            assert_eq!(Wide::of(a).order(Wide::of(b)), a.cmp(&b), "{a} {b}");
            assert_eq!(Wide::of(a).is_below(Wide::of(b)), a < b, "{a} {b}");
            assert_eq!(
                Wide::of(a).plus(Wide::of(b)).index(),
                Maybe::from(a.checked_add(b)),
                "{a} + {b}"
            );
            assert_eq!(
                Wide::of(a).minus(Wide::of(b)).index(),
                Maybe::from(a.checked_sub(b)),
                "{a} - {b}"
            );
        }
        for span in [1, 2, 3, 7, 8, 200, 256, limb() - 1, limb()] {
            assert_eq!(
                Wide::of(a).rem_euclid(span),
                a.rem_euclid(span),
                "{a} mod {span}"
            );
        }
    }
}

#[test]
fn past_the_index_the_two_limbs_keep_the_value() {
    // One under the bottom and one over the top are values the index does not
    // hold, and each is found by its own residues, worked by hand: `2^127` is a
    // multiple of 8, it is 128 modulo 200 and so `-2^127` is 72, and the two
    // differ by `2^128 + 1`, which is 1 modulo 8.
    let under = Wide::of(i128::MIN).minus(Wide::of(1));
    let over = Wide::of(i128::MAX).plus(Wide::of(1));
    assert_eq!(under.index(), Maybe::Isnt);
    assert_eq!(over.index(), Maybe::Isnt);
    assert!(under.is_below(Wide::of(i128::MIN)) && Wide::of(i128::MAX).is_below(over));
    assert_eq!(under.rem_euclid(8), 7);
    assert_eq!(under.rem_euclid(200), 71);
    assert_eq!(over.rem_euclid(8), 0);
    assert_eq!(over.rem_euclid(200), 128);
    assert!(under.is_negative() && !under.is_even());
    assert!(!over.is_negative() && over.is_even());
    assert_eq!(over.minus(under).index(), Maybe::Isnt);
    assert_eq!(over.minus(under).rem_euclid(8), 1);
    assert_eq!(over.minus(Wide::of(i128::MAX)).index(), Maybe::Is(1));
    assert_eq!(Wide::of(i128::MIN).minus(under).index(), Maybe::Is(1));
    assert_eq!(
        under.negated().minus(Wide::of(i128::MAX)).index(),
        Maybe::Is(2)
    );
}

#[test]
fn the_oracle_reads_each_mode_by_its_words() {
    let at = |whole: i128, num: i64| {
        Point {
            whole: Wide::of(whole),
            num,
            den: 4,
        }
    };
    let no = (0, 1);
    let cases: [(Mode, i128, i64, i128); 14] = [
        (Mode::Floor, -1, 1, -1),
        (Mode::Ceil, -1, 1, 0),
        (Mode::TowardZero, -1, 3, 0),
        (Mode::TowardZero, 2, 3, 2),
        (Mode::HalfUp, 2, 2, 3),
        (Mode::HalfUp, -3, 2, -3),
        (Mode::HalfUp, -3, 1, -3),
        (Mode::HalfUp, -3, 3, -2),
        (Mode::HalfEven, 2, 2, 2),
        (Mode::HalfEven, 3, 2, 4),
        (Mode::HalfEven, -3, 2, -2),
        (Mode::HalfEven, 3, 1, 3),
        (Mode::Floor, 5, 0, 5),
        (Mode::Ceil, 5, 0, 5),
    ];
    for (mode, whole, num, want) in cases {
        assert_eq!(
            rounded(mode, at(whole, num), no).index(),
            Maybe::Is(want),
            "{mode:?} {whole} {num}/4"
        );
    }
    // Up exactly when the dither is below the remainder, so never on the grid.
    assert_eq!(
        rounded(Mode::Stochastic, at(0, 1), (0, 8)).index(),
        Maybe::Is(1)
    );
    assert_eq!(
        rounded(Mode::Stochastic, at(0, 1), (2, 8)).index(),
        Maybe::Is(0)
    );
    assert_eq!(
        rounded(Mode::Stochastic, at(0, 1), (1, 8)).index(),
        Maybe::Is(1)
    );
    assert_eq!(
        rounded(Mode::Stochastic, at(0, 0), (0, 8)).index(),
        Maybe::Is(0)
    );
}

#[test]
fn the_oracle_completes_by_each_policy_s_words() {
    let under = Wide::of(i128::MIN).minus(Wide::of(1));
    let over = Wide::of(i128::MAX).plus(Wide::of(1));
    // `Integer<3>`, `[-4, 3]`, span 8: `-2^127 - 1` is 7 modulo 8 and `-4` is 4,
    // so the wrap lands 3 above the lowest slot; `2^127` is 0 and lands 4 above.
    assert_eq!(completed(Policy::Wrap, under, -4, 3), (-1, true));
    assert_eq!(completed(Policy::Wrap, over, -4, 3), (0, true));
    for policy in [Policy::Saturate, Policy::Clamp] {
        assert_eq!(completed(policy, under, -4, 3), (-4, true));
        assert_eq!(completed(policy, over, -4, 3), (3, true));
        assert_eq!(completed(policy, Wide::of(i128::MIN), -4, 3), (-4, true));
    }
    for policy in [Policy::Wrap, Policy::Saturate, Policy::Clamp] {
        assert_eq!(completed(policy, Wide::of(-4), -4, 3), (-4, false));
        assert_eq!(completed(policy, Wide::of(3), -4, 3), (3, false));
        assert_eq!(
            completed(policy, Wide::of(i128::MIN), i128::MIN, i128::MIN + 255),
            (i128::MIN, false)
        );
    }
}
