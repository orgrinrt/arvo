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

use crate::float::{FastFloat, StrictFloat};
use crate::ifixed::IFixed;
pub use arvo_numeric_contracts::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
use arvo_storage::{FBits, IBits, USize, fbits, ibits};
use arvo_transparent::Transparent;
use crate::strategy::{
    BitsContainerFor, Cold, FromU8Ieee, Hot, Ieee, Precise, ScalarEuclidRaw, Signed, Strategy,
    UScalarEuclidRaw, Unsigned, Warm, ifixed_bits, ufixed_bits,
};
use crate::fixed_scale::{FracShift, frac};
use crate::ufixed::UFixed;

// --- TotalOrd --------------------------------------------------------------
//
// Bodies use direct `<` / `>` / `==` comparison rather than the inherent
// `cmp` / `total_cmp` methods because those are not const-stable on
// rustc 1.96.0-nightly. The bare-primitive comparison operators ARE
// const-stable for every integer width, and the float bit-reinterpret
// XOR trick (see `total_cmp_f32` / `total_cmp_f64`) gives the same
// total ordering as `f*::total_cmp` while staying const-callable.

/// Const-callable equivalent of `f32::total_cmp`. Reinterprets the float
/// bits as i32, applies the standard XOR mask so positive floats sort
/// after negative floats by bit pattern, then compares as signed.
#[inline(always)]
const fn total_cmp_f32(a: f32, b: f32) -> Ordering {
    let mut left = a.to_bits() as i32;
    let mut right = b.to_bits() as i32;
    left ^= (((left >> 31) as u32) >> 1) as i32;
    right ^= (((right >> 31) as u32) >> 1) as i32;
    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Const-callable equivalent of `f64::total_cmp`. Same XOR-mask trick
/// as `total_cmp_f32`, widened to i64.
#[inline(always)]
const fn total_cmp_f64(a: f64, b: f64) -> Ordering {
    let mut left = a.to_bits() as i64;
    let mut right = b.to_bits() as i64;
    left ^= (((left >> 63) as u64) >> 1) as i64;
    right ^= (((right >> 63) as u64) >> 1) as i64;
    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const TotalOrd for UFixed<I, F, S>
where
    S: [const] BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Self: [const] arvo_storage::ConstOrd,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        const_ordering_to_core(<Self as arvo_storage::ConstOrd>::const_cmp(&self, &other))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> const TotalOrd for IFixed<I, F, S>
where
    S: [const] BitsContainerFor<{ ifixed_bits(I, F) }, Signed>,
    Self: [const] arvo_storage::ConstOrd,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        const_ordering_to_core(<Self as arvo_storage::ConstOrd>::const_cmp(&self, &other))
    }
}

/// Const-callable bridge from substrate `ConstOrdering` to core
/// `Ordering`. Wraps the existing bidirectional `From` impl in a
/// const fn since `From::from` is not yet const-stable as a trait
/// method on rustc 1.96.0-nightly.
#[inline(always)]
const fn const_ordering_to_core(o: arvo_storage::ConstOrdering) -> Ordering {
    match o {
        arvo_storage::ConstOrdering::Less => Ordering::Less,
        arvo_storage::ConstOrdering::Equal => Ordering::Equal,
        arvo_storage::ConstOrdering::Greater => Ordering::Greater,
    }
}

impl const TotalOrd for FastFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f32(<Self as Transparent>::raw(self), <Self as Transparent>::raw(other))
    }
}

impl const TotalOrd for FastFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f64(<Self as Transparent>::raw(self), <Self as Transparent>::raw(other))
    }
}

impl const TotalOrd for StrictFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f32(<Self as Transparent>::raw(self), <Self as Transparent>::raw(other))
    }
}

impl const TotalOrd for StrictFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f64(<Self as Transparent>::raw(self), <Self as Transparent>::raw(other))
    }
}

