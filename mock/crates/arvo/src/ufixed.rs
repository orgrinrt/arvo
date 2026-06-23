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

use core::ops::{Add, Div, Mul, Sub};

use notko::Outcome;

use crate::fixed_scale::{FracShift, frac};
use crate::markers::{BitPresentation, FractionLike, IntegerLike};
use crate::strategy::{
    ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq,
};
use arvo_storage::{Bits, Bool, FBits, IBits, USize};
use crate::strategy::{
    BitsContainerFor, Bounded, Hot, Identity, Precise, Strategy, UArith, UNarrowFrom, UWidenFrom,
    Unsigned, Warm, is_fractional, ufixed_bits,
};

/// Unsigned fixed-point value.
///
/// `I` = integer bits, `F` = fractional bits, `S` = strategy
/// (default `Warm`). Logical width is `I + F`; physical storage
/// width is determined by `S`. The wrapped `Bits<{I+F}, S>` carries
/// the storage primitive directly; `repr(transparent)` keeps the
/// layout identical to the underlying container.
#[repr(transparent)]
pub struct UFixed<const I: IBits, const F: FBits, S: Strategy = crate::strategy::Warm>(
    Bits<{ ufixed_bits(I, F) }, S>,
)
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>;

// SAFETY: `repr(transparent)` over `Bits<{I+F}, S>`. Layout-identical
// by Rust spec. The Transparent contract is what lets the canonical
// const surface (ConstPartialEq / ConstEq / ConstBitEq / ConstOrd /
// Bounded / Identity) read inner bits without resorting to `.0`
// field-access on the wrapper.
unsafe impl<const I: IBits, const F: FBits, S: Strategy> const arvo_transparent::Transparent
    for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    type Inner = Bits<{ ufixed_bits(I, F) }, S>;
}

// Generic Identity blanket on UFixed wires through the inner Bits's
// Identity blanket (which itself wires through the container's
// Identity per round 202605021600 step 4). Single predicate at the
// impl block — the inner Bits trait projection bundles the container
// requirement, sidestepping the generic_const_exprs cycle that
// previously tripped two-predicate forms.
/// The fixed-point one for `UFixed<I, F, S>`, raw `1 << F`: double the container raw 1 exactly F times
/// through the strategy's `u_add` (`F == 0` -> raw 1, the integer one). Routes only through the
/// `S: UArith<{ufixed_bits(I, F)}>` strategy bound and the inner `Bits` bound, never the container-T
/// projection, so it avoids the const-eval cycle a T-projection bound trips. A free const fn because
/// inherent impls cannot carry const trait bounds and the associated-const initializer cannot call the
/// const-trait method directly. See sketch 202606231130_fixed-point-one-construction.
const fn ufixed_fixed_one<const I: IBits, const F: FBits, S: Strategy>() -> UFixed<I, F, S>
where
    S: const UArith<{ ufixed_bits(I, F) }>,
    Bits<{ ufixed_bits(I, F) }, S>: const Identity,
{
    let mut acc = <Bits<{ ufixed_bits(I, F) }, S> as Identity>::ONE.to_raw();
    let mut doublings: u16 = 0; // lint:allow(no-bare-numeric) reason: const-loop counter for the 1<<F doubling; tracked: #256
    while doublings < F.raw() {
        acc = <S as UArith<{ ufixed_bits(I, F) }>>::u_add(acc, acc);
        doublings += 1;
    }
    UFixed::from_raw(acc)
}

impl<const I: IBits, const F: FBits, S: Strategy> const Identity for UFixed<I, F, S>
where
    S: const UArith<{ ufixed_bits(I, F) }>,
    Bits<{ ufixed_bits(I, F) }, S>: const Identity,
{
    const ZERO: Self = Self(<Bits<{ ufixed_bits(I, F) }, S> as Identity>::ZERO);
    const ONE: Self = ufixed_fixed_one::<I, F, S>();
}

// Generic Bounded blanket on UFixed wires through the inner Bits's
// Bounded blanket. Same single-predicate cycle-avoidance pattern as
// Identity. Closes Round 6 deviation 3 / NIT 5 (#325).
impl<const I: IBits, const F: FBits, S: Strategy> const Bounded for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] Bounded,
{
    const MIN: Self = Self(<Bits<{ ufixed_bits(I, F) }, S> as Bounded>::MIN);
    const MAX: Self = Self(<Bits<{ ufixed_bits(I, F) }, S> as Bounded>::MAX);
}

// ConstPartialEq / ConstEq / ConstBitEq / ConstOrd / ConstDefault
// blankets routed through the inner Bits. Same single-predicate
// cycle-avoidance pattern as Identity (each impl bounds only on the
// inner type's bridge bound, not the container projection).
impl<const I: IBits, const F: FBits, S: Strategy> const ConstPartialEq for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] ConstPartialEq,
{
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <Bits<{ ufixed_bits(I, F) }, S> as ConstPartialEq>::const_eq(&a, &b)
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const ConstEq for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] ConstEq,
{
}

