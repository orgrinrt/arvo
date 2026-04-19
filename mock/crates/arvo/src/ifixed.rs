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
use core::ops::{Add, Div, Mul, Sub};

use crate::markers::{BitPresentation, FractionLike, IntegerLike};
use crate::newtype::{FBits, IBits, USize};
use crate::strategy::{
    Hot, IArith, IContainerFor, INarrowFrom, IWidenFrom, Precise, Strategy, Warm, ifixed_bits,
    is_fractional,
};

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

// --- Same-strategy arithmetic ---------------------------------------------
//
// Signed counterpart of the UFixed arithmetic block. Same scope and
// same deferral notes: cross-width / cross-strategy remain out of
// this round pending const-expr machinery.
//
// TODO: cross-width arithmetic blocked on generic_const_exprs max() support — next round.
// TODO: cross-strategy arithmetic blocked on const-expr support for associated-type const projection — next round.

impl<const I: IBits, const F: FBits, S: Strategy> Add for IFixed<I, F, S>
where
    S: IArith<{ ifixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_raw(<S as IArith<{ ifixed_bits(I, F) }>>::i_add(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Sub for IFixed<I, F, S>
where
    S: IArith<{ ifixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_raw(<S as IArith<{ ifixed_bits(I, F) }>>::i_sub(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Mul for IFixed<I, F, S>
where
    S: IArith<{ ifixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self::from_raw(<S as IArith<{ ifixed_bits(I, F) }>>::i_mul(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Div for IFixed<I, F, S>
where
    S: IArith<{ ifixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self::from_raw(<S as IArith<{ ifixed_bits(I, F) }>>::i_div(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

// --- Strategy conversions -------------------------------------------------

impl<const I: IBits, const F: FBits> From<IFixed<I, F, Hot>> for IFixed<I, F, Warm>
where
    Hot: IContainerFor<{ ifixed_bits(I, F) }>,
    Warm: IWidenFrom<Hot, { ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: IFixed<I, F, Hot>) -> Self {
        Self::from_raw(<Warm as IWidenFrom<Hot, { ifixed_bits(I, F) }>>::i_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> From<IFixed<I, F, Hot>> for IFixed<I, F, Precise>
where
    Hot: IContainerFor<{ ifixed_bits(I, F) }>,
    Precise: IWidenFrom<Hot, { ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: IFixed<I, F, Hot>) -> Self {
        Self::from_raw(<Precise as IWidenFrom<Hot, { ifixed_bits(I, F) }>>::i_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> From<IFixed<I, F, Warm>> for IFixed<I, F, Precise>
where
    Warm: IContainerFor<{ ifixed_bits(I, F) }>,
    Precise: IWidenFrom<Warm, { ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: IFixed<I, F, Warm>) -> Self {
        Self::from_raw(<Precise as IWidenFrom<Warm, { ifixed_bits(I, F) }>>::i_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> TryFrom<IFixed<I, F, Warm>> for IFixed<I, F, Hot>
where
    Warm: IContainerFor<{ ifixed_bits(I, F) }>,
    Hot: INarrowFrom<Warm, { ifixed_bits(I, F) }>,
{
    type Error = ();
    #[inline(always)]
    fn try_from(src: IFixed<I, F, Warm>) -> Result<Self, Self::Error> {
        <Hot as INarrowFrom<Warm, { ifixed_bits(I, F) }>>::i_try_narrow(src.to_raw())
            .map(Self::from_raw)
    }
}

impl<const I: IBits, const F: FBits> TryFrom<IFixed<I, F, Precise>> for IFixed<I, F, Hot>
where
    Precise: IContainerFor<{ ifixed_bits(I, F) }>,
    Hot: INarrowFrom<Precise, { ifixed_bits(I, F) }>,
{
    type Error = ();
    #[inline(always)]
    fn try_from(src: IFixed<I, F, Precise>) -> Result<Self, Self::Error> {
        <Hot as INarrowFrom<Precise, { ifixed_bits(I, F) }>>::i_try_narrow(src.to_raw())
            .map(Self::from_raw)
    }
}
