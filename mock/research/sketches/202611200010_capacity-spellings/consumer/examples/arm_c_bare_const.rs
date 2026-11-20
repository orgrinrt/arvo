//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm C: a bare `const N: usize` per dimension, no stack type at all.
//! One-dimensional and nested arrays size from generic code with no feature
//! line; every dimension is a parameter of its own.

pub struct Plan<const UNITS: usize, const CORES: usize, const ACCUMS: usize> {
    pub order:   [u32; UNITS],
    pub publish: [[u32; ACCUMS]; CORES],
}

fn build<const UNITS: usize, const CORES: usize, const ACCUMS: usize>() -> u32 {
    let mut plan = Plan::<UNITS, CORES, ACCUMS> {
        order:   [0; UNITS],
        publish: [[0; ACCUMS]; CORES],
    };
    let mut i = 0;
    while i < UNITS {
        plan.order[i] = i as u32;
        i += 1;
    }
    plan.publish[1][2] = 7;
    plan.order.iter().sum::<u32>() + plan.publish[1][2]
}

fn main() {
    assert_eq!(build::<4, 2, 3>(), 6 + 7);
    println!("WORKS: arm C sum={}", build::<4, 2, 3>());
}
