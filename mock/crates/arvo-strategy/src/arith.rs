//! Strategy-keyed arithmetic dispatch.
//!
//! `UArith<N>` / `IArith<N>` declare the four core arithmetic ops
//! at the `(strategy, bit-width)` key. Wrapping for Hot / Warm /
//! Cold; saturating for Precise. `USaturating` / `ISaturating`
//! provide the per-container `MAX` accessor needed inside
//! generic-context saturating impls.
//!
//! Per-strategy semantics:
//!
//! - `Hot`: wrapping (single-op convention; overflow wraps).
//! - `Warm`: wrapping on the 2x container (safe for a single op).
//! - `Cold`: saturating bound at the 2x-widened equivalent; in
//!   this L0 round we use the container's own wrapping ops since
//!   Cold and Hot share container widths. Cross-op widen-narrow
//!   logic lands with the Cold widening table in a later round.
//! - `Precise`: saturating at the container level.
//!
//! Division by zero: Hot/Warm/Cold return the numerator unchanged
//! (wrapping math has no identity for zero, so propagating `a` is
//! the cheapest defined fallback that does not panic). Precise guards
//! and clamps to the logical bound on the side the quotient heads
//! toward, which is the maximum for a non-negative numerator and the
//! minimum for a negative one.

use crate::{BitsContainerFor, HasAxes, Signed, Unsigned};

/// Unsigned arithmetic dispatch for `(strategy, N)`.
///
/// Keyed on the same `N` that `BitsContainerFor` uses. Lets
/// `UFixed<I, F, S>` delegate arithmetic to the strategy-correct
/// container operation without re-bounding on the container type.
///
/// The `HasAxes` supertrait expresses the axis-driven dispatch
/// contract introduced in Pass E of round 202604281000: each impl's
/// behavior follows from `<Self as HasAxes>::Overflow` (Wrapping vs
/// Saturating) and `<Self as HasAxes>::Width` (Min vs DoubleLogical).
/// Per-strategy impls below are keyed on the bundled marker for
/// concrete-type dispatch; the axis projections document the design
/// intent and unblock future combinations like `Hot + Saturating`
/// once Rust trait specialisation makes axis-only dispatch possible.
pub const trait UArith<const N: u16>:
    [const] BitsContainerFor<N, Unsigned> + HasAxes
{
    /// Strategy-specific `+`.
    fn u_add(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Strategy-specific `-`.
    fn u_sub(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Strategy-specific `*`.
    fn u_mul(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Strategy-specific `/`. On a zero divisor the wrapping strategies
    /// return the numerator unchanged, and `Precise` clamps to the logical
    /// bound on the side the quotient heads toward: the maximum for a
    /// non-negative numerator, the minimum for a negative one.
    fn u_div(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Fixed-point `*` for values scaled by `2^FRAC`: the product of two
    /// `FRAC`-scaled values is `2^{2*FRAC}`-scaled, so it is shifted right
    /// by `FRAC` to return to `FRAC` scale. `FRAC == 0` reduces to
    /// `u_mul`. Min-container strategies multiply in a wider container so
    /// the product does not overflow before the shift; DoubleLogical
    /// strategies already hold the full product.
    fn u_mul_fixed<const FRAC: u16>(
        // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Fixed-point `/` for values scaled by `2^FRAC`: the quotient of two
    /// `FRAC`-scaled values cancels the scale, so the numerator is shifted
    /// left by `FRAC` before the divide to return to `FRAC` scale
    /// (`(a << FRAC) / b`). `FRAC == 0` reduces to `u_div`. The numerator
    /// is widened so `a << FRAC` does not overflow before the divide.
    /// Truncates toward zero.
    fn u_div_fixed<const FRAC: u16>(
        // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
}

/// Signed arithmetic dispatch for `(strategy, N)`.
///
/// `HasAxes` supertrait per the same Pass E convention as `UArith`.
pub const trait IArith<const N: u16>: [const] BitsContainerFor<N, Signed> + HasAxes {
    /// Strategy-specific `+`.
    fn i_add(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Strategy-specific `-`.
    fn i_sub(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Strategy-specific `*`.
    fn i_mul(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Strategy-specific `/`. On a zero divisor the wrapping strategies
    /// return the numerator unchanged, and `Precise` clamps to the logical
    /// bound on the side the quotient heads toward: the maximum for a
    /// non-negative numerator, the minimum for a negative one.
    fn i_div(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Fixed-point `*` for values scaled by `2^FRAC`: see `UArith::u_mul_fixed`.
    fn i_mul_fixed<const FRAC: u16>(
        // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Fixed-point `/` for values scaled by `2^FRAC`: see `UArith::u_div_fixed`.
    fn i_div_fixed<const FRAC: u16>(
        // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
}

/// Helper trait: yields the MAX value of an unsigned container type.
///
/// Needed because generic contexts can't call `T::MAX` directly.
/// `MAX` is an inherent associated const, not routed through any
/// `num-traits` style surface (which arvo doesn't carry).
pub const trait USaturating: Sized {
    /// `T::MAX` for this container.
    fn saturating_max() -> Self;
}

/// Signed counterpart of `USaturating`.
pub const trait ISaturating: Sized {
    /// `T::MAX` for this container.
    fn saturating_max() -> Self;
}

macro_rules! impl_saturating {
    (unsigned: $($ty:ty),+) => {
        $(impl const USaturating for $ty {
            #[inline(always)]
            fn saturating_max() -> Self { <$ty>::MAX }
        })+
    };
    (signed: $($ty:ty),+) => {
        $(impl const ISaturating for $ty {
            #[inline(always)]
            fn saturating_max() -> Self { <$ty>::MAX }
        })+
    };
}

impl_saturating!(unsigned: u8, u16, u32, u64, u128);
impl_saturating!(signed: i8, i16, i32, i64, i128);

// Wrapping arithmetic for Hot / Warm / Cold. Identical op surface;
// differentiated only by the container the (strategy, N) pair
