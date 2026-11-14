//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The wide integer of the suites as the step's four-limb integer.
//!
//! Kept apart from `big`, which reads nothing of this crate, so that another
//! crate's suite can take `big` as it is without reaching a private type.

use notko::Maybe;

use super::big::Big;
use crate::wide::Wide;

impl Big {
    /// The value as the step's four-limb integer, built from its two 128-bit
    /// halves by a product and a sum, so nothing of the step's division is read.
    pub(crate) fn wide(self) -> Wide {
        let (negative, [l0, l1, l2, l3]) = self.limbs();
        let lo = u128::from(l0) | (u128::from(l1) << 64);
        let hi = u128::from(l2) | (u128::from(l3) << 64);
        let Maybe::Is(shift) = Wide::power(2, 128) else { unreachable!() };
        let Maybe::Is(high) = Wide::of_unsigned(hi).times(shift) else {
            unreachable!("a magnitude under 2^256 has a high half under 2^128")
        };
        let Maybe::Is(whole) = high.plus(Wide::of_unsigned(lo)) else {
            unreachable!("a magnitude under 2^256 is formed in four limbs")
        };
        if negative { whole.negated() } else { whole }
    }
}
