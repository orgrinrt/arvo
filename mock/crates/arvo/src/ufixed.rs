//! Unsigned fixed-point type.
//!
//! `UFixed<I, F, S>` stores a non-negative fixed-point value with
//! `I` integer bits, `F` fractional bits, and strategy `S`. The
//! backing container is selected by the strategy via the sealed
//! `UContainerFor` table in `strategy.rs`. `repr(transparent)` over
//! the container — zero overhead after compilation.
//!
//! The `Warm` strategy has no `UContainerFor` impl for logical
//! widths above 32 bits; using `UFixed<_, _, Warm>` with `I + F > 32`
//! is a compile error by design (doc CL D2).

use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

use notko::Outcome;

use crate::markers::{BitPresentation, FractionLike, IntegerLike};
use arvo_storage::{FBits, IBits, USize};
use crate::strategy::{
    Hot, Precise, Strategy, UArith, UContainerFor, UNarrowFrom, UWidenFrom, Warm, is_fractional,
    ufixed_bits,
};

/// Unsigned fixed-point value.
///
/// `I` = integer bits, `F` = fractional bits, `S` = strategy
/// (default `Warm`). Logical width is `I + F`; physical storage
/// width is determined by `S`.
#[repr(transparent)]
pub struct UFixed<const I: IBits, const F: FBits, S: Strategy = crate::strategy::Warm>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    bits: <S as UContainerFor<{ ufixed_bits(I, F) }>>::T,
    _s: PhantomData<fn() -> S>,
}

impl<const I: IBits, const F: FBits, S: Strategy> UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    /// Construct from the raw container value.
    ///
    /// The value is interpreted as `I.F` fixed-point bits. No range
    /// check is performed; the caller is responsible for keeping the
    /// value inside the logical range.
    #[inline(always)]
    pub const fn from_raw(bits: <S as UContainerFor<{ ufixed_bits(I, F) }>>::T) -> Self {
        Self { bits, _s: PhantomData }
    }

    /// Extract the raw container value.
    #[inline(always)]
    pub const fn to_raw(self) -> <S as UContainerFor<{ ufixed_bits(I, F) }>>::T {
        self.bits
    }

    /// Logical bit width (`I + F`).
    #[inline(always)]
    pub const fn logical_width() -> USize {
        USize(ufixed_bits(I, F) as usize)
    }
}

// The `UContainerFor::T` type is always `Copy` in our dispatch table
// (u8/u16/u32/u64). Hand-written Copy / Clone / PartialEq / Eq / Default
// impls delegate to whatever the container supports without adding
// bounds that retrigger const-expr evaluation cycles.

impl<const I: IBits, const F: FBits, S: Strategy> Copy for UFixed<I, F, S> where
    S: UContainerFor<{ ufixed_bits(I, F) }>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Clone for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> PartialEq for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // Container types are all fixed-width unsigned ints; byte-wise
        // equality equals value equality.
        self.bits == other.bits
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Eq for UFixed<I, F, S> where
    S: UContainerFor<{ ufixed_bits(I, F) }>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Default for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_raw(<<S as UContainerFor<{ ufixed_bits(I, F) }>>::T as Default>::default())
    }
}

// --- Marker trait impls ----------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> BitPresentation for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    const LOGICAL_WIDTH: USize = USize(I.0 as usize + F.0 as usize);
}

// IntegerLike: only when F == 0. Using the named `FBits::ZERO`
// constant because struct construction is not allowed inside an
// anonymous const-generic argument on current nightly.
impl<const I: IBits, S: Strategy> IntegerLike for UFixed<I, { FBits::ZERO }, S> where
    S: UContainerFor<{ ufixed_bits(I, FBits::ZERO) }>
{
}

// FractionLike: F > 0. Encoded via a const-expression that fails to
// evaluate when `F == FBits::ZERO` (division by zero at const time).
impl<const I: IBits, const F: FBits, S: Strategy> FractionLike for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
    [(); 1 / is_fractional(F)]:,
{
}

