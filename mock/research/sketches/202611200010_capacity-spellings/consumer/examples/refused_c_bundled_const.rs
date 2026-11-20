//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm C bundled: the dimensions as associated constants of one parameter,
//! which is what hilavitkutin's `PlanDims` needs to stop threading fifteen
//! parameters through every signature.

pub trait Dims {
    const UNITS: usize;
}

pub struct Plan<D: Dims> {
    pub order: [u32; D::UNITS],
}

pub struct Default4;

impl Dims for Default4 {
    const UNITS: usize = 4;
}

fn main() {
    let _ = Plan::<Default4> {
        order: [0; 4],
    };
}
