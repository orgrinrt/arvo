//! Model of the sealed `NicheCarrier` vocabulary (87 section 1.3, adopted as
//! a working shape at 90b) at the smallest scale a cross-crate fabrication
//! attack can be mounted on. Compiled as an rlib; the attack files compile
//! against it with `--extern tower`.
//!
//! The seal follows file 46's completed shape: a private supertrait closes
//! the direct-impl route (E0277) and the supertrait route (E0603); the
//! orphan rules close the re-impl (E0117) and downstream-blanket (E0210)
//! routes for free; membership is by explicit per-type impl, never by a
//! granting blanket, so upstream growth of std's niche set cannot widen the
//! vocabulary silently.
#![no_std]

mod sealed {
    pub trait Sealed {}
}

/// The sealed vocabulary: std types carrying a language-declared validity
/// range, admitted one explicit impl at a time.
pub trait NicheCarrier: sealed::Sealed + Copy {
    /// the raw integer this niche type refines
    type Raw: Copy;
    /// bit width of the raw type; the entry's totality condition is stated
    /// against the inhabitant set of this width minus the excluded count
    const RAW_BITS: u32;
    /// number of excluded patterns (1 for the NonZero family)
    const EXCLUDED: u32;
}

impl sealed::Sealed for core::num::NonZeroU16 {}
impl NicheCarrier for core::num::NonZeroU16 {
    type Raw = u16;
    const RAW_BITS: u32 = 16;
    const EXCLUDED: u32 = 1;
}

/// Model of `Crosses`. In the real design this is the unsafe trait carrying
/// statements 0, P, C; here it exists so the one audited entry has a place
/// to live and the attacks have something to reach for.
pub unsafe trait Crosses {}

/// A lowering routed through a niche carrier.
pub struct ViaNiche<C: NicheCarrier>(core::marker::PhantomData<C>);

// SAFETY (the one audited trusted-base entry, model form): for every member
// of the sealed vocabulary, the excluded pattern is unreachable in safe
// code per the member's own documented contract in `core`; the tower proves
// nothing about it. Everything else the entry needs (inhabitant-totality of
// the decode, the width claim, the no-wrap condition) is const-checkable at
// the declaration and does not belong in this sentence (probes 2 and 3).
unsafe impl<C: NicheCarrier> Crosses for ViaNiche<C> {}