// --- Sqrt ------------------------------------------------------------------
//
// Integer UFixed (F == 0) uses `u*::isqrt`. Fractional UFixed is out
// of scope for this round. We spell out one impl per `(strategy, I)`
// pair so each impl has a concrete container type — avoids the
// const-expr cycle that a blanket `where <S as BitsContainerFor<..., Unsigned>>::T:
// ...` produces.

macro_rules! impl_sqrt_ufixed_concrete {
    ($strategy:ty, $($i:literal),+) => {
        $(
            impl const Sqrt for UFixed<{ ibits($i) }, { FBits::ZERO }, $strategy> {
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

// `is_nan` is not const-stable on f32/f64 in rustc 1.96.0-nightly; the
// const-callable substitute is `x != x` (NaN is the only float that
// fails self-equality). The rest of the body (bit-reinterpretation,
// float arithmetic, comparison) is const-stable.
#[inline(always)]
pub const fn sqrt_f32(x: f32) -> f32 {
    if x < 0.0 || x != x {
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
pub const fn sqrt_f64(x: f64) -> f64 {
    if x < 0.0 || x != x {
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

impl const Sqrt for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f32(<Self as Transparent>::raw(self)))
    }
}

impl const Sqrt for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f64(<Self as Transparent>::raw(self)))
    }
}

impl const Sqrt for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f32(<Self as Transparent>::raw(self)))
    }
}

impl const Sqrt for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f64(<Self as Transparent>::raw(self)))
    }
}

// --- Recip -----------------------------------------------------------------

impl const Recip for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f32 / <Self as Transparent>::raw(self))
    }
}

impl const Recip for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f64 / <Self as Transparent>::raw(self))
    }
}

impl const Recip for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f32 / <Self as Transparent>::raw(self))
    }
}

impl const Recip for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f64 / <Self as Transparent>::raw(self))
    }
}

// --- Abs -------------------------------------------------------------------

impl<const I: IBits, const F: FBits, S: Strategy> const Abs for UFixed<I, F, S>
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

impl const Abs for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f32(<Self as Transparent>::raw(self)))
    }
}

impl const Abs for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        FastFloat(abs_f64(<Self as Transparent>::raw(self)))
    }
}

impl const Abs for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f32(<Self as Transparent>::raw(self)))
    }
}

impl const Abs for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self {
        StrictFloat(abs_f64(<Self as Transparent>::raw(self)))
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

impl<F: [const] Ieee + [const] FromU8Ieee> const FromConstant for FastFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
        FastFloat(<F as FromU8Ieee>::from_u8_ieee(<USize as Transparent>::raw(C) as u8))
    }
}

impl<F: [const] Ieee + [const] FromU8Ieee> const FromConstant for StrictFloat<F> {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal `FromU8Ieee` bridge takes u8 by design; USize→u8 cast preserves IEEE lossless range for 0..=255; tracked: #123
        StrictFloat(<F as FromU8Ieee>::from_u8_ieee(<USize as Transparent>::raw(C) as u8))
    }
}

// FromU8Ieee lives in arvo-strategy::ieee per round 202605030400.
// The arvo facade re-exports it at lib.rs level; consumer code keeps
// the existing arvo::FromU8Ieee path. The FromConstant impls above
// reach the trait via the crate::FromU8Ieee re-export path.

// --- Float ConstSign impls — feed the predicate-wrapper blankets --------
//
// arvo-numeric-contracts ships `IsPositiveOf<T>` / `IsNonNegativeOf<T>`
// / `IsZeroOrPositiveOf<T>` blanket Predicate impls bound on `T:
// [const] ConstSign`. Fixed types pick up ConstSign automatically via
// the ConstOrd+Identity blanket. Floats opt out of ConstOrd (NaN
// breaks reflexivity), so they need direct ConstSign impls. Bodies
// use bare-primitive `>` / `>=` against 0.0 — const-callable on
// f32/f64. NaN compares as not-greater and not-greater-or-equal,
// consistently returning false for all sign predicates, which is the
// intended semantic.

