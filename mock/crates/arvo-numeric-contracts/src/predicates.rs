//! Wrapper types that bind a value to a predicate semantic.
//!
//! Round 202605021800 resolves the design tension in the Predicate
//! family: a single `Predicate::test()` body per type cannot satisfy
//! five distinct marker subtraits (`IsZero` / `IsPositive` /
//! `IsNonZero` / `IsNonNegative` / `IsZeroOrPositive`) at once. The
//! resolution is the wrapper-impl pattern.
//!
//! The ord-requiring predicates (IsPositive/IsNonNegative/
//! IsZeroOrPositive) route through a unified `ConstSign` trait
//! rather than `ConstOrd` directly. Fixed types pick up `ConstSign`
//! automatically through a blanket on `ConstOrd + Identity`. Floats
//! cannot impl `ConstOrd` (NaN breaks reflexivity), so they impl
//! `ConstSign` directly in the arvo facade with bare-primitive
//! comparison against 0.0. Single dispatch path through `ConstSign`
//! avoids the orphan-rule problem that float-specific predicate-
//! wrapper impls would otherwise hit.
//!
//! Each predicate becomes a wrapper type carrying the value plus the
//! predicate semantic at the type level:
//!
//! - `IsZeroOf<T>` impls `Predicate` with body `self == ZERO` and
//!   carries the `IsZero` marker.
//! - `IsPositiveOf<T>` impls `Predicate` with body `self > ZERO` and
//!   carries the `IsPositive` marker.
//! - `IsNonZeroOf<T>` impls `Predicate` with body `self != ZERO` and
//!   carries the `IsNonZero` marker.
//! - `IsNonNegativeOf<T>` impls `Predicate` with body `self >= ZERO`
//!   and carries the `IsNonNegative` marker.
//! - `IsZeroOrPositiveOf<T>` impls `Predicate` with body `self >= ZERO`
//!   and carries the `IsZeroOrPositive` marker.
//!
//! `IsNonNegative` and `IsZeroOrPositive` share a body but the marker
//! distinction lets consumers name the semantic they care about. For
//! unsigned types the predicate is trivially true; the marker still
//! gates which algorithms accept the wrapper.
//!
//! Generic blanket impls cover every numeric type that satisfies the
//! relevant substrate predicate-bridge bound (`ConstPartialEq` /
//! `ConstOrd` plus `Identity`). Floats opt out of `ConstOrd` (NaN
//! breaks reflexivity), so ord-requiring predicates on float types
//! get explicit per-type impls in the arvo facade.
//!
//! The wrappers are `repr(transparent)` over `T` and implement
//! `Transparent` so the substrate's normalising `raw()` projection
//! works on them, keeping the Pivot 5 no-`.0`-access discipline.

use arvo_storage::{Bool, ConstOrd, ConstPartialEq};
use arvo_strategy::Identity;
use arvo_transparent::Transparent;

use crate::{IsNonNegative, IsNonZero, IsPositive, IsZero, IsZeroOrPositive, Predicate};

/// Const-callable sign predicates over a numeric type.
///
/// Provides `is_positive` / `is_non_negative` / `is_zero_or_positive`
/// for use by the wrapper-pattern Predicate impls. A blanket impl
/// covers any `T: [const] ConstOrd + [const] Identity` (every fixed
/// type). Float wrappers (`FastFloat<F>` / `StrictFloat<F>`) impl
/// `ConstSign` directly in the arvo facade since they cannot impl
/// `ConstOrd` (NaN breaks reflexivity); their bodies use bare-
/// primitive `>` / `>=` against 0.0, which IS const-callable on
/// f32 / f64.
pub const trait ConstSign {
    /// True iff `self > 0`.
    fn is_positive(self) -> Bool;
    /// True iff `self >= 0`.
    fn is_non_negative(self) -> Bool;
    /// True iff `self >= 0` (semantic alias of `is_non_negative`;
    /// kept distinct so consumer-side bound surfaces can name the
    /// semantic they care about).
    fn is_zero_or_positive(self) -> Bool;
}

impl<T> const ConstSign for T
where
    T: Copy + [const] ConstOrd + [const] Identity,
{
    #[inline(always)]
    fn is_positive(self) -> Bool {
        <T as ConstOrd>::const_gt(&self, &<T as Identity>::ZERO)
    }
    #[inline(always)]
    fn is_non_negative(self) -> Bool {
        <T as ConstOrd>::const_ge(&self, &<T as Identity>::ZERO)
    }
    #[inline(always)]
    fn is_zero_or_positive(self) -> Bool {
        <T as ConstOrd>::const_ge(&self, &<T as Identity>::ZERO)
    }
}

