//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The long division under `Wide::floor_divided`: the magnitude over one limb,
//! and over two to four limbs by Knuth's algorithm D.

use super::Magnitude;

/// A magnitude over a divisor of one limb, limb by limb from the top.
#[inline]
pub(super) const fn by_one_limb(m: Magnitude, divisor: u64) -> (Magnitude, u128) {
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
pub(super) const fn by_limbs(
    m: Magnitude,
    divisor: Magnitude,
    limbs: usize,
) -> (Magnitude, Magnitude) {
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
