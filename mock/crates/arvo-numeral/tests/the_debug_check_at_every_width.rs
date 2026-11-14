//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where the `Abort` check of `from_index` aborts, at every width.
//!
//! The check asks a private verdict on the host integer, so it is shown where
//! it acts: the door aborts on exactly the host integers that name no member.
//! Every width from 1 to 64, both ranges, both policies and every mode, with
//! the 8-bit and the 64-bit host integers of each sign: every value within 16
//! of zero, both ends of the host integer and both ends of the range with
//! their neighbours. The 64-bit hosts reach the two cases a cast would lose, a
//! negative host into an unsigned range and an unsigned host above the signed
//! maximum into a signed one, and widths 1 and 64 are the ends of the shift
//! the verdict computes.

#![cfg(debug_assertions)]

mod common;

use arvo_numeral::{Abort, Num, Sole};
use common::hosts::{Value, hosts};
use common::range;

/// The door under `Abort` over the sweep's values of one host integer.
fn through<S: Sole, H: Value>(doors: &mut u64, aborted: &mut u64) {
    let (min, max) = range::<S>();
    for host in hosts::<S, H>(16) {
        let index = host.value();
        let outside = index < min || index > max;
        let aborts = std::panic::catch_unwind(move || {
            let _ = Num::<S, Abort>::from_index(host);
        })
        .is_err();
        assert_eq!(
            aborts, outside,
            "`from_index` at {index} into {min} to {max}: aborted {aborts}"
        );
        *doors += 1;
        *aborted += u64::from(aborts);
    }
}

#[derive(Default)]
struct Seen {
    doors:   u64,
    aborted: u64,
}

fn doors<S: Sole>(seen: &mut Seen) {
    through::<S, i8>(&mut seen.doors, &mut seen.aborted);
    through::<S, u8>(&mut seen.doors, &mut seen.aborted);
    through::<S, i64>(&mut seen.doors, &mut seen.aborted);
    through::<S, u64>(&mut seen.doors, &mut seen.aborted);
}

macro_rules! doors_at {
    ($w:literal, $seen:expr) => {
        every_signature_at!($w, doors, $seen)
    };
}

macro_rules! band {
    ($name:ident, $widths:ident) => {
        #[test]
        fn $name() {
            let mut seen = Seen::default();
            $widths!(doors_at!(&mut seen));
            // Each of the 320 signatures takes at least the values near zero
            // of four host integers, and some door of each aborts and some
            // does not.
            assert!(
                seen.doors >= 320 * 4 * 17,
                "the sweep took {} doors",
                seen.doors
            );
            assert!(seen.aborted >= 320 && seen.aborted < seen.doors);
        }
    };
}

band!(
    from_index_aborts_exactly_outside_the_range_at_widths_1_to_16,
    widths_1_to_16
);
band!(
    from_index_aborts_exactly_outside_the_range_at_widths_17_to_32,
    widths_17_to_32
);
band!(
    from_index_aborts_exactly_outside_the_range_at_widths_33_to_48,
    widths_33_to_48
);
band!(
    from_index_aborts_exactly_outside_the_range_at_widths_49_to_64,
    widths_49_to_64
);
