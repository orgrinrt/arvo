//! Facade re-export of the strategy surface.
//!
//! Per round 202604271346, the four strategy markers (`Hot`, `Warm`,
//! `Cold`, `Precise`), the `Strategy` marker trait, the
//! container-projection const traits (`UContainerFor` / `IContainerFor`),
//! the strategy-resolution const trait (`Resolve`), and the
//! arithmetic / widen / narrow trait families moved to the
//! `arvo-strategy` crate. This module re-exports them so existing
//! `arvo::Hot` / `arvo::Strategy` / `arvo::UContainerFor` import
//! paths remain valid.
//!
//! Const-generic-position helpers that bridge the typed meta-bit
//! newtypes (`IBits`, `FBits`, `Width`) into the bare-`u8` const
//! parameters of the projection traits stay in this module: those
//! helpers depend on the meta-bit types still hosted in the arvo
//! facade and would create a cycle if moved alongside the strategy
//! traits.

pub use arvo_strategy::{
    BitsContainerFor, Cold, Hot, IArith, IContainerFor, INarrowFrom, ISaturating, IWidenFrom,
    Precise, Resolve, Signed, Signedness, Strategy, UArith, UContainerFor, UNarrowFrom,
    USaturating, UWidenFrom, Unsigned, Warm,
};

use arvo_storage::{FBits, IBits};

/// Const-fn helper: total logical bits for a `UFixed<I, F, S>`.
///
/// `UFixed` logical width is `I + F`. The trait bound uses this to
/// look up the container type. Round 202604280806 widened the
/// meta-newtype carrier to a u16-backed `MetaCarrier`; this helper
/// narrows back to u8 because `Bits<const N: u8>` itself stays at
/// u8 (reachable bit-width caps at 255 in this round).
#[inline(always)]
pub const fn ufixed_bits(i: IBits, f: FBits) -> u8 {
    (i.raw() + f.raw()) as u8
}

/// Const-fn helper: total logical bits for an `IFixed<I, F, S>`.
///
/// `IFixed` reserves one bit for the sign; logical width is `1 + I + F`.
/// Same u16-to-u8 narrowing as `ufixed_bits`.
#[inline(always)]
pub const fn ifixed_bits(i: IBits, f: FBits) -> u8 {
    (1 + i.raw() + f.raw()) as u8
}

/// Indicator const: `1` when `f > 0`, `0` when `f == 0`.
///
/// Used to express "F has a fractional component" in const-generic
/// where-clauses without struct construction.
#[inline(always)]
pub const fn is_fractional(f: FBits) -> usize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic position helper; bare usize is the const-expression carrier here pending nightly relaxation; tracked: #256
    if f.raw() == 0 { 0 } else { 1 }
}

/// Whether a `Width` is `<= 64`. Used by Fnv1a's const-eval guard.
///
/// Predicate, not an accessor; an accessor of `Width` to its inner
/// `u8` is `width.raw()` (or `arvo::raw(width)` if you prefer the
/// prefix style).
#[inline(always)]
pub const fn width_le_64(n: arvo_storage::Width) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic where-clause predicate; bare bool needed for const-eval guard; tracked: #256
    n.raw() <= 64
}
