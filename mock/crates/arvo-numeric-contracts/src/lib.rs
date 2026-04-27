#![no_std]
#![feature(adt_const_params)]
#![feature(const_param_ty_trait)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-numeric-contracts. Numeric-op trait declarations.
//!
//! `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, plus the
//! `Predicate` family (`Predicate`, `IsZero`, `IsPositive`,
//! `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`). All `pub const
//! trait`. Default-impl bodies live on the arvo facade where they
//! compose `Bits<N, S>`, `UFixed`, `IFixed`. Concrete impls bind
//! per-type per-strategy.
//!
//! See `DESIGN.md` for the full surface.

use core::cmp::Ordering;

use arvo_storage::{Bool, USize};

/// Absolute value.
///
/// `Output` is `Self` for unsigned types and for signed types whose
/// domain represents the absolute value of its full range without
/// loss. Per-impl contracts state how edge values are handled
/// (saturate vs wrap) where relevant.
pub const trait Abs {
    /// Output type of the absolute value.
    type Output;
    /// Return the absolute value of `self`.
    fn abs(self) -> Self::Output;
}

/// Multiplicative inverse `1 / x`.
///
/// Domain restrictions are per-impl. For `UFixed<I, F, S>` with
/// `F == 0` this returns the fractional dual `UFixed<0, I, S>`; for
/// `F > 0` it returns the same shape via fixed-point reciprocal.
/// Float impls follow IEEE semantics.
pub const trait Recip {
    /// Output type of the reciprocal.
    type Output;
    /// Return `1 / self`.
    fn recip(self) -> Self::Output;
}

/// Square root.
///
/// Domain is non-negative values. Signed-input impls clamp negatives
/// to zero or panic per per-impl contract. Integer impls return the
/// floor of the true square root.
pub const trait Sqrt {
    /// Output type of the square root.
    type Output;
    /// Return the square root of `self`.
    fn sqrt(self) -> Self::Output;
}

/// Total ordering.
///
/// Distinct from `core::cmp::Ord` so float-bearing arvo types can
/// implement a strict-NaN-policy total order without conflicting
/// with the partial `core::PartialOrd`. Takes operands by value
/// since every arvo numeric type is `Copy`.
pub const trait TotalOrd {
    /// Return the total ordering of `self` vs `other`.
    fn total_cmp(self, other: Self) -> Ordering;
}

/// Compile-time constant constructor.
///
/// `T::from_constant::<C>()` produces the value `C` in the target
/// numeric type. Used by macros and bitfield generators that inject
/// typed constants into expressions. The `C: USize` const generic
/// lives on the method, not the trait, so a single impl per concrete
/// numeric type covers every constant; consumer bounds stay simple
/// (`T: FromConstant`) and call sites use turbofish
/// (`T::from_constant::<{USize(5)}>()`).
///
/// For fixed-point types, `C` is placed at the integer-bit position
/// (multiplied by `2^F`), so `from_constant::<{USize(1)}>()` gives
/// one whole unit. Out-of-range constants truncate at the container.
/// This trait is compile-time only; runtime conversions belong to a
/// different surface.
pub const trait FromConstant {
    /// Construct the value `C` in the target type.
    fn from_constant<const C: USize>() -> Self;
}

/// Boolean predicate over a numeric value.
///
/// The base trait for the named-predicate family. `test(self)` runs
/// the predicate and returns a `Bool`. Named subtypes (`IsZero`,
/// `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`)
/// share this method via supertrait composition; consumer call
/// sites name the specific predicate they require.
pub const trait Predicate {
    /// Run the predicate and return its boolean result.
    fn test(self) -> Bool;
}

/// Predicate that holds when `self` is the zero value of its type.
pub const trait IsZero: Predicate {}

/// Predicate that holds when `self` is strictly greater than zero.
pub const trait IsPositive: Predicate {}

/// Predicate that holds when `self` is not the zero value.
pub const trait IsNonZero: Predicate {}

/// Predicate that holds when `self` is not strictly less than zero.
pub const trait IsNonNegative: Predicate {}

/// Predicate that holds when `self` is zero or strictly positive.
pub const trait IsZeroOrPositive: Predicate {}