use arvo_numeric_contracts::ConstSign;
use arvo_storage::Bool;

macro_rules! float_const_sign_impl {
    ($wrapper:ident, $inner:ty, $zero:expr) => {
        impl const ConstSign for $wrapper<$inner> {
            #[inline(always)]
            fn is_positive(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) > $zero)
            }
            #[inline(always)]
            fn is_non_negative(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) >= $zero)
            }
            #[inline(always)]
            fn is_zero_or_positive(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) >= $zero)
            }
        }
    };
}

float_const_sign_impl!(FastFloat, f32, 0.0_f32);
float_const_sign_impl!(FastFloat, f64, 0.0_f64);
float_const_sign_impl!(StrictFloat, f32, 0.0_f32);
float_const_sign_impl!(StrictFloat, f64, 0.0_f64);

// --- ScalarEuclid / EuclidDiv / EvenSplittable (round 202606232230) -----
//
// Small separate single-method-family traits, blanket-impl'd over the
// fixed-point types via the inner `ScalarEuclidRaw` / `UScalarEuclidRaw`
// contract (the array-`Capacity`-contract pattern). The raw container
// arithmetic stays inside the contract; this surface is raw-free.

/// Euclidean division of a fixed-point value by an integer count.
///
/// `div_euclid_scalar` / `rem_euclid_scalar` divide at ULP granularity:
/// the base share and its remainder for an as-equal-as-possible split
/// into `n` parts. Contrast `EuclidDiv`, which floors to whole units.
pub trait ScalarEuclid: Sized {
    /// Euclidean quotient by an integer count (ULP granularity).
    fn div_euclid_scalar(self, n: USize) -> Self;
    /// Euclidean remainder by an integer count (the leftover ULPs).
    fn rem_euclid_scalar(self, n: USize) -> Self;
}

/// std-parity euclidean division by a `Self` divisor.
///
/// The quotient is floored to whole integer units (the coarser,
/// conserving variant, distinct from `ScalarEuclid`'s ULP split).
pub trait EuclidDiv: Sized {
    /// Whole-unit euclidean quotient by a fixed-point divisor.
    fn div_euclid(self, rhs: Self) -> Self;
    /// Euclidean remainder by a fixed-point divisor (the fixed leftover).
    fn rem_euclid(self, rhs: Self) -> Self;
}

/// Lazy iterator over `n` exact-conserving even shares of a value.
///
/// Yields `base + 1 ULP` for the first `r` shares (the remainder
/// distributed by index) then `base`, for `n` shares total summing to
/// the original. No allocation.
pub struct EvenShares<T> {
    base: T,
    base_plus: T,
    r: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
    n: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
    i: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
}

impl<T: Copy> Iterator for EvenShares<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { // lint:allow(no-bare-option) reason: std Iterator::next signature; tracked: #256
        if self.i >= self.n {
            return None;
        }
        let v = if self.i < self.r { self.base_plus } else { self.base };
        self.i += 1;
        Some(v)
    }
}

/// Split a value into `n` shares that sum to it exactly.
pub trait EvenSplittable: Sized {
    /// A no-alloc iterator of `n` exact-conserving shares.
    fn split_evenly(self, n: USize) -> EvenShares<Self>;
}