// --- Same-strategy arithmetic ---------------------------------------------
//
// Delegates to the strategy's `UArith` bridge. `UArith` is a
// supertrait of `UContainerFor`, so one bound pulls both in.
//
// Cross-width and cross-strategy arithmetic: DEFERRED. Computing
// `max(I1, I2)` / `max(F1, F2)` / `<S1 as Resolve<S2>>::Out` inside
// anonymous const-generic arguments runs into the same const-expr
// limits that drove the UArith bridge pattern. A blanket impl there
// requires either `feature(associated_const_equality)` stabilisation
// or a full pairwise-macro expansion (I2 * I2 * S2 * S2 matrix).
// Left for a follow-up round once the const-expr surface lands the
// necessary machinery.
//
// TODO: cross-width arithmetic blocked on generic_const_exprs max() support — next round.
// TODO: cross-strategy arithmetic blocked on const-expr support for associated-type const projection — next round.

impl<const I: IBits, const F: FBits, S: Strategy> Add for UFixed<I, F, S>
where
    S: UArith<{ ufixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_add(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Sub for UFixed<I, F, S>
where
    S: UArith<{ ufixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_sub(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Mul for UFixed<I, F, S>
where
    S: UArith<{ ufixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_mul(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Div for UFixed<I, F, S>
where
    S: UArith<{ ufixed_bits(I, F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_div(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

// --- Strategy conversions -------------------------------------------------
//
// Same `<I, F>`, different strategy. `From` for widen-free edges
// (Hot -> Warm, Hot -> Precise, Warm -> Precise). `TryFrom` for
// narrowing edges (Warm -> Hot, Precise -> Hot). Conversions use the
// `UWidenFrom` / `UNarrowFrom` bridges on the destination strategy,
// keyed on the shared `BITS = I + F`.

impl<const I: IBits, const F: FBits> From<UFixed<I, F, Hot>> for UFixed<I, F, Warm>
where
    Hot: UContainerFor<{ ufixed_bits(I, F) }>,
    Warm: UWidenFrom<Hot, { ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: UFixed<I, F, Hot>) -> Self {
        Self::from_raw(<Warm as UWidenFrom<Hot, { ufixed_bits(I, F) }>>::u_widen(
            src.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits> From<UFixed<I, F, Hot>> for UFixed<I, F, Precise>
where
    Hot: UContainerFor<{ ufixed_bits(I, F) }>,
    Precise: UWidenFrom<Hot, { ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: UFixed<I, F, Hot>) -> Self {
        Self::from_raw(<Precise as UWidenFrom<Hot, { ufixed_bits(I, F) }>>::u_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> From<UFixed<I, F, Warm>> for UFixed<I, F, Precise>
where
    Warm: UContainerFor<{ ufixed_bits(I, F) }>,
    Precise: UWidenFrom<Warm, { ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: UFixed<I, F, Warm>) -> Self {
        Self::from_raw(<Precise as UWidenFrom<Warm, { ufixed_bits(I, F) }>>::u_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> TryFrom<UFixed<I, F, Warm>> for UFixed<I, F, Hot>
where
    Warm: UContainerFor<{ ufixed_bits(I, F) }>,
    Hot: UNarrowFrom<Warm, { ufixed_bits(I, F) }>,
{
    type Error = ();
    #[inline(always)]
    fn try_from(src: UFixed<I, F, Warm>) -> Result<Self, Self::Error> { // lint:allow(no-bare-result) reason: core::convert::TryFrom::try_from trait-method signature returns Result<Self, Self::Error>; tracked: #115
        match <Hot as UNarrowFrom<Warm, { ufixed_bits(I, F) }>>::u_try_narrow(src.to_raw()) {
            Outcome::Ok(v) => Ok(Self::from_raw(v)),
            Outcome::Err(()) => Err(()),
        }
    }
}

impl<const I: IBits, const F: FBits> TryFrom<UFixed<I, F, Precise>> for UFixed<I, F, Hot>
where
    Precise: UContainerFor<{ ufixed_bits(I, F) }>,
    Hot: UNarrowFrom<Precise, { ufixed_bits(I, F) }>,
{
    type Error = ();
    #[inline(always)]
    fn try_from(src: UFixed<I, F, Precise>) -> Result<Self, Self::Error> { // lint:allow(no-bare-result) reason: core::convert::TryFrom::try_from trait-method signature returns Result<Self, Self::Error>; tracked: #115
        match <Hot as UNarrowFrom<Precise, { ufixed_bits(I, F) }>>::u_try_narrow(src.to_raw()) {
            Outcome::Ok(v) => Ok(Self::from_raw(v)),
            Outcome::Err(()) => Err(()),
        }
    }
}
