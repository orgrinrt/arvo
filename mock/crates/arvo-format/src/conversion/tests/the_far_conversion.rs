//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Conversions whose position lies far past the slot index, admitted and adapted
//! from their own floor.

use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::Dither;
use crate::conversion::{convert, converted_position, is_convertible};
use crate::overflow::{Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::Floor;
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::grid::Grid;

type Source = Grid<BinaryRationals, Constant<0>, Signed<64>, 0, 1>;
/// Eight unsigned bits at `2^-70`, where the source's top lands at
/// `(2^63 - 1) 2^70`, past `2^127`.
type Target = Grid<BinaryRationals, Constant<-70>, Unsigned<8>, 0, 1>;

#[test]
fn the_top_of_the_signed_64_bit_range_on_a_fine_8_bit_target_wraps_to_zero() {
    assert!(is_convertible::<Source, Target>().get());
    let top = Slot::at(i128::from(i64::MAX));
    let at = converted_position::<Source, Target>(top);
    assert!(at.is_on_grid().get());
    assert_eq!(at.slot(), Slot::at(i128::MAX));
    // `(2^63 - 1) 2^70` is zero modulo 256.
    assert_eq!(
        convert::<Source, Signature<Target, Adapt<Floor, Wrap>>>(top, Dither::UNUSED),
        Slot::ZERO
    );
    // Under saturation the top pins to 255 and a member below zero to 0.
    assert_eq!(
        convert::<Source, Signature<Target, Adapt<Floor, Saturate>>>(top, Dither::UNUSED),
        Slot::at(255)
    );
    assert_eq!(
        convert::<Source, Signature<Target, Adapt<Floor, Saturate>>>(Slot::at(-1), Dither::UNUSED),
        Slot::ZERO
    );
}
