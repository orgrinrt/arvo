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
//! the cheapest defined fallback that does not panic). Precise
//! guards and clamps to container max.

use crate::{BitsContainerFor, Cold, HasAxes, Hot, Precise, Signed, Unsigned, Warm};

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
pub const trait UArith<const N: u16>: [const] BitsContainerFor<N, Unsigned> + HasAxes {
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
    /// Strategy-specific `/`. Div-by-zero: wrapping strategies use
    /// `wrapping_div` (panic convention); Precise clamps to max.
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
    fn u_mul_fixed<const FRAC: u16>( // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T;
    /// Fixed-point `/` for values scaled by `2^FRAC`: the quotient of two
    /// `FRAC`-scaled values cancels the scale, so the numerator is shifted
    /// left by `FRAC` before the divide to return to `FRAC` scale
    /// (`(a << FRAC) / b`). `FRAC == 0` reduces to `u_div`. The numerator
    /// is widened so `a << FRAC` does not overflow before the divide.
    /// Truncates toward zero.
    fn u_div_fixed<const FRAC: u16>( // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
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
    /// Strategy-specific `/`. Div-by-zero: wrapping strategies use
    /// `wrapping_div` (panic convention); Precise clamps to max.
    fn i_div(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Fixed-point `*` for values scaled by `2^FRAC`: see `UArith::u_mul_fixed`.
    fn i_mul_fixed<const FRAC: u16>( // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T;
    /// Fixed-point `/` for values scaled by `2^FRAC`: see `UArith::u_div_fixed`.
    fn i_div_fixed<const FRAC: u16>( // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier, mirrors the const N: u16 width carrier on this trait; tracked: #256
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
// already selected.
macro_rules! impl_u_arith_wrapping {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) >> FRAC }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // DoubleLogical container holds `a << FRAC` for F <= N; div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        (a << FRAC) / b
                    }
                }
            }
        )+
    };
}

// Precise saturates at the LOGICAL bound, not the container bound (design topic
// `*_topic.precise-saturates-at-container-not-logical-bound.md`, round 202606231229). Precise uses a
// DoubleLogical (2x) container, so a result that exceeds the logical `0..=(1<<N)-1` range can still fit the
// container. The contract is that Precise clamps at the logical width N, so every op computes its result in
// the wide container then clamps to the logical bound. The bound is derived at const time from the
// const-generic width `$bits` over the dispatched container type, so no `$container` threading or per-width
// regrouping is needed.
macro_rules! impl_u_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let v = a.saturating_add(b);
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.saturating_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let v = a.saturating_mul(b);
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        hi
                    } else {
                        let v = a / b;
                        if v > hi { hi } else { v }
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // DoubleLogical container holds the 2N product, so wrapping is exact; clamp to logical.
                    let v = a.wrapping_mul(b) >> FRAC;
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX. DoubleLogical holds
                    // `a << FRAC` for F <= N; clamp the quotient to the logical bound.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        hi
                    } else {
                        let v = (a << FRAC) / b;
                        if v > hi { hi } else { v }
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) >> FRAC }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // DoubleLogical container holds `a << FRAC` for F <= N; div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        (a << FRAC) / b
                    }
                }
            }
        )+
    };
}

// Signed counterpart of the Precise logical-bound clamp (see the unsigned macro). The logical bound is
// `-(1<<(N-1)) ..= (1<<(N-1))-1`, derived at const time from the const-generic width `$bits`.
macro_rules! impl_i_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_add(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_sub(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_mul(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        hi
                    } else {
                        // Guard signed overflow (MIN / -1) with `saturating_div`, then clamp to logical.
                        let v = a.saturating_div(b);
                        if v < lo { lo } else if v > hi { hi } else { v }
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // DoubleLogical container holds the 2N product; arithmetic-shift floors, then clamp.
                    let v = a.wrapping_mul(b) >> FRAC;
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX. DoubleLogical holds
                    // `a << FRAC` for F <= N; clamp the quotient to the logical bound.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        hi
                    } else {
                        let v = (a << FRAC) / b;
                        if v < lo { lo } else if v > hi { hi } else { v }
                    }
                }
            }
        )+
    };
}

