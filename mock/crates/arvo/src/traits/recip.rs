//! Reciprocal, floats only.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::float::{FastFloat, StrictFloat};
pub use arvo_numeric_contracts::Recip;
use arvo_transparent::Transparent;

// --- Recip -----------------------------------------------------------------

const impl Recip for FastFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f32 / <Self as Transparent>::raw(self))
    }
}

const impl Recip for FastFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        FastFloat(1.0f64 / <Self as Transparent>::raw(self))
    }
}

const impl Recip for StrictFloat<f32> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f32 / <Self as Transparent>::raw(self))
    }
}

const impl Recip for StrictFloat<f64> {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        StrictFloat(1.0f64 / <Self as Transparent>::raw(self))
    }
}
