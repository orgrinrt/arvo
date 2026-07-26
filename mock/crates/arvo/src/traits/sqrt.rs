//! Square root, where it is unambiguous.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::float::{FastFloat, StrictFloat};
use crate::strategy::{Cold, Hot, Precise, Warm};
use crate::ufixed::UFixed;
pub use arvo_numeric_contracts::Sqrt;
use arvo_storage::{ibits, FBits};
use arvo_transparent::Transparent;

// --- Sqrt ------------------------------------------------------------------
//
// Integer UFixed (F == 0) uses `u*::isqrt`. Fractional UFixed is out
// of scope for this round. We spell out one impl per `(strategy, I)`
// pair so each impl has a concrete container type, which avoids the
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
// BACKLOG item ships (see arvo/BACKLOG.md, "correctly-rounded sqrt
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

const impl Sqrt for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f32(<Self as Transparent>::raw(self)))
    }
}

const impl Sqrt for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        FastFloat(sqrt_f64(<Self as Transparent>::raw(self)))
    }
}

const impl Sqrt for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f32(<Self as Transparent>::raw(self)))
    }
}

const impl Sqrt for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        StrictFloat(sqrt_f64(<Self as Transparent>::raw(self)))
    }
}