// Widening fixed-point multiply for Min-container wrapping strategies (Hot / Cold) at logical widths
// 1..=64. Their container equals the logical width, so a raw product overflows before the `>> FRAC`
// rescale; the fixed-point multiply widens to the native 2x type (i128/u128, wide enough for any product
// of two <=64-bit-container values), multiplies, shifts, and narrows back to `$container`. add/sub/mul/div
// are the same wrapping ops as `impl_*_arith_wrapping`. The `$container` parameter names the concrete
// container so the widen / narrow casts resolve. Logical widths 65..=128 (i128/u128 container) stay on the
// non-widening macro: their 2x is a 256-bit WideBits, deferred until a >64-bit fixed-point multiply is
// first needed (the non-widening body is correct for FRAC == 0).
macro_rules! impl_u_arith_wrapping_widen {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let ac: $container = a;
                    let bc: $container = b;
                    let prod: u128 = (ac as u128).wrapping_mul(bc as u128); // lint:allow(no-bare-numeric) reason: native 2x widen target for the fixed-point multiply; tracked: #256
                    let narrowed: $container = (prod >> FRAC) as $container;
                    narrowed
                }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // (a << FRAC) / b widened to the native 2x so `a << FRAC` does not overflow `$container`.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        let num: u128 = (a as u128) << FRAC; // lint:allow(no-bare-numeric) reason: native 2x widen numerator for the fixed-point divide; tracked: #256
                        (num / (b as u128)) as $container // lint:allow(no-bare-numeric) reason: native 2x widen divisor for the fixed-point divide; tracked: #256
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping_widen {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let ac: $container = a;
                    let bc: $container = b;
                    let prod: i128 = (ac as i128).wrapping_mul(bc as i128); // lint:allow(no-bare-numeric) reason: native 2x widen target for the fixed-point multiply; tracked: #256
                    let narrowed: $container = (prod >> FRAC) as $container;
                    narrowed
                }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // (a << FRAC) / b widened to the native 2x; signed `/` truncates toward zero.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        let num: i128 = (a as i128) << FRAC; // lint:allow(no-bare-numeric) reason: native 2x widen numerator for the fixed-point divide; tracked: #256
                        (num / (b as i128)) as $container // lint:allow(no-bare-numeric) reason: native 2x widen divisor for the fixed-point divide; tracked: #256
                    }
                }
            }
        )+
    };
}

// 256-bit intermediate for the >64-bit-logical fixed-point widening multiply. Min-container Hot / Cold at
// logical 65..=128 use a u128 / i128 container; the product of two 128-bit values needs 256 bits before the
// `>> FRAC` rescale. These const helpers form the 256-bit product as two u128 limbs and shift it back to one
// container word. All const-evaluable on the pinned nightly.
//
// SAFETY (const_unsigned_bigint_helpers carve-out, vetted ALLOWED, tracking rust-lang/rust#152015):
// `u128::carrying_mul` is runtime-stable since 1.91; only its const use is gated. Pure arithmetic, no
// soundness hole, and core uses the bigint helpers internally. `carrying_mul(b, 0)` returns the
// `(low, high)` u128 limbs of the full 128x128 product.
//
// TODO(perf, task #4 / research 202606231500): `carrying_mul` is portable (LLVM lowers u128 mul on every
// target, native 128-bit where available, software multi-limb / `__multi3` libcall where not), so this is
// correct on all hardware with no fallback needed. It is NOT hardware-gated. The open item is a
// perf-optimal per-target path: on targets where the generic i128 lowering is suboptimal (32-bit, no
// 128-bit-mul ISA), a cfg-gated explicit limb / intrinsic route may beat it. Add cfg-gated arms per
// arvo's always-optimal-internals (Kind 1 structural lowering), bench-driven, when a target warrants it.
#[inline(always)]
const fn umul256(a: u128, b: u128) -> (u128, u128) { // lint:allow(no-bare-numeric) reason: 256-bit widen limbs for the fixed-point multiply; tracked: #256
    a.carrying_mul(b, 0) // lint:allow(no-bare-numeric) reason: carrying_mul carry-in seed; tracked: #256
}

// 256-bit logical shift-right by `frac`, returning the low u128 (the narrowed container word).
#[inline(always)]
const fn shr256_lo(lo: u128, hi: u128, frac: u32) -> u128 { // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    if frac == 0 { // lint:allow(no-bare-numeric) reason: shift-amount compare; tracked: #256
        lo
    } else if frac < 128 { // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
        (lo >> frac) | (hi << (128 - frac)) // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
    } else if frac == 128 { // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
        hi
    } else {
        hi >> (frac - 128) // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
    }
}

// Unsigned 128-bit-container fixed-point multiply: full 256-bit product, shift, narrow.
#[inline(always)]
const fn u_mul_fixed_128(a: u128, b: u128, frac: u32) -> u128 { // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    let (lo, hi) = umul256(a, b);
    shr256_lo(lo, hi, frac)
}

