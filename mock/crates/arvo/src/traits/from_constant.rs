//! Construction from a typed constant.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::float::{FastFloat, StrictFloat};
use crate::ifixed::IFixed;
use crate::strategy::{Cold, FromU8Ieee, Hot, Ieee, Precise, Warm};
use crate::ufixed::UFixed;
pub use arvo_numeric_contracts::FromConstant;
use arvo_storage::{fbits, ibits, FBits, USize};
use arvo_transparent::Transparent;

// --- FromConstant ---------------------------------------------------------
//
// For UFixed / IFixed: `n` is placed at the integer-bit position
// (multiplied by 2^F). Per-(strategy, BITS) concrete impls avoid the
// const-expr cycle a container-associated-type bound would create.

macro_rules! impl_from_constant_ufixed {
    ($strategy:ty, $ctype:ty, $($i:literal),+) => {
        $(
            // F = 0 (integer UFixed).
            impl const FromConstant for UFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw(<USize as Transparent>::raw(C) as $ctype)
                }
            }
        )+
    };
}

macro_rules! impl_from_constant_ufixed_fractional {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl const FromConstant for UFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw((<USize as Transparent>::raw(C) as $ctype) << $f)
                }
            }
        )+
    };
}

// Hot integer UFixed at every I from 1 to 64.
impl_from_constant_ufixed!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_from_constant_ufixed!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_from_constant_ufixed!(
    Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_from_constant_ufixed!(
    Hot, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_from_constant_ufixed!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_from_constant_ufixed!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_from_constant_ufixed!(
    Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_from_constant_ufixed!(
    Cold, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_from_constant_ufixed!(Warm, u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_from_constant_ufixed!(Warm, u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_from_constant_ufixed!(
    Warm, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);

impl_from_constant_ufixed!(Precise, u16, 1, 2, 3, 4, 5, 6, 7, 8);
impl_from_constant_ufixed!(Precise, u32, 9, 10, 11, 12, 13, 14, 15, 16);
impl_from_constant_ufixed!(
    Precise, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
// Pass D of round 202604281000: Precise 33..=64 promoted to u128.
#[rustfmt::skip]
impl_from_constant_ufixed!(
    Precise, u128,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Fractional UFixed: a representative slice at I=8 for common F
// widths. Full coverage of every (I, F) pair would balloon; the
// tests exercise representative shapes and consumers compose their
// own wrappers. The trait surface is the guarantee; specific
// instantiations can be added as needed without an API break.

// Container buckets by BITS = I + F = 8 + F:
//   Hot/Cold:    1..=8 -> u8;   9..=16 -> u16; 17..=32 -> u32;  33..=64 -> u64.
//   Warm/Precise: 1..=8 -> u16; 9..=16 -> u32; 17..=32 -> u64.
impl_from_constant_ufixed_fractional!(Warm, u32, 8, 1, 2, 4, 8);
impl_from_constant_ufixed_fractional!(Warm, u64, 8, 16);
impl_from_constant_ufixed_fractional!(Hot, u16, 8, 1, 2, 4, 8);
impl_from_constant_ufixed_fractional!(Hot, u32, 8, 16);
impl_from_constant_ufixed_fractional!(Cold, u16, 8, 1, 2, 4, 8);
impl_from_constant_ufixed_fractional!(Cold, u32, 8, 16);
impl_from_constant_ufixed_fractional!(Precise, u32, 8, 1, 2, 4, 8);
impl_from_constant_ufixed_fractional!(Precise, u64, 8, 16);

// IFixed `from_constant`. BITS = 1 + I + F, so the I=i, F=0 case
// has BITS = i + 1.

macro_rules! impl_from_constant_ifixed {
    ($strategy:ty, $ctype:ty, $($i:literal),+) => {
        $(
            impl const FromConstant for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw(<USize as Transparent>::raw(C) as $ctype)
                }
            }
        )+
    };
}

macro_rules! impl_from_constant_ifixed_fractional {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl const FromConstant for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw((<USize as Transparent>::raw(C) as $ctype) << $f)
                }
            }
        )+
    };
}

// Hot IFixed integer: BITS = 1 + I ranges 2..=65; but container table
// caps at 64 bits. So I ranges 1..=63.
impl_from_constant_ifixed!(Hot, i8, 1, 2, 3, 4, 5, 6, 7);
impl_from_constant_ifixed!(Hot, i16, 8, 9, 10, 11, 12, 13, 14, 15);
impl_from_constant_ifixed!(
    Hot, i32, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_from_constant_ifixed!(
    Hot, i64,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

impl_from_constant_ifixed!(Cold, i8, 1, 2, 3, 4, 5, 6, 7);
impl_from_constant_ifixed!(Cold, i16, 8, 9, 10, 11, 12, 13, 14, 15);
impl_from_constant_ifixed!(
    Cold, i32, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_from_constant_ifixed!(
    Cold, i64,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

// Warm IFixed: BITS = 1 + I <= 32, so I <= 31.
impl_from_constant_ifixed!(Warm, i16, 1, 2, 3, 4, 5, 6, 7);
impl_from_constant_ifixed!(Warm, i32, 8, 9, 10, 11, 12, 13, 14, 15);
impl_from_constant_ifixed!(
    Warm, i64, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);

impl_from_constant_ifixed!(Precise, i16, 1, 2, 3, 4, 5, 6, 7);
impl_from_constant_ifixed!(Precise, i32, 8, 9, 10, 11, 12, 13, 14, 15);
impl_from_constant_ifixed!(
    Precise, i64, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
// Pass D of round 202604281000: Precise IFixed BITS=33..=64 (I=32..=63)
// promoted to i128.
#[rustfmt::skip]
impl_from_constant_ifixed!(
    Precise, i128,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

// Fractional IFixed: representative slice at I=7 (so BITS is nice).
// IFixed BITS = 1 + 7 + F = 8 + F. Buckets:
//   Hot/Cold:     9..=16 -> i16;  17..=32 -> i32.
//   Warm/Precise: 9..=16 -> i32;  17..=32 -> i64.
impl_from_constant_ifixed_fractional!(Warm, i32, 7, 1, 2, 4, 8);
impl_from_constant_ifixed_fractional!(Warm, i64, 7, 16);
impl_from_constant_ifixed_fractional!(Hot, i16, 7, 1, 2, 4, 8);
impl_from_constant_ifixed_fractional!(Hot, i32, 7, 16);
impl_from_constant_ifixed_fractional!(Cold, i16, 7, 1, 2, 4, 8);
impl_from_constant_ifixed_fractional!(Cold, i32, 7, 16);
impl_from_constant_ifixed_fractional!(Precise, i32, 7, 1, 2, 4, 8);
impl_from_constant_ifixed_fractional!(Precise, i64, 7, 16);

// Float FromConstant: the USize input bridges to the internal
// `FromU8Ieee` helper via an in-range `u8` cast; callers stay in
// USize for consistency with the public trait surface.

const impl<F: [const] Ieee + [const] FromU8Ieee> FromConstant for FastFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
        FastFloat(<F as FromU8Ieee>::from_u8_ieee(
            <USize as Transparent>::raw(C) as u8,
        ))
    }
}

const impl<F: [const] Ieee + [const] FromU8Ieee> FromConstant for StrictFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
        StrictFloat(<F as FromU8Ieee>::from_u8_ieee(
            <USize as Transparent>::raw(C) as u8,
        ))
    }
}

// FromU8Ieee lives in arvo-strategy::ieee per round 202605030400.
// The arvo facade re-exports it at lib.rs level; consumer code keeps
// the existing arvo::FromU8Ieee path. The FromConstant impls above
// reach the trait via the crate::FromU8Ieee re-export path.
