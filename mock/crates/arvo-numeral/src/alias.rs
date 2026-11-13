//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The integer and fixed-point points under their familiar names.
//!
//! Each alias takes what names its adaptation, read through `NamesAdaptation`,
//! and then the debug check, `Abort` unless named. No adaptation is a default:
//! which one `Warm` names for integers is unclaimed, so the consumer names it.
//! Reading the parameter through the projection is what lets a strategy name it
//! later, with `Warm` as its default, without any consumer written today
//! breaking.

use arvo_format::overflow::Overflow;
use arvo_format::points::{self, Fixed, Integer, UFixed};
use arvo_format::rounding::Rounding;
use arvo_format::{Adapt, Adaptation, Format, Signature};

use crate::admit::Sole;
use crate::check::{Abort, DebugCheck};
use crate::machine::copy;
use crate::num::Num;

/// A type that names an adaptation, which is what an alias's parameter is.
///
/// An adaptation names itself. A strategy, once its cell is claimed, names the
/// adaptation it selects. Open: what names an adaptation in another crate
/// implements it there.
///
/// It names one for the format `F`, since what a default names may depend on
/// the format: a preset that wraps integers and saturates fixed point is one
/// type with an impl per region, or one blanket impl where it names a single
/// adaptation throughout.
pub trait NamesAdaptation<F: Format> {
    /// The adaptation named for `F`.
    type Adaptation: Adaptation;
}

impl<F: Format, R: Rounding, O: Overflow> NamesAdaptation<F> for Adapt<R, O> {
    type Adaptation = Self;
}

/// An unsigned integer of `BITS` bits under the adaptation `A` names for it.
pub type Uint<const BITS: u32, A, C = Abort> =
    Num<Signature<UFixed<BITS, 0>, <A as NamesAdaptation<UFixed<BITS, 0>>>::Adaptation>, C>;

/// A signed integer of `BITS` bits under the adaptation `A` names for it.
pub type Int<const BITS: u32, A, C = Abort> =
    Num<Signature<Integer<BITS>, <A as NamesAdaptation<Integer<BITS>>>::Adaptation>, C>;

/// A signed fixed-point value of `BITS` bits with a quantum of two to the
/// `FRAC`, under the adaptation `A` names for it.
///
/// No rounding is a default: the consumer declares it, as
/// `Fix<32, -16, Adapt<HalfEven, Saturate>>` does. Every operation is integer
/// arithmetic on the declared width, so it gives the same bits on every target.
/// A sum under a wrapping policy is the same member in any order, and one under
/// a saturating policy over this signed range is not, so a consumer that splits
/// a sum declares its order.
pub type Fix<const BITS: u32, const FRAC: i32, A, C = Abort> =
    Num<Signature<Fixed<BITS, FRAC>, <A as NamesAdaptation<Fixed<BITS, FRAC>>>::Adaptation>, C>;

/// An unsigned fixed-point value of `BITS` bits with a quantum of two to the
/// `FRAC`, under the adaptation `A` names for it.
pub type UFix<const BITS: u32, const FRAC: i32, A, C = Abort> =
    Num<Signature<UFixed<BITS, FRAC>, <A as NamesAdaptation<UFixed<BITS, FRAC>>>::Adaptation>, C>;

/// The unsigned integer at the target's pointer width, under the adaptation `A`
/// names for it.
pub type USize<A, C = Abort> =
    Num<Signature<points::USize, <A as NamesAdaptation<points::USize>>::Adaptation>, C>;

/// The signed integer at the target's pointer width, under the adaptation `A`
/// names for it.
pub type ISize<A, C = Abort> =
    Num<Signature<points::ISize, <A as NamesAdaptation<points::ISize>>::Adaptation>, C>;

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
