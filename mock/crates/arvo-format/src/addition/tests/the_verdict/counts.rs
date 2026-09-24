//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The counts the canon records, reproduced, and shown not to move with the
//! fraction width or the radix.

use super::super::Table;
use super::{Sat, Wr};
use crate::addition::addition_is_associative;
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::Dither;
use crate::format::Format;
use crate::points::{Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::Signed;
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;

#[test]
fn the_counts_the_canon_records_are_reproduced() {
    assert_eq!(
        Table::of::<Sat<Integer<4>>>(Dither::UNUSED).divergent(),
        952
    );
    assert_eq!(Table::of::<Wr<Integer<4>>>(Dither::UNUSED).divergent(), 0);
    assert_eq!(
        Table::of::<Sat<UFixed<4, 0>>>(Dither::UNUSED).divergent(),
        0
    );
    assert_eq!(Table::of::<Wr<UFixed<4, 0>>>(Dither::UNUSED).divergent(), 0);
    assert_eq!(
        Table::of::<Sat<Integer<8>>>(Dither::UNUSED).divergent(),
        4_177_792
    );

    assert!(!addition_is_associative::<Sat<Integer<4>>>().get());
    assert!(addition_is_associative::<Wr<Integer<4>>>().get());
    assert!(addition_is_associative::<Sat<UFixed<4, 0>>>().get());
    assert!(!addition_is_associative::<Sat<Integer<8>>>().get());
}

/// The count and the verdict at one format, saturating, recorded per format.
struct AtEveryScale {
    counts:   [u64; 64],
    verdicts: [bool; 64],
    formats:  usize,
}

impl AtEveryScale {
    /// Nothing recorded yet.
    fn new() -> Self {
        Self {
            counts:   [0; 64],
            verdicts: [false; 64],
            formats:  0,
        }
    }
}

impl PerFormat for AtEveryScale {
    fn run<F: Format>(&mut self) {
        self.counts[self.formats] = Table::of::<Sat<F>>(Dither::UNUSED).divergent();
        self.verdicts[self.formats] = addition_is_associative::<Sat<F>>().get();
        self.formats += 1;
    }
}

/// Every exponent from minus eight to eight, at one ambient and phase, over four
/// signed bits.
macro_rules! every_scale {
    ($walk:ident; $ambient:ty, $pn:literal, $pd:literal) => {
        every_scale!(@ $walk; $ambient, $pn, $pd;
            -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8);
    };
    (@ $walk:ident; $ambient:ty, $pn:literal, $pd:literal; $($e:literal),+) => {
        $( $walk.run::<Grid<$ambient, Constant<{ $e }>, Signed<4>, { $pn }, { $pd }>>(); )+
    };
}

#[test]
fn the_count_and_the_verdict_do_not_move_with_the_fraction_width_or_the_radix() {
    let mut whole = AtEveryScale::new();
    every_scale!(whole; BinaryRationals, 0, 1);
    every_scale!(whole; DecimalRationals, 0, 1);
    assert_eq!(whole.formats, 34);
    assert!(
        whole.counts[.. 34].iter().all(|&c| c == 952),
        "{:?}",
        &whole.counts[.. 34]
    );
    assert!(whole.verdicts[.. 34].iter().all(|&v| !v));

    // The same at a fractional phase, where the count is whatever it is at one
    // scale and has to be that at every other.
    let mut third = AtEveryScale::new();
    every_scale!(third; BinaryRationals, 1, 3);
    every_scale!(third; DecimalRationals, 1, 3);
    assert_eq!(third.formats, 34);
    let first = third.counts[0];
    // Pinned so a map that stopped diverging at a third fails here rather than
    // vacuously agreeing with itself: the same count the whole-phase cross
    // above carries, which is the fraction-width independence claim holding at
    // a fractional phase too rather than a coincidence of the whole one.
    assert_eq!(first, 952);
    assert!(
        third.counts[.. 34].iter().all(|&c| c == first),
        "{:?}",
        &third.counts[.. 34]
    );
    assert!(
        third.verdicts[.. 34]
            .iter()
            .all(|&v| v == third.verdicts[0])
    );
}
