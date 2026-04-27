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
unsafe impl<const N: u8, S: Strategy> Transparent for Bits<N, S>
where
    S: UContainerFor<N>,
{
    type Inner = <S as UContainerFor<N>>::T;
}

// --- Per-size helpers (D-7 deletion target) ----------------------------------
//
// The `from_raw_uN` / `to_raw_uN` / `From<u64>` / `Bits::new` / `Bits::bits`
// surface is preserved verbatim during pass 2c so consumer call sites keep
// compiling. Pass 5 (D-7 per-size helper deletion) strips this entire block
// along with the consumer rewrites in arvo-hash and downstream crates.
// Carrying the surface forward (rather than deleting in pass 2c per the
// strict src-CL line 264 reading) keeps the round-202604271346 intermediate
// commits compile-clean. The mechanical deletion later in pass 5 stays
// trivial.

macro_rules! impl_bits_u64 {
    ($ty:ty, $($n:literal),+ $(,)?) => {
        $(
            impl Bits<$n, Hot> {
                const MASK_U64: u64 =
                    if $n == 64 { u64::MAX } else { (1u64 << $n) - 1 };

                /// Construct from a u64, masking to N bits.
                #[deprecated(since = "0.1.0", note = "use Bits::from_raw_u64; removal in pass 5 of round 202604271346")]
                pub const fn new(raw: u64) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-N narrow from u64 to the dispatched container; arvo-storage L0 root; tracked: #256
                    Self((raw & Self::MASK_U64) as $ty)
                }

                /// Widen the container back to u64.
                #[deprecated(since = "0.1.0", note = "use Bits::to_raw_u64; removal in pass 5 of round 202604271346")]
                pub const fn bits(self) -> u64 {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: widen dispatched container back to u64 for the uniform legacy API; tracked: #256
                    self.0 as u64
                }

                /// Construct from a u64, masking to N bits. Universal masking entry.
                pub const fn from_raw_u64(raw: u64) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-N narrow from u64 to the dispatched container; storage primitive at L0 root; tracked: #256
                    Self((raw & Self::MASK_U64) as $ty)
                }

                /// Project the container to a u64.
                pub const fn to_raw_u64(self) -> u64 {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: widen dispatched container back to u64 at the L0 root; tracked: #256
                    self.0 as u64
                }

                /// Construct from a u8, masking to N bits.
                ///
                /// For `N <= 8` this is the natural input width; for wider N
                /// the input is zero-extended after masking.
                pub const fn from_raw_u8(raw: u8) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: width-class sibling of from_raw_u64; tracked: #256
                    let masked = (raw as u64) & Self::MASK_U64;
                    Self(masked as $ty)
                }

                /// Project to u8 (truncating high bits if container is wider).
                ///
                /// Used by meta-layer wrappers (`IBits` / `FBits` / `Width`) whose
                /// inner `Bits<7, Hot>` always fits within u8.
                pub const fn to_raw_u8(self) -> u8 {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: widen dispatched container back to u8 at the L0 root; meta-layer accessor; tracked: #256
                    (self.0 as u64) as u8
                }
            }

            // Non-const `From<u64>`. Ergonomic `.into()` at runtime call sites.
            // For const contexts, use `Bits::from_raw_u64(raw)`.
            impl From<u64> for Bits<$n, Hot> {
                fn from(raw: u64) -> Self {
                    Self::from_raw_u64(raw)
                }
            }
        )+
    };
}

// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: container dispatch table names the native u8/u16/u32/u64 storage; tracked: #127
impl_bits_u64!(u8, 1, 2, 3, 4, 5, 6, 7, 8);
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #127
impl_bits_u64!(u16, 9, 10, 11, 12, 13, 14, 15, 16);
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #127
impl_bits_u64!(
    u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #127
#[rustfmt::skip]
impl_bits_u64!(
    u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
