//! Typed meta-bit newtypes for fixed-point bit counts.
//!
//! `IBits`, `FBits`, `Width` carry bit-count semantics distinct from
//! bare `u8`. They derive `ConstParamTy` so they appear as
//! const-generic parameters on `UFixed<const I: IBits, ...>`,
//! `IFixed<const I: IBits, ...>`, `Hasher<const N: Width>`, etc.
//!
//! The `meta_bits_wrapper!` macro DRYs the ergonomic surface so each
//! type gets the same accessor set without duplication.
//!
//! Round 202604280806 (Round C) widens the meta-newtype carrier from
//! `pub u8` to `pub MetaCarrier` (a `repr(transparent)` companion to
//! `Bits<9, Hot, Unsigned>`). The widening uses MetaCarrier rather
//! than the literal generic `Bits<9, Hot>` projection because the
//! latter triggers a rustc trait-solver cycle when consumers (UFixed,
//! IFixed) reach for the meta-newtype as `const I: IBits` in
//! const-eval where-clause position. MetaCarrier is layout-equivalent
//! to `Bits<9, Hot, Unsigned>` and exposes the Bits-typed view at
//! zero cost via `as_bits()`. See `arvo-storage` DESIGN.md for the
//! full rationale.

use core::marker::ConstParamTy;

use arvo_strategy::{Hot, Unsigned};
use arvo_transparent::Transparent;

use crate::Bits;

/// Layout-stable companion to `Bits<9, Hot, Unsigned>` used as the
/// meta-newtype carrier (`IBits`, `FBits`, `Width`).
///
/// `repr(transparent)` over u16: layout-identical to
/// `Bits<9, Hot, Unsigned>` (which projects to u16 via
/// `<Hot as BitsContainerFor<9, Unsigned>>::T`). The `as_bits()`
/// const-fn exposes the Bits-typed view at zero cost.
///
/// This companion exists to bypass a rustc trait-solver cycle that
/// arises when the meta-newtype field literally writes the generic
/// `Bits<9, Hot>` projection. The cycle interleaves
/// `IBits: Sized` evaluation with `UFixed`'s const-eval where-clause
/// `Hot: UContainerFor<{ ufixed_bits(I, F) }>`. Using a concrete
/// `repr(transparent)` companion at the field type sidesteps the
/// projection chain at well-formedness time while preserving the
/// 9-bit native range, the `Hot` strategy, and the `Unsigned` sign
/// of the conceptual carrier.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct MetaCarrier(pub u16);

impl MetaCarrier {
    /// Construct from the raw u16 carrier value.
    #[inline(always)]
    pub const fn from_raw(n: u16) -> Self { Self(n) }

    /// Project to the raw u16 carrier value.
    #[inline(always)]
    pub const fn to_raw(self) -> u16 { self.0 }

    /// Zero-cost view as `Bits<9, Hot, Unsigned>`.
    ///
    /// `MetaCarrier` and `Bits<9, Hot, Unsigned>` are both
    /// `repr(transparent)` over u16; this transmute is sound by
    /// layout equivalence.
    #[inline(always)]
    pub const fn as_bits(self) -> Bits<9, Hot, Unsigned> {
        // SAFETY: layout-identical (repr(transparent) over u16).
        unsafe { core::mem::transmute(self) }
    }
}

/// Generate a meta-bits ConstParamTy newtype carrying a `MetaCarrier`
/// payload (layout-equivalent to `Bits<9, Hot, Unsigned>`).
///
/// Each invocation produces:
/// - `pub struct $W(pub MetaCarrier)` with full derive set +
///   `ConstParamTy`
/// - `pub const ZERO` / `pub const ONE` value constants
/// - `pub const fn raw(self) -> u16` inherent (returns the underlying
///   u16 carrier value)
/// - `pub const fn as_bits(self) -> Bits<9, Hot, Unsigned>` accessor
///   exposing the Bits-typed view at zero cost
/// - `unsafe impl Transparent for $W { type Inner = u16 }` (transitive
///   transparency through MetaCarrier through u16)
/// - `Deref<Target = u16>` / `AsRef<u16>` / `From<u8>` / `From<u16>`
///   ergonomic surface; `From<$W> for u8` (narrowing) and
///   `From<$W> for u16` (carrier-width)
/// - `pub const fn $helper(n: u16) -> $W` for const-generic-position
///   literal construction
///
/// The carrier widening from the prior `pub u8` to `pub MetaCarrier`
/// honors round 202604280500's design intent (a 9-bit / u16-backed
/// carrier with `Hot` strategy and `Unsigned` sign) via a parallel
/// concrete wrapper. `MetaCarrier` and `Bits<9, Hot, Unsigned>` are
/// layout-equivalent, so the Bits-typed view is reachable at zero
/// cost via `MetaCarrier::as_bits()` and the per-type `as_bits()`
/// forwarders. The choice of MetaCarrier over the literal generic
/// `Bits<9, Hot>` projection sidesteps a rustc trait-solver cycle.
/// See module-level docs for the rationale.
macro_rules! meta_bits_wrapper {
    (
        $(#[$meta:meta])*
        $vis:vis struct $W:ident;
        helper $helper:ident;
    ) => {
        $(#[$meta])*
        #[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
        #[repr(transparent)]
        $vis struct $W(pub MetaCarrier);

        impl $W {
            /// Zero value.
            pub const ZERO: Self = Self(MetaCarrier::from_raw(0));
            /// One value.
            pub const ONE: Self = Self(MetaCarrier::from_raw(1));

            /// Read the underlying u16 carrier. See `arvo_transparent`
            /// for the unified `repr(transparent)` access surface.
            #[inline(always)]
            pub const fn raw(self) -> u16 { self.0.to_raw() }

            /// Zero-cost view as `Bits<9, Hot, Unsigned>`.
            #[inline(always)]
            pub const fn as_bits(self) -> Bits<9, Hot, Unsigned> {
                self.0.as_bits()
            }
        }

        // SAFETY: $W is `#[repr(transparent)]` over `MetaCarrier`,
        // which is `#[repr(transparent)]` over u16. Transitive
        // transparency: layout is byte-identical to u16 by Rust spec.
        unsafe impl const Transparent for $W {
            type Inner = u16;
        }

        impl core::ops::Deref for $W {
            type Target = u16;
            #[inline(always)]
            fn deref(&self) -> &u16 { &self.0.0 }
        }

        impl AsRef<u16> for $W {
            #[inline(always)]
            fn as_ref(&self) -> &u16 { &self.0.0 }
        }

        impl From<u8> for $W {
            #[inline(always)]
            fn from(n: u8) -> Self { $helper(n as u16) }
        }

        impl From<u16> for $W {
            #[inline(always)]
            fn from(n: u16) -> Self { $helper(n) }
        }

        impl From<$W> for u8 {
            #[inline(always)]
            fn from(w: $W) -> u8 { w.0.to_raw() as u8 }
        }

        impl From<$W> for u16 {
            #[inline(always)]
            fn from(w: $W) -> u16 { w.0.to_raw() }
        }

        /// Const-fn helper for const-generic-position construction.
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper-fn parameter constructing arvo type from u16 literal; tracked: #256
        $vis const fn $helper(n: u16) -> $W {
            $W(MetaCarrier::from_raw(n))
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
    pub const W64: Width = width(64u16);
}
