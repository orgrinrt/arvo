//! Opaque N-bit container. Non-arithmetic; compared by identity.
//!
//! `#[repr(transparent)]` newtype around `<S as UContainerFor<N>>::T`
//! — the smallest native unsigned that fits N bits under the chosen
//! strategy (u8 for 1..=8, u16 for 9..=16, u32 for 17..=32, u64
//! for 33..=64 under Hot). Dispatches its container through the
//! same `UContainerFor<N, S>` table that `UFixed` uses, so the
//! storage footprint matches a same-width UFixed down to the byte.
//!
//! Bits is NOT a UFixed alias and NOT a UFixed wrapper. It is a
//! parallel primitive family that reuses arvo's container-dispatch
//! table to pick its storage, then presents a deliberately smaller
//! trait surface: `HasBitWidth` / `BitAccess` / `BitSequence` /
//! `BitLogic` but no `Add` / `Sub` / `Mul` / `Div` / `Ord`. Bit
//! patterns are identities, not arithmetic values.
//!
//! Primary consumers: `arvo-hash`'s `ContentHash = Bits<28>`
//! (u32-backed, 4 bytes); `hilavitkutin-str`'s `Str(Bits<32>)`
//! (u32-backed, 4 bytes); any domain needing a fixed-width opaque
//! identity.

use crate::strategy::{Hot, Strategy, UContainerFor};

/// N-bit opaque bit-pattern. Transparent wrapper over the
/// strategy-dispatched container primitive.
#[repr(transparent)]
pub struct Bits<const N: u8, S: Strategy = Hot>(<S as UContainerFor<N>>::T)
where
    S: UContainerFor<N>;

// Manual trait impls without bounds on `S` — the strategy marker is
// a zero-sized phantom (only `<S as UContainerFor<N>>::T` physically
// stores data), so no S-level bound is needed beyond the where clause.

impl<const N: u8, S: Strategy> Copy for Bits<N, S> where S: UContainerFor<N> {}

impl<const N: u8, S: Strategy> Clone for Bits<N, S>
where
    S: UContainerFor<N>,
{
    fn clone(&self) -> Self { *self }
}

impl<const N: u8, S: Strategy> PartialEq for Bits<N, S>
where
    S: UContainerFor<N>,
{
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<const N: u8, S: Strategy> Eq for Bits<N, S> where S: UContainerFor<N> {}

impl<const N: u8, S: Strategy> core::hash::Hash for Bits<N, S>
where
    S: UContainerFor<N>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<const N: u8, S: Strategy> core::fmt::Debug for Bits<N, S>
where
    S: UContainerFor<N>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Bits").field(&self.0).finish()
    }
}

impl<const N: u8, S: Strategy> Default for Bits<N, S>
where
    S: UContainerFor<N>,
{
    fn default() -> Self {
        Self(<<S as UContainerFor<N>>::T as Default>::default())
    }
}

// `From<<S as UContainerFor<N>>::T> for Bits<N, S>` was considered
// but conflicts with core's blanket `impl<T> From<T> for T`: the
// associated type could theoretically equal `Bits<N, S>`, which the
// trait solver treats as ambiguous. The per-N `From<u64>` impls in
// the macro block below cover the common ergonomic case; consumers
// holding a concrete u8/u16/u32/u64 container value use
// `Bits::from_raw(...)` directly.

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

// Ergonomic surface (round 202604500000): `Deref` / `AsRef` / `From`
// pairs so wrappers above `Bits` (IBits, FBits, Width, USize) can
// route through one level of `*` instead of `.0.0` chains.

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

/// Per-N const-fn bridges for the u64-typed legacy path plus the
/// round-202604500000 width-class accessors (`from_raw_uN` /
/// `to_raw_uN`). The narrow / widen happens through an `as` cast
/// generated per-N via macro.
macro_rules! impl_bits_u64 {
    ($ty:ty, $($n:literal),+ $(,)?) => {
        $(
            impl Bits<$n, Hot> {
                const MASK_U64: u64 =
                    if $n == 64 { u64::MAX } else { (1u64 << $n) - 1 };

                /// Construct from a u64, masking to N bits.
                #[deprecated(since = "0.1.0", note = "use Bits::from_raw_u64; removal in a follow-up round")]
                pub const fn new(raw: u64) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-N narrow from u64 to the dispatched container; arvo storage primitive at L0 root; tracked: #256
                    Self((raw & Self::MASK_U64) as $ty)
                }

                /// Widen the container back to u64.
                #[deprecated(since = "0.1.0", note = "use Bits::to_raw_u64; removal in a follow-up round")]
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

            // Non-const `From<u64>` — ergonomic `.into()` at runtime
            // call sites. For const contexts, use `Bits::from_raw_u64(raw)`.
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

// --- Trait impls via BitPrim on the container ---------------------------

