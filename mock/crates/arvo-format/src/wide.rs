//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A signed integer four 64-bit limbs wide, for an exact step's numerator.
//!
//! Sign and magnitude, the magnitude in four limbs, lowest first. It holds only
//! what an exact step reads: a product, a sum, a negation, a power of the radix,
//! one division with the floor by a positive divisor in the four limbs, the
//! comparisons a remainder is read by, and the narrowing back to the slot index
//! or to the host's unsigned integer. Every operation that can leave the four
//! limbs answers `Isnt` rather than wrapping, and a step reads that as a step not
//! formed. The crate's, rather than one operation's, because every exact step
//! past the sum forms its numerator, its denominator and its remainder this wide.

use core::cmp::Ordering;

use notko::Maybe;

use crate::format::greatest_common_divisor_unsigned;
use crate::width::Bool;

/// A magnitude, four limbs, lowest first. A product of two is formed in eight
/// before its top four are checked for zero.
type Magnitude = [u64; 4];

/// A signed integer below `2^256` in magnitude.
///
/// Zero is never negative, so two equal values compare equal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Wide {
    negative:  bool,
    magnitude: Magnitude,
}

/// A floor division: the quotient, and the remainder from zero up to the divisor.
pub(crate) struct Divided {
    pub(crate) quotient:  Wide,
    pub(crate) remainder: Wide,
}

impl Wide {
    /// One.
    const ONE: Self = Self::signed(false, [1, 0, 0, 0]);

    /// A value of the slot index's own integer.
    #[inline]
    pub(crate) const fn of(value: i128) -> Self {
        let m = value.unsigned_abs();
        Self::signed(value < 0, [m as u64, (m >> 64) as u64, 0, 0])
    }

    /// A value of the host's unsigned 128-bit integer, which a denominator is.
    #[inline]
    pub(crate) const fn of_unsigned(value: u128) -> Self {
        Self::signed(false, [value as u64, (value >> 64) as u64, 0, 0])
    }

    /// The value with its sign turned, which always holds.
    #[inline]
    pub(crate) const fn negated(self) -> Self {
        Self::signed(!self.negative, self.magnitude)
    }

    /// A magnitude with a sign, zero taken as nonnegative.
    #[inline]
    const fn signed(negative: bool, magnitude: Magnitude) -> Self {
        let zero = magnitude[0] == 0 && magnitude[1] == 0 && magnitude[2] == 0 && magnitude[3] == 0;
        Self {
            negative: negative && !zero,
            magnitude,
        }
    }

    /// Whether the value is below zero.
    #[inline]
    pub(crate) const fn is_negative(self) -> Bool {
        Bool::of(self.negative)
    }

    /// Whether the value is zero.
    #[inline]
    pub(crate) const fn is_zero(self) -> Bool {
        let m = self.magnitude;
        Bool::of(m[0] == 0 && m[1] == 0 && m[2] == 0 && m[3] == 0)
    }

    /// How many bits the magnitude takes, zero for zero.
    #[inline]
    pub(crate) const fn bits(self) -> u32 {
        let mut i = 4;
        while i > 0 {
            i -= 1;
            let limb = self.magnitude[i];
            if limb != 0 {
                return i as u32 * 64 + (u64::BITS - limb.leading_zeros());
            }
        }
        0
    }

    /// The product, or nothing when it leaves the four limbs.
    #[inline]
    pub(crate) const fn times(self, other: Self) -> Maybe<Self> {
        let mut out = [0u64; 8];
        let mut i = 0;
        while i < 4 {
            let mut carry: u128 = 0;
            let mut j = 0;
            while j < 4 {
                // At most (2^64 - 1)^2 + 2 * (2^64 - 1), which is 2^128 - 1.
                let t = (self.magnitude[i] as u128) * (other.magnitude[j] as u128)
                    + out[i + j] as u128
                    + carry;
                out[i + j] = t as u64;
                carry = t >> 64;
                j += 1;
            }
            out[i + 4] = carry as u64;
            i += 1;
        }
        let mut high = 4;
        while high < 8 {
            if out[high] != 0 {
                return Maybe::Isnt;
            }
            high += 1;
        }
        Maybe::Is(Self::signed(self.negative != other.negative, [
            out[0], out[1], out[2], out[3],
        ]))
    }

