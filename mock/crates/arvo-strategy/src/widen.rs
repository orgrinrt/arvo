//! Strategy-keyed widening + narrowing conversions.
//!
//! `UWidenFrom<Src, N>`: produce `Self::T` (the Dst container) from
//! a `Src::T` (the source container), assuming Dst is at least as
//! wide as Src. Same for signed. Used by
//! `From<UFixed<I, F, Src>> for UFixed<I, F, Dst>` where Src widens
//! into Dst losslessly.
//!
//! `UNarrowFrom<Src, N>`: try to produce `Self::T` from `Src::T`,
//! returning `Outcome::Err(())` when the value doesn't fit Dst's
//! logical range. Used by
//! `TryFrom<UFixed<I, F, Src>> for UFixed<I, F, Dst>`.

use notko::Outcome;

use crate::{Hot, IContainerFor, Precise, UContainerFor, Warm};

/// Unsigned container widen bridge: `(Src, N) -> Dst::T`.
pub trait UWidenFrom<Src: UContainerFor<N>, const N: u8>: UContainerFor<N> {
    /// Widen an `Src::T` value into `Self::T`. Infallible by definition.
    fn u_widen(v: Src::T) -> Self::T;
}

/// Signed container widen bridge.
pub trait IWidenFrom<Src: IContainerFor<N>, const N: u8>: IContainerFor<N> {
    /// Widen an `Src::T` value into `Self::T`. Infallible by definition.
    fn i_widen(v: Src::T) -> Self::T;
}

/// Unsigned container narrow bridge with bounds check against the
/// logical range `[0, 2^N)`.
pub trait UNarrowFrom<Src: UContainerFor<N>, const N: u8>: UContainerFor<N> {
    /// Try to narrow `v` into `Self::T`. `Outcome::Err(())` when out of range.
    fn u_try_narrow(v: Src::T) -> Outcome<Self::T, ()>;
}

/// Signed container narrow bridge with logical-range check.
pub trait INarrowFrom<Src: IContainerFor<N>, const N: u8>: IContainerFor<N> {
    /// Try to narrow `v` into `Self::T`. `Outcome::Err(())` when out of range.
    fn i_try_narrow(v: Src::T) -> Outcome<Self::T, ()>;
}

// Helpers to spell out widen impls. Src and Dst are both container
// integer types; widen is `src as dst`. Narrow re-casts and checks
// against the logical range `(1 << N) - 1` for unsigned, and
// `[-(1 << (N-1)), (1 << (N-1)) - 1]` for signed.

macro_rules! impl_u_widen {
    ($src_strategy:ty => $dst_strategy:ty, $src_ty:ty => $dst_ty:ty, $($bits:literal),+) => {
        $(
            impl UWidenFrom<$src_strategy, $bits> for $dst_strategy {
                #[inline(always)]
                fn u_widen(v: $src_ty) -> $dst_ty { v as $dst_ty }
            }
        )+
    };
}

macro_rules! impl_i_widen {
    ($src_strategy:ty => $dst_strategy:ty, $src_ty:ty => $dst_ty:ty, $($bits:literal),+) => {
        $(
            impl IWidenFrom<$src_strategy, $bits> for $dst_strategy {
                #[inline(always)]
                fn i_widen(v: $src_ty) -> $dst_ty { v as $dst_ty }
            }
        )+
    };
}

macro_rules! impl_u_narrow {
    ($src_strategy:ty => $dst_strategy:ty, $src_ty:ty => $dst_ty:ty, $($bits:literal),+) => {
        $(
            impl UNarrowFrom<$src_strategy, $bits> for $dst_strategy {
                #[inline(always)]
                fn u_try_narrow(v: $src_ty) -> Outcome<$dst_ty, ()> {
                    // Logical max for this N is (1 << N) - 1.
                    // Compute via u128 so N up to 128 don't overflow
                    // the source type before the comparison.
                    let max_u128: u128 = (1u128 << $bits) - 1;
                    if (v as u128) > max_u128 { Outcome::Err(()) } else { Outcome::Ok(v as $dst_ty) }
                }
            }
        )+
    };
}