// Signed 128-bit-container fixed-point multiply: magnitude product, arithmetic-shift floor toward minus
// infinity (subtract 1 when the result is negative and any shifted-out low bit was set), reapply sign. This
// matches the `>> FRAC` floor the 1..=64 native-widen path and the catalogue assertions use.
#[inline(always)]
const fn i_mul_fixed_128(a: i128, b: i128, frac: u32) -> i128 { // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    let neg = (a < 0) != (b < 0); // lint:allow(no-bare-numeric) reason: sign test; tracked: #256
    let (lo, hi) = umul256(a.unsigned_abs(), b.unsigned_abs());
    let mag = shr256_lo(lo, hi, frac);
    if !neg {
        mag as i128 // lint:allow(no-bare-numeric) reason: magnitude back to signed container; tracked: #256
    } else {
        let dropped = if frac == 0 { // lint:allow(no-bare-numeric) reason: shift-amount compare; tracked: #256
            false
        } else if frac < 128 { // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
            (lo & ((1u128 << frac) - 1)) != 0 // lint:allow(no-bare-numeric) reason: low-bit drop mask; tracked: #256
        } else if frac == 128 { // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
            lo != 0 // lint:allow(no-bare-numeric) reason: low-limb drop test; tracked: #256
        } else {
            lo != 0 || (hi & ((1u128 << (frac - 128)) - 1)) != 0 // lint:allow(no-bare-numeric) reason: drop mask above 128; tracked: #256
        };
        let m = mag as i128; // lint:allow(no-bare-numeric) reason: magnitude back to signed container; tracked: #256
        if dropped { -m - 1 } else { -m } // lint:allow(no-bare-numeric) reason: floor correction; tracked: #256
    }
}

// Widening fixed-point multiply for Min-container Hot / Cold at logical 65..=128 (u128 / i128 container).
// add / sub / mul / div are the same wrapping ops as `impl_*_arith_wrapping`; only `*_mul_fixed` routes
// through the 256-bit helper, because at a 128-bit container the 2x intermediate is 256 bits. The 1..=64
// path stays on `impl_*_arith_wrapping_widen` (native i128 / u128 intermediate).
macro_rules! impl_u_arith_wrapping_widen256 {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { u_mul_fixed_128(a, b, FRAC as u32) } // lint:allow(no-bare-numeric) reason: FRAC widened to the helper shift-amount type; tracked: #256
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // CATALOGUE (tracked, task #5): correct only when `a << FRAC` fits the u128 container; the
                    // 65..=128 case needs 256/128 long division (no `carrying_div` intrinsic). The ignored
                    // catalogue test pins the target. div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity>::ZERO {
                        a
                    } else {
                        (a << FRAC) / b
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping_widen256 {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { i_mul_fixed_128(a, b, FRAC as u32) } // lint:allow(no-bare-numeric) reason: FRAC widened to the helper shift-amount type; tracked: #256
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // CATALOGUE (tracked, task #5): correct only when `a << FRAC` fits the i128 container; the
                    // 65..=128 case needs 256/128 long division. The ignored catalogue test pins the target.
                    // div-by-zero returns the numerator; signed `/` truncates toward zero.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity>::ZERO {
                        a
                    } else {
                        (a << FRAC) / b
                    }
                }
            }
        )+
    };
}