    /// The sum, or nothing when it leaves the four limbs.
    #[inline]
    pub(crate) const fn plus(self, other: Self) -> Maybe<Self> {
        if self.negative == other.negative {
            return match summed(self.magnitude, other.magnitude) {
                Maybe::Is(m) => Maybe::Is(Self::signed(self.negative, m)),
                Maybe::Isnt => Maybe::Isnt,
            };
        }
        if matches!(ordered(self.magnitude, other.magnitude), Ordering::Less) {
            Maybe::Is(Self::signed(
                other.negative,
                lessened(other.magnitude, self.magnitude),
            ))
        } else {
            Maybe::Is(Self::signed(
                self.negative,
                lessened(self.magnitude, other.magnitude),
            ))
        }
    }

    /// `base` to the power `exponent`, or nothing when it leaves the four limbs.
    ///
    /// By squaring, so the work is the exponent's bit count. A square is formed
    /// only while a higher bit is left to multiply it in, so a square that leaves
    /// the limbs means the power does: at a base of two or more the power is at
    /// least that square, and at zero or one no square grows.
    #[inline]
    pub(crate) const fn power(base: u64, exponent: u32) -> Maybe<Self> {
        let mut result = Self::ONE;
        let mut square = Self::signed(false, [base, 0, 0, 0]);
        let mut left = exponent;
        while left > 0 {
            if left & 1 == 1 {
                result = match result.times(square) {
                    Maybe::Is(r) => r,
                    Maybe::Isnt => return Maybe::Isnt,
                };
            }
            left >>= 1;
            if left > 0 {
                square = match square.times(square) {
                    Maybe::Is(s) => s,
                    Maybe::Isnt => return Maybe::Isnt,
                };
            }
        }
        Maybe::Is(result)
    }

    /// The floor of the value over a positive `divisor`, and the remainder.
    ///
    /// A long division of the four limbs by one where the divisor is under
    /// `2^64`, and by as many limbs as the divisor has a quotient limb at a time
    /// otherwise, then the floor: below zero with a remainder, the quotient's
    /// magnitude grows by one and the remainder is counted from the other side.
    /// That increment cannot leave the limbs, since a remainder needs a divisor of
    /// two or more and the quotient is then at most half the value. A divisor that
    /// is zero or below it is the caller's to refuse.
    #[inline]
    pub(crate) const fn floor_divided(self, divisor: Self) -> Divided {
        let v = divisor.magnitude;
        let limbs = if v[3] != 0 {
            4
        } else if v[2] != 0 {
            3
        } else if v[1] != 0 {
            2
        } else {
            1
        };
        let (quotient, rest) = if limbs == 1 {
            let (q, r) = by_one_limb(self.magnitude, v[0]);
            (q, [r as u64, (r >> 64) as u64, 0, 0])
        } else {
            by_limbs(self.magnitude, v, limbs)
        };
        let rest = Self::signed(false, rest);
        if self.negative && !rest.is_zero().get() {
            Divided {
                quotient:  Self::signed(true, incremented(quotient)),
                remainder: Self::signed(false, lessened(v, rest.magnitude)),
            }
        } else {
            Divided {
                quotient:  Self::signed(self.negative, quotient),
                remainder: rest,
            }
        }
    }

    /// The value in the slot index's own integer, or nothing when it is not one.
    #[inline]
    pub(crate) const fn index(self) -> Maybe<i128> {
        let m = self.magnitude;
        if m[2] != 0 || m[3] != 0 {
            return Maybe::Isnt;
        }
        let low = (m[0] as u128) | ((m[1] as u128) << 64);
        if !self.negative {
            return if low <= i128::MAX as u128 { Maybe::Is(low as i128) } else { Maybe::Isnt };
        }
        if low <= i128::MAX as u128 {
            Maybe::Is(-(low as i128))
        } else if low == i128::MIN.unsigned_abs() {
            Maybe::Is(i128::MIN)
        } else {
            Maybe::Isnt
        }
    }