impl<const I: IBits, const F: FBits, S: Strategy> const ConstBitEq for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] ConstBitEq,
{
    #[inline(always)]
    fn const_bit_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <Bits<{ ufixed_bits(I, F) }, S> as ConstBitEq>::const_bit_eq(&a, &b)
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const ConstOrd for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] ConstOrd,
{
    #[inline(always)]
    fn const_cmp(&self, other: &Self) -> ConstOrdering {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <Bits<{ ufixed_bits(I, F) }, S> as ConstOrd>::const_cmp(&a, &b)
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const ConstDefault for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S>: [const] Identity,
{
    #[inline(always)]
    fn const_default() -> Self {
        Self(<Bits<{ ufixed_bits(I, F) }, S> as Identity>::ZERO)
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    /// Construct from the raw container value.
    ///
    /// The value is interpreted as `I.F` fixed-point bits. No range
    /// check is performed; the caller is responsible for keeping the
    /// value inside the logical range.
    #[inline(always)]
    pub const fn from_raw(bits: <S as BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>>::T) -> Self {
        Self(Bits::from_raw(bits))
    }

    /// Extract the raw container value.
    #[inline(always)]
    pub const fn to_raw(self) -> <S as BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>>::T {
        self.0.to_raw()
    }

    /// Logical bit width (`I + F`).
    #[inline(always)]
    pub const fn logical_width() -> USize {
        USize(ufixed_bits(I, F) as usize)
    }
}

// Delegating Copy / Clone / PartialEq / Eq / Default to the wrapped
// `Bits<{I+F}, S>`. Bits' impls already cover the container family
// (`<S as BitsContainerFor<N, Unsigned>>::T` is always `Copy + PartialEq + Eq +
// Default`); UFixed inherits via the newtype wrap.

impl<const I: IBits, const F: FBits, S: Strategy> Copy for UFixed<I, F, S> where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Clone for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> PartialEq for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_raw() == other.to_raw()
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> Eq for UFixed<I, F, S> where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>
{
}

impl<const I: IBits, const F: FBits, S: Strategy> Default for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    #[inline(always)]
    fn default() -> Self {
        Self(Bits::default())
    }
}

// --- Marker trait impls ----------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> const BitPresentation for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    const LOGICAL_WIDTH: USize = crate::markers::logical_width_unsigned(I, F);
}

// IntegerLike: only when F == 0. Using the named `FBits::ZERO`
// constant because struct construction is not allowed inside an
// anonymous const-generic argument on current nightly.
impl<const I: IBits, S: Strategy> const IntegerLike for UFixed<I, { FBits::ZERO }, S> where
    S: BitsContainerFor<{ ufixed_bits(I, FBits::ZERO) }, Unsigned>
{
}

// FractionLike: F > 0. Encoded via a const-expression that fails to
// evaluate when `F == FBits::ZERO` (division by zero at const time).
impl<const I: IBits, const F: FBits, S: Strategy> const FractionLike for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
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

impl<const I: IBits, const F: FBits, S: Strategy> const Add for UFixed<I, F, S>
where
    S: [const] UArith<{ ufixed_bits(I, F) }>,
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

impl<const I: IBits, const F: FBits, S: Strategy> const Sub for UFixed<I, F, S>
where
    S: [const] UArith<{ ufixed_bits(I, F) }>,
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

impl<const I: IBits, const F: FBits, S: Strategy> const Mul for UFixed<I, F, S>
where
    S: [const] UArith<{ ufixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        // Fixed-point multiply: rescale by the fractional bit count F. F == 0 is integer multiply.
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_mul_fixed::<{ frac(F) }>(
            self.to_raw(),
            rhs.to_raw(),
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const Div for UFixed<I, F, S>
where
    S: [const] UArith<{ ufixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        // Fixed-point divide: rescale by the fractional bit count F. F == 0 is integer divide.
        Self::from_raw(<S as UArith<{ ufixed_bits(I, F) }>>::u_div_fixed::<{ frac(F) }>(
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
    Hot: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
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
    Hot: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Precise: UWidenFrom<Hot, { ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: UFixed<I, F, Hot>) -> Self {
        Self::from_raw(<Precise as UWidenFrom<Hot, { ufixed_bits(I, F) }>>::u_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> From<UFixed<I, F, Warm>> for UFixed<I, F, Precise>
where
    Warm: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Precise: UWidenFrom<Warm, { ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn from(src: UFixed<I, F, Warm>) -> Self {
        Self::from_raw(<Precise as UWidenFrom<Warm, { ufixed_bits(I, F) }>>::u_widen(src.to_raw()))
    }
}

impl<const I: IBits, const F: FBits> TryFrom<UFixed<I, F, Warm>> for UFixed<I, F, Hot>
where
    Warm: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
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
    Precise: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
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

impl<const I: IBits, const F: FBits, S: Strategy> core::fmt::Debug for UFixed<I, F, S>
where
    S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Bits<{ ufixed_bits(I, F) }, S, Unsigned>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UFixed").field(&self.0).finish()
    }
}
