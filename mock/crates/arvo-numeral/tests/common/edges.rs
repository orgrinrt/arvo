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
//! where that ratio's numerator and denominator fit four 64-bit limbs, the
//! integer it forms the product, the quotient and the conversion in and carries
//! a position's floor in. Each edge below is the last exponent at which that
//! holds, found from the members' own magnitudes, and the sweeps hold it against
//! the format's verdict: admitted at the edge, refused one past it.
//!
//! The file stands alone, so `room.rs`'s `bound` module takes it by path and
//! holds the value's own predicates against the same edges.

/// The widest magnitude of a range of `w` bits.
pub const fn widest(w: u32, signed: bool) -> i128 {
    if signed { 1i128 << (w - 1) } else { (1i128 << w) - 1 }
}

/// The largest `k` at least zero at which `m radix^k` is still an unsigned
/// integer four 64-bit limbs wide, for an `m` other than zero.
pub const fn four_limb_bound(m: u128, radix: i128) -> i32 {
    let mut limbs = [m as u64, (m >> 64) as u64, 0, 0];
    let mut k = 0;
    loop {
        let mut next = [0u64; 4];
        let mut carry: u128 = 0;
        let mut i = 0;
        while i < 4 {
            let t = limbs[i] as u128 * radix as u128 + carry;
            next[i] = t as u64;
            carry = t >> 64;
            i += 1;
        }
        if carry != 0 {
            return k;
        }
        limbs = next;
        k += 1;
    }
}

/// The highest exponent the product admits at a range of `w` bits.
///
/// At an exponent `e` of zero or more the product of the members at `a` and `b`
/// is at slot `a b r^e`, on the grid, and its numerator is largest in magnitude
/// at the square of the widest member, which has to fit the four limbs.
pub const fn product_highest(w: u32, signed: bool, radix: i128) -> i32 {
    let m = widest(w, signed) as u128;
    four_limb_bound(m * m, radix)
}

/// The deepest exponent the product admits: the remainder's denominator is the
/// radix to minus the exponent, four limbs wide, and nothing else bounds it, so
/// the same at every width and range.
pub const fn product_deepest(_w: u32, _signed: bool, radix: i128) -> i32 {
    -four_limb_bound(1, radix)
}

/// The highest exponent the quotient admits at a range of `w` bits: the
/// remainder's denominator is the divisor's slot times the radix to the
/// exponent, four limbs wide and at its largest at the widest member, and the
/// quotient itself only shrinks.
pub const fn quotient_highest(w: u32, signed: bool, radix: i128) -> i32 {
    four_limb_bound(widest(w, signed) as u128, radix)
}

/// The deepest exponent the quotient admits at a range of `w` bits: a quotient
/// by the slot one is the dividend's slot times the radix to minus the
/// exponent, at its largest at the widest member, and that position has to fit
/// the four limbs the format carries a position's floor in.
pub const fn quotient_deepest(w: u32, signed: bool, radix: i128) -> i32 {
    -four_limb_bound(widest(w, signed) as u128, radix)
}

/// The highest exponent a source of `w` bits converts from onto exponent zero:
/// each end of its range lands at its slot times the radix to the exponent,
/// which has to fit the four limbs the format carries a position's floor in,
/// the lowest signed member the larger in magnitude.
pub const fn source_highest(w: u32, signed: bool, radix: i128) -> i32 {
    four_limb_bound(widest(w, signed) as u128, radix)
}

/// The highest exponent a target converts onto from exponent zero: the
/// remainder's denominator is the radix to the exponent, four limbs wide, and
/// every member only shrinks, so the same at every width and range.
pub const fn target_highest(_w: u32, _signed: bool, radix: i128) -> i32 {
    four_limb_bound(1, radix)
}

/// The largest `k` at least zero at which `m radix^k` is still below `2^127`,
/// for an `m` other than zero: the bound under which the value forms a position
/// in the host's 128-bit integer rather than in the format's four limbs.
pub const fn host_bound(m: u128, radix: i128) -> i32 {
    let mut at = m;
    let mut k = 0;
    loop {
        match at.checked_mul(radix as u128) {
            Some(next) if next < 1u128 << 127 => {
                at = next;
                k += 1;
            },
            _ => return k,
        }
    }
}

/// The largest `k` at which `radix^k` is still a 128-bit unsigned integer.
pub const fn power_bound(radix: i128) -> i32 {
    let mut at = 1u128;
    let mut k = 0;
    loop {
        match at.checked_mul(radix as u128) {
            Some(next) => {
                at = next;
                k += 1;
            },
            None => return k,
        }
    }
}

/// The deepest exponent at which the value forms the quotient in the host's
/// integer, or the format's deepest where that is nearer zero: a quotient by the
/// slot one is the widest member times the radix to minus the exponent, which
/// has to stay below `2^127`. One past it the format's four limbs take over.
pub const fn quotient_room(w: u32, signed: bool, radix: i128) -> i32 {
    let room = -host_bound(widest(w, signed) as u128, radix);
    let deep = quotient_deepest(w, signed, radix);
    if room > deep { room } else { deep }
}

/// The deepest exponent at which the value forms the product in the host's
/// integer, or the format's deepest where that is nearer zero: the radix to
/// minus the exponent has to be a 128-bit integer, and the square of any member
/// up to 64 bits over it then stays below `2^127`.
pub const fn product_room(w: u32, signed: bool, radix: i128) -> i32 {
    let room = -power_bound(radix);
    let deep = product_deepest(w, signed, radix);
    if room > deep { room } else { deep }
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
