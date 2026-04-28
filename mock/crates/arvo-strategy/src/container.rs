//! Container-projection traits.
//!
//! `UContainerFor<N>` and `IContainerFor<N>` const traits map a
//! `(strategy, bit-width)` pair to a bare-primitive container type.
//! Per-N impls live below; their bodies project to `u8` / `u16` /
//! `u32` / `u64` (and signed counterparts) only, never `Bits`. The
//! `arvo-strategy` to `arvo-storage` direction stays acyclic.

use crate::{Cold, Hot, MultiContainer, Precise, Signed, Signedness, Strategy, Unsigned, Warm};

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
    note = "Hot / Cold cover 1..=128 directly; Warm / Precise cover 1..=64 via 2x-logical primitives. For wider widths, choose Hot or Cold explicitly: `Uint<128, Hot>`, etc. Beyond 128 bits, `Bits<N, S, Sign>` dispatches through `MultiContainer<HiT, LoT>` for storage; arithmetic on multi-value containers is BACKLOG-tracked."
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
    note = "Hot / Cold cover 1..=128 directly; Warm / Precise cover 1..=64 via 2x-logical primitives. For wider widths, choose Hot or Cold explicitly: `Int<128, Hot>`, etc. Beyond 128 bits, `Bits<N, S, Sign>` dispatches through `MultiContainer<HiT, LoT>` for storage; arithmetic on multi-value containers is BACKLOG-tracked."
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

/// Sign-aware container dispatch: `(strategy, N, Sign) -> type`.
///
/// Indirection trait used by `Bits<N, S, Sign>` to route through
/// either `UContainerFor<N>` or `IContainerFor<N>` based on the
/// `Sign` marker. Keeps the U / I tables independently extensible
/// while the Sign axis on `Bits` reaches the right table per the
/// blanket impls below.
///
/// `UFixed` / `IFixed` continue to bind on `UContainerFor<N>` /
/// `IContainerFor<N>` directly; only `Bits` itself binds on
/// `BitsContainerFor<N, Sign>`.
pub trait BitsContainerFor<const N: u8, Sign: Signedness>: Strategy {
    /// Concrete storage integer for this (strategy, bit-width, sign) triple.
    type T: Copy
        + Clone
        + PartialEq
        + Eq
        + Default
        + core::hash::Hash
        + core::fmt::Debug
        + 'static;
}

impl<S: Strategy, const N: u8> BitsContainerFor<N, Unsigned> for S
where
    S: UContainerFor<N>,
{
    type T = <S as UContainerFor<N>>::T;
}

impl<S: Strategy, const N: u8> BitsContainerFor<N, Signed> for S
where
    S: IContainerFor<N>,
{
    type T = <S as IContainerFor<N>>::T;
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
// Round 202604280500: Hot extends to 65..=128 via native u128.
#[rustfmt::skip]
impl_u_container!(
    Hot, u128,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);
// Round 202604280500: Hot extends to 129..=192 via MultiContainer<u64, u128>
// (smallest-footprint pair covering the 129..=192 logical range).
#[rustfmt::skip]
impl_u_container!(
    Hot, MultiContainer<u64, u128>,
    129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184,
    185, 186, 187, 188, 189, 190, 191, 192
);
// Round 202604280500: Hot extends to 193..=255 via MultiContainer<u128, u128>
// (193..=256 in design intent; const N: u8 caps at 255 in this round).
#[rustfmt::skip]
impl_u_container!(
    Hot, MultiContainer<u128, u128>,
    193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216,
    217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 243, 244, 245, 246, 247, 248,
    249, 250, 251, 252, 253, 254, 255
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
// Round 202604280500: Cold extends to 65..=128 via native u128 (mirrors Hot).
#[rustfmt::skip]
impl_u_container!(
    Cold, u128,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);
// Round 202604280500: Cold extends to 129..=192 via MultiContainer<u64, u128>
// (mirrors Hot; bitpacking is an access-path concern, not container-type).
#[rustfmt::skip]
impl_u_container!(
    Cold, MultiContainer<u64, u128>,
    129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184,
    185, 186, 187, 188, 189, 190, 191, 192
);
// Round 202604280500: Cold extends to 193..=255 via MultiContainer<u128, u128>.
#[rustfmt::skip]
impl_u_container!(
    Cold, MultiContainer<u128, u128>,
    193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216,
    217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 243, 244, 245, 246, 247, 248,
    249, 250, 251, 252, 253, 254, 255
);

// Warm: 2x aligned. Round 202604280500 extends Warm cap from 32 to 64
// via native u128 (the 2x-logical primitive at the new band).
impl_u_container!(Warm, u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_container!(Warm, u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_container!(
    Warm, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_container!(
    Warm, u128,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Precise: 2x aligned up to 32, u64 at 33..=64 with saturating semantics.
// Round 202604280500 keeps existing Precise 33..=64 = u64 (same as Hot
// container width, distinguished by saturating ops). Extending Precise
// to 65..=256 awaits the no-native-u256 design call (BACKLOG; same shape
// rationale as Warm).
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
// Round 202604280500: Hot extends to 65..=128 via native i128.
#[rustfmt::skip]
impl_i_container!(
    Hot, i128,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);
// Round 202604280500: Hot extends to 129..=192 via MultiContainer<i64, i128>
// (signed mirror of the unsigned Hot 129..=192 cells).
#[rustfmt::skip]
impl_i_container!(
    Hot, MultiContainer<i64, i128>,
    129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184,
    185, 186, 187, 188, 189, 190, 191, 192
);
// Round 202604280500: Hot extends to 193..=255 via MultiContainer<i128, i128>.
#[rustfmt::skip]
impl_i_container!(
    Hot, MultiContainer<i128, i128>,
    193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216,
    217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 243, 244, 245, 246, 247, 248,
    249, 250, 251, 252, 253, 254, 255
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
// Round 202604280500: Cold extends to 65..=128 via native i128 (mirrors Hot).
#[rustfmt::skip]
impl_i_container!(
    Cold, i128,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);
// Round 202604280500: Cold extends to 129..=192 via MultiContainer<i64, i128>.
#[rustfmt::skip]
impl_i_container!(
    Cold, MultiContainer<i64, i128>,
    129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160,
    161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 180, 181, 182, 183, 184,
    185, 186, 187, 188, 189, 190, 191, 192
);
// Round 202604280500: Cold extends to 193..=255 via MultiContainer<i128, i128>.
#[rustfmt::skip]
impl_i_container!(
    Cold, MultiContainer<i128, i128>,
    193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216,
    217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240,
    241, 242, 243, 244, 245, 246, 247, 248,
    249, 250, 251, 252, 253, 254, 255
);

impl_i_container!(Warm, i16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_container!(Warm, i32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_container!(
    Warm, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
// Round 202604280500: Warm extends to 33..=64 via native i128.
#[rustfmt::skip]
impl_i_container!(
    Warm, i128,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
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
