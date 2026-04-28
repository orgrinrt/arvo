//! Opaque N-bit container. Non-arithmetic; compared by identity.
//!
//! `#[repr(transparent)]` newtype around
//! `<S as BitsContainerFor<N, Sign>>::T`, the storage primitive that
//! holds N bits under the chosen strategy and signedness. Routes
//! through `BitsContainerFor<N, Sign>` (in `arvo-strategy`) which
//! projects to `UContainerFor<N>::T` for `Sign = Unsigned` or
//! `IContainerFor<N>::T` for `Sign = Signed`. Default `Sign = Unsigned`
//! keeps every existing call site unchanged; `IFixed<I, F, S>` reaches
//! for `Sign = Signed` internally.
//!
//! Container projections: u8 for 1..=8 under Hot, u16 for 9..=16,
//! u32 for 17..=32, u64 for 33..=64. Round 202604280500 extended the
//! tables: u128 for 65..=128 (Hot/Cold), u128 for 33..=64
//! (Warm/Precise as the 2x-logical primitive). 129..=255 dispatches
//! through `MultiContainer<HiT, LoT>` (storage-only this round;
//! arithmetic on multi-value containers is BACKLOG-tracked).
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
//! needing a fixed-width opaque identity. `UFixed<I, F, S>` wraps
//! `Bits<{I+F}, S, Unsigned>` (default Sign omitted at the call site).
//! `IFixed<I, F, S>` wraps `Bits<{1+I+F}, S, Signed>`.

use core::marker::ConstParamTy_;

use arvo_strategy::{BitsContainerFor, Hot, Signedness, Strategy, Unsigned};
use arvo_transparent::Transparent;

/// N-bit opaque bit-pattern. Transparent wrapper over the
/// strategy-and-sign-dispatched container primitive.
///
/// Derives the standard trait set so `ConstParamTy_` can attach.
/// `ConstParamTy_` requires `StructuralPartialEq + Eq` which only
/// the derive path auto-impls. The where-clause bounds on
/// `<S as BitsContainerFor<N, Sign>>::T` fall out of the trait's
/// associated-type bounds; consumers don't repeat them.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Bits<const N: u16, S: Strategy = Hot, Sign: Signedness = Unsigned>(
    <S as BitsContainerFor<N, Sign>>::T,
)
where
    S: BitsContainerFor<N, Sign>;

// Manual `ConstParamTy_` impl per the Q-A spike pattern. The
// derive cannot be used because the field type is a projection
// (`<S as BitsContainerFor<N, Sign>>::T`); the trait solver needs the
// projection path declared explicitly.
//
// SAFETY: `ConstParamTy_` requires structural eq + bitwise stable
// representation. `Bits<N, S, Sign>` is `repr(transparent)` over
// `<S as BitsContainerFor<N, Sign>>::T` (a native primitive or a
// `MultiContainer<HiT, LoT>` shape per the strategy projection);
// structural eq follows from the derived `PartialEq + Eq` plus the
// inner primitive's structural eq.
impl<const N: u16, S: Strategy + Eq, Sign: Signedness + Eq> ConstParamTy_ for Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: ConstParamTy_,
{}

impl<const N: u16, S: Strategy, Sign: Signedness> Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
{
    /// Construct from the raw container value.
    pub const fn from_raw(raw: <S as BitsContainerFor<N, Sign>>::T) -> Self {
        Self(raw)
    }

    /// Project to the raw container value.
    pub const fn to_raw(self) -> <S as BitsContainerFor<N, Sign>>::T {
        self.0
    }
}

// Ergonomic surface: `Deref` / `AsRef` so wrappers above `Bits`
// (downstream domain newtypes) can route through one level of `*`
// instead of `.0.0` chains.

impl<const N: u16, S: Strategy, Sign: Signedness> core::ops::Deref for Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
{
    type Target = <S as BitsContainerFor<N, Sign>>::T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: u16, S: Strategy, Sign: Signedness> AsRef<<S as BitsContainerFor<N, Sign>>::T>
    for Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
{
    #[inline(always)]
    fn as_ref(&self) -> &<S as BitsContainerFor<N, Sign>>::T {
        &self.0
    }
}

// SAFETY: `Bits<N, S, Sign>` is `repr(transparent)` over
// `<S as BitsContainerFor<N, Sign>>::T`; layout is byte-identical by
// Rust spec.
unsafe impl<const N: u16, S: Strategy, Sign: Signedness> const Transparent for Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
{
    type Inner = <S as BitsContainerFor<N, Sign>>::T;
}

// Per-size helpers deleted in pass 5 of round 202604271346 (D-7).
// The `from_raw_uN` / `to_raw_uN` / `From<u64>` / `Bits::new` / `Bits::bits`
// surface and the `impl_bits_u64!` macro family are gone. Consumers go
// through the typed `Bits::from_raw(<S as BitsContainerFor<N, Sign>>::T)`
// plus container-typed shift/mask, with `as` for final container
// truncation per the doc CL D-7 spec.
