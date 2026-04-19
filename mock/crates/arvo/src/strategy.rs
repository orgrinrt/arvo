//! Strategy markers for fixed-point types.
//!
//! Four strategies determine container width, arithmetic semantics,
//! and cross-width rules for `UFixed` / `IFixed`:
//!
//! | Strategy | Container          | Arithmetic           |
//! |----------|--------------------|----------------------|
//! | `Hot`    | min aligned        | wrapping             |
//! | `Warm`   | 2x logical         | wrapping (safe 1-op) |
//! | `Cold`   | min (bitpacked)    | widen-op-narrow      |
//! | `Precise`| 2x logical         | saturating           |
//!
//! Container selection is handled by the sealed `UContainerFor` /
//! `IContainerFor` traits: one impl per (strategy, bit-range) pair.
//! `Warm` is not implemented for the `33..=64` bit range, making
//! `UFixed<_, _, Warm>` with `I + F > 32` a compile-time error per
//! the doc CL D2 resolution.

use core::marker::ConstParamTy;

use crate::newtype::{FBits, IBits};

mod sealed {
    pub trait Sealed {}
}

/// Strategy marker trait.
///
/// Implemented by the four zero-sized markers `Hot`, `Warm`, `Cold`,
/// and `Precise`. Sealed — consumers cannot add new strategies.
pub trait Strategy: sealed::Sealed + Copy + Clone + Default + 'static {
    /// Human-readable name of the strategy.
    const NAME: &'static str;

    /// Conservativeness rank. Higher is more conservative. Used by
    /// cross-strategy operation resolution:
    /// `Precise > Cold > Warm > Hot`.
    const RANK: u8;
}

/// Optimised for L1 density and operation throughput.
///
/// Container is the minimum byte-aligned standard width that fits
/// `I + F` bits. Arithmetic is wrapping. Single instruction per op;
/// LLVM vectorises freely.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Hot;

/// Store big, operate fast. The development-friendly default.
///
/// Container is 2x the logical bit width. A single `add` / `sub` /
/// `mul` of two values within their logical range cannot overflow
/// the container. Bounded to `I + F <= 32` per doc CL D2: no u128
/// container is available, so Warm is forbidden at logical widths
/// beyond 32 bits rather than degrading silently.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Warm;

/// Store small, operate carefully. Optimised for storage density.
///
/// Minimum container, bitpacked for sub-byte values. Arithmetic
/// widens to 2x before operating, narrows back on store.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Cold;

/// Store exactly, operate exactly. Correctness above all.
///
/// Container is 2x the logical width (same physical layout as Warm).
/// Arithmetic is saturating: overflow clamps to logical min/max
/// rather than wrapping.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Precise;

impl sealed::Sealed for Hot {}
impl sealed::Sealed for Warm {}
impl sealed::Sealed for Cold {}
impl sealed::Sealed for Precise {}

impl Strategy for Hot {
    const NAME: &'static str = "Hot";
    const RANK: u8 = 0;
}
impl Strategy for Warm {
    const NAME: &'static str = "Warm";
    const RANK: u8 = 1;
}
impl Strategy for Cold {
    const NAME: &'static str = "Cold";
    const RANK: u8 = 2;
}
impl Strategy for Precise {
    const NAME: &'static str = "Precise";
    const RANK: u8 = 3;
}

/// Unsigned container dispatch: `(strategy, logical_bits) -> type`.
///
/// Implemented once per valid `(S, BITS)` pair. `UFixed<I, F, S>`
/// uses `<S as UContainerFor<{I.0 + F.0}>>::T` as its storage.
/// Absence of an impl is how Warm at `BITS > 32` becomes a compile
/// error.
///
/// The associated-type bound is the minimum surface every concrete
/// container satisfies (u8/u16/u32/u64). Keeping it broad here lets
/// `UFixed` delegate Copy/Eq/Ord/Default without re-bounding on the
/// const expression in every impl block.
pub trait UContainerFor<const BITS: u8>: Strategy {
    /// Concrete storage integer for this (strategy, bit-width) pair.
    type T: Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Default
        + core::hash::Hash
        + core::fmt::Debug
        + 'static;
}

/// Signed container dispatch. Same shape as `UContainerFor` with
/// signed integers. `BITS` is the total `1 + I + F` for `IFixed`.
pub trait IContainerFor<const BITS: u8>: Strategy {
    /// Concrete signed storage integer for this (strategy, bit-width) pair.
    type T: Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Default
        + core::hash::Hash
        + core::fmt::Debug
        + 'static;
}

// --- Container impl table --------------------------------------------------
//
// Hot / Cold: minimum aligned container.
//   1..=8  -> u8  / i8
//   9..=16 -> u16 / i16
//   17..=32 -> u32 / i32
//   33..=64 -> u64 / i64
//
// Warm / Precise: 2x logical width (one bucket up).
//   1..=8  -> u16 / i16
//   9..=16 -> u32 / i32
//   17..=32 -> u64 / i64
//   33..=64 -> (Warm unavailable per D2; Precise uses u64 with saturating ops)
//
// Implemented via per-value macro expansion so each concrete BITS
// value has its own impl. Avoids the need for const-range trait impls
// which generic_const_exprs does not currently support.

