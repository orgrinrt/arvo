//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The same number read through `count`, which builds.

use arvo_capacity::{Capacity, Dim, count};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Quiet;

fn read<C: Capacity>() -> usize {
    count::<C, Adapt<TowardZero, Wrap>, Quiet>().host()
}

fn main() {
    assert_eq!(read::<Dim<5>>(), 5);
}
