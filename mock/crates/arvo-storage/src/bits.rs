//! Opaque N-bit container. Non-arithmetic; compared by identity.
//!
//! `#[repr(transparent)]` newtype around `<S as UContainerFor<N>>::T`,
//! the smallest native unsigned that fits N bits under the chosen
//! strategy (u8 for 1..=8, u16 for 9..=16, u32 for 17..=32, u64
//! for 33..=64 under Hot). Dispatches its container through the
//! same `UContainerFor<N, S>` table that `UFixed` uses, so the
//! storage footprint matches a same-width UFixed down to the byte.
//!
//! Bits is NOT a UFixed alias and NOT a UFixed wrapper. It is a
//! parallel primitive family that reuses arvo's container-dispatch
//! table to pick its storage, then presents a deliberately smaller
//! trait surface: `HasBitWidth` / `BitAccess` / `BitSequence` /
//! `BitLogic` (declared in `arvo-bits-contracts`) but no `Add` /
//! `Sub` / `Mul` / `Div` / `Ord`. Bit patterns are identities, not
//! arithmetic values.
//!
//! Primary consumers: `arvo-hash`'s `ContentHash = Bits<28, S>`
//! (u32-backed under Warm, 4 bytes); `hilavitkutin-str`'s
//! `Str(Bits<32, S>)` (u32-backed under Warm, 4 bytes); any domain
//! needing a fixed-width opaque identity.

use core::marker::ConstParamTy_;

use arvo_strategy::{Hot, Strategy, UContainerFor};
use arvo_transparent::Transparent;

/// N-bit opaque bit-pattern. Transparent wrapper over the
/// strategy-dispatched container primitive.
///
/// Derives the standard trait set so `ConstParamTy_` can attach.
/// `ConstParamTy_` requires `StructuralPartialEq + Eq` which only
/// the derive path auto-impls. The where-clause bounds on
/// `<S as UContainerFor<N>>::T` fall out of `UContainerFor::T`'s
/// existing trait set; consumers don't repeat them.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Bits<const N: u8, S: Strategy = Hot>(<S as UContainerFor<N>>::T)
where
    S: UContainerFor<N>;

// Manual `ConstParamTy_` impl per the Q-A spike pattern. The
// derive cannot be used because the field type is a projection
// (`<S as UContainerFor<N>>::T`); the trait solver needs the
// projection path declared explicitly.
//
// SAFETY: `ConstParamTy_` requires structural eq + bitwise stable
// representation. `Bits<N, S>` is `repr(transparent)` over
// `<S as UContainerFor<N>>::T` (which is `u8`/`u16`/`u32`/`u64`
// under the per-N container table); structural eq follows from the
// derived `PartialEq + Eq` plus the inner primitive's structural eq.
impl<const N: u8, S: Strategy + Eq> ConstParamTy_ for Bits<N, S>
where
    S: UContainerFor<N>,
    <S as UContainerFor<N>>::T: ConstParamTy_,
{}

impl<const N: u8, S: Strategy> Bits<N, S>
where
    S: UContainerFor<N>,
{
    /// Construct from the raw container value.
    pub const fn from_raw(raw: <S as UContainerFor<N>>::T) -> Self {
        Self(raw)
    }

    /// Project to the raw container value.
    pub const fn to_raw(self) -> <S as UContainerFor<N>>::T {
        self.0
    }
}

// Ergonomic surface: `Deref` / `AsRef` so wrappers above `Bits`
// (downstream domain newtypes) can route through one level of `*`
// instead of `.0.0` chains.

impl<const N: u8, S: Strategy> core::ops::Deref for Bits<N, S>
where
    S: UContainerFor<N>,
{
    type Target = <S as UContainerFor<N>>::T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: u8, S: Strategy> AsRef<<S as UContainerFor<N>>::T> for Bits<N, S>
where
    S: UContainerFor<N>,
{
    #[inline(always)]
    fn as_ref(&self) -> &<S as UContainerFor<N>>::T {
        &self.0
    }
}

// SAFETY: `Bits<N, S>` is `repr(transparent)` over
// `<S as UContainerFor<N>>::T`; layout is byte-identical by Rust spec.
unsafe impl<const N: u8, S: Strategy> const Transparent for Bits<N, S>
where
    S: UContainerFor<N>,
{
    type Inner = <S as UContainerFor<N>>::T;
}

// Per-size helpers deleted in pass 5 of round 202604271346 (D-7).
// The `from_raw_uN` / `to_raw_uN` / `From<u64>` / `Bits::new` / `Bits::bits`
// surface and the `impl_bits_u64!` macro family are gone. Consumers go
// through the typed `Bits::from_raw(<S as UContainerFor<N>>::T)` plus
// container-typed shift/mask, with `as` for final container truncation per
// the doc CL D-7 spec.
