//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm B: a consumer declaring a `Cap` parameter of its own, as hilavitkutin's
//! `Reach<C, const N: Cap>` does, with no feature line.

use value_cap::Cap;

pub struct Reach<T, const N: Cap>(core::marker::PhantomData<T>);

fn main() {
    let _ = Reach::<u32, { Cap::new(2) }>(core::marker::PhantomData);
}