// Wrapping strategies: Hot / Warm / Cold.
impl_u_arith_wrapping_widen!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping_widen!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping_widen!(
    Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping_widen!(
    Hot, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Hot 65..=128: u128 container; add/sub/mul/div wrap, the fixed-point multiply takes the 256-bit widen
// path (round 202606231229). The 1..=64 Hot path above uses the native i128/u128-intermediate widen.
#[rustfmt::skip]
impl_u_arith_wrapping_widen256!(
    Hot,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_u_arith_wrapping_widen!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping_widen!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping_widen!(
    Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping_widen!(
    Cold, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Cold 65..=128: u128 container; fixed-point multiply takes the 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_u_arith_wrapping_widen256!(
    Cold,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_u_arith_wrapping!(Warm, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping!(Warm, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping!(
    Warm, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
// Round 202604280500: Warm 33..=64 wrapping (u128 carrier).
#[rustfmt::skip]
impl_u_arith_wrapping!(
    Warm,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_u_arith_saturating!(Precise, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_saturating!(Precise, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_saturating!(
    Precise, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_saturating!(
    Precise,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Signed.
impl_i_arith_wrapping_widen!(Hot, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping_widen!(Hot, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping_widen!(
    Hot, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping_widen!(
    Hot, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Hot 65..=128: i128 container; fixed-point multiply takes the signed 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_i_arith_wrapping_widen256!(
    Hot,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_i_arith_wrapping_widen!(Cold, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping_widen!(Cold, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping_widen!(
    Cold, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping_widen!(
    Cold, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Cold 65..=128: i128 container; fixed-point multiply takes the signed 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_i_arith_wrapping_widen256!(
    Cold,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_i_arith_wrapping!(Warm, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping!(Warm, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping!(
    Warm, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping!(
    Warm,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_arith_saturating!(Precise, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_saturating!(Precise, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_saturating!(
    Precise, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_saturating!(
    Precise,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// --- Generic const-bound traits (round 202605021600) -------------------
//
// `Bounded` and `Identity` are the canonical const-trait surfaces for
// bottom/top and zero/one constants. Any type that meaningfully carries
// a min/max can impl Bounded; any type with a multiplicative identity
// can impl Identity. The substrate uses these to drive blanket impls
// of mask EMPTY/FULL, UFixed/IFixed inherent ZERO/ONE, and Bits ZERO.

/// Per-type bottom/top const surface.
///
/// `pub const trait`. Implemented for u8 / u16 / u32 / u64 / u128 and
/// i8 / i16 / i32 / i64 / i128 by macro impls below. Substrate types
/// that wrap these (Bits, UFixed, IFixed, Mask families) gain Bounded
/// via blanket impls keyed on the underlying primitive.
pub const trait Bounded: Sized {
    /// The minimum representable value of this type.
    const MIN: Self;
    /// The maximum representable value of this type.
    const MAX: Self;
}

/// Per-type multiplicative identity const surface.
///
/// `pub const trait`. Carries `ZERO` and `ONE` const associated items.
/// Substrate types gain Identity via blanket impls keyed on the
/// underlying primitive's Identity impl.
pub const trait Identity: Sized {
    /// The additive identity of this type.
    const ZERO: Self;
    /// The multiplicative identity of this type.
    const ONE: Self;
}

macro_rules! impl_bounded_identity_u {
    ($($ty:ty),+) => {
        $(
            impl const Bounded for $ty {
                const MIN: Self = <$ty>::MIN;
                const MAX: Self = <$ty>::MAX;
            }
            impl const Identity for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )+
    };
}

macro_rules! impl_bounded_identity_i {
    ($($ty:ty),+) => {
        $(
            impl const Bounded for $ty {
                const MIN: Self = <$ty>::MIN;
                const MAX: Self = <$ty>::MAX;
            }
            impl const Identity for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )+
    };
}

impl_bounded_identity_u!(u8, u16, u32, u64, u128, usize);
impl_bounded_identity_i!(i8, i16, i32, i64, i128, isize);

// Float Bounded / Identity. Bottom-out at the language-defined MIN /
// MAX inherents and 0.0 / 1.0 literals. The `Ieee` seal on the public
// surface restricts these traits' use to `f32` / `f64` exposure
// through `FastFloat<F>` / `StrictFloat<F>`, but the substrate impls
// land here so the canonical const surface stays unified.
macro_rules! impl_bounded_identity_f {
    ($($ty:ty),+) => {
        $(
            impl const Bounded for $ty {
                const MIN: Self = <$ty>::MIN;
                const MAX: Self = <$ty>::MAX;
            }
            impl const Identity for $ty {
                const ZERO: Self = 0.0;
                const ONE: Self = 1.0;
            }
        )+
    };
}

impl_bounded_identity_f!(f32, f64);

// --- SignedIdentity (round 202605021800) ---------------------------------
//
// `SignedIdentity` is the signed-primitive companion to `Identity`. It
// adds `NEG_ONE`, the only constant beyond ZERO/ONE that consumers
// reach for on signed types where the asymmetry matters. Per round
// 202605021800, the prior `UPrimConst` / `IPrimConst` traits (which
// duplicated Bounded + Identity with a NEG_ONE on the signed side) are
// removed; the canonical surfaces are `Bounded`, `Identity`, and
// `SignedIdentity`. Consumers route through these, not through type-
// specific inherent constants.

/// Per-signed-type negative-one const surface.
///
/// `pub const trait`. Supertrait-bounded on `Identity`. Implemented for
/// `i8` / `i16` / `i32` / `i64` / `i128` / `isize` and substrate types
/// that wrap signed primitives (`IFixed`, `Bits<N, S, Signed>`).
pub const trait SignedIdentity: [const] Identity {
    /// The additive inverse of the multiplicative identity (signed -1).
    const NEG_ONE: Self;
}

macro_rules! impl_signed_identity {
    ($($ty:ty),+) => {
        $(impl const SignedIdentity for $ty {
            const NEG_ONE: Self = -1;
        })+
    };
}

impl_signed_identity!(i8, i16, i32, i64, i128, isize);

// Float SignedIdentity. ML / scientific code uses `-1.0` as a
// canonical constant; the substrate provides it through the same
// trait surface as signed integers so consumers reach for
// `<F as SignedIdentity>::NEG_ONE` rather than a type-specific path.
// Per audit pivot 7 followup: ship in this round, not later.
macro_rules! impl_signed_identity_f {
    ($($ty:ty),+) => {
        $(impl const SignedIdentity for $ty {
            const NEG_ONE: Self = -1.0;
        })+
    };
}

impl_signed_identity_f!(f32, f64);
