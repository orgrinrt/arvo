//! Generic fixed-width bitmask chassis.
//!
//! `Mask<W>` is the type-generic chassis over a bit-bearing word `W`.
//! `W` in regular use is a `Bits<N, S, Sign>` from arvo-storage. The
//! substrate's `BitsContainerFor<N, Sign>` projection routes the
//! container; the chassis works uniformly for native-primitive backings
//! (`Bits<8/16/32/64/128, ...>`) and the wide-bucket
//! (`Bits<129..=256, Hot/Warm/Cold/Precise, ...>`) via the
//! `BitPrim` impls on `WideBits<BYTES, A>` from arvo-bits-contracts.
//!
//! Round 202605031748 (#313) deleted the prior parallel `Mask256`
//! struct and the `Mask64` / `Mask256` shipping aliases. Per the
//! Strategy/Sign-discoverability discipline, consumers name the
//! chassis form (`Mask<Bits<64, Hot, Unsigned>>`) directly so the
//! axes stay visible at every use site.
//!
//! `empty()` and `full()` route through `<Self as Identity>::ZERO`
//! and `<Self as Bounded>::MAX` (audit Finding 11). The const-trait
//! delegates exercise the substrate's own bridges rather than
//! hand-coding `W::default()`.

use arvo::USize;
use arvo::strategy::{Bounded, Identity};
use arvo_bits_contracts::{BitAccess, BitSequence, HasBitWidth};

/// Generic fixed-width bitmask.
///
/// `W` is a single bit-bearing word that satisfies `BitSequence +
/// BitAccess + Copy + Default`. Width follows from
/// `<W as HasBitWidth>::WIDTH`.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    /// Raw word storage.
    pub word: W,
}

impl<W> Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    /// Construct a mask from a raw word.
    #[inline(always)]
    pub const fn from_word(word: W) -> Self {
        Self { word }
    }

    /// Extract the raw word.
    #[inline(always)]
    pub const fn to_word(self) -> W {
        self.word
    }

    /// Logical bit width of the mask (from `W::WIDTH`).
    #[inline(always)]
    pub const fn width() -> USize {
        <W as HasBitWidth>::WIDTH
    }
}

impl<W> Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + Identity,
{
    /// Empty mask (all bits cleared). Routes through
    /// `<Self as Identity>::ZERO`.
    #[inline(always)]
    pub fn empty() -> Self {
        <Self as Identity>::ZERO
    }
}

impl<W> Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + Bounded,
{
    /// Full mask (all bits set). Routes through
    /// `<Self as Bounded>::MAX`.
    #[inline(always)]
    pub fn full() -> Self {
        <Self as Bounded>::MAX
    }
}

impl<W> Default for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_word(W::default())
    }
}

// --- Generic Bounded / Identity blankets (round 202605021600) ----------
//
// `Mask<W>::MIN` (empty / all-zero) and `Mask<W>::MAX` (full / all-ones)
// flow through the underlying word's Bounded impl. Identity::ZERO is
// the empty mask (no bits set). ONE corresponds to the underlying
// word's ONE (the lowest bit set), which is occasionally useful for
// shift-and-test patterns.

impl<W> const Bounded for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + [const] Bounded,
{
    const MIN: Self = Self::from_word(<W as Bounded>::MIN);
    const MAX: Self = Self::from_word(<W as Bounded>::MAX);
}

impl<W> const Identity for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + [const] Identity,
{
    const ZERO: Self = Self::from_word(<W as Identity>::ZERO);
    const ONE: Self = Self::from_word(<W as Identity>::ONE);
}