impl<const I: IBits, const F: FBits, S: Strategy> ScalarEuclid for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
{
    #[inline]
    fn div_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(<S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::div_euclid_scalar(
            self.to_raw(),
            n.0,
        ))
    }
    #[inline]
    fn rem_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(<S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::rem_euclid_scalar(
            self.to_raw(),
            n.0,
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EuclidDiv for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        Self::from_raw(<S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::div_euclid_whole::<
            { frac(F) },
        >(self.to_raw(), rhs.to_raw()))
    }
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        Self::from_raw(<S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::rem_euclid_whole::<
            { frac(F) },
        >(self.to_raw(), rhs.to_raw()))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EvenSplittable for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
{
    #[inline]
    fn split_evenly(self, n: USize) -> EvenShares<Self> {
        let (base, base_plus, r) =
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::even_split_parts(self.to_raw(), n.0);
        EvenShares { base: Self::from_raw(base), base_plus: Self::from_raw(base_plus), r, n: n.0, i: 0 }
    }
}

#[cfg(test)]
mod euclid_split_tests {
    use super::*;
    use crate::ifixed::IFixed;

    // A non-power-of-two-width signed fixed-point: 13 integer bits, 16
    // fractional, Hot. Logical width 30 -> i32 container.
    type Q = IFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    fn q(raw: i32) -> Q { // lint:allow(no-bare-numeric) reason: test raw constructor over the i32 container; tracked: #256
        Q::from_raw(raw)
    }

    fn sum(shares: &[Q]) -> Q {
        let mut acc = q(0);
        let mut k = 0; // lint:allow(no-bare-numeric) reason: test loop cursor; tracked: #256
        while k < shares.len() {
            acc = acc + shares[k];
            k += 1;
        }
        acc
    }

    #[test]
    fn split_evenly_conserves_indivisible() {
        // 100 raw over 3: 34 + 33 + 33 == 100 exactly.
        let shares: [Q; 3] = {
            let mut it = q(100).split_evenly(USize(3));
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        };
        assert_eq!(it_none(q(100).split_evenly(USize(3))), 3);
        assert_eq!(sum(&shares).to_raw(), 100);
        assert_eq!(shares[0].to_raw(), 34);
        assert_eq!(shares[1].to_raw(), 33);
        assert_eq!(shares[2].to_raw(), 33);
    }

    fn it_none(mut it: EvenShares<Q>) -> usize { // lint:allow(no-bare-numeric) reason: test count of yielded shares; tracked: #256
        let mut c = 0; // lint:allow(no-bare-numeric) reason: test counter; tracked: #256
        while it.next().is_some() {
            c += 1;
        }
        c
    }

    #[test]
    fn split_evenly_deterministic() {
        let a = it_collect(q(67).split_evenly(USize(5)));
        let b = it_collect(q(67).split_evenly(USize(5)));
        assert_eq!(a, b);
        // 67 over 5: 14,14,13,13,13 -> sum 67.
        assert_eq!(a.iter().map(|s| s.to_raw()).sum::<i32>(), 67);
    }

    fn it_collect(mut it: EvenShares<Q>) -> [Q; 5] {
        [
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        ]
    }

    #[test]
    fn split_evenly_conserves_negative() {
        // a fixed-point value can be negative; euclidean split still conserves.
        let shares = it_collect3(q(-100).split_evenly(USize(3)));
        assert_eq!(shares.iter().map(|s| s.to_raw()).sum::<i32>(), -100);
    }

    fn it_collect3(mut it: EvenShares<Q>) -> [Q; 3] {
        [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
    }

    #[test]
    fn scalar_euclid_base_remainder_identity() {
        // base*n + remainder == original (ULP granularity).
        let base = q(100).div_euclid_scalar(USize(3));
        let rem = q(100).rem_euclid_scalar(USize(3));
        assert_eq!(base.to_raw(), 33);
        assert_eq!(rem.to_raw(), 1);
    }

    #[test]
    fn euclid_div_whole_unit() {
        // whole-unit divisor: 100.0 / 3.0 -> quotient floored to 33.0 (whole units),
        // remainder 1.0. raw: one unit == 1 << 16.
        let one = 1i32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = q(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = q(3 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        assert_eq!(a.div_euclid(b).to_raw(), 33 * one); // 33.0 whole units
        assert_eq!(a.rem_euclid(b).to_raw(), one); // 1.0 leftover
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> ScalarEuclid for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
{
    #[inline]
    fn div_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(<S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::div_euclid_scalar(
            self.to_raw(),
            n.0,
        ))
    }
    #[inline]
    fn rem_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(<S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::rem_euclid_scalar(
            self.to_raw(),
            n.0,
        ))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EuclidDiv for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        Self::from_raw(<S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::div_euclid_whole::<
            { frac(F) },
        >(self.to_raw(), rhs.to_raw()))
    }
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        Self::from_raw(<S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::rem_euclid_whole::<
            { frac(F) },
        >(self.to_raw(), rhs.to_raw()))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EvenSplittable for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
{
    #[inline]
    fn split_evenly(self, n: USize) -> EvenShares<Self> {
        let (base, base_plus, r) =
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::even_split_parts(self.to_raw(), n.0);
        EvenShares { base: Self::from_raw(base), base_plus: Self::from_raw(base_plus), r, n: n.0, i: 0 }
    }
}

#[cfg(test)]
mod ueuclid_split_tests {
    use super::*;
    use crate::ufixed::UFixed;

    // 13 integer bits, 16 fractional, Hot. Logical width 29 -> u32 container.
    type U = UFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    fn u(raw: u32) -> U { // lint:allow(no-bare-numeric) reason: test raw constructor over the u32 container; tracked: #256
        U::from_raw(raw)
    }

    #[test]
    fn usplit_conserves_indivisible() {
        let mut it = u(100).split_evenly(USize(3));
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        let c = it.next().unwrap();
        assert!(it.next().is_none());
        assert_eq!(a.to_raw() + b.to_raw() + c.to_raw(), 100);
        assert_eq!(a.to_raw(), 34); // first share carries the remainder ULP
        assert_eq!(b.to_raw(), 33);
        assert_eq!(c.to_raw(), 33);
    }

    #[test]
    fn uscalar_euclid_identity() {
        assert_eq!(u(100).div_euclid_scalar(USize(3)).to_raw(), 33);
        assert_eq!(u(100).rem_euclid_scalar(USize(3)).to_raw(), 1);
    }

    #[test]
    fn ueuclid_div_whole_unit() {
        let one = 1u32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = u(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = u(3 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        assert_eq!(a.div_euclid(b).to_raw(), 33 * one);
        assert_eq!(a.rem_euclid(b).to_raw(), one);
    }
}

#[cfg(test)]
mod euclid_overflow_tests {
    use super::*;
    use crate::ifixed::IFixed;
    use crate::ufixed::UFixed;

    // Width-8 containers: IFixed<7,0> -> i8 (logical 1+7+0=8), UFixed<8,0> -> u8.
    type Q8 = IFixed<{ ibits(7) }, { fbits(0) }, Hot>;
    type U8 = UFixed<{ ibits(8) }, { fbits(0) }, Hot>;
    type Q = IFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    #[test]
    fn split_one_at_signed_max_does_not_overflow() {
        // base == a == i8::MAX; the old eager `base + 1` panicked in debug. A 1-way split is the value.
        let mut it = Q8::from_raw(i8::MAX).split_evenly(USize(1)); // lint:allow(no-bare-numeric) reason: test raw MAX over the i8 container; tracked: #256
        assert_eq!(it.next().unwrap().to_raw(), i8::MAX);
        assert!(it.next().is_none());
    }

    #[test]
    fn split_one_at_unsigned_max_does_not_overflow() {
        let mut it = U8::from_raw(u8::MAX).split_evenly(USize(1)); // lint:allow(no-bare-numeric) reason: test raw MAX over the u8 container; tracked: #256
        assert_eq!(it.next().unwrap().to_raw(), u8::MAX);
        assert!(it.next().is_none());
    }

    #[test]
    fn euclid_div_subunit_divisor_saturates() {
        // 100.0 div_euclid (1 ULP): whole quotient ~6.5e6, shifted back overflows i32 -> saturates, no panic.
        let one = 1i32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = Q::from_raw(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = Q::from_raw(1); // 1 ULP, a sub-unit divisor
        assert_eq!(a.div_euclid(b).to_raw(), i32::MAX);
    }
}
