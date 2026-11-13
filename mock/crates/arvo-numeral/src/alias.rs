//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The integer points under their familiar names.
//!
//! Each alias takes what names its adaptation, read through `NamesAdaptation`,
//! and then the debug check, `Abort` unless named. No adaptation is a default:
//! which one `Warm` names for integers is unclaimed, so the consumer names it.
//! Reading the parameter through the projection is what lets a strategy name it
//! later, with `Warm` as its default, without any consumer written today
//! breaking.

use arvo_format::overflow::Overflow;
use arvo_format::points::{self, Integer, UFixed};
use arvo_format::rounding::Rounding;
use arvo_format::{Adapt, Adaptation, Signature};

use crate::admit::Sole;
use crate::check::{Abort, DebugCheck};
use crate::machine::copy;
use crate::num::Num;

/// A type that names an adaptation, which is what an alias's parameter is.
///
/// An adaptation names itself. A strategy, once its cell is claimed, names the
/// adaptation it selects. Open: what names an adaptation in another crate
/// implements it there.
pub trait NamesAdaptation {
    /// The adaptation named.
    type Adaptation: Adaptation;
}

impl<R: Rounding, O: Overflow> NamesAdaptation for Adapt<R, O> {
    type Adaptation = Self;
}

/// An unsigned integer of `BITS` bits under the adaptation `A` names.
pub type Uint<const BITS: u32, A, C = Abort> =
    Num<Signature<UFixed<BITS, 0>, <A as NamesAdaptation>::Adaptation>, C>;

/// A signed integer of `BITS` bits under the adaptation `A` names.
pub type Int<const BITS: u32, A, C = Abort> =
    Num<Signature<Integer<BITS>, <A as NamesAdaptation>::Adaptation>, C>;

/// The unsigned integer at the target's pointer width, under the adaptation `A`
/// names.
pub type USize<A, C = Abort> = Num<Signature<points::USize, <A as NamesAdaptation>::Adaptation>, C>;

/// The signed integer at the target's pointer width, under the adaptation `A`
/// names.
pub type ISize<A, C = Abort> = Num<Signature<points::ISize, <A as NamesAdaptation>::Adaptation>, C>;

impl<Ad: Adaptation, C: DebugCheck> Num<Signature<points::USize, Ad>, C>
where
    Signature<points::USize, Ad>: Sole,
{
    /// The host's `usize`. Every one is a member, so this is a bit copy.
    #[must_use]
    pub const fn from_host(host: usize) -> Self {
        Self::from_carrier(copy(host))
    }

    /// The host's `usize`, an unwrap door.
    #[must_use]
    pub const fn host(self) -> usize {
        copy(self.carrier())
    }
}

impl<Ad: Adaptation, C: DebugCheck> Num<Signature<points::ISize, Ad>, C>
where
    Signature<points::ISize, Ad>: Sole,
{
    /// The host's `isize`. Every one is a member, so this is a bit copy.
    #[must_use]
    pub const fn from_host(host: isize) -> Self {
        Self::from_carrier(copy(host))
    }

    /// The host's `isize`, an unwrap door.
    #[must_use]
    pub const fn host(self) -> isize {
        copy(self.carrier())
    }
}
