//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An oracle for the applied map that shares none of its arithmetic.
//!
//! The map carries a position past either end of the index as a pinned slot, a
//! step and a distance, because the index's own integer is the widest it has.
//! The oracle does not: it holds the position as an integer two limbs wide, so a
//! slot eight past `i128::MAX` is an ordinary value to it. It never reads
//! `Exact`, `Rounded`, `round_slot` or `complete_slot`, which is what lets it
//! disagree with them about arithmetic.
//!
//! Each mode is computed from the formula that defines it, over the exact
//! rational `whole + num / den`, and never by choosing between the two
//! neighbours of a position the way the map does: `floor(x)`, `ceil(x)`,
//! `sign(x) floor(|x|)`, `floor(x + 1/2)`, that same value stepped down to the
//! even neighbour where `x` is a tie and it came out odd, and `ceil(x - d)` for
//! the stochastic mode at a dither `d` in `[0, 1)`, with a dither below zero read
//! as `ceil` and one at or above one read as `floor`, which is what `Dither::at`
//! documents. So every tie rule it applies is the canon's rather than the
//! implementation's. Wrapping reduces the distance from the lowest slot modulo
//! the span, and saturation pins to the end of the range on the side the rounded
//! position left by.
//!
//! One reading is the implementation's own rather than a meaning the design
//! settles, so on it the oracle is not independent of the map. `Clamp` is read as
//! `Saturate`: the policy's rustdoc says a clamp pins to a declared bound that
//! need not be the range's own end, and a declared signature carries nowhere to
//! put that bound, so `complete_slot` pins to the range and the oracle does the
//! same. The `Clamp` cells of a sweep check that the map makes that collapse, and
//! nothing about a clamp to a bound.

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

/// `floor(whole + n / d)` for a positive `d`, exactly.
///
/// `div_euclid` by a positive divisor is the floor, so the fraction's own floor
/// is added to the whole part and nothing about which neighbour is nearer is
/// asked.
fn floor_of(whole: Wide, n: i128, d: i128) -> Wide {
    assert!(d > 0, "a denominator the oracle does not divide by");
    whole.plus(Wide::of(n.div_euclid(d)))
}

/// `ceil(whole + n / d)`, as `-floor(-(whole + n / d))`.
fn ceil_of(whole: Wide, n: i128, d: i128) -> Wide {
    floor_of(whole.negated(), -n, d).negated()
}

