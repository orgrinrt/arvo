//! Typed meta-bit newtypes for fixed-point bit counts.
//!
//! `IBits`, `FBits`, `Width` carry bit-count semantics distinct from
//! bare `u8`. They derive `ConstParamTy` so they appear as
//! const-generic parameters on `UFixed<const I: IBits, ...>`,
//! `IFixed<const I: IBits, ...>`, `Hasher<const N: Width>`, etc.
//!
//! The `meta_bits_wrapper!` macro DRYs the ergonomic surface so each
//! type gets the same accessor set without duplication.

use core::marker::ConstParamTy;

use arvo_transparent::Transparent;

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

            /// Read the inner `u8`. See `arvo_transparent` for the
            /// unified `repr(transparent)` access surface.
            #[inline(always)]
            pub const fn raw(self) -> u8 { self.0 }
        }

        // SAFETY: $W is `#[repr(transparent)]` over u8; layout is
        // byte-identical to u8 by Rust spec.
        unsafe impl Transparent for $W {
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
