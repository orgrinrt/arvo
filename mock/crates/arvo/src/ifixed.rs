//! Signed fixed-point type.
//!
//! Mirror of `UFixed` with signed storage. `IFixed<I, F, S>` stores
//! a signed fixed-point value with `I` integer bits, `F` fractional
//! bits, and strategy `S`; the sign bit is separate, so the logical
//! width is `1 + I + F`.
//!
//! Container is selected by the sealed `IContainerFor` table in
//! `strategy.rs`. `repr(transparent)` over the container. Signed
//! saturation (for `Precise` / widen-op) clamps to the logical range
//! `i_MIN..i_MAX` defined by `1 + I + F`, not the container range.
//!
//! `Warm` is a compile error for `1 + I + F > 32` per doc CL D2.

use core::marker::PhantomData;

use crate::markers::{BitPresentation, FractionLike, IntegerLike};
use crate::newtype::{FBits, IBits, USize};
use crate::strategy::{IContainerFor, Strategy, ifixed_bits, is_fractional};

/// Signed fixed-point value.
///
/// `I` = integer bits (magnitude), `F` = fractional bits, `S` =
/// strategy. The sign bit is implicit: logical width is `1 + I + F`.
#[repr(transparent)]
pub struct IFixed<const I: IBits, const F: FBits, S: Strategy = crate::strategy::Warm>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    bits: <S as IContainerFor<{ ifixed_bits(I, F) }>>::T,
    _s: PhantomData<fn() -> S>,
}

impl<const I: IBits, const F: FBits, S: Strategy> IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    /// Construct from the raw signed container value.
    ///
    /// Caller keeps the value inside the logical range.
    #[inline(always)]
    pub const fn from_raw(bits: <S as IContainerFor<{ ifixed_bits(I, F) }>>::T) -> Self {
        Self { bits, _s: PhantomData }
    }

    /// Extract the raw signed container value.
    #[inline(always)]
    pub const fn to_raw(self) -> <S as IContainerFor<{ ifixed_bits(I, F) }>>::T {
        self.bits
    }

    /// Logical bit width (`1 + I + F`).
    #[inline(always)]
    pub const fn logical_width() -> USize {
        USize(ifixed_bits(I, F) as usize)
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Copy for IFixed<I, F, S> where
    S: IContainerFor<{ ifixed_bits(I, F) }>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Clone for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> PartialEq for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Eq for IFixed<I, F, S> where
    S: IContainerFor<{ ifixed_bits(I, F) }>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Default for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_raw(<<S as IContainerFor<{ ifixed_bits(I, F) }>>::T as Default>::default())
    }
}

// --- Marker trait impls ----------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> BitPresentation for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    const LOGICAL_WIDTH: USize = USize(1 + I.0 as usize + F.0 as usize);
}

impl<const I: IBits, S: Strategy> IntegerLike for IFixed<I, { FBits::ZERO }, S> where
    S: IContainerFor<{ ifixed_bits(I, FBits::ZERO) }>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> FractionLike for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
    [(); 1 / is_fractional(F)]:,
{
}
