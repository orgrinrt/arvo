//! Container-projection traits.
//!
//! `UContainerFor<N>` and `IContainerFor<N>` const traits map a
//! `(strategy, bit-width)` pair to a bare-primitive container type.
//! Per-N impls live below; their bodies project to `u8` / `u16` /
//! `u32` / `u64` (and signed counterparts) only, never `Bits`. The
//! `arvo-strategy` to `arvo-storage` direction stays acyclic.

use crate::{Cold, Hot, Precise, Strategy, Warm};

/// Unsigned container dispatch: `(strategy, logical_bits) -> type`.
///
/// Implemented once per valid `(S, N)` pair. `UFixed<I, F, S>`
/// uses `<S as UContainerFor<{I.0 + F.0}>>::T` as its storage.
/// Absence of an impl is how Warm at `N > 32` becomes a compile
/// error.
///
/// The associated-type bound is the minimum surface every concrete
/// container satisfies (u8/u16/u32/u64). Keeping it broad here lets
/// `UFixed` delegate Copy/Eq/Ord/Default without re-bounding on the
/// const expression in every impl block.
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a container for {N}-bit width",
    note = "Warm caps at I+F<=32; for wider widths, choose Hot or Precise explicitly: `Uint64<Hot>`, `Uint64<Precise>`, etc."
)]
pub const trait UContainerFor<const N: u8>: Strategy {
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
/// signed integers. `N` is the total `1 + I + F` for `IFixed`.
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a signed container for {N}-bit width",
    note = "Warm caps at 1+I+F<=32; for wider widths, choose Hot or Precise explicitly: `Int64<Hot>`, `Int64<Precise>`, etc."
)]
pub const trait IContainerFor<const N: u8>: Strategy {
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

macro_rules! impl_u_container {
    ($strategy:ty, $ty:ty, $($bits:literal),+) => {
        $(
            impl const UContainerFor<$bits> for $strategy {
                type T = $ty;
            }
        )+
    };
}

macro_rules! impl_i_container {
    ($strategy:ty, $ty:ty, $($bits:literal),+) => {
        $(
            impl const IContainerFor<$bits> for $strategy {
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

// Warm: 2x aligned. Intentionally no impls for N > 32.
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

// Signed containers. Same buckets, signed integer types. `N` here
// is the total `1 + I + F` for IFixed.
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
