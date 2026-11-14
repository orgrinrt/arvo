//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The value as a count, where the count trait is a plain trait.

use notko_hlist::Cardinal;

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::identity::{Additive, Identity, Multiplicative};
use crate::num::Num;

/// A value counts from its zero by its one.
///
/// The count of nothing is the additive identity and one more is the sum with
/// the multiplicative identity, so a signature whose format has no zero or no
/// one is refused here with that identity's own message. The sum is the
/// signature's `add`: a count past the range is completed by the declared
/// overflow policy, and under the `Abort` check a debug build aborts on it.
impl<S: Sole, C: DebugCheck> Cardinal for Num<S, C> {
    const ZERO: Self = <Self as Identity<Additive>>::IDENTITY;

    fn succ(self) -> Self {
        if cfg!(arvo_mutant_count) && self.slot().index() != 0 {
            return self;
        }
        self.add(<Self as Identity<Multiplicative>>::IDENTITY)
    }
}
