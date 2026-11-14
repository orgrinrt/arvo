//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The tests' arithmetic against itself.
//!
//! `common::oracle` lands a ratio in the host's 128-bit integer where its
//! terms fit and in a wide integer where they do not. The two are different
//! arithmetic for one definition, so wherever the host's integer has room both
//! are run and must agree: every ratio of small terms under every adaptation,
//! and seeded ratios with scales at three radixes.

mod common;

use arvo_format::points::{Fixed, UFixed};
use arvo_numeral::Sole;
use common::SplitMix;
use common::oracle::{Scale, landed_both_ways};

fn agree<S: Sole>(compared: &mut u64) {
    let scales = |radix, power| {
        [Scale {
            radix,
            power,
        }]
    };
    for num in -40i128 ..= 40 {
        for den in (-9i128 ..= 9).filter(|d| *d != 0) {
            for power in -2 ..= 2 {
                let (in_room, wide) = landed_both_ways::<S>(&[num], den, &scales(3, power))
                    .expect("small terms fit the host's integer");
                assert_eq!(in_room, wide, "{num} / {den} at three to the {power}");
                *compared += 1;
            }
        }
    }
    let mut rng = SplitMix::seeded();
    for _ in 0 .. 2048 {
        let a = rng.next() as i64 as i128;
        let b = rng.next() as i64 as i128;
        let den = (rng.next() as i64 as i128) | 1;
        let radix = [2, 3, 10][(rng.next() % 3) as usize];
        let power = (rng.next() % 25) as i32 - 12;
        if let Some((in_room, wide)) = landed_both_ways::<S>(&[a, b], den, &scales(radix, power)) {
            assert_eq!(in_room, wide, "{a} * {b} / {den} at {radix} to the {power}");
            *compared += 1;
        }
    }
}

#[test]
fn the_two_ways_of_landing_a_ratio_agree_where_both_answer() {
    let mut compared = 0u64;
    every_adaptation!(Fixed<7, 0>, agree(&mut compared));
    every_adaptation!(UFixed<5, 0>, agree(&mut compared));
    every_adaptation!(Fixed<64, 0>, agree(&mut compared));
    every_adaptation!(UFixed<64, 0>, agree(&mut compared));
    // The small ratios alone, at forty signatures.
    assert!(
        compared > 40 * 81 * 18 * 5,
        "{compared} ratios were compared"
    );
}
