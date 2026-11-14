//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The constant door at both ends of the range, at every admitted width of both
//! slot families.
//!
//! Each end is written as a literal expression of the width, so a door whose
//! range check stood one slot in from either end, or a ladder whose ends moved,
//! stops this file building or fails its comparison with the range the format
//! declares. One past the top and one below the bottom are refused at build time
//! and live in `tests/ui` as pairs. The adapted door is walked with the map in
//! `apply/tests/totality.rs`.

use super::Member;
use crate::format::Format;
use crate::points::{Integer, UFixed};
use crate::slots::{ADMITTED_WIDTHS, Slot, Slots};

/// The two ends a door gave and the two the format declares.
fn ends<F: Format>(low: Member<F>, high: Member<F>) -> [(Slot, Slot); 2] {
    [(low.slot(), <F::Slots as Slots>::MIN), (high.slot(), <F::Slots as Slots>::MAX)]
}

#[test]
fn the_constant_door_holds_both_ends_of_every_admitted_range() {
    let mut walked = [0u32; 65];
    macro_rules! widths {
        ($($w:literal),+ $(,)?) => {
            $(
                for (got, want) in ends::<Integer<$w>>(
                    Member::constant::<{ -(1i128 << ($w - 1)) }>(),
                    Member::constant::<{ (1i128 << ($w - 1)) - 1 }>(),
                )
                .into_iter()
                .chain(ends::<UFixed<$w, 0>>(
                    Member::constant::<0>(),
                    Member::constant::<{ (1i128 << $w) - 1 }>(),
                ))
                {
                    assert_eq!(got, want, "width {}", $w);
                }
                walked[$w] += 1;
            )+
        };
    }
    widths!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
    );
    // The literal list is checked against the set the slot ladder admits rather
    // than trusted.
    for width in ADMITTED_WIDTHS {
        assert_eq!(
            walked[width.count() as usize],
            1,
            "width {} was not walked",
            width.count()
        );
    }
    assert_eq!(walked.iter().sum::<u32>() as usize, ADMITTED_WIDTHS.len());
}
