//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `law::the_multiplicative_inverse_round_trip`, at the widths this crate
//! reaches.
//!
//! The row states `(x * 3) * INV3 == x` under wrapping over unsigned integers,
//! with `INV3` the 64-bit inverse of three as a wrapping constant, true at every
//! width from one to 65 and false at 128. Both constants are the slots their
//! values wrap to at the width. Every member is checked at widths one to 20, and
//! past that the ends, the slots beside them and a stride of members through
//! the range. The unsigned 64-bit range at exponent zero is held out by the
//! product's slot-coordinate placeholder, and no range past 64 bits exists here,
//! so the cells at 64, 65 and 128 are not reached; the ticket
//! `arvo_law_the_multiplicative_inverse_round_trip_is_unreachabl` carries them.
//!
//! The cases that have to fail: the same round trip through zero, which is no
//! inverse at any width, has to break at every width from one up, and through
//! the inverse plus two, which is not an inverse at any width from two up, has
//! to break somewhere at each of those widths.

use super::Wr;
use crate::apply::Dither;
use crate::multiplication::{is_multipliable, mul};
use crate::points::UFixed;
use crate::slots::Slot;

/// The members a width's check reads: all of them up to 20 bits, and past that
/// the ends, their neighbours and a stride through the range.
fn members(width: u32) -> impl Iterator<Item = u128> {
    let top = (1u128 << width) - 1;
    let (every, strided) = if width <= 20 { (top + 1, 0) } else { (0, 4094) };
    let stride = (top / 4093).max(1) as usize;
    (0 .. every)
        .chain(
            // Saturating below the top, so width one keeps its two members.
            [
                0,
                1,
                2,
                3,
                top / 3,
                top / 3 + 1,
                top.saturating_sub(2),
                top.saturating_sub(1),
                top,
            ]
            .into_iter()
            .filter(move |&x| x <= top),
        )
        .chain((0 ..= top).step_by(stride).take(strided))
}

/// How many members break `(x * 3) * k == x` under wrapping at `W`, and how
/// many were checked.
fn broken<const W: u32>(k: u128) -> (u64, u64)
where
    crate::slots::Unsigned<W>: crate::slots::Slots,
{
    let mask = (1u128 << W) - 1;
    let three = Slot::at((3 & mask) as i128);
    let inverse = Slot::at((k & mask) as i128);
    let (mut wrong, mut checked) = (0u64, 0u64);
    for x in members(W) {
        let x = Slot::at(x as i128);
        let once = mul::<Wr<UFixed<W, 0>>>(x, three, Dither::UNUSED);
        let back = mul::<Wr<UFixed<W, 0>>>(once, inverse, Dither::UNUSED);
        wrong += u64::from(back != x);
        checked += 1;
    }
    (wrong, checked)
}

#[test]
fn the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63() {
    // The 64-bit multiplicative inverse of three.
    let inv3 = 0xAAAA_AAAA_AAAA_AAABu128;
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                assert!(is_multipliable::<UFixed<$w, 0>>().get(), "width {}", $w);
                let (wrong, checked) = broken::<$w>(inv3);
                assert_eq!(wrong, 0, "width {}", $w);
                assert!(checked >= 2, "width {}", $w);
                // Zero is no inverse at any width, so this breaks at `x = 1` from width one up.
                let (through_zero, _) = broken::<$w>(0);
                assert!(through_zero > 0, "zero round-tripped at width {}", $w);
                // Every odd constant is the inverse of three modulo two, so this arm starts at two.
                if $w >= 2 {
                    let (planted, _) = broken::<$w>(inv3 + 2);
                    assert!(planted > 0, "a non-inverse round-tripped at width {}", $w);
                }
            )+
        };
    }
    widths!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
    );
    // The first width the row states that this crate cannot carry.
    assert!(!is_multipliable::<UFixed<64, 0>>().get());
}