macro_rules! impl_i_narrow {
    ($src_strategy:ty => $dst_strategy:ty, $src_ty:ty => $dst_ty:ty, $($bits:literal),+) => {
        $(
            impl INarrowFrom<$src_strategy, $bits> for $dst_strategy {
                #[inline(always)]
                fn i_try_narrow(v: $src_ty) -> Outcome<$dst_ty, ()> {
                    // Signed logical range for N: [-(1 << (N-1)), (1 << (N-1)) - 1].
                    // Compute via i128 so N up to 127 don't overflow
                    // the source type during bound computation.
                    let min_i128: i128 = -(1i128 << ($bits - 1));
                    let max_i128: i128 = (1i128 << ($bits - 1)) - 1;
                    let v_i128: i128 = v as i128;
                    if v_i128 < min_i128 || v_i128 > max_i128 { Outcome::Err(()) } else { Outcome::Ok(v as $dst_ty) }
                }
            }
        )+
    };
}

// --- Hot -> Warm (unsigned) ---
// Hot N container -> Warm container at same N:
//   1..=8:  u8  -> u16
//   9..=16: u16 -> u32
//   17..=32: u32 -> u64
//   33..=64: u64 -> u128 (round 202604280500: Warm extended via u128).
impl_u_widen!(Hot => Warm, u8 => u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_widen!(Hot => Warm, u16 => u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_widen!(
    Hot => Warm, u32 => u64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_widen!(
    Hot => Warm, u64 => u128,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// --- Hot -> Precise (unsigned) ---
// Same container widths as Warm at N <= 32; at 33..=64 Precise uses
// u64 (same as Hot), so the widen is a no-op cast.
impl_u_widen!(Hot => Precise, u8 => u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_widen!(Hot => Precise, u16 => u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_widen!(
    Hot => Precise, u32 => u64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_widen!(
    Hot => Precise, u64 => u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// --- Warm -> Precise (unsigned) ---
// Both use 2x container at N <= 32; widening is a no-op cast. At
// 33..=64 Warm = u128, Precise = u64 (same as Hot); narrow not widen.
// The Warm -> Precise edge at 33..=64 ships as a Warm narrow rather
// than a widen; tracked alongside Cold widen / narrow as a follow-up
// once the cross-strategy resolution warning lands (Round C).
impl_u_widen!(Warm => Precise, u16 => u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_widen!(Warm => Precise, u32 => u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_widen!(
    Warm => Precise, u64 => u64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);

// --- Warm -> Hot, Precise -> Hot (unsigned, TryFrom) ---
impl_u_narrow!(Warm => Hot, u16 => u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_narrow!(Warm => Hot, u32 => u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_narrow!(
    Warm => Hot, u64 => u32,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_narrow!(
    Warm => Hot, u128 => u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_u_narrow!(Precise => Hot, u16 => u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_narrow!(Precise => Hot, u32 => u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_narrow!(
    Precise => Hot, u64 => u32,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_narrow!(
    Precise => Hot, u64 => u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// --- Signed mirrors ---
impl_i_widen!(Hot => Warm, i8 => i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_widen!(Hot => Warm, i16 => i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_widen!(
    Hot => Warm, i32 => i64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_widen!(
    Hot => Warm, i64 => i128,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_widen!(Hot => Precise, i8 => i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_widen!(Hot => Precise, i16 => i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_widen!(
    Hot => Precise, i32 => i64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_widen!(
    Hot => Precise, i64 => i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_widen!(Warm => Precise, i16 => i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_widen!(Warm => Precise, i32 => i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_widen!(
    Warm => Precise, i64 => i64,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
// Warm -> Precise at 33..=64 is a narrow (Warm = i128, Precise = i64);
// shipped as a follow-up alongside Cold widen / narrow + cross-strategy
// resolution (Round C).

impl_i_narrow!(Warm => Hot, i16 => i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_narrow!(Warm => Hot, i32 => i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_narrow!(
    Warm => Hot, i64 => i32,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_narrow!(
    Warm => Hot, i128 => i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_narrow!(Precise => Hot, i16 => i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_narrow!(Precise => Hot, i32 => i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_narrow!(
    Precise => Hot, i64 => i32,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_narrow!(
    Precise => Hot, i64 => i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Cold widen/narrow impls deferred. Cold participates in the widen
// lattice in a future round (Cold widens to Precise via the same
// shape used here; tracked in arvo's BACKLOG.md.tmpl under "audit-
// driven chain forward-looking entries"). When the impls land, the
// `Cold` import goes back into scope at the top of this file.
//
// Round 202604280500: 65..=128 widen / narrow cells (Hot <-> Warm /
// Precise) require Warm 65..=128 / Precise 65..=128 containers, which
// this round does not ship (BACKLOG: no native u256 for the 2x logical
// rule). Hot <-> Cold widen / narrow at the new band stays uniform
// (both u128) and lands when the Cold widen lattice is shipped.
