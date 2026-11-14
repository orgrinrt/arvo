//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The order of the members: the comparison and what reads it.

use core::cmp::Ordering;
use core::mem::size_of;

use arvo_format::Bool;

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::machine::copy;
use crate::num::{Num, ReprOf, dispatch};
use crate::rung;

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The order of the two members.
    #[must_use]
    pub const fn cmp(self, rhs: Self) -> Ordering {
        dispatch!(
            S,
            plain,
            [
                rung::cmp_s8,
                rung::cmp_u8,
                rung::cmp_s16,
                rung::cmp_u16,
                rung::cmp_s32,
                rung::cmp_u32,
                rung::cmp_s64,
                rung::cmp_u64
            ],
            [self.carrier(), rhs.carrier()],
            []
        )
    }

    /// Whether the two are the same member.
    #[must_use]
    pub const fn eq(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Equal))
    }

    /// Whether this member is below the other.
    #[must_use]
    pub const fn lt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Less))
    }

    /// Whether this member is below or at the other.
    #[must_use]
    pub const fn le(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.cmp(rhs), Ordering::Greater))
    }

    /// Whether this member is above the other.
    #[must_use]
    pub const fn gt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Greater))
    }

    /// Whether this member is above or at the other.
    #[must_use]
    pub const fn ge(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.cmp(rhs), Ordering::Less))
    }

    /// The lower of the two members.
    #[must_use]
    pub const fn min(self, rhs: Self) -> Self {
        if self.le(rhs).get() { self } else { rhs }
    }

    /// The higher of the two members.
    #[must_use]
    pub const fn max(self, rhs: Self) -> Self {
        if self.ge(rhs).get() { self } else { rhs }
    }
}
