//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A signed integer below `2^256` in magnitude, on a pair of the host's unsigned
//! 128-bit integers, for the edge sweep's oracle.
//!
//! Written apart from the crate's four-limb integer on purpose: another
//! representation, a product formed from the 64-bit halves of 128-bit words, and
//! a division that differs from that integer's on each side of `2^64`, bit-serial
//! where it divides limb by limb and a search over the quotient where it works a
//! bit at a time. The two agreeing is then evidence rather than one algorithm
//! checked against itself. The product's edge sweep reads it, and so does the
//! four-limb integer's own suite. The callers
//! keep every value under `2^256`, and a value that would leave it panics rather
//! than wraps.

use notko::Maybe;

/// A sign and a magnitude `hi * 2^128 + lo`, zero never negative.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Big {
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

/// `(hi * 2^128 + lo) * d` in three 128-bit words, highest first, for `hi`
/// below `2^64`.
fn tripled(hi: u128, lo: u128, d: u128) -> (u128, u128, u128) {
    let (l1, l0) = widening(lo, d);
    let (h1, h0) = widening(hi, d);
    let (mid, carry) = l1.overflowing_add(h0);
    (h1 + u128::from(carry), mid, l0)
}

impl Big {
    /// A value of the host's signed 128-bit integer.
    pub(crate) fn of(v: i128) -> Self {
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
    pub(crate) fn is_negative(self) -> bool {
        self.negative
    }

    /// The product.
    pub(crate) fn times(self, other: Self) -> Self {
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
    pub(crate) fn power(r: i128, k: u32) -> Self {
        (0 .. k).fold(Self::of(1), |acc, _| acc.times(Self::of(r)))
    }

    /// Whether this magnitude is below the other's.
    fn is_smaller(self, other: Self) -> bool {
        (self.hi, self.lo) < (other.hi, other.lo)
    }

    /// The sum.
    pub(crate) fn plus(self, other: Self) -> Self {
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

    /// A value of the host's unsigned 128-bit integer.
    pub(crate) fn of_unsigned(v: u128) -> Self {
        Self::signed(false, 0, v)
    }

    /// The floor of the value over a positive `d`, and the remainder.
    ///
    /// Below `2^64` one bit at a time from the top, where the step's integer
    /// divides limb by limb. From `2^64` up, where the step's integer is the one
    /// working a bit at a time, the quotient is found by search instead: each bit
    /// from the top is kept where the trial quotient times `d` stays at or below
    /// the magnitude, which reads only a product and a comparison.
    pub(crate) fn floor_over(self, d: u128) -> (Self, u128) {
        assert!(d > 0, "the oracle divides by a positive divisor");
        let (q_hi, q_lo, rem) =
            if d < 1 << 64 { self.bitwise_over(d) } else { self.searched_over(d) };
        let q = Self::signed(self.negative, q_hi, q_lo);
        if self.negative && rem != 0 {
            (q.plus(Self::of(-1)), d - rem)
        } else {
            (q, rem)
        }
    }

    /// The magnitude over `d` below `2^127`, a bit at a time.
    fn bitwise_over(self, d: u128) -> (u128, u128, u128) {
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
        (q_hi, q_lo, rem)
    }

    /// The magnitude over `d` from `2^64` up, by search over the quotient's bits.
    ///
    /// The quotient is below `2^192`, and a trial times `d` is formed in three
    /// words, so nothing here can pass what it is held in.
    fn searched_over(self, d: u128) -> (u128, u128, u128) {
        let (mut q_hi, mut q_lo) = (0u128, 0u128);
        for i in (0 .. 192u32).rev() {
            let (t_hi, t_lo) =
                if i >= 128 { (q_hi | 1 << (i - 128), q_lo) } else { (q_hi, q_lo | 1 << i) };
            let (p2, p1, p0) = tripled(t_hi, t_lo, d);
            if p2 == 0 && (p1, p0) <= (self.hi, self.lo) {
                (q_hi, q_lo) = (t_hi, t_lo);
            }
        }
        let (_, p1, p0) = tripled(q_hi, q_lo, d);
        let (rem, borrow) = self.lo.overflowing_sub(p0);
        assert!(
            self.hi - p1 == u128::from(borrow),
            "the oracle's remainder is below its divisor"
        );
        (q_hi, q_lo, rem)
    }

    /// The value in the host's signed 128-bit integer, or nothing.
    pub(crate) fn index(self) -> Maybe<i128> {
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

    /// The floor of the value over a positive `d` of any size the oracle holds,
    /// and the remainder.
    ///
    /// One bit at a time from the top, shifting the running remainder up and
    /// taking `d` off it wherever it reaches `d`. A bit shifted out of the top
    /// word is kept beside it, so the remainder, below `2 d`, is never lost.
    pub(crate) fn floor_over_big(self, d: Self) -> (Self, Self) {
        assert!(
            !d.negative && (d.hi | d.lo) != 0,
            "the oracle divides by a positive divisor"
        );
        let (mut q_hi, mut q_lo) = (0u128, 0u128);
        let (mut r_hi, mut r_lo) = (0u128, 0u128);
        for i in (0 .. 256u32).rev() {
            let bit = if i >= 128 { (self.hi >> (i - 128)) & 1 } else { (self.lo >> i) & 1 };
            let over = r_hi >> 127 == 1;
            r_hi = (r_hi << 1) | (r_lo >> 127);
            r_lo = (r_lo << 1) | bit;
            if over || (r_hi, r_lo) >= (d.hi, d.lo) {
                let (lo, borrow) = r_lo.overflowing_sub(d.lo);
                r_hi = r_hi.wrapping_sub(d.hi).wrapping_sub(u128::from(borrow));
                r_lo = lo;
                if i >= 128 {
                    q_hi |= 1 << (i - 128);
                } else {
                    q_lo |= 1 << i;
                }
            }
        }
        let q = Self::signed(self.negative, q_hi, q_lo);
        let r = Self::signed(false, r_hi, r_lo);
        if self.negative && (r_hi | r_lo) != 0 {
            (q.plus(Self::of(-1)), d.plus(r.negated()))
        } else {
            (q, r)
        }
    }

    /// The value with its sign turned.
    pub(crate) fn negated(self) -> Self {
        Self::signed(!self.negative, self.hi, self.lo)
    }

    /// The greatest common divisor of two values not below zero, one above it.
    pub(crate) fn common_divisor(self, other: Self) -> Self {
        let (mut a, mut b) = (self, other);
        while (b.hi | b.lo) != 0 {
            let rest = a.floor_over_big(b).1;
            a = b;
            b = rest;
        }
        a
    }

    /// The value as four limbs, lowest first, and its sign.
    pub(crate) fn limbs(self) -> (bool, [u64; 4]) {
        (self.negative, [
            self.lo as u64,
            (self.lo >> 64) as u64,
            self.hi as u64,
            (self.hi >> 64) as u64,
        ])
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

#[test]
fn the_search_agrees_with_the_host_and_multiplies_back_from_two_to_the_64() {
    // The search reads only a product and a comparison, so where the host holds
    // the answer it is checked against the host's own division, and past it by
    // multiplying back, at the divisor's two ends and between.
    let divisors = [1u128 << 64, (1 << 64) + 1, 3 << 70, i128::MAX as u128, 1 << 127, u128::MAX];
    for a in [0i128, 1, -1, i128::MAX, i128::MIN, -(1 << 100) - 7] {
        for d in divisors {
            let (q, r) = Big::of(a).floor_over(d);
            assert!(r < d, "{a} % {d}");
            assert_eq!(
                q.times(Big::of_unsigned(d)).plus(Big::of_unsigned(r)),
                Big::of(a),
                "{a} / {d}"
            );
            if let Ok(signed) = i128::try_from(d) {
                assert!(
                    matches!(q.index(), Maybe::Is(v) if v == a.div_euclid(signed)),
                    "{a} / {d}"
                );
            }
        }
    }
    for v in [
        Big::power(2, 255),
        Big::power(3, 160).plus(Big::of(-1)),
        Big::power(10, 76).times(Big::of(-1)),
    ] {
        for d in divisors {
            let (q, r) = v.floor_over(d);
            assert!(r < d);
            assert_eq!(q.times(Big::of_unsigned(d)).plus(Big::of_unsigned(r)), v);
        }
    }
}
