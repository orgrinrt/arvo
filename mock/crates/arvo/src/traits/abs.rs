//! Absolute value, including the unsigned identity case.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::float::{FastFloat, StrictFloat};
use crate::ifixed::IFixed;
use crate::strategy::{
    ufixed_bits, BitsContainerFor, Cold, Hot, Precise, Strategy, Unsigned, Warm,
};
use crate::ufixed::UFixed;
pub use arvo_numeric_contracts::Abs;
use arvo_storage::{fbits, ibits, FBits, IBits};
use arvo_transparent::Transparent;

// --- Abs -------------------------------------------------------------------

const impl<const I: IBits, const F: FBits, S: Strategy> Abs for UFixed<I, F, S>
where
    S: [const] BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
{
    type Output = Self;
    /// Identity: unsigned values are their own absolute value.
    #[inline(always)]
    fn abs(self) -> Self {
        self
    }
}

// IFixed abs: per-(strategy, I, F) concrete impls. Bounding on the
// container associated type projects back through the const expression
// and produces an evaluator cycle; spelling out concrete `IBits(i)` /
// `FBits(f)` values bypasses the projection.
//
// Hot / Warm / Cold use `wrapping_abs`. Precise uses `saturating_abs`.
// We cover every integer IFixed shape (F = 0) that the container table
// supports, plus a representative slice of fractional shapes matching
// the FromConstant coverage below.

macro_rules! impl_abs_ifixed_integer_wrap {
    ($strategy:ty, $ctype:ty, $($i:literal),+) => {
        $(
            impl const Abs for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                type Output = Self;
                #[inline(always)]
                fn abs(self) -> Self {
                    Self::from_raw(<$ctype>::wrapping_abs(self.to_raw()))
                }
            }
        )+
    };
}

macro_rules! impl_abs_ifixed_integer_sat {
    ($strategy:ty, $ctype:ty, $($i:literal),+) => {
        $(
            impl const Abs for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                type Output = Self;
                #[inline(always)]
                fn abs(self) -> Self {
                    Self::from_raw(<$ctype>::saturating_abs(self.to_raw()))
                }
            }
        )+
    };
}

macro_rules! impl_abs_ifixed_fractional_wrap {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl const Abs for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                type Output = Self;
                #[inline(always)]
                fn abs(self) -> Self {
                    Self::from_raw(<$ctype>::wrapping_abs(self.to_raw()))
                }
            }
        )+
    };
}

macro_rules! impl_abs_ifixed_fractional_sat {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl const Abs for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                type Output = Self;
                #[inline(always)]
                fn abs(self) -> Self {
                    Self::from_raw(<$ctype>::saturating_abs(self.to_raw()))
                }
            }
        )+
    };
}

// Integer IFixed (F = 0). IFixed BITS = 1 + I, container bucketed
// per strategy/BITS in `strategy.rs`. I values: BITS-1.
// Hot: BITS 1..=64 -> I 0..=63. We skip I=0 (degenerate: 1-bit sign-only).
impl_abs_ifixed_integer_wrap!(Hot, i8, 1, 2, 3, 4, 5, 6, 7);
impl_abs_ifixed_integer_wrap!(Hot, i16, 8, 9, 10, 11, 12, 13, 14, 15);
impl_abs_ifixed_integer_wrap!(
    Hot, i32, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_abs_ifixed_integer_wrap!(
    Hot, i64,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

impl_abs_ifixed_integer_wrap!(Cold, i8, 1, 2, 3, 4, 5, 6, 7);
impl_abs_ifixed_integer_wrap!(Cold, i16, 8, 9, 10, 11, 12, 13, 14, 15);
impl_abs_ifixed_integer_wrap!(
    Cold, i32, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_abs_ifixed_integer_wrap!(
    Cold, i64,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

impl_abs_ifixed_integer_wrap!(Warm, i16, 1, 2, 3, 4, 5, 6, 7);
impl_abs_ifixed_integer_wrap!(Warm, i32, 8, 9, 10, 11, 12, 13, 14, 15);
impl_abs_ifixed_integer_wrap!(
    Warm, i64, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);

// Precise integer IFixed: BITS = 1 + I, projection picks the
// container per (Strategy, BITS) cell. After Pass D of round
// 202604281000, Precise 33..=64 uses i128 (was i64); the I=32..=63
// arm shifts to i128 to match.
impl_abs_ifixed_integer_sat!(Precise, i16, 1, 2, 3, 4, 5, 6, 7);
impl_abs_ifixed_integer_sat!(Precise, i32, 8, 9, 10, 11, 12, 13, 14, 15);
impl_abs_ifixed_integer_sat!(
    Precise, i64, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_abs_ifixed_integer_sat!(
    Precise, i128,
    32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63
);

// Fractional IFixed at I=7 (matches FromConstant fractional coverage).
// BITS = 1 + 7 + F = 8 + F. Container buckets:
//   Hot/Cold 9..=16 -> i16;  17..=32 -> i32.
//   Warm/Precise 9..=16 -> i32; 17..=32 -> i64.
impl_abs_ifixed_fractional_wrap!(Warm, i32, 7, 1, 2, 4, 8);
impl_abs_ifixed_fractional_wrap!(Warm, i64, 7, 16);
impl_abs_ifixed_fractional_wrap!(Hot, i16, 7, 1, 2, 4, 8);
impl_abs_ifixed_fractional_wrap!(Hot, i32, 7, 16);
impl_abs_ifixed_fractional_wrap!(Cold, i16, 7, 1, 2, 4, 8);
impl_abs_ifixed_fractional_wrap!(Cold, i32, 7, 16);
impl_abs_ifixed_fractional_sat!(Precise, i32, 7, 1, 2, 4, 8);
impl_abs_ifixed_fractional_sat!(Precise, i64, 7, 16);

// Float abs. No_std-compatible via sign-bit clear. `f*::from_bits` and
// `f*::to_bits` are const-stable, so the helpers stay const fn and the
// impls can be const.

#[inline(always)]
pub const fn abs_f32(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffffu32)
}

#[inline(always)]
pub const fn abs_f64(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffffu64)
}

const impl Abs for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f32(<Self as Transparent>::raw(self)))
    }
}

const impl Abs for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f64(<Self as Transparent>::raw(self)))
    }
}

const impl Abs for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f32(<Self as Transparent>::raw(self)))
    }
}

const impl Abs for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f64(<Self as Transparent>::raw(self)))
    }
}
