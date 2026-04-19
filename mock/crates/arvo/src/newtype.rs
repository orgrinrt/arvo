//! Primitive newtypes.
//!
//! Small, transparent wrappers around raw Rust primitives used in the
//! arvo public API. Keeping them distinct lets downstream code state
//! intent at type level without paying for a runtime wrapper: every
//! newtype here is `repr(transparent)` or a single-field struct with
//! no extra state.
//!
//! The types split into two kinds:
//!
//! - Const-generic carriers: `IBits`, `FBits`, `USize`, `Cap`. These
//!   derive `ConstParamTy` so they can appear as const-generic
//!   parameters (for example on `UFixed<const I: IBits, ...>`).
//! - Value newtypes: `Bool`. Carries a `bool` through the API. Has
//!   `Deref<Target = bool>` and `Try<Output = bool>` so control flow
//!   stays ergonomic.

use core::convert::Infallible;
use core::marker::ConstParamTy;
use core::ops::{ControlFlow, Deref, FromResidual, Try};

/// Integer-bit count for fixed-point types.
///
/// Carries the integer bit width through type-level code. Distinct
/// from `FBits` so consumers cannot accidentally swap the two at
/// construction sites.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct IBits(pub u8);

/// Fractional-bit count for fixed-point types.
///
/// Carries the fractional bit width through type-level code. See
/// `IBits` for the integer counterpart.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct FBits(pub u8);

// `generic_const_exprs` on current nightly forbids struct construction
// and field access inside anonymous `{ ... }` const arguments. Any site
// that wants `IBits(0)` / `FBits(0)` etc. as a const-generic arg must
// reference one of these named constants.
impl IBits {
    /// Zero integer bits.
    pub const ZERO: IBits = IBits(0);
    /// One integer bit (used by the `Bit` alias in arvo-bits).
    pub const ONE: IBits = IBits(1);
}

impl FBits {
    /// Zero fractional bits (pure integer).
    pub const ZERO: FBits = FBits(0);
    /// One fractional bit.
    pub const ONE: FBits = FBits(1);
}

/// Index / count newtype wrapping `usize`.
///
/// Wraps `usize` for the arvo-types-only lint. `Deref<Target = usize>`
/// gives `array[*idx]` ergonomics. Derives `ConstParamTy` so `USize`
/// values can be used inside other const-generic newtypes (see `Cap`).
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct USize(pub usize);

impl Deref for USize {
    type Target = usize;
    #[inline(always)]
    fn deref(&self) -> &usize {
        &self.0
    }
}

/// Const-generic capacity for fixed-size structures.
///
/// Used wherever a const generic sizes an array. The type prevents
/// mixing capacities with unrelated integers. The const parameter
/// name carries the semantic distinction (`N`, `ROWS`, `NNZ`).
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Cap(pub USize);

/// Control-flow boolean.
///
/// Returned by predicates (`is_zero`, `bit`). Not a fixed-point
/// type — `Bool` is for branching, `Bit` (defined in arvo-bits) is
/// for 1-bit data storage.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Bool(pub bool);

impl Bool {
    /// Constant `Bool(true)`.
    pub const TRUE: Bool = Bool(true);
    /// Constant `Bool(false)`.
    pub const FALSE: Bool = Bool(false);
}

impl Deref for Bool {
    type Target = bool;
    #[inline(always)]
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl Try for Bool {
    type Output = bool;
    type Residual = Infallible;

    #[inline(always)]
    fn from_output(output: bool) -> Self {
        Bool(output)
    }

    #[inline(always)]
    fn branch(self) -> ControlFlow<Infallible, bool> {
        ControlFlow::Continue(self.0)
    }
}

impl FromResidual<Infallible> for Bool {
    #[inline(always)]
    fn from_residual(residual: Infallible) -> Self {
        match residual {}
    }
}

/// Bridge trait for code paths that need a raw `bool`.
///
/// Preferred path in WU code is `?`; `as_bool()` exists for boundary
/// compatibility with libraries that expose `bool` directly.
pub trait AsBool {
    /// Extract the inner `bool`.
    fn as_bool(&self) -> bool;
}

impl AsBool for Bool {
    #[inline(always)]
    fn as_bool(&self) -> bool {
        self.0
    }
}

impl From<bool> for Bool {
    #[inline(always)]
    fn from(b: bool) -> Self {
        Bool(b)
    }
}

impl From<Bool> for bool {
    #[inline(always)]
    fn from(b: Bool) -> Self {
        b.0
    }
}
