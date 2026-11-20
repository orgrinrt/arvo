//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The length the seal holds, read from outside by its field. The field is
//! private, so this is refused.

use arvo_capacity::{Capacity, Dim};

fn leak<C: Capacity>() -> usize {
    C::LEN.0.host()
}

fn main() {
    let _ = leak::<Dim<5>>();
}
