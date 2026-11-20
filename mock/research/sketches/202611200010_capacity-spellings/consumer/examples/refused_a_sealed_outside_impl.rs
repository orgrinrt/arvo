//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm A sealed: an outside capacity that reports a count its array lacks.

use capacity::Count;
use capacity::sealed::Capacity;

pub struct Liar;

impl Capacity for Liar {
    type Array<T> = [T; 4];

    const COUNT: Count = Count::from_host(8);

    fn filled<T: Copy>(value: T) -> [T; 4] {
        [value; 4]
    }
}

fn main() {
    let _ = Liar::COUNT.host();
}
