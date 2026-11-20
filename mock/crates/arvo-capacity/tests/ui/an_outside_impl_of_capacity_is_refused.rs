//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! A capacity of the consumer's own, whose array is four long and whose count
//! would be eight. The seal refuses it.

use arvo_capacity::Capacity;

struct Liar;

impl Capacity for Liar {
    type Array<T> = [T; 4];

    fn filled<T: Copy>(value: T) -> [T; 4] {
        [value; 4]
    }

    fn filled_with<T, F: FnMut() -> T>(mut f: F) -> [T; 4] {
        [f(), f(), f(), f()]
    }
}

fn main() {}
