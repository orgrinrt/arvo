//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A signed integer four 64-bit limbs wide, for an exact step's numerator.
//!
//! Sign and magnitude, the magnitude in four limbs, lowest first. It holds only
//! what an exact step reads: a product, a sum, a negation, a power of the radix,
//! one division with the floor by a positive divisor below `2^128`, and the
//! narrowing back to the slot index or to a denominator. Every operation that can
//! leave the four limbs answers `Isnt` rather than wrapping, and a step reads that
//! as a position past the index. The crate's, rather than one operation's, because
//! every exact step past the sum forms its numerator this wide.

use notko::Maybe;

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
    pub(crate) remainder: u128,
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
        if is_below(self.magnitude, other.magnitude).get() {
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
    /// `2^64`, and by two limbs a quotient limb at a time otherwise, then the floor: below
    /// zero with a remainder, the quotient's magnitude grows by one and the
    /// remainder is counted from the other side. That increment cannot leave the
    /// limbs, since a remainder needs a divisor of two or more and the quotient is
    /// then at most half the value. A zero divisor is the caller's to refuse.
    #[inline]
    pub(crate) const fn floor_divided(self, divisor: u128) -> Divided {
        let (quotient, rest) = if divisor >> 64 == 0 {
            by_one_limb(self.magnitude, divisor as u64)
        } else {
            by_two_limbs(self.magnitude, divisor)
        };
        if self.negative && rest != 0 {
            Divided {
                quotient:  Self::signed(true, incremented(quotient)),
                remainder: divisor - rest,
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

/// A magnitude over a divisor from `2^64` up, a limb of the quotient at a time.
///
/// The schoolbook long division with the divisor normalised, Knuth's algorithm D
/// at two limbs. Shifting both operands left until the divisor's top bit is set
/// makes the estimate of each quotient limb from the running remainder's top two
/// limbs and the divisor's top limb at most two too large, and the test against
/// the divisor's low limb then corrects it exactly: with a two-limb divisor that
/// test compares the whole product with the whole partial remainder, so the
/// multiply and subtract after it never borrows and needs no step back. The
/// remainder is the last two limbs shifted back down.
#[inline]
const fn by_two_limbs(m: Magnitude, divisor: u128) -> (Magnitude, u128) {
    // Below 64, since the divisor is at least `2^64`.
    let shift = divisor.leading_zeros();
    let d = divisor << shift;
    let high = (d >> 64) as u64 as u128;
    let low = d as u64 as u128;
    let mut u = [0u64; 5];
    if shift == 0 {
        u[0] = m[0];
        u[1] = m[1];
        u[2] = m[2];
        u[3] = m[3];
    } else {
        u[4] = m[3] >> (64 - shift);
        let mut i = 3;
        while i > 0 {
            u[i] = (m[i] << shift) | (m[i - 1] >> (64 - shift));
            i -= 1;
        }
        u[0] = m[0] << shift;
    }
    let mut quotient = [0u64; 4];
    let mut j = 3;
    while j > 0 {
        j -= 1;
        // The running remainder `u[j + 2], u[j + 1], u[j]` is below `d` times a
        // limb, so its top limb is at most `high` and the estimate at most one limb
        // and one past it.
        let top = ((u[j + 2] as u128) << 64) | u[j + 1] as u128;
        let mut estimate = top / high;
        let mut rest = top % high;
        while estimate >> 64 != 0 || estimate * low > ((rest << 64) | u[j] as u128) {
            estimate -= 1;
            rest += high;
            if rest >> 64 != 0 {
                break;
            }
        }
        // `estimate` is the quotient limb, so the product is not above the
        // running remainder and the subtraction leaves the top limb at zero.
        let p0 = estimate * low;
        let p1 = estimate * high + (p0 >> 64);
        let (r0, borrow0) = u[j].overflowing_sub(p0 as u64);
        let (t1, under1) = u[j + 1].overflowing_sub(p1 as u64);
        let (r1, under1_again) = t1.overflowing_sub(borrow0 as u64);
        u[j] = r0;
        u[j + 1] = r1;
        u[j + 2] = u[j + 2]
            .wrapping_sub((p1 >> 64) as u64)
            .wrapping_sub((under1 || under1_again) as u64);
        quotient[j] = estimate as u64;
    }
    let rest = (((u[1] as u128) << 64) | u[0] as u128) >> shift;
    (quotient, rest)
}

/// Whether magnitude `a` is below `b`.
#[inline]
const fn is_below(a: Magnitude, b: Magnitude) -> Bool {
    let mut i = 4;
    while i > 0 {
        i -= 1;
        if a[i] != b[i] {
            return Bool::of(a[i] < b[i]);
        }
    }
    Bool::FALSE
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
