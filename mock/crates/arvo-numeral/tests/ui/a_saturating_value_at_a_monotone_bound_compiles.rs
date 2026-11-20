//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

use arvo_contracts::MonotoneSum;
use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Quiet};

fn keeps_the_order<T: MonotoneSum>(t: T) -> T {
    t
}

fn main() {
    let _ = keeps_the_order(Int::<8, Adapt<TowardZero, Saturate>, Quiet>::constant::<1>());
}
