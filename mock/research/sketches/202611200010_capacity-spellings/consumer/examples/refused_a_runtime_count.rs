//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm A: a count that is not a constant, refused at build.

use capacity::Dim;
use capacity::sealed::Capacity;

fn sized(n: usize) -> usize {
    <Dim<n> as Capacity>::COUNT.host()
}

fn main() {
    let _ = sized(4);
}
