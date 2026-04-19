//! Shared test helpers for arvo-spectral integration tests.
//!
//! Ships a `TF` (Test Float) newtype wrapping `f32` with all the
//! arvo numeric traits power_iteration / fiedler_vector require.
//! Lives in the test crate only; not part of the shipping API.

#![allow(dead_code)]

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};

/// Test-only float newtype over `f32`.
///
/// Implements every trait the arvo-spectral public surface requires
/// of `F`. Test crates cannot impl the arvo traits on `f32` directly
/// (orphan rule), so a local newtype is the minimal path.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct TF(pub f32);

impl TF {
    pub const ZERO: Self = TF(0.0);
    pub const ONE: Self = TF(1.0);
}

impl Add for TF {
    type Output = TF;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        TF(self.0 + rhs.0)
    }
}

impl Sub for TF {
    type Output = TF;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        TF(self.0 - rhs.0)
    }
}

impl Mul for TF {
    type Output = TF;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        TF(self.0 * rhs.0)
    }
}

impl Sqrt for TF {
    #[inline(always)]
    fn sqrt(self) -> Self {
        // no_std sqrt via Newton-Raphson, matches arvo's internal fn.
        if self.0 < 0.0 || self.0.is_nan() {
            return TF(f32::NAN);
        }
        if self.0 == 0.0 {
            return self;
        }
        let bits = self.0.to_bits();
        let guess_bits = (bits >> 1) + (0x3f80_0000u32 >> 1);
        let mut g = f32::from_bits(guess_bits);
        let x = self.0;
        g = 0.5 * (g + x / g);
        g = 0.5 * (g + x / g);
        g = 0.5 * (g + x / g);
        g = 0.5 * (g + x / g);
        g = 0.5 * (g + x / g);
        TF(g)
    }
}

impl Recip for TF {
    #[inline(always)]
    fn recip(self) -> Self {
        TF(1.0_f32 / self.0)
    }
}

impl TotalOrd for TF {
    #[inline(always)]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl FromConstant for TF {
    #[inline(always)]
    fn from_constant(n: u8) -> Self {
        TF(n as f32)
    }
}
