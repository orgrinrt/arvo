//! `Width` newtype for logical bit-width meta-values.
//!
//! `Width` is a `ConstParamTy` newtype over `u16` carrying logical bit
//! count (1..=65535) through type-level code. It appears as the
//! const-generic parameter on `BitsContainerFor<const N: Width, Sign>`
//! and on `Bits<const N: Width, S, Sign>` in arvo-storage.
//!
//! Round 202605031400 (#316) relocated `Width` from arvo-storage to
//! arvo-strategy per `arvo-bridge-home-rule.md`: the projection trait
//! that returns Width-keyed types lives in arvo-strategy, so Width
//! needs to be reachable here. arvo-storage re-exports for source
//! compatibility within the workspace facade.
//!
//! The MetaCarrier-backed `IBits` / `FBits` newtypes stay in
//! arvo-storage (they bypass a different rustc trait-solver cycle in
//! UFixed / IFixed const-eval where-clauses; that cycle does not
//! affect Width).
//!
//! This module also ships the `tag` / `bytes_for` const-fn helpers
//! that drive Pattern C const-tag dispatch in `container.rs`, plus
//! `width` / `width_u8` / `width_u16` ergonomic constructors.

use core::marker::ConstParamTy;

use crate::{Bounded, Identity};

/// Logical bit-width carrier.
///
/// `repr(transparent)` over `u16`. Implements `ConstParamTy` so it
/// can appear in const-generic position on the projection traits.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Width(pub u16);

impl Width {
    /// Zero width.
    pub const ZERO: Self = Width(0);
    /// One bit.
    pub const ONE: Self = Width(1);
    /// 64-bit width (FNV-1a-64 cap).
    pub const W64: Self = Width(64);
    /// 128-bit native ladder cap.
    pub const W128: Self = Width(128);

    /// Read the underlying u16 carrier.
    #[inline(always)]
    pub const fn raw(self) -> u16 {
        self.0
    }
}

impl const Bounded for Width {
    const MIN: Self = Width(<u16 as Bounded>::MIN);
    const MAX: Self = Width(<u16 as Bounded>::MAX);
}

impl const Identity for Width {
    const ZERO: Self = Width(<u16 as Identity>::ZERO);
    const ONE: Self = Width(<u16 as Identity>::ONE);
}

/// Construct a `Width` from a u16 literal.
///
/// Const-fn helper for const-generic-position literal construction.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper-fn parameter constructing arvo type from u16 literal; tracked: #316
#[inline(always)]
pub const fn width(n: u16) -> Width {
    Width(n)
}

/// Construct a `Width` from a u8 literal (widening).
///
/// Used by arvo-hash's `Hasher<const N: u8>` projection sites that
/// need to lift the u8 const-generic to a Width for BitsContainerFor.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper for u8 → Width conversion at projection sites; tracked: #316
#[inline(always)]
pub const fn width_u8(n: u8) -> Width {
    Width(n as u16)
}

/// Construct a `Width` from a u16 literal (explicit form).
///
/// Synonym for `width()`, named for clarity at projection sites where
/// the source type is explicitly u16.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper for u16 → Width at projection sites; tracked: #316
#[inline(always)]
pub const fn width_u16(n: u16) -> Width {
    Width(n)
}

/// Construct a `Width` clamped to <= 64.
///
/// Helper for narrowing projection sites that bound on N <= 64
/// (FNV-1a-64 etc.). Saturates at 64 if `n > 64`.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic helper for clamped u8 → Width; tracked: #316
#[inline(always)]
pub const fn width_le_64(n: u8) -> Width {
    Width(if n > 64 { 64 } else { n as u16 })
}

/// Bucket tag for Pattern C const-tag dispatch (sketch 07).
///
/// 0..=4 are native buckets (u8 / u16 / u32 / u64 / u128). 5 is the
/// wide bucket (WideBits / AlignedWideBits16 above 128 logical bits).
#[inline(always)]
pub const fn tag(n: Width) -> usize {
    let n = n.0 as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}

/// Round logical bit count up to whole bytes.
///
/// `bytes_for(Width(129))` returns 17. Drives the BYTES const-generic
/// on `WideBits<BYTES>` / `AlignedWideBits16<BYTES>` for N > 128.
#[inline(always)]
pub const fn bytes_for(n: Width) -> usize {
    (n.0 as usize).div_ceil(8)
}

/// Mask the low N bits of a u64.
///
/// Returns `(1 << N) - 1` for `N < 64`; saturates to `u64::MAX` for `N >= 64`
/// to avoid the `1u64 << 64` undefined-behaviour shift.
///
/// Used by `NarrowFromU64` (arvo-bits-contracts) and any consumer that
/// needs to mask a u64 algorithm-state to N bits. Centralises the
/// pattern previously inlined four times in the FNV-1a / XxHash3 macro
/// pastes in arvo-hash.
// lint:allow(no-bare-numeric) reason: helper returns a u64 mask for use in
// algorithm-state narrowing; both parameter and return are algorithm-fixed
// widths at the substrate's u64-state hash boundary; tracked: #314
#[inline(always)]
pub const fn mask_low_bits(n: u16) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n) - 1
    }
}