macro_rules! impl_u_container {
    ($strategy:ty, $ty:ty, $($bits:literal),+) => {
        $(
            impl UContainerFor<$bits> for $strategy {
                type T = $ty;
            }
        )+
    };
}

macro_rules! impl_i_container {
    ($strategy:ty, $ty:ty, $($bits:literal),+) => {
        $(
            impl IContainerFor<$bits> for $strategy {
                type T = $ty;
            }
        )+
    };
}

// Hot: min aligned.
impl_u_container!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_container!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_container!(
    Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_container!(
    Hot, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Cold: same integer widths as Hot; Cold's bitpacking is an access-path
// concern, not a container-type concern. Column storage masks on access.
impl_u_container!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_container!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_container!(
    Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_container!(
    Cold, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Warm: 2x aligned. Intentionally no impls for BITS > 32.
impl_u_container!(Warm, u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_container!(Warm, u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_container!(
    Warm, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);

// Precise: 2x aligned up to 32, u64 at 33..=64 with saturating semantics.
impl_u_container!(Precise, u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_container!(Precise, u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_container!(
    Precise, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_container!(
    Precise, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Signed containers — same buckets, signed integer types. `BITS`
// here is the total `1 + I + F` for IFixed.
impl_i_container!(Hot, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_container!(Hot, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_container!(
    Hot, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_container!(
    Hot, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_container!(Cold, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_container!(Cold, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_container!(
    Cold, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_container!(
    Cold, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_container!(Warm, i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_container!(Warm, i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_container!(
    Warm, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);

impl_i_container!(Precise, i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_container!(Precise, i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_container!(
    Precise, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_container!(
    Precise, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

/// Const-fn helper: total logical bits for a `UFixed<I, F, S>`.
///
/// `UFixed` logical width is `I + F`. The trait bound uses this to
/// look up the container type.
#[inline(always)]
pub const fn ufixed_bits(i: IBits, f: FBits) -> u8 {
    i.0 + f.0
}

/// Const-fn helper: total logical bits for an `IFixed<I, F, S>`.
///
/// `IFixed` reserves one bit for the sign; logical width is `1 + I + F`.
#[inline(always)]
pub const fn ifixed_bits(i: IBits, f: FBits) -> u8 {
    1 + i.0 + f.0
}

/// Extract the inner `u8` of an `IBits`.
///
/// Wrapping this in a const fn lets it appear inside anonymous
/// const-generic expressions — direct field access (`I.0`) is not
/// permitted there on current nightly.
#[inline(always)]
pub const fn ibits_u8(i: IBits) -> u8 {
    i.0
}

/// Extract the inner `u8` of an `FBits`.
///
/// See `ibits_u8` for why this exists as a free function.
#[inline(always)]
pub const fn fbits_u8(f: FBits) -> u8 {
    f.0
}

/// Indicator const: `1` when `f > 0`, `0` when `f == 0`.
///
/// Used to express "F has a fractional component" in const-generic
/// where-clauses without field access or struct construction.
#[inline(always)]
pub const fn is_fractional(f: FBits) -> usize {
    if f.0 == 0 { 0 } else { 1 }
}

// --- Strategy resolution for cross-strategy ops ----------------------------
//
// `Precise > Cold > Warm > Hot`. The more conservative strategy wins.
// Encoded via trait-level selection: `Resolve<S1, S2>` picks the winner.
//
// Implemented as a nested table so blanket impls don't collide.

/// Resolve the more conservative of two strategies.
///
/// `Resolve<S1, S2>::Out` is the higher-rank strategy; see
/// `Strategy::RANK`.
pub trait Resolve<Other: Strategy>: Strategy {
    /// The resolved strategy: more conservative of `Self` and `Other`.
    type Out: Strategy;
}

macro_rules! impl_resolve {
    ($lhs:ty, $rhs:ty, $out:ty) => {
        impl Resolve<$rhs> for $lhs {
            type Out = $out;
        }
    };
}

// Self: identity.
impl_resolve!(Hot, Hot, Hot);
impl_resolve!(Warm, Warm, Warm);
impl_resolve!(Cold, Cold, Cold);
impl_resolve!(Precise, Precise, Precise);

// Hot with others.
impl_resolve!(Hot, Warm, Warm);
impl_resolve!(Hot, Cold, Cold);
impl_resolve!(Hot, Precise, Precise);
impl_resolve!(Warm, Hot, Warm);
impl_resolve!(Cold, Hot, Cold);
impl_resolve!(Precise, Hot, Precise);

// Warm with others.
impl_resolve!(Warm, Cold, Cold);
impl_resolve!(Warm, Precise, Precise);
impl_resolve!(Cold, Warm, Cold);
impl_resolve!(Precise, Warm, Precise);

// Cold with Precise.
impl_resolve!(Cold, Precise, Precise);
impl_resolve!(Precise, Cold, Precise);
