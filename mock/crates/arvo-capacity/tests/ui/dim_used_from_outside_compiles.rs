//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The twin: the same capacity of four, spelled `Dim<4>` from outside.

use arvo_capacity::{Capacity, Dim};

type Four = Dim<4>;

fn main() {
    let a = <Four as Capacity>::filled(0u8);
    assert_eq!(a.as_ref().len(), 4);
}