/// Wraps a value and tests whether it equals `<T as Identity>::ZERO`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IsZeroOf<T: Copy>(pub T);

/// Wraps a value and tests whether it is strictly greater than zero.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IsPositiveOf<T: Copy>(pub T);

/// Wraps a value and tests whether it is not equal to zero.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IsNonZeroOf<T: Copy>(pub T);

/// Wraps a value and tests whether it is not less than zero.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IsNonNegativeOf<T: Copy>(pub T);

/// Wraps a value and tests whether it is zero or strictly positive.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IsZeroOrPositiveOf<T: Copy>(pub T);

// SAFETY: All five wrappers are `repr(transparent)` over `T`. Layout
// is identical; transmute through `Transparent::raw` is sound.
unsafe impl<T: Copy> const Transparent for IsZeroOf<T> {
    type Inner = T;
}
unsafe impl<T: Copy> const Transparent for IsPositiveOf<T> {
    type Inner = T;
}
unsafe impl<T: Copy> const Transparent for IsNonZeroOf<T> {
    type Inner = T;
}
unsafe impl<T: Copy> const Transparent for IsNonNegativeOf<T> {
    type Inner = T;
}
unsafe impl<T: Copy> const Transparent for IsZeroOrPositiveOf<T> {
    type Inner = T;
}

// --- IsZeroOf: blanket Predicate + IsZero via ConstPartialEq + Identity --

impl<T> const Predicate for IsZeroOf<T>
where
    T: Copy + [const] ConstPartialEq + [const] Identity,
{
    #[inline(always)]
    fn test(self) -> Bool {
        let v = <Self as Transparent>::raw(self);
        <T as ConstPartialEq>::const_eq(&v, &<T as Identity>::ZERO)
    }
}

impl<T> const IsZero for IsZeroOf<T> where
    T: Copy + [const] ConstPartialEq + [const] Identity
{
}

// --- IsNonZeroOf: blanket Predicate + IsNonZero via ConstPartialEq -------

impl<T> const Predicate for IsNonZeroOf<T>
where
    T: Copy + [const] ConstPartialEq + [const] Identity,
{
    #[inline(always)]
    fn test(self) -> Bool {
        let v = <Self as Transparent>::raw(self);
        <T as ConstPartialEq>::const_ne(&v, &<T as Identity>::ZERO)
    }
}

impl<T> const IsNonZero for IsNonZeroOf<T> where
    T: Copy + [const] ConstPartialEq + [const] Identity
{
}

// --- IsPositiveOf: blanket Predicate + IsPositive via ConstSign ----------
//
// ConstSign auto-blanket covers fixed types via ConstOrd + Identity.
// Floats impl ConstSign directly in arvo facade since they opt out of
// ConstOrd (NaN breaks reflexivity).

impl<T> const Predicate for IsPositiveOf<T>
where
    T: Copy + [const] ConstSign,
{
    #[inline(always)]
    fn test(self) -> Bool {
        <T as ConstSign>::is_positive(<Self as Transparent>::raw(self))
    }
}

impl<T> const IsPositive for IsPositiveOf<T> where T: Copy + [const] ConstSign {}

// --- IsNonNegativeOf: blanket Predicate + IsNonNegative via ConstSign ----

impl<T> const Predicate for IsNonNegativeOf<T>
where
    T: Copy + [const] ConstSign,
{
    #[inline(always)]
    fn test(self) -> Bool {
        <T as ConstSign>::is_non_negative(<Self as Transparent>::raw(self))
    }
}

impl<T> const IsNonNegative for IsNonNegativeOf<T> where T: Copy + [const] ConstSign {}

// --- IsZeroOrPositiveOf: blanket Predicate + IsZeroOrPositive via ConstSign

impl<T> const Predicate for IsZeroOrPositiveOf<T>
where
    T: Copy + [const] ConstSign,
{
    #[inline(always)]
    fn test(self) -> Bool {
        <T as ConstSign>::is_zero_or_positive(<Self as Transparent>::raw(self))
    }
}

impl<T> const IsZeroOrPositive for IsZeroOrPositiveOf<T> where T: Copy + [const] ConstSign {}
