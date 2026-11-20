//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The same capacity named as a type, which builds.

use arvo_capacity::{Capacity, Dim};

type Units = Dim<4>;

fn main() {
    assert_eq!(<Units as Capacity>::filled(0u8).as_ref().len(), 4);
}
