//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm B with the consumer taking the feature after all: an array sized by a
//! `Cap` parameter. The feature line here is the arm already failing the
//! obligation; the example asks whether even that is enough.

#![feature(adt_const_params)]

use value_cap::Cap;

pub struct Seq<T, const N: Cap>([T; N.get()]);

fn main() {
    let _ = Seq::<u32, { Cap::new(2) }>([0; 2]);
}
