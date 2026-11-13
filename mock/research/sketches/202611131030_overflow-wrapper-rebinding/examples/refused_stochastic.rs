//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A control that must not build: a signature the admission refuses.
//!
//! The same MATLAB format as the admitted arm, under a rounding mode that reads
//! a dither. If this builds, "admitted" in the main arm says nothing.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::overflow::Saturate;
use arvo_format::rounding::Stochastic;
use arvo_format::standards;
use arvo_numeral::{Abort, Num};

fn main() {
    let value: Num<Signature<standards::Fi<16, 8>, Adapt<Stochastic, Saturate>>, Abort> =
        Num::constant::<3>();
    let _ = value;
}
