//! Set operations and bit scanning for the generic `Mask<W>` chassis.
//!
//! Round 202605031748 (#313) collapsed the prior per-flavour ops
//! (Mask64-specific + Mask256 four-word unrolls) onto a single
//! generic impl block. The substrate's `BitLogic` / `BitSequence` /
//! `BitAccess` trait-method dispatches handle the per-W
//! composition: for native-primitive W like `Bits<64, Hot,
//! Unsigned>`, dispatches resolve to single-instruction primitive
//! ops; for wide-bucket W like `Bits<256, Hot, Unsigned>`, the
//! `BitPrim` impl on `WideBits<BYTES, A>` carries the byte-by-byte
//! unroll at the substrate layer.
//!
//! Same trait methods, same routing across all widths. The chassis
//! stays one block.

use core::ops::{BitAnd, BitOr, BitXor, Not};

use arvo::strategy::{Additive, Bounded, Identity};
use arvo::{Bool, USize};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_mask_contracts::MaskOps;

use crate::mask::Mask;

// --- Generic Mask<W> set ops + scans ---------------------------------------

impl<W> Mask<W>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    /// Union (bitwise OR).
    #[inline(always)]
    pub fn union(self, other: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitor(self.word, other.word))
    }

    /// Intersection (bitwise AND).
    #[inline(always)]
    pub fn intersection(self, other: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitand(self.word, other.word))
    }

    /// Difference (`self & !other`).
    #[inline(always)]
    pub fn difference(self, other: Self) -> Self {
        let not_other = <W as BitLogic>::bitnot(other.word);
        Self::from_word(<W as BitLogic>::bitand(self.word, not_other))
    }

    /// Complement (bitwise NOT).
    #[inline(always)]
    pub fn complement(self) -> Self {
        Self::from_word(<W as BitLogic>::bitnot(self.word))
    }

    /// `Bool::TRUE` when every bit is zero.
    #[inline(always)]
    pub fn is_empty(self) -> Bool {
        <W as BitSequence>::is_zero(self.word)
    }

    /// `Bool::TRUE` when `self` and `other` share any bit.
    #[inline(always)]
    pub fn intersects(self, other: Self) -> Bool {
        let meet = <W as BitLogic>::bitand(self.word, other.word);
        Bool(!<W as BitSequence>::is_zero(meet).0)
    }

    /// `Bool::TRUE` when bit at `pos` is set.
    #[inline(always)]
    pub fn contains(self, pos: USize) -> Bool {
        <W as BitAccess>::bit(self.word, pos)
    }

    /// Set bit at `pos`.
    #[inline(always)]
    pub fn insert(&mut self, pos: USize) {
        self.word = <W as BitAccess>::with_bit_set(self.word, pos);
    }

    /// Clear bit at `pos`.
    #[inline(always)]
    pub fn remove(&mut self, pos: USize) {
        self.word = <W as BitAccess>::with_bit_cleared(self.word, pos);
    }

    /// Popcount.
    #[inline(always)]
    pub fn count(self) -> USize {
        <W as BitSequence>::count_ones(self.word)
    }

    /// Lowest set bit index. Returns `W::WIDTH` if the mask is empty
    /// (matches `trailing_zeros` semantics on a zero word).
    #[inline(always)]
    pub fn lowest_set(self) -> USize {
        <W as BitSequence>::trailing_zeros(self.word)
    }

    /// Highest set bit index. Returns `W::WIDTH` if the mask is empty.
    #[inline(always)]
    pub fn highest_set(self) -> USize {
        if <W as BitSequence>::is_zero(self.word).0 {
            return Self::width();
        }
        let lz = <W as BitSequence>::leading_zeros(self.word);
        let width = Self::width();
        (width - USize(1)) - lz
    }

    /// Iterator over set bit indices, lowest-first.
    #[inline(always)]
    pub fn iter_set_bits(self) -> SetBitsIter<W> {
        SetBitsIter {
            remaining: self.word,
        }
    }
}

