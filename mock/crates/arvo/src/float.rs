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
use crate::strategy::{
    Additive, Bounded, ConstBitEq, ConstDefault, ConstPartialEq, Identity, Ieee,
};
use arvo_storage::Bool;

// Ieee + FromU8Ieee live in arvo-strategy::ieee per round 202605030400.
// The arvo facade re-exports both at lib.rs level; consumer code
// keeps the existing arvo::Ieee / arvo::FromU8Ieee paths.

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

const impl<F: Ieee> FloatLike for FastFloat<F> {}
const impl<F: Ieee> FloatLike for StrictFloat<F> {}

// SAFETY: `repr(transparent)` over `F` (which is `f32` or `f64` via
// the `Ieee` seal). Layout-identical by Rust spec.
const unsafe impl<F: Ieee> arvo_transparent::Transparent for FastFloat<F> {
    type Inner = F;
}

const unsafe impl<F: Ieee> arvo_transparent::Transparent for StrictFloat<F> {
    type Inner = F;
}

// --- Canonical const surfaces for FastFloat / StrictFloat ----------------
//
// `Bounded`, `Identity`, `ConstPartialEq`, `ConstBitEq`,
// `ConstDefault` blanket impls keyed on the inner `F: [const] Ieee`
// projection.
//
// `ConstEq` (the reflexivity-promising marker) is deliberately NOT
// implemented for FastFloat / StrictFloat because NaN breaks
// reflexivity (`NaN.const_eq(&NaN) == FALSE` under PartialEq).
// Consumers who need reflexive equality on float values reach for
// `ConstBitEq` (bitwise equality, always reflexive). Consumers who
// need partial equality reach for `ConstPartialEq`. Total ordering on
// floats routes through `arvo-numeric-contracts::TotalOrd`.
//
// Note on `Identity::<Additive>::IDENTITY` value-vs-formal-additive-identity under
// fast-math: `FastFloat<F>::ZERO = 0.0` is the *value* zero. Under
// LLVM fast-math passes (signed-zero-ignoring), `x + 0.0` may be
// rewritten to `x` even for `x = -0.0`; the additive-identity
// property is preserved up to fast-math's signed-zero collapse,
// which is the documented contract of `FastFloat`.

const impl<F: [const] Ieee> Bounded for FastFloat<F> {
    const MIN: Self = Self(<F as Bounded>::MIN);
    const MAX: Self = Self(<F as Bounded>::MAX);
}

const impl<F: [const] Ieee + [const] Identity<Op>, Op> Identity<Op> for FastFloat<F> {
    const IDENTITY: Self = Self(<F as Identity<Op>>::IDENTITY);
}

const impl<F: [const] Ieee + [const] ConstPartialEq> ConstPartialEq for FastFloat<F> {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <F as ConstPartialEq>::const_eq(&a, &b)
    }
}

const impl<F: [const] Ieee + [const] ConstBitEq> ConstBitEq for FastFloat<F> {
    #[inline(always)]
    fn const_bit_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <F as ConstBitEq>::const_bit_eq(&a, &b)
    }
}

const impl<F: [const] Ieee> ConstDefault for FastFloat<F> {
    #[inline(always)]
    fn const_default() -> Self {
        Self(<F as Identity<Additive>>::IDENTITY)
    }
}

const impl<F: [const] Ieee> Bounded for StrictFloat<F> {
    const MIN: Self = Self(<F as Bounded>::MIN);
    const MAX: Self = Self(<F as Bounded>::MAX);
}

const impl<F: [const] Ieee + [const] Identity<Op>, Op> Identity<Op> for StrictFloat<F> {
    const IDENTITY: Self = Self(<F as Identity<Op>>::IDENTITY);
}

const impl<F: [const] Ieee + [const] ConstPartialEq> ConstPartialEq for StrictFloat<F> {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <F as ConstPartialEq>::const_eq(&a, &b)
    }
}

const impl<F: [const] Ieee + [const] ConstBitEq> ConstBitEq for StrictFloat<F> {
    #[inline(always)]
    fn const_bit_eq(&self, other: &Self) -> Bool {
        let a = <Self as arvo_transparent::Transparent>::raw(*self);
        let b = <Self as arvo_transparent::Transparent>::raw(*other);
        <F as ConstBitEq>::const_bit_eq(&a, &b)
    }
}

const impl<F: [const] Ieee> ConstDefault for StrictFloat<F> {
    #[inline(always)]
    fn const_default() -> Self {
        Self(<F as Identity<Additive>>::IDENTITY)
    }
}

// --- core::ops arithmetic -------------------------------------------------
//
// Delegating impls for the five arith ops on both wrappers. The `F: Ieee`
// bound plus the per-op `<Output = F>` bound is sufficient; f32 and f64
// satisfy all of these in `core::ops`, so the Ieee seal is preserved.

macro_rules! float_binop_impl {
    ($wrapper:ident, $op:ident, $method:ident) => {
        const impl<F: Ieee + [const] core::ops::$op<Output = F>> core::ops::$op for $wrapper<F> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, other: Self) -> Self {
                Self(<F as core::ops::$op>::$method(self.0, other.0))
            }
        }
    };
}

macro_rules! float_neg_impl {
    ($wrapper:ident) => {
        const impl<F: Ieee + [const] core::ops::Neg<Output = F>> core::ops::Neg for $wrapper<F> {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self {
                Self(<F as core::ops::Neg>::neg(self.0))
            }
        }
    };
}

float_binop_impl!(FastFloat, Add, add);
float_binop_impl!(FastFloat, Sub, sub);
float_binop_impl!(FastFloat, Mul, mul);
float_binop_impl!(FastFloat, Div, div);
float_neg_impl!(FastFloat);

float_binop_impl!(StrictFloat, Add, add);
float_binop_impl!(StrictFloat, Sub, sub);
float_binop_impl!(StrictFloat, Mul, mul);
float_binop_impl!(StrictFloat, Div, div);
float_neg_impl!(StrictFloat);

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