    /// The order of two values.
    #[inline]
    pub(crate) const fn ordering(self, other: Self) -> Ordering {
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => ordered(self.magnitude, other.magnitude),
            (true, true) => ordered(other.magnitude, self.magnitude),
        }
    }

    /// `self * by` against `other * other_by`, for two values not below zero.
    ///
    /// Each product is a four-limb value times one limb, held in five, so neither
    /// side can leave what it is held in.
    #[inline]
    pub(crate) const fn scaled_against(self, by: u64, other: Self, other_by: u64) -> Ordering {
        let left = scaled(self.magnitude, by);
        let right = scaled(other.magnitude, other_by);
        let mut i = 5;
        while i > 0 {
            i -= 1;
            if left[i] != right[i] {
                return if left[i] < right[i] { Ordering::Less } else { Ordering::Greater };
            }
        }
        Ordering::Equal
    }

    /// The greatest common divisor of two values not below zero, one of them
    /// above it.
    ///
    /// Euclid's, on the host's 128-bit integer where both fit, which is every pair
    /// a 128-bit denominator leaves, and on the four limbs otherwise.
    #[inline]
    pub(crate) const fn common_divisor(self, other: Self) -> Self {
        if let (Maybe::Is(a), Maybe::Is(b)) = (self.unsigned(), other.unsigned()) {
            return Self::of_unsigned(greatest_common_divisor_unsigned(a, b));
        }
        let mut a = self;
        let mut b = other;
        while !b.is_zero().get() {
            let rest = a.floor_divided(b).remainder;
            a = b;
            b = rest;
        }
        a
    }

    /// The value as the host's unsigned 128-bit integer, or nothing when it is
    /// below zero or past `2^128 - 1`.
    #[inline]
    pub(crate) const fn unsigned(self) -> Maybe<u128> {
        let m = self.magnitude;
        if self.negative || m[2] != 0 || m[3] != 0 {
            return Maybe::Isnt;
        }
        Maybe::Is((m[0] as u128) | ((m[1] as u128) << 64))
    }
}

/// A magnitude over a divisor of one limb, limb by limb from the top.
#[inline]
const fn by_one_limb(m: Magnitude, divisor: u64) -> (Magnitude, u128) {
    let d = divisor as u128;
    let mut quotient = [0u64; 4];
    let mut rest: u128 = 0;
    let mut i = 4;
    while i > 0 {
        i -= 1;
        let current = (rest << 64) | m[i] as u128;
        quotient[i] = (current / d) as u64;
        rest = current % d;
    }
    (quotient, rest)
}

/// A magnitude over a divisor of `limbs` limbs, two to four, a limb of the
/// quotient at a time.
///
/// The schoolbook long division with the divisor normalised, Knuth's algorithm D.
/// Shifting both operands left until the divisor's top bit is set makes the
/// estimate of each quotient limb from the running remainder's top two limbs and
/// the divisor's top limb at most two too large, and the test against the
/// divisor's second limb takes it down to at most one too large. At two limbs
/// that test compares the whole product with the whole partial remainder, so the
/// estimate is then exact and the subtraction never borrows. From three limbs it
/// sees only the top of the product, so the subtraction can borrow, and then the
/// estimate was one too large: the divisor is added back and the limb lowered by
/// one. The remainder is the bottom `limbs` limbs shifted back down.
#[inline]
const fn by_limbs(m: Magnitude, divisor: Magnitude, limbs: usize) -> (Magnitude, Magnitude) {
    // Below 64, since the divisor's top limb is not zero.
    let shift = divisor[limbs - 1].leading_zeros();
    let v = shifted_up(divisor, shift);
    let mut u = [0u64; 5];
    let dividend = shifted_up(m, shift);
    u[0] = dividend[0];
    u[1] = dividend[1];
    u[2] = dividend[2];
    u[3] = dividend[3];
    if shift != 0 {
        u[4] = m[3] >> (64 - shift);
    }
    let high = v[limbs - 1] as u128;
    let second = v[limbs - 2] as u128;
    let mut quotient = [0u64; 4];
    let mut j = 5 - limbs;
    while j > 0 {
        j -= 1;
        // The running remainder `u[j + limbs] .. u[j]` is below `v` times a limb,
        // so its top limb is at most `high` and the estimate at most one limb and
        // one past it.
        let top = ((u[j + limbs] as u128) << 64) | u[j + limbs - 1] as u128;
        let mut estimate = top / high;
        let mut rest = top % high;
        while estimate >> 64 != 0 || estimate * second > ((rest << 64) | u[j + limbs - 2] as u128) {
            estimate -= 1;
            rest += high;
            if rest >> 64 != 0 {
                break;
            }
        }
        // Subtract `estimate` times `v` from the running remainder, the product
        // a limb at a time with its carry, the difference with its borrow.
        let mut carry: u128 = 0;
        let mut borrow = false;
        let mut i = 0;
        while i < limbs {
            let product = estimate * v[i] as u128 + carry;
            carry = product >> 64;
            let (d, under) = u[j + i].overflowing_sub(product as u64);
            let (d, under_again) = d.overflowing_sub(borrow as u64);
            u[j + i] = d;
            borrow = under || under_again;
            i += 1;
        }
        let (d, under) = u[j + limbs].overflowing_sub(carry as u64);
        let (d, under_again) = d.overflowing_sub(borrow as u64);
        u[j + limbs] = d;
        if under || under_again {
            // One too large: add `v` back, the carry out of the top dropped
            // because it cancels the borrow that brought us here.
            estimate -= 1;
            let mut carry_back = false;
            let mut i = 0;
            while i < limbs {
                let (s, over) = u[j + i].overflowing_add(v[i]);
                let (s, over_again) = s.overflowing_add(carry_back as u64);
                u[j + i] = s;
                carry_back = over || over_again;
                i += 1;
            }
            u[j + limbs] = u[j + limbs].wrapping_add(carry_back as u64);
        }
        quotient[j] = estimate as u64;
    }
    let mut rest = [0u64; 4];
    let mut i = 0;
    while i < limbs {
        rest[i] = if shift == 0 { u[i] } else { (u[i] >> shift) | (u[i + 1] << (64 - shift)) };
        i += 1;
    }
    (quotient, rest)
}

