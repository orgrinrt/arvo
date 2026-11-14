//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The last exponents the format admits each operation at, in the tests' own
//! arithmetic.
//!
//! Every format a sweep takes has a zero phase and a constant quantum, so a
//! member at slot `a` and exponent `e` is `a r^e`, and an exact position is a
//! ratio of integers over a power of the radix. The format admits an operation
//! where that ratio's denominator fits an unsigned 128-bit integer and every
//! position it can name at the ends of a range is one a signed 128-bit slot
//! index holds. Each edge below is the last exponent at which both hold, found
//! from the members' own magnitudes, and the sweeps hold it against the format's
//! verdict: admitted at the edge, refused one past it.

use super::shapes::widest;

/// The largest `k` at which `radix^k` is still an unsigned 128-bit integer.
pub const fn power_bound(radix: i128) -> i32 {
    let mut power = 1u128;
    let mut k = 0;
    while let Some(next) = power.checked_mul(radix as u128) {
        power = next;
        k += 1;
    }
    k
}

/// The largest `k` at least zero at which `m radix^k` is at most `limit`, or
/// minus one where `m` itself is past it. Zero bounds nothing, and is answered
/// with the largest `k` there is.
const fn scaled_bound(m: u128, radix: i128, limit: u128) -> i32 {
    if m > limit {
        return -1;
    }
    if m == 0 {
        return i32::MAX;
    }
    let mut at = m;
    let mut k = 0;
    while let Some(next) = at.checked_mul(radix as u128) {
        if next > limit {
            break;
        }
        at = next;
        k += 1;
    }
    k
}

/// The highest exponent the product admits at a range of `w` bits.
///
/// At an exponent `e` of zero or more the product of the members at `a` and `b`
/// is at slot `a b r^e`, on the grid, and the largest in magnitude is the
/// square of the widest member. Where that is past the slot index at zero the
/// highest is minus one, where a remainder takes the excess.
pub const fn product_highest(w: u32, signed: bool, radix: i128) -> i32 {
    let m = widest(w, signed) as u128;
    match m.checked_mul(m) {
        Some(square) => scaled_bound(square, radix, i128::MAX as u128),
        None => -1,
    }
}

/// The deepest exponent the product admits: the remainder's denominator is the
/// radix to minus the exponent, and nothing else bounds it, so the same at
/// every width and range.
pub const fn product_deepest(_w: u32, _signed: bool, radix: i128) -> i32 {
    -power_bound(radix)
}

/// The highest exponent the quotient admits at a range of `w` bits: the
/// remainder's denominator is the divisor's slot times the radix to the
/// exponent, at its largest at the widest member, and the quotient itself only
/// shrinks.
pub const fn quotient_highest(w: u32, signed: bool, radix: i128) -> i32 {
    scaled_bound(widest(w, signed) as u128, radix, u128::MAX)
}

/// The highest exponent a source of `w` bits converts from onto exponent zero:
/// each end of its range lands at its slot times the radix to the exponent,
/// which has to be a slot the index holds, the lowest signed member down to
/// minus two to the 127.
pub const fn source_highest(w: u32, signed: bool, radix: i128) -> i32 {
    if signed {
        let low = scaled_bound(1u128 << (w - 1), radix, 1u128 << 127);
        let high = scaled_bound((1u128 << (w - 1)) - 1, radix, i128::MAX as u128);
        if low < high { low } else { high }
    } else {
        scaled_bound(widest(w, false) as u128, radix, i128::MAX as u128)
    }
}

/// The highest exponent a target converts onto from exponent zero: the
/// remainder's denominator is the radix to the exponent, and every member only
/// shrinks, so the same at every width and range.
pub const fn target_highest(_w: u32, _signed: bool, radix: i128) -> i32 {
    power_bound(radix)
}

/// An operation's verdict at one exponent and the next past it, the first
/// admitted and the second refused, for one format family at one width and range.
#[macro_export]
macro_rules! edge {
    ($verdict:ident, $family:ident, $w:literal, $e:expr, $past:expr, $what:literal) => {{
        const E: i32 = $e;
        const PAST: i32 = $past;
        assert!(
            arvo_format::$verdict::<$family!($w, E)>().get(),
            "{} refused at its edge, {} bits, exponent {}",
            $what,
            $w,
            E
        );
        assert!(
            !arvo_format::$verdict::<$family!($w, PAST)>().get(),
            "{} admitted past its edge, {} bits, exponent {}",
            $what,
            $w,
            PAST
        );
    }};
}

/// The conversion's verdict on one pair of formats at its edge and on the pair
/// one past it, the first admitted and the second refused.
#[macro_export]
macro_rules! conversion_edge {
    ($from:ty => $onto:ty, past $past_from:ty => $past_onto:ty, $w:literal, $what:literal) => {{
        assert!(
            arvo_format::is_convertible::<$from, $onto>().get(),
            "the conversion refused at the {} edge, {} bits",
            $what,
            $w
        );
        assert!(
            !arvo_format::is_convertible::<$past_from, $past_onto>().get(),
            "the conversion admitted past the {} edge, {} bits",
            $what,
            $w
        );
    }};
}
