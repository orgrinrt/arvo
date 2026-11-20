//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! A capacity sized by a number known only when the program runs. A count
//! chosen at run time is not a capacity, and the build refuses it.

use arvo_capacity::{Capacity, Dim};

fn main() {
    let n = core::hint::black_box(4usize);
    let a = <Dim<n> as Capacity>::filled(0u8);
    let _ = a;
}
