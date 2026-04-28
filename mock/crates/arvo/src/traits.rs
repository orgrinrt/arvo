//! Numeric traits.
//!
//! Five composable traits give generic algorithms a minimum vocabulary
//! for arithmetic over arvo's numeric family without pulling in
//! per-strategy arithmetic impls.
//!
//! | Trait          | Surface                                     | Expected on         |
//! |----------------|---------------------------------------------|---------------------|
//! | `TotalOrd`     | `total_cmp(&self, &other) -> Ordering`      | all numerics        |
//! | `Sqrt`         | `sqrt(self) -> Self`                        | floats, integer UFixed |
//! | `Recip`        | `recip(self) -> Self`                       | floats              |
//! | `Abs`          | `abs(self) -> Self`                         | signed + UFixed (id) |
//! | `FromConstant` | `from_constant(USize) -> Self`              | every concrete type |
//!
//! Fractional UFixed / IFixed do NOT get `Sqrt` / `Recip` in this
//! round — those require fixed-point arithmetic tables that land in
//! a later round. The trait surface exists; the per-type impls stop
//! at the unambiguous cases (integer UFixed sqrt via `u*::isqrt`, and
//! every float wrapper).

use core::cmp::Ordering;

use crate::float::{FastFloat, Ieee, StrictFloat};
use crate::ifixed::IFixed;
pub use arvo_numeric_contracts::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
use arvo_storage::{FBits, IBits, USize, fbits, ibits};
use crate::strategy::{
    Cold, Hot, IContainerFor, Precise, Strategy, UContainerFor, Warm, ifixed_bits, ufixed_bits,
};
use crate::ufixed::UFixed;

// --- TotalOrd --------------------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> TotalOrd for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.to_raw().cmp(&other.to_raw())
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> TotalOrd for IFixed<I, F, S>
where
    S: IContainerFor<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.to_raw().cmp(&other.to_raw())
    }
}

impl TotalOrd for FastFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl TotalOrd for FastFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl TotalOrd for StrictFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl TotalOrd for StrictFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

// --- Sqrt ------------------------------------------------------------------
//
// Integer UFixed (F == 0) uses `u*::isqrt`. Fractional UFixed is out
// of scope for this round. We spell out one impl per `(strategy, I)`
// pair so each impl has a concrete container type — avoids the
// const-expr cycle that a blanket `where <S as UContainerFor<...>>::T:
// ...` produces.

macro_rules! impl_sqrt_ufixed_concrete {
    ($strategy:ty, $($i:literal),+) => {
        $(
            impl Sqrt for UFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                type Output = Self;
                #[inline(always)]
                fn sqrt(self) -> Self {
                    Self::from_raw(self.to_raw().isqrt())
                }
            }
        )+
    };
}

// Hot: integer UFixed<I, 0, Hot> across I = 1..=64.
impl_sqrt_ufixed_concrete!(
    Hot, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);
impl_sqrt_ufixed_concrete!(
    Cold, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);
impl_sqrt_ufixed_concrete!(
    Warm, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32
);
impl_sqrt_ufixed_concrete!(
    Precise, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

// Float sqrt. `f32::sqrt` / `f64::sqrt` are std-only; we use a
// Newton-Raphson iteration with a bit-manipulated seed to stay
// no_std without libm.
//
// # WARNING: 7-ULP Newton-Raphson approximation
//
// Three (f32) / four (f64) NR iterations land within ~7 ULP of the
// correctly-rounded IEEE 754 result. The `StrictFloat` bit-exact
// contract is therefore weakened in no_std builds to "deterministic
// but not correctly-rounded". Consumer code that needs a correctly-
// rounded sqrt must link libm at a higher layer until the substrate
// BACKLOG item ships (see arvo/BACKLOG.md — "correctly-rounded sqrt
// via libm feature gate").

#[inline(always)]
fn sqrt_f32(x: f32) -> f32 {
    if x < 0.0 || x.is_nan() {
        return f32::NAN;
    }
    if x == 0.0 {
        return x;
    }
    let bits = x.to_bits();
    let guess_bits = (bits >> 1) + (0x3f80_0000u32 >> 1);
    let mut g = f32::from_bits(guess_bits);
    g = 0.5 * (g + x / g);
    g = 0.5 * (g + x / g);
    g = 0.5 * (g + x / g);
    g
}

#[inline(always)]
fn sqrt_f64(x: f64) -> f64 {
    if x < 0.0 || x.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return x;
    }
    let bits = x.to_bits();
    let guess_bits = (bits >> 1) + (0x3ff0_0000_0000_0000u64 >> 1);
    let mut g = f64::from_bits(guess_bits);
    g = 0.5 * (g + x / g);
    g = 0.5 * (g + x / g);
    g = 0.5 * (g + x / g);
    g = 0.5 * (g + x / g);
    g
}

