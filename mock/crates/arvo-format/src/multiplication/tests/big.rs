//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A signed integer below `2^256` in magnitude, on a pair of the host's unsigned
//! 128-bit integers, for the edge sweep's oracle.
//!
//! Written apart from the step's four-limb integer on purpose: another
//! representation, a product formed from the 64-bit halves of 128-bit words, and
//! a division that is bit-serial rather than limb by limb. The two agreeing is
//! then evidence rather than one algorithm checked against itself. The callers
//! keep every value under `2^256`, and a value that would leave it panics rather
//! than wraps.

use notko::Maybe;

/// A sign and a magnitude `hi * 2^128 + lo`, zero never negative.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Big {
    negative: bool,
    hi:       u128,
    lo:       u128,
}

/// The low 64 bits of a 128-bit word.
fn low(v: u128) -> u128 {
    v & (u128::MAX >> 64)
}

/// The full product of two 128-bit words, as its high and low words.
fn widening(a: u128, b: u128) -> (u128, u128) {
    let (a1, a0) = (a >> 64, low(a));
    let (b1, b0) = (b >> 64, low(b));
    let (p00, p01, p10, p11) = (a0 * b0, a0 * b1, a1 * b0, a1 * b1);
    let mid = (p00 >> 64) + low(p01) + low(p10);
    let lo = low(p00) | (mid << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}

impl Big {
    /// A value of the host's signed 128-bit integer.
    pub(super) fn of(v: i128) -> Self {
        Self::signed(v < 0, 0, v.unsigned_abs())
    }

    fn signed(negative: bool, hi: u128, lo: u128) -> Self {
        Self {
            negative: negative && (hi | lo) != 0,
            hi,
            lo,
        }
    }

    /// Whether the value is below zero.
    pub(super) fn is_negative(self) -> bool {
        self.negative
    }

    /// The product.
    pub(super) fn times(self, other: Self) -> Self {
        assert!(
            self.hi == 0 || other.hi == 0,
            "the oracle's product passed 2^256"
        );
        let (hi, lo) = widening(self.lo, other.lo);
        let cross = self
            .hi
            .checked_mul(other.lo)
            .and_then(|x| x.checked_add(self.lo.checked_mul(other.hi)?))
            .and_then(|x| x.checked_add(hi))
            .expect("the oracle's product passed 2^256");
        Self::signed(self.negative != other.negative, cross, lo)
    }

    /// `r` to the power `k`, by repeated products.
    pub(super) fn power(r: i128, k: u32) -> Self {
        (0 .. k).fold(Self::of(1), |acc, _| acc.times(Self::of(r)))
    }

    /// Whether this magnitude is below the other's.
    fn is_smaller(self, other: Self) -> bool {
        (self.hi, self.lo) < (other.hi, other.lo)
    }

    /// The sum.
    pub(super) fn plus(self, other: Self) -> Self {
        if self.negative == other.negative {
            let (lo, carry) = self.lo.overflowing_add(other.lo);
            let hi = self
                .hi
                .checked_add(other.hi)
                .and_then(|x| x.checked_add(u128::from(carry)))
                .expect("the oracle's sum passed 2^256");
            return Self::signed(self.negative, hi, lo);
        }
        let (big, small) = if self.is_smaller(other) { (other, self) } else { (self, other) };
        let (lo, borrow) = big.lo.overflowing_sub(small.lo);
        let hi = big.hi - small.hi - u128::from(borrow);
        Self::signed(big.negative, hi, lo)
    }

    /// The floor of the value over a positive `d` below `2^127`, and the
    /// remainder, one bit at a time from the top.
    pub(super) fn floor_over(self, d: u128) -> (Self, u128) {
        let (mut q_hi, mut q_lo, mut rem) = (0u128, 0u128, 0u128);
        for i in (0 .. 256u32).rev() {
            let bit = if i >= 128 { (self.hi >> (i - 128)) & 1 } else { (self.lo >> i) & 1 };
            rem = (rem << 1) | bit;
            if rem >= d {
                rem -= d;
                if i >= 128 {
                    q_hi |= 1 << (i - 128);
                } else {
                    q_lo |= 1 << i;
                }
            }
        }
        let q = Self::signed(self.negative, q_hi, q_lo);
        if self.negative && rem != 0 {
            (q.plus(Self::of(-1)), d - rem)
        } else {
            (q, rem)
        }
    }

    /// The value in the host's signed 128-bit integer, or nothing.
    pub(super) fn index(self) -> Maybe<i128> {
        if self.hi != 0 {
            return Maybe::Isnt;
        }
        match (self.negative, i128::try_from(self.lo)) {
            (false, Ok(v)) => Maybe::Is(v),
            (true, Ok(v)) => Maybe::Is(-v),
            (true, Err(_)) if self.lo == i128::MIN.unsigned_abs() => Maybe::Is(i128::MIN),
            _ => Maybe::Isnt,
        }
    }
}

#[test]
fn the_pair_agrees_with_the_host_where_the_host_holds_the_answer() {
    let values = [0i128, 1, -1, 7, -13, 1 << 64, -(1 << 63), i64::MAX as i128, 1 << 100];
    for a in values {
        for b in values {
            if let Some(p) = a.checked_mul(b) {
                assert!(
                    matches!(Big::of(a).times(Big::of(b)).index(), Maybe::Is(v) if v == p),
                    "{a} * {b}"
                );
            }
            if let Some(s) = a.checked_add(b) {
                assert!(
                    matches!(Big::of(a).plus(Big::of(b)).index(), Maybe::Is(v) if v == s),
                    "{a} + {b}"
                );
            }
        }
        for d in [1u128, 3, 10, 1 << 40, i64::MAX as u128] {
            let (q, r) = Big::of(a).floor_over(d);
            assert!(
                matches!(q.index(), Maybe::Is(v) if v == a.div_euclid(d as i128)),
                "{a} / {d}"
            );
            assert_eq!(r as i128, a.rem_euclid(d as i128), "{a} % {d}");
        }
    }
    // Past the host: `2^200` over three, multiplied back.
    let v = Big::power(2, 200);
    let (q, r) = v.floor_over(3);
    assert_eq!(q.times(Big::of(3)).plus(Big::of(r as i128)), v);
    assert!(matches!(q.index(), Maybe::Isnt));
}
