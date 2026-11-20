//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! A capacity constructed as a value. `Dim<N>` has no values, so this is refused.

use arvo_capacity::Dim;

fn main() {
    let _ = Dim::<4>;
}