/// The four limbs shifted up by `shift`, below 64, with what passes the top
/// dropped; the caller keeps it where it needs it.
#[inline]
const fn shifted_up(m: Magnitude, shift: u32) -> Magnitude {
    if shift == 0 {
        return m;
    }
    [
        m[0] << shift,
        (m[1] << shift) | (m[0] >> (64 - shift)),
        (m[2] << shift) | (m[1] >> (64 - shift)),
        (m[3] << shift) | (m[2] >> (64 - shift)),
    ]
}

/// The order of magnitude `a` against `b`.
#[inline]
const fn ordered(a: Magnitude, b: Magnitude) -> Ordering {
    let mut i = 4;
    while i > 0 {
        i -= 1;
        if a[i] != b[i] {
            return if a[i] < b[i] { Ordering::Less } else { Ordering::Greater };
        }
    }
    Ordering::Equal
}

/// `m * by` in five limbs, which holds it.
#[inline]
const fn scaled(m: Magnitude, by: u64) -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut carry: u128 = 0;
    let mut i = 0;
    while i < 4 {
        // At most (2^64 - 1)^2 + (2^64 - 1), below 2^128.
        let t = (m[i] as u128) * (by as u128) + carry;
        out[i] = t as u64;
        carry = t >> 64;
        i += 1;
    }
    out[4] = carry as u64;
    out
}

/// `a + b`, or nothing when it leaves the four limbs.
#[inline]
const fn summed(a: Magnitude, b: Magnitude) -> Maybe<Magnitude> {
    let mut out = [0u64; 4];
    let mut carry = 0u128;
    let mut i = 0;
    while i < 4 {
        let t = a[i] as u128 + b[i] as u128 + carry;
        out[i] = t as u64;
        carry = t >> 64;
        i += 1;
    }
    if carry == 0 { Maybe::Is(out) } else { Maybe::Isnt }
}

/// `a - b`, where `b` is not above `a`.
#[inline]
const fn lessened(a: Magnitude, b: Magnitude) -> Magnitude {
    let mut out = [0u64; 4];
    let mut borrow = 0u64;
    let mut i = 0;
    while i < 4 {
        let (d, under) = a[i].overflowing_sub(b[i]);
        let (d, under_again) = d.overflowing_sub(borrow);
        out[i] = d;
        borrow = (under || under_again) as u64;
        i += 1;
    }
    out
}

/// `m + 1`, where `m` is below the top of the four limbs.
#[inline]
const fn incremented(m: Magnitude) -> Magnitude {
    let mut out = m;
    let mut i = 0;
    while i < 4 {
        let (s, over) = out[i].overflowing_add(1);
        out[i] = s;
        if !over {
            break;
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests;
