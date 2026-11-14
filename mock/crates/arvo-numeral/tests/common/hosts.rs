//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The host integers a door is swept with.

use arvo_numeral::{HostInteger, Sole};

use super::range;

/// A host integer's value, wide enough for any of the ten.
pub trait Value: HostInteger + TryFrom<i128> + std::panic::UnwindSafe {
    const MIN: Self;
    const MAX: Self;
    fn value(self) -> i128;
}

macro_rules! host_values {
    ($($t:ty),+) => {
        $(
            impl Value for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
                fn value(self) -> i128 {
                    self as i128
                }
            }
        )+
    };
}

host_values!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

/// The values of the host integer `H` a sweep takes for the signature `S`:
/// every value within `near` of zero, both ends of the host integer with their
/// neighbours, and both ends of the signature's range with theirs.
pub fn hosts<S: Sole, H: Value>(near: i128) -> Vec<H> {
    let (min, max) = range::<S>();
    let ends = [
        H::MIN.value(),
        H::MIN.value() + 1,
        H::MAX.value() - 1,
        H::MAX.value(),
        min - 1,
        min,
        min + 1,
        max - 1,
        max,
        max + 1,
    ];
    (-near ..= near)
        .chain(ends)
        .filter_map(|index| H::try_from(index).ok())
        .collect()
}