/// The slot a mode puts the position on, from the formula that defines the mode.
///
/// The dither is a ratio over a positive denominator, read by the stochastic mode
/// alone.
pub(super) fn rounded(mode: Mode, p: Point, dither: (i64, i64)) -> Wide {
    let (whole, n, d) = (p.whole, p.num as i128, p.den as i128);
    // `x + 1/2`, over the doubled denominator.
    let (half_n, half_d) = (2 * n + d, 2 * d);
    match mode {
        Mode::Floor => floor_of(whole, n, d),
        Mode::Ceil => ceil_of(whole, n, d),
        // `sign(x) floor(|x|)`. `x` is negative exactly when its floor is.
        Mode::TowardZero => {
            if floor_of(whole, n, d).is_negative() {
                floor_of(whole.negated(), -n, d).negated()
            } else {
                floor_of(whole, n, d)
            }
        },
        Mode::HalfUp => floor_of(whole, half_n, half_d),
        // `floor(x + 1/2)`, stepped down where `x` is a tie and that came out odd.
        // A tie is where `x + 1/2` is whole, so the step lands on the even
        // neighbour, which is the lower one.
        Mode::HalfEven => {
            let r = floor_of(whole, half_n, half_d);
            let tie = half_n.rem_euclid(half_d) == 0;
            if tie && !r.is_even() { r.minus(Wide::of(1)) } else { r }
        },
        // `ceil(x - d)` for a dither in `[0, 1)`, and the two ends `Dither::at`
        // documents outside it.
        Mode::Stochastic => {
            let (dn, dd) = (dither.0 as i128, dither.1 as i128);
            assert!(dd > 0, "a dither the oracle does not read");
            if dn < 0 {
                ceil_of(whole, n, d)
            } else if dn >= dd {
                floor_of(whole, n, d)
            } else {
                ceil_of(whole, n * dd - dn * d, d * dd)
            }
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
        // `Clamp` as `Saturate`, the collapse `complete_slot` makes, taken from
        // the implementation rather than from the policy's own rustdoc.
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
fn the_oracle_reads_each_mode_as_the_canon_formula_says() {
    // Worked by hand from the formulas, in quarters. The half-up rows are the
    // ruling's own examples: a tie goes toward positive infinity at every sign,
    // so `-2.5` goes to `-2` and `-0.5` to `0`, where ties away from zero would
    // give `-3` and `-1`.
    let at = |whole: i128, num: i64| {
        Point {
            whole: Wide::of(whole),
            num,
            den: 4,
        }
    };
    let no = (0, 1);
    let cases: [(Mode, i128, i64, i128); 22] = [
        (Mode::Floor, -1, 1, -1),
        (Mode::Ceil, -1, 1, 0),
        (Mode::TowardZero, -1, 3, 0),
        (Mode::TowardZero, 2, 3, 2),
        (Mode::TowardZero, -3, 2, -2),
        (Mode::TowardZero, 0, 0, 0),
        (Mode::HalfUp, 2, 2, 3),
        (Mode::HalfUp, -3, 2, -2),
        (Mode::HalfUp, -1, 2, 0),
        (Mode::HalfUp, 0, 2, 1),
        (Mode::HalfUp, -3, 1, -3),
        (Mode::HalfUp, -3, 3, -2),
        (Mode::HalfEven, 2, 2, 2),
        (Mode::HalfEven, 3, 2, 4),
        (Mode::HalfEven, -3, 2, -2),
        (Mode::HalfEven, -2, 2, -2),
        (Mode::HalfEven, -1, 2, 0),
        (Mode::HalfEven, 3, 1, 3),
        (Mode::HalfEven, 3, 3, 4),
        (Mode::Floor, 5, 0, 5),
        (Mode::Ceil, 5, 0, 5),
        (Mode::HalfUp, 5, 0, 5),
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
    // Outside `[0, 1)` the dither is read as `Dither::at` documents it.
    assert_eq!(
        rounded(Mode::Stochastic, at(-3, 3), (-1, 8)).index(),
        Maybe::Is(-2)
    );
    assert_eq!(
        rounded(Mode::Stochastic, at(-3, 3), (8, 8)).index(),
        Maybe::Is(-3)
    );
    assert_eq!(
        rounded(Mode::Stochastic, at(-3, 0), (-1, 8)).index(),
        Maybe::Is(-3)
    );
}

#[test]
fn past_the_index_the_formulas_answer_as_they_do_inside_it() {
    // A tie a half under the index and a half over it, which the map carries as
    // a pinned slot and a distance and the oracle as an ordinary value.
    let under = Point {
        whole: Wide::of(i128::MIN).minus(Wide::of(1)),
        num:   1,
        den:   2,
    };
    let over = Point {
        whole: Wide::of(i128::MAX),
        num:   1,
        den:   2,
    };
    let no = (0, 1);
    assert_eq!(rounded(Mode::HalfUp, under, no), Wide::of(i128::MIN));
    assert_eq!(rounded(Mode::HalfEven, under, no), Wide::of(i128::MIN));
    assert_eq!(rounded(Mode::TowardZero, under, no), Wide::of(i128::MIN));
    assert_eq!(rounded(Mode::Floor, under, no), under.whole);
    assert_eq!(
        rounded(Mode::HalfUp, over, no),
        Wide::of(i128::MAX).plus(Wide::of(1))
    );
    assert_eq!(
        rounded(Mode::HalfEven, over, no),
        Wide::of(i128::MAX).plus(Wide::of(1))
    );
    assert_eq!(rounded(Mode::TowardZero, over, no), Wide::of(i128::MAX));
}

#[test]
fn the_oracle_wraps_by_the_span_and_pins_saturate_and_clamp_alike() {
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
