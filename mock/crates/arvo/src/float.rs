//! IEEE float wrappers.
//!
//! Floats are a separate type family from fixed-point: no strategy
//! marker. `FastFloat<F>` enables fast-math semantics (reassociation,
//! reciprocal approximation, no NaN propagation) on its operations.
//! `StrictFloat<F>` holds IEEE 754 bit-exact semantics.
//!
//! `Float<F>` is a cfg-resolved alias: `FastFloat<F>` when the
//! `arvo_fast_math` cfg is active (set by hilavitkutin-build when the
//! FastMath pragma is selected), `StrictFloat<F>` otherwise.
//!
//! The float width parameter `F` is sealed: only `f32` and `f64`
//! implement `Ieee`.

use crate::markers::FloatLike;

mod sealed {
    /// Hidden supertrait used to seal `Ieee`.
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// IEEE float width marker. Sealed: implementable only for `f32`
/// and `f64`.
pub trait Ieee: sealed::Sealed + Copy + Default + PartialEq + PartialOrd + 'static {
    /// Width of this IEEE type in bits.
    const WIDTH: u8;
    /// Zero value of this float.
    const ZERO: Self;
    /// One (multiplicative identity) of this float.
    const ONE: Self;
}

impl Ieee for f32 {
    const WIDTH: u8 = 32;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Ieee for f64 {
    const WIDTH: u8 = 64;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

/// Fast-math IEEE wrapper.
///
/// Operations on `FastFloat<F>` may be reassociated, may use
/// reciprocal approximation, and do not guarantee NaN propagation.
/// LLVM fast-math flags apply to arithmetic through this type.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct FastFloat<F: Ieee>(pub F);

/// Strict IEEE 754 float wrapper.
///
/// Bit-exact, NaN-propagating, order-preserving. Used where
/// reproducibility matters (comparison, validation, user-visible
/// numeric output).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct StrictFloat<F: Ieee>(pub F);

impl<F: Ieee> FastFloat<F> {
    /// Wrap an IEEE float in fast-math semantics.
    #[inline(always)]
    pub const fn new(f: F) -> Self {
        Self(f)
    }

    /// Extract the wrapped IEEE float.
    #[inline(always)]
    pub const fn into_inner(self) -> F {
        self.0
    }
}

impl<F: Ieee> StrictFloat<F> {
    /// Wrap an IEEE float in strict IEEE 754 semantics.
    #[inline(always)]
    pub const fn new(f: F) -> Self {
        Self(f)
    }

    /// Extract the wrapped IEEE float.
    #[inline(always)]
    pub const fn into_inner(self) -> F {
        self.0
    }
}

impl<F: Ieee> FloatLike for FastFloat<F> {}
impl<F: Ieee> FloatLike for StrictFloat<F> {}

/// Resolved `Float` alias.
///
/// `FastFloat<F>` when the `arvo_fast_math` cfg is active;
/// `StrictFloat<F>` otherwise. The default IEEE width is `f32`.
#[cfg(arvo_fast_math)]
pub type Float<F = f32> = FastFloat<F>;

/// Resolved `Float` alias.
///
/// `StrictFloat<F>` outside the `arvo_fast_math` cfg. The default
/// IEEE width is `f32`.
#[cfg(not(arvo_fast_math))]
pub type Float<F = f32> = StrictFloat<F>;
