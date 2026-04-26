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

/// Generate a meta-bits ConstParamTy newtype wrapping `u8`, with
/// `Deref` / `AsRef` / `From` to/from `u8` and a lowercase const-fn
/// helper for const-generic-position construction.
///
/// DRYs the ergonomic surface of `IBits` / `FBits` / `Width`. The
/// underlying field is `u8` (not `Bits<7, Hot>`) until rustc resolves
/// the `<S as UContainerFor<N>>::T` projection cascade through the
/// `ConstParamTy_` ↔ `Sized` query graph; tracked alongside the
/// UFixed/IFixed ConstParamTy follow-up in `BACKLOG.md.tmpl`.
/// Bumping the inner type is a one-line macro change once the cycle
/// is resolvable; consumer call sites stay unchanged because the
/// helper signature (`fn $helper(n: u8) -> $W`) doesn't move.
/// Generate a meta-bits ConstParamTy newtype with the unified
/// `Transparent` ergonomic surface.
///
/// Each invocation produces:
/// - `pub struct $W(pub u8)` with full derive set + `ConstParamTy`
/// - `pub const ZERO` / `pub const ONE` value constants
/// - `pub const fn raw(self) -> u8` inherent (const-fn callable;
///   no trait import required at the call site)
/// - `unsafe impl Transparent for $W { type Inner = u8 }`
///   (enables `arvo::raw(w)` free fn and `Transparent::raw(w)`
///   trait method without per-type code)
/// - `Deref<Target = u8>` / `AsRef<u8>` / `From<u8>` / `From<$W> for u8`
/// - `pub const fn $helper(n: u8) -> $W` for const-generic-position
///   literal construction
///
/// The inherent `.raw()`, the trait `Transparent::raw()`, and the
/// free `arvo::raw()` all collapse to the same transmute at codegen.
macro_rules! meta_bits_wrapper {
    (
        $(#[$meta:meta])*
        $vis:vis struct $W:ident;
        helper $helper:ident;
    ) => {
        $(#[$meta])*
        #[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
        #[repr(transparent)]
        $vis struct $W(pub u8);

        impl $W {
            /// Zero value.
            pub const ZERO: Self = Self(0);
            /// One value.
            pub const ONE: Self = Self(1);

            /// Read the inner `u8`. See [`crate::transparent`] for the
            /// unified `repr(transparent)` access surface.
            #[inline(always)]
            pub const fn raw(self) -> u8 { self.0 }
        }

        // SAFETY: $W is `#[repr(transparent)]` over u8; layout is
        // byte-identical to u8 by Rust spec.
        unsafe impl $crate::transparent::Transparent for $W {
            type Inner = u8;
        }

        impl core::ops::Deref for $W {
            type Target = u8;
            #[inline(always)]
            fn deref(&self) -> &u8 { &self.0 }
        }

        impl AsRef<u8> for $W {
            #[inline(always)]
            fn as_ref(&self) -> &u8 { &self.0 }
        }

        impl From<u8> for $W {
            #[inline(always)]
            fn from(n: u8) -> Self { $helper(n) }
        }

        impl From<$W> for u8 {
            #[inline(always)]
            fn from(w: $W) -> u8 { w.0 }
        }

        /// Const-fn helper for const-generic-position construction.
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper-fn parameter constructing arvo type from u8 literal; tracked: #256
        $vis const fn $helper(n: u8) -> $W {
            $W(n)
        }
    };
}

meta_bits_wrapper! {
    /// Integer-bit count for fixed-point types.
    ///
    /// Carries the integer bit width through type-level code. Distinct
    /// from `FBits` so consumers cannot accidentally swap the two at
    /// construction sites.
    pub struct IBits;
    helper ibits;
}

meta_bits_wrapper! {
    /// Fractional-bit count for fixed-point types.
    ///
    /// Carries the fractional bit width through type-level code. See
    /// `IBits` for the integer counterpart.
    pub struct FBits;
    helper fbits;
}

meta_bits_wrapper! {
    /// Bit-width meta value (1..=128).
    ///
    /// Used as the const-generic param type for `Hasher<const N: Width>`
    /// and `Fnv1a<const N: Width>` (in arvo-hash).
    pub struct Width;
    helper width;
}

impl Width {
    /// 64-bit width (FNV-1a-64 cap).
    pub const W64: Width = width(64);
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
