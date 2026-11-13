//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The integer points under their familiar names.

use arvo_format::overflow::Wrap;
use arvo_format::points::{self, Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

use crate::machine::copy;
use crate::num::Num;

/// An unsigned integer of `BITS` bits under the adaptation `A`.
pub type Uint<const BITS: u32, A> = Num<Signature<UFixed<BITS, 0>, A>>;

/// A signed integer of `BITS` bits under the adaptation `A`.
pub type Int<const BITS: u32, A> = Num<Signature<Integer<BITS>, A>>;

/// The unsigned integer at the target's pointer width.
pub type USize = Num<Signature<points::USize, Adapt<TowardZero, Wrap>>>;

/// The signed integer at the target's pointer width.
pub type ISize = Num<Signature<points::ISize, Adapt<TowardZero, Wrap>>>;

impl USize {
    /// The host's `usize`. Every one is a member, so this is a bit copy.
    #[must_use]
    pub const fn from_host(host: usize) -> Self {
        Self::from_raw(copy(host))
    }

    /// The host's `usize`, an unwrap door.
    #[must_use]
    pub const fn host(self) -> usize {
        copy(self.raw())
    }
}

impl ISize {
    /// The host's `isize`. Every one is a member, so this is a bit copy.
    #[must_use]
    pub const fn from_host(host: isize) -> Self {
        Self::from_raw(copy(host))
    }

    /// The host's `isize`, an unwrap door.
    #[must_use]
    pub const fn host(self) -> isize {
        copy(self.raw())
    }
}