/// Generic iterator over set bits of a `Mask<W>`.
///
/// Advance via `BitSequence::trailing_zeros` +
/// `BitLogic::clear_lowest_set_bit`; yields bit indices lowest-first.
#[derive(Copy, Clone)]
pub struct SetBitsIter<W>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    remaining: W,
}

impl<W> Iterator for SetBitsIter<W>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    type Item = USize;

    #[inline(always)]
    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn next(&mut self) -> Option<USize> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        if <W as BitSequence>::is_zero(self.remaining).0 {
            return None;
        }
        let idx = <W as BitSequence>::trailing_zeros(self.remaining);
        self.remaining = <W as BitLogic>::clear_lowest_set_bit(self.remaining);
        Some(idx)
    }
}

// --- core::ops impls (round 202605021600, lifted to chassis 202605031748) ---
//
// `BitLogic` on `Bits<N, S, Sign>` is `impl const`, so the chassis
// `core::ops` mirrors are also const-callable across the W range.

const impl<W> BitAnd for Mask<W>
where
    W: BitSequence + BitAccess + [const] BitLogic + Copy + Default,
{
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitand(self.word, rhs.word))
    }
}

const impl<W> BitOr for Mask<W>
where
    W: BitSequence + BitAccess + [const] BitLogic + Copy + Default,
{
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitor(self.word, rhs.word))
    }
}

const impl<W> BitXor for Mask<W>
where
    W: BitSequence + BitAccess + [const] BitLogic + Copy + Default,
{
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitxor(self.word, rhs.word))
    }
}

const impl<W> Not for Mask<W>
where
    W: BitSequence + BitAccess + [const] BitLogic + Copy + Default,
{
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Self::from_word(<W as BitLogic>::bitnot(self.word))
    }
}

// --- MaskOps blanket impl (round 202605040602, #315) ----------------
//
// The chassis already routes set / clear / test / count / union / etc
// through the underlying word's bit traits. The blanket lifts those
// inherent routings into the `MaskOps` const-trait surface, so
// downstream code that bounds on `[const] MaskOps` reaches the same
// codegen path that direct chassis calls produce. The bound mirrors
// the chassis bound on `Mask<W>` plus `[const] BitAccess + [const]
// BitLogic + [const] BitSequence + [const] Bounded + [const]
// Identity` for const callability through trait projection.
//
// `empty` and `full` route through `Identity::<Additive>::IDENTITY` and `Bounded::MAX`
// (matching the audit Finding-11 fix in the chassis inherent block).

const impl<W> MaskOps for Mask<W>
where
    W: Copy
        + Default
        + [const] BitAccess
        + [const] BitLogic
        + [const] BitSequence
        + [const] Bounded
        + [const] Identity<Additive>,
{
    #[inline(always)]
    fn empty() -> Self {
        <Self as Identity<Additive>>::IDENTITY
    }

    #[inline(always)]
    fn full() -> Self {
        <Self as Bounded>::MAX
    }

    #[inline(always)]
    fn set(self, idx: USize) -> Self {
        Self::from_word(<W as BitAccess>::with_bit_set(self.word, idx))
    }

    #[inline(always)]
    fn clear(self, idx: USize) -> Self {
        Self::from_word(<W as BitAccess>::with_bit_cleared(self.word, idx))
    }

    #[inline(always)]
    fn test(self, idx: USize) -> Bool {
        <W as BitAccess>::bit(self.word, idx)
    }

    #[inline(always)]
    fn count(self) -> USize {
        <W as BitSequence>::count_ones(self.word)
    }

    #[inline(always)]
    fn union(self, other: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitor(self.word, other.word))
    }

    #[inline(always)]
    fn intersection(self, other: Self) -> Self {
        Self::from_word(<W as BitLogic>::bitand(self.word, other.word))
    }

    #[inline(always)]
    fn difference(self, other: Self) -> Self {
        let not_other = <W as BitLogic>::bitnot(other.word);
        Self::from_word(<W as BitLogic>::bitand(self.word, not_other))
    }

    #[inline(always)]
    fn complement(self) -> Self {
        Self::from_word(<W as BitLogic>::bitnot(self.word))
    }
}