impl Sqrt for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f32(self.0))
    }
}

impl Sqrt for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f64(self.0))
    }
}

impl Sqrt for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f32(self.0))
    }
}

impl Sqrt for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f64(self.0))
    }
}

// --- Recip -----------------------------------------------------------------

impl Recip for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f32 / self.0)
    }
}

impl Recip for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f64 / self.0)
    }
}

impl Recip for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f32 / self.0)
    }
}

impl Recip for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f64 / self.0)
    }
}

// --- Abs -------------------------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> Abs for UFixed<I, F, S>
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
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
            impl Abs for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
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
            impl Abs for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
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
            impl Abs for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
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
            impl Abs for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
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

impl_abs_ifixed_integer_sat!(Precise, i16, 1, 2, 3, 4, 5, 6, 7);
impl_abs_ifixed_integer_sat!(Precise, i32, 8, 9, 10, 11, 12, 13, 14, 15);
impl_abs_ifixed_integer_sat!(
    Precise, i64, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
#[rustfmt::skip]
impl_abs_ifixed_integer_sat!(
    Precise, i64,
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

// Float abs. No_std-compatible via sign-bit clear.

#[inline(always)]
fn abs_f32(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffffu32)
}

#[inline(always)]
fn abs_f64(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffffu64)
}

impl Abs for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f32(self.0))
    }
}

impl Abs for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f64(self.0))
    }
}

impl Abs for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f32(self.0))
    }
}

impl Abs for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f64(self.0))
    }
}

// --- FromConstant ---------------------------------------------------------
//
// For UFixed / IFixed: `n` is placed at the integer-bit position
// (multiplied by 2^F). Per-(strategy, BITS) concrete impls avoid the
// const-expr cycle a container-associated-type bound would create.

macro_rules! impl_from_constant_ufixed {
    ($strategy:ty, $ctype:ty, $($i:literal),+) => {
        $(
            // F = 0 (integer UFixed).
            impl FromConstant for UFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw(C.0 as $ctype)
                }
            }
        )+
    };
}

macro_rules! impl_from_constant_ufixed_fractional {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl FromConstant for UFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw((C.0 as $ctype) << $f)
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
#[rustfmt::skip]
impl_from_constant_ufixed!(
    Precise, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Fractional UFixed: a representative slice at I=8 for common F
// widths. Full coverage of every (I, F) pair would balloon; the
// tests exercise representative shapes and consumers compose their
// own wrappers. The trait surface is the guarantee — specific
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
            impl FromConstant for IFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw(C.0 as $ctype)
                }
            }
        )+
    };
}

macro_rules! impl_from_constant_ifixed_fractional {
    ($strategy:ty, $ctype:ty, $i:literal, $($f:literal),+) => {
        $(
            impl FromConstant for IFixed<{ ibits($i) }, { fbits($f) }, $strategy> {
                #[inline(always)]
                fn from_constant<const C: USize>() -> Self {
                    Self::from_raw((C.0 as $ctype) << $f)
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
#[rustfmt::skip]
impl_from_constant_ifixed!(
    Precise, i64,
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

impl<F: Ieee + FromU8Ieee> FromConstant for FastFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        FastFloat(<F as FromU8Ieee>::from_u8_ieee(C.0 as u8)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
    }
}

impl<F: Ieee + FromU8Ieee> FromConstant for StrictFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        StrictFloat(<F as FromU8Ieee>::from_u8_ieee(C.0 as u8)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
    }
}

/// Lossless `u8 -> IEEE float` bridge.
pub trait FromU8Ieee: Ieee {
    /// Convert the given `u8` into this IEEE float type.
    fn from_u8_ieee(n: u8) -> Self;
}

impl FromU8Ieee for f32 {
    #[inline(always)]
    fn from_u8_ieee(n: u8) -> Self {
        n as f32
    }
}

impl FromU8Ieee for f64 {
    #[inline(always)]
    fn from_u8_ieee(n: u8) -> Self {
        n as f64
    }
}
