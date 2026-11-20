//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm A open: the same outside capacity against the open trait. It builds,
//! and generic code that bounds its loops by the count and indexes the array
//! reads past it. This is the trait-method route of
//! `retirement::dl_capacity_checked_at_the_one_construction_door`: no door is
//! passed, so no check at a door sees it.

use capacity::Count;
use capacity::open::Capacity;

pub struct Liar;

impl Capacity for Liar {
    type Array<T> = [T; 4];

    const COUNT: Count = Count::from_host(8);

    fn filled<T: Copy>(value: T) -> [T; 4] {
        [value; 4]
    }
}

fn last_by_count<C: Capacity>() -> Option<u32> {
    let array = C::filled(1u32);
    array.as_ref().get(C::COUNT.host() - 1).copied()
}

fn main() {
    let array_len = <Liar as Capacity>::filled(0u8).as_ref().len();
    let count = Liar::COUNT.host();
    println!(
        "BUILDS: open Liar array length={array_len} count={count}; last element by count={:?}",
        last_by_count::<Liar>()
    );
}
