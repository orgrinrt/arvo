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

use crate::{Cold, HasAxes, Hot, IContainerFor, Precise, UContainerFor, Warm};

/// Unsigned arithmetic dispatch for `(strategy, N)`.
///
/// Keyed on the same `N` that `UContainerFor` uses. Lets
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
pub const trait UArith<const N: u16>: [const] UContainerFor<N> + HasAxes {
    /// Strategy-specific `+`.
    fn u_add(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `-`.
    fn u_sub(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `*`.
    fn u_mul(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `/`. Div-by-zero: wrapping strategies use
    /// `wrapping_div` (panic convention); Precise clamps to max.
    fn u_div(a: Self::T, b: Self::T) -> Self::T;
}

/// Signed arithmetic dispatch for `(strategy, N)`.
///
/// `HasAxes` supertrait per the same Pass E convention as `UArith`.
pub const trait IArith<const N: u16>: [const] IContainerFor<N> + HasAxes {
    /// Strategy-specific `+`.
    fn i_add(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `-`.
    fn i_sub(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `*`.
    fn i_mul(a: Self::T, b: Self::T) -> Self::T;
    /// Strategy-specific `/`. Div-by-zero: wrapping strategies use
    /// `wrapping_div` (panic convention); Precise clamps to max.
    fn i_div(a: Self::T, b: Self::T) -> Self::T;
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
                fn u_add(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T {
                    if b == <<Self as UContainerFor<$bits>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
            }
        )+
    };
}

macro_rules! impl_u_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.saturating_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.saturating_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T { a.saturating_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as UContainerFor<$bits>>::T, b: <Self as UContainerFor<$bits>>::T)
                    -> <Self as UContainerFor<$bits>>::T {
                    // Precise never panics on div-by-zero: clamp to MAX.
                    if b == <<Self as UContainerFor<$bits>>::T as Identity>::ZERO {
                        <<Self as UContainerFor<$bits>>::T as USaturating>::saturating_max()
                    } else {
                        a / b
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
                fn i_add(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T {
                    if b == <<Self as IContainerFor<$bits>>::T as Identity>::ZERO {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.saturating_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.saturating_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T { a.saturating_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as IContainerFor<$bits>>::T, b: <Self as IContainerFor<$bits>>::T)
                    -> <Self as IContainerFor<$bits>>::T {
                    // Precise guards against div-by-zero: clamp to MAX.
                    if b == <<Self as IContainerFor<$bits>>::T as Identity>::ZERO {
                        <<Self as IContainerFor<$bits>>::T as ISaturating>::saturating_max()
                    } else {
                        // Guard signed overflow (MIN / -1) by preferring
                        // `saturating_div` rather than `wrapping_div`.
                        a.saturating_div(b)
                    }
                }
            }
        )+
    };
}

// Wrapping strategies: Hot / Warm / Cold.
impl_u_arith_wrapping!(Hot, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping!(Hot, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping!(
    Hot, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping!(
    Hot,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Round 202604280500: Hot 65..=128 wrapping (u128 wrapping_add etc.).
#[rustfmt::skip]
impl_u_arith_wrapping!(
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

impl_u_arith_wrapping!(Cold, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping!(Cold, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping!(
    Cold, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping!(
    Cold,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
#[rustfmt::skip]
impl_u_arith_wrapping!(
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
impl_i_arith_wrapping!(Hot, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping!(Hot, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping!(
    Hot, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping!(
    Hot,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
#[rustfmt::skip]
impl_i_arith_wrapping!(
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

impl_i_arith_wrapping!(Cold, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping!(Cold, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping!(
    Cold, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping!(
    Cold,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
#[rustfmt::skip]
impl_i_arith_wrapping!(
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
