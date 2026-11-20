//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The twin: the same count as a constant the build evaluates.

use arvo_capacity::{Capacity, Dim};

const N: usize = 2 * 2;

fn main() {
    let a = <Dim<N> as Capacity>::filled(0u8);
    assert_eq!(a.as_ref().len(), 4);
}
